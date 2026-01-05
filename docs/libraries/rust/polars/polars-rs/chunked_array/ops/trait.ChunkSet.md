# Trait ChunkSet Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#130" class="src">Source</a>

``` rust
pub trait ChunkSet<'a, A, B> {
    // Required methods
    fn scatter_single<I>(
        &'a self,
        idx: I,
        opt_value: Option<A>,
    ) -> Result<Self, PolarsError>
       where I: IntoIterator<Item = u32>,
             Self: Sized;
    fn scatter_with<I, F>(&'a self, idx: I, f: F) -> Result<Self, PolarsError>
       where I: IntoIterator<Item = u32>,
             Self: Sized,
             F: Fn(Option<A>) -> Option<B>;
    fn set(
        &'a self,
        mask: &ChunkedArray<BooleanType>,
        opt_value: Option<A>,
    ) -> Result<Self, PolarsError>
       where Self: Sized;
}
```

Expand description

Create a `ChunkedArray` with new values by index or by boolean mask.

Note that these operations clone data. This is however the only way we can modify at mask or index level as the underlying Arrow arrays are immutable.

## Required Methods<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#tymethod.scatter_single" class="fn">scatter_single</a>\<I\>( &'a self, idx: I, opt_value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<A\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Set the values at indexes `idx` to some optional value `Option<T>`.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#example" class="doc-anchor">§</a>Example

``` rust
let ca = UInt32Chunked::new("a".into(), &[1, 2, 3]);
let new = ca.scatter_single(vec![0, 1], Some(10)).unwrap();

assert_eq!(Vec::from(&new), &[Some(10), Some(10), Some(3)]);
```

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#tymethod.scatter_with" class="fn">scatter_with</a>\<I, F\>(&'a self, idx: I, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<A\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<B\>,

Set the values at indexes `idx` by applying a closure to these values.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#example-1" class="doc-anchor">§</a>Example

``` rust
let ca = Int32Chunked::new("a".into(), &[1, 2, 3]);
let new = ca.scatter_with(vec![0, 1], |opt_v| opt_v.map(|v| v - 5)).unwrap();

assert_eq!(Vec::from(&new), &[Some(-4), Some(-3), Some(3)]);
```

#### fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#tymethod.set" class="fn">set</a>( &'a self, mask: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, opt_value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<A\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Set the values where the mask evaluates to `true` to some optional value `Option<T>`.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#example-2" class="doc-anchor">§</a>Example

``` rust
let ca = Int32Chunked::new("a".into(), &[1, 2, 3]);
let mask = BooleanChunked::new("mask".into(), &[false, true, false]);
let new = ca.set(&mask, Some(5)).unwrap();
assert_eq!(Vec::from(&new), &[Some(1), Some(5), Some(3)]);
```

## Implementors<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#impl-ChunkSet%3C&#39;a,+%26str,+String%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSet.html" class="trait" title="trait polars::prelude::ChunkSet">ChunkSet</a>\<'a, &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#impl-ChunkSet%3C&#39;a,+%26%5Bu8%5D,+Vec%3Cu8%3E%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSet.html" class="trait" title="trait polars::prelude::ChunkSet">ChunkSet</a>\<'a, &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#impl-ChunkSet%3C&#39;a,+bool,+bool%3E-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSet.html" class="trait" title="trait polars::prelude::ChunkSet">ChunkSet</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/trait.ChunkSet.html#impl-ChunkSet%3C&#39;a,+%3CT+as+PolarsNumericType%3E::Native,+%3CT+as+PolarsNumericType%3E::Native%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkSet.html" class="trait" title="trait polars::prelude::ChunkSet">ChunkSet</a>\<'a, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
