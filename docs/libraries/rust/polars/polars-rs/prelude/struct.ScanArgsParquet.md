# Struct ScanArgsParquet Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/scan/parquet.rs.html#12" class="src">Source</a>

``` rust
pub struct ScanArgsParquet {Show 13 fields
    pub n_rows: Option<usize>,
    pub parallel: ParallelStrategy,
    pub row_index: Option<RowIndex>,
    pub cloud_options: Option<CloudOptions>,
    pub hive_options: HiveOptions,
    pub use_statistics: bool,
    pub schema: Option<Arc<Schema<DataType>>>,
    pub low_memory: bool,
    pub rechunk: bool,
    pub cache: bool,
    pub glob: bool,
    pub include_file_paths: Option<PlSmallStr>,
    pub allow_missing_columns: bool,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.n_rows" class="anchor field">§</a>`n_rows: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.parallel" class="anchor field">§</a>`parallel: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ParallelStrategy.html" class="enum" title="enum polars::prelude::ParallelStrategy"><code>ParallelStrategy</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.row_index" class="anchor field">§</a>`row_index: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex"><code>RowIndex</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.cloud_options" class="anchor field">§</a>`cloud_options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions"><code>CloudOptions</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.hive_options" class="anchor field">§</a>`hive_options: `<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.HiveOptions.html" class="struct" title="struct polars_io::options::HiveOptions"><code>HiveOptions</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.use_statistics" class="anchor field">§</a>`use_statistics: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.low_memory" class="anchor field">§</a>`low_memory: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.rechunk" class="anchor field">§</a>`rechunk: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.cache" class="anchor field">§</a>`cache: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.glob" class="anchor field">§</a>`glob: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Expand path given via globbing rules.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.include_file_paths" class="anchor field">§</a>`include_file_paths: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#structfield.allow_missing_columns" class="anchor field">§</a>`allow_missing_columns: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#impl-Clone-for-ScanArgsParquet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html" class="struct" title="struct polars::prelude::ScanArgsParquet">ScanArgsParquet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html" class="struct" title="struct polars::prelude::ScanArgsParquet">ScanArgsParquet</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#impl-Default-for-ScanArgsParquet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html" class="struct" title="struct polars::prelude::ScanArgsParquet">ScanArgsParquet</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html" class="struct" title="struct polars::prelude::ScanArgsParquet">ScanArgsParquet</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html#blanket-implementations" class="anchor">§</a>
