# Enum ConfigFileType Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1604" class="src">Source</a>

``` rust
pub enum ConfigFileType {
    CSV,
    PARQUET,
    JSON,
}
```

Expand description

These file types have special built in behavior for configuration. Use TableOptions::Extensions for configuring other file types.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#variant.CSV" class="anchor">§</a>

### CSV

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#variant.PARQUET" class="anchor">§</a>

### PARQUET

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#variant.JSON" class="anchor">§</a>

### JSON

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#impl-Clone-for-ConfigFileType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html" class="enum" title="enum datafusion::config::ConfigFileType">ConfigFileType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html" class="enum" title="enum datafusion::config::ConfigFileType">ConfigFileType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#impl-Debug-for-ConfigFileType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html" class="enum" title="enum datafusion::config::ConfigFileType">ConfigFileType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html#blanket-implementations" class="anchor">§</a>
