// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/albedo_factor.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: A color multiplier, usually applied to a whole entity, e.g. a mesh.
#[derive(
    Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck::Pod, bytemuck::Zeroable,
)]
#[repr(transparent)]
pub struct AlbedoFactor(pub crate::datatypes::Rgba32);

impl ::re_types_core::SizeBytes for AlbedoFactor {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Rgba32>::is_pod()
    }
}

impl<T: Into<crate::datatypes::Rgba32>> From<T> for AlbedoFactor {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Rgba32> for AlbedoFactor {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Rgba32 {
        &self.0
    }
}

impl std::ops::Deref for AlbedoFactor {
    type Target = crate::datatypes::Rgba32;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Rgba32 {
        &self.0
    }
}

impl std::ops::DerefMut for AlbedoFactor {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Rgba32 {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(AlbedoFactor);

impl ::re_types_core::Loggable for AlbedoFactor {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.AlbedoFactor".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        crate::datatypes::Rgba32::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Rgba32::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::Rgba32::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow2::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::Rgba32::from_arrow(arrow_data).map(bytemuck::cast_vec)
    }
}

impl ::re_types_core::Component for AlbedoFactor {
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new(Self::name())
    }
}

// impl ::re_types_core::ComponentBatch for AlbedoFactor {
//     #[inline]
//     fn descriptor(&self) -> ComponentDescriptor {
//         ComponentDescriptor::new(self.name())
//     }
// }
