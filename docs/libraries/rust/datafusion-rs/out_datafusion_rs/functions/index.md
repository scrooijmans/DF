# Module functions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#831" class="src">Source</a>

Expand description

re-export of [`datafusion_functions`](https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions/index.html "mod datafusion_functions") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/index.html" class="mod" title="mod datafusion::functions::core">core</a>

Core datafusion expressions These are always available and not controlled by a feature flag “core” DataFusion functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/crypto/index.html" class="mod" title="mod datafusion::functions::crypto">crypto</a>

“crypto” DataFusion functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/datetime/index.html" class="mod" title="mod datafusion::functions::datetime">datetime</a>

Date and time expressions. Contains functions such as to_timestamp Enabled via feature flag `datetime_expressions` date & time DataFusion functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/encoding/index.html" class="mod" title="mod datafusion::functions::encoding">encoding</a>

Encoding expressions. Contains Hex and binary `encode` and `decode` functions. Enabled via feature flag `encoding_expressions`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/expr_fn/index.html" class="mod" title="mod datafusion::functions::expr_fn">expr_fn</a>

Fluent-style API for creating `Expr`s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/macros/index.html" class="mod" title="mod datafusion::functions::macros">macros</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/math/index.html" class="mod" title="mod datafusion::functions::math">math</a>

Mathematical functions. Enabled via feature flag `math_expressions` “math” DataFusion functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/planner/index.html" class="mod" title="mod datafusion::functions::planner">planner</a>

SQL planning extensions like [`UserDefinedFunctionPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/functions/planner/struct.UserDefinedFunctionPlanner.html "struct datafusion::functions::planner::UserDefinedFunctionPlanner")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/regex/index.html" class="mod" title="mod datafusion::functions::regex">regex</a>

Regular expression functions. Enabled via feature flag `regex_expressions` “regex” DataFusion functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/string/index.html" class="mod" title="mod datafusion::functions::string">string</a>

“string” DataFusion functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/index.html" class="mod" title="mod datafusion::functions::strings">strings</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/unicode/index.html" class="mod" title="mod datafusion::functions::unicode">unicode</a>

“unicode” DataFusion functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/utils/index.html" class="mod" title="mod datafusion::functions::utils">utils</a>

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/macro.downcast_arg.html" class="macro" title="macro datafusion::functions::downcast_arg">downcast_arg</a>  
Downcast an argument to a specific array type, returning an internal error if the cast fails

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/macro.downcast_named_arg.html" class="macro" title="macro datafusion::functions::downcast_named_arg">downcast_named_arg</a>  
Downcast a named argument to a specific array type, returning an internal error if the cast fails

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/macro.export_functions.html" class="macro" title="macro datafusion::functions::export_functions">export_functions</a>  
macro that exports a list of function names as:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/macro.make_udf_function.html" class="macro" title="macro datafusion::functions::make_udf_function">make_udf_function</a>  
Creates a singleton `ScalarUDF` of the `$UDF` function and a function named `$NAME` which returns that singleton.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/fn.all_default_functions.html" class="fn" title="fn datafusion::functions::all_default_functions">all_default_functions</a>  
Return all default functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/fn.register_all.html" class="fn" title="fn datafusion::functions::register_all">register_all</a>  
Registers all enabled packages with a [`FunctionRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html "trait datafusion::execution::FunctionRegistry")
