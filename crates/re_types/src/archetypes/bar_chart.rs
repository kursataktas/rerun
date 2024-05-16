// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/bar_chart.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A bar chart.
///
/// The x values will be the indices of the array, and the bar heights will be the provided values.
///
/// ## Example
///
/// ### Simple bar chart
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_bar_chart").spawn()?;
///
///     rec.log(
///         "bar_chart",
///         &rerun::BarChart::new([8_i64, 4, 0, 9, 1, 4, 1, 6, 9, 0].as_slice()),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/barchart_simple/cf6014b18265edfcaa562c06526c0716b296b193/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/barchart_simple/cf6014b18265edfcaa562c06526c0716b296b193/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/barchart_simple/cf6014b18265edfcaa562c06526c0716b296b193/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/barchart_simple/cf6014b18265edfcaa562c06526c0716b296b193/1200w.png">
///   <img src="https://static.rerun.io/barchart_simple/cf6014b18265edfcaa562c06526c0716b296b193/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct BarChart {
    /// The values. Should always be a rank-1 tensor.
    pub values: crate::components::TensorData,

    /// The color of the bar chart
    pub color: Option<crate::components::Color>,
}

impl ::re_types_core::SizeBytes for BarChart {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.values.heap_size_bytes() + self.color.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::TensorData>::is_pod() && <Option<crate::components::Color>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.TensorData".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.BarChartIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Color".into()]);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.TensorData".into(),
            "rerun.components.BarChartIndicator".into(),
            "rerun.components.Color".into(),
        ]
    });

impl BarChart {
    /// The total number of components in the archetype: 1 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`BarChart`] [`::re_types_core::Archetype`]
pub type BarChartIndicator = ::re_types_core::GenericIndicatorComponent<BarChart>;

impl ::re_types_core::Archetype for BarChart {
    type Indicator = BarChartIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.BarChart".into()
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: BarChartIndicator = BarChartIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let values = {
            let array = arrays_by_name
                .get("rerun.components.TensorData")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.BarChart#values")?;
            <crate::components::TensorData>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.BarChart#values")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.BarChart#values")?
        };
        let color = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            <crate::components::Color>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.BarChart#color")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { values, color })
    }
}

impl ::re_types_core::AsComponents for BarChart {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.values as &dyn ComponentBatch).into()),
            self.color
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl BarChart {
    /// Create a new `BarChart`.
    #[inline]
    pub fn new(values: impl Into<crate::components::TensorData>) -> Self {
        Self {
            values: values.into(),
            color: None,
        }
    }

    /// The color of the bar chart
    #[inline]
    pub fn with_color(mut self, color: impl Into<crate::components::Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}
