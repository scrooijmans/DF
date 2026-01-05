# Trait UserDefinedLogicalNodeCore Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/extension.rs.html#229-230" class="src">Source</a>

``` rust
pub trait UserDefinedLogicalNodeCore:
    Sized
    + Debug
    + Eq
    + PartialOrd
    + Hash
    + Send
    + Sync
    + 'static {
    // Required methods
    fn name(&self) -> &str;
    fn inputs(&self) -> Vec<&LogicalPlan>;
    fn schema(&self) -> &Arc<DFSchema>;
    fn expressions(&self) -> Vec<Expr>;
    fn fmt_for_explain(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
    fn with_exprs_and_inputs(
        &self,
        exprs: Vec<Expr>,
        inputs: Vec<LogicalPlan>,
    ) -> Result<Self, DataFusionError>;

    // Provided methods
    fn check_invariants(
        &self,
        _check: InvariantLevel,
    ) -> Result<(), DataFusionError> { ... }
    fn prevent_predicate_push_down_columns(&self) -> HashSet<String> { ... }
    fn necessary_children_exprs(
        &self,
        _output_columns: &[usize],
    ) -> Option<Vec<Vec<usize>>> { ... }
    fn supports_limit_pushdown(&self) -> bool { ... }
}
```

Expand description

This trait facilitates implementation of the [`UserDefinedLogicalNode`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode").

See the example in [user_defined_plan.rs](https://github.com/apache/datafusion/blob/main/datafusion/core/tests/user_defined/user_defined_plan.rs) file for an example of how to use this extension API.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return the plan’s name.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#tymethod.inputs" class="fn">inputs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>

Return the logical plan’s inputs.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#tymethod.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\>

Return the output schema of this logical plan node.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#tymethod.expressions" class="fn">expressions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Returns all expressions in the current logical plan node. This should not include expressions of any inputs (aka non-recursively). These expressions are used for optimizer passes and rewrites.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#tymethod.fmt_for_explain" class="fn">fmt_for_explain</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Write a single line, human readable string to `f` for use in explain plan.

For example: `TopK: k=10`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#tymethod.with_exprs_and_inputs" class="fn">with_exprs_and_inputs</a>( &self, exprs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, inputs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html" class="enum" title="enum datafusion::logical_expr::LogicalPlan">LogicalPlan</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new `UserDefinedLogicalNode` with the specified children and expressions. This function is used during optimization when the plan is being rewritten and a new instance of the `UserDefinedLogicalNode` must be created.

Note that exprs and inputs are in the same order as the result of self.inputs and self.exprs.

So, \`self.with_exprs_and_inputs(exprs, ..).expressions() == exprs

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#method.check_invariants" class="fn">check_invariants</a>( &self, \_check: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.InvariantLevel.html" class="enum" title="enum datafusion::logical_expr::InvariantLevel">InvariantLevel</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Perform check of invariants for the extension node.

This is the default implementation for extension nodes.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#method.prevent_predicate_push_down_columns" class="fn">prevent_predicate_push_down_columns</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

A list of output columns (e.g. the names of columns in self.schema()) for which predicates can not be pushed below this node without changing the output.

By default, this returns all columns and thus prevents any predicates from being pushed below this node.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#method.necessary_children_exprs" class="fn">necessary_children_exprs</a>( &self, \_output_columns: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>\>

Returns the necessary input columns for this node required to compute the columns in the output schema

This is used for projection push-down when DataFusion has determined that only a subset of the output columns of this node are needed by its parents. This API is used to tell DataFusion which, if any, of the input columns are no longer needed.

Return `None`, the default, if this information can not be determined. Returns `Some(_)` with the column indices for each child of this node that are needed to compute `output_columns`

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#method.supports_limit_pushdown" class="fn">supports_limit_pushdown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if a limit can be safely pushed down through this `UserDefinedLogicalNode` node.

If this method returns `true`, and the query plan contains a limit at the output of this node, DataFusion will push the limit to the input of this node.

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNodeCore.html#implementors" class="anchor">§</a>
