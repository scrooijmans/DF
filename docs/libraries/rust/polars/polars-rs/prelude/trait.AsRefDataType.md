# Trait AsRefDataType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/dtype.rs.html#147" class="src">Source</a>

``` rust
pub trait AsRefDataType {
    // Required method
    fn as_ref_dtype(&self) -> &DataType;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsRefDataType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsRefDataType.html#tymethod.as_ref_dtype" class="fn">as_ref_dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsRefDataType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.AsRefDataType.html#impl-AsRefDataType-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsRefDataType.html" class="trait" title="trait polars::prelude::AsRefDataType">AsRefDataType</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>,
