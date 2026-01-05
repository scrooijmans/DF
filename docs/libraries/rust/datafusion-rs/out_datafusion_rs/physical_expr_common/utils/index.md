# Module utils Copy item path

<a href="https://docs.rs/datafusion-physical-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr_common/lib.rs.html#38" class="src">Source</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/utils/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/utils/fn.scatter.html" class="fn" title="fn datafusion::physical_expr_common::utils::scatter">scatter</a>  
Scatter `truthy` array by boolean mask. When the mask evaluates `true`, next values of `truthy` are taken, when the mask evaluates `false` values null values are filled.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/utils/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_common/utils/type.ExprPropertiesNode.html" class="type" title="type datafusion::physical_expr_common::utils::ExprPropertiesNode">ExprPropertiesNode</a>  
Represents a [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") node with associated properties (order and range) in a context where properties are tracked.
