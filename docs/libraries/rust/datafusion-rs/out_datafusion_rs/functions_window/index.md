# Module functions_window Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/lib.rs.html#847" class="src">Source</a>

Expand description

re-export of [`datafusion_functions_window`](https://docs.rs/datafusion-functions-window/50.2.0/x86_64-unknown-linux-gnu/datafusion_functions_window/index.html "mod datafusion_functions_window") crate

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/cume_dist/index.html" class="mod" title="mod datafusion::functions_window::cume_dist">cume_dist</a>  
`cume_dist` window function implementation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/expr_fn/index.html" class="mod" title="mod datafusion::functions_window::expr_fn">expr_fn</a>  
Fluent-style API for creating `Expr`s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/lead_lag/index.html" class="mod" title="mod datafusion::functions_window::lead_lag">lead_lag</a>  
`lead` and `lag` window function implementations

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/macros/index.html" class="mod" title="mod datafusion::functions_window::macros">macros</a>  
Convenience macros for defining a user-defined window function and associated expression API (fluent style).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/nth_value/index.html" class="mod" title="mod datafusion::functions_window::nth_value">nth_value</a>  
`nth_value` window function implementation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/ntile/index.html" class="mod" title="mod datafusion::functions_window::ntile">ntile</a>  
`ntile` window function implementation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/planner/index.html" class="mod" title="mod datafusion::functions_window::planner">planner</a>  
SQL planning extensions like [`WindowFunctionPlanner`](https://docs.rs/datafusion/50.2.0/datafusion/functions_window/planner/struct.WindowFunctionPlanner.html "struct datafusion::functions_window::planner::WindowFunctionPlanner")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/rank/index.html" class="mod" title="mod datafusion::functions_window::rank">rank</a>  
Implementation of `rank`, `dense_rank`, and `percent_rank` window functions, which can be evaluated at runtime during query execution.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/row_number/index.html" class="mod" title="mod datafusion::functions_window::row_number">row_number</a>  
`row_number` window function implementation

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/macro.create_udwf_expr.html" class="macro" title="macro datafusion::functions_window::create_udwf_expr">create_udwf_expr</a>  
Create a [`WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction "variant datafusion::prelude::Expr::WindowFunction") expression that exposes a fluent API which you can use to build more complex expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/macro.define_udwf_and_expr.html" class="macro" title="macro datafusion::functions_window::define_udwf_and_expr">define_udwf_and_expr</a>  
Defines a user-defined window function.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/macro.get_or_init_udwf.html" class="macro" title="macro datafusion::functions_window::get_or_init_udwf">get_or_init_udwf</a>  
Lazily initializes a user-defined window function exactly once when called concurrently. Repeated calls return a reference to the same instance.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/fn.all_default_window_functions.html" class="fn" title="fn datafusion::functions_window::all_default_window_functions">all_default_window_functions</a>  
Returns all default window functions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_window/fn.register_all.html" class="fn" title="fn datafusion::functions_window::register_all">register_all</a>  
Registers all enabled packages with a [`FunctionRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html "trait datafusion::execution::FunctionRegistry")
