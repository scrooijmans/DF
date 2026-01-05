# Struct PruningPredicate Copy item path

<a href="https://docs.rs/datafusion-pruning/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_pruning/pruning_predicate.rs.html#363" class="src">Source</a>

``` rust
pub struct PruningPredicate { /* private fields */ }
```

Expand description

Used to prove that arbitrary predicates (boolean expression) can not possibly evaluate to `true` given information about a column provided by [`PruningStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html "trait datafusion::common::pruning::PruningStatistics").

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#introduction" class="doc-anchor">§</a>Introduction

`PruningPredicate` analyzes filter expressions using statistics such as min/max values and null counts, attempting to prove a “container” (e.g. Parquet Row Group) can be skipped without reading the actual data, potentially leading to significant performance improvements.

For example, `PruningPredicate`s are used to prune Parquet Row Groups based on the min/max values found in the Parquet metadata. If the `PruningPredicate` can prove that the filter can never evaluate to `true` for any row in the Row Group, the entire Row Group is skipped during query execution.

The `PruningPredicate` API is general, and can be used for pruning other types of containers (e.g. files) based on statistics that may be known from external catalogs (e.g. Delta Lake) or other sources. How this works is a subtle topic. See the Background and Implementation section for details.

`PruningPredicate` supports:

1.  Arbitrary expressions (including user defined functions)

2.  Vectorized evaluation (provide more than one set of statistics at a time) so it is suitable for pruning 1000s of containers.

3.  Any source of information that implements the [`PruningStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html "trait datafusion::common::pruning::PruningStatistics") trait (not just Parquet metadata).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#example" class="doc-anchor">§</a>Example

See the [`pruning.rs` example in the `datafusion-examples`](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/pruning.rs) for a complete example of how to use `PruningPredicate` to prune files based on min/max values.

Given an expression like `x = 5` and statistics for 3 containers (Row Groups, files, etc) `A`, `B`, and `C`:

``` text
  A: {x_min = 0, x_max = 4}
  B: {x_min = 2, x_max = 10}
  C: {x_min = 5, x_max = 8}
```

`PruningPredicate` will conclude that the rows in container `A` can never be true (as the maximum value is only `4`), so it can be pruned:

``` text
A: false (no rows could possibly match x = 5)
B: true  (rows might match x = 5)
C: true  (rows might match x = 5)
```

See [`PruningPredicate::try_new`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.try_new "associated function datafusion::physical_optimizer::pruning::PruningPredicate::try_new") and [`PruningPredicate::prune`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.prune "method datafusion::physical_optimizer::pruning::PruningPredicate::prune") for more information.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#background" class="doc-anchor">§</a>Background

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#boolean-tri-state-logic" class="doc-anchor">§</a>Boolean Tri-state logic

To understand the details of the rest of this documentation, it is important to understand how the tri-state boolean logic in SQL works. As this is somewhat esoteric, we review it here.

SQL has a notion of `NULL` that represents the value is `“unknown”` and this uncertainty propagates through expressions. SQL `NULL` behaves very differently than the `NULL` in most other languages where it is a special, sentinel value (e.g. `0` in `C/C++`). While representing uncertainty with `NULL` is powerful and elegant, SQL `NULL`s are often deeply confusing when first encountered as they behave differently than most programmers may expect.

In most other programming languages,

- `a == NULL` evaluates to `true` if `a` also had the value `NULL`
- `a == NULL` evaluates to `false` if `a` has any other value

However, in SQL `a = NULL` **always** evaluates to `NULL` (never `true` or `false`):

| Expression    | Result |
|---------------|--------|
| `1 = NULL`    | `NULL` |
| `NULL = NULL` | `NULL` |

Also important is how `AND` and `OR` works with tri-state boolean logic as (perhaps counterintuitively) the result is **not** always NULL. While consistent with the notion of `NULL` representing “unknown”, this is again, often deeply confusing 🤯 when first encountered.

| Expression | Result | Intuition |
|----|----|----|
| `NULL AND true` | `NULL` | The `NULL` stands for “unknown” and if it were `true` or `false` the overall expression value could change |
| `NULL AND false` | `false` | If the `NULL` was either `true` or `false` the overall expression is still `false` |
| `NULL AND NULL` | `NULL` |  |

| Expression | Result | Intuition |
|----|----|----|
| `NULL OR true` | `true` | If the `NULL` was either `true` or `false` the overall expression is still `true` |
| `NULL OR false` | `NULL` | The `NULL` stands for “unknown” and if it were `true` or `false` the overall expression value could change |
| `NULL OR NULL` | `NULL` |  |

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#sql-filter-semantics" class="doc-anchor">§</a>SQL Filter Semantics

The SQL `WHERE` clause has a boolean expression, often called a filter or predicate. The semantics of this predicate are that the query evaluates the predicate for each row in the input tables and:

- Rows that evaluate to `true` are returned in the query results

- Rows that evaluate to `false` are not returned (“filtered out” or “pruned” or “skipped”).

- Rows that evaluate to `NULL` are **NOT** returned (also “filtered out”). Note: *this treatment of `NULL` is **DIFFERENT** than how `NULL` is treated in the rewritten predicate described below.*

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#pruningpredicate-implementation" class="doc-anchor">§</a>`PruningPredicate` Implementation

Armed with the information in the Background section, we can now understand how the `PruningPredicate` logic works.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#interface" class="doc-anchor">§</a>Interface

**Inputs**

1.  An input schema describing what columns exist

2.  A predicate (expression that evaluates to a boolean)

3.  [`PruningStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html "trait datafusion::common::pruning::PruningStatistics") that provides information about columns in that schema, for multiple “containers”. For each column in each container, it provides optional information on contained values, min_values, max_values, null_counts counts, and row_counts counts.

**Outputs**: A (non null) boolean value for each container:

- `true`: There MAY be rows that match the predicate

- `false`: There are no rows that could possibly match the predicate (the predicate can never possibly be true). The container can be pruned (skipped) entirely.

While `PruningPredicate` will never return a `NULL` value, the rewritten predicate (as returned by `build_predicate_expression` and used internally by `PruningPredicate`) may evaluate to `NULL` when some of the min/max values or null / row counts are not known.

In order to be correct, `PruningPredicate` must return false **only** if it can determine that for all rows in the container, the predicate could never evaluate to `true` (always evaluates to either `NULL` or `false`).

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#contains-analysis-and-minmax-rewrite" class="doc-anchor">§</a>Contains Analysis and Min/Max Rewrite

`PruningPredicate` works by first analyzing the predicate to see what [`LiteralGuarantee`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/struct.LiteralGuarantee.html "struct datafusion::physical_expr::utils::LiteralGuarantee") must hold for the predicate to be true.

Then, the `PruningPredicate` rewrites the original predicate into an expression that references the min/max values of each column in the original predicate.

When the min/max values are actually substituted in to this expression and evaluated, the result means

- `true`: there MAY be rows that pass the predicate, **KEEPS** the container

- `NULL`: there MAY be rows that pass the predicate, **KEEPS** the container Note that rewritten predicate can evaluate to NULL when some of the min/max values are not known. *Note that this is different than the SQL filter semantics where `NULL` means the row is filtered out.*

- `false`: there are no rows that could possibly match the predicate, **PRUNES** the container

For example, given a column `x`, the `x_min`, `x_max`, `x_null_count`, and `x_row_count` represent the minimum and maximum values, the null count of column `x`, and the row count of column `x`, provided by the `PruningStatistics`. `x_null_count` and `x_row_count` are used to handle the case where the column `x` is known to be all `NULL`s. Note this is different from knowing nothing about the column `x`, which confusingly is encoded by returning `NULL` for the min/max values from [`PruningStatistics::max_values`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.max_values "method datafusion::common::pruning::PruningStatistics::max_values") and [`PruningStatistics::min_values`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.min_values "method datafusion::common::pruning::PruningStatistics::min_values").

Here are some examples of the rewritten predicates:

| Original Predicate | Rewritten Predicate |
|----|----|
| `x = 5` | `x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)` |
| `x < 5` | `x_null_count != x_row_count THEN false (x_max < 5)` |
| `x = 5 AND y = 10` | `x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max) AND y_null_count != y_row_count (y_min <= 10 AND 10 <= y_max)` |
| `x IS NULL` | `x_null_count > 0` |
| `x IS NOT NULL` | `x_null_count != row_count` |
| `CAST(x as int) = 5` | `x_null_count != x_row_count (CAST(x_min as int) <= 5 AND 5 <= CAST(x_max as int))` |

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#predicate-evaluation" class="doc-anchor">§</a>Predicate Evaluation

The PruningPredicate works in two passes

**First pass**: For each `LiteralGuarantee` calls [`PruningStatistics::contained`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.contained "method datafusion::common::pruning::PruningStatistics::contained") and rules out containers where the LiteralGuarantees are not satisfied

**Second Pass**: Evaluates the rewritten expression using the min/max/null_counts/row_counts values for each column for each container. For any container that this expression evaluates to `false`, it rules out those containers.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#example-1" class="doc-anchor">§</a>Example 1

Given the predicate, `x = 5 AND y = 10`, the rewritten predicate would look like:

``` sql
x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)
AND
y_null_count != y_row_count AND (y_min <= 10 AND 10 <= y_max)
```

If we know that for a given container, `x` is between `1 and 100` and we know that `y` is between `4` and `7`, we know nothing about the null count and row count of `x` and `y`, the input statistics might look like:

| Column         | Value  |
|----------------|--------|
| `x_min`        | `1`    |
| `x_max`        | `100`  |
| `x_null_count` | `null` |
| `x_row_count`  | `null` |
| `y_min`        | `4`    |
| `y_max`        | `7`    |
| `y_null_count` | `null` |
| `y_row_count`  | `null` |

When these statistics values are substituted in to the rewritten predicate and simplified, the result is `false`:

- `null != null AND (1 <= 5 AND 5 <= 100) AND null != null AND (4 <= 10 AND 10 <= 7)`
- `null = null` is `null` which is not true, so the AND moves on to the next clause
- `null and (1 <= 5 AND 5 <= 100) AND null AND (4 <= 10 AND 10 <= 7)`
- evaluating the clauses further we get:
- `null and true and null and false`
- `null and false`
- `false`

Returning `false` means the container can be pruned, which matches the intuition that `x = 5 AND y = 10` can’t be true for any row if all values of `y` are `7` or less.

Note that if we had ended up with `null AND true AND null AND true` the result would have been `null`. `null` is treated the same as`true`, because we can’t prove that the predicate is `false.`

If, for some other container, we knew `y` was between the values `4` and `15`, then the rewritten predicate evaluates to `true` (verifying this is left as an exercise to the reader – are you still here?), and the container **could not** be pruned. The intuition is that there may be rows where the predicate *might* evaluate to `true`, and the only way to find out is to do more analysis, for example by actually reading the data and evaluating the predicate row by row.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#example-2" class="doc-anchor">§</a>Example 2

Given the same predicate, `x = 5 AND y = 10`, the rewritten predicate would look like the same as example 1:

``` sql
x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)
AND
y_null_count != y_row_count AND (y_min <= 10 AND 10 <= y_max)
```

If we know that for another given container, `x_min` is NULL and `x_max` is NULL (the min/max values are unknown), `x_null_count` is `100` and `x_row_count` is `100`; we know that `y` is between `4` and `7`, but we know nothing about the null count and row count of `y`. The input statistics might look like:

| Column         | Value  |
|----------------|--------|
| `x_min`        | `null` |
| `x_max`        | `null` |
| `x_null_count` | `100`  |
| `x_row_count`  | `100`  |
| `y_min`        | `4`    |
| `y_max`        | `7`    |
| `y_null_count` | `null` |
| `y_row_count`  | `null` |

When these statistics values are substituted in to the rewritten predicate and simplified, the result is `false`:

- `100 != 100 AND (null <= 5 AND 5 <= null) AND null = null AND (4 <= 10 AND 10 <= 7)`
- `false AND null AND null AND false`
- `false AND false`
- `false`

Returning `false` means the container can be pruned, which matches the intuition that `x = 5 AND y = 10` can’t be true because all values in `x` are known to be NULL.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#related-work" class="doc-anchor">§</a>Related Work

[`PruningPredicate`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html "struct datafusion::physical_optimizer::pruning::PruningPredicate") implements the type of min/max pruning described in Section `3.3.3` of the [`Snowflake SIGMOD Paper`](https://dl.acm.org/doi/10.1145/2882903.2903741). The technique is described by various research such as [small materialized aggregates](https://www.vldb.org/conf/1998/p476.pdf), [zone maps](https://dl.acm.org/doi/10.1007/978-3-642-03730-6_10), and [data skipping](https://dl.acm.org/doi/10.1145/2588555.2610515).

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#impl-PruningPredicate" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html" class="struct" title="struct datafusion::physical_optimizer::pruning::PruningPredicate">PruningPredicate</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.try_new" class="fn">try_new</a>( expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html" class="struct" title="struct datafusion::physical_optimizer::pruning::PruningPredicate">PruningPredicate</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to create a new instance of [`PruningPredicate`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html "struct datafusion::physical_optimizer::pruning::PruningPredicate")

This will translate the provided `expr` filter expression into a *pruning predicate*.

A pruning predicate is one that has been rewritten in terms of the min and max values of column references and that evaluates to FALSE if the filter predicate would evaluate FALSE *for every row* whose values fell within the min / max ranges (aka could be pruned).

The pruning predicate evaluates to TRUE or NULL if the filter predicate *might* evaluate to TRUE for at least one row whose values fell within the min/max ranges (in other words they might pass the predicate)

For example, the filter expression `(column / 2) = 4` becomes the pruning predicate `(column_min / 2) <= 4 && 4 <= (column_max / 2))`

See the struct level documentation on [`PruningPredicate`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html "struct datafusion::physical_optimizer::pruning::PruningPredicate") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.prune" class="fn">prune</a>\<S\>(&self, statistics: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;S</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where S: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html" class="trait" title="trait datafusion::common::pruning::PruningStatistics">PruningStatistics</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

For each set of statistics, evaluates the pruning predicate and returns a `bool` with the following meaning for a all rows whose values match the statistics:

`true`: There MAY be rows that match the predicate

`false`: There are no rows that could possibly match the predicate

Note: the predicate passed to `prune` should already be simplified as much as possible (e.g. this pass doesn’t handle some expressions like `b = false`, but it does handle the simplified version `b`. See [`ExprSimplifier`](https://docs.rs/datafusion/latest/datafusion/optimizer/simplify_expressions/struct.ExprSimplifier.html) to simplify expressions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Return a reference to the input schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.orig_expr" class="fn">orig_expr</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>

Returns a reference to the physical expr used to construct this pruning predicate

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.predicate_expr" class="fn">predicate_expr</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>

Returns a reference to the predicate expr

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.literal_guarantees" class="fn">literal_guarantees</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/utils/struct.LiteralGuarantee.html" class="struct" title="struct datafusion::physical_expr::utils::LiteralGuarantee">LiteralGuarantee</a>\]

Returns a reference to the literal guarantees

Note that **All** `LiteralGuarantee`s must be satisfied for the expression to possibly be `true`. If any is not satisfied, the expression is guaranteed to be `null` or `false`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.always_true" class="fn">always_true</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this pruning predicate can not prune anything.

This happens if the predicate is a literal `true` and literal_guarantees is empty.

This can happen when a predicate is simplified to a constant `true`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.required_columns" class="fn">required_columns</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.RequiredColumns.html" class="struct" title="struct datafusion::physical_optimizer::pruning::RequiredColumns">RequiredColumns</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.literal_columns" class="fn">literal_columns</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Names of the columns that are known to be / not be in a set of literals (constants). These are the columns the that may be passed to [`PruningStatistics::contained`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html#tymethod.contained "method datafusion::common::pruning::PruningStatistics::contained") during pruning.

This is useful to avoid fetching statistics for columns that will not be used in the predicate. For example, it can be used to avoid reading unneeded bloom filters (a non trivial operation).

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#impl-Clone-for-PruningPredicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html" class="struct" title="struct datafusion::physical_optimizer::pruning::PruningPredicate">PruningPredicate</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html" class="struct" title="struct datafusion::physical_optimizer::pruning::PruningPredicate">PruningPredicate</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#impl-Debug-for-PruningPredicate" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html" class="struct" title="struct datafusion::physical_optimizer::pruning::PruningPredicate">PruningPredicate</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html#blanket-implementations" class="anchor">§</a>
