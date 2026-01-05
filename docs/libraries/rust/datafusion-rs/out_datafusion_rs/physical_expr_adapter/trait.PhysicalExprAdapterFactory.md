# Trait PhysicalExprAdapterFactory Copy item path

<a href="https://docs.rs/datafusion-physical-expr-adapter/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_adapter/schema_rewriter.rs.html#146" class="src">Source</a>

``` rust
pub trait PhysicalExprAdapterFactory:
    Send
    + Sync
    + Debug {
    // Required method
    fn create(
        &self,
        logical_file_schema: Arc<Schema>,
        physical_file_schema: Arc<Schema>,
    ) -> Arc<dyn PhysicalExprAdapter>;
}
```

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html#tymethod.create" class="fn">create</a>( &self, logical_file_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, physical_file_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapter">PhysicalExprAdapter</a>\>

Create a new instance of the physical expression adapter.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html#impl-PhysicalExprAdapterFactory-for-DefaultPhysicalExprAdapterFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapterFactory">PhysicalExprAdapterFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapterFactory">DefaultPhysicalExprAdapterFactory</a>
