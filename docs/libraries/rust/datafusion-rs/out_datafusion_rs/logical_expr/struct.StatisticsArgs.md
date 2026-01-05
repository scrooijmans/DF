# Struct StatisticsArgs Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/udaf.rs.html#106" class="src">Source</a>

``` rust
pub struct StatisticsArgs<'a> {
    pub statistics: &'a Statistics,
    pub return_type: &'a DataType,
    pub is_distinct: bool,
    pub exprs: &'a [Arc<dyn PhysicalExpr>],
}
```

Expand description

Arguments passed to [`AggregateUDFImpl::value_from_stats`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.value_from_stats "method datafusion::logical_expr::AggregateUDFImpl::value_from_stats")

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#structfield.statistics" class="anchor field">§</a>`statistics: &'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics"><code>Statistics</code></a>

The statistics of the aggregate input

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#structfield.return_type" class="anchor field">§</a>`return_type: &'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType"><code>DataType</code></a>

The resolved return type of the aggregate function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#structfield.is_distinct" class="anchor field">§</a>`is_distinct: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether the aggregate function is distinct.

``` sql
SELECT COUNT(DISTINCT column1) FROM t;
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#structfield.exprs" class="anchor field">§</a>`exprs: &'a [`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>]`

The physical expression of arguments the aggregate function takes.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#impl-Debug-for-StatisticsArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html" class="struct" title="struct datafusion::logical_expr::StatisticsArgs">StatisticsArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.StatisticsArgs.html#blanket-implementations" class="anchor">§</a>
