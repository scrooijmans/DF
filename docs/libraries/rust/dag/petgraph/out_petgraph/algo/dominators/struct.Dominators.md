# Struct Dominators Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/dominators.rs.html#24-30" class="src">Source</a>

``` rust
pub struct Dominators<N>where
    N: Copy + Eq + Hash,{ /* private fields */ }
```

Expand description

The dominance relation for some graph and root.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#impl-Dominators%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html" class="struct" title="struct petgraph::algo::dominators::Dominators">Dominators</a>\<N\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#method.root" class="fn">root</a>(&self) -\> N

Get the root node used to construct these dominance relations.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#method.immediate_dominator" class="fn">immediate_dominator</a>(&self, node: N) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>

Get the immediate dominator of the given node.

Returns `None` for any node that is not reachable from the root, and for the root itself.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#method.strict_dominators" class="fn">strict_dominators</a>(&self, node: N) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.DominatorsIter.html" class="struct" title="struct petgraph::algo::dominators::DominatorsIter">DominatorsIter</a>\<'\_, N\>\>

Iterate over the given node’s strict dominators.

If the given node is not reachable from the root, then `None` is returned.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#method.dominators" class="fn">dominators</a>(&self, node: N) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.DominatorsIter.html" class="struct" title="struct petgraph::algo::dominators::DominatorsIter">DominatorsIter</a>\<'\_, N\>\>

Iterate over all of the given node’s dominators (including the given node itself).

If the given node is not reachable from the root, then `None` is returned.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#method.immediately_dominated_by" class="fn">immediately_dominated_by</a>(&self, node: N) -\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.DominatedByIter.html" class="struct" title="struct petgraph::algo::dominators::DominatedByIter">DominatedByIter</a>\<'\_, N\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#" class="tooltip" data-notable-ty="DominatedByIter&lt;&#39;_, N&gt;">ⓘ</a>

Iterate over all nodes immediately dominated by the given node (not including the given node itself).

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#impl-Clone-for-Dominators%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html" class="struct" title="struct petgraph::algo::dominators::Dominators">Dominators</a>\<N\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html" class="struct" title="struct petgraph::algo::dominators::Dominators">Dominators</a>\<N\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#impl-Debug-for-Dominators%3CN%3E" class="anchor">§</a>

### impl\<N\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html" class="struct" title="struct petgraph::algo::dominators::Dominators">Dominators</a>\<N\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/dominators/struct.Dominators.html#blanket-implementations" class="anchor">§</a>
