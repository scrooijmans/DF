# Trait Nullable Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/matrix_graph.rs.html#50-67" class="src">Source</a>

``` rust
pub trait Nullable:
    Default
    + Into<Option<Self::Wrapped>>
    + Sealed { }
```

Expand description

Wrapper trait for an `Option`, allowing user-defined structs to be input as containers when defining a null element.

Note: this trait is currently *sealed* and cannot be implemented for types outside this crate.

## Dyn Compatibility<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html#impl-Nullable-for-Option%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a> for <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

## Implementors<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html#impl-Nullable-for-NotZero%3CT%3E" class="anchor">§</a>

### impl\<T: <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Zero.html" class="trait" title="trait petgraph::matrix_graph::Zero">Zero</a>\> <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/trait.Nullable.html" class="trait" title="trait petgraph::matrix_graph::Nullable">Nullable</a> for <a href="https://docs.rs/petgraph/latest/petgraph/matrix_graph/struct.NotZero.html" class="struct" title="struct petgraph::matrix_graph::NotZero">NotZero</a>\<T\>
