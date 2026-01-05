# Macro handle_state Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/joins/utils.rs.html#1403" class="src">Source</a>

``` rust
macro_rules! handle_state {
    ($match_case:expr) => { ... };
}
```

Expand description

The `handle_state` macro is designed to process the result of a state-changing operation. It operates on a `StatefulStreamResult` by matching its variants and executing corresponding actions. This macro is used to streamline code that deals with state transitions, reducing boilerplate and improving readability.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/macro.handle_state.html#cases" class="doc-anchor">§</a>Cases

- `Ok(StatefulStreamResult::Continue)`: Continues the loop, indicating the stream join operation should proceed to the next step.
- `Ok(StatefulStreamResult::Ready(result))`: Returns a `Poll::Ready` with the result, either yielding a value or indicating the stream is awaiting more data.
- `Err(e)`: Returns a `Poll::Ready` containing an error, signaling an issue during the stream join operation.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/macro.handle_state.html#arguments" class="doc-anchor">§</a>Arguments

- `$match_case`: An expression that evaluates to a `Result<StatefulStreamResult<_>>`.
