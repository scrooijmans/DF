# Module signals Copy item path

<a href="https://docs.rs/polars-error/0.51.0/x86_64-unknown-linux-gnu/src/polars_error/lib.rs.html#12" class="src">Source</a>

## Structs<a href="https://docs.rs/polars/latest/polars/error/signals/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/error/signals/struct.KeyboardInterrupt.html" class="struct" title="struct polars::error::signals::KeyboardInterrupt">KeyboardInterrupt</a>  
Python hooks SIGINT to instead generate a KeyboardInterrupt exception. So we do the same to try and abort long-running computations and return to Python so that the Python exception can be generated.

## Functions<a href="https://docs.rs/polars/latest/polars/error/signals/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/error/signals/fn.catch_keyboard_interrupt.html" class="fn" title="fn polars::error::signals::catch_keyboard_interrupt">catch_keyboard_interrupt</a>  
Runs the passed function, catching any KeyboardInterrupts if they occur while running the function.

<a href="https://docs.rs/polars/latest/polars/error/signals/fn.register_polars_keyboard_interrupt_hook.html" class="fn" title="fn polars::error::signals::register_polars_keyboard_interrupt_hook">register_polars_keyboard_interrupt_hook</a>  
<a href="https://docs.rs/polars/latest/polars/error/signals/fn.try_raise_keyboard_interrupt.html" class="fn" title="fn polars::error::signals::try_raise_keyboard_interrupt">try_raise_keyboard_interrupt</a>  
Checks if the keyboard interrupt flag is set, and if yes panics as a keyboard interrupt. This function is very cheap.
