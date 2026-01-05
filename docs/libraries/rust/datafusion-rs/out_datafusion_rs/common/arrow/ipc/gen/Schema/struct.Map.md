# Struct Map Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#1855" class="src">Source</a>

``` rust
pub struct Map<'a> {
    pub _tab: Table<'a>,
}
```

Expand description

A Map is a logical nested type that is represented as

List\<entries: Struct\<key: K, value: V\>\>

In this layout, the keys and values are each respectively contiguous. We do not constrain the key and value types, so the application is responsible for ensuring that the keys are hashable and unique. Whether the keys are sorted may be set in the metadata for this field.

In a field with Map type, the field has a child Struct field, which then has two children: key type and the second the value type. The names of the child fields may be respectively “entries”, “key”, and “value”, but this is not enforced.

Map

``` text
  - child[0] entries: Struct
    - child[0] key: K
    - child[1] value: V
```

Neither the “entries” field nor the “key” field may be nullable.

The metadata is structured so that Arrow systems without special handling for Map can make Map an alias for List. The “layout” attribute for the Map field must have the same contents as a List.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#structfield._tab" class="anchor field">§</a>`_tab: `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table"><code>Table</code></a>`<'a>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#impl-Map%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#associatedconstant.VT_KEYSSORTED" class="constant">VT_KEYSSORTED</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 4u16

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.init_from_table" class="fn">init_from_table</a>(table: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table">Table</a>\<'a\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.create" class="fn">create</a>\<'bldr, 'args, 'mut_bldr, A\>( \_fbb: &'mut_bldr mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'bldr, A\>, args: &'args <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.MapArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::MapArgs">MapArgs</a>, ) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'bldr\>\>

where 'bldr: 'args, 'args: 'mut_bldr, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'bldr,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.keysSorted" class="fn">keysSorted</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Set to true if the keys within each value are sorted

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#impl-Clone-for-Map%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#impl-Debug-for-Map%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#impl-Follow%3C&#39;a%3E-for-Map%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#associatedtype.Inner" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.follow" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>(buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#impl-PartialEq-for-Map%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#impl-Verifiable-for-Map%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html" class="trait" title="trait flatbuffers::verifier::Verifiable">Verifiable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#method.run_verifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html#tymethod.run_verifier" class="fn">run_verifier</a>( v: &mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/struct.Verifier.html" class="struct" title="struct flatbuffers::verifier::Verifier">Verifier</a>\<'\_, '\_\>, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/enum.InvalidFlatbuffer.html" class="enum" title="enum flatbuffers::verifier::InvalidFlatbuffer">InvalidFlatbuffer</a>\>

Runs the verifier for this type, assuming its at position `pos` in the verifier’s buffer. Should not need to be called directly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#impl-Copy-for-Map%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#impl-StructuralPartialEq-for-Map%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Schema/struct.Map.html#blanket-implementations" class="anchor">§</a>
