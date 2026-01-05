# Struct IndentVisitor Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/display.rs.html#42" class="src">Source</a>

``` rust
pub struct IndentVisitor<'a, 'b> { /* private fields */ }
```

Expand description

Formats plans with a single line per node. For example:

Projection: id Filter: state Eq Utf8("CO")  
CsvScan: employee.csv projection=Some(\[0, 3\])“;

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#impl-IndentVisitor%3C&#39;a,+&#39;b%3E" class="anchor">§</a>

### impl\<'a, 'b\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html" class="struct" title="struct datafusion::logical_expr::logical_plan::display::IndentVisitor">IndentVisitor</a>\<'a, 'b\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#method.new" class="fn">new</a>(f: &'a mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'b\>, with_schema: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html" class="struct" title="struct datafusion::logical_expr::logical_plan::display::IndentVisitor">IndentVisitor</a>\<'a, 'b\>

Create a visitor that will write a formatted LogicalPlan to f. If `with_schema` is true, includes schema information on each line.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#impl-TreeNodeVisitor%3C&#39;n%3E-for-IndentVisitor%3C&#39;_,+&#39;_%3E" class="anchor">§</a>

### impl\<'n\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeVisitor">TreeNodeVisitor</a>\<'n\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html" class="struct" title="struct datafusion::logical_expr::logical_plan::display::IndentVisitor">IndentVisitor</a>\<'\_, '\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#associatedtype.Node" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

The node type which is visitable.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#method.f_down" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_down" class="fn">f_down</a>( &mut self, plan: &'n <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing down the tree, before any children are visited. Default implementation continues the recursion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#method.f_up" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html#method.f_up" class="fn">f_up</a>( &mut self, \_plan: &'n <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing up the tree after children are visited. Default implementation continues the recursion.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/logical_plan/display/struct.IndentVisitor.html#blanket-implementations" class="anchor">§</a>
