# Struct Metric Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/metrics/mod.rs.html#71" class="src">Source</a>

``` rust
pub struct Metric { /* private fields */ }
```

Expand description

Something that tracks a value of interest (metric) of a DataFusion [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") execution.

Typically [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric")s are not created directly, but instead are created using [`MetricBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricBuilder.html "struct datafusion::physical_plan::metrics::MetricBuilder") or methods on [`ExecutionPlanMetricsSet`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html "struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet").

``` rust
 use datafusion_physical_plan::metrics::*;

 let metrics = ExecutionPlanMetricsSet::new();
 assert!(metrics.clone_inner().output_rows().is_none());

 // Create a counter to increment using the MetricBuilder
 let partition = 1;
 let output_rows = MetricBuilder::new(&metrics)
     .output_rows(partition);

 // Counter can be incremented
 output_rows.add(13);

 // The value can be retrieved directly:
 assert_eq!(output_rows.value(), 13);

 // As well as from the metrics set
 assert_eq!(metrics.clone_inner().output_rows(), Some(13));
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#impl-Metric" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::Metric">Metric</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.new" class="fn">new</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/enum.MetricValue.html" class="enum" title="enum datafusion::physical_plan::metrics::MetricValue">MetricValue</a>, partition: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::Metric">Metric</a>

Create a new [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric"). Consider using [`MetricBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricBuilder.html "struct datafusion::physical_plan::metrics::MetricBuilder") rather than this function directly.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.new_with_labels" class="fn">new_with_labels</a>( value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/enum.MetricValue.html" class="enum" title="enum datafusion::physical_plan::metrics::MetricValue">MetricValue</a>, partition: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, labels: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Label.html" class="struct" title="struct datafusion::physical_plan::metrics::Label">Label</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::Metric">Metric</a>

Create a new [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric"). Consider using [`MetricBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricBuilder.html "struct datafusion::physical_plan::metrics::MetricBuilder") rather than this function directly.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.with_label" class="fn">with_label</a>(self, label: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Label.html" class="struct" title="struct datafusion::physical_plan::metrics::Label">Label</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::Metric">Metric</a>

Add a new label to this metric

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.labels" class="fn">labels</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Label.html" class="struct" title="struct datafusion::physical_plan::metrics::Label">Label</a>\]

What labels are present for this metric?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.value" class="fn">value</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/enum.MetricValue.html" class="enum" title="enum datafusion::physical_plan::metrics::MetricValue">MetricValue</a>

Return a reference to the value of this metric

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.value_mut" class="fn">value_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/enum.MetricValue.html" class="enum" title="enum datafusion::physical_plan::metrics::MetricValue">MetricValue</a>

Return a mutable reference to the value of this metric

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.partition" class="fn">partition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Return a reference to the partition

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#impl-Debug-for-Metric" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::Metric">Metric</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#impl-Display-for-Metric" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::Metric">Metric</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html#blanket-implementations" class="anchor">§</a>
