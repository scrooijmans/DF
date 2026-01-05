# Struct DefaultPhysicalExprAdapter Copy item path

<a href="https://docs.rs/datafusion-physical-expr-adapter/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_adapter/schema_rewriter.rs.html#193" class="src">Source</a>

``` rust
pub struct DefaultPhysicalExprAdapter { /* private fields */ }
```

Expand description

Default implementation for rewriting physical expressions to match different schemas.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#example" class="doc-anchor">§</a>Example

``` rust
use datafusion_physical_expr_adapter::{DefaultPhysicalExprAdapterFactory, PhysicalExprAdapterFactory};
use arrow::datatypes::Schema;
use std::sync::Arc;

let factory = DefaultPhysicalExprAdapterFactory;
let adapter = factory.create(Arc::new(logical_file_schema.clone()), Arc::new(physical_file_schema.clone()));
let adapted_predicate = adapter.rewrite(predicate)?;
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#impl-DefaultPhysicalExprAdapter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapter">DefaultPhysicalExprAdapter</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#method.new" class="fn">new</a>( logical_file_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, physical_file_schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapter">DefaultPhysicalExprAdapter</a>

Create a new instance of the default physical expression adapter.

This adapter rewrites expressions to match the physical schema of the file being scanned, handling type mismatches and missing columns by filling them with default values.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#impl-Clone-for-DefaultPhysicalExprAdapter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapter">DefaultPhysicalExprAdapter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapter">DefaultPhysicalExprAdapter</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#impl-Debug-for-DefaultPhysicalExprAdapter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapter">DefaultPhysicalExprAdapter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#impl-PhysicalExprAdapter-for-DefaultPhysicalExprAdapter" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapter">PhysicalExprAdapter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html" class="struct" title="struct datafusion::physical_expr_adapter::DefaultPhysicalExprAdapter">DefaultPhysicalExprAdapter</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#method.rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#tymethod.rewrite" class="fn">rewrite</a>( &self, expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite a physical expression to match the target schema. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#tymethod.rewrite)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#method.with_partition_values" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html#tymethod.with_partition_values" class="fn">with_partition_values</a>( &self, partition_values: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapter.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapter">PhysicalExprAdapter</a>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/struct.DefaultPhysicalExprAdapter.html#blanket-implementations" class="anchor">§</a>
