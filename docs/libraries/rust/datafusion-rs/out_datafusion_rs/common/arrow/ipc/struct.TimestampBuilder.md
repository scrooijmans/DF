# Struct TimestampBuilder Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/gen/Schema.rs.html#3676" class="src">Source</a>

``` rust
pub struct TimestampBuilder<'a, 'b, A>where
    'a: 'b,
    A: Allocator + 'a,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html#impl-TimestampBuilder%3C&#39;a,+&#39;b,+A%3E" class="anchor">§</a>

### impl\<'a, 'b, A\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::TimestampBuilder">TimestampBuilder</a>\<'a, 'b, A\>

where 'a: 'b, A: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/trait.Allocator.html" class="trait" title="trait flatbuffers::builder::Allocator">Allocator</a> + 'a,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html#method.add_unit" class="fn">add_unit</a>(&mut self, unit: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimeUnit.html" class="struct" title="struct datafusion::common::arrow::ipc::TimeUnit">TimeUnit</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html#method.add_timezone" class="fn">add_timezone</a>(&mut self, timezone: <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<&'b <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html#method.new" class="fn">new</a>( \_fbb: &'b mut <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/builder/struct.FlatBufferBuilder.html" class="struct" title="struct flatbuffers::builder::FlatBufferBuilder">FlatBufferBuilder</a>\<'a, A\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::TimestampBuilder">TimestampBuilder</a>\<'a, 'b, A\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/flatbuffers/25.2.10/x86_64-unknown-linux-gnu/flatbuffers/primitives/struct.WIPOffset.html" class="struct" title="struct flatbuffers::primitives::WIPOffset">WIPOffset</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>\<'a\>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html#blanket-implementations" class="anchor">§</a>
