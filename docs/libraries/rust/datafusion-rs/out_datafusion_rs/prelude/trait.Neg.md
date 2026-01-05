# Trait Neg Copy item path

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#692" class="src">Source</a>

``` rust
pub trait Neg {
    type Output;

    // Required method
    fn neg(self) -> Self::Output;
}
```

Expand description

The unary negation operator `-`.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#examples" class="doc-anchor">§</a>Examples

An implementation of `Neg` for `Sign`, which allows the use of `-` to negate its value.

``` rust
use std::ops::Neg;

#[derive(Debug, PartialEq)]
enum Sign {
    Negative,
    Zero,
    Positive,
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Negative => Sign::Positive,
            Sign::Zero => Sign::Zero,
            Sign::Positive => Sign::Negative,
        }
    }
}

// A negative positive is a negative.
assert_eq!(-Sign::Positive, Sign::Negative);
// A double negative is a positive.
assert_eq!(-Sign::Negative, Sign::Positive);
// Zero is its own negation.
assert_eq!(-Sign::Zero, Sign::Zero);
```

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#required-associated-types" class="anchor">§</a>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#695" class="src">Source</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a>

The resulting type after applying the `-` operator.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#required-methods" class="anchor">§</a>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#708" class="src">Source</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#tymethod.neg" class="fn">neg</a>(self) -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

Performs the unary `-` operation.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#example" class="doc-anchor">§</a>Example

``` rust
let x: i32 = 12;
assert_eq!(-x, -12);
```

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#implementors" class="anchor">§</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26f16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26f32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26f64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26f128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-6" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-7" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-8" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-9" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-10" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2320-2330" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26NonZero%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-11" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2332-2342" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26NonZero%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-12" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2344-2354" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26NonZero%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-13" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2356-2366" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26NonZero%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-14" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2368-2378" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26NonZero%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-15" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2407-2417" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26NonZero%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-16" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-17" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-18" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-19" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-20" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-21" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-22" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-23" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-24" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-25" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-26" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-27" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-28" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-29" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-30" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-31" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-32" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-33" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-34" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26BigDecimal" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-35" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26bf16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-36" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26f16-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-37" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-38" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-SortProperties" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-39" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/enum.SortProperties.html" class="enum" title="enum datafusion::logical_expr::sort_properties::SortProperties">SortProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `- <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-40" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Sign" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/enum.Sign.html" class="enum" title="enum num_bigint::bigint::Sign">Sign</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-41" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/enum.Sign.html" class="enum" title="enum num_bigint::bigint::Sign">Sign</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-f16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-42" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f16.html" class="primitive">f16</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-f32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-43" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-f64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-44" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-f128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-45" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.f128.html" class="primitive">f128</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-46" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-47" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-48" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-49" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-50" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/arith.rs.html#729" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-51" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-IntervalDayTime" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-52" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-IntervalMonthDayNano" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-53" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-54" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2320-2330" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-NonZero%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-55" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2332-2342" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-NonZero%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-56" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2344-2354" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-NonZero%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-57" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2356-2366" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-NonZero%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-58" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2368-2378" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-NonZero%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-59" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

1.71.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#2407-2417" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-NonZero%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-60" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-61" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-62" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-63" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-64" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-65" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#1017" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-66" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-67" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-68" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-69" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-70" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-71" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-72" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-73" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-74" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-75" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-76" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-77" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

1.10.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-78" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-BigDecimal" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-79" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimal.html" class="struct" title="struct bigdecimal::BigDecimal">BigDecimal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-BigDecimalRef%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimalRef.html" class="struct" title="struct bigdecimal::BigDecimalRef">BigDecimalRef</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-80" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/bigdecimal/0.4.8/x86_64-unknown-linux-gnu/bigdecimal/struct.BigDecimalRef.html" class="struct" title="struct bigdecimal::BigDecimalRef">BigDecimalRef</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-TimeDelta" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-81" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-bf16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-82" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/bfloat/struct.bf16.html" class="struct" title="struct half::bfloat::bf16">bf16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-f16-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-83" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-84" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Timespec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/timespec/struct.Timespec.html" class="struct" title="struct rustix::timespec::Timespec">Timespec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-85" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/timespec/struct.Timespec.html" class="struct" title="struct rustix::timespec::Timespec">Timespec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-ATerm" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/array/struct.ATerm.html" class="struct" title="struct typenum::array::ATerm">ATerm</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-86" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/array/struct.ATerm.html" class="struct" title="struct typenum::array::ATerm">ATerm</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Z0" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.Z0.html" class="struct" title="struct typenum::int::Z0">Z0</a>

`-Z0 = Z0`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-87" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.Z0.html" class="struct" title="struct typenum::int::Z0">Z0</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Complex%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &'a <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\<Output = T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-88" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Ratio%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &'a <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\<Output = T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-89" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &'a <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-90" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26Bound%3C&#39;py,+PyComplex%3E" class="anchor">§</a>

### impl\<'py\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'py, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/complex/struct.PyComplex.html" class="struct" title="struct pyo3::types::complex::PyComplex">PyComplex</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-91" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'py, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/complex/struct.PyComplex.html" class="struct" title="struct pyo3::types::complex::PyComplex">PyComplex</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Borrowed%3C&#39;_,+&#39;py,+PyComplex%3E" class="anchor">§</a>

### impl\<'py\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Borrowed.html" class="struct" title="struct pyo3::instance::Borrowed">Borrowed</a>\<'\_, 'py, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/complex/struct.PyComplex.html" class="struct" title="struct pyo3::types::complex::PyComplex">PyComplex</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-92" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'py, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/complex/struct.PyComplex.html" class="struct" title="struct pyo3::types::complex::PyComplex">PyComplex</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Bound%3C&#39;py,+PyComplex%3E" class="anchor">§</a>

### impl\<'py\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'py, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/complex/struct.PyComplex.html" class="struct" title="struct pyo3::types::complex::PyComplex">PyComplex</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-93" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'py, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/complex/struct.PyComplex.html" class="struct" title="struct pyo3::types::complex::PyComplex">PyComplex</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-F32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F32.html" class="struct" title="struct zerocopy::byteorder::F32">F32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-94" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F32.html" class="struct" title="struct zerocopy::byteorder::F32">F32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-F64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F64.html" class="struct" title="struct zerocopy::byteorder::F64">F64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-95" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.F64.html" class="struct" title="struct zerocopy::byteorder::F64">F64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-I16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-96" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-I32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-97" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-I64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-98" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-I128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-99" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Isize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-100" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-%26NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for &<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-101" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Complex%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.Num.html" class="trait" title="trait num_traits::Num">Num</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\<Output = T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-102" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-complex/0.4.6/x86_64-unknown-linux-gnu/num_complex/struct.Complex.html" class="struct" title="struct num_complex::Complex">Complex</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Ratio%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://docs.rs/num-integer/0.1.46/x86_64-unknown-linux-gnu/num_integer/trait.Integer.html" class="trait" title="trait num_integer::Integer">Integer</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\<Output = T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-103" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-rational/0.4.2/x86_64-unknown-linux-gnu/num_rational/struct.Ratio.html" class="struct" title="struct num_rational::Ratio">Ratio</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-NotNan%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

where T: <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/float/trait.Float.html" class="trait" title="trait num_traits::float::Float">Float</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-104" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.NotNan.html" class="struct" title="struct ordered_float::NotNan">NotNan</a>\<T\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-OrderedFloat%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-105" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-NInt%3CU%3E" class="anchor">§</a>

### impl\<U\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<U\>

where U: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a>,

`-NInt = PInt`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-106" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<U\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-PInt%3CU%3E" class="anchor">§</a>

### impl\<U\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.PInt.html" class="struct" title="struct typenum::int::PInt">PInt</a>\<U\>

where U: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.NonZero.html" class="trait" title="trait typenum::marker_traits::NonZero">NonZero</a>,

`-PInt = NInt`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-107" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/int/struct.NInt.html" class="struct" title="struct typenum::int::NInt">NInt</a>\<U\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-TArr%3CV,+A%3E" class="anchor">§</a>

### impl\<V, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/array/struct.TArr.html" class="struct" title="struct typenum::array::TArr">TArr</a>\<V, A\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>, A: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-108" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/array/struct.TArr.html" class="struct" title="struct typenum::array::TArr">TArr</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>, \<A as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Simd%3Cf32,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-109" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Simd%3Cf64,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-110" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Simd%3Ci8,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-111" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Simd%3Ci16,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-112" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Simd%3Ci32,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-113" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Simd%3Ci64,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-114" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#impl-Neg-for-Simd%3Cisize,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output-115" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>, N\>
