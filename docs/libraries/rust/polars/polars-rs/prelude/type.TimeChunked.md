# Type Alias TimeChunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/time.rs.html#6" class="src">Source</a>

``` rust
pub type TimeChunked = Logical<TimeType, Int64Type>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.TimeChunked.html#aliased-type" class="anchor">§</a>

``` rust
pub struct TimeChunked {
    pub phys: ChunkedArray<Int64Type>,
    pub dtype: DataType,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/type.TimeChunked.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/type.TimeChunked.html#structfield.phys" class="anchor field">§</a>`phys: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray"><code>ChunkedArray</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type"><code>Int64Type</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/type.TimeChunked.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>
