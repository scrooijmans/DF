# Trait ChunkApply Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#205" class="src">Source</a>

``` rust
pub trait ChunkApply<'a, T> {
    type FuncRet;

    // Required methods
    fn apply_values<F>(&'a self, f: F) -> Self
       where F: Fn(T) -> Self::FuncRet + Copy;
    fn apply<F>(&'a self, f: F) -> Self
       where F: Fn(Option<T>) -> Option<Self::FuncRet> + Copy;
    fn apply_to_slice<F, S>(&'a self, f: F, slice: &mut [S])
       where F: Fn(Option<T>, &S) -> S;
}
```

Expand description

Fastest way to do elementwise operations on a [`ChunkedArray<T>`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") when the operation is cheaper than branching due to null checking.

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype">FuncRet</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#tymethod.apply_values" class="fn">apply_values</a>\<F\>(&'a self, f: F) -\> Self

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype" title="type polars::prelude::ChunkApply::FuncRet">FuncRet</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Apply a closure elementwise. This is fastest when the null check branching is more expensive than the closure application. Often it is.

Null values remain null.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
fn double(ca: &UInt32Chunked) -> UInt32Chunked {
    ca.apply_values(|v| v * 2)
}
```

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#tymethod.apply" class="fn">apply</a>\<F\>(&'a self, f: F) -\> Self

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype" title="type polars::prelude::ChunkApply::FuncRet">FuncRet</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

Apply a closure elementwise including null values.

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#tymethod.apply_to_slice" class="fn">apply_to_slice</a>\<F, S\>(&'a self, f: F, slice: &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[S]</a>)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;S</a>) -\> S,

Apply a closure elementwise and write results to a mutable slice.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#impl-ChunkApply%3C&#39;a,+%26str%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html" class="trait" title="trait polars::prelude::ChunkApply">ChunkApply</a>\<'a, &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype">FuncRet</a> = <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#impl-ChunkApply%3C&#39;a,+%26%5Bu8%5D%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html" class="trait" title="trait polars::prelude::ChunkApply">ChunkApply</a>\<'a, &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype">FuncRet</a> = <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'a, \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#impl-ChunkApply%3C&#39;a,+bool%3E-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html" class="trait" title="trait polars::prelude::ChunkApply">ChunkApply</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype">FuncRet</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#impl-ChunkApply%3C&#39;a,+Series%3E-for-ChunkedArray%3CListType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html" class="trait" title="trait polars::prelude::ChunkApply">ChunkApply</a>\<'a, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype">FuncRet</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#impl-ChunkApply%3C&#39;a,+%3CT+as+PolarsNumericType%3E::Native%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html" class="trait" title="trait polars::prelude::ChunkApply">ChunkApply</a>\<'a, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet-5" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype">FuncRet</a> = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#impl-ChunkApply%3C&#39;a,+%26T%3E-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html" class="trait" title="trait polars::prelude::ChunkApply">ChunkApply</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet-6" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkApply.html#associatedtype.FuncRet" class="associatedtype">FuncRet</a> = T
