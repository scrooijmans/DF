# Trait PhysicalExprAdapter Copy item path

<a href="https://docs.rs/datafusion-physical-expr-adapter/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_adapter/schema_rewriter.rs.html#124" class="src">Source</a>

``` rust
pub trait PhysicalExprAdapter:
    Send
    + Sync
    + Debug {
    // Required methods
    fn rewrite(
        &self,
        expr: Arc<dyn PhysicalExpr>,
    ) -> Result<Arc<dyn PhysicalExpr>, DataFusionError>;
    fn with_partition_values(
        &self,
        partition_values: Vec<(Arc<Field>, ScalarValue)>,
    ) -> Arc<dyn PhysicalExprAdapter>;
}
```

Expand description

Trait for adapting physical expressions to match a target schema.

This is used in file scans to rewrite expressions so that they can be evaluated against the physical schema of the file being scanned. It allows for handling differences between logical and physical schemas, such as type mismatches or missing columns.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#overview" class="doc-anchor">§</a>Overview

The `PhysicalExprAdapter` allows rewriting physical expressions to match different schemas, including:

- **Type casting**: When logical and physical schemas have different types, expressions are automatically wrapped with cast operations. For example, `lit(ScalarValue::Int32(123)) = int64_column` gets rewritten to `lit(ScalarValue::Int32(123)) = cast(int64_column, 'Int32')`. Note that this does not attempt to simplify such expressions - that is done by shared simplifiers.

- **Missing columns**: When a column exists in the logical schema but not in the physical schema, references to it are replaced with null literals.

- **Struct field access**: Expressions like `struct_column.field_that_is_missing_in_schema` are rewritten to `null` when the field doesn’t exist in the physical schema.

- **Partition columns**: Partition column references can be replaced with their literal values when scanning specific partitions.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#custom-implementations" class="doc-anchor">§</a>Custom Implementations

You can create a custom implementation of this trait to handle specific rewriting logic. For example, to fill in missing columns with default values instead of nulls:

``` rust
use datafusion_physical_expr_adapter::{PhysicalExprAdapter, PhysicalExprAdapterFactory};
use arrow::datatypes::{Schema, Field, DataType, FieldRef, SchemaRef};
use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
use datafusion_common::{Result, ScalarValue, tree_node::{Transformed, TransformedResult, TreeNode}};
use datafusion_physical_expr::expressions::{self, Column};
use std::sync::Arc;

#[derive(Debug)]
pub struct CustomPhysicalExprAdapter {
    logical_file_schema: SchemaRef,
    physical_file_schema: SchemaRef,
}

impl PhysicalExprAdapter for CustomPhysicalExprAdapter {
    fn rewrite(&self, expr: Arc<dyn PhysicalExpr>) -> Result<Arc<dyn PhysicalExpr>> {
        expr.transform(|expr| {
            if let Some(column) = expr.as_any().downcast_ref::<Column>() {
                // Check if the column exists in the physical schema
                if self.physical_file_schema.index_of(column.name()).is_err() {
                    // If the column is missing, fill it with a default value instead of null
                    // The default value could be stored in the table schema's column metadata for example.
                    let default_value = ScalarValue::Int32(Some(0));
                    return Ok(Transformed::yes(expressions::lit(default_value)));
                }
            }
            // If the column exists, return it as is
            Ok(Transformed::no(expr))
        }).data()
    }

    fn with_partition_values(
        &self,
        partition_values: Vec<(FieldRef, ScalarValue)>,
    ) -> Arc<dyn PhysicalExprAdapter> {
        // For simplicity, this example ignores partition values
        Arc::new(CustomPhysicalExprAdapter {
            logical_file_schema: self.logical_file_schema.clone(),
            physical_file_schema: self.physical_file_schema.clone(),
        })
    }
}

#[derive(Debug)]
pub struct CustomPhysicalExprAdapterFactory;

impl PhysicalExprAdapterFactory for CustomPhysicalExprAdapterFactory {
    fn create(
        &self,
        logical_file_schema: SchemaRef,
        physical_file_schema: SchemaRef,
    ) -> Arc<dyn PhysicalExprAdapter> {
        Arc::new(CustomPhysicalExprAdapter {
            logical_file_schema,
            physical_file_schema,
        })
    }
}
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#tymethod.rewrite" class="fn">rewrite</a>( &self, expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite a physical expression to match the target schema.

This method should return a transformed expression that matches the target schema.

Arguments:

- `expr`: The physical expression to rewrite.
- `logical_file_schema`: The logical schema of the table being queried, excluding any partition columns.
- `physical_file_schema`: The physical schema of the file being scanned.
- `partition_values`: Optional partition values to use for rewriting partition column references. These are handled as if they were columns appended onto the logical file schema.

Returns:

- `Arc<dyn PhysicalExpr>`: The rewritten physical expression that can be evaluated against the physical schema.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#tymethod.with_partition_values" class="fn">with_partition_values</a>( &self, partition_values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapter">PhysicalExprAdapter</a>\>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#impl-PhysicalExprAdapter-for-DefaultPhysicalExprAdapter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapter">PhysicalExprAdapter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapter">DefaultPhysicalExprAdapter</a>
