# Module eliminate_cross_join Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#45" class="src">Source</a>

Expand description

[`EliminateCrossJoin`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_cross_join/struct.EliminateCrossJoin.html "struct datafusion::optimizer::eliminate_cross_join::EliminateCrossJoin") converts `CROSS JOIN` to `INNER JOIN` if join predicates are available.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_cross_join/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/eliminate_cross_join/struct.EliminateCrossJoin.html" class="struct" title="struct datafusion::optimizer::eliminate_cross_join::EliminateCrossJoin">EliminateCrossJoin</a>
