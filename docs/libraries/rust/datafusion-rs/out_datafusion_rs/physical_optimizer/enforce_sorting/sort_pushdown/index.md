# Module sort_pushdown Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_sorting/mod.rs.html#39" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/struct.ParentRequirements.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::sort_pushdown::ParentRequirements">ParentRequirements</a>  
This is a “data class” we use within the [`EnforceSorting`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html "struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting") rule to push down [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") in the plan. In some cases, we can reduce the total computational cost by pushing down `SortExec`s through some executors. The object carries the parent required ordering and the (optional) `fetch` value of the parent node as its data.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/fn.assign_initial_requirements.html" class="fn" title="fn datafusion::physical_optimizer::enforce_sorting::sort_pushdown::assign_initial_requirements">assign_initial_requirements</a>  
Assigns the ordering requirement of the root node to the its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/fn.pushdown_sorts.html" class="fn" title="fn datafusion::physical_optimizer::enforce_sorting::sort_pushdown::pushdown_sorts">pushdown_sorts</a>  
Tries to push down the sort requirements as far as possible, if decides a `SortExec` is unnecessary removes it.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/type.SortPushDown.html" class="type" title="type datafusion::physical_optimizer::enforce_sorting::sort_pushdown::SortPushDown">SortPushDown</a>
