// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs"

#include "affix_fuzzer17.hpp"

#include "../datatypes/affix_fuzzer3.hpp"
#include "../rerun.hpp"

#include <arrow/api.h>

namespace rr {
    namespace components {
        const char* AffixFuzzer17::NAME = "rerun.testing.components.AffixFuzzer17";

        const std::shared_ptr<arrow::DataType>& AffixFuzzer17::to_arrow_datatype() {
            static const auto datatype = arrow::list(arrow::field(
                "item",
                rr::datatypes::AffixFuzzer3::to_arrow_datatype(),
                true,
                nullptr
            ));
            return datatype;
        }

        arrow::Result<std::shared_ptr<arrow::ListBuilder>> AffixFuzzer17::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (!memory_pool) {
                return arrow::Status::Invalid("Memory pool is null.");
            }

            return arrow::Result(std::make_shared<arrow::ListBuilder>(
                memory_pool,
                rr::datatypes::AffixFuzzer3::new_arrow_array_builder(memory_pool).ValueOrDie()
            ));
        }

        arrow::Status AffixFuzzer17::fill_arrow_array_builder(
            arrow::ListBuilder* builder, const AffixFuzzer17* elements, size_t num_elements
        ) {
            if (!builder) {
                return arrow::Status::Invalid("Passed array builder is null.");
            }
            if (!elements) {
                return arrow::Status::Invalid("Cannot serialize null pointer to arrow array.");
            }

            return arrow::Status::NotImplemented(
                "TODO(andreas): custom data types in lists/fixedsizelist are not yet implemented"
            );

            return arrow::Status::OK();
        }

        arrow::Result<rr::DataCell> AffixFuzzer17::to_data_cell(
            const AffixFuzzer17* instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool* pool = arrow::default_memory_pool();

            ARROW_ASSIGN_OR_RAISE(auto builder, AffixFuzzer17::new_arrow_array_builder(pool));
            if (instances && num_instances > 0) {
                ARROW_RETURN_NOT_OK(
                    AffixFuzzer17::fill_arrow_array_builder(builder.get(), instances, num_instances)
                );
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            auto schema = arrow::schema(
                {arrow::field(AffixFuzzer17::NAME, AffixFuzzer17::to_arrow_datatype(), false)}
            );

            rr::DataCell cell;
            cell.component_name = AffixFuzzer17::NAME;
            ARROW_ASSIGN_OR_RAISE(
                cell.buffer,
                rr::ipc_from_table(*arrow::Table::Make(schema, {array}))
            );

            return cell;
        }
    } // namespace components
} // namespace rr
