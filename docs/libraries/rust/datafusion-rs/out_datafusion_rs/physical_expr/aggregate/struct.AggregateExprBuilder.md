# Struct AggregateExprBuilder Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/aggregate.rs.html#61" class="src">Source</a>

``` rust
pub struct AggregateExprBuilder { /* private fields */ }
```

Expand description

Builder for physical [`AggregateFunctionExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateFunctionExpr.html "struct datafusion::physical_expr::aggregate::AggregateFunctionExpr")

`AggregateFunctionExpr` contains the information necessary to call an aggregate expression.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#impl-AggregateExprBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.new" class="fn">new</a>( fun: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html" class="struct" title="struct datafusion::logical_expr::AggregateUDF">AggregateUDF</a>\>, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateFunctionExpr.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateFunctionExpr">AggregateFunctionExpr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Constructs an `AggregateFunctionExpr` from the builder

Note that an [`Self::alias`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.alias "method datafusion::physical_expr::aggregate::AggregateExprBuilder::alias") must be provided before calling this method.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#example-create-an-aggregateudf" class="doc-anchor">§</a>Example: Create an [`AggregateUDF`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.AggregateUDF.html "struct datafusion::logical_expr::AggregateUDF")

In the following example, [`AggregateFunctionExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateFunctionExpr.html "struct datafusion::physical_expr::aggregate::AggregateFunctionExpr") will be built using [`AggregateExprBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html "struct datafusion::physical_expr::aggregate::AggregateExprBuilder") which provides a build function. Full example could be accessed from the source file.

``` rust
fn build_aggregate_expr() -> Result<()> {
    let args = vec![Arc::new(Column::new("a", 0)) as Arc<dyn PhysicalExpr>];
    let order_by = vec![PhysicalSortExpr {
        expr: Arc::new(Column::new("x", 1)) as Arc<dyn PhysicalExpr>,
        options: Default::default(),
    }];

    let first_value = AggregateUDF::from(FirstValueUdf::new());

    let aggregate_expr = AggregateExprBuilder::new(
        Arc::new(first_value),
        args
    )
    .order_by(order_by)
    .alias("first_a_by_x")
    .ignore_nulls()
    .build()?;

    Ok(())
}
```

This creates a physical expression equivalent to SQL: `first_value(a ORDER BY x) IGNORE NULLS AS first_a_by_x`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.alias" class="fn">alias</a>(self, alias: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.human_display" class="fn">human_display</a>(self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.schema" class="fn">schema</a>(self, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.order_by" class="fn">order_by</a>(self, order_bys: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::PhysicalSortExpr">PhysicalSortExpr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.reversed" class="fn">reversed</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.with_reversed" class="fn">with_reversed</a>(self, is_reversed: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.distinct" class="fn">distinct</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.with_distinct" class="fn">with_distinct</a>(self, is_distinct: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.ignore_nulls" class="fn">ignore_nulls</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.with_ignore_nulls" class="fn">with_ignore_nulls</a>(self, ignore_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#impl-Clone-for-AggregateExprBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#impl-Debug-for-AggregateExprBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html" class="struct" title="struct datafusion::physical_expr::aggregate::AggregateExprBuilder">AggregateExprBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/aggregate/struct.AggregateExprBuilder.html#blanket-implementations" class="anchor">§</a>
