# Module eliminate_join Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#49" class="src">Source</a>

Expand description

[`EliminateJoin`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_join/struct.EliminateJoin.html "struct datafusion::optimizer::eliminate_join::EliminateJoin") rewrites `INNER JOIN` with `true`/`null`

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_join/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_join/struct.EliminateJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_join::EliminateJoin">EliminateJoin</a>  
Eliminates joins when join condition is false. Replaces joins when inner join condition is true with a cross join.
