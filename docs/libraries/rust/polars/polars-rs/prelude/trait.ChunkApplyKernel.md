# Trait ChunkApplyKernel Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#634" class="src">Source</a>

``` rust
pub trait ChunkApplyKernel<A>where
    A: Array,{
    // Required methods
    fn apply_kernel(&self, f: &dyn Fn(&A) -> Box<dyn Array>) -> Self;
    fn apply_kernel_cast<S>(
        &self,
        f: &dyn Fn(&A) -> Box<dyn Array>,
    ) -> ChunkedArray<S>
       where S: PolarsDataType;
}
```

Expand description

Apply kernels on the arrow array chunks in a ChunkedArray.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#tymethod.apply_kernel" class="fn">apply_kernel</a>(&self, f: &dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;A</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>) -\> Self

Apply kernel and return result as a new ChunkedArray.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#tymethod.apply_kernel_cast" class="fn">apply_kernel_cast</a>\<S\>( &self, f: &dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;A</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<S\>

where S: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsDataType.html" class="trait" title="trait polars::prelude::PolarsDataType">PolarsDataType</a>,

Apply a kernel that outputs an array of different type.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#impl-ChunkApplyKernel%3CBinaryViewArrayGeneric%3Cstr%3E%3E-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html" class="trait" title="trait polars::prelude::ChunkApplyKernel">ChunkApplyKernel</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#impl-ChunkApplyKernel%3CBinaryViewArrayGeneric%3C%5Bu8%5D%3E%3E-for-ChunkedArray%3CBinaryType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html" class="trait" title="trait polars::prelude::ChunkApplyKernel">ChunkApplyKernel</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/struct.BinaryViewArrayGeneric.html" class="struct" title="struct polars_arrow::array::binview::BinaryViewArrayGeneric">BinaryViewArrayGeneric</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#impl-ChunkApplyKernel%3CBooleanArray%3E-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html" class="trait" title="trait polars::prelude::ChunkApplyKernel">ChunkApplyKernel</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/boolean/struct.BooleanArray.html" class="struct" title="struct polars_arrow::array::boolean::BooleanArray">BooleanArray</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html#impl-ChunkApplyKernel%3CPrimitiveArray%3C%3CT+as+PolarsNumericType%3E::Native%3E%3E-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkApplyKernel.html" class="trait" title="trait polars::prelude::ChunkApplyKernel">ChunkApplyKernel</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,
