# Struct DefaultPhysicalExprAdapterFactory Copy item path

<a href="https://docs.rs/datafusion-physical-expr-adapter/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_adapter/schema_rewriter.rs.html#156" class="src">Source</a>

``` rust
pub struct DefaultPhysicalExprAdapterFactory;
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#impl-Clone-for-DefaultPhysicalExprAdapterFactory" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapterFactory">DefaultPhysicalExprAdapterFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapterFactory">DefaultPhysicalExprAdapterFactory</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#impl-Debug-for-DefaultPhysicalExprAdapterFactory" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapterFactory">DefaultPhysicalExprAdapterFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#impl-PhysicalExprAdapterFactory-for-DefaultPhysicalExprAdapterFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapterFactory">PhysicalExprAdapterFactory</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapterFactory">DefaultPhysicalExprAdapterFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#method.create" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html#tymethod.create" class="fn">create</a>( &self, logical_file_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, physical_file_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapter">PhysicalExprAdapter</a>\>

Create a new instance of the physical expression adapter.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapterFactory.html#blanket-implementations" class="anchor">§</a>
