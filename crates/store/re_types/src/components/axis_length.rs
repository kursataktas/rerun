// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/components/axis_length.fbs".

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

/// **Component**: The length of an axis in local units of the space.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AxisLength(pub crate::datatypes::Float32);

impl ::re_types_core::SizeBytes for AxisLength {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Float32>::is_pod()
    }
}

impl<T: Into<crate::datatypes::Float32>> From<T> for AxisLength {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Float32> for AxisLength {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Float32 {
        &self.0
    }
}

impl std::ops::Deref for AxisLength {
    type Target = crate::datatypes::Float32;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Float32 {
        &self.0
    }
}

impl std::ops::DerefMut for AxisLength {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Float32 {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(AxisLength);

impl ::re_types_core::Loggable for AxisLength {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.AxisLength".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        crate::datatypes::Float32::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Float32::to_arrow_opt(data.into_iter().map(|datum| {
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
        crate::datatypes::Float32::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow2::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::Float32::from_arrow(arrow_data).map(|v| v.into_iter().map(Self).collect())
    }
}
