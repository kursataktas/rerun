// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/material.fbs".

#pragma once

#include "../result.hpp"
#include "rgba32.hpp"

#include <cstdint>
#include <memory>
#include <optional>

namespace arrow {
    class Array;
    class DataType;
    class StructBuilder;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: Material properties of a mesh.
    struct Material {
        /// Optional color multiplier.
        std::optional<rerun::datatypes::Rgba32> albedo_factor;

      public:
        Material() = default;

        Material(std::optional<rerun::datatypes::Rgba32> albedo_factor_)
            : albedo_factor(albedo_factor_) {}

        Material& operator=(std::optional<rerun::datatypes::Rgba32> albedo_factor_) {
            albedo_factor = albedo_factor_;
            return *this;
        }

        Material(uint32_t rgba_) : albedo_factor(rgba_) {}

        Material& operator=(uint32_t rgba_) {
            albedo_factor = rgba_;
            return *this;
        }
    };
} // namespace rerun::datatypes

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<datatypes::Material> {
        static constexpr const char Name[] = "rerun.datatypes.Material";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::datatypes::Material` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const datatypes::Material* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StructBuilder* builder, const datatypes::Material* elements, size_t num_elements
        );
    };
} // namespace rerun
