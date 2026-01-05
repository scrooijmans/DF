# Function ensure_sorting Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_sorting/mod.rs.html#475-477" class="src">Source</a>

``` rust
pub fn ensure_sorting(
    requirements: PlanContext<bool>,
) -> Result<Transformed<PlanContext<bool>>, DataFusionError>
```

Expand description

This function enforces sorting requirements and makes optimizations without violating these requirements whenever possible. Requires a bottom-up traversal.

**Steps**

1.  Analyze if there are any immediate removals of [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s. If so, removes them (see `analyze_immediate_sort_removal`).
2.  For each child of the plan, if the plan requires an input ordering:
    - Checks if ordering is satisfied with the child. If not:
      - If the child has an output ordering, removes the unnecessary `SortExec`.
      - Adds sort above the child plan.
    - (Plan not requires input ordering)
      - Checks if the `SortExec` is neutralized in the plan. If so, removes it.
3.  Check and modify window operator:
    - Checks if the plan is a window operator, and connected with a sort. If so, either tries to update the window definition or removes unnecessary [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s (see `adjust_window_sort_removal`).
4.  Check and remove possibly unnecessary SPM:
    - Checks if the plan is SPM and child 1 output partitions, if so decides this SPM is unnecessary and removes it from the plan.
