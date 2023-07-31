// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs"

#pragma once

#include "../datatypes/flattened_scalar.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

namespace rr {
    namespace datatypes {
        struct AffixFuzzer1 {
            std::optional<float> single_float_optional;

            std::string single_string_required;

            std::optional<std::string> single_string_optional;

            std::optional<std::vector<float>> many_floats_optional;

            std::vector<std::string> many_strings_required;

            std::optional<std::vector<std::string>> many_strings_optional;

            float flattened_scalar;

            rr::datatypes::FlattenedScalar almost_flattened_scalar;

            std::optional<bool> from_parent;

          public:
            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::StructBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::StructBuilder* builder, const AffixFuzzer1* elements, size_t num_elements
            );
        };
    } // namespace datatypes
} // namespace rr
