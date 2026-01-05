# Struct BytesContentRange Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/bytes_content_range.rs.html#50-57" class="src">Source</a>

``` rust
pub struct BytesContentRange(/* private fields */);
```

Expand description

BytesContentRange is the content range of bytes.

`<unit>` should always be `bytes`.

``` text
Content-Range: bytes <range-start>-<range-end>/<size>
Content-Range: bytes <range-start>-<range-end>/*
Content-Range: bytes */<size>
```

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#notes" class="doc-anchor">Â§</a>Notes

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#usage-of-the-default" class="doc-anchor">Â§</a>Usage of the default.

`BytesContentRange::default` is not a valid content range. Please make sure their comes up with `with_range` or `with_size` call.

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#allow-clippylen_without_is_empty" class="doc-anchor">Â§</a>Allow clippy::len_without_is_empty

BytesContentRange implements `len()` but not `is_empty()` because itâ€™s useless.

- When BytesContentRangeâ€™s range is known, it must be non-empty.
- When BytesContentRangeâ€™s range is no known, we donâ€™t know whether itâ€™s empty.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.with_range" class="fn">with_range</a>(self, start: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, end: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> Self

Update BytesContentRange with range.

The range is inclusive: `[start..=end]` as described in `content-range`.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.with_size" class="fn">with_size</a>(self, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> Self

Update BytesContentRange with size.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Get the length that specified by this BytesContentRange, return `None` if range is not known.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Get the size of this BytesContentRange, return `None` if size is not known.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.range" class="fn">range</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\>

Get the range inclusive of this BytesContentRange, return `None` if range is not known.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.range_inclusive" class="fn">range_inclusive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html" class="struct" title="struct core::ops::range::RangeInclusive">RangeInclusive</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\>

Get the range inclusive of this BytesContentRange, return `None` if range is not known.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.to_header" class="fn">to_header</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Convert bytes content range into Content-Range header.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-Clone-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-Debug-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-Default-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-Deserialize%3C&#39;de%3E-for-BytesContentRange" class="anchor">Â§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.deserialize" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-Display-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-FromStr-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#associatedtype.Err" class="anchor">Â§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.from_str" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-PartialEq-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-Serialize-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#method.serialize" class="anchor">Â§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-Copy-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-Eq-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#impl-StructuralPartialEq-for-BytesContentRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html" class="struct" title="struct opendal::raw::BytesContentRange">BytesContentRange</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesContentRange.html#blanket-implementations" class="anchor">Â§</a>
