# Struct BaselineMetrics Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/metrics/baseline.rs.html#47" class="src">Source</a>

``` rust
pub struct BaselineMetrics { /* private fields */ }
```

Expand description

Helper for creating and tracking common “baseline” metrics for each operator

Example:

``` rust
use datafusion_physical_plan::metrics::{BaselineMetrics, ExecutionPlanMetricsSet};
let metrics = ExecutionPlanMetricsSet::new();

let partition = 2;
let baseline_metrics = BaselineMetrics::new(&metrics, partition);

// during execution, in CPU intensive operation:
let timer = baseline_metrics.elapsed_compute().timer();
// .. do CPU intensive work
timer.done();

// when operator is finished:
baseline_metrics.done();
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#impl-BaselineMetrics" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.new" class="fn">new</a>( metrics: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>, partition: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>

Create a new BaselineMetric structure, and set `start_time` to now

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.intermediate" class="fn">intermediate</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>

Returns a [`BaselineMetrics`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html "struct datafusion::physical_plan::metrics::BaselineMetrics") that updates the same `elapsed_compute` ignoring all other metrics

This is useful when an operator offloads some of its intermediate work to separate tasks that as a result won’t be recorded by [`Self::record_poll`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.record_poll "method datafusion::physical_plan::metrics::BaselineMetrics::record_poll")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.elapsed_compute" class="fn">elapsed_compute</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Time.html" class="struct" title="struct datafusion::physical_plan::metrics::Time">Time</a>

return the metric for cpu time spend in this operator

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.output_rows" class="fn">output_rows</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Count.html" class="struct" title="struct datafusion::physical_plan::metrics::Count">Count</a>

return the metric for the total number of output rows produced

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.done" class="fn">done</a>(&self)

Records the fact that this operator’s execution is complete (recording the `end_time` metric).

Note care should be taken to call `done()` manually if `BaselineMetrics` is not `drop`ped immediately upon operator completion, as async streams may not be dropped immediately depending on the consumer.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.record_output" class="fn">record_output</a>(&self, num_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Record that some number of rows have been produced as output

See the [`RecordOutput`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html "trait datafusion::physical_plan::metrics::RecordOutput") for conveniently recording record batch output for other thing

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.try_done" class="fn">try_done</a>(&self)

If not previously recorded `done()`, record

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.record_poll" class="fn">record_poll</a>( &self, poll: <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>\>

Process a poll result of a stream producing output for an operator.

Note: this method only updates `output_rows` and `end_time` metrics. Remember to update `elapsed_compute` and other metrics manually.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#impl-Clone-for-BaselineMetrics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#impl-Debug-for-BaselineMetrics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#impl-Drop-for-BaselineMetrics" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html#blanket-implementations" class="anchor">§</a>
