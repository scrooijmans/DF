# Struct PhysicalExprSimplifier Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/simplifier/mod.rs.html#36" class="src">Source</a>

``` rust
pub struct PhysicalExprSimplifier<'a> { /* private fields */ }
```

Expand description

Simplifies physical expressions by applying various optimizations

This can be useful after adapting expressions from a table schema to a file schema. For example, casts added to match the types may potentially be unwrapped.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#impl-PhysicalExprSimplifier%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html" class="struct" title="struct datafusion::physical_expr::PhysicalExprSimplifier">PhysicalExprSimplifier</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#method.new" class="fn">new</a>(schema: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html" class="struct" title="struct datafusion::physical_expr::PhysicalExprSimplifier">PhysicalExprSimplifier</a>\<'a\>

Create a new physical expression simplifier

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#method.simplify" class="fn">simplify</a>( &mut self, expr: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Simplify a physical expression

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#impl-TreeNodeRewriter-for-PhysicalExprSimplifier%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html" class="struct" title="struct datafusion::physical_expr::PhysicalExprSimplifier">PhysicalExprSimplifier</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#associatedtype.Node" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html" class="trait" title="trait datafusion::physical_expr::PhysicalExpr">PhysicalExpr</a>\>

The node type which is rewritable.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#method.f_up" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_up" class="fn">f_up</a>( &mut self, node: \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html" class="struct" title="struct datafusion::physical_expr::PhysicalExprSimplifier">PhysicalExprSimplifier</a>\<'a\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html" class="struct" title="struct datafusion::physical_expr::PhysicalExprSimplifier">PhysicalExprSimplifier</a>\<'a\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing up the tree after all children have been rewritten. Default implementation returns the node as is and continues recursion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#method.f_down" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_down" class="fn">f_down</a>( &mut self, node: Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing down the tree before any children are rewritten. Default implementation returns the node as is and continues recursion.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.PhysicalExprSimplifier.html#blanket-implementations" class="anchor">§</a>
