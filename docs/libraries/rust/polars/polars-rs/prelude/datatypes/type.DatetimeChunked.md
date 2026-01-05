# Type Alias DatetimeChunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/datetime.rs.html#5" class="src">Source</a>

``` rust
pub type DatetimeChunked = Logical<DatetimeType, Int64Type>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DatetimeChunked.html#aliased-type" class="anchor">§</a>

``` rust
pub struct DatetimeChunked {
    pub phys: ChunkedArray<Int64Type>,
    pub dtype: DataType,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DatetimeChunked.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DatetimeChunked.html#structfield.phys" class="anchor field">§</a>`phys: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray"><code>ChunkedArray</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type"><code>Int64Type</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DatetimeChunked.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>
