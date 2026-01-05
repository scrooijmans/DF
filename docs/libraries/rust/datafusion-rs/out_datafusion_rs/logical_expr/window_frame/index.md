# Module window_frame Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/lib.rs.html#77" class="src">Source</a>

Expand description

Window frame module

The frame-spec determines which output rows are read by an aggregate window function. The frame-spec consists of four parts:

- A frame type - either ROWS, RANGE or GROUPS,
- A starting frame boundary,
- An ending frame boundary,
- An EXCLUDE clause.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_frame/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_frame/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::window_frame::WindowFrame">WindowFrame</a>  
The frame specification determines which output rows are read by an aggregate window function. The ending frame boundary can be omitted if the `BETWEEN` and `AND` keywords that surround the starting frame boundary are also omitted, in which case the ending frame boundary defaults to `CURRENT ROW`.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_frame/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_frame/enum.WindowFrameBound.html" class="enum" title="enum datafusion::logical_expr::window_frame::WindowFrameBound">WindowFrameBound</a>  
There are five ways to describe starting and ending frame boundaries:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/window_frame/enum.WindowFrameUnits.html" class="enum" title="enum datafusion::logical_expr::window_frame::WindowFrameUnits">WindowFrameUnits</a>  
There are three frame types: ROWS, GROUPS, and RANGE. The frame type determines how the starting and ending boundaries of the frame are measured.
