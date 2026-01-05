# Struct AccumulatorArgs Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/accumulator.rs.html#29" class="src">Source</a>

``` rust
pub struct AccumulatorArgs<'a> {
    pub return_field: Arc<Field>,
    pub schema: &'a Schema,
    pub ignore_nulls: bool,
    pub order_bys: &'a [PhysicalSortExpr],
    pub is_reversed: bool,
    pub name: &'a str,
    pub is_distinct: bool,
    pub exprs: &'a [Arc<dyn PhysicalExpr>],
}
```

Expand description

[`AccumulatorArgs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html "struct datafusion::logical_expr::function::AccumulatorArgs") contains information about how an aggregate function was called, including the types of its arguments and any optional ordering expressions.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#structfield.return_field" class="anchor field">§</a>`return_field: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field"><code>Field</code></a>`>`

The return field of the aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#structfield.schema" class="anchor field">§</a>`schema: &'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema"><code>Schema</code></a>

The schema of the input arguments

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#structfield.ignore_nulls" class="anchor field">§</a>`ignore_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to ignore nulls.

SQL allows the user to specify `IGNORE NULLS`, for example:

``` sql
SELECT FIRST_VALUE(column1) IGNORE NULLS FROM t;
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#structfield.order_bys" class="anchor field">§</a>`order_bys: &'a [`<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr"><code>PhysicalSortExpr</code></a>`]`

The expressions in the `ORDER BY` clause passed to this aggregator.

SQL allows the user to specify the ordering of arguments to the aggregate using an `ORDER BY`. For example:

``` sql
SELECT FIRST_VALUE(column1 ORDER BY column2) FROM t;
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#structfield.is_reversed" class="anchor field">§</a>`is_reversed: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether the aggregation is running in reverse order

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#structfield.name" class="anchor field">§</a>`name: &'a `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

The name of the aggregate expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#structfield.is_distinct" class="anchor field">§</a>`is_distinct: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether the aggregate function is distinct.

``` sql
SELECT COUNT(DISTINCT column1) FROM t;
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#structfield.exprs" class="anchor field">§</a>`exprs: &'a [`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr"><code>PhysicalExpr</code></a>`>]`

The physical expression of arguments the aggregate function takes.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#impl-AccumulatorArgs%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'\_\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#method.return_type" class="fn">return_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns the return type of the aggregate function.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#impl-Clone-for-AccumulatorArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#impl-Debug-for-AccumulatorArgs%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html#blanket-implementations" class="anchor">§</a>
