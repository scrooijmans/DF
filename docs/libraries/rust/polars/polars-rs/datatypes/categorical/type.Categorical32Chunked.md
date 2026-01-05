# Type Alias Categorical32Chunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/categorical.rs.html#16" class="src">Source</a>

``` rust
pub type Categorical32Chunked = Logical<Categorical32Type, <Categorical32Type as PolarsCategoricalType>::PolarsPhysical>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/type.Categorical32Chunked.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Categorical32Chunked {
    pub phys: ChunkedArray<UInt32Type>,
    pub dtype: DataType,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/type.Categorical32Chunked.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/type.Categorical32Chunked.html#structfield.phys" class="anchor field">§</a>`phys: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray"><code>ChunkedArray</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type"><code>UInt32Type</code></a>`>`<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/type.Categorical32Chunked.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>
