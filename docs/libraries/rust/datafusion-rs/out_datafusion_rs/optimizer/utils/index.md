# Module utils Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/lib.rs.html#65" class="src">Source</a>

Expand description

Utility functions leveraged by the query optimizer rules

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/struct.NamePreserver.html" class="struct" title="struct datafusion::optimizer::utils::NamePreserver">NamePreserver</a>  
Handles ensuring the name of rewritten expressions is not changed.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/fn.evaluates_to_null.html" class="fn" title="fn datafusion::optimizer::utils::evaluates_to_null">evaluates_to_null</a>  
Determines if an expression will always evaluate to null. `c0 + 8` return true `c0 IS NULL` return false `CASE WHEN c0 > 1 then 0 else 1` return false

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/fn.is_restrict_null_predicate.html" class="fn" title="fn datafusion::optimizer::utils::is_restrict_null_predicate">is_restrict_null_predicate</a>  
Determine whether a predicate can restrict NULLs. e.g. `c0 > 8` return true; `c0 IS NULL` return false.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/utils/fn.log_plan.html" class="fn" title="fn datafusion::optimizer::utils::log_plan">log_plan</a>  
Log the plan in debug/tracing mode after some part of the optimizer runs
