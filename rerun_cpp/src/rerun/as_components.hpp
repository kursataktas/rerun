#pragma once

#include "collection.hpp"
#include "indicator_component.hpp"
#include "serialized_component_batch.hpp"

namespace rerun {
    /// The AsComponents trait is used to convert a type into a list of serialized component.
    ///
    /// It is implemented for various built-in types as well as collections of components.
    /// You can build your own archetypes by implementing this trait.
    /// Anything that implements `AsComponents` can be logged to a recording stream.
    template <typename T>
    struct AsComponents {
        /// \private
        /// `NoAsComponentsFor` always evaluates to false, but in a way that requires template instantiation.
        template <typename T2>
        struct NoAsComponentsFor : std::false_type {};

        // TODO(andreas): This should also mention an example of how to implement this.
        static_assert(
            NoAsComponentsFor<T>::value,
            "AsComponents is not implemented for this type. "
            "It is implemented for all built-in archetypes as well as std::vector, std::array, and "
            "c-arrays of components. "
            "You can add your own implementation by specializing AsComponents<T> for your type T."
        );
    };

    // Documenting the builtin generic `AsComponents` impls is too much clutter for the doc class overview.
    /// \cond private

    /// AsComponents for a Collection of components.
    template <typename TComponent>
    struct AsComponents<Collection<TComponent>> {
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const Collection<TComponent>& components
        ) {
            auto cell_result = TComponent::to_data_cell(components.data(), components.size());
            RR_RETURN_NOT_OK(cell_result.error);

            return Result<std::vector<SerializedComponentBatch>>(
                {SerializedComponentBatch(std::move(cell_result.value), components.size())}
            );
        }
    };

    /// AsComponents for a std::vector of components.
    template <typename TComponent>
    struct AsComponents<std::vector<TComponent>> {
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const std::vector<TComponent>& components
        ) {
            return AsComponents<Collection<TComponent>>::serialize(components);
        }
    };

    /// AsComponents for std::initializer_list
    template <typename TComponent>
    struct AsComponents<std::initializer_list<TComponent>> {
        static Result<std::vector<SerializedComponentBatch>> serialize(
            std::initializer_list<TComponent> components
        ) {
            return AsComponents<Collection<TComponent>>::serialize(components);
        }
    };

    /// AsComponents for an std::array of components.
    template <typename TComponent, size_t NumInstances>
    struct AsComponents<std::array<TComponent, NumInstances>> {
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const std::array<TComponent, NumInstances>& components
        ) {
            return AsComponents<Collection<TComponent>>::serialize(components);
        }
    };

    /// AsComponents for an c-array of components.
    template <typename TComponent, size_t NumInstances>
    struct AsComponents<TComponent[NumInstances]> {
        static Result<std::vector<SerializedComponentBatch>> serialize(const TComponent (&components
        )[NumInstances]) {
            return AsComponents<Collection<TComponent>>::serialize(components);
        }
    };

    /// AsComponents for single indicators
    template <const char Name[]>
    struct AsComponents<components::IndicatorComponent<Name>> {
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const components::IndicatorComponent<Name>& indicator
        ) {
            return AsComponents<Collection<components::IndicatorComponent<Name>>>::serialize(
                indicator
            );
        }
    };

    /// \endcond
} // namespace rerun
