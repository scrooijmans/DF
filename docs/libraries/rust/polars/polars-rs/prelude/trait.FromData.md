# Trait FromData Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/default_arrays.rs.html#8" class="src">Source</a>

``` rust
pub trait FromData<T> {
    // Required method
    fn from_data_default(values: T, validity: Option<Bitmap>) -> Self;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromData.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.FromData.html#tymethod.from_data_default" class="fn">from_data_default</a>(values: T, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>) -\> Self

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromData.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromData.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromData.html#impl-FromData%3CBitmap%3E-for-BooleanArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.FromData.html" class="trait" title="trait polars::prelude::FromData">FromData</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/boolean/struct.BooleanArray.html" class="struct" title="struct polars_arrow::array::boolean::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromData.html#impl-FromData%3CBuffer%3CT%3E%3E-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.FromData.html" class="trait" title="trait polars::prelude::FromData">FromData</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/buffer/immutable/struct.Buffer.html" class="struct" title="struct polars_arrow::buffer::immutable::Buffer">Buffer</a>\<T\>\> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/primitive/struct.PrimitiveArray.html" class="struct" title="struct polars_arrow::array::primitive::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/types/native/trait.NativeType.html" class="trait" title="trait polars_arrow::types::native::NativeType">NativeType</a>,
