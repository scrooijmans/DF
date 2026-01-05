# Module joins Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#75" class="src">Source</a>

Expand description

DataFusion Join implementations

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/utils/index.html" class="mod" title="mod datafusion::physical_plan::joins::utils">utils</a>  
Join related functionality used both on logical and physical plans

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.CrossJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::CrossJoinExec">CrossJoinExec</a>  
Cross Join Execution Plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>  
Join execution plan: Evaluates equijoin predicates in parallel on multiple partitions using a hash table and an optional filter list to apply post join.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.NestedLoopJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::NestedLoopJoinExec">NestedLoopJoinExec</a>  
NestedLoopJoinExec is a build-probe join operator designed for joins that do not have equijoin keys in their `ON` clause.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>  
Join execution plan that executes equi-join predicates on multiple partitions using Sort-Merge join algorithm and applies an optional filter post join. Can be used to join arbitrarily large inputs where one or both of the inputs don’t fit in the available memory.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SymmetricHashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SymmetricHashJoinExec">SymmetricHashJoinExec</a>  
A symmetric hash join with range conditions is when both streams are hashed on the join key and the resulting hash tables are used to join the streams. The join is considered symmetric because the hash table is built on the join keys from both streams, and the matching of rows is based on the values of the join keys in both streams. This type of join is efficient in streaming context as it allows for fast lookups in the hash table, rather than having to scan through one or both of the streams to find matching rows, also it only considers the elements from the stream that fall within a certain sliding window (w/ range conditions), making it more efficient and less likely to store stale data. This enables operating on unbounded streaming data without any memory issues.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/enum.PartitionMode.html" class="enum" title="enum datafusion::physical_plan::joins::PartitionMode">PartitionMode</a>  
Hash join Partitioning mode

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/enum.StreamJoinPartitionMode.html" class="enum" title="enum datafusion::physical_plan::joins::StreamJoinPartitionMode">StreamJoinPartitionMode</a>  
Partitioning mode to use for symmetric hash join

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/type.JoinOn.html" class="type" title="type datafusion::physical_plan::joins::JoinOn">JoinOn</a>  
The on clause of the join, as vector of (left, right) columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/type.JoinOnRef.html" class="type" title="type datafusion::physical_plan::joins::JoinOnRef">JoinOnRef</a>  
Reference for JoinOn.
