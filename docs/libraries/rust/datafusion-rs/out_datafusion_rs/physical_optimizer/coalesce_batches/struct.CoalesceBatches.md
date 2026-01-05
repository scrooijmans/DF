# Struct CoalesceBatches Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/coalesce_batches.rs.html#38" class="src">Source</a>

``` rust
pub struct CoalesceBatches {}
```

Expand description

Optimizer rule that introduces CoalesceBatchesExec to avoid overhead with small batches that are produced by highly selective filters

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#impl-CoalesceBatches" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_batches::CoalesceBatches">CoalesceBatches</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_batches::CoalesceBatches">CoalesceBatches</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#impl-Debug-for-CoalesceBatches" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_batches::CoalesceBatches">CoalesceBatches</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#impl-Default-for-CoalesceBatches" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_batches::CoalesceBatches">CoalesceBatches</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_batches::CoalesceBatches">CoalesceBatches</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#impl-PhysicalOptimizerRule-for-CoalesceBatches" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html" class="struct" title="struct datafusion::physical_optimizer::coalesce_batches::CoalesceBatches">CoalesceBatches</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#method.optimize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.optimize" class="fn">optimize</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan` to an optimized form

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#method.schema_check" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.schema_check" class="fn">schema_check</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

A flag to indicate whether the physical planner should validate that the rule will not change the schema of the plan after the rewriting. Some of the optimization rules might change the nullable properties of the schema and should disable the schema check.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/coalesce_batches/struct.CoalesceBatches.html#blanket-implementations" class="anchor">§</a>
