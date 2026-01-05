# Module function Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#51" class="src">Source</a>

Expand description

Function module contains typing and signature for built-in and user defined functions.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::AccumulatorArgs">AccumulatorArgs</a>  
[`AccumulatorArgs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.AccumulatorArgs.html "struct datafusion::logical_expr::function::AccumulatorArgs") contains information about how an aggregate function was called, including the types of its arguments and any optional ordering expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.ExpressionArgs.html" class="struct" title="struct datafusion::logical_expr::function::ExpressionArgs">ExpressionArgs</a>  
Arguments passed to user-defined window function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.PartitionEvaluatorArgs.html" class="struct" title="struct datafusion::logical_expr::function::PartitionEvaluatorArgs">PartitionEvaluatorArgs</a>  
Arguments passed to created user-defined window function state during physical execution.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html" class="struct" title="struct datafusion::logical_expr::function::StateFieldsArgs">StateFieldsArgs</a>  
[`StateFieldsArgs`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.StateFieldsArgs.html "struct datafusion::logical_expr::function::StateFieldsArgs") contains information about the fields that an aggregate function’s accumulator should have. Used for `AggregateUDFImpl::state_fields`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/struct.WindowUDFFieldArgs.html" class="struct" title="struct datafusion::logical_expr::function::WindowUDFFieldArgs">WindowUDFFieldArgs</a>  
Metadata for defining the result field from evaluating a user-defined window function.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/enum.Hint.html" class="enum" title="enum datafusion::logical_expr::function::Hint">Hint</a>

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/type.AccumulatorFactoryFunction.html" class="type" title="type datafusion::logical_expr::function::AccumulatorFactoryFunction">AccumulatorFactoryFunction</a>  
Factory that returns an accumulator for the given aggregate function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/type.AggregateFunctionSimplification.html" class="type" title="type datafusion::logical_expr::function::AggregateFunctionSimplification">AggregateFunctionSimplification</a>  
[crate::udaf::AggregateUDFImpl::simplify](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html#method.simplify "method datafusion::logical_expr::AggregateUDFImpl::simplify") simplifier closure A closure with two arguments:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/type.PartitionEvaluatorFactory.html" class="type" title="type datafusion::logical_expr::function::PartitionEvaluatorFactory">PartitionEvaluatorFactory</a>  
Factory that creates a PartitionEvaluator for the given window function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/type.ReturnTypeFunction.html" class="type" title="type datafusion::logical_expr::function::ReturnTypeFunction">ReturnTypeFunction</a>  
Factory that returns the functions’s return type given the input argument types

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/type.ScalarFunctionImplementation.html" class="type" title="type datafusion::logical_expr::function::ScalarFunctionImplementation">ScalarFunctionImplementation</a>  
Scalar function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/type.StateTypeFunction.html" class="type" title="type datafusion::logical_expr::function::StateTypeFunction">StateTypeFunction</a>  
Factory that returns the types used by an aggregator to serialize its state, given its return datatype.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/function/type.WindowFunctionSimplification.html" class="type" title="type datafusion::logical_expr::function::WindowFunctionSimplification">WindowFunctionSimplification</a>  
[crate::udwf::WindowUDFImpl::simplify](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html#method.simplify "method datafusion::logical_expr::WindowUDFImpl::simplify") simplifier closure A closure with two arguments:
