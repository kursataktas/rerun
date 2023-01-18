use re_data_store::{query_transform, InstanceIdHash, ObjPath, ObjectProps};
use re_log_types::{
    coordinates::{Handedness, SignedAxis3},
    IndexHash, Pinhole, Transform, ViewCoordinates,
};
use re_query::{query_entity_with_primary, QueryError};
use re_renderer::renderer::LineStripFlags;

use crate::{
    misc::{space_info::query_view_coordinates, ObjectPathSelectionResult, ViewerContext},
    ui::{
        scene::SceneQuery,
        transform_cache::{ReferenceFromObjTransform, TransformCache},
        view_spatial::{SceneSpatial, SpaceCamera3D},
    },
};

use super::ScenePart;

/// `ScenePart` for classic data path
pub struct CamerasPartClassic;

impl ScenePart for CamerasPartClassic {
    fn load(
        &self,
        scene: &mut SceneSpatial,
        ctx: &mut ViewerContext<'_>,
        query: &SceneQuery<'_>,
        transforms: &TransformCache,
    ) {
        crate::profile_scope!("CamerasPartClassic");

        // Atypical query. But gone soon anyways once everything is Arrow driven (where this isn't as special!)
        for (obj_path, props) in query.iter_entities() {
            // TODO(andreas): What about time ranges? See also https://github.com/rerun-io/rerun/issues/723
            let query_time = ctx.rec_cfg.time_ctrl.time_i64();
            let Some(Transform::Pinhole(pinhole)) = query_transform(
                    &ctx.log_db.obj_db,
                    &query.timeline,
                    obj_path,
                    query_time) else {
                continue;
            };
            let instance_hash = {
                if props.interactive {
                    InstanceIdHash::from_path_and_index(obj_path, IndexHash::NONE)
                } else {
                    InstanceIdHash::NONE
                }
            };
            let highlighted_paths = ctx.hovered().is_path_selected(obj_path.hash());

            let view_coordinates = determine_view_coordinates(
                &ctx.log_db.obj_db,
                &ctx.rec_cfg.time_ctrl,
                obj_path.clone(),
            );

            CamerasPart::visit_instance(
                scene,
                obj_path,
                &props,
                transforms,
                instance_hash,
                pinhole,
                view_coordinates,
                &highlighted_paths,
            );
        }
    }
}

/// Determine the view coordinates (i.e.) the axis semantics.
///
/// The recommended way to log this is on the object holding the extrinsic camera properties
/// (i.e. the last rigid transform from here)
/// But for ease of use allow it everywhere along the path.
///
/// TODO(andreas): Doing a search upwards here isn't great. Maybe this can be part of the transform cache or similar?
fn determine_view_coordinates(
    obj_db: &re_data_store::log_db::ObjDb,
    time_ctrl: &crate::misc::TimeControl,
    mut obj_path: ObjPath,
) -> ViewCoordinates {
    loop {
        if let Some(view_coordinates) = query_view_coordinates(obj_db, time_ctrl, &obj_path) {
            return view_coordinates;
        }

        if let Some(parent) = obj_path.parent() {
            obj_path = parent;
        } else {
            // Keep in mind, there is no universal convention for any of this!
            // https://twitter.com/freyaholmer/status/1325556229410861056
            return ViewCoordinates::from_up_and_handedness(
                SignedAxis3::POSITIVE_Y,
                Handedness::Right,
            );
        }
    }
}

pub struct CamerasPart;

impl CamerasPart {
    #[allow(clippy::too_many_arguments)]
    fn visit_instance(
        scene: &mut SceneSpatial,
        obj_path: &ObjPath,
        props: &ObjectProps,
        transforms: &TransformCache,
        instance: InstanceIdHash,
        pinhole: Pinhole,
        view_coordinates: ViewCoordinates,
        highlighted_paths: &ObjectPathSelectionResult,
    ) {
        // The transform *at* this object path already has the pinhole transformation we got passed in!
        // This makes sense, since if there's an image logged here one would expect that the transform applies.
        // We're however first interested in the rigid transform that led here, so query the parent transform.
        //
        // Note that currently a transform on an object can't have both a pinhole AND a rigid transform,
        // which makes this rather well defined here.
        let parent_path = obj_path
            .parent()
            .expect("root path can't be part of scene query");
        let ReferenceFromObjTransform::Reachable(world_from_parent) =
            transforms.reference_from_obj(&parent_path) else {
                return;
            };

        // If this transform is not representable as rigid transform, the camera is probably under another camera transform,
        // in which case we don't (yet) know how to deal with this!
        let Some(world_from_camera) = macaw::IsoTransform::from_mat4(&world_from_parent) else {
            return;
        };

        // Actual primitives are generated later.
        // Currently, we need information about viewport to display it correctly.
        // TODO(andreas): Would be great if we add all the lines etc. right away!
        //                  Let's attempt this as part of
        //                  https://github.com/rerun-io/rerun/issues/681 (Improve camera frustum length heuristic & editability)
        //                  and https://github.com/rerun-io/rerun/issues/686 (Replace camera mesh with expressive camera gizmo (extension of current frustum)
        scene.space_cameras.push(SpaceCamera3D {
            obj_path: obj_path.clone(),
            instance,
            view_coordinates,
            world_from_camera,
            pinhole: Some(pinhole),
        });

        let frustum_length = props.pinhole_image_plane_distance(&pinhole);

        // TODO(andreas): FOV fallback doesn't make much sense. What does pinhole without fov mean?
        let fov_y = pinhole.fov_y().unwrap_or(std::f32::consts::FRAC_PI_2);
        let fy = (fov_y * 0.5).tan() * frustum_length;
        let fx = fy * pinhole.aspect_ratio().unwrap_or(1.0);

        let image_center_pixel = pinhole.resolution().unwrap_or(glam::Vec2::ZERO) * 0.5;
        let principal_point_offset_pixel = image_center_pixel - pinhole.principal_point();
        let principal_point_offset =
            principal_point_offset_pixel / pinhole.resolution().unwrap_or(glam::Vec2::ONE);
        // Don't multiply with (fx,fy) because that would multiply the aspect ratio twice!
        // Times two since fy is the half screen size (extending from -fy to fy!).
        let offset = principal_point_offset * (fy * 2.0);

        let corners = [
            (offset + glam::vec2(fx, -fy)).extend(frustum_length),
            (offset + glam::vec2(fx, fy)).extend(frustum_length),
            (offset + glam::vec2(-fx, fy)).extend(frustum_length),
            (offset + glam::vec2(-fx, -fy)).extend(frustum_length),
        ];
        let triangle_frustum_offset = fy * 1.05;
        let up_triangle = [
            // Use only fx for with and height of the triangle, so that the aspect ratio of the triangle is always the same.
            (offset + glam::vec2(-fx * 0.25, -triangle_frustum_offset)).extend(frustum_length),
            (offset + glam::vec2(0.0, -fx * 0.25 - triangle_frustum_offset)).extend(frustum_length),
            (offset + glam::vec2(fx * 0.25, -triangle_frustum_offset)).extend(frustum_length),
        ];

        let segments = [
            // Frustum corners
            (glam::Vec3::ZERO, corners[0]),
            (glam::Vec3::ZERO, corners[1]),
            (glam::Vec3::ZERO, corners[2]),
            (glam::Vec3::ZERO, corners[3]),
            // rectangle around "far plane"
            (corners[0], corners[1]),
            (corners[1], corners[2]),
            (corners[2], corners[3]),
            (corners[3], corners[0]),
            // triangle indicating up direction
            (up_triangle[0], up_triangle[1]),
            (up_triangle[1], up_triangle[2]),
            (up_triangle[2], up_triangle[0]),
        ];

        let (line_radius, line_color) =
            if highlighted_paths.is_index_selected(instance.instance_index_hash) {
                (
                    re_renderer::Size::new_points(2.0),
                    SceneSpatial::HOVER_COLOR,
                )
            } else {
                (
                    re_renderer::Size::new_points(1.0),
                    SceneSpatial::CAMERA_COLOR,
                )
            };
        scene
            .primitives
            .line_strips
            .batch("camera frustum")
            .world_from_obj(world_from_parent)
            .add_segments(segments.into_iter())
            .radius(line_radius)
            .color(line_color)
            .flags(
                LineStripFlags::NO_COLOR_GRADIENT
                    | LineStripFlags::CAP_END_ROUND
                    | LineStripFlags::CAP_START_ROUND,
            )
            .user_data(instance);
    }
}

impl ScenePart for CamerasPart {
    fn load(
        &self,
        scene: &mut SceneSpatial,
        ctx: &mut ViewerContext<'_>,
        query: &SceneQuery<'_>,
        transforms: &TransformCache,
    ) {
        crate::profile_scope!("CamerasPart");

        for (ent_path, props) in query.iter_entities() {
            let query = re_arrow_store::LatestAtQuery::new(query.timeline, query.latest_at);

            match query_entity_with_primary::<Transform>(
                &ctx.log_db.obj_db.arrow_store,
                &query,
                ent_path,
                &[],
            )
            .and_then(|entity_view| {
                let highlighted_paths = ctx.hovered().is_path_selected(ent_path.hash());

                entity_view.visit1(|instance, transform| {
                    let Transform::Pinhole(pinhole) = transform else {
                        return;
                    };

                    let instance_hash = {
                        if props.interactive {
                            InstanceIdHash::from_path_and_arrow_instance(ent_path, &instance)
                        } else {
                            InstanceIdHash::NONE
                        }
                    };

                    let view_coordinates = determine_view_coordinates(
                        &ctx.log_db.obj_db,
                        &ctx.rec_cfg.time_ctrl,
                        ent_path.clone(),
                    );

                    Self::visit_instance(
                        scene,
                        ent_path,
                        &props,
                        transforms,
                        instance_hash,
                        pinhole,
                        view_coordinates,
                        &highlighted_paths,
                    );
                })
            }) {
                Ok(_) | Err(QueryError::PrimaryNotFound) => {}
                Err(err) => {
                    re_log::error_once!("Unexpected error querying '{:?}': {:?}", ent_path, err);
                }
            }
        }
    }
}
