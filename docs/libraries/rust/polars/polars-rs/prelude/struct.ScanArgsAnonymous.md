# Struct ScanArgsAnonymous Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/scan/anonymous_scan.rs.html#8" class="src">Source</a>

``` rust
pub struct ScanArgsAnonymous {
    pub infer_schema_length: Option<usize>,
    pub schema: Option<Arc<Schema<DataType>>>,
    pub skip_rows: Option<usize>,
    pub n_rows: Option<usize>,
    pub row_index: Option<RowIndex>,
    pub name: &'static str,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#structfield.infer_schema_length" class="anchor field">§</a>`infer_schema_length: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#structfield.skip_rows" class="anchor field">§</a>`skip_rows: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#structfield.n_rows" class="anchor field">§</a>`n_rows: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#structfield.row_index" class="anchor field">§</a>`row_index: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex"><code>RowIndex</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#structfield.name" class="anchor field">§</a>`name: &'static `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#impl-Clone-for-ScanArgsAnonymous" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html" class="struct" title="struct polars::prelude::ScanArgsAnonymous">ScanArgsAnonymous</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html" class="struct" title="struct polars::prelude::ScanArgsAnonymous">ScanArgsAnonymous</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#impl-Default-for-ScanArgsAnonymous" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html" class="struct" title="struct polars::prelude::ScanArgsAnonymous">ScanArgsAnonymous</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html" class="struct" title="struct polars::prelude::ScanArgsAnonymous">ScanArgsAnonymous</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html#blanket-implementations" class="anchor">§</a>
