# Trait StaticArray Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/array/static_array.rs.html#19-24" class="src">Source</a>

``` rust
pub trait StaticArray:
    Array
    + for<'a> ArrayFromIterDtype<Self::ValueT<'a>>
    + for<'a> ArrayFromIterDtype<Self::ZeroableValueT<'a>>
    + for<'a> ArrayFromIterDtype<Option<Self::ValueT<'a>>>
    + Clone {
    type ValueT<'a>: Clone
       where Self: 'a;
    type ZeroableValueT<'a>: Zeroable + From<Self::ValueT<'a>>
       where Self: 'a;
    type ValueIterT<'a>: DoubleEndedIterator<Item = Self::ValueT<'a>> + TrustedLen + Send + Sync
       where Self: 'a;

Show 13 methods    // Required methods
    fn with_validity_typed(self, validity: Option<Bitmap>) -> Self;
    fn full_null(length: usize, dtype: ArrowDataType) -> Self;

    // Provided methods
    fn get(&self, idx: usize) -> Option<Self::ValueT<'_>> { ... }
    unsafe fn get_unchecked(&self, idx: usize) -> Option<Self::ValueT<'_>> { ... }
    fn last(&self) -> Option<Self::ValueT<'_>> { ... }
    fn value(&self, idx: usize) -> Self::ValueT<'_> { ... }
    unsafe fn value_unchecked(&self, idx: usize) -> Self::ValueT<'_> { ... }
    fn as_slice(&self) -> Option<&[Self::ValueT<'_>]> { ... }
    fn iter(
        &self,
    ) -> ZipValidity<Self::ValueT<'_>, Self::ValueIterT<'_>, BitmapIter<'_>> { ... }
    fn values_iter(&self) -> Self::ValueIterT<'_> { ... }
    fn from_vec(v: Vec<Self::ValueT<'_>>, dtype: ArrowDataType) -> Self { ... }
    fn from_zeroable_vec(
        v: Vec<Self::ZeroableValueT<'_>>,
        dtype: ArrowDataType,
    ) -> Self { ... }
    fn full(
        length: usize,
        value: Self::ValueT<'_>,
        dtype: ArrowDataType,
    ) -> Self { ... }
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\>: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> where Self: 'a

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\>: <a href="https://docs.rs/bytemuck/1.23.1/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html" class="trait" title="trait bytemuck::zeroable::Zeroable">Zeroable</a> + <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'a\>\> where Self: 'a

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\>: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html" class="trait" title="trait core::iter::traits::double_ended::DoubleEndedIterator">DoubleEndedIterator</a>\<Item = Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'a\>\> + <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> where Self: 'a

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#tymethod.with_validity_typed" class="fn">with_validity_typed</a>(self, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>) -\> Self

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#tymethod.full_null" class="fn">full_null</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> Self

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.get" class="fn">get</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.get_unchecked" class="fn">get_unchecked</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#safety" class="doc-anchor">§</a>Safety

It is the callers responsibility that the `idx < self.len()`.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.last" class="fn">last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.value" class="fn">value</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#safety-1" class="doc-anchor">§</a>Safety

It is the callers responsibility that the `idx < self.len()`.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.as_slice" class="fn">as_slice</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\]\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.iter" class="fn">iter</a>( &self, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/zip_validity/enum.ZipValidity.html" class="enum" title="enum polars_arrow::bitmap::utils::zip_validity::ZipValidity">ZipValidity</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>, Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype" title="type polars::prelude::StaticArray::ValueIterT">ValueIterT</a>\<'\_\>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/iterator/struct.BitmapIter.html" class="struct" title="struct polars_arrow::bitmap::utils::iterator::BitmapIter">BitmapIter</a>\<'\_\>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.values_iter" class="fn">values_iter</a>(&self) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype" title="type polars::prelude::StaticArray::ValueIterT">ValueIterT</a>\<'\_\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.from_vec" class="fn">from_vec</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> Self

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.from_zeroable_vec" class="fn">from_zeroable_vec</a>( v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype" title="type polars::prelude::StaticArray::ZeroableValueT">ZeroableValueT</a>\<'\_\>\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, ) -\> Self

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#method.full" class="fn">full</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, value: Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype" title="type polars::prelude::StaticArray::ValueT">ValueT</a>\<'\_\>, dtype: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> Self

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-BinaryArray%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binary/struct.BinaryArray.html" class="struct" title="struct polars_arrow::array::binary::BinaryArray">BinaryArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/iterator/struct.ArrayValuesIter.html" class="struct" title="struct polars_arrow::array::iterator::ArrayValuesIter">ArrayValuesIter</a>\<'a, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binary/struct.BinaryArray.html" class="struct" title="struct polars_arrow::array::binary::BinaryArray">BinaryArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-BinaryViewArrayGeneric%3Cstr%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/iterator/struct.ArrayValuesIter.html" class="struct" title="struct polars_arrow::array::iterator::ArrayValuesIter">ArrayValuesIter</a>\<'a, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-BinaryViewArrayGeneric%3C%5Bu8%5D%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/iterator/struct.ArrayValuesIter.html" class="struct" title="struct polars_arrow::array::iterator::ArrayValuesIter">ArrayValuesIter</a>\<'a, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-BooleanArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/boolean/struct.BooleanArray.html" class="struct" title="struct polars_arrow::array::boolean::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/utils/iterator/struct.BitmapIter.html" class="struct" title="struct polars_arrow::bitmap::utils::iterator::BitmapIter">BitmapIter</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-FixedSizeListArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/fixed_size_list/struct.FixedSizeListArray.html" class="struct" title="struct polars_arrow::array::fixed_size_list::FixedSizeListArray">FixedSizeListArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/iterator/struct.ArrayValuesIter.html" class="struct" title="struct polars_arrow::array::iterator::ArrayValuesIter">ArrayValuesIter</a>\<'a, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/fixed_size_list/struct.FixedSizeListArray.html" class="struct" title="struct polars_arrow::array::fixed_size_list::FixedSizeListArray">FixedSizeListArray</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-ListArray%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/iterator/struct.ArrayValuesIter.html" class="struct" title="struct polars_arrow::array::iterator::ArrayValuesIter">ArrayValuesIter</a>\<'a, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-Utf8Array%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/utf8/struct.Utf8Array.html" class="struct" title="struct polars_arrow::array::utf8::Utf8Array">Utf8Array</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-7" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/iterator/struct.ArrayValuesIter.html" class="struct" title="struct polars_arrow::array::iterator::ArrayValuesIter">ArrayValuesIter</a>\<'a, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/utf8/struct.Utf8Array.html" class="struct" title="struct polars_arrow::array::utf8::Utf8Array">Utf8Array</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-8" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/iter/sources/repeat/struct.Repeat.html" class="struct" title="struct core::iter::sources::repeat::Repeat">Repeat</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-9" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'a, T\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#impl-StaticArray-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html" class="trait" title="trait polars::prelude::StaticArray">StaticArray</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/types/native/trait.NativeType.html" class="trait" title="trait polars_arrow::types::native::NativeType">NativeType</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueT" class="associatedtype">ValueT</a>\<'a\> = T

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ZeroableValueT" class="associatedtype">ZeroableValueT</a>\<'a\> = T

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT-10" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.StaticArray.html#associatedtype.ValueIterT" class="associatedtype">ValueIterT</a>\<'a\> = <a href="https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html" class="struct" title="struct core::iter::adapters::copied::Copied">Copied</a>\<<a href="https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html" class="struct" title="struct core::slice::iter::Iter">Iter</a>\<'a, T\>\>
