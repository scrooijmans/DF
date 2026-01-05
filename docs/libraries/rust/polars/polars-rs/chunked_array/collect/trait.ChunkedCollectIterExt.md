# Trait ChunkedCollectIterExt Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/collect.rs.html#24" class="src">Source</a>

``` rust
pub trait ChunkedCollectIterExt<T>: Sized + Iteratorwhere
    T: PolarsDataType,{
    // Provided methods
    fn collect_ca_with_dtype(
        self,
        name: PlSmallStr,
        dtype: DataType,
    ) -> ChunkedArray<T>
       where <T as PolarsDataType>::Array: ArrayFromIterDtype<Self::Item> { ... }
    fn collect_ca_like(
        self,
        name_dtype_src: &ChunkedArray<T>,
    ) -> ChunkedArray<T>
       where <T as PolarsDataType>::Array: ArrayFromIterDtype<Self::Item> { ... }
    fn collect_ca_trusted_with_dtype(
        self,
        name: PlSmallStr,
        dtype: DataType,
    ) -> ChunkedArray<T>
       where <T as PolarsDataType>::Array: ArrayFromIterDtype<Self::Item>,
             Self: TrustedLen { ... }
    fn collect_ca_trusted_like(
        self,
        name_dtype_src: &ChunkedArray<T>,
    ) -> ChunkedArray<T>
       where <T as PolarsDataType>::Array: ArrayFromIterDtype<Self::Item>,
             Self: TrustedLen { ... }
    fn try_collect_ca_with_dtype<U, E>(
        self,
        name: PlSmallStr,
        dtype: DataType,
    ) -> Result<ChunkedArray<T>, E>
       where <T as PolarsDataType>::Array: ArrayFromIterDtype<U>,
             Self: Iterator<Item = Result<U, E>> { ... }
    fn try_collect_ca_like<U, E>(
        self,
        name_dtype_src: &ChunkedArray<T>,
    ) -> Result<ChunkedArray<T>, E>
       where <T as PolarsDataType>::Array: ArrayFromIterDtype<U>,
             Self: Iterator<Item = Result<U, E>> { ... }
    fn try_collect_ca_trusted_with_dtype<U, E>(
        self,
        name: PlSmallStr,
        dtype: DataType,
    ) -> Result<ChunkedArray<T>, E>
       where <T as PolarsDataType>::Array: ArrayFromIterDtype<U>,
             Self: Iterator<Item = Result<U, E>> + TrustedLen { ... }
    fn try_collect_ca_trusted_like<U, E>(
        self,
        name_dtype_src: &ChunkedArray<T>,
    ) -> Result<ChunkedArray<T>, E>
       where <T as PolarsDataType>::Array: ArrayFromIterDtype<U>,
             Self: Iterator<Item = Result<U, E>> + TrustedLen { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#method.collect_ca_with_dtype" class="fn">collect_ca_with_dtype</a>( self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#method.collect_ca_like" class="fn">collect_ca_like</a>(self, name_dtype_src: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#method.collect_ca_trusted_with_dtype" class="fn">collect_ca_trusted_with_dtype</a>( self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#method.collect_ca_trusted_like" class="fn">collect_ca_trusted_like</a>( self, name_dtype_src: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#method.try_collect_ca_with_dtype" class="fn">try_collect_ca_with_dtype</a>\<U, E\>( self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, E\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#method.try_collect_ca_like" class="fn">try_collect_ca_like</a>\<U, E\>( self, name_dtype_src: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, E\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#method.try_collect_ca_trusted_with_dtype" class="fn">try_collect_ca_trusted_with_dtype</a>\<U, E\>( self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, E\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#method.try_collect_ca_trusted_like" class="fn">try_collect_ca_trusted_like</a>\<U, E\>( self, name_dtype_src: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>, E\>

where \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html#associatedtype.Array" class="associatedtype" title="type polars::prelude::PolarsDataType::Array">Array</a>: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/collect/trait.ChunkedCollectIterExt.html#impl-ChunkedCollectIterExt%3CT%3E-for-I" class="anchor">§</a>

### impl\<T, I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkedCollectIterExt.html" class="trait" title="trait polars::prelude::ChunkedCollectIterExt">ChunkedCollectIterExt</a>\<T\> for I

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>,
