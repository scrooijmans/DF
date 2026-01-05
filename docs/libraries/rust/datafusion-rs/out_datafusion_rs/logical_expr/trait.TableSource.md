# Trait TableSource Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/table_source.rs.html#94" class="src">Source</a>

``` rust
pub trait TableSource: Sync + Send {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn schema(&self) -> Arc<Schema>;

    // Provided methods
    fn constraints(&self) -> Option<&Constraints> { ... }
    fn table_type(&self) -> TableType { ... }
    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> Result<Vec<TableProviderFilterPushDown>, DataFusionError> { ... }
    fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>> { ... }
    fn get_column_default(&self, _column: &str) -> Option<&Expr> { ... }
}
```

Expand description

Planning time information about a table.

This trait is used during logical query planning and optimizations, and provides a subset of the [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html) trait, such as schema information and filter push-down capabilities. The [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html) trait provides additional information needed for physical query execution, such as the ability to perform a scan or insert data.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#see-also" class="doc-anchor">§</a>See Also:

[`DefaultTableSource`](https://docs.rs/datafusion/latest/datafusion/datasource/default_table_source/struct.DefaultTableSource.html) to go from [`TableProvider`](https://docs.rs/datafusion/latest/datafusion/datasource/trait.TableProvider.html), to `TableSource`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#rationale" class="doc-anchor">§</a>Rationale

The reason for having two separate traits is to avoid having the logical plan code be dependent on the DataFusion execution engine. Some projects use DataFusion’s logical plans and have their own execution engine.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get a reference to the schema for this table

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.constraints" class="fn">constraints</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>\>

Get primary key indices, if any

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.table_type" class="fn">table_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>

Get the type of this table for metadata/catalog purposes.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.supports_filters_pushdown" class="fn">supports_filters_pushdown</a>( &self, filters: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html" class="enum" title="enum datafusion::logical_expr::TableProviderFilterPushDown">TableProviderFilterPushDown</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Tests whether the table provider can make use of any or all filter expressions to optimize data retrieval. Only non-volatile expressions are passed to this function.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.get_logical_plan" class="fn">get_logical_plan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>\>

Get the Logical plan of this table provider, if available.

For example, a view may have a logical plan, but a CSV file does not.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.get_column_default" class="fn">get_column_default</a>(&self, \_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Get the default value for a column, if available.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#impl-TableSource-for-DefaultTableSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html" class="struct" title="struct datafusion::datasource::DefaultTableSource">DefaultTableSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#impl-TableSource-for-LogicalTableSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.LogicalTableSource.html" class="struct" title="struct datafusion::logical_expr::LogicalTableSource">LogicalTableSource</a>
