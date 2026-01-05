# Trait ArrayCollectIterExt Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/array/static_array_collect.rs.html#132" class="src">Source</a>

``` rust
pub trait ArrayCollectIterExt<A>: Sized + Iteratorwhere
    A: StaticArray,{
    // Provided methods
    fn collect_arr(self) -> A
       where A: ArrayFromIter<Self::Item> { ... }
    fn collect_arr_trusted(self) -> A
       where A: ArrayFromIter<Self::Item>,
             Self: TrustedLen { ... }
    fn try_collect_arr<U, E>(self) -> Result<A, E>
       where A: ArrayFromIter<U>,
             Self: Iterator<Item = Result<U, E>> { ... }
    fn try_collect_arr_trusted<U, E>(self) -> Result<A, E>
       where A: ArrayFromIter<U>,
             Self: Iterator<Item = Result<U, E>> + TrustedLen { ... }
    fn collect_arr_with_dtype(self, dtype: ArrowDataType) -> A
       where A: ArrayFromIterDtype<Self::Item> { ... }
    fn collect_arr_trusted_with_dtype(self, dtype: ArrowDataType) -> A
       where A: ArrayFromIterDtype<Self::Item>,
             Self: TrustedLen { ... }
    fn try_collect_arr_with_dtype<U, E>(
        self,
        dtype: ArrowDataType,
    ) -> Result<A, E>
       where A: ArrayFromIterDtype<U>,
             Self: Iterator<Item = Result<U, E>> { ... }
    fn try_collect_arr_trusted_with_dtype<U, E>(
        self,
        dtype: ArrowDataType,
    ) -> Result<A, E>
       where A: ArrayFromIterDtype<U>,
             Self: Iterator<Item = Result<U, E>> + TrustedLen { ... }
}
```

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#method.collect_arr" class="fn">collect_arr</a>(self) -\> A

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#method.collect_arr_trusted" class="fn">collect_arr_trusted</a>(self) -\> A

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#method.try_collect_arr" class="fn">try_collect_arr</a>\<U, E\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<A, E\>

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#method.try_collect_arr_trusted" class="fn">try_collect_arr_trusted</a>\<U, E\>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<A, E\>

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#method.collect_arr_with_dtype" class="fn">collect_arr_with_dtype</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> A

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#method.collect_arr_trusted_with_dtype" class="fn">collect_arr_trusted_with_dtype</a>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> A

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::iterator::Iterator::Item">Item</a>\>, Self: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#method.try_collect_arr_with_dtype" class="fn">try_collect_arr_with_dtype</a>\<U, E\>(self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<A, E\>

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#method.try_collect_arr_trusted_with_dtype" class="fn">try_collect_arr_trusted_with_dtype</a>\<U, E\>( self, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<A, E\>

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<U\>, Self: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<U, E\>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html#impl-ArrayCollectIterExt%3CA%3E-for-I" class="anchor">§</a>

### impl\<A, I\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayCollectIterExt.html" class="trait" title="trait polars::prelude::ArrayCollectIterExt">ArrayCollectIterExt</a>\<A\> for I

where A: <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>,
