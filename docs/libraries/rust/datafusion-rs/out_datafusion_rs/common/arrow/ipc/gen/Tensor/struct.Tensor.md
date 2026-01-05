# Struct Tensor Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Tensor.rs.html#158" class="src">Source</a>

``` rust
pub struct Tensor<'a> {
    pub _tab: Table<'a>,
}
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#structfield._tab" class="anchor field">§</a>`_tab: `<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table"><code>Table</code></a>`<'a>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#impl-Tensor%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#associatedconstant.VT_TYPE_TYPE" class="constant">VT_TYPE_TYPE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 4u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#associatedconstant.VT_TYPE_" class="constant">VT_TYPE_</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 6u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#associatedconstant.VT_SHAPE" class="constant">VT_SHAPE</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 8u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#associatedconstant.VT_STRIDES" class="constant">VT_STRIDES</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 10u16

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#associatedconstant.VT_DATA" class="constant">VT_DATA</a>: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a> = 12u16

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.init_from_table" class="fn">init_from_table</a>(table: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table">Table</a>\<'a\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.create" class="fn">create</a>\<'bldr, 'args, 'mut_bldr, A\>( \_fbb: &'mut_bldr mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'bldr, A\>, args: &'args <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorArgs">TensorArgs</a>\<'args\>, ) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'bldr\>\>

where 'bldr: 'args, 'args: 'mut_bldr, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'bldr,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_type" class="fn">type_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_" class="fn">type_</a>(&self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/table/struct.Table.html" class="struct" title="struct flatbuffers::table::Table">Table</a>\<'a\>

The type of data contained in a value cell. Currently only fixed-width value types are supported, no strings or nested types

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.shape" class="fn">shape</a>(&self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'a, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.ForwardsUOffset.html" class="struct" title="struct flatbuffers::primitives::ForwardsUOffset">ForwardsUOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorDim.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorDim">TensorDim</a>\<'a\>\>\>

The dimensions of the tensor, optionally named

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.strides" class="fn">strides</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/vector/struct.Vector.html" class="struct" title="struct flatbuffers::vector::Vector">Vector</a>\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>

Non-negative byte offsets to advance one value cell along each dimension If omitted, default to row-major order (C-like).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.data" class="fn">data</a>(&self) -\> &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

The location and size of the tensor’s data

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_null" class="fn">type_as_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Null.html" class="struct" title="struct datafusion::common::arrow::ipc::Null">Null</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_int" class="fn">type_as_int</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_floating_point" class="fn">type_as_floating_point</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FloatingPoint.html" class="struct" title="struct datafusion::common::arrow::ipc::FloatingPoint">FloatingPoint</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_binary" class="fn">type_as_binary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Binary.html" class="struct" title="struct datafusion::common::arrow::ipc::Binary">Binary</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_utf_8" class="fn">type_as_utf_8</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Utf8.html" class="struct" title="struct datafusion::common::arrow::ipc::Utf8">Utf8</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_bool" class="fn">type_as_bool</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Bool.html" class="struct" title="struct datafusion::common::arrow::ipc::Bool">Bool</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_decimal" class="fn">type_as_decimal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Decimal.html" class="struct" title="struct datafusion::common::arrow::ipc::Decimal">Decimal</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_date" class="fn">type_as_date</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Date.html" class="struct" title="struct datafusion::common::arrow::ipc::Date">Date</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_time" class="fn">type_as_time</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Time.html" class="struct" title="struct datafusion::common::arrow::ipc::Time">Time</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_timestamp" class="fn">type_as_timestamp</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_interval" class="fn">type_as_interval</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Interval.html" class="struct" title="struct datafusion::common::arrow::ipc::Interval">Interval</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_list" class="fn">type_as_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.List.html" class="struct" title="struct datafusion::common::arrow::ipc::List">List</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_struct_" class="fn">type_as_struct_</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Struct_.html" class="struct" title="struct datafusion::common::arrow::ipc::Struct_">Struct_</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_union" class="fn">type_as_union</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Union.html" class="struct" title="struct datafusion::common::arrow::ipc::Union">Union</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_fixed_size_binary" class="fn">type_as_fixed_size_binary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FixedSizeBinary.html" class="struct" title="struct datafusion::common::arrow::ipc::FixedSizeBinary">FixedSizeBinary</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_fixed_size_list" class="fn">type_as_fixed_size_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FixedSizeList.html" class="struct" title="struct datafusion::common::arrow::ipc::FixedSizeList">FixedSizeList</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_map" class="fn">type_as_map</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_duration" class="fn">type_as_duration</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Duration.html" class="struct" title="struct datafusion::common::arrow::ipc::Duration">Duration</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_large_binary" class="fn">type_as_large_binary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeBinary.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeBinary">LargeBinary</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_large_utf_8" class="fn">type_as_large_utf_8</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeUtf8.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeUtf8">LargeUtf8</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_large_list" class="fn">type_as_large_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeList.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeList">LargeList</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_run_end_encoded" class="fn">type_as_run_end_encoded</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.RunEndEncoded.html" class="struct" title="struct datafusion::common::arrow::ipc::RunEndEncoded">RunEndEncoded</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_binary_view" class="fn">type_as_binary_view</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BinaryView.html" class="struct" title="struct datafusion::common::arrow::ipc::BinaryView">BinaryView</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_utf_8_view" class="fn">type_as_utf_8_view</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Utf8View.html" class="struct" title="struct datafusion::common::arrow::ipc::Utf8View">Utf8View</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_list_view" class="fn">type_as_list_view</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.ListView.html" class="struct" title="struct datafusion::common::arrow::ipc::ListView">ListView</a>\<'a\>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.type_as_large_list_view" class="fn">type_as_large_list_view</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeListView.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeListView">LargeListView</a>\<'a\>\>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#impl-Clone-for-Tensor%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#impl-Debug-for-Tensor%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#impl-Follow%3C&#39;a%3E-for-Tensor%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#associatedtype.Inner" class="anchor">§</a>

#### type <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype">Inner</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.follow" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow" class="fn">follow</a>(buf: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], loc: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\> as <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html" class="trait" title="trait flatbuffers::follow::Follow">Follow</a>\<'a\>\>::<a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#associatedtype.Inner" class="associatedtype" title="type flatbuffers::follow::Follow::Inner">Inner</a>

Safety [Read more](https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/follow/trait.Follow.html#tymethod.follow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#impl-PartialEq-for-Tensor%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#impl-Verifiable-for-Tensor%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html" class="trait" title="trait flatbuffers::verifier::Verifiable">Verifiable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#method.run_verifier" class="anchor">§</a>

#### fn <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/trait.Verifiable.html#tymethod.run_verifier" class="fn">run_verifier</a>( v: &mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/struct.Verifier.html" class="struct" title="struct flatbuffers::verifier::Verifier">Verifier</a>\<'\_, '\_\>, pos: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/verifier/enum.InvalidFlatbuffer.html" class="enum" title="enum flatbuffers::verifier::InvalidFlatbuffer">InvalidFlatbuffer</a>\>

Runs the verifier for this type, assuming its at position `pos` in the verifier’s buffer. Should not need to be called directly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#impl-Copy-for-Tensor%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#impl-StructuralPartialEq-for-Tensor%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>\<'a\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/Tensor/struct.Tensor.html#blanket-implementations" class="anchor">§</a>
