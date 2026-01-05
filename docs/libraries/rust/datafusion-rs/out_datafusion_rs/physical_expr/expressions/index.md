# Module expressions Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/lib.rs.html#35" class="src">Source</a>

Expand description

Defines physical expressions that can evaluated at runtime during query execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.BinaryExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::BinaryExpr">BinaryExpr</a>

Binary expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CaseExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CaseExpr">CaseExpr</a>

The CASE expression is similar to a series of nested if/else and there are two forms that can be used. The first form consists of a series of boolean “when” expressions with corresponding “then” expressions, and an optional “else” expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.CastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::CastExpr">CastExpr</a>

CAST expression casts an expression to a specific data type and returns a runtime error on invalid cast

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.Column.html" class="struct" title="struct datafusion::physical_expr::expressions::Column">Column</a>

Represents the column at a given index in a RecordBatch

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.DynamicFilterPhysicalExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::DynamicFilterPhysicalExpr">DynamicFilterPhysicalExpr</a>

A dynamic [`PhysicalExpr`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/trait.PhysicalExpr.html "trait datafusion::physical_expr::PhysicalExpr") that can be updated by anyone with a reference to it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.InListExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::InListExpr">InListExpr</a>

InList

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.IsNotNullExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::IsNotNullExpr">IsNotNullExpr</a>

IS NOT NULL expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.IsNullExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::IsNullExpr">IsNullExpr</a>

IS NULL expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.LikeExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::LikeExpr">LikeExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.Literal.html" class="struct" title="struct datafusion::physical_expr::expressions::Literal">Literal</a>

Represents a literal value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.NegativeExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::NegativeExpr">NegativeExpr</a>

Negative expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.NoOp.html" class="struct" title="struct datafusion::physical_expr::expressions::NoOp">NoOp</a>

A place holder expression, can not be evaluated.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.NotExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::NotExpr">NotExpr</a>

Not expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.PhysicalSortExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::PhysicalSortExpr">PhysicalSortExpr</a>

Represents Sort operation for a column in a RecordBatch

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.TryCastExpr.html" class="struct" title="struct datafusion::physical_expr::expressions::TryCastExpr">TryCastExpr</a>

TRY_CAST expression casts an expression to a specific data type and returns NULL on invalid cast

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/struct.UnKnownColumn.html" class="struct" title="struct datafusion::physical_expr::expressions::UnKnownColumn">UnKnownColumn</a>

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/enum.StatsType.html" class="enum" title="enum datafusion::physical_expr::expressions::StatsType">StatsType</a>  
TODO: Move this to functions-aggregate module Enum used for differentiating population and sample for statistical functions

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.binary.html" class="fn" title="fn datafusion::physical_expr::expressions::binary">binary</a>  
Create a binary expression whose arguments are correctly coerced. This function errors if it is not possible to coerce the arguments to computational types supported by the operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.case.html" class="fn" title="fn datafusion::physical_expr::expressions::case">case</a>  
Create a CASE expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.cast.html" class="fn" title="fn datafusion::physical_expr::expressions::cast">cast</a>  
Return a PhysicalExpression representing `expr` casted to `cast_type`, if any casting is needed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.col.html" class="fn" title="fn datafusion::physical_expr::expressions::col">col</a>  
Create a column expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.format_state_name.html" class="fn" title="fn datafusion::physical_expr::expressions::format_state_name">format_state_name</a>  
Build state name. State is the intermediate state of the aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.in_list.html" class="fn" title="fn datafusion::physical_expr::expressions::in_list">in_list</a>  
Creates a unary expression InList

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.is_not_null.html" class="fn" title="fn datafusion::physical_expr::expressions::is_not_null">is_not_null</a>  
Create an IS NOT NULL expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.is_null.html" class="fn" title="fn datafusion::physical_expr::expressions::is_null">is_null</a>  
Create an IS NULL expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.like.html" class="fn" title="fn datafusion::physical_expr::expressions::like">like</a>  
Create a like expression, erroring if the argument types are not compatible.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.lit.html" class="fn" title="fn datafusion::physical_expr::expressions::lit">lit</a>  
Create a literal expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.negative.html" class="fn" title="fn datafusion::physical_expr::expressions::negative">negative</a>  
Creates a unary expression NEGATIVE

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.not.html" class="fn" title="fn datafusion::physical_expr::expressions::not">not</a>  
Creates a unary expression NOT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.similar_to.html" class="fn" title="fn datafusion::physical_expr::expressions::similar_to">similar_to</a>  
Create a similar to expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.try_cast.html" class="fn" title="fn datafusion::physical_expr::expressions::try_cast">try_cast</a>  
Return a PhysicalExpression representing `expr` casted to `cast_type`, if any casting is needed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/expressions/fn.with_new_schema.html" class="fn" title="fn datafusion::physical_expr::expressions::with_new_schema">with_new_schema</a>  
Rewrites an expression according to new schema; i.e. changes the columns it refers to with the column at corresponding index in the new schema. Returns an error if the given schema has fewer columns than the original schema. Note that the resulting expression may not be valid if data types in the new schema is incompatible with expression nodes.
