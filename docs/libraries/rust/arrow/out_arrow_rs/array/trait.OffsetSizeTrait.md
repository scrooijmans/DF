# Trait OffsetSizeTrait Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/list_array.rs.html#40" class="src">Source</a>

``` rust
pub trait OffsetSizeTrait:
    ArrowNativeType
    + AddAssign
    + Integer {
    const IS_LARGE: bool;
    const PREFIX: &'static str;
    const MAX_OFFSET: usize;
}
```

Expand description

A type that can be used within a variable-size array to encode offset information

See [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray"), [`LargeListArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html "type arrow::array::LargeListArray"), [`BinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryArray.html "type arrow::array::BinaryArray"), [`LargeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryArray.html "type arrow::array::LargeBinaryArray"), [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray") and [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray")

## Required Associated Constants<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#required-associated-consts" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.IS_LARGE" class="constant">IS_LARGE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

True for 64 bit offset size and false for 32 bit offset size

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Prefix for the offset size

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.MAX_OFFSET" class="constant">MAX_OFFSET</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The max `usize` offset

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#impl-OffsetSizeTrait-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.IS_LARGE-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.IS_LARGE" class="constant">IS_LARGE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> = false

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.PREFIX-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = ""

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.MAX_OFFSET-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.MAX_OFFSET" class="constant">MAX_OFFSET</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> = 2_147_483_647usize

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#impl-OffsetSizeTrait-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.IS_LARGE-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.IS_LARGE" class="constant">IS_LARGE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> = true

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.PREFIX-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.PREFIX" class="constant">PREFIX</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "Large"

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.MAX_OFFSET-2" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#associatedconstant.MAX_OFFSET" class="constant">MAX_OFFSET</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> = 9_223_372_036_854_775_807usize

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html#implementors" class="anchor">§</a>
