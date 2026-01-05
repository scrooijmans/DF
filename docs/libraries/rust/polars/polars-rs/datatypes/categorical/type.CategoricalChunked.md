# Type Alias CategoricalChunked Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/logical/categorical.rs.html#13" class="src">Source</a>

``` rust
pub type CategoricalChunked<T> = Logical<T, <T as PolarsCategoricalType>::PolarsPhysical>;
```

## Aliased Type<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/type.CategoricalChunked.html#aliased-type" class="anchor">§</a>

``` rust
pub struct CategoricalChunked<T> {
    pub phys: ChunkedArray<<T as PolarsCategoricalType>::PolarsPhysical>,
    pub dtype: DataType,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/type.CategoricalChunked.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/type.CategoricalChunked.html#structfield.phys" class="anchor field">§</a>`phys: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray"><code>ChunkedArray</code></a>`<<T as `<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html" class="trait" title="trait polars::prelude::PolarsCategoricalType"><code>PolarsCategoricalType</code></a>`>::`<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsCategoricalType.html#associatedtype.PolarsPhysical" class="associatedtype" title="type polars::prelude::PolarsCategoricalType::PolarsPhysical"><code>PolarsPhysical</code></a>`>`<a href="https://docs.rs/polars/latest/polars/datatypes/categorical/type.CategoricalChunked.html#structfield.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>
