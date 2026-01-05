# Module eliminate_limit Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#50" class="src">Source</a>

Expand description

[`EliminateLimit`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_limit/struct.EliminateLimit.html "struct datafusion::optimizer::eliminate_limit::EliminateLimit") eliminates `LIMIT` when possible

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_limit/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_limit/struct.EliminateLimit.html" class="struct" title="struct datafusion::optimizer::eliminate_limit::EliminateLimit">EliminateLimit</a>  
Optimizer rule to replace `LIMIT 0` or `LIMIT` whose ancestor LIMIT’s skip is greater than or equal to current’s fetch
