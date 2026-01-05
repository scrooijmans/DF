# Struct DefaultTableSource Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/default_table_source.rs.html#36" class="src">Source</a>

``` rust
pub struct DefaultTableSource {
    pub table_provider: Arc<dyn TableProvider>,
}
```

Expand description

Implements [`TableSource`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html "trait datafusion::logical_expr::TableSource") for a [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")

This structure adapts a [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider") (a physical plan trait) to the [`TableSource`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html "trait datafusion::logical_expr::TableSource") (logical plan trait).

It is used so logical plans in the `datafusion_expr` crate do not have a direct dependency on physical plans, such as [`TableProvider`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")s.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#structfield.table_provider" class="anchor field">§</a>`table_provider: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider"><code>TableProvider</code></a>`>`

table provider

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#impl-DefaultTableSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html" class="struct" title="struct datafusion::datasource::DefaultTableSource">DefaultTableSource</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#method.new" class="fn">new</a>(table_provider: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html" class="struct" title="struct datafusion::datasource::DefaultTableSource">DefaultTableSource</a>

Create a new DefaultTableSource to wrap a TableProvider

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#impl-TableSource-for-DefaultTableSource" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html" class="trait" title="trait datafusion::logical_expr::TableSource">TableSource</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html" class="struct" title="struct datafusion::datasource::DefaultTableSource">DefaultTableSource</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the table source as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get a reference to the schema for this table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#method.constraints" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.constraints" class="fn">constraints</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>\>

Get a reference to applicable constraints, if any exists.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#method.table_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.table_type" class="fn">table_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/enum.TableType.html" class="enum" title="enum datafusion::datasource::TableType">TableType</a>

Get the type of this table for metadata/catalog purposes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#method.supports_filters_pushdown" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.supports_filters_pushdown" class="fn">supports_filters_pushdown</a>( &self, filter: &\[&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TableProviderFilterPushDown.html" class="enum" title="enum datafusion::logical_expr::TableProviderFilterPushDown">TableProviderFilterPushDown</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Tests whether the table provider can make use of any or all filter expressions to optimize data retrieval.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#method.get_logical_plan" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.get_logical_plan" class="fn">get_logical_plan</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>\>

Get the Logical plan of this table provider, if available. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.get_logical_plan)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#method.get_column_default" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.TableSource.html#method.get_column_default" class="fn">get_column_default</a>(&self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Get the default value for a column, if available.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/struct.DefaultTableSource.html#blanket-implementations" class="anchor">§</a>
