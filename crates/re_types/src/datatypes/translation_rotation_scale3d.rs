// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// Representation of an affine transform via separate translation, rotation & scale.
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct TranslationRotationScale3D {
    /// 3D translation vector, applied last.
    pub translation: Option<crate::datatypes::Vec3D>,

    /// 3D rotation, applied second.
    pub rotation: Option<crate::datatypes::Rotation3D>,

    /// 3D scale, applied first.
    pub scale: Option<crate::datatypes::Scale3D>,

    /// If true, the transform maps from the parent space to the space where the transform was logged.
    /// Otherwise, the transform maps from the space to its parent.
    pub from_parent: bool,
}

impl<'a> From<TranslationRotationScale3D> for ::std::borrow::Cow<'a, TranslationRotationScale3D> {
    #[inline]
    fn from(value: TranslationRotationScale3D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a TranslationRotationScale3D>
    for ::std::borrow::Cow<'a, TranslationRotationScale3D>
{
    #[inline]
    fn from(value: &'a TranslationRotationScale3D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for TranslationRotationScale3D {
    type Name = crate::DatatypeName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = Box<
        dyn ::fallible_iterator::FallibleIterator<
                Item = Self::Item<'a>,
                Error = crate::DeserializationError,
            > + 'a,
    >;
    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.TranslationRotationScale3D".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "translation".to_owned(),
                data_type: <crate::datatypes::Vec3D>::to_arrow_datatype(),
                is_nullable: true,
                metadata: [].into(),
            },
            Field {
                name: "rotation".to_owned(),
                data_type: <crate::datatypes::Rotation3D>::to_arrow_datatype(),
                is_nullable: true,
                metadata: [].into(),
            },
            Field {
                name: "scale".to_owned(),
                data_type: <crate::datatypes::Scale3D>::to_arrow_datatype(),
                is_nullable: true,
                metadata: [].into(),
            },
            Field {
                name: "from_parent".to_owned(),
                data_type: DataType::Boolean,
                is_nullable: false,
                metadata: [].into(),
            },
        ])
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                (if let Some(ext) = extension_wrapper {
                    DataType::Extension(
                        ext.to_owned(),
                        Box::new(
                            <crate::datatypes::TranslationRotationScale3D>::to_arrow_datatype(),
                        ),
                        None,
                    )
                } else {
                    <crate::datatypes::TranslationRotationScale3D>::to_arrow_datatype()
                })
                .to_logical_type()
                .clone(),
                vec![
                    {
                        let (somes, translation): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| {
                                        let Self { translation, .. } = &**datum;
                                        translation.clone()
                                    })
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let translation_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let translation_inner_data: Vec<_> = translation
                                .iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::datatypes::Vec3D(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .flatten()
                                .map(Some)
                                .collect();
                            let translation_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                            FixedSizeListArray::new(
                                {
                                    _ = extension_wrapper;
                                    DataType::FixedSizeList(
                                        Box::new(Field {
                                            name: "item".to_owned(),
                                            data_type: DataType::Float32,
                                            is_nullable: false,
                                            metadata: [].into(),
                                        }),
                                        3usize,
                                    )
                                    .to_logical_type()
                                    .clone()
                                },
                                PrimitiveArray::new(
                                    {
                                        _ = extension_wrapper;
                                        DataType::Float32.to_logical_type().clone()
                                    },
                                    translation_inner_data
                                        .into_iter()
                                        .map(|v| v.unwrap_or_default())
                                        .collect(),
                                    translation_inner_bitmap,
                                )
                                .boxed(),
                                translation_bitmap,
                            )
                            .boxed()
                        }
                    },
                    {
                        let (somes, rotation): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| {
                                        let Self { rotation, .. } = &**datum;
                                        rotation.clone()
                                    })
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let rotation_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = rotation_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::Rotation3D::try_to_arrow_opt(rotation, None::<&str>)?
                        }
                    },
                    {
                        let (somes, scale): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| {
                                        let Self { scale, .. } = &**datum;
                                        scale.clone()
                                    })
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let scale_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = scale_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::Scale3D::try_to_arrow_opt(scale, None::<&str>)?
                        }
                    },
                    {
                        let (somes, from_parent): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { from_parent, .. } = &**datum;
                                    from_parent.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let from_parent_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        BooleanArray::new(
                            {
                                _ = extension_wrapper;
                                DataType::Boolean.to_logical_type().clone()
                            },
                            from_parent
                                .into_iter()
                                .map(|v| v.unwrap_or_default())
                                .collect(),
                            from_parent_bitmap,
                        )
                        .boxed()
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        use ::fallible_iterator::{FallibleIterator as _, IteratorExt as _};
        Ok({
            { let data = data . as_any () . downcast_ref :: < :: arrow2 :: array :: StructArray > () . ok_or_else (|| crate :: DeserializationError :: DatatypeMismatch { expected : data . data_type () . clone () , got : data . data_type () . clone () , backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) . map_err (| err | crate :: DeserializationError :: Context { location : "rerun.datatypes.TranslationRotationScale3D" . into () , source : Box :: new (err) , }

) ? ; let (data_fields , data_arrays , data_bitmap) = (data . fields () , data . values () , data . validity ()) ; let is_valid = move | i | data_bitmap . map_or (true , | bitmap | bitmap . get_bit (i)) ; let arrays_by_name : :: std :: collections :: HashMap < _ , _ > = data_fields . iter () . map (| field | field . name . as_str ()) . zip (data_arrays) . collect () ; let translation = { let data = & * * arrays_by_name ["translation"];

 { let data = data . as_any () . downcast_ref :: < :: arrow2 :: array :: FixedSizeListArray > () . unwrap () ; let bitmap = data . validity () . cloned () ; let offsets = (0 ..) . step_by (3usize) . zip ((3usize ..) . step_by (3usize) . take (data . len ())) ; let data = & * * data . values () ; let data = data . as_any () . downcast_ref :: < Float32Array > () . unwrap () . into_iter () . map (| v | v . copied ()) . map (Ok) . transpose_into_fallible :: < _ , crate :: DeserializationError > () . map (| v | v . ok_or_else (|| crate :: DeserializationError :: MissingData { backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

)) . collect :: < Vec < _ >> () . unwrap () ; offsets . enumerate () . map (move | (i , (start , end)) | bitmap . as_ref () . map_or (true , | bitmap | bitmap . get_bit (i)) . then (|| { if end as usize > data . len () { return Err (crate :: DeserializationError :: OffsetsMismatch { bounds : (start as usize , end as usize) , len : data . len () , backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) ; }

 # [allow (unsafe_code , clippy :: undocumented_unsafe_blocks)] let data = unsafe { data . get_unchecked (start as usize .. end as usize) }

 ; let arr = array_init :: from_iter (data . iter () . copied ()) . unwrap () ; Ok (arr) }

) . transpose ()) . map (| res | res . map (| opt | opt . map (| v | crate :: datatypes :: Vec3D (v)))) . transpose_into_fallible :: < _ , crate :: DeserializationError > () }

 }

 ; let rotation = { let data = & * * arrays_by_name ["rotation"];

 crate :: datatypes :: Rotation3D :: try_from_arrow_opt (data) . unwrap () . into_iter () . map (Ok) . transpose_into_fallible :: < _ , crate :: DeserializationError > () }

 ; let scale = { let data = & * * arrays_by_name ["scale"];

 crate :: datatypes :: Scale3D :: try_from_arrow_opt (data) . unwrap () . into_iter () . map (Ok) . transpose_into_fallible :: < _ , crate :: DeserializationError > () }

 ; let from_parent = { let data = & * * arrays_by_name ["from_parent"];

 data . as_any () . downcast_ref :: < BooleanArray > () . unwrap () . into_iter () . map (Ok) . transpose_into_fallible :: < _ , crate :: DeserializationError > () }

 ; crate :: izip ! (translation , rotation , scale , from_parent) . enumerate () . map (move | (i , (translation , rotation , scale , from_parent)) | is_valid (i) . then (|| Ok (Self { translation , rotation , scale , from_parent : from_parent . ok_or_else (|| crate :: DeserializationError :: MissingData { backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) . map_err (| err | crate :: DeserializationError :: Context { location : "rerun.datatypes.TranslationRotationScale3D#from_parent" . into () , source : Box :: new (err) , }

) ? , }

)) . transpose ()) }

 . collect :: < Vec < Option < _ >> > () . map_err (| err | crate :: DeserializationError :: Context { location : "rerun.datatypes.TranslationRotationScale3D" . into () , source : Box :: new (err) , }

) ?
        })
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        use ::fallible_iterator::{FallibleIterator as _, IteratorExt as _};
        Ok(Box::new({
            {
                let data = data
                    .as_any()
                    .downcast_ref::<::arrow2::array::StructArray>()
                    .ok_or_else(|| crate::DeserializationError::DatatypeMismatch {
                        expected: data.data_type().clone(),
                        got: data.data_type().clone(),
                        backtrace: ::backtrace::Backtrace::new_unresolved(),
                    })
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.datatypes.TranslationRotationScale3D".into(),
                        source: Box::new(err),
                    })?;
                let (data_fields, data_arrays, data_bitmap) =
                    (data.fields(), data.values(), data.validity());
                let is_valid = move |i| data_bitmap.map_or(true, |bitmap| bitmap.get_bit(i));
                let arrays_by_name: ::std::collections::HashMap<_, _> = data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(data_arrays)
                    .collect();
                let translation = {
                    let data = &**arrays_by_name["translation"];

                    {
                        let data = data
                            .as_any()
                            .downcast_ref::<::arrow2::array::FixedSizeListArray>()
                            .unwrap();
                        let bitmap = data.validity().cloned();
                        let offsets = (0..)
                            .step_by(3usize)
                            .zip((3usize..).step_by(3usize).take(data.len()));
                        let data = &**data.values();
                        let data = data
                            .as_any()
                            .downcast_ref::<Float32Array>()
                            .unwrap()
                            .into_iter()
                            .map(|v| v.copied())
                            .map(Ok)
                            .transpose_into_fallible::<_, crate::DeserializationError>()
                            .map(|v| {
                                v.ok_or_else(|| crate::DeserializationError::MissingData {
                                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                                })
                            })
                            .collect::<Vec<_>>()
                            .unwrap();
                        offsets
                            .enumerate()
                            .map(move |(i, (start, end))| {
                                bitmap
                                    .as_ref()
                                    .map_or(true, |bitmap| bitmap.get_bit(i))
                                    .then(|| {
                                        if end as usize > data.len() {
                                            return Err(
                                                crate::DeserializationError::OffsetsMismatch {
                                                    bounds: (start as usize, end as usize),
                                                    len: data.len(),
                                                    backtrace:
                                                        ::backtrace::Backtrace::new_unresolved(),
                                                },
                                            );
                                        }

                                        #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                        let data = unsafe {
                                            data.get_unchecked(start as usize..end as usize)
                                        };
                                        let arr =
                                            array_init::from_iter(data.iter().copied()).unwrap();
                                        Ok(arr)
                                    })
                                    .transpose()
                            })
                            .map(|res| res.map(|opt| opt.map(|v| crate::datatypes::Vec3D(v))))
                            .transpose_into_fallible::<_, crate::DeserializationError>()
                    }
                };
                let rotation = {
                    let data = &**arrays_by_name["rotation"];

                    crate::datatypes::Rotation3D::try_from_arrow_opt(data)
                        .unwrap()
                        .into_iter()
                        .map(Ok)
                        .transpose_into_fallible::<_, crate::DeserializationError>()
                };
                let scale = {
                    let data = &**arrays_by_name["scale"];

                    crate::datatypes::Scale3D::try_from_arrow_opt(data)
                        .unwrap()
                        .into_iter()
                        .map(Ok)
                        .transpose_into_fallible::<_, crate::DeserializationError>()
                };
                let from_parent = {
                    let data = &**arrays_by_name["from_parent"];

                    data.as_any()
                        .downcast_ref::<BooleanArray>()
                        .unwrap()
                        .into_iter()
                        .map(Ok)
                        .transpose_into_fallible::<_, crate::DeserializationError>()
                };
                crate :: izip ! (translation , rotation , scale , from_parent) . enumerate () . map (move | (i , (translation , rotation , scale , from_parent)) | is_valid (i) . then (|| Ok (Self { translation , rotation , scale , from_parent : from_parent . ok_or_else (|| crate :: DeserializationError :: MissingData { backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) . map_err (| err | crate :: DeserializationError :: Context { location : "rerun.datatypes.TranslationRotationScale3D#from_parent" . into () , source : Box :: new (err) , }

) ? , }

)) . transpose ())
            }
        }))
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Datatype for TranslationRotationScale3D {}
