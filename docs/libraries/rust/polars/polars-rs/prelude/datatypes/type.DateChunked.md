# Type Alias DateChunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/date.rs.html#3" class="src">Source</a>

``` rust
pub type DateChunked = Logical<DateType, Int32Type>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DateChunked.html#aliased-type" class="anchor">§</a>

``` rust
pub struct DateChunked {
    pub phys: ChunkedArray<Int32Type>,
    pub dtype: DataType,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DateChunked.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DateChunked.html#structfield.phys" class="anchor field">§</a>`phys: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray"><code>ChunkedArray</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type"><code>Int32Type</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DateChunked.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>
