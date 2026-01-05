# Trait ArrayFromIterDtype Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/array/static_array_collect.rs.html#20" class="src">Source</a>

``` rust
pub trait ArrayFromIterDtype<T>: Sized {
    // Required methods
    fn arr_from_iter_with_dtype<I>(dtype: ArrowDataType, iter: I) -> Self
       where I: IntoIterator<Item = T>;
    fn try_arr_from_iter_with_dtype<E, I>(
        dtype: ArrowDataType,
        iter: I,
    ) -> Result<Self, E>
       where I: IntoIterator<Item = Result<T, E>>;

    // Provided methods
    fn arr_from_iter_trusted_with_dtype<I>(
        dtype: ArrowDataType,
        iter: I,
    ) -> Self
       where I: IntoIterator<Item = T>,
             <I as IntoIterator>::IntoIter: TrustedLen { ... }
    fn try_arr_from_iter_trusted_with_dtype<E, I>(
        dtype: ArrowDataType,
        iter: I,
    ) -> Result<Self, E>
       where I: IntoIterator<Item = Result<T, E>>,
             <I as IntoIterator>::IntoIter: TrustedLen { ... }
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#tymethod.arr_from_iter_with_dtype" class="fn">arr_from_iter_with_dtype</a>\<I\>(dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#tymethod.try_arr_from_iter_with_dtype" class="fn">try_arr_from_iter_with_dtype</a>\<E, I\>( dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, iter: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>,

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#method.arr_from_iter_trusted_with_dtype" class="fn">arr_from_iter_trusted_with_dtype</a>\<I\>(dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#method.try_arr_from_iter_trusted_with_dtype" class="fn">try_arr_from_iter_trusted_with_dtype</a>\<E, I\>( dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, iter: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#impl-ArrayFromIterDtype%3COption%3C()%3E%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#impl-ArrayFromIterDtype%3COption%3CBox%3Cdyn+Array%3E%3E%3E-for-FixedSizeListArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/fixed_size_list/struct.FixedSizeListArray.html" class="struct" title="struct polars_arrow::array::fixed_size_list::FixedSizeListArray">FixedSizeListArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#impl-ArrayFromIterDtype%3C()%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#impl-ArrayFromIterDtype%3CBox%3Cdyn+Array%3E%3E-for-FixedSizeListArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/fixed_size_list/struct.FixedSizeListArray.html" class="struct" title="struct polars_arrow::array::fixed_size_list::FixedSizeListArray">FixedSizeListArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#impl-ArrayFromIterDtype%3COption%3CT%3E%3E-for-ListArray%3Ci64%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: AsArray,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#impl-ArrayFromIterDtype%3CT%3E-for-ListArray%3Ci64%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<T\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: AsArray,

<a href="https://docs.rs/polars/latest/polars/prelude/datatypes/trait.ArrayFromIterDtype.html#impl-ArrayFromIterDtype%3CT%3E-for-A" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIterDtype.html" class="trait" title="trait polars::prelude::ArrayFromIterDtype">ArrayFromIterDtype</a>\<T\> for A

where A: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/static_array/trait.ParameterFreeDtypeStaticArray.html" class="trait" title="trait polars_arrow::array::static_array::ParameterFreeDtypeStaticArray">ParameterFreeDtypeStaticArray</a> + <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<T\>,
