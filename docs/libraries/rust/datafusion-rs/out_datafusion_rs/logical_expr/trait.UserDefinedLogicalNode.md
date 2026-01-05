# Trait UserDefinedLogicalNode Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/extension.rs.html#32" class="src">Source</a>

``` rust
pub trait UserDefinedLogicalNode:
    Debug
    + Send
    + Sync {
Show 14 methods    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn name(&self) -> &str;
    fn inputs(&self) -> Vec<&LogicalPlan>;
    fn schema(&self) -> &Arc<DFSchema>;
    fn check_invariants(
        &self,
        check: InvariantLevel,
    ) -> Result<(), DataFusionError>;
    fn expressions(&self) -> Vec<Expr>;
    fn fmt_for_explain(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
    fn with_exprs_and_inputs(
        &self,
        exprs: Vec<Expr>,
        inputs: Vec<LogicalPlan>,
    ) -> Result<Arc<dyn UserDefinedLogicalNode>, DataFusionError>;
    fn dyn_hash(&self, state: &mut dyn Hasher);
    fn dyn_eq(&self, other: &dyn UserDefinedLogicalNode) -> bool;
    fn dyn_ord(&self, other: &dyn UserDefinedLogicalNode) -> Option<Ordering>;

    // Provided methods
    fn prevent_predicate_push_down_columns(&self) -> HashSet<String> { ... }
    fn necessary_children_exprs(
        &self,
        _output_columns: &[usize],
    ) -> Option<Vec<Vec<usize>>> { ... }
    fn supports_limit_pushdown(&self) -> bool { ... }
}
```

Expand description

This defines the interface for [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan") nodes that can be used to extend DataFusion with custom relational operators.

The [`UserDefinedLogicalNodeCore`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html "trait datafusion::logical_expr::UserDefinedLogicalNodeCore") trait is *the recommended way to implement* this trait and avoids having implementing some required boiler plate code.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Return a reference to self as Any, to support dynamic downcasting

Typically this will look like:

``` rust

  // canonical boiler plate
  fn as_any(&self) -> &dyn Any {
     self
  }
```

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return the plan’s name.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.inputs" class="fn">inputs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>

Return the logical plan’s inputs.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>

Return the output schema of this logical plan node.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.check_invariants" class="fn">check_invariants</a>(&self, check: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.InvariantLevel.html" class="enum" title="enum datafusion::logical_expr::InvariantLevel">InvariantLevel</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Perform check of invariants for the extension node.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.expressions" class="fn">expressions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Returns all expressions in the current logical plan node. This should not include expressions of any inputs (aka non-recursively).

These expressions are used for optimizer passes and rewrites. See [`LogicalPlan::expressions`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#method.expressions "method datafusion::logical_expr::LogicalPlan::expressions") for more details.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.fmt_for_explain" class="fn">fmt_for_explain</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Write a single line, human readable string to `f` for use in explain plan.

For example: `TopK: k=10`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.with_exprs_and_inputs" class="fn">with_exprs_and_inputs</a>( &self, exprs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, inputs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new `UserDefinedLogicalNode` with the specified children and expressions. This function is used during optimization when the plan is being rewritten and a new instance of the `UserDefinedLogicalNode` must be created.

Note that exprs and inputs are in the same order as the result of self.inputs and self.exprs.

So, \`self.with_exprs_and_inputs(exprs, ..).expressions() == exprs

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.dyn_hash" class="fn">dyn_hash</a>(&self, state: &mut dyn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>)

Update the hash `state` with this node requirements from [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash").

Note: consider using [`UserDefinedLogicalNodeCore`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html "trait datafusion::logical_expr::UserDefinedLogicalNodeCore") instead of [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode") directly.

This method is required to support hashing [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s. To implement it, typically the type implementing [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode") typically implements [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") and then the following boiler plate is used:

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#example" class="doc-anchor">§</a>Example:

``` rust
// User defined node that derives Hash
#[derive(Hash, Debug, PartialEq, Eq)]
struct MyNode {
  val: u64
}

// impl UserDefinedLogicalNode {
// ...
  // Boiler plate to call the derived Hash impl
  fn dyn_hash(&self, state: &mut dyn std::hash::Hasher) {
    use std::hash::Hash;
    let mut s = state;
    self.hash(&mut s);
  }
// }
```

Note: [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode") is not constrained by [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") directly because it must remain object safe.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.dyn_eq" class="fn">dyn_eq</a>(&self, other: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Compare `other`, respecting requirements from [std::cmp::Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq").

Note: consider using [`UserDefinedLogicalNodeCore`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html "trait datafusion::logical_expr::UserDefinedLogicalNodeCore") instead of [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode") directly.

When `other` has an another type than `self`, then the values are *not* equal.

This method is required to support Eq on [`LogicalPlan`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html "enum datafusion::logical_expr::LogicalPlan")s. To implement it, typically the type implementing [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode") typically implements [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") and then the following boiler plate is used:

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#example-1" class="doc-anchor">§</a>Example:

``` rust
// User defined node that derives Eq
#[derive(Hash, Debug, PartialEq, Eq)]
struct MyNode {
  val: u64
}

// impl UserDefinedLogicalNode {
// ...
  // Boiler plate to call the derived Eq impl
  fn dyn_eq(&self, other: &dyn UserDefinedLogicalNode) -> bool {
    match other.as_any().downcast_ref::<Self>() {
      Some(o) => self == o,
      None => false,
    }
  }
// }
```

Note: [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode") is not constrained by [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") directly because it must remain object safe.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#tymethod.dyn_ord" class="fn">dyn_ord</a>(&self, other: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.prevent_predicate_push_down_columns" class="fn">prevent_predicate_push_down_columns</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

A list of output columns (e.g. the names of columns in self.schema()) for which predicates can not be pushed below this node without changing the output.

By default, this returns all columns and thus prevents any predicates from being pushed below this node.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.necessary_children_exprs" class="fn">necessary_children_exprs</a>( &self, \_output_columns: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>\>

Returns the necessary input columns for this node required to compute the columns in the output schema

This is used for projection push-down when DataFusion has determined that only a subset of the output columns of this node are needed by its parents. This API is used to tell DataFusion which, if any, of the input columns are no longer needed.

Return `None`, the default, if this information can not be determined. Returns `Some(_)` with the column indices for each child of this node that are needed to compute `output_columns`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.supports_limit_pushdown" class="fn">supports_limit_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if a limit can be safely pushed down through this `UserDefinedLogicalNode` node.

If this method returns `true`, and the query plan contains a limit at the output of this node, DataFusion will push the limit to the input of this node.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#impl-Hash-for-dyn+UserDefinedLogicalNode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#impl-PartialEq-for-dyn+UserDefinedLogicalNode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a> + 'static)) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#impl-PartialOrd-for-dyn+UserDefinedLogicalNode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>( &self, other: &(dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a> + 'static), ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#impl-Eq-for-dyn+UserDefinedLogicalNode" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html#impl-UserDefinedLogicalNode-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNode">UserDefinedLogicalNode</a> for T

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html" class="trait" title="trait datafusion::logical_expr::UserDefinedLogicalNodeCore">UserDefinedLogicalNodeCore</a>,

Automatically derive UserDefinedLogicalNode to `UserDefinedLogicalNode` to avoid boiler plate for implementing `as_any`, `Hash` and `PartialEq`
