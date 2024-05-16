// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#pragma once

#include <cstdint>
#include <memory>
#include <rerun/result.hpp>
#include <string>
#include <utility>

namespace arrow {
    class Array;
    class DataType;
    class StringBuilder;
} // namespace arrow

namespace rerun::components {
    struct AffixFuzzer9 {
        std::string single_string_required;

      public:
        AffixFuzzer9() = default;

        AffixFuzzer9(std::string single_string_required_)
            : single_string_required(std::move(single_string_required_)) {}

        AffixFuzzer9& operator=(std::string single_string_required_) {
            single_string_required = std::move(single_string_required_);
            return *this;
        }
    };
} // namespace rerun::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<components::AffixFuzzer9> {
        static constexpr const char Name[] = "rerun.testing.components.AffixFuzzer9";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::components::AffixFuzzer9` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::AffixFuzzer9* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StringBuilder* builder, const components::AffixFuzzer9* elements,
            size_t num_elements
        );
    };
} // namespace rerun
