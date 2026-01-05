# Struct i256 Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/bigint/mod.rs.html#58" class="src">Source</a>

``` rust
#[repr(C)]pub struct i256 { /* private fields */ }
```

Expand description

A signed 256-bit integer

## Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

#### pub const <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The additive identity for this integer type, i.e. `0`.

#### pub const <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The multiplicative identity for this integer type, i.e. `1`.

#### pub const <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.MINUS_ONE" class="constant">MINUS_ONE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The multiplicative inverse for this integer type, i.e. `-1`.

#### pub const <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.MAX" class="constant">MAX</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The maximum value that can be represented by this integer type

#### pub const <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.MIN" class="constant">MIN</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The minimum value that can be represented by this integer type

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from_le_bytes" class="fn">from_le_bytes</a>(b: \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">32</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Create an integer value from its representation as a byte array in little-endian.

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from_be_bytes" class="fn">from_be_bytes</a>(b: \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">32</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Create an integer value from its representation as a byte array in big-endian.

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from_i128" class="fn">from_i128</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Create an `i256` value from a 128-bit value.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from_string" class="fn">from_string</a>(value_str: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Create an integer value from its representation as string.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from_f64" class="fn">from_f64</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Create an optional i256 from the provided `f64`. Returning `None` if overflow occurred

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from_parts" class="fn">from_parts</a>(low: <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>, high: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Create an i256 from the provided low u128 and high i128

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_parts" class="fn">to_parts</a>(self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>)

Returns this `i256` as a low u128 and high i128

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_i128" class="fn">to_i128</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

Converts this `i256` into an `i128` returning `None` if this would result in truncation/overflow

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.as_i128" class="fn">as_i128</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

Wraps this `i256` into an `i128`

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_le_bytes" class="fn">to_le_bytes</a>(self) -\> \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">32</a>\]

Return the memory representation of this integer as a byte array in little-endian byte order.

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_be_bytes" class="fn">to_be_bytes</a>(self) -\> \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">32</a>\]

Return the memory representation of this integer as a byte array in big-endian byte order.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.wrapping_abs" class="fn">wrapping_abs</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Computes the absolute value of this i256

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.checked_abs" class="fn">checked_abs</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Computes the absolute value of this i256 returning `None` if `Self == Self::MIN`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.wrapping_neg" class="fn">wrapping_neg</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Negates this i256

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.checked_neg" class="fn">checked_neg</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Negates this i256 returning `None` if `Self == Self::MIN`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.wrapping_add" class="fn">wrapping_add</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Performs wrapping addition

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.checked_add" class="fn">checked_add</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Performs checked addition

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.wrapping_sub" class="fn">wrapping_sub</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Performs wrapping subtraction

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.checked_sub" class="fn">checked_sub</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Performs checked subtraction

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.wrapping_mul" class="fn">wrapping_mul</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Performs wrapping multiplication

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.checked_mul" class="fn">checked_mul</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Performs checked multiplication

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.wrapping_div" class="fn">wrapping_div</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Performs wrapping division

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.checked_div" class="fn">checked_div</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Performs checked division

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.wrapping_rem" class="fn">wrapping_rem</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Performs wrapping remainder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.checked_rem" class="fn">checked_rem</a>(self, other: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Performs checked remainder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.checked_pow" class="fn">checked_pow</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Performs checked exponentiation

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.wrapping_pow" class="fn">wrapping_pow</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Performs wrapping exponentiation

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.signum" class="fn">signum</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Returns a number [`i256`](https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html "struct arrow::datatypes::i256") representing sign of this [`i256`](https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html "struct arrow::datatypes::i256").

0 if the number is zero 1 if the number is positive -1 if the number is negative

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_negative" class="fn">is_negative</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if this [`i256`](https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html "struct arrow::datatypes::i256") is negative

#### pub const fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_positive" class="fn">is_positive</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if this [`i256`](https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html "struct arrow::datatypes::i256") is positive

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Add%3C%26i256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-6" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.add-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: &'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Add%3C%26i256%3E-for-i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.add-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Add%3Ci256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.add-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Add-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.add" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html" class="trait" title="trait core::ops::arith::Add">Add</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Add::Output">Output</a>

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-AddAssign-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html" class="trait" title="trait core::ops::arith::AddAssign">AddAssign</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.add_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign" class="fn">add_assign</a>(&mut self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>)

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-ArrowNativeType-for-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize" class="fn">from_usize</a>(u: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>

Convert native integer type from usize [Read more](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.from_usize)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.as_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.as_usize" class="fn">as_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Convert to usize according to the [`as`](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) operator

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.usize_as" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.usize_as" class="fn">usize_as</a>(i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Convert from usize according to the [`as`](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) operator

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_usize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize" class="fn">to_usize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Convert native type to usize. [Read more](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_usize)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_isize-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize" class="fn">to_isize</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

Convert native type to isize. [Read more](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_isize)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_i64-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64" class="fn">to_i64</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Convert native type to i64. [Read more](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#tymethod.to_i64)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.get_byte_width" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html#method.get_byte_width" class="fn">get_byte_width</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the byte width of this native type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-ArrowNativeTypeOp-for-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.ZERO-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#associatedconstant.ZERO" class="constant">ZERO</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> = i256::ZERO

The additive identity

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.ONE-1" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#associatedconstant.ONE" class="constant">ONE</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> = i256::ONE

The multiplicative identity

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.MIN_TOTAL_ORDER" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#associatedconstant.MIN_TOTAL_ORDER" class="constant">MIN_TOTAL_ORDER</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> = i256::MIN

The minimum value and identity for the `max` aggregation. Note that the aggregation uses the total order predicate for floating point values, which means that this value is a negative NaN.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedconstant.MAX_TOTAL_ORDER" class="anchor">§</a>

#### const <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#associatedconstant.MAX_TOTAL_ORDER" class="constant">MAX_TOTAL_ORDER</a>: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> = i256::MAX

The maximum value and identity for the `min` aggregation. Note that the aggregation uses the total order predicate for floating point values, which means that this value is a positive NaN.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.add_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.add_checked" class="fn">add_checked</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked addition operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.add_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.add_wrapping" class="fn">add_wrapping</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Wrapping addition operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.sub_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.sub_checked" class="fn">sub_checked</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked subtraction operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.sub_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.sub_wrapping" class="fn">sub_wrapping</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Wrapping subtraction operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mul_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.mul_checked" class="fn">mul_checked</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked multiplication operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mul_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.mul_wrapping" class="fn">mul_wrapping</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Wrapping multiplication operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.div_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.div_checked" class="fn">div_checked</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked division operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.div_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.div_wrapping" class="fn">div_wrapping</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Wrapping division operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mod_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.mod_checked" class="fn">mod_checked</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked remainder operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mod_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.mod_wrapping" class="fn">mod_wrapping</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Wrapping remainder operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.neg_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.neg_checked" class="fn">neg_checked</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked negation operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.pow_checked" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.pow_checked" class="fn">pow_checked</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Checked exponentiation operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.pow_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.pow_wrapping" class="fn">pow_wrapping</a>(self, exp: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Wrapping exponentiation operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.neg_wrapping" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.neg_wrapping" class="fn">neg_wrapping</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Wrapping negation operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_zero" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.is_zero" class="fn">is_zero</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if zero else false

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.compare" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.compare" class="fn">compare</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

Compare operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#tymethod.is_eq" class="fn">is_eq</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Equality operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_ne" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#method.is_ne" class="fn">is_ne</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Not equal operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_lt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#method.is_lt" class="fn">is_lt</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Less than operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_le" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#method.is_le" class="fn">is_le</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Less than equals operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_gt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#method.is_gt" class="fn">is_gt</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Greater than operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.is_ge" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html#method.is_ge" class="fn">is_ge</a>(self, rhs: Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Greater than equals operation

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-AsPrimitive%3Ci256%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.AsPrimitive.html" class="trait" title="trait num_traits::cast::AsPrimitive">AsPrimitive</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.as_" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.AsPrimitive.html#tymethod.as_" class="fn">as_</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Convert a value to another, using the `as` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-BitAnd-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.bitand" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html" class="trait" title="trait core::ops::bit::BitAnd">BitAnd</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitAnd::Output">Output</a>

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-BitOr-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.bitor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html" class="trait" title="trait core::ops::bit::BitOr">BitOr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitOr::Output">Output</a>

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-BitXor-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.bitxor" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html" class="trait" title="trait core::ops::bit::BitXor">BitXor</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::BitXor::Output">Output</a>

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Clone-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Debug-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Default-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Display-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Div%3C%26i256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-18" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.div-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: &'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Div%3C%26i256%3E-for-i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-17" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.div-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Div%3Ci256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-16" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.div-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Div-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-15" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.div" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html" class="trait" title="trait core::ops::arith::Div">Div</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Div::Output">Output</a>

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-DivAssign-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html" class="trait" title="trait core::ops::arith::DivAssign">DivAssign</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.div_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign" class="fn">div_assign</a>(&mut self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>)

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-From%3Ci16%3E-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-From%3Ci256%3E-for-NativeAdapter%3CDecimal256Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-From%3Ci32%3E-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-From%3Ci64%3E-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-From%3Ci8%3E-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-FromStr-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = ParseI256Error

The associated error which can be returned from parsing.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>, \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Hash-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Mul%3C%26i256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-14" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mul-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: &'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Mul%3C%26i256%3E-for-i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-13" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mul-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Mul%3Ci256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-12" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mul-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Mul-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-11" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mul" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html" class="trait" title="trait core::ops::arith::Mul">Mul</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Mul::Output">Output</a>

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-MulAssign-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html" class="trait" title="trait core::ops::arith::MulAssign">MulAssign</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.mul_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign" class="fn">mul_assign</a>(&mut self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>)

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Neg-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html" class="trait" title="trait core::ops::arith::Neg">Neg</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-23" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.neg" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg" class="fn">neg</a>(self) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html" class="trait" title="trait core::ops::arith::Neg">Neg</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Neg::Output">Output</a>

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Ord-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-PartialEq-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-PartialOrd-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Rem%3C%26i256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-22" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.rem-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: &'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Rem%3C%26i256%3E-for-i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-21" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.rem-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Rem%3Ci256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-20" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.rem-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Rem-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-19" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.rem" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html" class="trait" title="trait core::ops::arith::Rem">Rem</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Rem::Output">Output</a>

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-RemAssign-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html" class="trait" title="trait core::ops::arith::RemAssign">RemAssign</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.rem_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign" class="fn">rem_assign</a>(&mut self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>)

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Shl%3Cu8%3E-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html" class="trait" title="trait core::ops::bit::Shl">Shl</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-24" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `<<` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.shl" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl" class="fn">shl</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html" class="trait" title="trait core::ops::bit::Shl">Shl</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::Shl::Output">Output</a>

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Shr%3Cu8%3E-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html" class="trait" title="trait core::ops::bit::Shr">Shr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-25" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `>>` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.shr" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr" class="fn">shr</a>(self, rhs: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html" class="trait" title="trait core::ops::bit::Shr">Shr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output" class="associatedtype" title="type core::ops::bit::Shr::Output">Output</a>

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Sub%3C%26i256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-10" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.sub-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: &'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<&'b <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Sub%3C%26i256%3E-for-i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-9" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.sub-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Sub%3Ci256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-8" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.sub-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Sub-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#associatedtype.Output-7" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.sub" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>) -\> \<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html" class="trait" title="trait core::ops::arith::Sub">Sub</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type core::ops::arith::Sub::Output">Output</a>

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-SubAssign-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html" class="trait" title="trait core::ops::arith::SubAssign">SubAssign</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.sub_assign" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign" class="fn">sub_assign</a>(&mut self, rhs: <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>)

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-ToPrimitive-for-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_i64" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#tymethod.to_i64" class="fn">to_i64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Converts the value of `self` to an `i64`. If the value cannot be represented by an `i64`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_f64" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_f64" class="fn">to_f64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

Converts the value of `self` to an `f64`. Overflows may map to positive or negative inifinity, otherwise `None` is returned if the value cannot be represented by an `f64`. [Read more](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_f64)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_u64" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#tymethod.to_u64" class="fn">to_u64</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Converts the value of `self` to a `u64`. If the value cannot be represented by a `u64`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_isize" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_isize" class="fn">to_isize</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

Converts the value of `self` to an `isize`. If the value cannot be represented by an `isize`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_i8" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_i8" class="fn">to_i8</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

Converts the value of `self` to an `i8`. If the value cannot be represented by an `i8`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_i16" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_i16" class="fn">to_i16</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

Converts the value of `self` to an `i16`. If the value cannot be represented by an `i16`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_i32" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_i32" class="fn">to_i32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Converts the value of `self` to an `i32`. If the value cannot be represented by an `i32`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_i128-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_i128" class="fn">to_i128</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

Converts the value of `self` to an `i128`. If the value cannot be represented by an `i128` (`i64` under the default implementation), then `None` is returned. [Read more](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_i128)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_usize" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_usize" class="fn">to_usize</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Converts the value of `self` to a `usize`. If the value cannot be represented by a `usize`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_u8" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_u8" class="fn">to_u8</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

Converts the value of `self` to a `u8`. If the value cannot be represented by a `u8`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_u16" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_u16" class="fn">to_u16</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

Converts the value of `self` to a `u16`. If the value cannot be represented by a `u16`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_u32" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_u32" class="fn">to_u32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

Converts the value of `self` to a `u32`. If the value cannot be represented by a `u32`, then `None` is returned.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_u128" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_u128" class="fn">to_u128</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

Converts the value of `self` to a `u128`. If the value cannot be represented by a `u128` (`u64` under the default implementation), then `None` is returned. [Read more](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_u128)

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#method.to_f32" class="anchor">§</a>

#### fn <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html#method.to_f32" class="fn">to_f32</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>

Converts the value of `self` to an `f32`. Overflows may map to positive or negative inifinity, otherwise `None` is returned if the value cannot be represented by an `f32`.

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Copy-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-Eq-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#impl-StructuralPartialEq-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html" class="struct" title="struct arrow::datatypes::i256">i256</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html#blanket-implementations" class="anchor">§</a>
