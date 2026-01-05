# Struct BytesRange Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/http_util/bytes_range.rs.html#42-47" class="src">Source</a>

``` rust
pub struct BytesRange(/* private fields */);
```

Expand description

BytesRange(offset, size) carries a range of content.

BytesRange implements `ToString` which can be used as `Range` HTTP header directly.

`<unit>` should always be `bytes`.

``` text
Range: bytes=<range-start>-
Range: bytes=<range-start>-<range-end>
```

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#notes" class="doc-anchor">Â§</a>Notes

We donâ€™t support tailing read like `Range: bytes=-<range-end>`

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-BytesRange" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.new" class="fn">new</a>(offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> Self

Create a new `BytesRange`

It better to use `BytesRange::from(1024..2048)` to construct.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#note" class="doc-anchor">Â§</a>Note

The behavior for `None` and `Some` of `size` is different.

- size=None =\> `bytes=<offset>-`, read from `<offset>` until the end
- size=Some(1024) =\> `bytes=<offset>-<offset + 1024>`, read 1024 bytes starting from the `<offset>`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Get offset of BytesRange.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Get size of BytesRange.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.advance" class="fn">advance</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

Advance the range by `n` bytes.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#panics" class="doc-anchor">Â§</a>Panics

Panic if input `n` is larger than the size of the range.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.is_full" class="fn">is_full</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this range is full of this content.

If this range is full, we donâ€™t need to specify it in http request.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.to_header" class="fn">to_header</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Convert bytes range into Range header.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.to_range" class="fn">to_range</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Convert bytes range into rust range.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-Clone-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-Debug-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-Default-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-Display-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-From%3CT%3E-for-BytesRange" class="anchor">Â§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<T\> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(range: T) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-FromStr-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#associatedtype.Err" class="anchor">Â§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.from_str" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-PartialEq-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-Copy-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-Eq-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#impl-StructuralPartialEq-for-BytesRange" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html" class="struct" title="struct opendal::raw::BytesRange">BytesRange</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.BytesRange.html#blanket-implementations" class="anchor">Â§</a>
