# Struct Type Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#780" class="src">Source</a>

``` rust
#[repr(transparent)]pub struct Type(pub u8);
```

Expand description

------------------------------------------------------------------------

Top-level Type value, enabling extensible type-specific metadata. We can add new logical types to Type without breaking backwards compatibility

## Tuple Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#structfield.0" class="anchor field">§</a>`0: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Type" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.NONE" class="constant">NONE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Null" class="constant">Null</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Int" class="constant">Int</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.FloatingPoint" class="constant">FloatingPoint</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Binary" class="constant">Binary</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Utf8" class="constant">Utf8</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Bool" class="constant">Bool</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Decimal" class="constant">Decimal</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Date" class="constant">Date</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Time" class="constant">Time</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Timestamp" class="constant">Timestamp</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Interval" class="constant">Interval</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.List" class="constant">List</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Struct_" class="constant">Struct_</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Union" class="constant">Union</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.FixedSizeBinary" class="constant">FixedSizeBinary</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.FixedSizeList" class="constant">FixedSizeList</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Map" class="constant">Map</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Duration" class="constant">Duration</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.LargeBinary" class="constant">LargeBinary</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.LargeUtf8" class="constant">LargeUtf8</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.LargeList" class="constant">LargeList</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.RunEndEncoded" class="constant">RunEndEncoded</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.BinaryView" class="constant">BinaryView</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.Utf8View" class="constant">Utf8View</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.ListView" class="constant">ListView</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.LargeListView" class="constant">LargeListView</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.ENUM_MIN" class="constant">ENUM_MIN</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 0u8

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.ENUM_MAX" class="constant">ENUM_MAX</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a> = 26u8

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedconstant.ENUM_VALUES" class="constant">ENUM_VALUES</a>: &'static \[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>\]

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.variant_name" class="fn">variant_name</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the variant’s name or “” if unknown.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Clone-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Debug-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Default-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-EndianScalar-for-Type" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/endian_scalar/trait.EndianScalar.html" class="trait" title="trait flatbuffers::endian_scalar::EndianScalar">EndianScalar</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedtype.Scalar" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/endian_scalar/trait.EndianScalar.html#associatedtype.Scalar" class="associatedtype">Scalar</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.to_little_endian" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/endian_scalar/trait.EndianScalar.html#tymethod.to_little_endian" class="fn">to_little_endian</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.from_little_endian" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/endian_scalar/trait.EndianScalar.html#tymethod.from_little_endian" class="fn">from_little_endian</a>(v: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Follow%3C&#39;a%3E-for-Type" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedtype.Inner" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.follow" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>(buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Hash-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Ord-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-PartialEq-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-PartialOrd-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Push-for-Type" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html" class="trait" title="trait flatbuffers::push::Push">Push</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.push" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#tymethod.push" class="fn">push</a>(&self, dst: &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], \_written_len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#tymethod.push)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.size" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#method.size" class="fn">size</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.alignment" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/trait.Push.html#method.alignment" class="fn">alignment</a>() -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/push/struct.PushAlignment.html" class="struct" title="struct flatbuffers::push::PushAlignment">PushAlignment</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Verifiable-for-Type" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html" class="trait" title="trait flatbuffers::verifier::Verifiable">Verifiable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#method.run_verifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html#tymethod.run_verifier" class="fn">run_verifier</a>( v: &mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/struct.Verifier.html" class="struct" title="struct flatbuffers::verifier::Verifier">Verifier</a>\<'\_, '\_\>, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/enum.InvalidFlatbuffer.html" class="enum" title="enum flatbuffers::verifier::InvalidFlatbuffer">InvalidFlatbuffer</a>\>

Runs the verifier for this type, assuming its at position `pos` in the verifier’s buffer. Should not need to be called directly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Copy-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-Eq-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-SimpleToVerifyInSlice-for-Type" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.SimpleToVerifyInSlice.html" class="trait" title="trait flatbuffers::verifier::SimpleToVerifyInSlice">SimpleToVerifyInSlice</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#impl-StructuralPartialEq-for-Type" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html#blanket-implementations" class="anchor">§</a>
