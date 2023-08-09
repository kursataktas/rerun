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

/// A line strip in 3D space.
///
/// A line strip is a list of points connected by line segments. It can be used to draw
/// approximations of smooth curves.
///
/// The points will be connected in order, like so:
/// ```text
///        2------3     5
///       /        \   /
/// 0----1          \ /
///                  4
/// ```
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct LineStrip3D(pub Vec<crate::datatypes::Vec3D>);

impl<I: Into<crate::datatypes::Vec3D>, T: IntoIterator<Item = I>> From<T> for LineStrip3D {
    fn from(v: T) -> Self {
        Self(v.into_iter().map(|v| v.into()).collect())
    }
}

impl<'a> From<LineStrip3D> for ::std::borrow::Cow<'a, LineStrip3D> {
    #[inline]
    fn from(value: LineStrip3D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a LineStrip3D> for ::std::borrow::Cow<'a, LineStrip3D> {
    #[inline]
    fn from(value: &'a LineStrip3D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for LineStrip3D {
    type Name = crate::ComponentName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = Box<
        dyn ::fallible_iterator::FallibleIterator<
                Item = Self::Item<'a>,
                Error = crate::DeserializationError,
            > + 'a,
    >;
    #[inline]
    fn name() -> Self::Name {
        "rerun.linestrip3d".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::List(Box::new(Field {
            name: "item".to_owned(),
            data_type: <crate::datatypes::Vec3D>::to_arrow_datatype(),
            is_nullable: false,
            metadata: [].into(),
        }))
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
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                let data0_inner_data: Vec<_> = data0
                    .iter()
                    .flatten()
                    .flatten()
                    .cloned()
                    .map(Some)
                    .collect();
                let data0_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                let offsets = ::arrow2::offset::Offsets::<i32>::try_from_lengths(
                    data0
                        .iter()
                        .map(|opt| opt.as_ref().map(|datum| datum.len()).unwrap_or_default()),
                )
                .unwrap()
                .into();
                ListArray::new(
                    {
                        _ = extension_wrapper;
                        DataType::Extension(
                            "rerun.components.LineStrip3D".to_owned(),
                            Box::new(DataType::List(Box::new(Field {
                                name: "item".to_owned(),
                                data_type: <crate::datatypes::Vec3D>::to_arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            }))),
                            None,
                        )
                        .to_logical_type()
                        .clone()
                    },
                    offsets,
                    {
                        use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                        let data0_inner_data_inner_data: Vec<_> = data0_inner_data
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
                        let data0_inner_data_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                        FixedSizeListArray::new(
                            {
                                _ = extension_wrapper;
                                DataType::Extension(
                                    "rerun.components.LineStrip3D".to_owned(),
                                    Box::new(DataType::FixedSizeList(
                                        Box::new(Field {
                                            name: "item".to_owned(),
                                            data_type: DataType::Float32,
                                            is_nullable: false,
                                            metadata: [].into(),
                                        }),
                                        3usize,
                                    )),
                                    None,
                                )
                                .to_logical_type()
                                .clone()
                            },
                            PrimitiveArray::new(
                                {
                                    _ = extension_wrapper;
                                    DataType::Extension(
                                        "rerun.components.LineStrip3D".to_owned(),
                                        Box::new(DataType::Float32),
                                        None,
                                    )
                                    .to_logical_type()
                                    .clone()
                                },
                                data0_inner_data_inner_data
                                    .into_iter()
                                    .map(|v| v.unwrap_or_default())
                                    .collect(),
                                data0_inner_data_inner_bitmap,
                            )
                            .boxed(),
                            data0_inner_bitmap,
                        )
                        .boxed()
                    },
                    data0_bitmap,
                )
                .boxed()
            }
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
            {
                let data = data
                    .as_any()
                    .downcast_ref::<::arrow2::array::ListArray<i32>>()
                    .unwrap();
                let bitmap = data.validity().cloned();
                let offsets = {
                    let offsets = data.offsets();
                    offsets.iter().copied().zip(offsets.iter().copied().skip(1))
                };
                let data = &**data.values();
                let data = {
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
                                        return Err(crate::DeserializationError::OffsetsMismatch {
                                            bounds: (start as usize, end as usize),
                                            len: data.len(),
                                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                                        });
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data =
                                        unsafe { data.get_unchecked(start as usize..end as usize) };
                                    let arr = array_init::from_iter(data.iter().copied()).unwrap();
                                    Ok(arr)
                                })
                                .transpose()
                        })
                        .map(|res| res.map(|opt| opt.map(|v| crate::datatypes::Vec3D(v))))
                        .transpose_into_fallible::<_, crate::DeserializationError>()
                }
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
                                    return Err(crate::DeserializationError::OffsetsMismatch {
                                        bounds: (start as usize, end as usize),
                                        len: data.len(),
                                        backtrace: ::backtrace::Backtrace::new_unresolved(),
                                    });
                                }

                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                let data = unsafe {
                                    data.get_unchecked(start as usize..end as usize).to_vec()
                                };
                                Ok(data)
                            })
                            .transpose()
                    })
                    .transpose_into_fallible::<_, crate::DeserializationError>()
            }
            .map(|v| {
                v.ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
            })
            .map(|v| Ok(Some(Self(v))))
            .collect::<Vec<Option<_>>>()
            .map_err(|err| crate::DeserializationError::Context {
                location: "rerun.linestrip3d".into(),
                source: Box::new(err),
            })?
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
                    .downcast_ref::<::arrow2::array::ListArray<i32>>()
                    .unwrap();
                let bitmap = data.validity().cloned();
                let offsets = {
                    let offsets = data.offsets();
                    offsets.iter().copied().zip(offsets.iter().copied().skip(1))
                };
                let data = &**data.values();
                let data = {
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
                                        return Err(crate::DeserializationError::OffsetsMismatch {
                                            bounds: (start as usize, end as usize),
                                            len: data.len(),
                                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                                        });
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data =
                                        unsafe { data.get_unchecked(start as usize..end as usize) };
                                    let arr = array_init::from_iter(data.iter().copied()).unwrap();
                                    Ok(arr)
                                })
                                .transpose()
                        })
                        .map(|res| res.map(|opt| opt.map(|v| crate::datatypes::Vec3D(v))))
                        .transpose_into_fallible::<_, crate::DeserializationError>()
                }
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
                                    return Err(crate::DeserializationError::OffsetsMismatch {
                                        bounds: (start as usize, end as usize),
                                        len: data.len(),
                                        backtrace: ::backtrace::Backtrace::new_unresolved(),
                                    });
                                }

                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                let data = unsafe {
                                    data.get_unchecked(start as usize..end as usize).to_vec()
                                };
                                Ok(data)
                            })
                            .transpose()
                    })
                    .transpose_into_fallible::<_, crate::DeserializationError>()
            }
            .map(|v| {
                v.ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
            })
            .map(|v| Ok(Some(Self(v))))
        }))
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Component for LineStrip3D {}
