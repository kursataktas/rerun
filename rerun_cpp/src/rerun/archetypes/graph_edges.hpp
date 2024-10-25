// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/graph_edges.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/graph_edge.hpp"
#include "../components/graph_type.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A list of edges in a graph.
    ///
    /// By default, edges are undirected.
    struct GraphEdges {
        /// A list of node IDs.
        Collection<rerun::components::GraphEdge> edges;

        /// Specifies if the graph is directed or undirected.
        std::optional<rerun::components::GraphType> graph_type;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.GraphEdgesIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        GraphEdges() = default;
        GraphEdges(GraphEdges&& other) = default;

        explicit GraphEdges(Collection<rerun::components::GraphEdge> _edges)
            : edges(std::move(_edges)) {}

        /// Specifies if the graph is directed or undirected.
        GraphEdges with_graph_type(rerun::components::GraphType _graph_type) && {
            graph_type = std::move(_graph_type);
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
    struct AsComponents<archetypes::GraphEdges> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(const archetypes::GraphEdges& archetype
        );
    };
} // namespace rerun
