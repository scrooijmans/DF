# Module propagate_empty_relation Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#58" class="src">Source</a>

Expand description

[`PropagateEmptyRelation`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/propagate_empty_relation/struct.PropagateEmptyRelation.html "struct datafusion::optimizer::propagate_empty_relation::PropagateEmptyRelation") eliminates nodes fed by `EmptyRelation`

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/propagate_empty_relation/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/propagate_empty_relation/struct.PropagateEmptyRelation.html" class="struct" title="struct datafusion::optimizer::propagate_empty_relation::PropagateEmptyRelation">PropagateEmptyRelation</a>  
Optimization rule that bottom-up to eliminate plan by propagating empty_relation.
