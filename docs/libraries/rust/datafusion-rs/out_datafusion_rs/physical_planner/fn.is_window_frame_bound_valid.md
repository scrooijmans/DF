# Function is_window_frame_bound_valid Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/physical_planner.rs.html#1617-1630" class="src">Source</a>

``` rust
pub fn is_window_frame_bound_valid(window_frame: &WindowFrame) -> bool
```

Expand description

Check if window bounds are valid after schema information is available, and window_frame bounds are casted to the corresponding column type. queries like: OVER (ORDER BY a RANGES BETWEEN 3 PRECEDING AND 5 PRECEDING) OVER (ORDER BY a RANGES BETWEEN INTERVAL ‘3 DAY’ PRECEDING AND ‘5 DAY’ PRECEDING) are rejected
