# Trait TernaryFnMut Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#26" class="src">Source</a>

``` rust
pub trait TernaryFnMut<A1, A2, A3>: FnMut(A1, A2, A3) -> Self::Ret {
    type Ret;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.TernaryFnMut.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.TernaryFnMut.html#associatedtype.Ret" class="associatedtype">Ret</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.TernaryFnMut.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.TernaryFnMut.html#impl-TernaryFnMut%3CA1,+A2,+A3%3E-for-T" class="anchor">§</a>

### impl\<A1, A2, A3, R, T\> <a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.TernaryFnMut.html" class="trait" title="trait polars::prelude::arity::TernaryFnMut">TernaryFnMut</a>\<A1, A2, A3\> for T

where T: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(A1, A2, A3) -\> R,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.TernaryFnMut.html#associatedtype.Ret-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/arity/trait.TernaryFnMut.html#associatedtype.Ret" class="associatedtype">Ret</a> = R
