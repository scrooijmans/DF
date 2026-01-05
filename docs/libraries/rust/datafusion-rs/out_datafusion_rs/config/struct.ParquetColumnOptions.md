# Struct ParquetColumnOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#2097-2138" class="src">Source</a>

``` rust
pub struct ParquetColumnOptions {
    pub bloom_filter_enabled: Option<bool>,
    pub encoding: Option<String>,
    pub dictionary_enabled: Option<bool>,
    pub compression: Option<String>,
    pub statistics_enabled: Option<String>,
    pub bloom_filter_fpp: Option<f64>,
    pub bloom_filter_ndv: Option<u64>,
}
```

Expand description

Options controlling parquet format for individual columns.

See [`ParquetOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html "struct datafusion::config::ParquetOptions") for more details

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#structfield.bloom_filter_enabled" class="anchor field">§</a>`bloom_filter_enabled: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Sets if bloom filter is enabled for the column path.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#structfield.encoding" class="anchor field">§</a>`encoding: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets encoding for the column path. Valid values are: plain, plain_dictionary, rle, bit_packed, delta_binary_packed, delta_length_byte_array, delta_byte_array, rle_dictionary, and byte_stream_split. These values are not case-sensitive. If NULL, uses default parquet options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#structfield.dictionary_enabled" class="anchor field">§</a>`dictionary_enabled: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Sets if dictionary encoding is enabled for the column path. If NULL, uses default parquet options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#structfield.compression" class="anchor field">§</a>`compression: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets default parquet compression codec for the column path. Valid values are: uncompressed, snappy, gzip(level), lzo, brotli(level), lz4, zstd(level), and lz4_raw. These values are not case-sensitive. If NULL, uses default parquet options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#structfield.statistics_enabled" class="anchor field">§</a>`statistics_enabled: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Sets if statistics are enabled for the column Valid values are: “none”, “chunk”, and “page” These values are not case sensitive. If NULL, uses default parquet options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#structfield.bloom_filter_fpp" class="anchor field">§</a>`bloom_filter_fpp: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive"><code>f64</code></a>`>`

Sets bloom filter false positive probability for the column path. If NULL, uses default parquet options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#structfield.bloom_filter_ndv" class="anchor field">§</a>`bloom_filter_ndv: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

Sets bloom filter number of distinct values. If NULL, uses default parquet options

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#impl-Clone-for-ParquetColumnOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#impl-ConfigField-for-ParquetColumnOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#method.set" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.set" class="fn">set</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, key_prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_description: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#impl-Debug-for-ParquetColumnOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#impl-Default-for-ParquetColumnOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#impl-PartialEq-for-ParquetColumnOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#impl-StructuralPartialEq-for-ParquetColumnOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html#blanket-implementations" class="anchor">§</a>
