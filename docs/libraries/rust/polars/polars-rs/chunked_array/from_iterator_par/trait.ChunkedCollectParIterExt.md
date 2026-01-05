# Trait ChunkedCollectParIterExt Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/from_iterator_par.rs.html#268" class="src">Source</a>

``` rust
pub trait ChunkedCollectParIterExt: ParallelIterator {
    // Provided method
    fn collect_ca_with_dtype<B>(self, name: PlSmallStr, dtype: DataType) -> B
       where B: FromParIterWithDtype<Self::Item>,
             Self: Sized { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.ChunkedCollectParIterExt.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.ChunkedCollectParIterExt.html#method.collect_ca_with_dtype" class="fn">collect_ca_with_dtype</a>\<B\>(self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> B

where B: <a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.FromParIterWithDtype.html" class="trait" title="trait polars::chunked_array::from_iterator_par::FromParIterWithDtype">FromParIterWithDtype</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.ChunkedCollectParIterExt.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.ChunkedCollectParIterExt.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.ChunkedCollectParIterExt.html#impl-ChunkedCollectParIterExt-for-I" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/polars/latest/polars/chunked_array/from_iterator_par/trait.ChunkedCollectParIterExt.html" class="trait" title="trait polars::chunked_array::from_iterator_par::ChunkedCollectParIterExt">ChunkedCollectParIterExt</a> for I

where I: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>,
