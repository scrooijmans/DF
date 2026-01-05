# Trait BitOr Copy item path

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#254" class="src">Source</a>

``` rust
pub trait BitOr<Rhs = Self> {
    type Output;

    // Required method
    fn bitor(self, rhs: Rhs) -> Self::Output;
}
```

Expand description

The bitwise OR operator `|`.

Note that `Rhs` is `Self` by default, but this is not mandatory.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#examples" class="doc-anchor">§</a>Examples

An implementation of `BitOr` for a wrapper around `bool`.

``` rust
use std::ops::BitOr;

#[derive(Debug, PartialEq)]
struct Scalar(bool);

impl BitOr for Scalar {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

assert_eq!(Scalar(true) | Scalar(true), Scalar(true));
assert_eq!(Scalar(true) | Scalar(false), Scalar(true));
assert_eq!(Scalar(false) | Scalar(true), Scalar(true));
assert_eq!(Scalar(false) | Scalar(false), Scalar(false));
```

An implementation of `BitOr` for a wrapper around `Vec<bool>`.

``` rust
use std::ops::BitOr;

#[derive(Debug, PartialEq)]
struct BooleanVector(Vec<bool>);

impl BitOr for BooleanVector {
    type Output = Self;

    fn bitor(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        Self(
            lhs.iter()
                .zip(rhs.iter())
                .map(|(x, y)| *x | *y)
                .collect()
        )
    }
}

let bv1 = BooleanVector(vec![true, true, false, false]);
let bv2 = BooleanVector(vec![true, false, true, false]);
let expected = BooleanVector(vec![true, true, true, false]);
assert_eq!(bv1 | bv2, expected);
```

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#required-associated-types" class="anchor">§</a>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#257" class="src">Source</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a>

The resulting type after applying the `|` operator.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#required-methods" class="anchor">§</a>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#271" class="src">Source</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: Rhs) -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

Performs the `|` operation.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(true | false, true);
assert_eq!(false | false, false);
assert_eq!(5u8 | 1u8, 5);
assert_eq!(5u8 | 2u8, 7);
```

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> | <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-bool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-6" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-7" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-8" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-9" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-10" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-11" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-12" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-13" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-14" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-i256" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-15" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

1.75.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/net/ip_addr.rs.html#2476-2490" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Ipv4Addr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-16" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>

1.75.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/net/ip_addr.rs.html#2476-2490" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Ipv6Addr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-17" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-18" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-19" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-20" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-21" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-22" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-23" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-24" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-25" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-26" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-27" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-28" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Saturating%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-29" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-30" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-31" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-32" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-33" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-34" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-35" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-36" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-37" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-38" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-39" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-40" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-41" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Flags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/ffi/struct.Flags.html" class="struct" title="struct arrow_schema::ffi::Flags">Flags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-42" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/ffi/struct.Flags.html" class="struct" title="struct arrow_schema::ffi::Flags">Flags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-43" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-44" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-CreateFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/inotify/struct.CreateFlags.html" class="struct" title="struct rustix::backend::fs::inotify::CreateFlags">CreateFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-45" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/inotify/struct.CreateFlags.html" class="struct" title="struct rustix::backend::fs::inotify::CreateFlags">CreateFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-ReadFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/inotify/struct.ReadFlags.html" class="struct" title="struct rustix::backend::fs::inotify::ReadFlags">ReadFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-46" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/inotify/struct.ReadFlags.html" class="struct" title="struct rustix::backend::fs::inotify::ReadFlags">ReadFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-WatchFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/inotify/struct.WatchFlags.html" class="struct" title="struct rustix::backend::fs::inotify::WatchFlags">WatchFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-47" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/inotify/struct.WatchFlags.html" class="struct" title="struct rustix::backend::fs::inotify::WatchFlags">WatchFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Access" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.Access.html" class="struct" title="struct rustix::backend::fs::types::Access">Access</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-48" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.Access.html" class="struct" title="struct rustix::backend::fs::types::Access">Access</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-AtFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.AtFlags.html" class="struct" title="struct rustix::backend::fs::types::AtFlags">AtFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-49" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.AtFlags.html" class="struct" title="struct rustix::backend::fs::types::AtFlags">AtFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-FallocateFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.FallocateFlags.html" class="struct" title="struct rustix::backend::fs::types::FallocateFlags">FallocateFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-50" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.FallocateFlags.html" class="struct" title="struct rustix::backend::fs::types::FallocateFlags">FallocateFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-MemfdFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.MemfdFlags.html" class="struct" title="struct rustix::backend::fs::types::MemfdFlags">MemfdFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-51" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.MemfdFlags.html" class="struct" title="struct rustix::backend::fs::types::MemfdFlags">MemfdFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Mode" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.Mode.html" class="struct" title="struct rustix::backend::fs::types::Mode">Mode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-52" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.Mode.html" class="struct" title="struct rustix::backend::fs::types::Mode">Mode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-OFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.OFlags.html" class="struct" title="struct rustix::backend::fs::types::OFlags">OFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-53" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.OFlags.html" class="struct" title="struct rustix::backend::fs::types::OFlags">OFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-RenameFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.RenameFlags.html" class="struct" title="struct rustix::backend::fs::types::RenameFlags">RenameFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-54" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.RenameFlags.html" class="struct" title="struct rustix::backend::fs::types::RenameFlags">RenameFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-ResolveFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.ResolveFlags.html" class="struct" title="struct rustix::backend::fs::types::ResolveFlags">ResolveFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-55" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.ResolveFlags.html" class="struct" title="struct rustix::backend::fs::types::ResolveFlags">ResolveFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-SealFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.SealFlags.html" class="struct" title="struct rustix::backend::fs::types::SealFlags">SealFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-56" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.SealFlags.html" class="struct" title="struct rustix::backend::fs::types::SealFlags">SealFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-StatVfsMountFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.StatVfsMountFlags.html" class="struct" title="struct rustix::backend::fs::types::StatVfsMountFlags">StatVfsMountFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-57" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/fs/types/struct.StatVfsMountFlags.html" class="struct" title="struct rustix::backend::fs::types::StatVfsMountFlags">StatVfsMountFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-DupFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/io/types/struct.DupFlags.html" class="struct" title="struct rustix::backend::io::types::DupFlags">DupFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-58" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/io/types/struct.DupFlags.html" class="struct" title="struct rustix::backend::io::types::DupFlags">DupFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-FdFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/io/types/struct.FdFlags.html" class="struct" title="struct rustix::backend::io::types::FdFlags">FdFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-59" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/io/types/struct.FdFlags.html" class="struct" title="struct rustix::backend::io::types::FdFlags">FdFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-ReadWriteFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/io/types/struct.ReadWriteFlags.html" class="struct" title="struct rustix::backend::io::types::ReadWriteFlags">ReadWriteFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-60" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/backend/io/types/struct.ReadWriteFlags.html" class="struct" title="struct rustix::backend::io::types::ReadWriteFlags">ReadWriteFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-IFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/fs/ioctl/struct.IFlags.html" class="struct" title="struct rustix::fs::ioctl::IFlags">IFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-61" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/fs/ioctl/struct.IFlags.html" class="struct" title="struct rustix::fs::ioctl::IFlags">IFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-StatxAttributes" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/fs/statx/struct.StatxAttributes.html" class="struct" title="struct rustix::fs::statx::StatxAttributes">StatxAttributes</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-62" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/fs/statx/struct.StatxAttributes.html" class="struct" title="struct rustix::fs::statx::StatxAttributes">StatxAttributes</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-StatxFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/fs/statx/struct.StatxFlags.html" class="struct" title="struct rustix::fs::statx::StatxFlags">StatxFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-63" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/fs/statx/struct.StatxFlags.html" class="struct" title="struct rustix::fs::statx::StatxFlags">StatxFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-XattrFlags" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/fs/xattr/struct.XattrFlags.html" class="struct" title="struct rustix::fs::xattr::XattrFlags">XattrFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-64" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/rustix/1.0.8/x86_64-unknown-linux-gnu/rustix/fs/xattr/struct.XattrFlags.html" class="struct" title="struct rustix::fs::xattr::XattrFlags">XattrFlags</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Choice" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/subtle/2.6.1/x86_64-unknown-linux-gnu/subtle/struct.Choice.html" class="struct" title="struct subtle::Choice">Choice</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-65" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/subtle/2.6.1/x86_64-unknown-linux-gnu/subtle/struct.Choice.html" class="struct" title="struct subtle::Choice">Choice</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-B0" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B0.html" class="struct" title="struct typenum::bit::B0">B0</a>

Or with 0 ( 0 \| 0 = 0)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-66" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B0.html" class="struct" title="struct typenum::bit::B0">B0</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-LengthHint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/struct.LengthHint.html" class="struct" title="struct writeable::LengthHint">LengthHint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-67" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/writeable/0.6.1/x86_64-unknown-linux-gnu/writeable/struct.LengthHint.html" class="struct" title="struct writeable::LengthHint">LengthHint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26bool%3E-for-%26bool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-68" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26bool%3E-for-bool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-69" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i8%3E-for-%26i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-70" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i8%3E-for-i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-71" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i16%3E-for-%26i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-72" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i16%3E-for-i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-73" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i32%3E-for-%26i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-74" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i32%3E-for-i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-75" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i64%3E-for-%26i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-76" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i64%3E-for-i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-77" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i128%3E-for-%26i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-78" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26i128%3E-for-i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-79" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26isize%3E-for-%26isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-80" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26isize%3E-for-isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-81" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u8%3E-for-%26u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-82" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u8%3E-for-u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-83" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u16%3E-for-%26u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-84" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u16%3E-for-u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-85" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u32%3E-for-%26u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-86" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u32%3E-for-u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-87" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u64%3E-for-%26u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-88" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u64%3E-for-u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-89" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u128%3E-for-%26u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-90" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26u128%3E-for-u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-91" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26usize%3E-for-%26usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-92" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26usize%3E-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-93" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26BooleanBuffer%3E-for-%26BooleanBuffer" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::BooleanBuffer">BooleanBuffer</a>\> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::BooleanBuffer">BooleanBuffer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-94" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.BooleanBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::BooleanBuffer">BooleanBuffer</a>

1.75.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/net/ip_addr.rs.html#2476-2490" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Ipv4Addr%3E-for-%26Ipv4Addr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>\> for &<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-95" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>

1.75.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/net/ip_addr.rs.html#2476-2490" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Ipv4Addr%3E-for-Ipv4Addr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>\> for <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-96" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>

1.75.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/net/ip_addr.rs.html#2476-2490" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Ipv6Addr%3E-for-%26Ipv6Addr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>\> for &<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-97" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>

1.75.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/net/ip_addr.rs.html#2476-2490" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Ipv6Addr%3E-for-Ipv6Addr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>\> for <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-98" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci8%3E%3E-for-%26Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-99" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci8%3E%3E-for-Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-100" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci16%3E%3E-for-%26Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-101" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci16%3E%3E-for-Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-102" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci32%3E%3E-for-%26Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-103" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci32%3E%3E-for-Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-104" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci64%3E%3E-for-%26Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-105" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci64%3E%3E-for-Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-106" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci128%3E%3E-for-%26Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-107" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Ci128%3E%3E-for-Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-108" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cisize%3E%3E-for-%26Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-109" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cisize%3E%3E-for-Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-110" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu8%3E%3E-for-%26Saturating%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-111" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu8%3E%3E-for-Saturating%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-112" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu16%3E%3E-for-%26Saturating%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-113" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu16%3E%3E-for-Saturating%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-114" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu32%3E%3E-for-%26Saturating%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-115" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu32%3E%3E-for-Saturating%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-116" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu64%3E%3E-for-%26Saturating%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-117" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu64%3E%3E-for-Saturating%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-118" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu128%3E%3E-for-%26Saturating%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-119" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cu128%3E%3E-for-Saturating%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-120" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cusize%3E%3E-for-%26Saturating%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-121" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Saturating%3Cusize%3E%3E-for-Saturating%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-122" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci8%3E%3E-for-%26Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-123" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci8%3E%3E-for-Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-124" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci16%3E%3E-for-%26Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-125" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci16%3E%3E-for-Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-126" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci32%3E%3E-for-%26Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-127" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci32%3E%3E-for-Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-128" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci64%3E%3E-for-%26Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-129" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci64%3E%3E-for-Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-130" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci128%3E%3E-for-%26Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-131" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Ci128%3E%3E-for-Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-132" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cisize%3E%3E-for-%26Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-133" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cisize%3E%3E-for-Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-134" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu8%3E%3E-for-%26Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-135" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu8%3E%3E-for-Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-136" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu16%3E%3E-for-%26Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-137" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu16%3E%3E-for-Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-138" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu32%3E%3E-for-%26Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-139" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu32%3E%3E-for-Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-140" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu64%3E%3E-for-%26Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-141" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu64%3E%3E-for-Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-142" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu128%3E%3E-for-%26Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-143" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cu128%3E%3E-for-Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-144" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cusize%3E%3E-for-%26Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-145" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Wrapping%3Cusize%3E%3E-for-Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-146" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26BigInt%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-147" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26BigInt%3E-for-BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-148" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26BigUint%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-149" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26BigUint%3E-for-BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-150" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cbool%3E-for-%26bool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-151" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci8%3E-for-%26i8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-152" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci16%3E-for-%26i16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-153" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci32%3E-for-%26i32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-154" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci64%3E-for-%26i64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-155" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci128%3E-for-%26i128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-156" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cisize%3E-for-%26isize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-157" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu8%3E-for-%26u8" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-158" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu16%3E-for-%26u16" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-159" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu32%3E-for-%26u32" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-160" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu64%3E-for-%26u64" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-161" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu128%3E-for-%26u128" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-162" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/ops/bit.rs.html#291" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cusize%3E-for-%26usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-163" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.75.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/net/ip_addr.rs.html#2476-2490" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CIpv4Addr%3E-for-%26Ipv4Addr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>\> for &<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-164" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv4Addr.html" class="struct" title="struct core::net::ip_addr::Ipv4Addr">Ipv4Addr</a>

1.75.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/net/ip_addr.rs.html#2476-2490" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CIpv6Addr%3E-for-%26Ipv6Addr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>\> for &<a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-165" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/net/ip_addr/struct.Ipv6Addr.html" class="struct" title="struct core::net::ip_addr::Ipv6Addr">Ipv6Addr</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Ci8%3E%3E-for-%26Saturating%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-166" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Ci16%3E%3E-for-%26Saturating%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-167" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Ci32%3E%3E-for-%26Saturating%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-168" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Ci64%3E%3E-for-%26Saturating%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-169" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Ci128%3E%3E-for-%26Saturating%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-170" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Cisize%3E%3E-for-%26Saturating%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-171" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Cu8%3E%3E-for-%26Saturating%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-172" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Cu16%3E%3E-for-%26Saturating%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-173" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Cu32%3E%3E-for-%26Saturating%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-174" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Cu64%3E%3E-for-%26Saturating%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-175" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Cu128%3E%3E-for-%26Saturating%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-176" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.74.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/saturating.rs.html#551" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSaturating%3Cusize%3E%3E-for-%26Saturating%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-177" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html" class="struct" title="struct core::num::saturating::Saturating">Saturating</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Ci8%3E%3E-for-%26Wrapping%3Ci8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-178" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Ci16%3E%3E-for-%26Wrapping%3Ci16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-179" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Ci32%3E%3E-for-%26Wrapping%3Ci32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-180" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Ci64%3E%3E-for-%26Wrapping%3Ci64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-181" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Ci128%3E%3E-for-%26Wrapping%3Ci128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-182" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Cisize%3E%3E-for-%26Wrapping%3Cisize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-183" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Cu8%3E%3E-for-%26Wrapping%3Cu8%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-184" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Cu16%3E%3E-for-%26Wrapping%3Cu16%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-185" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Cu32%3E%3E-for-%26Wrapping%3Cu32%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-186" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Cu64%3E%3E-for-%26Wrapping%3Cu64%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-187" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Cu128%3E%3E-for-%26Wrapping%3Cu128%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-188" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

1.14.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/wrapping.rs.html#566" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CWrapping%3Cusize%3E%3E-for-%26Wrapping%3Cusize%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for &<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-189" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = \<<a href="https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html" class="struct" title="struct core::num::wrapping::Wrapping">Wrapping</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CBigInt%3E-for-%26BigInt" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-190" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/bigint/struct.BigInt.html" class="struct" title="struct num_bigint::bigint::BigInt">BigInt</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CBigUint%3E-for-%26BigUint" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>\> for &<a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-191" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/num-bigint/0.4.6/x86_64-unknown-linux-gnu/num_bigint/biguint/struct.BigUint.html" class="struct" title="struct num_bigint::biguint::BigUint">BigUint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CB1%3E-for-B0" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B0.html" class="struct" title="struct typenum::bit::B0">B0</a>

Or with 0 ( 0 \| 1 = 1)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-192" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-%26FixedBitSet" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for &'a <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-193" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/fixedbitset/0.5.7/x86_64-unknown-linux-gnu/fixedbitset/struct.FixedBitSet.html" class="struct" title="struct fixedbitset::FixedBitSet">FixedBitSet</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Q%3E-for-%26DashMap%3CK,+V,+S%3E" class="anchor">§</a>

### impl\<'a, K, V, S, Q\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Q</a>\> for &'a <a href="https://docs.rs/dashmap/6.1.0/x86_64-unknown-linux-gnu/dashmap/struct.DashMap.html" class="struct" title="struct dashmap::DashMap">DashMap</a>\<K, V, S\>

where K: 'a + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<Q\>, V: 'a, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, Q: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-194" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/dashmap/6.1.0/x86_64-unknown-linux-gnu/dashmap/mapref/one/struct.RefMut.html" class="struct" title="struct dashmap::mapref::one::RefMut">RefMut</a>\<'a, K, V\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Simd%3CT,+N%3E%3E-for-%26Simd%3CT,+N%3E" class="anchor">§</a>

### impl\<'lhs, 'rhs, T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&'rhs <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\> for &'lhs <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-195" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CUTerm%3E-for-UInt%3CU,+B%3E" class="anchor">§</a>

### impl\<B, U\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UTerm.html" class="struct" title="struct typenum::uint::UTerm">UTerm</a>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<U, B\>

where B: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Bit.html" class="trait" title="trait typenum::marker_traits::Bit">Bit</a>, U: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>,

`X | UTerm = X`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-196" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<U, B\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-I16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-197" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-I32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-198" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-I64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-199" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-I128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-200" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Isize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-201" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-U16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-202" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-U32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-203" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-U64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-204" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-U128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-205" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Usize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-206" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci16%3E-for-I16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-207" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci32%3E-for-I32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-208" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci64%3E-for-I64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-209" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Ci128%3E-for-I128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-210" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cisize%3E-for-Isize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-211" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu16%3E-for-U16%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-212" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu32%3E-for-U32%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-213" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu64%3E-for-U64%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-214" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cu128%3E-for-U128%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-215" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cusize%3E-for-Usize%3CO%3E" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-216" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CI16%3CO%3E%3E-for-i16" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-217" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I16.html" class="struct" title="struct zerocopy::byteorder::I16">I16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CI32%3CO%3E%3E-for-i32" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-218" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I32.html" class="struct" title="struct zerocopy::byteorder::I32">I32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CI64%3CO%3E%3E-for-i64" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-219" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I64.html" class="struct" title="struct zerocopy::byteorder::I64">I64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CI128%3CO%3E%3E-for-i128" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-220" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.I128.html" class="struct" title="struct zerocopy::byteorder::I128">I128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CIsize%3CO%3E%3E-for-isize" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-221" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Isize.html" class="struct" title="struct zerocopy::byteorder::Isize">Isize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CU16%3CO%3E%3E-for-u16" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-222" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U16.html" class="struct" title="struct zerocopy::byteorder::U16">U16</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CU32%3CO%3E%3E-for-u32" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-223" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U32.html" class="struct" title="struct zerocopy::byteorder::U32">U32</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CU64%3CO%3E%3E-for-u64" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-224" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U64.html" class="struct" title="struct zerocopy::byteorder::U64">U64</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CU128%3CO%3E%3E-for-u128" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-225" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.U128.html" class="struct" title="struct zerocopy::byteorder::U128">U128</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CUsize%3CO%3E%3E-for-usize" class="anchor">§</a>

### impl\<O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

where O: <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/trait.ByteOrder.html" class="trait" title="trait zerocopy::byteorder::ByteOrder">ByteOrder</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-226" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/zerocopy/0.8.26/x86_64-unknown-linux-gnu/zerocopy/byteorder/struct.Usize.html" class="struct" title="struct zerocopy::byteorder::Usize">Usize</a>\<O\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CRhs%3E-for-B1" class="anchor">§</a>

### impl\<Rhs\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Rhs\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>

where Rhs: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Bit.html" class="trait" title="trait typenum::marker_traits::Bit">Bit</a>,

Or with 1 ( 1 \| B = 1)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-227" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>

1.45.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#319-321" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-NonZero%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/trait.ZeroablePrimitive.html" class="trait" title="trait core::num::nonzero::ZeroablePrimitive">ZeroablePrimitive</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Output = T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-228" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<T\>

1.45.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#349-351" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CNonZero%3CT%3E%3E-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<T\>\> for T

where T: <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/trait.ZeroablePrimitive.html" class="trait" title="trait core::num::nonzero::ZeroablePrimitive">ZeroablePrimitive</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Output = T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-229" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<T\>

1.45.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143802 "Tracking issue for const_ops")) · <a href="https://doc.rust-lang.org/nightly/src/core/num/nonzero.rs.html#334-336" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CT%3E-for-NonZero%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<T\> for <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/trait.ZeroablePrimitive.html" class="trait" title="trait core::num::nonzero::ZeroablePrimitive">ZeroablePrimitive</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Output = T\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-230" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<T\>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/collections/btree/set.rs.html#1724" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26BTreeSet%3CT,+A%3E%3E-for-%26BTreeSet%3CT,+A%3E" class="anchor">§</a>

### impl\<T, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html" class="struct" title="struct alloc::collections::btree::set::BTreeSet">BTreeSet</a>\<T, A\>\> for &<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html" class="struct" title="struct alloc::collections::btree::set::BTreeSet">BTreeSet</a>\<T, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, A: <a href="https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html" class="trait" title="trait core::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-231" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html" class="struct" title="struct alloc::collections::btree::set::BTreeSet">BTreeSet</a>\<T, A\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26IndexSet%3CT,+S2%3E%3E-for-%26IndexSet%3CT,+S1%3E" class="anchor">§</a>

### impl\<T, S1, S2\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://docs.rs/indexmap/2.11.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html" class="struct" title="struct indexmap::set::IndexSet">IndexSet</a>\<T, S2\>\> for &<a href="https://docs.rs/indexmap/2.11.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html" class="struct" title="struct indexmap::set::IndexSet">IndexSet</a>\<T, S1\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, S1: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>, S2: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-232" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/indexmap/2.11.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html" class="struct" title="struct indexmap::set::IndexSet">IndexSet</a>\<T, S1\>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/collections/hash/set.rs.html#1166-1169" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26HashSet%3CT,+S%3E%3E-for-%26HashSet%3CT,+S%3E" class="anchor">§</a>

### impl\<T, S\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<T, S\>\> for &std::collections::hash::set::<a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<T, S\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-233" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<T, S\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26HashSet%3CT,+S,+A%3E%3E-for-%26HashSet%3CT,+S,+A%3E" class="anchor">§</a>

### impl\<T, S, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<T, S, A\>\> for &hashbrown::set::<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<T, S, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>, A: <a href="https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/allocator_api2/stable/alloc/trait.Allocator.html" class="trait" title="trait allocator_api2::stable::alloc::Allocator">Allocator</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-234" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<T, S, A\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26HashSet%3CT,+S,+A%3E%3E-for-%26HashSet%3CT,+S,+A%3E-1" class="anchor">§</a>

### impl\<T, S, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<T, S, A\>\> for &hashbrown::set::<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<T, S, A\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, S: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html" class="trait" title="trait core::hash::BuildHasher">BuildHasher</a> + <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>, A: <a href="https://docs.rs/allocator-api2/0.2.21/x86_64-unknown-linux-gnu/allocator_api2/stable/alloc/trait.Allocator.html" class="trait" title="trait allocator_api2::stable::alloc::Allocator">Allocator</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-235" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<T, S\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Mask%3CT,+N%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/struct.Mask.html" class="struct" title="struct core::core_simd::masks::Mask">Mask</a>\<T, N\>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/trait.MaskElement.html" class="trait" title="trait core::core_simd::masks::MaskElement">MaskElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-236" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/struct.Mask.html" class="struct" title="struct core::core_simd::masks::Mask">Mask</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3C%26Simd%3CT,+N%3E%3E-for-Simd%3CT,+N%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<&<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-237" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3Cbool%3E-for-Mask%3CT,+N%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/struct.Mask.html" class="struct" title="struct core::core_simd::masks::Mask">Mask</a>\<T, N\>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/trait.MaskElement.html" class="trait" title="trait core::core_simd::masks::MaskElement">MaskElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-238" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/struct.Mask.html" class="struct" title="struct core::core_simd::masks::Mask">Mask</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CMask%3CT,+N%3E%3E-for-bool" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/struct.Mask.html" class="struct" title="struct core::core_simd::masks::Mask">Mask</a>\<T, N\>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/trait.MaskElement.html" class="trait" title="trait core::core_simd::masks::MaskElement">MaskElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-239" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/masks/struct.Mask.html" class="struct" title="struct core::core_simd::masks::Mask">Mask</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CSimd%3CT,+N%3E%3E-for-%26Simd%3CT,+N%3E" class="anchor">§</a>

### impl\<T, const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\> for &<a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

where T: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>\>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-240" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<T, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CU%3E-for-UTerm" class="anchor">§</a>

### impl\<U\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<U\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UTerm.html" class="struct" title="struct typenum::uint::UTerm">UTerm</a>

where U: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>,

`UTerm | X = X`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-241" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = U

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CUInt%3CUr,+B0%3E%3E-for-UInt%3CUl,+B0%3E" class="anchor">§</a>

### impl\<Ul, Ur\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ur, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B0.html" class="struct" title="struct typenum::bit::B0">B0</a>\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B0.html" class="struct" title="struct typenum::bit::B0">B0</a>\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Ur\>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>,

`UInt<Ul, B0> | UInt<Ur, B0> = UInt<Ul | Ur, B0>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-242" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B0.html" class="struct" title="struct typenum::bit::B0">B0</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CUInt%3CUr,+B0%3E%3E-for-UInt%3CUl,+B1%3E" class="anchor">§</a>

### impl\<Ul, Ur\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ur, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B0.html" class="struct" title="struct typenum::bit::B0">B0</a>\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Ur\>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>,

`UInt<Ul, B1> | UInt<Ur, B0> = UInt<Ul | Ur, B1>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-243" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CUInt%3CUr,+B1%3E%3E-for-UInt%3CUl,+B0%3E" class="anchor">§</a>

### impl\<Ul, Ur\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ur, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B0.html" class="struct" title="struct typenum::bit::B0">B0</a>\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Ur\>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>,

`UInt<Ul, B0> | UInt<Ur, B1> = UInt<Ul | Ur, B1>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-244" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr%3CUInt%3CUr,+B1%3E%3E-for-UInt%3CUl,+B1%3E" class="anchor">§</a>

### impl\<Ul, Ur\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<<a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ur, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>\> for <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<Ul, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>

where Ul: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a> + <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Ur\>, Ur: <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/marker_traits/trait.Unsigned.html" class="trait" title="trait typenum::marker_traits::Unsigned">Unsigned</a>,

`UInt<Ul, B1> | UInt<Ur, B1> = UInt<Ul | Ur, B1>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-245" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/uint/struct.UInt.html" class="struct" title="struct typenum::uint::UInt">UInt</a>\<\<Ul as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>\<Ur\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::BitOr::Output">Output</a>, <a href="https://docs.rs/typenum/1.18.0/x86_64-unknown-linux-gnu/typenum/bit/struct.B1.html" class="struct" title="struct typenum::bit::B1">B1</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Ci8,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-246" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Ci16,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-247" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Ci32,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-248" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Ci64,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-249" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Cisize,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-250" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Cu8,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-251" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Cu16,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-252" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Cu32,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-253" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Cu64,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-254" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, N\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#impl-BitOr-for-Simd%3Cusize,+N%3E" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, N\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/trait.SimdElement.html" class="trait" title="trait core::core_simd::vector::SimdElement">SimdElement</a>, <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/struct.LaneCount.html" class="struct" title="struct core::core_simd::lane_count::LaneCount">LaneCount</a>\<N\>: <a href="https://doc.rust-lang.org/nightly/core/core_simd/lane_count/trait.SupportedLaneCount.html" class="trait" title="trait core::core_simd::lane_count::SupportedLaneCount">SupportedLaneCount</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output-255" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://doc.rust-lang.org/nightly/core/core_simd/vector/struct.Simd.html" class="struct" title="struct core::core_simd::vector::Simd">Simd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, N\>
