# Module var_provider Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#76" class="src">Source</a>

Expand description

Variable provider

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/var_provider/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/var_provider/enum.VarType.html" class="enum" title="enum datafusion::logical_expr::var_provider::VarType">VarType</a>  
Variable type, system/user defined

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/var_provider/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/var_provider/trait.VarProvider.html" class="trait" title="trait datafusion::logical_expr::var_provider::VarProvider">VarProvider</a>  
A var provider for `@variable` and `@@variable` runtime values.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/var_provider/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/var_provider/fn.is_system_variables.html" class="fn" title="fn datafusion::logical_expr::var_provider::is_system_variables">is_system_variables</a>  
Returns true if the specified string is a “system” variable such as `@@version`
