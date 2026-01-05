# Struct NotZero Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/matrix_graph.rs.html#92" class="src">Source</a>

``` rust
pub struct NotZero<T>(/* private fields */);
```

Expand description

`NotZero` is used to optimize the memory usage of edge weights `E` in a [`MatrixGraph`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.MatrixGraph.html), replacing the default `Option<E>` sentinel.

Pre-requisite: edge weight should implement [`Zero`](https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html).

Note that if you’re already using the standard non-zero types (such as `NonZeroU32`), you don’t have to use this wrapper and can leave the default `Null` type argument.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html#impl-Default-for-NotZero%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a>\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html" class="struct" title="struct petgraph::matrix_graph::NotZero">NotZero</a>\<T\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html#impl-From%3CNotZero%3CT%3E%3E-for-Option%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html" class="struct" title="struct petgraph::matrix_graph::NotZero">NotZero</a>\<T\>\> for <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(not_zero: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html" class="struct" title="struct petgraph::matrix_graph::NotZero">NotZero</a>\<T\>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html#impl-Nullable-for-NotZero%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html" class="struct" title="struct petgraph::matrix_graph::NotZero">NotZero</a>\<T\>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html#blanket-implementations" class="anchor">§</a>
