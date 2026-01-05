# Module sorts Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#83" class="src">Source</a>

Expand description

Sort functionalities

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/partial_sort/index.html" class="mod" title="mod datafusion::physical_plan::sorts::partial_sort">partial_sort</a>  
Partial Sort deals with input data that partially satisfies the required sort order. Such an input data can be partitioned into segments where each segment already has the required information for lexicographic sorting so sorting can be done without loading the entire dataset.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/index.html" class="mod" title="mod datafusion::physical_plan::sorts::sort">sort</a>  
Sort that deals with an arbitrary size of the input. It will do in-memory sorting if it has enough memory budget but spills to disk if needed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/index.html" class="mod" title="mod datafusion::physical_plan::sorts::sort_preserving_merge">sort_preserving_merge</a>  
[`SortPreservingMergeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html "struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec") merges multiple sorted streams into one sorted stream.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/streaming_merge/index.html" class="mod" title="mod datafusion::physical_plan::sorts::streaming_merge">streaming_merge</a>  
Merge that deals with an arbitrary size of streaming inputs. This is an order-preserving merge.
