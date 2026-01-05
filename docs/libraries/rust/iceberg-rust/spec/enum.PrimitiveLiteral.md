# Enum PrimitiveLiteral Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/values.rs.html#65-88" class="src">Source</a>

``` rust
pub enum PrimitiveLiteral {
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(OrderedFloat<f32>),
    Double(OrderedFloat<f64>),
    String(String),
    Binary(Vec<u8>),
    Int128(i128),
    UInt128(u128),
    AboveMax,
    BelowMin,
}
```

Expand description

Values present in iceberg type

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.Boolean" class="anchor">§</a>

### Boolean(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

0x00 for false, non-zero byte for true

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.Int" class="anchor">§</a>

### Int(<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>)

Stored as 4-byte little-endian

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.Long" class="anchor">§</a>

### Long(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>)

Stored as 8-byte little-endian

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.Float" class="anchor">§</a>

### Float(<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>)

Stored as 4-byte little-endian

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.Double" class="anchor">§</a>

### Double(<a href="https://docs.rs/ordered-float/2.10.1/x86_64-unknown-linux-gnu/ordered_float/struct.OrderedFloat.html" class="struct" title="struct ordered_float::OrderedFloat">OrderedFloat</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>)

Stored as 8-byte little-endian

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.String" class="anchor">§</a>

### String(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

UTF-8 bytes (without length)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.Binary" class="anchor">§</a>

### Binary(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>)

Binary value (without length)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.Int128" class="anchor">§</a>

### Int128(<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>)

Stored as 16-byte little-endian

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.UInt128" class="anchor">§</a>

### UInt128(<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>)

Stored as 16-byte little-endian

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.AboveMax" class="anchor">§</a>

### AboveMax

When a number is larger than it can hold

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#variant.BelowMin" class="anchor">§</a>

### BelowMin

When a number is smaller than it can hold

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.is_nan" class="fn">is_nan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the Literal represents a primitive type that can be a NaN, and that it’s value is NaN

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-Clone-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-Debug-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-From%3CDatum%3E-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-Hash-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-PartialEq-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-PartialOrd-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-Eq-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#impl-StructuralPartialEq-for-PrimitiveLiteral" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html#blanket-implementations" class="anchor">§</a>
