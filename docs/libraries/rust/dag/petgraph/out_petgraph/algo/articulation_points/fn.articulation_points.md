# Function articulation_points Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/articulation_points.rs.html#51-73" class="src">Source</a>

``` rust
pub fn articulation_points<G>(g: G) -> HashSet<G::NodeId>where
    G: IntoNodeReferences + IntoEdges + NodeIndexable + GraphProp,
    G::NodeWeight: Clone,
    G::EdgeWeight: Clone + PartialOrd,
    G::NodeId: Eq + Hash,
```

Expand description

Find articulation points in a graph using [Tarjan’s algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm).

Compute the articulation points of a graph (Nodes, which would increase the number of connected components in the graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/articulation_points/fn.articulation_points.html#arguments" class="doc-anchor">§</a>Arguments

- `graph`: A directed graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/articulation_points/fn.articulation_points.html#returns" class="doc-anchor">§</a>Returns

- `HashSet`: [`hashbrown::HashSet`](https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html "struct hashbrown::set::HashSet") of the node ids which correspond to the articulation points of the graph.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/articulation_points/fn.articulation_points.html#complexity" class="doc-anchor">§</a>Complexity

- Time complexity: **O(\|V\| + \|E\|)**,
- Auxiliary space: **O(\|V\|)**,

where **\|V\|** is the number of nodes and **\|E\|** is the number of edges.

## <a href="https://docs.rs/petgraph/latest/petgraph/algo/articulation_points/fn.articulation_points.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::{
    algo::articulation_points,
    graph::{NodeIndex, UnGraph},
    algo::articulation_points::articulation_points,
};

let mut gr = UnGraph::<&str, ()>::new_undirected();
let a = gr.add_node("A");
let b = gr.add_node("B");
let c = gr.add_node("C");

gr.add_edge(a, b, ());
gr.add_edge(b, c, ());

let articulation_points: Vec<&str> = articulation_points(&gr)
    .into_iter()
    .map(|node_idx| gr[node_idx])
    .collect();

// Articulation Points: ["B"]
println!("Articulation Points: {:?}", articulation_points);
```
