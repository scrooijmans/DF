# Struct Matching Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/matching.rs.html#12-16" class="src">Source</a>

``` rust
pub struct Matching<G: GraphBase> { /* private fields */ }
```

Expand description

Computed [*matching*](https://en.wikipedia.org/wiki/Matching_(graph_theory)#Definitions) of the graph.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#impl-Matching%3CG%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html" class="struct" title="struct petgraph::algo::matching::Matching">Matching</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#method.mate" class="fn">mate</a>(&self, node: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>\>

Gets the matched counterpart of given node, if there is any.

Returns `None` if the node is not matched or does not exist.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#method.edges" class="fn">edges</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.MatchedEdges.html" class="struct" title="struct petgraph::algo::matching::MatchedEdges">MatchedEdges</a>\<'\_, G\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#" class="tooltip" data-notable-ty="MatchedEdges&lt;&#39;_, G&gt;">ⓘ</a>

Iterates over all edges from the matching.

An edge is represented by its endpoints. The graph is considered undirected and every pair of matched nodes is reported only once.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#method.nodes" class="fn">nodes</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.MatchedNodes.html" class="struct" title="struct petgraph::algo::matching::MatchedNodes">MatchedNodes</a>\<'\_, G\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#" class="tooltip" data-notable-ty="MatchedNodes&lt;&#39;_, G&gt;">ⓘ</a>

Iterates over all nodes from the matching.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#method.contains_edge" class="fn">contains_edge</a>(&self, a: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>, b: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if given edge is in the matching, or `false` otherwise.

If any of the nodes does not exist, `false` is returned.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#method.contains_node" class="fn">contains_node</a>(&self, node: G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphBase.html#associatedtype.NodeId" class="associatedtype" title="type petgraph::visit::GraphBase::NodeId">NodeId</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if given node is in the matching, or `false` otherwise.

If the node does not exist, `false` is returned.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Gets the number of matched **edges**.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the number of matched **edges** is 0.

<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#impl-Matching%3CG%3E-1" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html" class="struct" title="struct petgraph::algo::matching::Matching">Matching</a>\<G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeCount.html" class="trait" title="trait petgraph::visit::NodeCount">NodeCount</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#method.is_perfect" class="fn">is_perfect</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the matching is perfect.

A matching is [*perfect*](https://en.wikipedia.org/wiki/Matching_(graph_theory)#Definitions) if every node in the graph is incident to an edge from the matching.

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/matching/struct.Matching.html#blanket-implementations" class="anchor">§</a>
