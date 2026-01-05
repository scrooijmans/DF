# Struct AnonymousScanArgs Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/plans/anonymous_scan.rs.html#8" class="src">Source</a>

``` rust
pub struct AnonymousScanArgs {
    pub n_rows: Option<usize>,
    pub with_columns: Option<Arc<[PlSmallStr]>>,
    pub schema: Arc<Schema<DataType>>,
    pub output_schema: Option<Arc<Schema<DataType>>>,
    pub predicate: Option<Expr>,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html#structfield.n_rows" class="anchor field">§</a>`n_rows: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html#structfield.with_columns" class="anchor field">§</a>`with_columns: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<[`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`]>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html#structfield.output_schema" class="anchor field">§</a>`output_schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html#structfield.predicate" class="anchor field">§</a>`predicate: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr"><code>Expr</code></a>`>`

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.AnonymousScanArgs.html#blanket-implementations" class="anchor">§</a>
