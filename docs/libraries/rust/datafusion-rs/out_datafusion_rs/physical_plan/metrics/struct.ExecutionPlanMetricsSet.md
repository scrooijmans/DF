# Struct ExecutionPlanMetricsSet Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/metrics/mod.rs.html#358" class="src">Source</a>

``` rust
pub struct ExecutionPlanMetricsSet { /* private fields */ }
```

Expand description

A set of [`Metric`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html "struct datafusion::physical_plan::Metric")s for an individual “operator” (e.g. `&dyn ExecutionPlan`).

This structure is intended as a convenience for [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") implementations so they can generate different streams for multiple partitions but easily report them together.

Each `clone()` of this structure will add metrics to the same underlying metrics set

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#impl-ExecutionPlanMetricsSet" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

Create a new empty shared metrics set

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#method.register" class="fn">register</a>(&self, metric: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.Metric.html" class="struct" title="struct datafusion::physical_plan::Metric">Metric</a>\>)

Add the specified metric to the underlying metric set

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#method.clone_inner" class="fn">clone_inner</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::MetricsSet">MetricsSet</a>

Return a clone of the inner [`MetricsSet`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.MetricsSet.html "struct datafusion::physical_plan::metrics::MetricsSet")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#impl-Clone-for-ExecutionPlanMetricsSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#impl-Debug-for-ExecutionPlanMetricsSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#impl-Default-for-ExecutionPlanMetricsSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html" class="struct" title="struct datafusion::physical_plan::metrics::ExecutionPlanMetricsSet">ExecutionPlanMetricsSet</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/metrics/struct.ExecutionPlanMetricsSet.html#blanket-implementations" class="anchor">§</a>
