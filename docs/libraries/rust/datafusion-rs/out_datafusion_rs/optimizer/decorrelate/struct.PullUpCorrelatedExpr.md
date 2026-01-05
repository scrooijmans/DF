# Struct PullUpCorrelatedExpr Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/decorrelate.rs.html#47" class="src">Source</a>

``` rust
pub struct PullUpCorrelatedExpr {
    pub join_filters: Vec<Expr>,
    pub correlated_subquery_cols_map: HashMap<LogicalPlan, BTreeSet<Column>>,
    pub in_predicate_opt: Option<Expr>,
    pub exists_sub_query: bool,
    pub can_pull_up: bool,
    pub need_handle_count_bug: bool,
    pub collected_count_expr_map: HashMap<LogicalPlan, HashMap<String, Expr>>,
    pub pull_up_having_expr: Option<Expr>,
    pub pulled_up_scalar_agg: bool,
    /* private fields */
}
```

Expand description

This struct rewrite the sub query plan by pull up the correlated expressions(contains outer reference columns) from the inner subquery’s ‘Filter’. It adds the inner reference columns to the ‘Projection’ or ‘Aggregate’ of the subquery if they are missing, so that they can be evaluated by the parent operator as the join condition.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.join_filters" class="anchor field">§</a>`join_filters: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr"><code>Expr</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.correlated_subquery_cols_map" class="anchor field">§</a>`correlated_subquery_cols_map: `<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap"><code>HashMap</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan"><code>LogicalPlan</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html" class="struct" title="struct alloc::collections::btree::set::BTreeSet"><code>BTreeSet</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column"><code>Column</code></a>`>>`

mapping from the plan to its holding correlated columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.in_predicate_opt" class="anchor field">§</a>`in_predicate_opt: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr"><code>Expr</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.exists_sub_query" class="anchor field">§</a>`exists_sub_query: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Is this an Exists(Not Exists) SubQuery. Defaults to **FALSE**

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.can_pull_up" class="anchor field">§</a>`can_pull_up: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Can the correlated expressions be pulled up. Defaults to **TRUE**

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.need_handle_count_bug" class="anchor field">§</a>`need_handle_count_bug: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Do we need to handle [the count bug](https://github.com/apache/datafusion/issues/10553) during the pull up process.

The “count bug” was described in [Optimization of Nested SQL Queries Revisited](https://dl.acm.org/doi/pdf/10.1145/38714.38723). This bug is not specific to the COUNT function, and it can occur with any aggregate function, such as SUM, AVG, etc. The anomaly arises because aggregates fail to distinguish between an empty set and null values when optimizing a correlated query as a join. Here, we use “the count bug” to refer to all such cases.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.collected_count_expr_map" class="anchor field">§</a>`collected_count_expr_map: `<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap"><code>HashMap</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan"><code>LogicalPlan</code></a>`, `<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap"><code>HashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr"><code>Expr</code></a>`>>`

mapping from the plan to its expressions’ evaluation result on empty batch

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.pull_up_having_expr" class="anchor field">§</a>`pull_up_having_expr: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr"><code>Expr</code></a>`>`

pull up having expr, which must be evaluated after the Join

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#structfield.pulled_up_scalar_agg" class="anchor field">§</a>`pulled_up_scalar_agg: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

whether we have converted a scalar aggregation into a group aggregation. When unnesting lateral joins, we need to produce a left outer join in such cases.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#impl-PullUpCorrelatedExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#method.with_need_handle_count_bug" class="fn">with_need_handle_count_bug</a>( self, need_handle_count_bug: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

Set if we need to handle [the count bug](https://github.com/apache/datafusion/issues/10553) during the pull up process

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#method.with_in_predicate_opt" class="fn">with_in_predicate_opt</a>( self, in_predicate_opt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

Set the in_predicate_opt

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#method.with_exists_sub_query" class="fn">with_exists_sub_query</a>( self, exists_sub_query: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

Set if this is an Exists(Not Exists) SubQuery

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#impl-Debug-for-PullUpCorrelatedExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#impl-Default-for-PullUpCorrelatedExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#impl-TreeNodeRewriter-for-PullUpCorrelatedExpr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html" class="struct" title="struct datafusion::optimizer::decorrelate::PullUpCorrelatedExpr">PullUpCorrelatedExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#associatedtype.Node" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#associatedtype.Node" class="associatedtype">Node</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>

The node type which is rewritable.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#method.f_down" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_down" class="fn">f_down</a>( &mut self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing down the tree before any children are rewritten. Default implementation returns the node as is and continues recursion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#method.f_up" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html#method.f_up" class="fn">f_up</a>( &mut self, plan: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Invoked while traversing up the tree after all children have been rewritten. Default implementation returns the node as is and continues recursion.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate/struct.PullUpCorrelatedExpr.html#blanket-implementations" class="anchor">§</a>
