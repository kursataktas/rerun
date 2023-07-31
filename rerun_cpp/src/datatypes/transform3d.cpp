// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/transform3d.fbs"

#include "transform3d.hpp"

#include "../datatypes/translation_and_mat3x3.hpp"
#include "../datatypes/translation_rotation_scale3d.hpp"

#include <arrow/api.h>

namespace rr {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType>& Transform3D::to_arrow_datatype() {
            static const auto datatype = arrow::dense_union({
                arrow::field("_null_markers", arrow::null(), true, nullptr),
                arrow::field(
                    "TranslationAndMat3x3",
                    rr::datatypes::TranslationAndMat3x3::to_arrow_datatype(),
                    false,
                    nullptr
                ),
                arrow::field(
                    "TranslationRotationScale",
                    rr::datatypes::TranslationRotationScale3D::to_arrow_datatype(),
                    false,
                    nullptr
                ),
            });
            return datatype;
        }

        arrow::Result<std::shared_ptr<arrow::DenseUnionBuilder>>
            Transform3D::new_arrow_array_builder(arrow::MemoryPool* memory_pool) {
            if (!memory_pool) {
                return arrow::Status::Invalid("Memory pool is null.");
            }

            return arrow::Result(std::make_shared<arrow::DenseUnionBuilder>(
                memory_pool,
                std::vector<std::shared_ptr<arrow::ArrayBuilder>>({
                    std::make_shared<arrow::NullBuilder>(memory_pool),
                    rr::datatypes::TranslationAndMat3x3::new_arrow_array_builder(memory_pool)
                        .ValueOrDie(),
                    rr::datatypes::TranslationRotationScale3D::new_arrow_array_builder(memory_pool)
                        .ValueOrDie(),
                }),
                to_arrow_datatype()
            ));
        }

        arrow::Status Transform3D::fill_arrow_array_builder(
            arrow::DenseUnionBuilder* builder, const Transform3D* elements, size_t num_elements
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
