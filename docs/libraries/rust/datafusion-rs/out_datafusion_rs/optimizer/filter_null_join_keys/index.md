# Module filter_null_join_keys Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#55" class="src">Source</a>

Expand description

[`FilterNullJoinKeys`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/filter_null_join_keys/struct.FilterNullJoinKeys.html "struct datafusion::optimizer::filter_null_join_keys::FilterNullJoinKeys") adds filters to join inputs when input isn’t nullable

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/filter_null_join_keys/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/filter_null_join_keys/struct.FilterNullJoinKeys.html" class="struct" title="struct datafusion::optimizer::filter_null_join_keys::FilterNullJoinKeys">FilterNullJoinKeys</a>  
The FilterNullJoinKeys rule will identify joins with equi-join conditions where the join key is nullable and then insert an `IsNotNull` filter on the nullable side since null values can never match.
