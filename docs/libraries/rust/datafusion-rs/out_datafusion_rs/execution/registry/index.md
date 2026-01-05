# Module registry Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/lib.rs.html#40" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/struct.MemoryFunctionRegistry.html" class="struct" title="struct datafusion::execution::registry::MemoryFunctionRegistry">MemoryFunctionRegistry</a>  
A [`FunctionRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html "trait datafusion::execution::FunctionRegistry") that uses in memory [`HashMap`](https://docs.rs/datafusion/50.2.0/datafusion/common/type.HashMap.html "type datafusion::common::HashMap")s

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::registry::FunctionRegistry">FunctionRegistry</a>  
A registry knows how to build logical expressions out of user-defined function’ names

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/registry/trait.SerializerRegistry.html" class="trait" title="trait datafusion::execution::registry::SerializerRegistry">SerializerRegistry</a>  
Serializer and deserializer registry for extensions like [UserDefinedLogicalNode](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.UserDefinedLogicalNode.html "trait datafusion::logical_expr::UserDefinedLogicalNode").
