# Enum DfsEvent Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/visit/dfsvisit.rs.html#10-23" class="src">Source</a>

``` rust
pub enum DfsEvent<N> {
    Discover(N, Time),
    TreeEdge(N, N),
    BackEdge(N, N),
    CrossForwardEdge(N, N),
    Finish(N, Time),
}
```

Expand description

A depth first search (DFS) visitor event.

## Variants<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#variants" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#variant.Discover" class="anchor">§</a>

### Discover(N, <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Time.html" class="struct" title="struct petgraph::visit::Time">Time</a>)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#variant.TreeEdge" class="anchor">§</a>

### TreeEdge(N, N)

An edge of the tree formed by the traversal.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#variant.BackEdge" class="anchor">§</a>

### BackEdge(N, N)

An edge to an already visited node.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#variant.CrossForwardEdge" class="anchor">§</a>

### CrossForwardEdge(N, N)

A cross or forward edge.

For an edge *(u, v)*, if the discover time of *v* is greater than *u*, then it is a forward edge, else a cross edge.

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#variant.Finish" class="anchor">§</a>

### Finish(N, <a href="https://docs.rs/petgraph/latest/petgraph/visit/struct.Time.html" class="struct" title="struct petgraph::visit::Time">Time</a>)

All edges from a node have been reported.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#impl-Clone-for-DfsEvent%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html" class="enum" title="enum petgraph::visit::DfsEvent">DfsEvent</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html" class="enum" title="enum petgraph::visit::DfsEvent">DfsEvent</a>\<N\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#impl-Debug-for-DfsEvent%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html" class="enum" title="enum petgraph::visit::DfsEvent">DfsEvent</a>\<N\>

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#impl-Copy-for-DfsEvent%3CN%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html" class="enum" title="enum petgraph::visit::DfsEvent">DfsEvent</a>\<N\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/visit/enum.DfsEvent.html#blanket-implementations" class="anchor">§</a>
