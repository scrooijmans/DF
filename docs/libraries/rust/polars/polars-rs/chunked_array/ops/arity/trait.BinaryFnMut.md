# Trait BinaryFnMut Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#36" class="src">Source</a>

``` rust
pub trait BinaryFnMut<A1, A2>: FnMut(A1, A2) -> Self::Ret {
    type Ret;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.BinaryFnMut.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.BinaryFnMut.html#associatedtype.Ret" class="associatedtype">Ret</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.BinaryFnMut.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.BinaryFnMut.html#impl-BinaryFnMut%3CA1,+A2%3E-for-T" class="anchor">§</a>

### impl\<A1, A2, R, T\> <a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.BinaryFnMut.html" class="trait" title="trait polars::prelude::arity::BinaryFnMut">BinaryFnMut</a>\<A1, A2\> for T

where T: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(A1, A2) -\> R,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.BinaryFnMut.html#associatedtype.Ret-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.BinaryFnMut.html#associatedtype.Ret" class="associatedtype">Ret</a> = R
