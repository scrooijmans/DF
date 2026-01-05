# Module expr_rewriter Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#49" class="src">Source</a>

Expand description

Expression rewriter

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/struct.NamePreserver.html" class="struct" title="struct datafusion::logical_expr::expr_rewriter::NamePreserver">NamePreserver</a>  
Handles ensuring the name of rewritten expressions is not changed.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/enum.SavedName.html" class="enum" title="enum datafusion::logical_expr::expr_rewriter::SavedName">SavedName</a>  
If the qualified name of an expression is remembered, it will be preserved when rewriting the expression

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/trait.FunctionRewrite.html" class="trait" title="trait datafusion::logical_expr::expr_rewriter::FunctionRewrite">FunctionRewrite</a>  
Trait for rewriting [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")s into function calls.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.coerce_plan_expr_for_schema.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::coerce_plan_expr_for_schema">coerce_plan_expr_for_schema</a>  
Returns plan with expressions coerced to types compatible with schema types

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.create_col_from_scalar_expr.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::create_col_from_scalar_expr">create_col_from_scalar_expr</a>  
Create a Column from the Scalar Expr

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.normalize_col.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::normalize_col">normalize_col</a>  
Recursively call `LogicalPlanBuilder::normalize` on all [`Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html "struct datafusion::prelude::Column") expressions in the `expr` expression tree.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.normalize_col_with_schemas_and_ambiguity_check.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::normalize_col_with_schemas_and_ambiguity_check">normalize_col_with_schemas_and_ambiguity_check</a>  
See [`Column::normalize_with_schemas_and_ambiguity_check`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html#method.normalize_with_schemas_and_ambiguity_check "method datafusion::prelude::Column::normalize_with_schemas_and_ambiguity_check") for usage

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.normalize_cols.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::normalize_cols">normalize_cols</a>  
Recursively normalize all [`Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html "struct datafusion::prelude::Column") expressions in a list of expression trees

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.normalize_sorts.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::normalize_sorts">normalize_sorts</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.replace_col.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::replace_col">replace_col</a>  
Recursively replace all [`Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html "struct datafusion::prelude::Column") expressions in a given expression tree with `Column` expressions provided by the hash map argument.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.rewrite_sort_cols_by_aggs.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::rewrite_sort_cols_by_aggs">rewrite_sort_cols_by_aggs</a>  
Rewrite sort on aggregate expressions to sort on the column of aggregate output For example, `max(x)` is written to `col("max(x)")`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.strip_outer_reference.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::strip_outer_reference">strip_outer_reference</a>  
Recursively remove all the \[‘OuterReferenceColumn’\] and return the inside Column in the expression tree.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.unalias.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::unalias">unalias</a>  
Recursively un-alias an expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.unnormalize_col.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::unnormalize_col">unnormalize_col</a>  
Recursively ‘unnormalize’ (remove all qualifiers) from an expression tree.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_rewriter/fn.unnormalize_cols.html" class="fn" title="fn datafusion::logical_expr::expr_rewriter::unnormalize_cols">unnormalize_cols</a>  
Recursively un-normalize all [`Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html "struct datafusion::prelude::Column") expressions in a list of expression trees
