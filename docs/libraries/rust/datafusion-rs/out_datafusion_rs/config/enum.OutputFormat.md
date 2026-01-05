# Enum OutputFormat Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#2649" class="src">Source</a>

``` rust
pub enum OutputFormat {
    CSV(CsvOptions),
    JSON(JsonOptions),
    PARQUET(TableParquetOptions),
    AVRO,
    ARROW,
}
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#variant.CSV" class="anchor">§</a>

### CSV(<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#variant.JSON" class="anchor">§</a>

### JSON(<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#variant.PARQUET" class="anchor">§</a>

### PARQUET(<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#variant.AVRO" class="anchor">§</a>

### AVRO

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#variant.ARROW" class="anchor">§</a>

### ARROW

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#impl-Clone-for-OutputFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html" class="enum" title="enum datafusion::config::OutputFormat">OutputFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html" class="enum" title="enum datafusion::config::OutputFormat">OutputFormat</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#impl-Debug-for-OutputFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html" class="enum" title="enum datafusion::config::OutputFormat">OutputFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#impl-Display-for-OutputFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html" class="enum" title="enum datafusion::config::OutputFormat">OutputFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#impl-PartialEq-for-OutputFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html" class="enum" title="enum datafusion::config::OutputFormat">OutputFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html" class="enum" title="enum datafusion::config::OutputFormat">OutputFormat</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#impl-StructuralPartialEq-for-OutputFormat" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html" class="enum" title="enum datafusion::config::OutputFormat">OutputFormat</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html#blanket-implementations" class="anchor">§</a>
