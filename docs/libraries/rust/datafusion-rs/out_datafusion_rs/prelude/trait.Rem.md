# Trait Rem Copy item path

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#571" class="src">Source</a>

``` rust
pub trait Rem<Rhs = Self> {
    type Output;

    // Required method
    fn rem(self, rhs: Rhs) -> Self::Output;
}
```

Expand description

The remainder operator `%`.

Note that `Rhs` is `Self` by default, but this is not mandatory.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#examples" class="doc-anchor">§</a>Examples

This example implements `Rem` on a `SplitSlice` object. After `Rem` is implemented, one can use the `%` operator to find out what the remaining elements of the slice would be after splitting it into equal slices of a given length.

``` rust
use std::ops::Rem;

#[derive(PartialEq, Debug)]
struct SplitSlice<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Rem<usize> for SplitSlice<'a, T> {
    type Output = Self;

    fn rem(self, modulus: usize) -> Self::Output {
        let len = self.slice.len();
        let rem = len % modulus;
        let start = len - rem;
        Self {slice: &self.slice[start..]}
    }
}

// If we were to divide &[0, 1, 2, 3, 4, 5, 6, 7] into slices of size 3,
// the remainder would be &[6, 7].
assert_eq!(SplitSlice { slice: &[0, 1, 2, 3, 4, 5, 6, 7] } % 3,
           SplitSlice { slice: &[6, 7] });
```

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#required-associated-types" class="anchor">§</a>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#574" class="src">Source</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a>

The resulting type after applying the `%` operator.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#required-methods" class="anchor">§</a>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#586" class="src">Source</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: Rhs) -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

Performs the `%` operation.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#example" class="doc-anchor">§</a>Example

``` rust
assert_eq!(12 % 10, 2);
```

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> % <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-f16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>

The remainder from the division of two floats.

The remainder has the same sign as the dividend and is computed as: `x - (x / y).trunc() * y`.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
let x: f32 = 50.50;
let y: f32 = 8.125;
let remainder = x - (x / y).trunc() * y;

// The answer to both operations is 1.75
assert_eq!(x % y, remainder);
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

The remainder from the division of two floats.

The remainder has the same sign as the dividend and is computed as: `x - (x / y).trunc() * y`.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
let x: f32 = 50.50;
let y: f32 = 8.125;
let remainder = x - (x / y).trunc() * y;

// The answer to both operations is 1.75
assert_eq!(x % y, remainder);
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

The remainder from the division of two floats.

The remainder has the same sign as the dividend and is computed as: `x - (x / y).trunc() * y`.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
let x: f32 = 50.50;
let y: f32 = 8.125;
let remainder = x - (x / y).trunc() * y;

// The answer to both operations is 1.75
assert_eq!(x % y, remainder);
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-f128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>

The remainder from the division of two floats.

The remainder has the same sign as the dividend and is computed as: `x - (x / y).trunc() * y`.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
let x: f32 = 50.50;
let y: f32 = 8.125;
let remainder = x - (x / y).trunc() * y;

// The answer to both operations is 1.75
assert_eq!(x % y, remainder);
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0` or if `self / other` results in overflow.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-6" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-1" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0` or if `self / other` results in overflow.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-7" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-2" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0` or if `self / other` results in overflow.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-8" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-3" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0` or if `self / other` results in overflow.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-9" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-4" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0` or if `self / other` results in overflow.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-10" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-5" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0` or if `self / other` results in overflow.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-11" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-6" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-12" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-7" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-13" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-8" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-14" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-9" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-15" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-10" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-16" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

This operation satisfies `n % d == n - (n / d) * d`. The result has the same sign as the left operand.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#panics-11" class="doc-anchor">§</a>Panics

This operation will panic if `other == 0`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-17" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-IntervalDayTime" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-18" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-19" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-20" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-21" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-22" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-23" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-24" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-25" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-26" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-27" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-28" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-29" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-30" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-31" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Saturating%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-32" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-33" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-34" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-35" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-36" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-37" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-38" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-39" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-40" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-41" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-42" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-43" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

1.7.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-44" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-BigDecimal" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-45" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-bf16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-46" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-f16-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-47" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-48" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-49" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f16%3E-for-%26f16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-50" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f16%3E-for-f16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-51" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f32%3E-for-%26f32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-52" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f32%3E-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-53" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f64%3E-for-%26f64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-54" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f64%3E-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-55" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f128%3E-for-%26f128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-56" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f128%3E-for-f128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-57" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i8%3E-for-%26i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-58" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i8%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-59" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i8%3E-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-60" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i8%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-61" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i16%3E-for-%26i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-62" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i16%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-63" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i16%3E-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-64" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i16%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-65" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i32%3E-for-%26i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-66" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i32%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-67" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i32%3E-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-68" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i32%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-69" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i64%3E-for-%26i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-70" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i64%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-71" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i64%3E-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-72" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i64%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-73" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i128%3E-for-%26i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-74" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i128%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-75" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i128%3E-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-76" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i128%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-77" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26isize%3E-for-%26isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-78" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26isize%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-79" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26isize%3E-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-80" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26isize%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-81" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u8%3E-for-%26u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-82" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u8%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-83" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u8%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-84" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u8%3E-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-85" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u8%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-86" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u8%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-87" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u16%3E-for-%26u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-88" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u16%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-89" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u16%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-90" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u16%3E-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-91" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u16%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-92" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u16%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-93" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u32%3E-for-%26u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-94" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u32%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-95" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u32%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-96" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u32%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-97" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u32%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-98" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u32%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-99" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u64%3E-for-%26u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-100" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u64%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-101" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u64%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-102" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u64%3E-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-103" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u64%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-104" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u64%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-105" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u128%3E-for-%26u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-106" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u128%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-107" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u128%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-108" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u128%3E-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-109" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u128%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-110" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26u128%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-111" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26usize%3E-for-%26usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-112" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26usize%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-113" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26usize%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-114" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26usize%3E-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-115" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26usize%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-116" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26usize%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-117" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci8%3E%3E-for-%26Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-118" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci8%3E%3E-for-Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-119" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci16%3E%3E-for-%26Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-120" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci16%3E%3E-for-Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-121" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci32%3E%3E-for-%26Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-122" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci32%3E%3E-for-Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-123" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci64%3E%3E-for-%26Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-124" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci64%3E%3E-for-Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-125" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci128%3E%3E-for-%26Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-126" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Ci128%3E%3E-for-Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-127" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cisize%3E%3E-for-%26Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-128" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cisize%3E%3E-for-Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-129" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu8%3E%3E-for-%26Saturating%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-130" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu8%3E%3E-for-Saturating%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-131" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu16%3E%3E-for-%26Saturating%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-132" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu16%3E%3E-for-Saturating%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-133" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu32%3E%3E-for-%26Saturating%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-134" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu32%3E%3E-for-Saturating%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-135" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu64%3E%3E-for-%26Saturating%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-136" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu64%3E%3E-for-Saturating%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-137" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu128%3E%3E-for-%26Saturating%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-138" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cu128%3E%3E-for-Saturating%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-139" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cusize%3E%3E-for-%26Saturating%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-140" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Saturating%3Cusize%3E%3E-for-Saturating%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-141" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci8%3E%3E-for-%26Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-142" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci8%3E%3E-for-Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-143" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci16%3E%3E-for-%26Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-144" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci16%3E%3E-for-Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-145" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci32%3E%3E-for-%26Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-146" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci32%3E%3E-for-Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-147" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci64%3E%3E-for-%26Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-148" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci64%3E%3E-for-Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-149" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci128%3E%3E-for-%26Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-150" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Ci128%3E%3E-for-Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-151" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cisize%3E%3E-for-%26Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-152" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cisize%3E%3E-for-Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-153" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu8%3E%3E-for-%26Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-154" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu8%3E%3E-for-Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-155" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu16%3E%3E-for-%26Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-156" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu16%3E%3E-for-Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-157" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu32%3E%3E-for-%26Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-158" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu32%3E%3E-for-Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-159" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu64%3E%3E-for-%26Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-160" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu64%3E%3E-for-Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-161" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu128%3E%3E-for-%26Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-162" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cu128%3E%3E-for-Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-163" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cusize%3E%3E-for-%26Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-164" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Wrapping%3Cusize%3E%3E-for-Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-165" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigDecimal%3E-for-%26BigDecimal" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>\> for &<a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-166" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigDecimal%3E-for-BigDecimal" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>\> for <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-167" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26bf16%3E-for-%26bf16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>\> for &<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-168" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26bf16%3E-for-bf16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>\> for <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-169" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f16%3E-for-%26f16-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>\> for &<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-170" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26f16%3E-for-f16-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>\> for <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-171" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-172" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-173" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-174" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-175" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-176" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-177" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-178" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-179" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-180" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-181" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-182" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-183" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-184" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-185" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-186" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-187" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-188" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-189" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-190" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-191" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-192" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-193" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-194" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-195" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-196" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigInt%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-197" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-%26u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-198" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-%26u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-199" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-%26u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-200" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-%26u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-201" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-%26u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-202" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-%26usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-203" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-204" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-205" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-206" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-207" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-208" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-209" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-210" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26BigUint%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-211" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cf16%3E-for-%26f16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-212" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cf32%3E-for-%26f32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-213" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cf64%3E-for-%26f64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-214" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#650" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cf128%3E-for-%26f128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-215" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci8%3E-for-%26i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-216" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci8%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-217" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci8%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-218" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci16%3E-for-%26i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-219" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci16%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-220" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci16%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-221" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci32%3E-for-%26i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-222" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci32%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-223" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci32%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-224" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci64%3E-for-%26i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-225" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci64%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-226" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci64%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-227" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci128%3E-for-%26i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-228" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci128%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-229" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci128%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-230" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cisize%3E-for-%26isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-231" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cisize%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-232" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cisize%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-233" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu8%3E-for-%26u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-234" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu8%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-235" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu8%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-236" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu8%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-237" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu8%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-238" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu16%3E-for-%26u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-239" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu16%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-240" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu16%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-241" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu16%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-242" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu16%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-243" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu32%3E-for-%26u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-244" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu32%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-245" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu32%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-246" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu32%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-247" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu32%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-248" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu64%3E-for-%26u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-249" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu64%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-250" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu64%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-251" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu64%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-252" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu64%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-253" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu128%3E-for-%26u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-254" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu128%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-255" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu128%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-256" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu128%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-257" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu128%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-258" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#613-616" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cusize%3E-for-%26usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-259" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cusize%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-260" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cusize%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-261" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cusize%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-262" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cusize%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-263" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.51.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2221-2231" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNonZero%3Cu8%3E%3E-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-264" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

1.51.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2233-2243" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNonZero%3Cu16%3E%3E-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-265" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

1.51.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2245-2255" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNonZero%3Cu32%3E%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-266" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

1.51.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2257-2267" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNonZero%3Cu64%3E%3E-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-267" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

1.51.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2269-2279" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNonZero%3Cu128%3E%3E-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-268" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

1.51.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2308-2318" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNonZero%3Cusize%3E%3E-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-269" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Ci8%3E%3E-for-%26Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-270" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Ci16%3E%3E-for-%26Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-271" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Ci32%3E%3E-for-%26Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-272" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Ci64%3E%3E-for-%26Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-273" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Ci128%3E%3E-for-%26Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-274" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Cisize%3E%3E-for-%26Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-275" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Cu8%3E%3E-for-%26Saturating%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-276" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Cu16%3E%3E-for-%26Saturating%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-277" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Cu32%3E%3E-for-%26Saturating%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-278" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Cu64%3E%3E-for-%26Saturating%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-279" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Cu128%3E%3E-for-%26Saturating%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-280" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSaturating%3Cusize%3E%3E-for-%26Saturating%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-281" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Ci8%3E%3E-for-%26Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-282" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Ci16%3E%3E-for-%26Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-283" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Ci32%3E%3E-for-%26Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-284" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Ci64%3E%3E-for-%26Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-285" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Ci128%3E%3E-for-%26Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-286" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Cisize%3E%3E-for-%26Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-287" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Cu8%3E%3E-for-%26Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-288" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Cu16%3E%3E-for-%26Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-289" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Cu32%3E%3E-for-%26Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-290" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Cu64%3E%3E-for-%26Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-291" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Cu128%3E%3E-for-%26Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-292" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CWrapping%3Cusize%3E%3E-for-%26Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-293" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigDecimal%3E-for-%26BigDecimal" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>\> for &<a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-294" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cbf16%3E-for-%26bf16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>\> for &<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-295" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cf16%3E-for-%26f16-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>\> for &<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-296" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-297" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-298" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-299" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-300" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-301" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-302" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-303" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-304" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-305" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-306" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-307" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-308" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-309" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-310" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-311" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-312" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-313" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-314" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-315" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-316" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-317" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-318" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-319" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-320" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigInt%3E-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-321" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-%26u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-322" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-%26u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-323" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-%26u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-324" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-%26u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-325" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-%26u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-326" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-%26usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-327" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-328" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-329" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-330" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-331" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-332" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-333" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CBigUint%3E-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-334" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cf32%3E%3E-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-335" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cf64%3E%3E-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-336" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci8%3E%3E-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-337" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci16%3E%3E-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-338" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci32%3E%3E-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-339" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci64%3E%3E-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-340" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci128%3E%3E-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-341" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cisize%3E%3E-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-342" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu8%3E%3E-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-343" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu16%3E%3E-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-344" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu32%3E%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-345" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu64%3E%3E-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-346" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu128%3E%3E-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-347" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cusize%3E%3E-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-348" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26IntervalDayTime%3E-for-IntervalDayTime" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-349" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26IntervalMonthDayNano%3E-for-IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-350" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i256%3E-for-i256" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-351" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cf32%3E%3E-for-f32" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-352" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cf64%3E%3E-for-f64" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-353" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci8%3E%3E-for-i8" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-354" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci16%3E%3E-for-i16" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-355" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci32%3E%3E-for-i32" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-356" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci64%3E%3E-for-i64" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-357" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci128%3E%3E-for-i128" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-358" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cisize%3E%3E-for-isize" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-359" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu8%3E%3E-for-u8" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-360" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu16%3E%3E-for-u16" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-361" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu32%3E%3E-for-u32" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-362" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu64%3E%3E-for-u64" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-363" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu128%3E%3E-for-u128" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-364" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cusize%3E%3E-for-usize" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-365" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CIntervalDayTime%3E-for-%26IntervalDayTime" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-366" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CIntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-367" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-368" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cf32%3E%3E-for-%26f32" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-369" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cf64%3E%3E-for-%26f64" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-370" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci8%3E%3E-for-%26i8" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-371" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci16%3E%3E-for-%26i16" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-372" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci32%3E%3E-for-%26i32" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-373" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci64%3E%3E-for-%26i64" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-374" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Ci128%3E%3E-for-%26i128" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-375" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cisize%3E%3E-for-%26isize" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-376" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu8%3E%3E-for-%26u8" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-377" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu16%3E%3E-for-%26u16" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-378" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu32%3E%3E-for-%26u32" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-379" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu64%3E%3E-for-%26u64" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-380" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cu128%3E%3E-for-%26u128" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-381" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3Cusize%3E%3E-for-%26usize" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-382" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cf32%3E%3E-for-%26f32" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-383" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cf64%3E%3E-for-%26f64" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-384" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci8%3E%3E-for-%26i8" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-385" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci16%3E%3E-for-%26i16" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-386" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci32%3E%3E-for-%26i32" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-387" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci64%3E%3E-for-%26i64" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-388" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Ci128%3E%3E-for-%26i128" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-389" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cisize%3E%3E-for-%26isize" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-390" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu8%3E%3E-for-%26u8" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-391" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu16%3E%3E-for-%26u16" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-392" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu32%3E%3E-for-%26u32" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-393" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu64%3E%3E-for-%26u64" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-394" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cu128%3E%3E-for-%26u128" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-395" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3Cusize%3E%3E-for-%26usize" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-396" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26IntervalDayTime%3E-for-%26IntervalDayTime" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-397" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26IntervalMonthDayNano%3E-for-%26IntervalMonthDayNano" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-398" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26i256%3E-for-%26i256" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'b <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-399" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3CT%3E%3E-for-%26Complex%3CT%3E" class="anchor">§</a>

### impl\<'a, 'b, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'b <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>\> for &'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-400" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Ratio%3CT%3E%3E-for-%26Ratio%3CT%3E" class="anchor">§</a>

### impl\<'a, 'b, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'b <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>\> for &'a <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-401" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26T%3E-for-%26Complex%3CT%3E" class="anchor">§</a>

### impl\<'a, 'b, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for &'b <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-402" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26T%3E-for-%26Ratio%3CT%3E" class="anchor">§</a>

### impl\<'a, 'b, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'b T</a>\> for &'a <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-403" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-%26OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for &'a <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-404" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Complex%3CT%3E%3E-for-Complex%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>\> for <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-405" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Ratio%3CT%3E%3E-for-Ratio%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>\> for <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-406" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26OrderedFloat%3CT%3E%3E-for-OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'a <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>\> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-407" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26T%3E-for-%26OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for &'a <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-408" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26T%3E-for-Complex%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-409" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26T%3E-for-Ratio%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-410" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26T%3E-for-OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-411" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CComplex%3CT%3E%3E-for-%26Complex%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>\> for &'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-412" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CRatio%3CT%3E%3E-for-%26Ratio%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>\> for &'a <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-413" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3COrderedFloat%3CT%3E%3E-for-%26OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>\> for &'a <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-414" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CT%3E-for-%26Complex%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\> for &'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-415" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CT%3E-for-%26Ratio%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\> for &'a <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-416" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CT%3E-for-%26OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\> for &'a <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-417" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Simd%3CT,+N%3E%3E-for-%26Simd%3CT,+N%3E" class="anchor">§</a>

### impl\<'lhs, 'rhs, T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&'rhs <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\> for &'lhs <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-418" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CI%3E-for-Z0" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<I\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.Z0.html" class="struct" title="struct typenum::int::Z0">Z0</a>

where I: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Integer.html" class="trait" title="trait typenum::marker_traits::Integer">Integer</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a>,

`Z0 % I = Z0` where `I != 0`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-419" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.Z0.html" class="struct" title="struct typenum::int::Z0">Z0</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-F32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F32.html" class="struct" title="struct zerocopy::byteorder::F32">F32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-420" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F32.html" class="struct" title="struct zerocopy::byteorder::F32">F32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-F64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F64.html" class="struct" title="struct zerocopy::byteorder::F64">F64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-421" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F64.html" class="struct" title="struct zerocopy::byteorder::F64">F64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-I16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-422" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-I32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-423" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-I64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-424" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-I128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-425" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Isize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-426" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-U16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-427" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-U32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-428" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-U64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-429" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-U128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-430" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Usize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-431" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cf32%3E-for-F32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F32.html" class="struct" title="struct zerocopy::byteorder::F32">F32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-432" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F32.html" class="struct" title="struct zerocopy::byteorder::F32">F32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cf64%3E-for-F64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F64.html" class="struct" title="struct zerocopy::byteorder::F64">F64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-433" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F64.html" class="struct" title="struct zerocopy::byteorder::F64">F64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci16%3E-for-I16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-434" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci32%3E-for-I32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-435" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci64%3E-for-I64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-436" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Ci128%3E-for-I128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-437" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cisize%3E-for-Isize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-438" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu16%3E-for-U16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-439" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu32%3E-for-U32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-440" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu64%3E-for-U64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-441" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cu128%3E-for-U128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-442" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3Cusize%3E-for-Usize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-443" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CF32%3CO%3E%3E-for-f32" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F32.html" class="struct" title="struct zerocopy::byteorder::F32">F32</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-444" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F32.html" class="struct" title="struct zerocopy::byteorder::F32">F32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CF64%3CO%3E%3E-for-f64" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F64.html" class="struct" title="struct zerocopy::byteorder::F64">F64</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-445" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F64.html" class="struct" title="struct zerocopy::byteorder::F64">F64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CI16%3CO%3E%3E-for-i16" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-446" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CI32%3CO%3E%3E-for-i32" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-447" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CI64%3CO%3E%3E-for-i64" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-448" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CI128%3CO%3E%3E-for-i128" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-449" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CIsize%3CO%3E%3E-for-isize" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-450" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CU16%3CO%3E%3E-for-u16" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-451" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CU32%3CO%3E%3E-for-u32" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-452" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CU64%3CO%3E%3E-for-u64" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-453" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CU128%3CO%3E%3E-for-u128" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-454" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CUsize%3CO%3E%3E-for-usize" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-455" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CRhs%3E-for-ATerm" class="anchor">§</a>

### impl\<Rhs\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Rhs\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/array/struct.ATerm.html" class="struct" title="struct typenum::array::ATerm">ATerm</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-456" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/array/struct.ATerm.html" class="struct" title="struct typenum::array::ATerm">ATerm</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-%26NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for &<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-457" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Complex%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-458" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Ratio%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-459" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-460" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-461" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26NotNan%3CT%3E%3E-for-NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>\> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-462" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26T%3E-for-%26NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\> for &<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-463" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26T%3E-for-NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-464" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNotNan%3CT%3E%3E-for-%26NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>\> for &<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-465" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CT%3E-for-%26NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\> for &<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-466" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CT%3E-for-Complex%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\> for <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-467" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CT%3E-for-Ratio%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\> for <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-468" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CT%3E-for-NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

Calculates `%` with a float directly.

Panics if the provided value is NaN or the computation results in NaN

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-469" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CT%3E-for-OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<T\> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-470" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3C%26Simd%3CT,+N%3E%3E-for-Simd%3CT,+N%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<&<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-471" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CSimd%3CT,+N%3E%3E-for-%26Simd%3CT,+N%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\> for &<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-472" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CUInt%3CUr,+Br%3E%3E-for-UInt%3CUl,+Bl%3E" class="anchor">§</a>

### impl\<Ul, Bl, Ur, Br\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ur, Br\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, Bl\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>, Bl: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Bit.html" class="trait" title="trait typenum::marker_traits::Bit">Bit</a>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>, Br: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Bit.html" class="trait" title="trait typenum::marker_traits::Bit">Bit</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, Bl\>: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Len.html" class="trait" title="trait typenum::type_operators::Len">Len</a>, \<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, Bl\> as <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Len.html" class="trait" title="trait typenum::type_operators::Len">Len</a>\>::<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Len.html#associatedtype.Output" class="associatedtype" title="type typenum::type_operators::Len::Output">Output</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>: PrivateDiv\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, Bl\>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ur, Br\>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UTerm.html" class="struct" title="struct typenum::uint::UTerm">UTerm</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UTerm.html" class="struct" title="struct typenum::uint::UTerm">UTerm</a>, \<\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, Bl\> as <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Len.html" class="trait" title="trait typenum::type_operators::Len">Len</a>\>::<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Len.html#associatedtype.Output" class="associatedtype" title="type typenum::type_operators::Len::Output">Output</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Sub::Output">Output</a>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-473" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a> as PrivateDiv\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, Bl\>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ur, Br\>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UTerm.html" class="struct" title="struct typenum::uint::UTerm">UTerm</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UTerm.html" class="struct" title="struct typenum::uint::UTerm">UTerm</a>, \<\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, Bl\> as <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Len.html" class="trait" title="trait typenum::type_operators::Len">Len</a>\>::<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Len.html#associatedtype.Output" class="associatedtype" title="type typenum::type_operators::Len::Output">Output</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Sub::Output">Output</a>\>\>::Remainder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNInt%3CUr%3E%3E-for-NInt%3CUl%3E" class="anchor">§</a>

### impl\<Ul, Ur\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ur\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ul\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ul\>: PrivateRem\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ur\>\>,

`$A<Ul> % $B<Ur> = $R<Ul % Ur>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-474" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ul\> as PrivateRem\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ur\>\>\>::Output

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CNInt%3CUr%3E%3E-for-PInt%3CUl%3E" class="anchor">§</a>

### impl\<Ul, Ur\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ur\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ul\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ul\>: PrivateRem\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ur\>\>,

`$A<Ul> % $B<Ur> = $R<Ul % Ur>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-475" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ul\> as PrivateRem\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ur\>\>\>::Output

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CPInt%3CUr%3E%3E-for-NInt%3CUl%3E" class="anchor">§</a>

### impl\<Ul, Ur\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ur\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ul\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ul\>: PrivateRem\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ur\>\>,

`$A<Ul> % $B<Ur> = $R<Ul % Ur>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-476" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<Ul\> as PrivateRem\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ur\>\>\>::Output

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CPInt%3CUr%3E%3E-for-PInt%3CUl%3E" class="anchor">§</a>

### impl\<Ul, Ur\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ur\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ul\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ul\>: PrivateRem\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ur\>\>,

`$A<Ul> % $B<Ur> = $R<Ul % Ur>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-477" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ul\> as PrivateRem\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<Ur\>\>\>::Output

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CUInt%3CUr,+Br%3E%3E-for-UTerm" class="anchor">§</a>

### impl\<Ur, Br\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ur, Br\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UTerm.html" class="struct" title="struct typenum::uint::UTerm">UTerm</a>

where Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>, Br: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Bit.html" class="trait" title="trait typenum::marker_traits::Bit">Bit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-478" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UTerm.html" class="struct" title="struct typenum::uint::UTerm">UTerm</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem%3CRhs%3E-for-TArr%3CV,+A%3E" class="anchor">§</a>

### impl\<V, A, Rhs\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Rhs\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/array/struct.TArr.html" class="struct" title="struct typenum::array::TArr">TArr</a>\<V, A\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Rhs\>, A: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Rhs\>, Rhs: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-479" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/array/struct.TArr.html" class="struct" title="struct typenum::array::TArr">TArr</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Rhs\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>, \<A as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>\<Rhs\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Rem::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Cf32,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-480" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Cf64,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-481" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Ci8,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-482" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Ci16,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-483" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Ci32,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-484" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Ci64,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-485" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Cisize,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-486" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Cu8,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-487" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Cu16,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-488" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Cu32,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-489" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Cu64,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-490" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#impl-Rem-for-Simd%3Cusize,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output-491" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, N\>
