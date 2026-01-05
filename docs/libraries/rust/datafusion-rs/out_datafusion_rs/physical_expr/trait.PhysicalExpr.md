# Trait PhysicalExpr Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/physical_expr.rs.html#71" class="src">Source</a>

``` rust
pub trait PhysicalExpr:
    Any
    + Send
    + Sync
    + Display
    + Debug
    + DynEq
    + DynHash {
Show 17 methods    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn evaluate(
        &self,
        batch: &RecordBatch,
    ) -> Result<ColumnarValue, DataFusionError>;
    fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>;
    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn PhysicalExpr>>,
    ) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>;
    fn fmt_sql(&self, f: &mut Formatter<'_>) -> Result<(), Error>;

    // Provided methods
    fn data_type(
        &self,
        input_schema: &Schema,
    ) -> Result<DataType, DataFusionError> { ... }
    fn nullable(&self, input_schema: &Schema) -> Result<bool, DataFusionError> { ... }
    fn return_field(
        &self,
        input_schema: &Schema,
    ) -> Result<Arc<Field>, DataFusionError> { ... }
    fn evaluate_selection(
        &self,
        batch: &RecordBatch,
        selection: &BooleanArray,
    ) -> Result<ColumnarValue, DataFusionError> { ... }
    fn evaluate_bounds(
        &self,
        _children: &[&Interval],
    ) -> Result<Interval, DataFusionError> { ... }
    fn propagate_constraints(
        &self,
        _interval: &Interval,
        _children: &[&Interval],
    ) -> Result<Option<Vec<Interval>>, DataFusionError> { ... }
    fn evaluate_statistics(
        &self,
        children: &[&Distribution],
    ) -> Result<Distribution, DataFusionError> { ... }
    fn propagate_statistics(
        &self,
        parent: &Distribution,
        children: &[&Distribution],
    ) -> Result<Option<Vec<Distribution>>, DataFusionError> { ... }
    fn get_properties(
        &self,
        _children: &[ExprProperties],
    ) -> Result<ExprProperties, DataFusionError> { ... }
    fn snapshot(&self) -> Result<Option<Arc<dyn PhysicalExpr>>, DataFusionError> { ... }
    fn snapshot_generation(&self) -> u64 { ... }
    fn is_volatile_node(&self) -> bool { ... }
}
```

Expand description

[`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr")s represent expressions such as `A + 1` or `CAST(c1 AS int)`.

`PhysicalExpr` knows its type, nullability and can be evaluated directly on a [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") (see [`Self::evaluate`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.evaluate "method datafusion_physical_expr_common::physical_expr::PhysicalExpr::evaluate::evaluate")).

`PhysicalExpr` are the physical counterpart to [`Expr`](https://docs.rs/datafusion/latest/datafusion/logical_expr/enum.Expr.html) used in logical planning. They are typically created from [`Expr`](https://docs.rs/datafusion/latest/datafusion/logical_expr/enum.Expr.html) by a [`PhysicalPlanner`](https://docs.rs/datafusion/latest/datafusion/physical_planner/trait.PhysicalPlanner.html) invoked from a higher level API

Some important examples of `PhysicalExpr` are:

- [`Column`](https://docs.rs/datafusion/latest/datafusion/physical_expr/expressions/struct.Column.html): Represents a column at a given index in a RecordBatch

To create `PhysicalExpr` from `Expr`, see

- [`SessionContext::create_physical_expr`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.create_physical_expr): A high level API
- [`create_physical_expr`](https://docs.rs/datafusion/latest/datafusion/physical_expr/fn.create_physical_expr.html): A low level API

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#formatting-physicalexpr-as-strings" class="doc-anchor">§</a>Formatting `PhysicalExpr` as strings

There are three ways to format `PhysicalExpr` as a string:

- [`Debug`](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"): Standard Rust debugging format (e.g. `Constant { value: ... }`)
- [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"): Detailed SQL-like format that shows expression structure (e.g. (`Utf8 ("foobar")`). This is often used for debugging and tests
- [`Self::fmt_sql`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.fmt_sql "method datafusion_physical_expr_common::physical_expr::PhysicalExpr::fmt_sql::fmt_sql"): SQL-like human readable format (e.g. (‘foobar’)`), See also [`sql_fmt\`\]

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the physical expression as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.evaluate" class="fn">evaluate</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate an expression against a RecordBatch

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.children" class="fn">children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Get a list of child PhysicalExpr that provide the input for this expr.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.with_new_children" class="fn">with_new_children</a>( self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<Self\>, children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a new PhysicalExpr where all children were replaced by new exprs.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#tymethod.fmt_sql" class="fn">fmt_sql</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format this `PhysicalExpr` in nice human readable “SQL” format

Specifically, this format is designed to be readable by humans, at the expense of details. Use `Display` or `Debug` for more detailed representation.

See the [`fmt_sql`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.fmt_sql.html "fn datafusion::physical_expr_common::physical_expr::fmt_sql") function for an example of printing `PhysicalExpr`s as SQL.

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.data_type" class="fn">data_type</a>(&self, input_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Get the data type of this expression, given the schema of the input

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.nullable" class="fn">nullable</a>(&self, input_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Determine whether this expression is nullable, given the schema of the input

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.return_field" class="fn">return_field</a>( &self, input_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

The output field associated with this expression

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.evaluate_selection" class="fn">evaluate_selection</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, selection: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Evaluate an expression against a RecordBatch after first applying a validity array

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.evaluate_bounds" class="fn">evaluate_bounds</a>( &self, \_children: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Computes the output interval for the expression, given the input intervals.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#parameters" class="doc-anchor">§</a>Parameters

- `children` are the intervals for the children (inputs) of this expression.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#returns" class="doc-anchor">§</a>Returns

A `Result` containing the output interval for the expression in case of success, or an error object in case of failure.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#example" class="doc-anchor">§</a>Example

If the expression is `a + b`, and the input intervals are `a: [1, 2]` and `b: [3, 4]`, then the output interval would be `[4, 6]`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.propagate_constraints" class="fn">propagate_constraints</a>( &self, \_interval: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>, \_children: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates bounds for child expressions, given a known interval for this expression.

This is used to propagate constraints down through an expression tree.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#parameters-1" class="doc-anchor">§</a>Parameters

- `interval` is the currently known interval for this expression.
- `children` are the current intervals for the children of this expression.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#returns-1" class="doc-anchor">§</a>Returns

A `Result` containing a `Vec` of new intervals for the children (in order) in case of success, or an error object in case of failure.

If constraint propagation reveals an infeasibility for any child, returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). If none of the children intervals change as a result of propagation, may return an empty vector instead of cloning `children`. This is the default (and conservative) return value.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#example-1" class="doc-anchor">§</a>Example

If the expression is `a + b`, the current `interval` is `[4, 5]` and the inputs `a` and `b` are respectively given as `[0, 2]` and `[-∞, 4]`, then propagation would return `[0, 2]` and `[2, 4]` as `b` must be at least `2` to make the output at least `4`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.evaluate_statistics" class="fn">evaluate_statistics</a>( &self, children: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Computes the output statistics for the expression, given the input statistics.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#parameters-2" class="doc-anchor">§</a>Parameters

- `children` are the statistics for the children (inputs) of this expression.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#returns-2" class="doc-anchor">§</a>Returns

A `Result` containing the output statistics for the expression in case of success, or an error object in case of failure.

Expressions (should) implement this function and utilize the independence assumption, match on children distribution types and compute the output statistics accordingly. The default implementation simply creates an unknown output distribution by combining input ranges. This logic loses distribution information, but is a safe default.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.propagate_statistics" class="fn">propagate_statistics</a>( &self, parent: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>, children: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/statistics/enum.Distribution.html" class="enum" title="enum datafusion::logical_expr::statistics::Distribution">Distribution</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates children statistics using the given parent statistic for this expression.

This is used to propagate statistics down through an expression tree.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#parameters-3" class="doc-anchor">§</a>Parameters

- `parent` is the currently known statistics for this expression.
- `children` are the current statistics for the children of this expression.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#returns-3" class="doc-anchor">§</a>Returns

A `Result` containing a `Vec` of new statistics for the children (in order) in case of success, or an error object in case of failure.

If statistics propagation reveals an infeasibility for any child, returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"). If none of the children statistics change as a result of propagation, may return an empty vector instead of cloning `children`. This is the default (and conservative) return value.

Expressions (should) implement this function and apply Bayes rule to reconcile and update parent/children statistics. This involves utilizing the independence assumption, and matching on distribution types. The default implementation simply creates an unknown distribution if it can narrow the range by propagating ranges. This logic loses distribution information, but is a safe default.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.get_properties" class="fn">get_properties</a>( &self, \_children: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sort_properties/struct.ExprProperties.html" class="struct" title="struct datafusion::logical_expr::sort_properties::ExprProperties">ExprProperties</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Calculates the properties of this [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") based on its children’s properties (i.e. order and range), recursively aggregating the information from its children. In cases where the [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") has no children (e.g., `Literal` or `Column`), these properties should be specified externally, as the function defaults to unknown properties.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.snapshot" class="fn">snapshot</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Take a snapshot of this `PhysicalExpr`, if it is dynamic.

“Dynamic” in this case means containing references to structures that may change during plan execution, such as hash tables.

This method is used to capture the current state of `PhysicalExpr`s that may contain dynamic references to other operators in order to serialize it over the wire or treat it via downcast matching.

You should not call this method directly as it does not handle recursion. Instead use [`snapshot_physical_expr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.snapshot_physical_expr.html "fn datafusion::physical_expr_common::physical_expr::snapshot_physical_expr") to handle recursion and capture the full state of the `PhysicalExpr`.

This is expected to return “simple” expressions that do not have mutable state and are composed of DataFusion’s built-in `PhysicalExpr` implementations. Callers however should *not* assume anything about the returned expressions since callers and implementers may not agree on what “simple” or “built-in” means. In other words, if you need to serialize a `PhysicalExpr` across the wire you should call this method and then try to serialize the result, but you should handle unknown or unexpected `PhysicalExpr` implementations gracefully just as if you had not called this method at all.

In particular, consider:

- A `PhysicalExpr` that references the current state of a `datafusion::physical_plan::TopK` that is involved in a query with `SELECT * FROM t1 ORDER BY a LIMIT 10`. This function may return something like `a >= 12`.
- A `PhysicalExpr` that references the current state of a `datafusion::physical_plan::joins::HashJoinExec` from a query such as `SELECT * FROM t1 JOIN t2 ON t1.a = t2.b`. This function may return something like `t2.b IN (1, 5, 7)`.

A system or function that can only deal with a hardcoded set of `PhysicalExpr` implementations or needs to serialize this state to bytes may not be able to handle these dynamic references. In such cases, we should return a simplified version of the `PhysicalExpr` that does not contain these dynamic references.

Systems that implement remote execution of plans, e.g. serialize a portion of the query plan and send it across the wire to a remote executor may want to call this method after every batch on the source side and broadcast / update the current snapshot to the remote executor.

Note for implementers: this method should *not* handle recursion. Recursion is handled in [`snapshot_physical_expr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.snapshot_physical_expr.html "fn datafusion::physical_expr_common::physical_expr::snapshot_physical_expr").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.snapshot_generation" class="fn">snapshot_generation</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Returns the generation of this `PhysicalExpr` for snapshotting purposes. The generation is an arbitrary u64 that can be used to track changes in the state of the `PhysicalExpr` over time without having to do an exhaustive comparison. This is useful to avoid unnecessary computation or serialization if there are no changes to the expression. In particular, dynamic expressions that may change over time; this allows cheap checks for changes. Static expressions that do not change over time should return 0, as does the default implementation. You should not call this method directly as it does not handle recursion. Instead use [`snapshot_generation`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.snapshot_generation.html "fn datafusion::physical_expr_common::physical_expr::snapshot_generation") to handle recursion and capture the full state of the `PhysicalExpr`.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.is_volatile_node" class="fn">is_volatile_node</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the expression node is volatile, i.e. whether it can return different results when evaluated multiple times with the same input.

Note: unlike [`is_volatile`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/physical_expr/fn.is_volatile.html "fn datafusion::physical_expr_common::physical_expr::is_volatile"), this function does not consider inputs:

- `random()` returns `true`,
- `a + random()` returns `false` (because the operation `+` itself is not volatile.)

The default to this function was set to `false` when it was created to avoid imposing API churn on implementers, but this is not a safe default in general. It is highly recommended that volatile expressions implement this method and return `true`. This default may be removed in the future if it causes problems or we decide to eat the cost of the breaking change and require all implementers to make a choice.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-DynTreeNode-for-dyn+PhysicalExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html" class="trait" title="trait datafusion::common::tree_node::DynTreeNode">DynTreeNode</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.arc_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#tymethod.arc_children" class="fn">arc_children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>

Returns all children of the specified `TreeNode`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.with_new_arc_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.DynTreeNode.html#tymethod.with_new_arc_children" class="fn">with_new_arc_children</a>( &self, arc_self: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, new_children: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs a new node with the specified children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-Hash-for-dyn+PhysicalExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PartialEq-for-dyn+PhysicalExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-Eq-for-dyn+PhysicalExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-AsyncFuncExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/async_scalar_function/struct.AsyncFuncExpr.html" class="struct" title="struct datafusion::physical_expr::async_scalar_function::AsyncFuncExpr">AsyncFuncExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-BinaryExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.BinaryExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::BinaryExpr">BinaryExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-CaseExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CaseExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CaseExpr">CaseExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-CastExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-Column" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.Column.html" class="struct" title="struct datafusion::physical_expr::expressions::Column">Column</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-DynamicFilterPhysicalExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.DynamicFilterPhysicalExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::DynamicFilterPhysicalExpr">DynamicFilterPhysicalExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-InListExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.InListExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::InListExpr">InListExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-IsNotNullExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.IsNotNullExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::IsNotNullExpr">IsNotNullExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-IsNullExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.IsNullExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::IsNullExpr">IsNullExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-LikeExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.LikeExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::LikeExpr">LikeExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-Literal" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.Literal.html" class="struct" title="struct datafusion::physical_expr::expressions::Literal">Literal</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-NegativeExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.NegativeExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::NegativeExpr">NegativeExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-NoOp" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.NoOp.html" class="struct" title="struct datafusion::physical_expr::expressions::NoOp">NoOp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-NotExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.NotExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::NotExpr">NotExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-TryCastExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.TryCastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::TryCastExpr">TryCastExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-UnKnownColumn" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.UnKnownColumn.html" class="struct" title="struct datafusion::physical_expr::expressions::UnKnownColumn">UnKnownColumn</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html#impl-PhysicalExpr-for-ScalarFunctionExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.ScalarFunctionExpr.html" class="struct" title="struct datafusion::physical_expr::ScalarFunctionExpr">ScalarFunctionExpr</a>
