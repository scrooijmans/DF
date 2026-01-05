# Struct TypeCoercionRewriter Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/analyzer/type_coercion.rs.html#152" class="src">Source</a>

``` rust
pub struct TypeCoercionRewriter<'a> { /* private fields */ }
```

Expand description

Rewrite expressions to apply type coercion.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#impl-TypeCoercionRewriter%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html" class="struct" title="struct datafusion::optimizer::analyzer::type_coercion::TypeCoercionRewriter">TypeCoercionRewriter</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.new" class="fn">new</a>(schema: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html" class="struct" title="struct datafusion::optimizer::analyzer::type_coercion::TypeCoercionRewriter">TypeCoercionRewriter</a>\<'a\>

Create a new [`TypeCoercionRewriter`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html "struct datafusion::optimizer::analyzer::type_coercion::TypeCoercionRewriter") with a provided schema representing both the inputs and output of the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") node.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_plan" class="fn">coerce_plan</a>( &mut self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce the [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan").

Refer to [`TypeCoercionRewriter::coerce_join`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_join "method datafusion::optimizer::analyzer::type_coercion::TypeCoercionRewriter::coerce_join") and [`TypeCoercionRewriter::coerce_union`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_union "associated function datafusion::optimizer::analyzer::type_coercion::TypeCoercionRewriter::coerce_union") for type-coercion approach.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_join" class="fn">coerce_join</a>( &mut self, join: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Join.html" class="struct" title="struct datafusion::logical_expr::Join">Join</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce join equality expressions and join filter

Joins must be treated specially as their equality expressions are stored as a parallel list of left and right expressions, rather than a single equality expression

For example, on_exprs like `t1.a = t2.b AND t1.x = t2.y` will be stored as a list of `(t1.a, t2.b), (t1.x, t2.y)`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_union" class="fn">coerce_union</a>(union_plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Union.html" class="struct" title="struct datafusion::logical_expr::Union">Union</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Coerce the union’s inputs to a common schema compatible with all inputs. This occurs after wildcard expansion and the coercion of the input expressions.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#impl-TreeNodeRewriter-for-TypeCoercionRewriter%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html" class="struct" title="struct datafusion::optimizer::analyzer::type_coercion::TypeCoercionRewriter">TypeCoercionRewriter</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#associatedtype.Node" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The node type which is rewritable.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.f_up" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_up" class="fn">f_up</a>(&mut self, expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing up the tree after all children have been rewritten. Default implementation returns the node as is and continues recursion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.f_down" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_down" class="fn">f_down</a>( &mut self, node: Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype" title="type datafusion::common::tree_node::TreeNodeRewriter::Node">Node</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing down the tree before any children are rewritten. Default implementation returns the node as is and continues recursion.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#blanket-implementations" class="anchor">§</a>
