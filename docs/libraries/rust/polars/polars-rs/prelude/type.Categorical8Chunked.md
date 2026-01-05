# Type Alias Categorical8Chunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/categorical.rs.html#14" class="src">Source</a>

``` rust
pub type Categorical8Chunked = Logical<Categorical8Type, <Categorical8Type as PolarsCategoricalType>::PolarsPhysical>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.Categorical8Chunked.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Categorical8Chunked {
    pub phys: ChunkedArray<UInt8Type>,
    pub dtype: DataType,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/type.Categorical8Chunked.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/type.Categorical8Chunked.html#structfield.phys" class="anchor field">§</a>`phys: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray"><code>ChunkedArray</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type"><code>UInt8Type</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/type.Categorical8Chunked.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>
