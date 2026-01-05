# Struct ExprBoundaries Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/analysis.rs.html#82" class="src">Source</a>

``` rust
pub struct ExprBoundaries {
    pub column: Column,
    pub interval: Option<Interval>,
    pub distinct_count: Precision<usize>,
}
```

Expand description

Represents the boundaries (e.g. min and max values) of a particular column

This is used range analysis of expressions, to determine if the expression limits the value of particular columns (e.g. analyzing an expression such as `time < 50` would result in a boundary interval for `time` having a max value of `50`).

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#structfield.column" class="anchor field">§</a>`column: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.Column.html" class="struct" title="struct datafusion::physical_expr::expressions::Column"><code>Column</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#structfield.interval" class="anchor field">§</a>`interval: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval"><code>Interval</code></a>`>`

Minimum and maximum values this expression can have. A `None` value indicates that evaluating the given column results in an empty set. For example, if the column `a` has values in the range \[10, 20\], and there is a filter asserting that `a > 50`, then the resulting interval range of `a` will be `None`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#structfield.distinct_count" class="anchor field">§</a>`distinct_count: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision"><code>Precision</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`

Maximum number of distinct values this expression can produce, if known.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#impl-ExprBoundaries" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#method.try_from_column" class="fn">try_from_column</a>( schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, col_stats: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>, col_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new `ExprBoundaries` object from column level statistics.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#method.try_new_unbounded" class="fn">try_new_unbounded</a>( schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create `ExprBoundaries` that represent no known bounds for all the columns in `schema`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#impl-Clone-for-ExprBoundaries" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#impl-Debug-for-ExprBoundaries" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#impl-PartialEq-for-ExprBoundaries" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#impl-StructuralPartialEq-for-ExprBoundaries" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html#blanket-implementations" class="anchor">§</a>
