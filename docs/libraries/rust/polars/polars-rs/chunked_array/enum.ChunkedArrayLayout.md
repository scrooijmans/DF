# Enum ChunkedArrayLayout Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/mod.rs.html#690" class="src">Source</a>

``` rust
pub enum ChunkedArrayLayout<'a, T>where
    T: PolarsDataType,{
    SingleNoNull(&'a <T as PolarsDataType>::Array),
    Single(&'a <T as PolarsDataType>::Array),
    MultiNoNull(&'a ChunkedArray<T>),
    Multi(&'a ChunkedArray<T>),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/chunked_array/enum.ChunkedArrayLayout.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/enum.ChunkedArrayLayout.html#variant.SingleNoNull" class="anchor">§</a>

### SingleNoNull(&'a \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>)

<a href="https://docs.rs/polars/latest/polars/chunked_array/enum.ChunkedArrayLayout.html#variant.Single" class="anchor">§</a>

### Single(&'a \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>)

<a href="https://docs.rs/polars/latest/polars/chunked_array/enum.ChunkedArrayLayout.html#variant.MultiNoNull" class="anchor">§</a>

### MultiNoNull(&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>)

<a href="https://docs.rs/polars/latest/polars/chunked_array/enum.ChunkedArrayLayout.html#variant.Multi" class="anchor">§</a>

### Multi(&'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/enum.ChunkedArrayLayout.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/enum.ChunkedArrayLayout.html#blanket-implementations" class="anchor">§</a>
