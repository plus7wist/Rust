/// An edge type putted into a adjacency list.
pub trait AdjacencyEdge {
    type Weight;

    /// Label of the target node of this edge.
    fn target(&self) -> usize;
    fn weight(&self) -> Self::Weight;
}

/// An graph type contains adjacency list. Nodes in graph are labeled by `0..graph.node_count()`.
pub trait AdjacencyGraph<'a> {
    type Edge: AdjacencyEdge + 'a;
    type EdgeIter: Iterator<Item = &'a Self::Edge>;

    fn adjacencies(&'a self, u: usize) -> Self::EdgeIter;
    fn node_count(&self) -> usize;
}
