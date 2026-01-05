# Enum MatrixError Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/matrix_graph.rs.html#194-200" class="src">Source</a>

``` rust
pub enum MatrixError {
    NodeIxLimit,
    NodeMissed(usize),
}
```

Expand description

The error type for fallible `MatrixGraph` operations.

## Variants<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#variants" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#variant.NodeIxLimit" class="anchor">§</a>

### NodeIxLimit

The `MatrixGraph` is at the maximum number of nodes for its index.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#variant.NodeMissed" class="anchor">§</a>

### NodeMissed(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

The node with the specified index is missing from the graph.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#impl-Clone-for-MatrixError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#impl-Debug-for-MatrixError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#impl-Display-for-MatrixError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#impl-Error-for-MatrixError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

Available on **crate feature `std`** only.

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#111" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#137" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#147" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#impl-PartialEq-for-MatrixError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#impl-Copy-for-MatrixError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#impl-Eq-for-MatrixError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#impl-StructuralPartialEq-for-MatrixError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html" class="enum" title="enum petgraph::matrix_graph::MatrixError">MatrixError</a>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/enum.MatrixError.html#blanket-implementations" class="anchor">§</a>
