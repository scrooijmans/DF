# Trait ChunkedCollectInferIterExt Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/collect.rs.html#130" class="src">Source</a>

``` rust
pub trait ChunkedCollectInferIterExt<T>: Sized + Iteratorwhere
    T: PolarsDataType,{
    // Provided methods
    fn collect_ca(self, name: PlSmallStr) -> ChunkedArray<T>
       where <T as PolarsDataType>::Array: ArrayFromIter<Self::Item> { ... }
    fn collect_ca_trusted(self, name: PlSmallStr) -> ChunkedArray<T>
       where <T as PolarsDataType>::Array: ArrayFromIter<Self::Item>,
             Self: TrustedLen { ... }
    fn try_collect_ca<U, E>(
        self,
        name: PlSmallStr,
    ) -> Result<ChunkedArray<T>, E>
       where <T as PolarsDataType>::Array: ArrayFromIter<U>,
             Self: Iterator<Item = Result<U, E>> { ... }
    fn try_collect_ca_trusted<U, E>(
        self,
        name: PlSmallStr,
    ) -> Result<ChunkedArray<T>, E>
       where <T as PolarsDataType>::Array: ArrayFromIter<U>,
             Self: Iterator<Item = Result<U, E>> + TrustedLen { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html#method.collect_ca" class="fn">collect_ca</a>(self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html#method.collect_ca_trusted" class="fn">collect_ca_trusted</a>(self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html#method.try_collect_ca" class="fn">try_collect_ca</a>\<U, E\>(self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, E\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html#method.try_collect_ca_trusted" class="fn">try_collect_ca_trusted</a>\<U, E\>( self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, E\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectInferIterExt.html#impl-ChunkedCollectInferIterExt%3CT%3E-for-I" class="anchor">§</a>

### impl\<T, I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedCollectInferIterExt.html" class="trait" title="trait polars::prelude::ChunkedCollectInferIterExt">ChunkedCollectInferIterExt</a>\<T\> for I

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>,
