# Trait ChunkFilter Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#460" class="src">Source</a>

``` rust
pub trait ChunkFilter<T>where
    T: PolarsDataType,{
    // Required method
    fn filter(
        &self,
        filter: &ChunkedArray<BooleanType>,
    ) -> Result<ChunkedArray<T>, PolarsError>
       where Self: Sized;
}
```

Expand description

Filter values by a boolean mask.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFilter.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFilter.html#tymethod.filter" class="fn">filter</a>( &self, filter: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Filter values in the ChunkedArray with a boolean mask.

``` rust
let array = Int32Chunked::new("array".into(), &[1, 2, 3]);
let mask = BooleanChunked::new("mask".into(), &[true, false, true]);

let filtered = array.filter(&mask).unwrap();
assert_eq!(Vec::from(&filtered), [Some(1), Some(3)])
```

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFilter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFilter.html#impl-ChunkFilter%3CObjectType%3CT%3E%3E-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFilter.html" class="trait" title="trait polars::prelude::ChunkFilter">ChunkFilter</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFilter.html#impl-ChunkFilter%3CT%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkFilter.html" class="trait" title="trait polars::prelude::ChunkFilter">ChunkFilter</a>\<T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\<IsObject = <a href="https://docs.rs/polars/latest/polars/prelude/struct.FalseT.html" class="struct" title="struct polars::prelude::FalseT">FalseT</a>\>,
