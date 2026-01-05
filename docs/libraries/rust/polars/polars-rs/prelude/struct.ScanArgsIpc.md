# Struct ScanArgsIpc Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/scan/ipc.rs.html#11" class="src">Source</a>

``` rust
pub struct ScanArgsIpc {
    pub n_rows: Option<usize>,
    pub cache: bool,
    pub rechunk: bool,
    pub row_index: Option<RowIndex>,
    pub cloud_options: Option<CloudOptions>,
    pub hive_options: HiveOptions,
    pub include_file_paths: Option<PlSmallStr>,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#structfield.n_rows" class="anchor field">§</a>`n_rows: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#structfield.cache" class="anchor field">§</a>`cache: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#structfield.rechunk" class="anchor field">§</a>`rechunk: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#structfield.row_index" class="anchor field">§</a>`row_index: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex"><code>RowIndex</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#structfield.cloud_options" class="anchor field">§</a>`cloud_options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions"><code>CloudOptions</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#structfield.hive_options" class="anchor field">§</a>`hive_options: `<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.HiveOptions.html" class="struct" title="struct polars_io::options::HiveOptions"><code>HiveOptions</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#structfield.include_file_paths" class="anchor field">§</a>`include_file_paths: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`>`

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#impl-Clone-for-ScanArgsIpc" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html" class="struct" title="struct polars::prelude::ScanArgsIpc">ScanArgsIpc</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html" class="struct" title="struct polars::prelude::ScanArgsIpc">ScanArgsIpc</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#impl-Default-for-ScanArgsIpc" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html" class="struct" title="struct polars::prelude::ScanArgsIpc">ScanArgsIpc</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html" class="struct" title="struct polars::prelude::ScanArgsIpc">ScanArgsIpc</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html#blanket-implementations" class="anchor">§</a>
