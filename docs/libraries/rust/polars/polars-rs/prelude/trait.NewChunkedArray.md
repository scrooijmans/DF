# Trait NewChunkedArray Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/mod.rs.html#57" class="src">Source</a>

``` rust
pub trait NewChunkedArray<T, N> {
    // Required methods
    fn from_slice(name: PlSmallStr, v: &[N]) -> Self;
    fn from_slice_options(name: PlSmallStr, opt_v: &[Option<N>]) -> Self;
    fn from_iter_options(
        name: PlSmallStr,
        it: impl Iterator<Item = Option<N>>,
    ) -> Self;
    fn from_iter_values(name: PlSmallStr, it: impl Iterator<Item = N>) -> Self;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#tymethod.from_slice" class="fn">from_slice</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, v: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[N]</a>) -\> Self

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#tymethod.from_slice_options" class="fn">from_slice_options</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, opt_v: &\[<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>\]) -\> Self

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#tymethod.from_iter_options" class="fn">from_iter_options</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, it: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<N\>\>, ) -\> Self

Create a new ChunkedArray from an iterator.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#tymethod.from_iter_values" class="fn">from_iter_values</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, it: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = N\>) -\> Self

Create a new ChunkedArray from an iterator.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#impl-NewChunkedArray%3CBooleanType,+bool%3E-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html" class="trait" title="trait polars::prelude::NewChunkedArray">NewChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#impl-NewChunkedArray%3CBinaryType,+B%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<B\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html" class="trait" title="trait polars::prelude::NewChunkedArray">NewChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>, B\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

where B: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#impl-NewChunkedArray%3CStringType,+S%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html" class="trait" title="trait polars::prelude::NewChunkedArray">NewChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>, S\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#impl-NewChunkedArray%3CObjectType%3CT%3E,+T%3E-for-ChunkedArray%3CObjectType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html" class="trait" title="trait polars::prelude::NewChunkedArray">NewChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>, T\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ObjectType.html" class="struct" title="struct polars::prelude::ObjectType">ObjectType</a>\<T\>\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsObject.html" class="trait" title="trait polars::prelude::PolarsObject">PolarsObject</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html#impl-NewChunkedArray%3CT,+%3CT+as+PolarsNumericType%3E::Native%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.NewChunkedArray.html" class="trait" title="trait polars::prelude::NewChunkedArray">NewChunkedArray</a>\<T, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
