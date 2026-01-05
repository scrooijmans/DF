# Struct OutputRequirements Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/output_requirements.rs.html#56" class="src">Source</a>

``` rust
pub struct OutputRequirements { /* private fields */ }
```

Expand description

This rule either adds or removes [`OutputRequirements`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html "struct datafusion::physical_optimizer::output_requirements::OutputRequirements")s to/from the physical plan according to its `mode` attribute, which is set by the constructors `new_add_mode` and `new_remove_mode`. With this rule, we can keep track of the global requirements (ordering and distribution) across rules.

The primary use case of this node and rule is to specify and preserve the desired output ordering and distribution the entire plan. When sending to a single client, a single partition may be desirable, but when sending to a multi-partitioned writer, keeping multiple partitions may be better.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#impl-OutputRequirements" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirements">OutputRequirements</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#method.new_add_mode" class="fn">new_add_mode</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirements">OutputRequirements</a>

Create a new rule which works in `Add` mode; i.e. it simply adds a top-level [`OutputRequirementExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirementExec.html "struct datafusion::physical_optimizer::output_requirements::OutputRequirementExec") into the physical plan to keep track of global ordering and distribution requirements if there are any. Note that this rule should run at the beginning.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#method.new_remove_mode" class="fn">new_remove_mode</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirements">OutputRequirements</a>

Create a new rule which works in `Remove` mode; i.e. it simply removes the top-level [`OutputRequirementExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirementExec.html "struct datafusion::physical_optimizer::output_requirements::OutputRequirementExec") from the physical plan if there is any. We do this because a `OutputRequirementExec` is an ancillary, non-executable operator whose sole purpose is to track global requirements during optimization. Therefore, a `OutputRequirementExec` should not appear in the final plan.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#impl-Debug-for-OutputRequirements" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirements">OutputRequirements</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#impl-PhysicalOptimizerRule-for-OutputRequirements" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirements">OutputRequirements</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#method.optimize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.optimize" class="fn">optimize</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, \_config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan` to an optimized form

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#method.schema_check" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.schema_check" class="fn">schema_check</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

A flag to indicate whether the physical planner should validate that the rule will not change the schema of the plan after the rewriting. Some of the optimization rules might change the nullable properties of the schema and should disable the schema check.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html#blanket-implementations" class="anchor">§</a>
