# Module join_selection Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#35" class="src">Source</a>

Expand description

The [`JoinSelection`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/struct.JoinSelection.html "struct datafusion::physical_optimizer::join_selection::JoinSelection") rule tries to modify a given plan so that it can accommodate infinite sources and utilize statistical information (if there is any) to obtain more performant plans. To achieve the first goal, it tries to transform a non-runnable query (with the given infinite sources) into a runnable query by replacing pipeline-breaking join operations with pipeline-friendly ones. To achieve the second goal, it selects the proper `PartitionMode` and the build side using the available statistics for hash joins.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/struct.JoinSelection.html" class="struct" title="struct datafusion::physical_optimizer::join_selection::JoinSelection">JoinSelection</a>  
The [`JoinSelection`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/struct.JoinSelection.html "struct datafusion::physical_optimizer::join_selection::JoinSelection") rule tries to modify a given plan so that it can accommodate infinite sources and optimize joins in the plan according to available statistical information, if there is any.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/fn.hash_join_swap_subrule.html" class="fn" title="fn datafusion::physical_optimizer::join_selection::hash_join_swap_subrule">hash_join_swap_subrule</a>  
This subrule will swap build/probe sides of a hash join depending on whether one of its inputs may produce an infinite stream of records. The rule ensures that the left (build) side of the hash join always operates on an input stream that will produce a finite set of records. If the left side can not be chosen to be “finite”, the join sides stay the same as the original query.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/join_selection/type.PipelineFixerSubrule.html" class="type" title="type datafusion::physical_optimizer::join_selection::PipelineFixerSubrule">PipelineFixerSubrule</a>  
Pipeline-fixing join selection subrule.
