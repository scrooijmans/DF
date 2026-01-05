# Trait FilterNode Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/filter.rs.html#18-21" class="src">Source</a>

``` rust
pub trait FilterNode<N> {
    // Required method
    fn include_node(&self, node: N) -> bool;
}
```

Expand description

A graph filter for nodes.

## Required Methods<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#tymethod.include_node" class="fn">include_node</a>(&self, node: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true to have the node be part of the graph

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#impl-FilterNode%3CN%3E-for-%26FixedBitSet" class="anchor">§</a>

### impl\<N\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html" class="trait" title="trait petgraph::visit::FilterNode">FilterNode</a>\<N\> for &<a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

where <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#method.include_node" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#tymethod.include_node" class="fn">include_node</a>(&self, n: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#impl-FilterNode%3CN%3E-for-FixedBitSet" class="anchor">§</a>

### impl\<N\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html" class="trait" title="trait petgraph::visit::FilterNode">FilterNode</a>\<N\> for <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

where <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\>,

This filter includes the nodes that are contained in the set.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#method.include_node-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#tymethod.include_node" class="fn">include_node</a>(&self, n: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#impl-FilterNode%3CN%3E-for-%26HashSet%3CN,+S%3E" class="anchor">§</a>

### impl\<N, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html" class="trait" title="trait petgraph::visit::FilterNode">FilterNode</a>\<N\> for &<a href="https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<N, S\>

where <a href="https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<N, S\>: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\>,

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#method.include_node-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#tymethod.include_node" class="fn">include_node</a>(&self, n: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#impl-FilterNode%3CN%3E-for-HashSet%3CN,+S%3E" class="anchor">§</a>

### impl\<N, S\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html" class="trait" title="trait petgraph::visit::FilterNode">FilterNode</a>\<N\> for <a href="https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<N, S\>

where <a href="https://docs.rs/hashbrown/0.16.0/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<N, S\>: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\>,

This filter includes the nodes that are contained in the set.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#method.include_node-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#tymethod.include_node" class="fn">include_node</a>(&self, n: N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html#impl-FilterNode%3CN%3E-for-F" class="anchor">§</a>

### impl\<F, N\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.FilterNode.html" class="trait" title="trait petgraph::visit::FilterNode">FilterNode</a>\<N\> for F

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(N) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>,
