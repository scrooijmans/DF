# Trait IsFirstDistinct Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#647" class="src">Source</a>

``` rust
pub trait IsFirstDistinct<T>where
    T: PolarsDataType,{
    // Provided method
    fn is_first_distinct(
        &self,
    ) -> Result<ChunkedArray<BooleanType>, PolarsError> { ... }
}
```

Expand description

Mask the first unique values as `true`

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.IsFirstDistinct.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IsFirstDistinct.html#method.is_first_distinct" class="fn">is_first_distinct</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.IsFirstDistinct.html#implementors" class="anchor">§</a>
