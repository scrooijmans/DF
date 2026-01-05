# Type Alias DecimalChunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/decimal.rs.html#7" class="src">Source</a>

``` rust
pub type DecimalChunked = Logical<DecimalType, Int128Type>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DecimalChunked.html#aliased-type" class="anchor">§</a>

``` rust
pub struct DecimalChunked {
    pub phys: ChunkedArray<Int128Type>,
    pub dtype: DataType,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DecimalChunked.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DecimalChunked.html#structfield.phys" class="anchor field">§</a>`phys: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray"><code>ChunkedArray</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int128Type.html" class="struct" title="struct polars::prelude::Int128Type"><code>Int128Type</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/type.DecimalChunked.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>
