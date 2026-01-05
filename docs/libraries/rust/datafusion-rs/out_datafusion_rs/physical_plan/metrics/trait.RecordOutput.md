# Trait RecordOutput Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/metrics/baseline.rs.html#190" class="src">Source</a>

``` rust
pub trait RecordOutput {
    // Required method
    fn record_output(self, bm: &BaselineMetrics) -> Self;
}
```

Expand description

Trait for things that produce output rows as a result of execution.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#tymethod.record_output" class="fn">record_output</a>(self, bm: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>) -\> Self

Record that some number of output rows have been produced

Meant to be composable so that instead of returning `batch` the operator can return `batch.record_output(baseline_metrics)`

## Dyn Compatibility<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#impl-RecordOutput-for-Option%3C%26RecordBatch%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html" class="trait" title="trait datafusion::physical_plan::metrics::RecordOutput">RecordOutput</a> for <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#method.record_output" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#tymethod.record_output" class="fn">record_output</a>(self, bm: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#impl-RecordOutput-for-Option%3CRecordBatch%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html" class="trait" title="trait datafusion::physical_plan::metrics::RecordOutput">RecordOutput</a> for <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#method.record_output-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#tymethod.record_output" class="fn">record_output</a>(self, bm: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#impl-RecordOutput-for-Result%3CRecordBatch,+DataFusionError%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html" class="trait" title="trait datafusion::physical_plan::metrics::RecordOutput">RecordOutput</a> for <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#method.record_output-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#tymethod.record_output" class="fn">record_output</a>( self, bm: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#impl-RecordOutput-for-usize" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html" class="trait" title="trait datafusion::physical_plan::metrics::RecordOutput">RecordOutput</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#method.record_output-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#tymethod.record_output" class="fn">record_output</a>(self, bm: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#impl-RecordOutput-for-%26RecordBatch" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html" class="trait" title="trait datafusion::physical_plan::metrics::RecordOutput">RecordOutput</a> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html#impl-RecordOutput-for-RecordBatch" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html" class="trait" title="trait datafusion::physical_plan::metrics::RecordOutput">RecordOutput</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>
