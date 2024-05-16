// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/enum_test.fbs".

#include "enum_test.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    const std::shared_ptr<arrow::DataType>& Loggable<components::EnumTest>::arrow_datatype() {
        static const auto datatype = arrow::sparse_union({
            arrow::field("_null_markers", arrow::null(), true, nullptr),
            arrow::field("Up", arrow::null(), true),
            arrow::field("Down", arrow::null(), true),
            arrow::field("Right", arrow::null(), true),
            arrow::field("Left", arrow::null(), true),
            arrow::field("Forward", arrow::null(), true),
            arrow::field("Back", arrow::null(), true),
        });
        return datatype;
    }

    Result<std::shared_ptr<arrow::Array>> Loggable<components::EnumTest>::to_arrow(
        const components::EnumTest* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();
        auto datatype = arrow_datatype();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(datatype, pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(Loggable<components::EnumTest>::fill_arrow_array_builder(
                static_cast<arrow::SparseUnionBuilder*>(builder.get()),
                instances,
                num_instances
            ));
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));
        return array;
    }

    rerun::Error Loggable<components::EnumTest>::fill_arrow_array_builder(
        arrow::SparseUnionBuilder* builder, const components::EnumTest* elements,
        size_t num_elements
    ) {
        if (builder == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
        }
        if (elements == nullptr) {
            return rerun::Error(
                ErrorCode::UnexpectedNullArgument,
                "Cannot serialize null pointer to arrow array."
            );
        }

        ARROW_RETURN_NOT_OK(builder->Reserve(static_cast<int64_t>(num_elements)));
        for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
            const auto variant = elements[elem_idx];
            ARROW_RETURN_NOT_OK(builder->Append(static_cast<int8_t>(variant)));
        }

        return Error::ok();
    }
} // namespace rerun
