# Module decorrelate_predicate_subquery Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#44" class="src">Source</a>

Expand description

[`DecorrelatePredicateSubquery`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_predicate_subquery/struct.DecorrelatePredicateSubquery.html "struct datafusion::optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery") converts `IN`/`EXISTS` subquery predicates to `SEMI`/`ANTI` joins

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_predicate_subquery/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/decorrelate_predicate_subquery/struct.DecorrelatePredicateSubquery.html" class="struct" title="struct datafusion::optimizer::decorrelate_predicate_subquery::DecorrelatePredicateSubquery">DecorrelatePredicateSubquery</a>  
Optimizer rule for rewriting predicate(IN/EXISTS) subquery to left semi/anti joins
