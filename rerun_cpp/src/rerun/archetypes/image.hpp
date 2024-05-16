// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/image.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../components/draw_order.hpp"
#include "../components/tensor_data.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A monochrome or color image.
    ///
    /// The order of dimensions in the underlying `TensorData` follows the typical
    /// row-major, interleaved-pixel image format. Additionally, Rerun orders the
    /// `TensorDimension`s within the shape description from outer-most to inner-most.
    ///
    /// As such, the shape of the `TensorData` must be mappable to:
    /// - A `HxW` tensor, treated as a grayscale image.
    /// - A `HxWx3` tensor, treated as an RGB image.
    /// - A `HxWx4` tensor, treated as an RGBA image.
    ///
    /// Leading and trailing unit-dimensions are ignored, so that
    /// `1x480x640x3x1` is treated as a `480x640x3` RGB image.
    ///
    /// Rerun also supports compressed image encoded as JPEG, N12, and YUY2.
    /// Using these formats can save a lot of bandwidth and memory.
    /// See [`rerun::datatypes::TensorBuffer`] for more.
    ///
    /// Since the underlying `rerun::datatypes::TensorData` uses `rerun::Collection` internally,
    /// data can be passed in without a copy from raw pointers or by reference from `std::vector`/`std::array`/c-arrays.
    /// If needed, this "borrow-behavior" can be extended by defining your own `rerun::CollectionAdapter`.
    ///
    /// ## Example
    ///
    /// ### image_simple:
    /// ![image](https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <vector>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_image");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Create a synthetic image.
    ///     const int HEIGHT = 200;
    ///     const int WIDTH = 300;
    ///     std::vector<uint8_t> data(WIDTH * HEIGHT * 3, 0);
    ///     for (size_t i = 0; i <data.size(); i += 3) {
    ///         data[i] = 255;
    ///     }
    ///     for (size_t y = 50; y <150; ++y) {
    ///         for (size_t x = 50; x <150; ++x) {
    ///             data[(y * WIDTH + x) * 3 + 0] = 0;
    ///             data[(y * WIDTH + x) * 3 + 1] = 255;
    ///             data[(y * WIDTH + x) * 3 + 2] = 0;
    ///         }
    ///     }
    ///
    ///     rec.log("image", rerun::Image({HEIGHT, WIDTH, 3}, data));
    /// }
    /// ```
    struct Image {
        /// The image data. Should always be a rank-2 or rank-3 tensor.
        rerun::components::TensorData data;

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        std::optional<rerun::components::DrawOrder> draw_order;

      public:
        static constexpr const char IndicatorComponentName[] = "rerun.components.ImageIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        // Extensions to generated type defined in 'image_ext.cpp'

        /// New Image from height/width/channel and tensor buffer.
        ///
        /// \param shape
        /// Shape of the image. Calls `Error::handle()` if the shape is not rank 2 or 3.
        /// Sets the dimension names to "height", "width" and "channel" if they are not specified.
        /// \param buffer
        /// The tensor buffer containing the image data.
        explicit Image(Collection<datatypes::TensorDimension> shape, datatypes::TensorBuffer buffer)
            : Image(datatypes::TensorData(std::move(shape), std::move(buffer))) {}

        /// New depth image from tensor data.
        ///
        /// \param data_
        /// The tensor buffer containing the image data.
        /// Sets the dimension names to "height",  "width" and "channel" if they are not specified.
        /// Calls `Error::handle()` if the shape is not rank 2 or 3.
        explicit Image(rerun::components::TensorData data_);

        /// New image from dimensions and pointer to image data.
        ///
        /// Type must be one of the types supported by `rerun::datatypes::TensorData`.
        /// \param shape
        /// Shape of the image. Calls `Error::handle()` if the shape is not rank 2 or 3.
        /// Sets the dimension names to "height", "width" and "channel" if they are not specified.
        /// Determines the number of elements expected to be in `data`.
        /// \param data_
        /// Target of the pointer must outlive the archetype.
        template <typename TElement>
        explicit Image(Collection<datatypes::TensorDimension> shape, const TElement* data_)
            : Image(datatypes::TensorData(std::move(shape), data_)) {}

      public:
        Image() = default;
        Image(Image&& other) = default;

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        Image with_draw_order(rerun::components::DrawOrder _draw_order) && {
            draw_order = std::move(_draw_order);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::Image> {
        /// Serialize all set component batches.
        static Result<std::vector<DataCell>> serialize(const archetypes::Image& archetype);
    };
} // namespace rerun
