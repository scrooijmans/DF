# Trait ArrowNativeType Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/native.rs.html#50-51" class="src">Source</a>

``` rust
pub trait ArrowNativeType:
    Debug
    + Send
    + Sync
    + Copy
    + PartialOrd
    + Default
    + Sealed
    + 'static {
    // Required methods
    fn from_usize(_: usize) -> Option<Self>;
    fn as_usize(self) -> usize;
    fn usize_as(i: usize) -> Self;
    fn to_usize(self) -> Option<usize>;
    fn to_isize(self) -> Option<isize>;
    fn to_i64(self) -> Option<i64>;

    // Provided method
    fn get_byte_width() -> usize { ... }
}
```

Expand description

Trait expressing a Rust type that has the same in-memory representation as Arrow.

This includes `i16`, `f32`, but excludes `bool` (which in arrow is represented in bits).

In little endian machines, types that implement [`ArrowNativeType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html "trait arrow::datatypes::ArrowNativeType") can be memcopied to arrow buffers as is.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#transmute-safety" class="doc-anchor">§</a>Transmute Safety

A type T implementing this trait means that any arbitrary slice of bytes of length and alignment `size_of::<T>()` can be safely interpreted as a value of that type without being unsound, i.e. potentially resulting in undefined behaviour.

Note: in the case of floating point numbers this transmutation can result in a signalling NaN, which, whilst sound, can be unwieldy. In general, whilst it is perfectly sound to reinterpret bytes as different types using this trait, it is likely unwise. For more information see [f32::from_bits](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.from_bits "associated function f32::from_bits") and [f64::from_bits](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.from_bits "associated function f64::from_bits").

Note: `bool` is restricted to `0` or `1`, and so `bool: !ArrowNativeType`

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#sealed" class="doc-anchor">§</a>Sealed

Due to the above restrictions, this trait is sealed to prevent accidental misuse

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(\_: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self\>

Convert native integer type from usize

Returns `None` if [`Self`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html "trait arrow::datatypes::ArrowNativeType") is not an integer or conversion would result in truncation/overflow

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert to usize according to the [`as`](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) operator

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Convert from usize according to the [`as`](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) operator

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Convert native type to usize.

Returns `None` if [`Self`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html "trait arrow::datatypes::ArrowNativeType") is not an integer or conversion would result in truncation/overflow

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

Convert native type to isize.

Returns `None` if [`Self`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html "trait arrow::datatypes::ArrowNativeType") is not an integer or conversion would result in truncation/overflow

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Convert native type to i64.

Returns `None` if [`Self`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html "trait arrow::datatypes::ArrowNativeType") is not an integer or conversion would result in truncation/overflow

## Provided Methods<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.get_byte_width" class="fn">get_byte_width</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the byte width of this native type.

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(\_: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(\_: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-5" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-6" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-7" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-8" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-9" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-10" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-11" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-f16" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.from_usize-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(\_: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_usize-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_isize-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.to_i64-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.as_usize-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.usize_as-12" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

## Implementors<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-IntervalDayTime" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#impl-ArrowNativeType-for-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>
