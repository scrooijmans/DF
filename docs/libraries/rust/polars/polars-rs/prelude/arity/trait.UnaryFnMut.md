# Trait UnaryFnMut Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/arity.rs.html#16" class="src">Source</a>

``` rust
pub trait UnaryFnMut<A1>: FnMut(A1) -> Self::Ret {
    type Ret;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.UnaryFnMut.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.UnaryFnMut.html#associatedtype.Ret" class="associatedtype">Ret</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.UnaryFnMut.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.UnaryFnMut.html#impl-UnaryFnMut%3CA1%3E-for-T" class="anchor">§</a>

### impl\<A1, R, T\> <a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.UnaryFnMut.html" class="trait" title="trait polars::prelude::arity::UnaryFnMut">UnaryFnMut</a>\<A1\> for T

where T: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(A1) -\> R,

<a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.UnaryFnMut.html#associatedtype.Ret-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/arity/trait.UnaryFnMut.html#associatedtype.Ret" class="associatedtype">Ret</a> = R
