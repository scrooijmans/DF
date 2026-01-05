# Module metrics Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#78" class="src">Source</a>

Expand description

Metrics for recording information about execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.BaselineMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::BaselineMetrics">BaselineMetrics</a>  
Helper for creating and tracking common “baseline” metrics for each operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Count.html" class="struct" title="struct datafusion::physical_plan::metrics::Count">Count</a>  
A counter to record things such as number of input or output rows

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>  
A set of [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric")s for an individual “operator” (e.g. `&dyn ExecutionPlan`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Gauge.html" class="struct" title="struct datafusion::physical_plan::metrics::Gauge">Gauge</a>  
A gauge is the simplest metrics type. It just returns a value. For example, you can easily expose current memory consumption with a gauge.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Label.html" class="struct" title="struct datafusion::physical_plan::metrics::Label">Label</a>  
`name=value` pairs identifying a metric. This concept is called various things in various different systems:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::metrics::Metric">Metric</a>  
Something that tracks a value of interest (metric) of a DataFusion [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") execution.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricBuilder.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricBuilder">MetricBuilder</a>  
Structure for constructing metrics, counters, timers, etc.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>  
A snapshot of the metrics for a particular ([`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ScopedTimerGuard.html" class="struct" title="struct datafusion::physical_plan::metrics::ScopedTimerGuard">ScopedTimerGuard</a>  
RAAI structure that adds all time between its construction and destruction to the CPU time or the first call to `stop` whichever comes first

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.SpillMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::SpillMetrics">SpillMetrics</a>  
Helper for creating and tracking spill-related metrics for each operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.SplitMetrics.html" class="struct" title="struct datafusion::physical_plan::metrics::SplitMetrics">SplitMetrics</a>  
Metrics for tracking [`crate::stream::BatchSplitStream`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.BatchSplitStream.html "struct datafusion::physical_plan::stream::BatchSplitStream") activity

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Time.html" class="struct" title="struct datafusion::physical_plan::metrics::Time">Time</a>  
Measure a potentially non contiguous duration of time

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.Timestamp.html" class="struct" title="struct datafusion::physical_plan::metrics::Timestamp">Timestamp</a>  
Stores a single timestamp, stored as the number of nanoseconds elapsed from Jan 1, 1970 UTC

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/enum.MetricValue.html" class="enum" title="enum datafusion::physical_plan::metrics::MetricValue">MetricValue</a>  
Possible values for a [super::Metric](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric").

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.CustomMetricValue.html" class="trait" title="trait datafusion::physical_plan::metrics::CustomMetricValue">CustomMetricValue</a>  
A trait for implementing custom metric values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/trait.RecordOutput.html" class="trait" title="trait datafusion::physical_plan::metrics::RecordOutput">RecordOutput</a>  
Trait for things that produce output rows as a result of execution.
