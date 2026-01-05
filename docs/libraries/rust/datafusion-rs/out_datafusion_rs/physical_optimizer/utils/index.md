# Module utils Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#46" class="src">Source</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.add_sort_above.html" class="fn" title="fn datafusion::physical_optimizer::utils::add_sort_above">add_sort_above</a>  
This utility function adds a `SortExec` above an operator according to the given ordering requirements while preserving the original partitioning.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.add_sort_above_with_check.html" class="fn" title="fn datafusion::physical_optimizer::utils::add_sort_above_with_check">add_sort_above_with_check</a>  
This utility function adds a `SortExec` above an operator according to the given ordering requirements while preserving the original partitioning. If requirement is already satisfied no `SortExec` is added.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.is_coalesce_partitions.html" class="fn" title="fn datafusion::physical_optimizer::utils::is_coalesce_partitions">is_coalesce_partitions</a>  
Checks whether the given operator is a [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.is_limit.html" class="fn" title="fn datafusion::physical_optimizer::utils::is_limit">is_limit</a>  
Checks whether the given operator is a limit; i.e. either a [`LocalLimitExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.LocalLimitExec.html "struct datafusion::physical_plan::limit::LocalLimitExec") or a [`GlobalLimitExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.GlobalLimitExec.html "struct datafusion::physical_plan::limit::GlobalLimitExec").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.is_repartition.html" class="fn" title="fn datafusion::physical_optimizer::utils::is_repartition">is_repartition</a>  
Checks whether the given operator is a [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.is_sort.html" class="fn" title="fn datafusion::physical_optimizer::utils::is_sort">is_sort</a>  
Checks whether the given operator is a [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.is_sort_preserving_merge.html" class="fn" title="fn datafusion::physical_optimizer::utils::is_sort_preserving_merge">is_sort_preserving_merge</a>  
Checks whether the given operator is a [`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.is_union.html" class="fn" title="fn datafusion::physical_optimizer::utils::is_union">is_union</a>  
Checks whether the given operator is a [`UnionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/struct.UnionExec.html "struct datafusion::physical_plan::union::UnionExec").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/utils/fn.is_window.html" class="fn" title="fn datafusion::physical_optimizer::utils::is_window">is_window</a>  
Checks whether the given operator is a window; i.e. either a [`WindowAggExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowAggExec.html "struct datafusion::physical_plan::windows::WindowAggExec") or a [`BoundedWindowAggExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.BoundedWindowAggExec.html "struct datafusion::physical_plan::windows::BoundedWindowAggExec").
