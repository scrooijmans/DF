# Struct DfsSpace Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/algo/mod.rs.html#301-303" class="src">Source</a>

``` rust
pub struct DfsSpace<N, VM> { /* private fields */ }
```

Expand description

Workspace for a graph traversal.

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#impl-DfsSpace%3CN,+VM%3E" class="anchor">§</a>

### impl\<N, VM\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html" class="struct" title="struct petgraph::algo::DfsSpace">DfsSpace</a>\<N, VM\>

where N: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>, VM: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#method.new" class="fn">new</a>\<G\>(g: G) -\> Self

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphRef.html" class="trait" title="trait petgraph::visit::GraphRef">GraphRef</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Visitable.html" class="trait" title="trait petgraph::visit::Visitable">Visitable</a>\<NodeId = N, Map = VM\>,

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#impl-Clone-for-DfsSpace%3CN,+VM%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, VM: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html" class="struct" title="struct petgraph::algo::DfsSpace">DfsSpace</a>\<N, VM\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html" class="struct" title="struct petgraph::algo::DfsSpace">DfsSpace</a>\<N, VM\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#impl-Debug-for-DfsSpace%3CN,+VM%3E" class="anchor">§</a>

### impl\<N: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, VM: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html" class="struct" title="struct petgraph::algo::DfsSpace">DfsSpace</a>\<N, VM\>

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#impl-Default-for-DfsSpace%3CN,+VM%3E" class="anchor">§</a>

### impl\<N, VM\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html" class="struct" title="struct petgraph::algo::DfsSpace">DfsSpace</a>\<N, VM\>

where VM: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.VisitMap.html" class="trait" title="trait petgraph::visit::VisitMap">VisitMap</a>\<N\> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/algo/struct.DfsSpace.html#blanket-implementations" class="anchor">§</a>
