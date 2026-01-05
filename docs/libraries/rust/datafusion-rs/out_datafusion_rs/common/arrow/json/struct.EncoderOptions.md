# Struct EncoderOptions Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/writer/encoder.rs.html#33" class="src">Source</a>

``` rust
pub struct EncoderOptions { /* private fields */ }
```

Expand description

Configuration options for the JSON encoder.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#impl-EncoderOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.with_explicit_nulls" class="fn">with_explicit_nulls</a>(self, explicit_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

Set whether to include nulls in the output or elide them.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.with_struct_mode" class="fn">with_struct_mode</a>(self, struct_mode: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/enum.StructMode.html" class="enum" title="enum datafusion::common::arrow::json::StructMode">StructMode</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

Set whether to encode structs as JSON objects or JSON arrays of their values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.with_encoder_factory" class="fn">with_encoder_factory</a>( self, encoder_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/trait.EncoderFactory.html" class="trait" title="trait datafusion::common::arrow::json::EncoderFactory">EncoderFactory</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

Set an optional hook for customizing encoding behavior.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.explicit_nulls" class="fn">explicit_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Get whether to include nulls in the output or elide them.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.struct_mode" class="fn">struct_mode</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/enum.StructMode.html" class="enum" title="enum datafusion::common::arrow::json::StructMode">StructMode</a>

Get whether to encode structs as JSON objects or JSON arrays of their values.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.encoder_factory" class="fn">encoder_factory</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/trait.EncoderFactory.html" class="trait" title="trait datafusion::common::arrow::json::EncoderFactory">EncoderFactory</a>\>\>

Get the optional hook for customizing encoding behavior.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#impl-Clone-for-EncoderOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#impl-Debug-for-EncoderOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#impl-Default-for-EncoderOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html#blanket-implementations" class="anchor">§</a>
