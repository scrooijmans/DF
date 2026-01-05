# Module window_state Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#78" class="src">Source</a>

Expand description

Structures used to hold window function state (for implementing WindowUDFs)

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/struct.PartitionBatchState.html" class="struct" title="struct datafusion::logical_expr::window_state::PartitionBatchState">PartitionBatchState</a>  
State for each unique partition determined according to PARTITION BY column(s)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/struct.WindowAggState.html" class="struct" title="struct datafusion::logical_expr::window_state::WindowAggState">WindowAggState</a>  
Holds the state of evaluating a window function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/struct.WindowFrameStateGroups.html" class="struct" title="struct datafusion::logical_expr::window_state::WindowFrameStateGroups">WindowFrameStateGroups</a>  
This structure encapsulates all the state information we require as we scan groups of data while processing window frames.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/struct.WindowFrameStateRange.html" class="struct" title="struct datafusion::logical_expr::window_state::WindowFrameStateRange">WindowFrameStateRange</a>  
This structure encapsulates all the state information we require as we scan ranges of data while processing RANGE frames. Attribute `sort_options` stores the column ordering specified by the ORDER BY clause. This information is used to calculate the range.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_state/enum.WindowFrameContext.html" class="enum" title="enum datafusion::logical_expr::window_state::WindowFrameContext">WindowFrameContext</a>  
This object stores the window frame state for use in incremental calculations.
