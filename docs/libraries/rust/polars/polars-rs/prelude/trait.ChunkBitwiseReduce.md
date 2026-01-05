# Trait ChunkBitwiseReduce Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/mod.rs.html#300" class="src">Source</a>

``` rust
pub trait ChunkBitwiseReduce {
    type Physical;

    // Required methods
    fn and_reduce(&self) -> Option<Self::Physical>;
    fn or_reduce(&self) -> Option<Self::Physical>;
    fn xor_reduce(&self) -> Option<Self::Physical>;
}
```

Expand description

Bitwise Reduction Operations.

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#associatedtype.Physical" class="associatedtype">Physical</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#tymethod.and_reduce" class="fn">and_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::ChunkBitwiseReduce::Physical">Physical</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#tymethod.or_reduce" class="fn">or_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::ChunkBitwiseReduce::Physical">Physical</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#tymethod.xor_reduce" class="fn">xor_reduce</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#associatedtype.Physical" class="associatedtype" title="type polars::prelude::ChunkBitwiseReduce::Physical">Physical</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#impl-ChunkBitwiseReduce-for-ChunkedArray%3CBooleanType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html" class="trait" title="trait polars::prelude::ChunkBitwiseReduce">ChunkBitwiseReduce</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#associatedtype.Physical-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#associatedtype.Physical" class="associatedtype">Physical</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#impl-ChunkBitwiseReduce-for-ChunkedArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html" class="trait" title="trait polars::prelude::ChunkBitwiseReduce">ChunkBitwiseReduce</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<T\>

where T: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>, \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/types/native/trait.NativeType.html" class="trait" title="trait polars_arrow::types::native::NativeType">NativeType</a>, <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<\<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>: <a href="https://docs.rs/polars-compute/0.51.0/x86_64-unknown-linux-gnu/polars_compute/bitwise/trait.BitwiseKernel.html" class="trait" title="trait polars_compute::bitwise::BitwiseKernel">BitwiseKernel</a>\<Scalar = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>,

<a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#associatedtype.Physical-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.ChunkBitwiseReduce.html#associatedtype.Physical" class="associatedtype">Physical</a> = \<T as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>
