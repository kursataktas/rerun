// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs"

#include "affix_fuzzer3.hpp"

#include "../datatypes/affix_fuzzer1.hpp"

#include <arrow/api.h>

namespace rr {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType>& AffixFuzzer3::to_arrow_datatype() {
            static const auto datatype = arrow::dense_union({
                arrow::field("_null_markers", arrow::null(), true, nullptr),
                arrow::field("degrees", arrow::float32(), false, nullptr),
                arrow::field("radians", arrow::float32(), false, nullptr),
                arrow::field(
                    "craziness",
                    arrow::list(arrow::field(
                        "item",
                        rr::datatypes::AffixFuzzer1::to_arrow_datatype(),
                        false,
                        nullptr
                    )),
                    false,
                    nullptr
                ),
                arrow::field(
                    "fixed_size_shenanigans",
                    arrow::fixed_size_list(
                        arrow::field("item", arrow::float32(), false, nullptr),
                        3
                    ),
                    false,
                    nullptr
                ),
            });
            return datatype;
        }

        arrow::Result<std::shared_ptr<arrow::DenseUnionBuilder>>
            AffixFuzzer3::new_arrow_array_builder(arrow::MemoryPool* memory_pool) {
            if (!memory_pool) {
                return arrow::Status::Invalid("Memory pool is null.");
            }

            return arrow::Result(std::make_shared<arrow::DenseUnionBuilder>(
                memory_pool,
                std::vector<std::shared_ptr<arrow::ArrayBuilder>>({
                    std::make_shared<arrow::NullBuilder>(memory_pool),
                    std::make_shared<arrow::FloatBuilder>(memory_pool),
                    std::make_shared<arrow::FloatBuilder>(memory_pool),
                    std::make_shared<arrow::ListBuilder>(
                        memory_pool,
                        rr::datatypes::AffixFuzzer1::new_arrow_array_builder(memory_pool)
                            .ValueOrDie()
                    ),
                    std::make_shared<arrow::FixedSizeListBuilder>(
                        memory_pool,
                        std::make_shared<arrow::FloatBuilder>(memory_pool),
                        3
                    ),
                }),
                to_arrow_datatype()
            ));
        }

        arrow::Status AffixFuzzer3::fill_arrow_array_builder(
            arrow::DenseUnionBuilder* builder, const AffixFuzzer3* elements, size_t num_elements
        ) {
            if (!builder) {
                return arrow::Status::Invalid("Passed array builder is null.");
            }
            if (!elements) {
                return arrow::Status::Invalid("Cannot serialize null pointer to arrow array.");
            }

            for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                const auto& element = elements[elem_idx];
            }
            return arrow::Status::NotImplemented("TODO(andreas): unions are not yet implemented");

            return arrow::Status::OK();
        }
    } // namespace datatypes
} // namespace rr
