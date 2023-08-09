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

/// Draw order used for the display order of 2D elements.
///
/// Higher values are drawn on top of lower values.
/// An entity can have only a single draw order component.
/// Within an entity draw order is governed by the order of the components.
///
/// Draw order for entities with the same draw order is generally undefined.
#[derive(Clone, Debug, Copy)]
#[repr(transparent)]
pub struct DrawOrder(pub f32);

impl<'a> From<DrawOrder> for ::std::borrow::Cow<'a, DrawOrder> {
    #[inline]
    fn from(value: DrawOrder) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a DrawOrder> for ::std::borrow::Cow<'a, DrawOrder> {
    #[inline]
    fn from(value: &'a DrawOrder) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for DrawOrder {
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
        "rerun.draw_order".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Float32
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
            PrimitiveArray::new(
                {
                    _ = extension_wrapper;
                    DataType::Extension(
                        "rerun.components.DrawOrder".to_owned(),
                        Box::new(DataType::Float32),
                        None,
                    )
                    .to_logical_type()
                    .clone()
                },
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
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
            data.as_any()
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
                .map(|v| Ok(Some(Self(v))))
                .collect::<Vec<Option<_>>>()
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.draw_order".into(),
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
            data.as_any()
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
                .map(|v| Ok(Some(Self(v))))
        }))
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Component for DrawOrder {}
