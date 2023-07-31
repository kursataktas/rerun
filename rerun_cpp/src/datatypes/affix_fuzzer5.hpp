// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs"

#pragma once

#include "../datatypes/affix_fuzzer4.hpp"

#include <arrow/type_fwd.h>
#include <cstdint>
#include <optional>
#include <utility>

namespace rr {
    namespace datatypes {
        struct AffixFuzzer5 {
            std::optional<rr::datatypes::AffixFuzzer4> single_optional_union;

          public:
            AffixFuzzer5(std::optional<rr::datatypes::AffixFuzzer4> single_optional_union)
                : single_optional_union(std::move(single_optional_union)) {}

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& to_arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static arrow::Result<std::shared_ptr<arrow::StructBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static arrow::Status fill_arrow_array_builder(
                arrow::StructBuilder* builder, const AffixFuzzer5* elements, size_t num_elements
            );
        };
    } // namespace datatypes
} // namespace rr
