# Module pruning Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#53" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/struct.CompositePruningStatistics.html" class="struct" title="struct datafusion::common::pruning::CompositePruningStatistics">CompositePruningStatistics</a>  
Combine multiple [`PruningStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html "trait datafusion::common::pruning::PruningStatistics") into a single [`CompositePruningStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/struct.CompositePruningStatistics.html "struct datafusion::common::pruning::CompositePruningStatistics"). This can be used to combine statistics from different sources, for example partition values and file statistics. This allows pruning with filters that depend on multiple sources of statistics, such as `WHERE partition_col = data_col`. This is done by iterating over the statistics and returning the first one that has information for the requested column. If multiple statistics have information for the same column, the first one is returned without any regard for completeness or accuracy. That is: if the first statistics has information for a column, even if it is incomplete, that is returned even if a later statistics has more complete information.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/struct.PartitionPruningStatistics.html" class="struct" title="struct datafusion::common::pruning::PartitionPruningStatistics">PartitionPruningStatistics</a>  
Prune files based on their partition values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/struct.PrunableStatistics.html" class="struct" title="struct datafusion::common::pruning::PrunableStatistics">PrunableStatistics</a>  
Prune a set of containers represented by their statistics.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html" class="trait" title="trait datafusion::common::pruning::PruningStatistics">PruningStatistics</a>  
A source of runtime statistical information to [`PruningPredicate`](https://docs.rs/datafusion/latest/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html)s.
