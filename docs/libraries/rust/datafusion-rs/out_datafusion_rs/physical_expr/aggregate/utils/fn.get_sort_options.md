# Function get_sort_options Copy item path

<a href="https://docs.rs/datafusion-functions-aggregate-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_aggregate_common/utils.rs.html#61" class="src">Source</a>

``` rust
pub fn get_sort_options(ordering_req: &LexOrdering) -> Vec<SortOptions>
```

Expand description

Selects the sort option attribute from all the given `PhysicalSortExpr`s.
