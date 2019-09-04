use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Edge<W> {
    weight: W,
    dst: usize,
}

impl<W> Edge<W> {
    pub fn new(weight: W, dst: usize) -> Self {
        Self { weight, dst }
    }
}

/// Adjacency list for a directed weighted graph.
pub type Graph<W> = Vec<Vec<Edge<W>>>;

/// Dijkstra's single source shortest path algorithm.
///
/// ## Example
///
/// ```
/// use the_algorithms_rust::data_structures::dijkstra::{self, Edge, Graph};
///
/// let mut graph: Graph<u32> = vec![vec![]; 6];
/// let mut add_edge = |u: usize, v: usize, w: u32| { graph[u].push(Edge::new(w, v)) };
///
/// // 0 --(2)--> 1 --(8)--> 3 --(3)--> 4
/// // |          ^          ^
/// // |          |          |
/// // |         (3)         |
/// // |          |          |
/// // |---(1)--> 2 --(20)---|          5
/// add_edge(0, 1, 2);
/// add_edge(0, 2, 1);
/// add_edge(2, 1, 3);
/// add_edge(2, 3, 20);
/// add_edge(1, 3, 8);
/// add_edge(3, 4, 3);
///
/// let dist = dijkstra::sssp(&graph, 0);
///
/// assert_eq!(dist[0], Some(0));
/// assert_eq!(dist[1], Some(2));
/// assert_eq!(dist[2], Some(1));
/// assert_eq!(dist[3], Some(10));
/// assert_eq!(dist[4], Some(13));
/// assert_eq!(dist[5], None);
/// ```
pub fn sssp<W>(graph: &Graph<W>, source: usize) -> Vec<Option<W>>
where
    W: Ord + Copy + Default + std::ops::Add<Output = W>,
{
    #[derive(Eq, PartialEq, Clone, Copy)]
    struct VetInSet<W> {
        dist: W,
        vet: usize,
    }

    impl<W> PartialOrd for VetInSet<W>
    where
        W: Ord,
    {
        fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
            Some(self.cmp(rhs))
        }
    }

    impl<W> Ord for VetInSet<W>
    where
        W: Ord,
    {
        fn cmp(&self, rhs: &Self) -> Ordering {
            self.dist.cmp(&rhs.dist)
        }
    }

    let n = graph.len();

    let mut set = BTreeSet::new();
    let mut dist = vec![None; n];

    dist[source] = Some(W::default());
    set.insert(VetInSet {
        dist: W::default(),
        vet: source,
    });

    while let Some(min) = set.iter().copied().next() {
        assert!(set.remove(&min));

        let u = min.vet;
        let udist = min.dist;
        for next in graph[u].iter() {
            let v = next.dst;
            let edge = next.weight;
            let alt = udist + edge; // Alt distance to `v`.

            let update = match dist[v] {
                None => true, // First reach `v`.
                Some(vdist) if alt < vdist => {
                    assert!(set.remove(&VetInSet {
                        dist: vdist,
                        vet: v
                    }));
                    true
                }
                Some(_) => false,
            };

            if update {
                dist[v] = Some(alt);
                set.insert(VetInSet { dist: alt, vet: v });
            }
        }
    }

    dist
}

pub trait AdjacencyEdge {
    type Weight;

    fn target(&self) -> usize;
    fn weight(&self) -> Self::Weight;
}

pub trait AdjacencyGraph<'a> {
    type Edge: AdjacencyEdge + 'a;
    type EdgeIter: Iterator<Item = &'a Self::Edge>;

    fn adjacencies(&'a self, u: usize) -> Self::EdgeIter;
    fn node_count(&self) -> usize;
}

impl<W> AdjacencyEdge for Edge<W>
where
    W: Copy,
{
    type Weight = W;

    fn target(&self) -> usize {
        self.dst
    }
    fn weight(&self) -> W {
        self.weight
    }
}

impl<'a, W: 'a> AdjacencyGraph<'a> for Graph<W>
where
    W: Copy,
{
    type Edge = Edge<W>;
    type EdgeIter = std::slice::Iter<'a, Edge<W>>;

    fn adjacencies(&'a self, u: usize) -> Self::EdgeIter {
        self[u].iter()
    }

    fn node_count(&self) -> usize {
        self.len()
    }
}

/// ## Example
///
/// ```
/// use the_algorithms_rust::data_structures::dijkstra::{self, Edge, Graph};
///
/// let mut graph: Graph<u32> = vec![vec![]; 6];
/// let mut add_edge = |u: usize, v: usize, w: u32| { graph[u].push(Edge::new(w, v)) };
///
/// // 0 --(2)--> 1 --(8)--> 3 --(3)--> 4
/// // |          ^          ^
/// // |          |          |
/// // |         (3)         |
/// // |          |          |
/// // |---(1)--> 2 --(20)---|          5
/// add_edge(0, 1, 2);
/// add_edge(0, 2, 1);
/// add_edge(2, 1, 3);
/// add_edge(2, 3, 20);
/// add_edge(1, 3, 8);
/// add_edge(3, 4, 3);
///
/// let dist = dijkstra::sssp_trait(&graph, 0);
///
/// assert_eq!(dist[0], Some(0));
/// assert_eq!(dist[1], Some(2));
/// assert_eq!(dist[2], Some(1));
/// assert_eq!(dist[3], Some(10));
/// assert_eq!(dist[4], Some(13));
/// assert_eq!(dist[5], None);
/// ```
pub fn sssp_trait<'a, G, E: 'a, W>(graph: &'a G, source: usize) -> Vec<Option<W>>
where
    G: AdjacencyGraph<'a, Edge = E>,
    E: AdjacencyEdge<Weight = W>,
    W: Ord + Copy + Default + std::ops::Add<Output = W>,
{
    #[derive(Eq, PartialEq, Clone, Copy)]
    struct VetInSet<W> {
        dist: W,
        vet: usize,
    }

    impl<W> PartialOrd for VetInSet<W>
    where
        W: Ord,
    {
        fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
            Some(self.cmp(rhs))
        }
    }

    impl<W> Ord for VetInSet<W>
    where
        W: Ord,
    {
        fn cmp(&self, rhs: &Self) -> Ordering {
            self.dist.cmp(&rhs.dist)
        }
    }

    let n = graph.node_count();

    let mut set = BTreeSet::new();
    let mut dist = vec![None; n];

    dist[source] = Some(W::default());
    set.insert(VetInSet {
        dist: W::default(),
        vet: source,
    });

    while let Some(min) = set.iter().copied().next() {
        assert!(set.remove(&min));

        let u = min.vet;
        let udist = min.dist;
        for next in graph.adjacencies(u) {
            let v = next.target();
            let edge = next.weight();
            let alt = udist + edge; // Alt distance to `v`.

            let update = match dist[v] {
                None => true, // First reach `v`.
                Some(vdist) if alt < vdist => {
                    assert!(set.remove(&VetInSet {
                        dist: vdist,
                        vet: v
                    }));
                    true
                }
                Some(_) => false,
            };

            if update {
                dist[v] = Some(alt);
                set.insert(VetInSet { dist: alt, vet: v });
            }
        }
    }

    dist
}
