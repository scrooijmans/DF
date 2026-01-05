# Struct AnalysisContext Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/analysis.rs.html#38" class="src">Source</a>

``` rust
pub struct AnalysisContext {
    pub boundaries: Vec<ExprBoundaries>,
    pub selectivity: Option<f64>,
}
```

Expand description

The shared context used during the analysis of an expression. Includes the boundaries for all known columns.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#structfield.boundaries" class="anchor field">§</a>`boundaries: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries"><code>ExprBoundaries</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#structfield.selectivity" class="anchor field">§</a>`selectivity: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive"><code>f64</code></a>`>`

The estimated percentage of rows that this expression would select, if it were to be used as a boolean predicate on a filter. The value will be between 0.0 (selects nothing) and 1.0 (selects everything).

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#impl-AnalysisContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#method.new" class="fn">new</a>(boundaries: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ExprBoundaries.html" class="struct" title="struct datafusion::physical_expr::ExprBoundaries">ExprBoundaries</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#method.with_selectivity" class="fn">with_selectivity</a>(self, selectivity: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#method.try_from_statistics" class="fn">try_from_statistics</a>( input_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, statistics: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::ColumnStatistics">ColumnStatistics</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new analysis context from column statistics.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#impl-Clone-for-AnalysisContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#impl-Debug-for-AnalysisContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#impl-PartialEq-for-AnalysisContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#impl-StructuralPartialEq-for-AnalysisContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html" class="struct" title="struct datafusion::physical_expr::AnalysisContext">AnalysisContext</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.AnalysisContext.html#blanket-implementations" class="anchor">§</a>
