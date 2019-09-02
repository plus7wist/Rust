use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

/// 'Graph<N, E, Ty>' is a data structure for graphs with optionally
/// weighted nodes and edges of arbitrary type. This implementation
/// is inteded for educational purposes only.
/// See [petgraph](https://docs.rs/crate/petgraph/) for a full featured
/// implementation.
#[derive(Debug, PartialEq, Clone)]
pub struct Graph<N, E, Ty = Directed> {
    nodes: FreeMap<Node<N>>,
    edges: FreeMap<Edge<E>>,
    ty: PhantomData<Ty>,
}

impl<N, E, Ty> Graph<N, E, Ty>
where
    Ty: EdgeType,
{
    pub fn new() -> Self {
        Graph {
            nodes: FreeMap::new(),
            edges: FreeMap::new(),
            ty: PhantomData,
        }
    }

    /// Returns true iff graph is directed.
    pub fn is_directed(&self) -> bool {
        Ty::is_directed()
    }

    /// Add node to graph and return its assigned index.
    pub fn add_node(&mut self, weight: N) -> usize {
        self.nodes.insert(Node { weight })
    }

    /// Remove node indexed at 'index' and all edges containing that
    /// node. Optionally returns the weight of the removed node.
    pub fn remove_node(&mut self, index: usize) -> Option<N> {
        match self.nodes.remove(&index) {
            Some(node) => {
                // TODO Only keep those edges that don't contain index as head or tail
                self.edges
                    .map
                    .retain(|_, edge| edge.head != index && edge.tail != index);
                return Some(node.weight);
            }
            None => {
                return None;
            }
        }
    }

    /// Given two indices head and tail, attempt to add an edge whose
    /// starting node has index head and whose ending node has index
    /// tail to the graph. If this succeeds, return index assigned to
    /// that edge. Otherwise return None.
    pub fn add_edge(&mut self, weight: E, head: usize, tail: usize) -> Option<usize> {
        if !self.nodes.contains_key(&head) || !self.nodes.contains_key(&tail) {
            None
        } else {
            Some(self.edges.insert(Edge { weight, head, tail }))
        }
    }

    /// Remove edge indexed at 'index'
    pub fn remove_edge(&mut self, index: usize) {
        self.edges.remove(&index);
    }

    /// Return first index of an edge of the form head --> tail.  This
    /// is not necessarily the unique index since we explicitly allow
    /// multiple edges between two nodes.
    pub fn find_edge(&self, head: usize, tail: usize) -> Option<usize> {
        let edges: Vec<usize> = self.find_n_edges(1, head, tail);
        if edges.len() == 0 {
            return None;
        } else {
            return Some(edges[0]);
        }
    }

    /// Return the indices of all edges of the form head --> tail
    pub fn find_edges(&self, head: usize, tail: usize) -> Vec<usize> {
        self.find_n_edges(0, head, tail)
    }

    /// Returns up to the first n edges of the form head --> tail. If
    /// n == 0, it returns all available edges.
    pub fn find_n_edges(&self, n: usize, head: usize, tail: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        for (index, edge) in self.edges.map.iter() {
            if [edge.head, edge.tail] == [head, tail]
                || (self.is_directed() && [edge.head, edge.tail] == [tail, head])
            {
                result.push(*index);
            }
            if n != 0 && result.len() == n {
                break;
            }
        }
        result.sort();
        result
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

/// The graph's node type.
#[derive(Debug, PartialEq, Clone)]
pub struct Node<N> {
    // node data
    pub weight: N,
}

/// The graph's edge type.
#[derive(Debug, PartialEq, Clone)]
pub struct Edge<E> {
    // edge data
    pub weight: E,

    // indeces of head and tail
    head: usize,
    tail: usize,
}

pub trait EdgeType {
    fn is_directed() -> bool;
}

/// Marker for directed graphs
pub enum Directed {}

impl EdgeType for Directed {
    fn is_directed() -> bool {
        true
    }
}

/// Marker for undirected graphs
pub enum Undirected {}

impl EdgeType for Undirected {
    fn is_directed() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_node() {
        let mut graph: Graph<u32, u32> = Graph::new();
        // test if the new node has been given index 0
        assert_eq!(graph.add_node(5), 0);

        // test if there is a unique element in graph.nodes
        assert_eq!(graph.node_count(), 1);

        // test if graph.nodes contains a node with index 0
        assert_eq!(graph.nodes.contains_key(&0), true);

        // test if the node with index 0 has weight 5
        assert_eq!(graph.nodes.get(&0).unwrap().weight, 5);
    }

    #[test]
    fn add_edge() {
        let mut graph: Graph<(), String> = Graph::new();
        // Since we don't care about the weights of the nodes, we use
        // the unit type here. Unforunately, since there are no
        // optional arguments in Rust functions, we have to insert it
        // as an argument below as well.
        let start = graph.add_node(());
        let end = graph.add_node(());
        graph.add_edge(String::from("edge"), start, end);

        // Test if graph contains a unique edge.
        assert_eq!(graph.edge_count(), 1);

        // Test if graph's unique edge has weight 3.
        assert_eq!(graph.edges.get(&0).unwrap().weight, String::from("edge"));

        // Test if adding an illegal edge results in None
        assert_eq!(graph.add_edge(String::from("None"), 2, 3), None);
    }

    #[test]
    fn find_edge() {
        let mut graph: Graph<(), String> = Graph::new();
        let start_0 = graph.add_node(());
        let end_0 = graph.add_node(());
        let index_0 = graph
            .add_edge(String::from("edge 0"), start_0, end_0)
            .unwrap();

        let start_1 = graph.add_node(());
        let end_1 = graph.add_node(());
        let index_1 = graph
            .add_edge(String::from("edge 1"), start_1, end_1)
            .unwrap();

        assert_eq!(graph.find_edge(start_1, end_1).unwrap(), index_1);
        assert_eq!(graph.find_edge(start_0, end_0).unwrap(), index_0);
    }

    #[test]
    fn find_edges() {
        let mut graph: Graph<(), String> = Graph::new();
        let start = graph.add_node(());
        let end = graph.add_node(());
        let index_0 = graph.add_edge(String::from("edge 0"), start, end).unwrap();
        let index_1 = graph.add_edge(String::from("edge 1"), start, end).unwrap();

        // test if graph contains two distinct edges of the form
        // start --> end
        let mut indices: Vec<usize> = Vec::new();
        indices.push(index_0);
        indices.push(index_1);
        assert_eq!(graph.find_edges(start, end), indices);
    }

    #[test]
    fn remove_node() {
        let mut graph: Graph<u32, String> = Graph::new();
        let start = graph.add_node(2);
        let end = graph.add_node(3);
        graph
            .add_edge(String::from("start --> end"), start, end)
            .unwrap();
        graph
            .add_edge(String::from("end --> start"), start, end)
            .unwrap();
        graph
            .add_edge(String::from("start --> start"), start, start)
            .unwrap();
        let index = graph
            .add_edge(String::from("end --> end"), end, end)
            .unwrap();

        // Test if the weight of the removed node has been returned
        assert_eq!(graph.remove_node(start).unwrap(), 2);
        // Test if there is a unique remaining edge.
        assert_eq!(graph.edges.len(), 1);
        // Test if the unique remaining edge is the correct one.
        assert_eq!(
            *graph.edges.get(&index).unwrap(),
            Edge {
                weight: String::from("end --> end"),
                head: end,
                tail: end
            }
        );
    }
}

#[derive(PartialEq, Debug, Clone)]
struct FreeMap<Data> {
    pub map: HashMap<usize, Data>,
    free: HashSet<usize>,
}

impl<Data> FreeMap<Data> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            free: HashSet::new(),
        }
    }

    fn insert(&mut self, data: Data) -> usize {
        let mut index = self.map.len();

        self.free.iter().next().map(|free| *free).map(|free| {
            index = free;
            let _ = self.free.remove(&free);
        });
        self.map.insert(index, data);

        index
    }

    fn contains_key(&self, index: &usize) -> bool {
        self.map.contains_key(index)
    }

    #[allow(dead_code)]
    fn get(&self, index: &usize) -> Option<&Data> {
        self.map.get(index)
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn remove(&mut self, index: &usize) -> Option<Data> {
        if let Some(data) = self.map.remove(&index) {
            self.free.insert(*index);
            Some(data)
        } else {
            None
        }
    }
}
