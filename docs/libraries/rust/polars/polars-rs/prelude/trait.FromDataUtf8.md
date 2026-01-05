# Trait FromDataUtf8 Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/default_arrays.rs.html#25" class="src">Source</a>

``` rust
pub trait FromDataUtf8 {
    // Required method
    unsafe fn from_data_unchecked_default(
        offsets: Buffer<i64>,
        values: Buffer<u8>,
        validity: Option<Bitmap>,
    ) -> Self;
}
```

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromDataUtf8.html#required-methods" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.FromDataUtf8.html#tymethod.from_data_unchecked_default" class="fn">from_data_unchecked_default</a>( offsets: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/buffer/immutable/struct.Buffer.html" class="struct" title="struct polars_arrow::buffer::immutable::Buffer">Buffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, values: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/buffer/immutable/struct.Buffer.html" class="struct" title="struct polars_arrow::buffer::immutable::Buffer">Buffer</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>, validity: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/immutable/struct.Bitmap.html" class="struct" title="struct polars_arrow::bitmap::immutable::Bitmap">Bitmap</a>\>, ) -\> Self

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.FromDataUtf8.html#safety" class="doc-anchor">§</a>Safety

`values` buffer must contain valid utf8 between every `offset`

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromDataUtf8.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromDataUtf8.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.FromDataUtf8.html#impl-FromDataUtf8-for-Utf8Array%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.FromDataUtf8.html" class="trait" title="trait polars::prelude::FromDataUtf8">FromDataUtf8</a> for <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/utf8/struct.Utf8Array.html" class="struct" title="struct polars_arrow::array::utf8::Utf8Array">Utf8Array</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>
