# Function parallelize_sorts Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_sorting/mod.rs.html#390-392" class="src">Source</a>

``` rust
pub fn parallelize_sorts(
    requirements: PlanContext<bool>,
) -> Result<Transformed<PlanContext<bool>>, DataFusionError>
```

Expand description

Transform [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") + [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") cascades into [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")

- [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec") cascades, as illustrated below.

A [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") + [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") cascade combines partitions first, and then sorts:

``` text
  ┌ ─ ─ ─ ─ ─ ┐
   ┌─┬─┬─┐
  ││B│A│D│... ├──┐
   └─┴─┴─┘       │
  └ ─ ─ ─ ─ ─ ┘  │  ┌────────────────────────┐   ┌ ─ ─ ─ ─ ─ ─ ┐   ┌────────┐    ┌ ─ ─ ─ ─ ─ ─ ─ ┐
   Partition 1   │  │        Coalesce        │    ┌─┬─┬─┬─┬─┐      │        │     ┌─┬─┬─┬─┬─┐
                 ├──▶(no ordering guarantees)│──▶││B│E│A│D│C│...───▶  Sort  ├───▶││A│B│C│D│E│... │
                 │  │                        │    └─┴─┴─┴─┴─┘      │        │     └─┴─┴─┴─┴─┘
  ┌ ─ ─ ─ ─ ─ ┐  │  └────────────────────────┘   └ ─ ─ ─ ─ ─ ─ ┘   └────────┘    └ ─ ─ ─ ─ ─ ─ ─ ┘
   ┌─┬─┐         │                                 Partition                       Partition
  ││E│C│ ...  ├──┘
   └─┴─┘
  └ ─ ─ ─ ─ ─ ┘
   Partition 2
```

A [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") + [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec") cascade sorts each partition first, then merges partitions while preserving the sort:

``` text
  ┌ ─ ─ ─ ─ ─ ┐   ┌────────┐   ┌ ─ ─ ─ ─ ─ ┐
   ┌─┬─┬─┐        │        │    ┌─┬─┬─┐
  ││B│A│D│... │──▶│  Sort  │──▶││A│B│D│... │──┐
   └─┴─┴─┘        │        │    └─┴─┴─┘       │
  └ ─ ─ ─ ─ ─ ┘   └────────┘   └ ─ ─ ─ ─ ─ ┘  │  ┌─────────────────────┐    ┌ ─ ─ ─ ─ ─ ─ ─ ┐
   Partition 1                  Partition 1   │  │                     │     ┌─┬─┬─┬─┬─┐
                                              ├──▶ SortPreservingMerge ├───▶││A│B│C│D│E│... │
                                              │  │                     │     └─┴─┴─┴─┴─┘
  ┌ ─ ─ ─ ─ ─ ┐   ┌────────┐   ┌ ─ ─ ─ ─ ─ ┐  │  └─────────────────────┘    └ ─ ─ ─ ─ ─ ─ ─ ┘
   ┌─┬─┐          │        │    ┌─┬─┐         │                               Partition
  ││E│C│ ...  │──▶│  Sort  ├──▶││C│E│ ...  │──┘
   └─┴─┘          │        │    └─┴─┘
  └ ─ ─ ─ ─ ─ ┘   └────────┘   └ ─ ─ ─ ─ ─ ┘
   Partition 2                  Partition 2
```

The latter [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") + [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec") cascade performs sorting first on a per-partition basis, thereby parallelizing the sort.

The outcome is that plans of the form

``` text
     "SortExec: expr=\[a@0 ASC\]",
     "  ...nodes..."
     "    CoalescePartitionsExec",
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```

are transformed into

``` text
     "SortPreservingMergeExec: \[a@0 ASC\]",
     "  SortExec: expr=\[a@0 ASC\]",
     "    ...nodes..."
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```

by following connections from [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec")s to [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec")s. By performing sorting in parallel, we can increase performance in some scenarios.

This optimization requires that there are no nodes between the [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") and the [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec"), which requires single partitioning. Do not parallelize when the following scenario occurs:

``` text
     "SortExec: expr=\[a@0 ASC\]",
     "  ...nodes requiring single partitioning..."
     "    CoalescePartitionsExec",
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```

**Steps**

1.  Checks if the plan is either a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec"), a [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec"), or a [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec"). Otherwise, does nothing.
2.  If the plan is a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") or a final [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec") (i.e. output partitioning is 1):
    - Check for [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") in children. If found, check if it can be removed (with possible [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec")s). If so, remove (see `remove_bottleneck_in_subplan`).
    - If the plan is satisfying the ordering requirements, add a `SortExec`.
    - Add an SPM above the plan and return.
3.  If the plan is a [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec"):
    - Check if it can be removed (with possible [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec")s). If so, remove (see `remove_bottleneck_in_subplan`).
