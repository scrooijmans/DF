# Struct LogicalTableSource Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/builder.rs.html#2044" class="src">Source</a>

``` rust
pub struct LogicalTableSource { /* private fields */ }
```

Expand description

Basic TableSource implementation intended for use in tests and documentation. It is expected that users will provide their own TableSource implementations or use DataFusion’s DefaultTableSource.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#impl-LogicalTableSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html" class="struct" title="struct datafusion::logical_expr::LogicalTableSource">LogicalTableSource</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.new" class="fn">new</a>(table_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html" class="struct" title="struct datafusion::logical_expr::LogicalTableSource">LogicalTableSource</a>

Create a new LogicalTableSource

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.with_constraints" class="fn">with_constraints</a>(self, constraints: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html" class="struct" title="struct datafusion::logical_expr::LogicalTableSource">LogicalTableSource</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#impl-TableSource-for-LogicalTableSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html" class="struct" title="struct datafusion::logical_expr::LogicalTableSource">LogicalTableSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get a reference to the schema for this table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.constraints" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.constraints" class="fn">constraints</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>\>

Get primary key indices, if any

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.supports_filters_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.supports_filters_pushdown" class="fn">supports_filters_pushdown</a>( &self, filters: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html" class="enum" title="enum datafusion::logical_expr::TableProviderFilterPushDown">TableProviderFilterPushDown</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Tests whether the table provider can make use of any or all filter expressions to optimize data retrieval. Only non-volatile expressions are passed to this function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.table_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.table_type" class="fn">table_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>

Get the type of this table for metadata/catalog purposes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.get_logical_plan" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.get_logical_plan" class="fn">get_logical_plan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>\>

Get the Logical plan of this table provider, if available. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.get_logical_plan)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#method.get_column_default" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.get_column_default" class="fn">get_column_default</a>(&self, \_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Get the default value for a column, if available.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html#blanket-implementations" class="anchor">§</a>
