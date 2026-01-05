# Function depth_first_search Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/dfsvisit.rs.html#241-259" class="src">Source</a>

``` rust
pub fn depth_first_search<G, I, F, C>(graph: G, starts: I, visitor: F) -> Cwhere
    G: IntoNeighbors + Visitable,
    I: IntoIterator<Item = G::NodeId>,
    F: FnMut(DfsEvent<G::NodeId>) -> C,
    C: ControlFlow,
```

Expand description

A recursive depth first search.

Starting points are the nodes in the iterator `starts` (specify just one start node *x* by using `Some(x)`).

The traversal emits discovery and finish events for each reachable node, and edge classification of each reachable edge. `visitor` is called for each event, see [`DfsEvent`](https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html) for possible values.

The return value should implement the trait `ControlFlow`, and can be used to change the control flow of the search.

`Control` Implements `ControlFlow` such that `Control::Continue` resumes the search. `Control::Break` will stop the visit early, returning the contained value. `Control::Prune` will stop traversing any additional edges from the current node and proceed immediately to the `Finish` event.

There are implementations of `ControlFlow` for `()`, and `Result<C, E>` where `C: ControlFlow`. The implementation for `()` will continue until finished. For `Result`, upon encountering an `E` it will break, otherwise acting the same as `C`.

**Panics** if you attempt to prune a node from its `Finish` event.

## <a href="https://docs.rs/petgraph/latest/petgraph/visit/fn.depth_first_search.html#example-returning-control" class="doc-anchor">§</a>Example returning `Control`.

Find a path from node 0 to 5, and exit the visit as soon as we reach the goal node.

``` rust
use petgraph::prelude::*;
use petgraph::graph::node_index as n;
use petgraph::visit::depth_first_search;
use petgraph::visit::{DfsEvent, Control};

let gr: Graph<(), ()> = Graph::from_edges(&[
    (0, 1), (0, 2), (0, 3),
    (1, 3),
    (2, 3), (2, 4),
    (4, 0), (4, 5),
]);

// record each predecessor, mapping node → node
let mut predecessor = vec![NodeIndex::end(); gr.node_count()];
let start = n(0);
let goal = n(5);
depth_first_search(&gr, Some(start), |event| {
    if let DfsEvent::TreeEdge(u, v) = event {
        predecessor[v.index()] = u;
        if v == goal {
            return Control::Break(v);
        }
    }
    Control::Continue
});

let mut next = goal;
let mut path = vec![next];
while next != start {
    let pred = predecessor[next.index()];
    path.push(pred);
    next = pred;
}
path.reverse();
assert_eq!(&path, &[n(0), n(2), n(4), n(5)]);
```

## <a href="https://docs.rs/petgraph/latest/petgraph/visit/fn.depth_first_search.html#example-returning-a-result" class="doc-anchor">§</a>Example returning a `Result`.

``` rust
use petgraph::graph::node_index as n;
use petgraph::prelude::*;
use petgraph::visit::depth_first_search;
use petgraph::visit::{DfsEvent, Time};

let gr: Graph<(), ()> = Graph::from_edges(&[(0, 1), (1, 2), (1, 1), (2, 1)]);
let start = n(0);
let mut back_edges = 0;
let mut discover_time = 0;
// Stop the search, the first time a BackEdge is encountered.
let result = depth_first_search(&gr, Some(start), |event| {
    match event {
        // In the cases where Ok(()) is returned,
        // Result falls back to the implementation of Control on the value ().
        // In the case of (), this is to always return Control::Continue.
        // continuing the search.
        DfsEvent::Discover(_, Time(t)) => {
            discover_time = t;
            Ok(())
        }
        DfsEvent::BackEdge(_, _) => {
            back_edges += 1;
            // the implementation of ControlFlow for Result,
            // treats this Err value as Continue::Break
            Err(event)
        }
        _ => Ok(()),
    }
});

// Even though the graph has more than one cycle,
// The number of back_edges visited by the search should always be 1.
assert_eq!(back_edges, 1);
println!("discover time:{:?}", discover_time);
println!("number of backedges encountered: {}", back_edges);
println!("back edge: {:?}", result);
```
