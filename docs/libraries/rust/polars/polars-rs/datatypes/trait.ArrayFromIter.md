# Trait ArrayFromIter Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/array/static_array_collect.rs.html#47" class="src">Source</a>

``` rust
pub trait ArrayFromIter<T>: Sized {
    // Required methods
    fn arr_from_iter<I>(iter: I) -> Self
       where I: IntoIterator<Item = T>;
    fn try_arr_from_iter<E, I>(iter: I) -> Result<Self, E>
       where I: IntoIterator<Item = Result<T, E>>;

    // Provided methods
    fn arr_from_iter_trusted<I>(iter: I) -> Self
       where I: IntoIterator<Item = T>,
             <I as IntoIterator>::IntoIter: TrustedLen { ... }
    fn try_arr_from_iter_trusted<E, I>(iter: I) -> Result<Self, E>
       where I: IntoIterator<Item = Result<T, E>>,
             <I as IntoIterator>::IntoIter: TrustedLen { ... }
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#tymethod.arr_from_iter" class="fn">arr_from_iter</a>\<I\>(iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>,

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#tymethod.try_arr_from_iter" class="fn">try_arr_from_iter</a>\<E, I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>,

## Provided Methods<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#method.arr_from_iter_trusted" class="fn">arr_from_iter_trusted</a>\<I\>(iter: I) -\> Self

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#method.try_arr_from_iter_trusted" class="fn">try_arr_from_iter_trusted</a>\<E, I\>(iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, E\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, E\>\>, \<I as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/trusted_len/trait.TrustedLen.html" class="trait" title="trait polars_arrow::trusted_len::TrustedLen">TrustedLen</a>,

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3Cbool%3E%3E-for-BooleanArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/boolean/struct.BooleanArray.html" class="struct" title="struct polars_arrow::array::boolean::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3C()%3E%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3Cbool%3E-for-BooleanArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/boolean/struct.BooleanArray.html" class="struct" title="struct polars_arrow::array::boolean::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3C()%3E-for-StructArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3C%26T%3E%3E-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>\> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3C%26T%3E-for-ObjectArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for <a href="https://docs.rs/polars/latest/polars/chunked_array/object/struct.ObjectArray.html" class="struct" title="struct polars::chunked_array::object::ObjectArray">ObjectArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3CT%3E%3E-for-BinaryArray%3Ci64%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binary/struct.BinaryArray.html" class="struct" title="struct polars_arrow::array::binary::BinaryArray">BinaryArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: IntoBytes,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3CT%3E%3E-for-BinaryViewArrayGeneric%3Cstr%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

where T: StrIntoBytes,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3CT%3E%3E-for-BinaryViewArrayGeneric%3C%5Bu8%5D%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

where T: IntoBytes,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3CT%3E%3E-for-ListArray%3Ci64%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/list/struct.ListArray.html" class="struct" title="struct polars_arrow::array::list::ListArray">ListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: AsArray,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3CT%3E%3E-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/types/native/trait.NativeType.html" class="trait" title="trait polars_arrow::types::native::NativeType">NativeType</a>,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3COption%3CT%3E%3E-for-Utf8Array%3Ci64%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/utf8/struct.Utf8Array.html" class="struct" title="struct polars_arrow::array::utf8::Utf8Array">Utf8Array</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: StrIntoBytes,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3CT%3E-for-BinaryArray%3Ci64%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<T\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binary/struct.BinaryArray.html" class="struct" title="struct polars_arrow::array::binary::BinaryArray">BinaryArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: IntoBytes,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3CT%3E-for-BinaryViewArrayGeneric%3Cstr%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<T\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

where T: StrIntoBytes,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3CT%3E-for-BinaryViewArrayGeneric%3C%5Bu8%5D%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<T\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

where T: IntoBytes,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3CT%3E-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<T\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/types/native/trait.NativeType.html" class="trait" title="trait polars_arrow::types::native::NativeType">NativeType</a>,

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.ArrayFromIter.html#impl-ArrayFromIter%3CT%3E-for-Utf8Array%3Ci64%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ArrayFromIter.html" class="trait" title="trait polars::prelude::ArrayFromIter">ArrayFromIter</a>\<T\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/utf8/struct.Utf8Array.html" class="struct" title="struct polars_arrow::array::utf8::Utf8Array">Utf8Array</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

where T: StrIntoBytes,
