# Module enforce_sorting Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#32" class="src">Source</a>

Expand description

EnforceSorting optimizer rule inspects the physical plan with respect to local sorting requirements and does the following:

- Adds a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") when a requirement is not met,
- Removes an already-existing [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") if it is possible to prove that this sort is unnecessary

The rule can work on valid *and* invalid physical plans with respect to sorting requirements, but always produces a valid physical plan in this sense.

A non-realistic but easy to follow example for sort removals: Assume that we somehow get the fragment

``` text
SortExec: expr=[nullable_col@0 ASC]
  SortExec: expr=[non_nullable_col@1 ASC]
```

in the physical plan. The first sort is unnecessary since its result is overwritten by another [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec"). Therefore, this rule removes it from the physical plan.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/replace_with_order_preserving_variants/index.html" class="mod" title="mod datafusion::physical_optimizer::enforce_sorting::replace_with_order_preserving_variants">replace_with_order_preserving_variants</a>

Optimizer rule that replaces executors that lose ordering with their order-preserving variants when it is helpful; either in terms of performance or to accommodate unbounded streams by fixing the pipeline.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/sort_pushdown/index.html" class="mod" title="mod datafusion::physical_optimizer::enforce_sorting::sort_pushdown">sort_pushdown</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html" class="struct" title="struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting">EnforceSorting</a>  
This rule inspects [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")’s in the given physical plan in order to remove unnecessary sorts, and optimize sort performance across the plan.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/fn.ensure_sorting.html" class="fn" title="fn datafusion::physical_optimizer::enforce_sorting::ensure_sorting">ensure_sorting</a>  
This function enforces sorting requirements and makes optimizations without violating these requirements whenever possible. Requires a bottom-up traversal.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/fn.parallelize_sorts.html" class="fn" title="fn datafusion::physical_optimizer::enforce_sorting::parallelize_sorts">parallelize_sorts</a>  
Transform [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") + [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") cascades into [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/type.PlanWithCorrespondingCoalescePartitions.html" class="type" title="type datafusion::physical_optimizer::enforce_sorting::PlanWithCorrespondingCoalescePartitions">PlanWithCorrespondingCoalescePartitions</a>  
This object is used within the [`EnforceSorting`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html "struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting") rule to track the closest [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") descendant(s) for every child of a plan. The data attribute stores whether the plan is a `CoalescePartitionsExec` or is connected to a `CoalescePartitionsExec` via its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/type.PlanWithCorrespondingSort.html" class="type" title="type datafusion::physical_optimizer::enforce_sorting::PlanWithCorrespondingSort">PlanWithCorrespondingSort</a>  
This context object is used within the [`EnforceSorting`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html "struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting") rule to track the closest [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") descendant(s) for every child of a plan. The data attribute stores whether the plan is a `SortExec` or is connected to a `SortExec` via its children.
