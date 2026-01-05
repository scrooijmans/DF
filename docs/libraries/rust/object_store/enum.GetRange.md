# Enum GetRange Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/util.rs.html#193-208" class="src">Source</a>

``` rust
pub enum GetRange {
    Bounded(Range<u64>),
    Offset(u64),
    Suffix(u64),
}
```

Expand description

Request only a portion of an object’s bytes

These can be created from [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize") ranges, like

``` rust
let range1: GetRange = (50..150).into();
let range2: GetRange = (50..=150).into();
let range3: GetRange = (50..).into();
let range4: GetRange = (..150).into();
```

Implementations may wish to inspect [`GetResult`](https://docs.rs/object_store/latest/object_store/struct.GetResult.html "struct object_store::GetResult") for the exact byte range returned.

## Variants<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#variants" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#variant.Bounded" class="anchor">§</a>

### Bounded(<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>)

Request a specific range of bytes

If the given range is zero-length or starts after the end of the object, an error will be returned. Additionally, if the range ends after the end of the object, the entire remainder of the object will be returned. Otherwise, the exact requested range will be returned.

Note that range is u64 (i.e., not usize), as `object_store` supports 32-bit architectures such as WASM

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#variant.Offset" class="anchor">§</a>

### Offset(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Request all bytes starting from a given byte offset

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#variant.Suffix" class="anchor">§</a>

### Suffix(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Request up to the last n bytes

## Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#impl-GetRange" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.is_valid" class="fn">is_valid</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, InvalidGetRange\>

Check if the range is valid.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.as_range" class="fn">as_range</a>(&self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, InvalidGetRange\>

Convert to a [`Range`](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range") if [valid](https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.is_valid "method object_store::GetRange::is_valid").

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#impl-Clone-for-GetRange" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#impl-Debug-for-GetRange" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#impl-Display-for-GetRange" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#impl-From%3CT%3E-for-GetRange" class="anchor">§</a>

### impl\<T: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<T\> for <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: T) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#impl-PartialEq-for-GetRange" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#impl-Eq-for-GetRange" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#impl-StructuralPartialEq-for-GetRange" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html" class="enum" title="enum object_store::GetRange">GetRange</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.GetRange.html#blanket-implementations" class="anchor">§</a>
