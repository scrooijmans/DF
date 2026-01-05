# Struct LazyFrame Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/frame/mod.rs.html#72" class="src">Source</a>

``` rust
pub struct LazyFrame {
    pub logical_plan: DslPlan,
    /* private fields */
}
```

Available on **crate feature `lazy`** only.

Expand description

Lazy abstraction over an eager `DataFrame`.

It really is an abstraction over a logical plan. The methods of this struct will incrementally modify a logical plan until output is requested (via [`collect`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.collect "method polars::prelude::LazyFrame::collect")).

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#structfield.logical_plan" class="anchor field">§</a>`logical_plan: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan"><code>DslPlan</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-LazyFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.to_dot" class="fn">to_dot</a>(&self, optimized: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a dot language representation of the LogicalPlan.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.to_dot_streaming_phys" class="fn">to_dot_streaming_phys</a>( &self, optimized: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a dot language representation of the streaming physical plan.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-LazyFrame-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.set_cached_arena" class="fn">set_cached_arena</a>(&self, lp_arena: <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/enum.IR.html" class="enum" title="enum polars_plan::plans::ir::IR">IR</a>\>, expr_arena: <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/enum.AExpr.html" class="enum" title="enum polars_plan::plans::aexpr::AExpr">AExpr</a>\>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.schema_with_arenas" class="fn">schema_with_arenas</a>( &mut self, lp_arena: &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/enum.IR.html" class="enum" title="enum polars_plan::plans::ir::IR">IR</a>\>, expr_arena: &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/enum.AExpr.html" class="enum" title="enum polars_plan::plans::aexpr::AExpr">AExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.collect_schema" class="fn">collect_schema</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a handle to the schema — a map from column names to data types — of the current `LazyFrame` computation.

Returns an `Err` if the logical plan has already encountered an error (i.e., if `self.collect()` would fail), `Ok` otherwise.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-LazyFrame-2" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.collect_concurrently" class="fn">collect_concurrently</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.InProcessQuery.html" class="struct" title="struct polars::prelude::InProcessQuery">InProcessQuery</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-LazyFrame-3" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.get_current_optimizations" class="fn">get_current_optimizations</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.OptFlags.html" class="struct" title="struct polars::prelude::OptFlags">OptFlags</a>

Get current optimizations.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_optimizations" class="fn">with_optimizations</a>(self, opt_state: <a href="https://docs.rs/polars/latest/polars/prelude/struct.OptFlags.html" class="struct" title="struct polars::prelude::OptFlags">OptFlags</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Set allowed optimizations.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.without_optimizations" class="fn">without_optimizations</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Turn off all optimizations.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_projection_pushdown" class="fn">with_projection_pushdown</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Toggle projection pushdown optimization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_cluster_with_columns" class="fn">with_cluster_with_columns</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Toggle cluster with columns optimization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_collapse_joins" class="fn">with_collapse_joins</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Toggle collapse joins optimization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_check_order" class="fn">with_check_order</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Check if operations are order dependent and unset maintaining_order if the order would not be observed.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_predicate_pushdown" class="fn">with_predicate_pushdown</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Toggle predicate pushdown optimization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_type_coercion" class="fn">with_type_coercion</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Toggle type coercion optimization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_type_check" class="fn">with_type_check</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Toggle type check optimization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_simplify_expr" class="fn">with_simplify_expr</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Toggle expression simplification optimization on or off.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_slice_pushdown" class="fn">with_slice_pushdown</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Toggle slice pushdown optimization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_new_streaming" class="fn">with_new_streaming</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_row_estimate" class="fn">with_row_estimate</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Try to estimate the number of rows so that joins can determine which side to keep in memory.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method._with_eager" class="fn">_with_eager</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Run every node eagerly. This turns off multi-node optimizations.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.describe_plan" class="fn">describe_plan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Return a String describing the naive (un-optimized) logical plan.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.describe_plan_tree" class="fn">describe_plan_tree</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Return a String describing the naive (un-optimized) logical plan in tree format.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.describe_optimized_plan" class="fn">describe_optimized_plan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Return a String describing the optimized logical plan.

Returns `Err` if optimizing the logical plan fails.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.describe_optimized_plan_tree" class="fn">describe_optimized_plan_tree</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Return a String describing the optimized logical plan in tree format.

Returns `Err` if optimizing the logical plan fails.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.explain" class="fn">explain</a>(&self, optimized: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Return a String describing the logical plan.

If `optimized` is `true`, explains the optimized plan. If `optimized` is `false`, explains the naive, un-optimized plan.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sort" class="fn">sort</a>( self, by: impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, sort_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Add a sort operation to the logical plan.

Sorts the LazyFrame by the column name specified using the provided options.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example" class="doc-anchor">§</a>Example

Sort DataFrame by ‘sepal_width’ column:

``` rust
fn sort_by_a(df: DataFrame) -> LazyFrame {
    df.lazy().sort(["sepal_width"], Default::default())
}
```

Sort by a single column with specific order:

``` rust
fn sort_with_specific_order(df: DataFrame, descending: bool) -> LazyFrame {
    df.lazy().sort(
        ["sepal_width"],
        SortMultipleOptions::new()
            .with_order_descending(descending)
    )
}
```

Sort by multiple columns with specifying order for each column:

``` rust
fn sort_by_multiple_columns_with_specific_order(df: DataFrame) -> LazyFrame {
    df.lazy().sort(
        ["sepal_width", "sepal_length"],
        SortMultipleOptions::new()
            .with_order_descending_multi([false, true])
    )
}
```

See [`SortMultipleOptions`](https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html "struct polars::prelude::SortMultipleOptions") for more options.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sort_by_exprs" class="fn">sort_by_exprs</a>\<E\>( self, by_exprs: E, sort_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

Add a sort operation to the logical plan.

Sorts the LazyFrame by the provided list of expressions, which will be turned into concrete columns before sorting.

See [`SortMultipleOptions`](https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html "struct polars::prelude::SortMultipleOptions") for more options.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-1" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

/// Sort DataFrame by 'sepal_width' column
fn example(df: DataFrame) -> LazyFrame {
      df.lazy()
        .sort_by_exprs(vec![col("sepal_width")], Default::default())
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.top_k" class="fn">top_k</a>\<E\>( self, k: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, by_exprs: E, sort_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.bottom_k" class="fn">bottom_k</a>\<E\>( self, k: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, by_exprs: E, sort_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.reverse" class="fn">reverse</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Reverse the `DataFrame` from top to bottom.

Row `i` becomes row `number_of_rows - i - 1`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-2" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn example(df: DataFrame) -> LazyFrame {
      df.lazy()
        .reverse()
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.rename" class="fn">rename</a>\<I, J, T, S\>(self, existing: I, new: J, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, J: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Rename columns in the DataFrame.

`existing` and `new` are iterables of the same length containing the old and corresponding new column names. Renaming happens to all `existing` columns simultaneously, not iteratively. If `strict` is true, all columns in `existing` must be present in the `LazyFrame` when `rename` is called; otherwise, only those columns that are actually found will be renamed (others will be ignored).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.drop" class="fn">drop</a>(self, columns: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Removes columns from the DataFrame. Note that it’s better to only select the columns you need and let the projection pushdown optimize away the unneeded columns.

Any given columns that are not in the schema will give a [`PolarsError::ColumnNotFound`](https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html#variant.ColumnNotFound "variant polars::prelude::PolarsError::ColumnNotFound") error while materializing the [`LazyFrame`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.shift" class="fn">shift</a>\<E\>(self, n: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Shift the values by a given period and fill the parts that will be empty due to this operation with `Nones`.

See the method on [Series](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.shift "method polars::prelude::SeriesTrait::shift") for more info on the `shift` operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.shift_and_fill" class="fn">shift_and_fill</a>\<E, IE\>(self, n: E, fill_value: IE) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>, IE: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Shift the values by a given period and fill the parts that will be empty due to this operation with the result of the `fill_value` expression.

See the method on [Series](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.shift "method polars::prelude::SeriesTrait::shift") for more info on the `shift` operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.fill_null" class="fn">fill_null</a>\<E\>(self, fill_value: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Fill None values in the DataFrame with an expression.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.fill_nan" class="fn">fill_nan</a>\<E\>(self, fill_value: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Fill NaN values in the DataFrame with an expression.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.cache" class="fn">cache</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Caches the result into a new LazyFrame.

This should be used to prevent computations running multiple times.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.cast" class="fn">cast</a>( self, dtypes: <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Cast named frame columns, resulting in a new LazyFrame with updated dtypes

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.cast_all" class="fn">cast_all</a>(self, dtype: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>\>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Cast all frame columns to the given dtype, resulting in a new LazyFrame

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.optimize" class="fn">optimize</a>( self, lp_arena: &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/enum.IR.html" class="enum" title="enum polars_plan::plans::ir::IR">IR</a>\>, expr_arena: &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/enum.AExpr.html" class="enum" title="enum polars_plan::plans::aexpr::AExpr">AExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Node.html" class="struct" title="struct polars_utils::arena::Node">Node</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.to_alp_optimized" class="fn">to_alp_optimized</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/struct.IRPlan.html" class="struct" title="struct polars_plan::plans::ir::IRPlan">IRPlan</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.to_alp" class="fn">to_alp</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/struct.IRPlan.html" class="struct" title="struct polars_plan::plans::ir::IRPlan">IRPlan</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method._collect_post_opt" class="fn">_collect_post_opt</a>\<P\>(self, post_opt: P) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Node.html" class="struct" title="struct polars_utils::arena::Node">Node</a>, &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/enum.IR.html" class="enum" title="enum polars_plan::plans::ir::IR">IR</a>\>, &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/enum.AExpr.html" class="enum" title="enum polars_plan::plans::aexpr::AExpr">AExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.collect_with_engine" class="fn">collect_with_engine</a>( self, engine: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Engine.html" class="enum" title="enum polars::prelude::Engine">Engine</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Execute all the lazy operations and collect them into a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") using a specified `engine`.

The query is optimized prior to execution.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.explain_all" class="fn">explain_all</a>( plans: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>\>, opt_state: <a href="https://docs.rs/polars/latest/polars/prelude/struct.OptFlags.html" class="struct" title="struct polars::prelude::OptFlags">OptFlags</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.collect_all_with_engine" class="fn">collect_all_with_engine</a>( plans: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>\>, engine: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Engine.html" class="enum" title="enum polars::prelude::Engine">Engine</a>, opt_state: <a href="https://docs.rs/polars/latest/polars/prelude/struct.OptFlags.html" class="struct" title="struct polars::prelude::OptFlags">OptFlags</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.collect" class="fn">collect</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Execute all the lazy operations and collect them into a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

The query is optimized prior to execution.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-3" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.lazy()
      .group_by([col("foo")])
      .agg([col("bar").sum(), col("ham").mean().alias("avg_ham")])
      .collect()
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method._profile_post_opt" class="fn">_profile_post_opt</a>\<P\>( self, post_opt: P, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Node.html" class="struct" title="struct polars_utils::arena::Node">Node</a>, &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/ir/enum.IR.html" class="enum" title="enum polars_plan::plans::ir::IR">IR</a>\>, &mut <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/arena/struct.Arena.html" class="struct" title="struct polars_utils::arena::Arena">Arena</a>\<<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/aexpr/enum.AExpr.html" class="enum" title="enum polars_plan::plans::aexpr::AExpr">AExpr</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.profile" class="fn">profile</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Profile a LazyFrame.

This will run the query and return a tuple containing the materialized DataFrame and a DataFrame that contains profiling information of each node that is executed.

The units of the timings are microseconds.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sink_parquet" class="fn">sink_parquet</a>( self, target: <a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkTarget.html" class="enum" title="enum polars::prelude::SinkTarget">SinkTarget</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, sink_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions">SinkOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Stream a query result into a parquet file. This is useful if the final result doesn’t fit into memory. This methods will return an error if the query cannot be completely done in a streaming fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sink_ipc" class="fn">sink_ipc</a>( self, target: <a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkTarget.html" class="enum" title="enum polars::prelude::SinkTarget">SinkTarget</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriterOptions.html" class="struct" title="struct polars::prelude::IpcWriterOptions">IpcWriterOptions</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, sink_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions">SinkOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Stream a query result into an ipc/arrow file. This is useful if the final result doesn’t fit into memory. This methods will return an error if the query cannot be completely done in a streaming fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sink_csv" class="fn">sink_csv</a>( self, target: <a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkTarget.html" class="enum" title="enum polars::prelude::SinkTarget">SinkTarget</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriterOptions.html" class="struct" title="struct polars::prelude::CsvWriterOptions">CsvWriterOptions</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, sink_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions">SinkOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Stream a query result into an csv file. This is useful if the final result doesn’t fit into memory. This methods will return an error if the query cannot be completely done in a streaming fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sink_json" class="fn">sink_json</a>( self, target: <a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkTarget.html" class="enum" title="enum polars::prelude::SinkTarget">SinkTarget</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriterOptions.html" class="struct" title="struct polars::prelude::JsonWriterOptions">JsonWriterOptions</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, sink_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions">SinkOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Stream a query result into a JSON file. This is useful if the final result doesn’t fit into memory. This methods will return an error if the query cannot be completely done in a streaming fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sink_parquet_partitioned" class="fn">sink_parquet_partitioned</a>( self, base_path: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\>, file_path_cb: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallback.html" class="enum" title="enum polars::prelude::PartitionTargetCallback">PartitionTargetCallback</a>\>, variant: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariant.html" class="enum" title="enum polars::prelude::PartitionVariant">PartitionVariant</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriteOptions.html" class="struct" title="struct polars::prelude::ParquetWriteOptions">ParquetWriteOptions</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, sink_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions">SinkOptions</a>, per_partition_sort_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortColumn.html" class="struct" title="struct polars::prelude::SortColumn">SortColumn</a>\>\>, finish_callback: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkFinishCallback.html" class="enum" title="enum polars::prelude::SinkFinishCallback">SinkFinishCallback</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Stream a query result into a parquet file in a partitioned manner. This is useful if the final result doesn’t fit into memory. This methods will return an error if the query cannot be completely done in a streaming fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sink_ipc_partitioned" class="fn">sink_ipc_partitioned</a>( self, base_path: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\>, file_path_cb: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallback.html" class="enum" title="enum polars::prelude::PartitionTargetCallback">PartitionTargetCallback</a>\>, variant: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariant.html" class="enum" title="enum polars::prelude::PartitionVariant">PartitionVariant</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriterOptions.html" class="struct" title="struct polars::prelude::IpcWriterOptions">IpcWriterOptions</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, sink_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions">SinkOptions</a>, per_partition_sort_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortColumn.html" class="struct" title="struct polars::prelude::SortColumn">SortColumn</a>\>\>, finish_callback: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkFinishCallback.html" class="enum" title="enum polars::prelude::SinkFinishCallback">SinkFinishCallback</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Stream a query result into an ipc/arrow file in a partitioned manner. This is useful if the final result doesn’t fit into memory. This methods will return an error if the query cannot be completely done in a streaming fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sink_csv_partitioned" class="fn">sink_csv_partitioned</a>( self, base_path: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\>, file_path_cb: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallback.html" class="enum" title="enum polars::prelude::PartitionTargetCallback">PartitionTargetCallback</a>\>, variant: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariant.html" class="enum" title="enum polars::prelude::PartitionVariant">PartitionVariant</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvWriterOptions.html" class="struct" title="struct polars::prelude::CsvWriterOptions">CsvWriterOptions</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, sink_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions">SinkOptions</a>, per_partition_sort_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortColumn.html" class="struct" title="struct polars::prelude::SortColumn">SortColumn</a>\>\>, finish_callback: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkFinishCallback.html" class="enum" title="enum polars::prelude::SinkFinishCallback">SinkFinishCallback</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Stream a query result into an csv file in a partitioned manner. This is useful if the final result doesn’t fit into memory. This methods will return an error if the query cannot be completely done in a streaming fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sink_json_partitioned" class="fn">sink_json_partitioned</a>( self, base_path: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\>, file_path_cb: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionTargetCallback.html" class="enum" title="enum polars::prelude::PartitionTargetCallback">PartitionTargetCallback</a>\>, variant: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PartitionVariant.html" class="enum" title="enum polars::prelude::PartitionVariant">PartitionVariant</a>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JsonWriterOptions.html" class="struct" title="struct polars::prelude::JsonWriterOptions">JsonWriterOptions</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, sink_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SinkOptions.html" class="struct" title="struct polars::prelude::SinkOptions">SinkOptions</a>, per_partition_sort_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortColumn.html" class="struct" title="struct polars::prelude::SortColumn">SortColumn</a>\>\>, finish_callback: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.SinkFinishCallback.html" class="enum" title="enum polars::prelude::SinkFinishCallback">SinkFinishCallback</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Stream a query result into a JSON file in a partitioned manner. This is useful if the final result doesn’t fit into memory. This methods will return an error if the query cannot be completely done in a streaming fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.try_new_streaming_if_requested" class="fn">try_new_streaming_if_requested</a>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-stream/0.51.0/x86_64-unknown-linux-gnu/polars_stream/skeleton/enum.QueryResult.html" class="enum" title="enum polars_stream::skeleton::QueryResult">QueryResult</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.filter" class="fn">filter</a>(self, predicate: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Filter frame rows that match a predicate expression.

The expression must yield boolean values (note that rows where the predicate resolves to `null` are *not* included in the resulting frame).

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-4" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn example(df: DataFrame) -> LazyFrame {
      df.lazy()
        .filter(col("sepal_width").is_not_null())
        .select([col("sepal_width"), col("sepal_length")])
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.remove" class="fn">remove</a>(self, predicate: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Remove frame rows that match a predicate expression.

The expression must yield boolean values (note that rows where the predicate resolves to `null` are *not* removed from the resulting frame).

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-5" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn example(df: DataFrame) -> LazyFrame {
      df.lazy()
        .remove(col("sepal_width").is_null())
        .select([col("sepal_width"), col("sepal_length")])
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.select" class="fn">select</a>\<E\>(self, exprs: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

Select (and optionally rename, with [`alias`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.alias "method polars::prelude::Expr::alias")) columns from the query.

Columns can be selected with [`col`](https://docs.rs/polars/latest/polars/prelude/fn.col.html "fn polars::prelude::col"); If you want to select all columns use `col(PlSmallStr::from_static("*"))`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-6" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

/// This function selects column "foo" and column "bar".
/// Column "bar" is renamed to "ham".
fn example(df: DataFrame) -> LazyFrame {
      df.lazy()
        .select([col("foo"),
                  col("bar").alias("ham")])
}

/// This function selects all columns except "foo"
fn exclude_a_column(df: DataFrame) -> LazyFrame {
      df.lazy()
        .select([all().exclude_cols(["foo"]).as_expr()])
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.select_seq" class="fn">select_seq</a>\<E\>(self, exprs: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.group_by" class="fn">group_by</a>\<E, IE\>(self, by: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[IE]</a>\>, IE: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Performs a “group-by” on a `LazyFrame`, producing a [`LazyGroupBy`](https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html "struct polars::prelude::LazyGroupBy"), which can subsequently be aggregated.

Takes a list of expressions to group on.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-7" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn example(df: DataFrame) -> LazyFrame {
      df.lazy()
       .group_by([col("date")])
       .agg([
           col("rain").min().alias("min_rain"),
           col("rain").sum().alias("sum_rain"),
           col("rain").quantile(lit(0.5), QuantileMethod::Nearest).alias("median_rain"),
       ])
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.rolling" class="fn">rolling</a>\<E\>( self, index_column: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, group_by: E, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingGroupOptions.html" class="struct" title="struct polars::prelude::RollingGroupOptions">RollingGroupOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

Create rolling groups based on a time column.

Also works for index values of type UInt32, UInt64, Int32, or Int64.

Different from a [`group_by_dynamic`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.group_by_dynamic "method polars::prelude::LazyFrame::group_by_dynamic"), the windows are now determined by the individual values and are not of constant intervals. For constant intervals use *group_by_dynamic*

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.group_by_dynamic" class="fn">group_by_dynamic</a>\<E\>( self, index_column: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, group_by: E, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

Group based on a time value (or index value of type Int32, Int64).

Time windows are calculated and rows are assigned to windows. Different from a normal group_by is that a row can be member of multiple groups. The time/index window could be seen as a rolling window, with a window size determined by dates/times/values instead of slots in the DataFrame.

A window is defined by:

- every: interval of the window
- period: length of the window
- offset: offset of the window

The `group_by` argument should be empty `[]` if you don’t want to combine this with a ordinary group_by on these keys.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.group_by_stable" class="fn">group_by_stable</a>\<E, IE\>(self, by: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[IE]</a>\>, IE: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Similar to [`group_by`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.group_by "method polars::prelude::LazyFrame::group_by"), but order of the DataFrame is maintained.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.anti_join" class="fn">anti_join</a>\<E\>( self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, left_on: E, right_on: E, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Left anti join this query with another lazy query.

Matches on the values of the expressions `left_on` and `right_on`. For more flexible join logic, see [`join`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join "method polars::prelude::LazyFrame::join") or [`join_builder`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join_builder "method polars::prelude::LazyFrame::join_builder").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-8" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;
fn anti_join_dataframes(ldf: LazyFrame, other: LazyFrame) -> LazyFrame {
        ldf
        .anti_join(other, col("foo"), col("bar").cast(DataType::String))
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.cross_join" class="fn">cross_join</a>( self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, suffix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Creates the Cartesian product from both frames, preserving the order of the left keys.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.left_join" class="fn">left_join</a>\<E\>( self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, left_on: E, right_on: E, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Left outer join this query with another lazy query.

Matches on the values of the expressions `left_on` and `right_on`. For more flexible join logic, see [`join`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join "method polars::prelude::LazyFrame::join") or [`join_builder`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join_builder "method polars::prelude::LazyFrame::join_builder").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-9" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;
fn left_join_dataframes(ldf: LazyFrame, other: LazyFrame) -> LazyFrame {
        ldf
        .left_join(other, col("foo"), col("bar"))
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.inner_join" class="fn">inner_join</a>\<E\>( self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, left_on: E, right_on: E, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Inner join this query with another lazy query.

Matches on the values of the expressions `left_on` and `right_on`. For more flexible join logic, see [`join`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join "method polars::prelude::LazyFrame::join") or [`join_builder`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join_builder "method polars::prelude::LazyFrame::join_builder").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-10" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;
fn inner_join_dataframes(ldf: LazyFrame, other: LazyFrame) -> LazyFrame {
        ldf
        .inner_join(other, col("foo"), col("bar").cast(DataType::String))
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.full_join" class="fn">full_join</a>\<E\>( self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, left_on: E, right_on: E, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Full outer join this query with another lazy query.

Matches on the values of the expressions `left_on` and `right_on`. For more flexible join logic, see [`join`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join "method polars::prelude::LazyFrame::join") or [`join_builder`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join_builder "method polars::prelude::LazyFrame::join_builder").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-11" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;
fn full_join_dataframes(ldf: LazyFrame, other: LazyFrame) -> LazyFrame {
        ldf
        .full_join(other, col("foo"), col("bar"))
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.semi_join" class="fn">semi_join</a>\<E\>( self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, left_on: E, right_on: E, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>,

Left semi join this query with another lazy query.

Matches on the values of the expressions `left_on` and `right_on`. For more flexible join logic, see [`join`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join "method polars::prelude::LazyFrame::join") or [`join_builder`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join_builder "method polars::prelude::LazyFrame::join_builder").

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-12" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;
fn semi_join_dataframes(ldf: LazyFrame, other: LazyFrame) -> LazyFrame {
        ldf
        .semi_join(other, col("foo"), col("bar").cast(DataType::String))
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join" class="fn">join</a>\<E\>( self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, left_on: E, right_on: E, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinArgs.html" class="struct" title="struct polars::prelude::JoinArgs">JoinArgs</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

Generic function to join two LazyFrames.

`join` can join on multiple columns, given as two list of expressions, and with a [`JoinType`](https://docs.rs/polars/latest/polars/prelude/enum.JoinType.html "enum polars::prelude::JoinType") specified by `how`. Non-joined column names in the right DataFrame that already exist in this DataFrame are suffixed with `"_right"`. For control over how columns are renamed and parallelization options, use [`join_builder`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join_builder "method polars::prelude::LazyFrame::join_builder").

Any provided `args.slice` parameter is not considered, but set by the internal optimizer.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-13" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;

fn example(ldf: LazyFrame, other: LazyFrame) -> LazyFrame {
        ldf
        .join(other, [col("foo"), col("bar")], [col("foo"), col("bar")], JoinArgs::new(JoinType::Inner))
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.join_builder" class="fn">join_builder</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html" class="struct" title="struct polars::prelude::JoinBuilder">JoinBuilder</a>

Consume `self` and return a [`JoinBuilder`](https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html "struct polars::prelude::JoinBuilder") to customize a join on this LazyFrame.

After the `JoinBuilder` has been created and set up, calling [`finish()`](https://docs.rs/polars/latest/polars/prelude/struct.JoinBuilder.html#method.finish "method polars::prelude::JoinBuilder::finish") on it will give back the `LazyFrame` representing the `join` operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_column" class="fn">with_column</a>(self, expr: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Add or replace a column, given as an expression, to a DataFrame.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-14" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;
fn add_column(df: DataFrame) -> LazyFrame {
    df.lazy()
        .with_column(
            when(col("sepal_length").lt(lit(5.0)))
            .then(lit(10))
            .otherwise(lit(1))
            .alias("new_column_name"),
        )
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_columns" class="fn">with_columns</a>\<E\>(self, exprs: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

Add or replace multiple columns, given as expressions, to a DataFrame.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#example-15" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_lazy::prelude::*;
fn add_columns(df: DataFrame) -> LazyFrame {
    df.lazy()
        .with_columns(
            vec![lit(10).alias("foo"), lit(100).alias("bar")]
         )
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_columns_seq" class="fn">with_columns_seq</a>\<E\>(self, exprs: E) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where E: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\]\>,

Add or replace multiple columns to a DataFrame, but evaluate them sequentially.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.match_to_schema" class="fn">match_to_schema</a>( self, schema: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, per_column: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.MatchToSchemaPerColumn.html" class="struct" title="struct polars::prelude::MatchToSchemaPerColumn">MatchToSchemaPerColumn</a>\]\>, extra_columns: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ExtraColumnsPolicy.html" class="enum" title="enum polars::prelude::ExtraColumnsPolicy">ExtraColumnsPolicy</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Match or evolve to a certain schema.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.pipe_with_schema" class="fn">pipe_with_schema</a>( self, callback: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlanCallback.html" class="enum" title="enum polars::prelude::PlanCallback">PlanCallback</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>, <a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_context" class="fn">with_context</a>\<C\>(self, contexts: C) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where C: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>\]\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.max" class="fn">max</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as their maximum values.

Aggregated columns will have the same names as the original columns.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.min" class="fn">min</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as their minimum values.

Aggregated columns will have the same names as the original columns.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.sum" class="fn">sum</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as their sum values.

Aggregated columns will have the same names as the original columns.

- Boolean columns will sum to a `u32` containing the number of `true`s.
- For integer columns, the ordinary checks for overflow are performed: if running in `debug` mode, overflows will panic, whereas in `release` mode overflows will silently wrap.
- String columns will sum to None.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.mean" class="fn">mean</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as their mean values.

- Boolean and integer columns are converted to `f64` before computing the mean.
- String columns will have a mean of None.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.median" class="fn">median</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as their median values.

- Boolean and integer results are converted to `f64`. However, they are still susceptible to overflow before this conversion occurs.
- String columns will sum to None.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.quantile" class="fn">quantile</a>(self, quantile: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>, method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as their quantile values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.std" class="fn">std</a>(self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as their standard deviation values.

`ddof` is the “Delta Degrees of Freedom”; `N - ddof` will be the denominator when computing the variance, where `N` is the number of rows.

> In standard statistical practice, `ddof=1` provides an unbiased estimator of the variance of a hypothetical infinite population. `ddof=0` provides a maximum likelihood estimate of the variance for normally distributed variables. The standard deviation computed in this function is the square root of the estimated variance, so even with `ddof=1`, it will not be an unbiased estimate of the standard deviation per se.

Source: [Numpy](https://numpy.org/doc/stable/reference/generated/numpy.std.html#)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.var" class="fn">var</a>(self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as their variance values.

`ddof` is the “Delta Degrees of Freedom”; `N - ddof` will be the denominator when computing the variance, where `N` is the number of rows.

> In standard statistical practice, `ddof=1` provides an unbiased estimator of the variance of a hypothetical infinite population. `ddof=0` provides a maximum likelihood estimate of the variance for normally distributed variables.

Source: [Numpy](https://numpy.org/doc/stable/reference/generated/numpy.var.html#)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.explode" class="fn">explode</a>(self, columns: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Apply explode operation. [See eager explode](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.explode "method polars::prelude::DataFrame::explode").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.null_count" class="fn">null_count</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Aggregate all the columns as the sum of their null value count.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.unique_stable" class="fn">unique_stable</a>( self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>, keep_strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.UniqueKeepStrategy.html" class="enum" title="enum polars::prelude::UniqueKeepStrategy">UniqueKeepStrategy</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Drop non-unique rows and maintain the order of kept rows.

`subset` is an optional `Vec` of column names to consider for uniqueness; if `None`, all columns are considered.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.unique_stable_generic" class="fn">unique_stable_generic</a>( self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>, keep_strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.UniqueKeepStrategy.html" class="enum" title="enum polars::prelude::UniqueKeepStrategy">UniqueKeepStrategy</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.unique" class="fn">unique</a>( self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>, keep_strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.UniqueKeepStrategy.html" class="enum" title="enum polars::prelude::UniqueKeepStrategy">UniqueKeepStrategy</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Drop non-unique rows without maintaining the order of kept rows.

The order of the kept rows may change; to maintain the original row order, use [`unique_stable`](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.unique_stable "method polars::prelude::LazyFrame::unique_stable").

`subset` is an optional `Vec` of column names to consider for uniqueness; if None, all columns are considered.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.unique_generic" class="fn">unique_generic</a>( self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>, keep_strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.UniqueKeepStrategy.html" class="enum" title="enum polars::prelude::UniqueKeepStrategy">UniqueKeepStrategy</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.drop_nans" class="fn">drop_nans</a>(self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Drop rows containing one or more NaN values.

`subset` is an optional `Vec` of column names to consider for NaNs; if None, all floating point columns are considered.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.drop_nulls" class="fn">drop_nulls</a>(self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Drop rows containing one or more None values.

`subset` is an optional `Vec` of column names to consider for nulls; if None, all columns are considered.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.slice" class="fn">slice</a>(self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Slice the DataFrame using an offset (starting row) and a length.

If `offset` is negative, it is counted from the end of the DataFrame. For instance, `lf.slice(-5, 3)` gets three rows, starting at the row fifth from the end.

If `offset` and `len` are such that the slice extends beyond the end of the DataFrame, the portion between `offset` and the end will be returned. In this case, the number of rows in the returned DataFrame will be less than `len`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.first" class="fn">first</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Get the first row.

Equivalent to `self.slice(0, 1)`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.last" class="fn">last</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Get the last row.

Equivalent to `self.slice(-1, 1)`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.tail" class="fn">tail</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Get the last `n` rows.

Equivalent to `self.slice(-(n as i64), n)`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.limit" class="fn">limit</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Limit the DataFrame to the first `n` rows.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.map" class="fn">map</a>\<F\>( self, function: F, optimizations: <a href="https://docs.rs/polars/latest/polars/prelude/struct.OptFlags.html" class="struct" title="struct polars::prelude::OptFlags">OptFlags</a>, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.UdfSchema.html" class="trait" title="trait polars::prelude::UdfSchema">UdfSchema</a>\>\>, name: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where F: 'static + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Apply a function/closure once the logical plan get executed.

The function has access to the whole materialized DataFrame at the time it is called.

To apply specific functions to specific columns, use [`Expr::map`](https://docs.rs/polars/latest/polars/prelude/enum.Expr.html#method.map "method polars::prelude::Expr::map") in conjunction with `LazyFrame::with_column` or `with_columns`.

###### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#warning" class="doc-anchor">§</a>Warning

This can blow up in your face if the schema is changed due to the operation. The optimizer relies on a correct schema.

You can toggle certain optimizations off.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.with_row_index" class="fn">with_row_index</a>\<S\>(self, name: S, offset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Add a new column at index 0 that counts the rows.

`name` is the name of the new column. `offset` is where to start counting from; if `None`, it is set to `0`.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#warning-1" class="doc-anchor">§</a>Warning

This can have a negative effect on query performance. This may for instance block predicate pushdown optimization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.count" class="fn">count</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Return the number of non-null elements for each column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.unnest" class="fn">unnest</a>(self, cols: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Selector.html" class="enum" title="enum polars::prelude::Selector">Selector</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Unnest the given `Struct` columns: the fields of the `Struct` type will be inserted as columns.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-LazyFrame-4" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.anonymous_scan" class="fn">anonymous_scan</a>( function: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousScan.html" class="trait" title="trait polars::prelude::AnonymousScan">AnonymousScan</a>\>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsAnonymous.html" class="struct" title="struct polars::prelude::ScanArgsAnonymous">ScanArgsAnonymous</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-LazyFrame-5" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.scan_ipc" class="fn">scan_ipc</a>( path: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html" class="struct" title="struct polars::prelude::ScanArgsIpc">ScanArgsIpc</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a LazyFrame directly from a ipc scan.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.scan_ipc_files" class="fn">scan_ipc_files</a>( paths: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html" class="struct" title="struct polars::prelude::ScanArgsIpc">ScanArgsIpc</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.scan_ipc_sources" class="fn">scan_ipc_sources</a>( sources: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsIpc.html" class="struct" title="struct polars::prelude::ScanArgsIpc">ScanArgsIpc</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-LazyFrame-6" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.scan_parquet" class="fn">scan_parquet</a>( path: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html" class="struct" title="struct polars::prelude::ScanArgsParquet">ScanArgsParquet</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a LazyFrame directly from a parquet scan.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.scan_parquet_sources" class="fn">scan_parquet_sources</a>( sources: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html" class="struct" title="struct polars::prelude::ScanArgsParquet">ScanArgsParquet</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a LazyFrame directly from a parquet scan.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.scan_parquet_files" class="fn">scan_parquet_files</a>( paths: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.ScanArgsParquet.html" class="struct" title="struct polars::prelude::ScanArgsParquet">ScanArgsParquet</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a LazyFrame directly from a parquet scan.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-Clone-for-LazyFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-Default-for-LazyFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-From%3CDslPlan%3E-for-LazyFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(plan: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DslPlan.html" class="enum" title="enum polars::prelude::DslPlan">DslPlan</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-From%3CLazyGroupBy%3E-for-LazyFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(lgb: <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyGroupBy.html" class="struct" title="struct polars::prelude::LazyGroupBy">LazyGroupBy</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#impl-IntoLazy-for-LazyFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html" class="trait" title="trait polars::prelude::IntoLazy">IntoLazy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#method.lazy" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html#tymethod.lazy" class="fn">lazy</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html#blanket-implementations" class="anchor">§</a>
