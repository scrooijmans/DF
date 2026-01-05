# Struct ScopedTimerGuard Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/metrics/value.rs.html#321" class="src">Source</a>

``` rust
pub struct ScopedTimerGuard<'a> { /* private fields */ }
```

Expand description

RAAI structure that adds all time between its construction and destruction to the CPU time or the first call to `stop` whichever comes first

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#impl-ScopedTimerGuard%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html" class="struct" title="struct datafusion::physical_plan::metrics::ScopedTimerGuard">ScopedTimerGuard</a>\<'\_\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#method.stop" class="fn">stop</a>(&mut self)

Stop the timer timing and record the time taken

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#method.restart" class="fn">restart</a>(&mut self)

Restarts the timer recording from the current time

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#method.done" class="fn">done</a>(self)

Stop the timer, record the time taken and consume self

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#method.stop_with" class="fn">stop_with</a>(&mut self, end_time: <a href="https://doc.rust-lang.org/nightly/std/time/struct.Instant.html" class="struct" title="struct std::time::Instant">Instant</a>)

Stop the timer timing and record the time taken since the given endpoint.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#method.done_with" class="fn">done_with</a>(self, end_time: <a href="https://doc.rust-lang.org/nightly/std/time/struct.Instant.html" class="struct" title="struct std::time::Instant">Instant</a>)

Stop the timer, record the time taken since `end_time` endpoint, and consume self.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#impl-Drop-for-ScopedTimerGuard%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html" class="struct" title="struct datafusion::physical_plan::metrics::ScopedTimerGuard">ScopedTimerGuard</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html#blanket-implementations" class="anchor">§</a>
