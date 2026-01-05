# Struct WriterBuilder Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/writer/mod.rs.html#200" class="src">Source</a>

``` rust
pub struct WriterBuilder(/* private fields */);
```

Expand description

JSON writer builder.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#impl-WriterBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

Create a new builder for configuring JSON writing options.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#example" class="doc-anchor">§</a>Example

``` rust

fn example() -> Writer<File, LineDelimited> {
    let file = File::create("target/out.json").unwrap();

    // create a builder that keeps keys with null values
    let builder = WriterBuilder::new().with_explicit_nulls(true);
    let writer = builder.build::<_, LineDelimited>(file);

    writer
}
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.explicit_nulls" class="fn">explicit_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if this writer is configured to keep keys with null values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.with_explicit_nulls" class="fn">with_explicit_nulls</a>(self, explicit_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

Set whether to keep keys with null values, or to omit writing them.

For example, with [`LineDelimited`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.LineDelimited.html "struct datafusion::common::arrow::json::writer::LineDelimited") format:

Skip nulls (set to `false`):

``` json
{"foo":1}
{"foo":1,"bar":2}
{}
```

Keep nulls (set to `true`):

``` json
{"foo":1,"bar":null}
{"foo":1,"bar":2}
{"foo":null,"bar":null}
```

Default is to skip nulls (set to `false`). If `struct_mode == ListOnly`, nulls will be written explicitly regardless of this setting.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.struct_mode" class="fn">struct_mode</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/enum.StructMode.html" class="enum" title="enum datafusion::common::arrow::json::StructMode">StructMode</a>

Returns if this writer is configured to write structs as JSON Objects or Arrays.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.with_struct_mode" class="fn">with_struct_mode</a>(self, struct_mode: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/enum.StructMode.html" class="enum" title="enum datafusion::common::arrow::json::StructMode">StructMode</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

Set the [`StructMode`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/enum.StructMode.html "enum datafusion::common::arrow::json::StructMode") for the writer, which determines whether structs are encoded to JSON as objects or lists. For more details refer to the enum documentation. Default is to use `ObjectOnly`. If this is set to `ListOnly`, nulls will be written explicitly regardless of the `explicit_nulls` setting.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.with_encoder_factory" class="fn">with_encoder_factory</a>( self, factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/trait.EncoderFactory.html" class="trait" title="trait datafusion::common::arrow::json::EncoderFactory">EncoderFactory</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

Set an encoder factory to use when creating encoders for writing JSON.

This can be used to override how some types are encoded or to provide a fallback for types that are not supported by the default encoder.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.build" class="fn">build</a>\<W, F\>(self, writer: W) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::json::Writer">Writer</a>\<W, F\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>, F: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/trait.JsonFormat.html" class="trait" title="trait datafusion::common::arrow::json::writer::JsonFormat">JsonFormat</a>,

Create a new `Writer` with specified `JsonFormat` and builder options.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#impl-Clone-for-WriterBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#impl-Debug-for-WriterBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#impl-Default-for-WriterBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/struct.WriterBuilder.html#blanket-implementations" class="anchor">§</a>
