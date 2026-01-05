# Struct Paths Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/bellman_ford.rs.html#12-15" class="src">Source</a>

``` rust
pub struct Paths<NodeId, EdgeWeight> {
    pub distances: Vec<EdgeWeight>,
    pub predecessors: Vec<Option<NodeId>>,
}
```

## Fields<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#fields" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#structfield.distances" class="anchor field">§</a>`distances: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<EdgeWeight>`<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#structfield.predecessors" class="anchor field">§</a>`predecessors: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<NodeId>>`

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#impl-Clone-for-Paths%3CNodeId,+EdgeWeight%3E" class="anchor">§</a>

### impl\<NodeId: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, EdgeWeight: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html" class="struct" title="struct petgraph::algo::bellman_ford::Paths">Paths</a>\<NodeId, EdgeWeight\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html" class="struct" title="struct petgraph::algo::bellman_ford::Paths">Paths</a>\<NodeId, EdgeWeight\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#impl-Debug-for-Paths%3CNodeId,+EdgeWeight%3E" class="anchor">§</a>

### impl\<NodeId: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, EdgeWeight: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html" class="struct" title="struct petgraph::algo::bellman_ford::Paths">Paths</a>\<NodeId, EdgeWeight\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/struct.Paths.html#blanket-implementations" class="anchor">§</a>
