# Struct EnforceDistribution Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_distribution.rs.html#183" class="src">Source</a>

``` rust
pub struct EnforceDistribution {}
```

Expand description

The `EnforceDistribution` rule ensures that distribution requirements are met. In doing so, this rule will increase the parallelism in the plan by introducing repartitioning operators to the physical plan.

For example, given an input such as:

``` text
┌─────────────────────────────────┐
│                                 │
│          ExecutionPlan          │
│                                 │
└─────────────────────────────────┘
            ▲         ▲
            │         │
      ┌─────┘         └─────┐
      │                     │
      │                     │
      │                     │
┌───────────┐         ┌───────────┐
│           │         │           │
│ batch A1  │         │ batch B1  │
│           │         │           │
├───────────┤         ├───────────┤
│           │         │           │
│ batch A2  │         │ batch B2  │
│           │         │           │
├───────────┤         ├───────────┤
│           │         │           │
│ batch A3  │         │ batch B3  │
│           │         │           │
└───────────┘         └───────────┘

     Input                 Input
       A                     B
```

This rule will attempt to add a `RepartitionExec` to increase parallelism (to 3, in this case) and create the following arrangement:

``` text
    ┌─────────────────────────────────┐
    │                                 │
    │          ExecutionPlan          │
    │                                 │
    └─────────────────────────────────┘
              ▲      ▲       ▲            Input now has 3
              │      │       │             partitions
      ┌───────┘      │       └───────┐
      │              │               │
      │              │               │
┌───────────┐  ┌───────────┐   ┌───────────┐
│           │  │           │   │           │
│ batch A1  │  │ batch A3  │   │ batch B3  │
│           │  │           │   │           │
├───────────┤  ├───────────┤   ├───────────┤
│           │  │           │   │           │
│ batch B2  │  │ batch B1  │   │ batch A2  │
│           │  │           │   │           │
└───────────┘  └───────────┘   └───────────┘
      ▲              ▲               ▲
      │              │               │
      └─────────┐    │    ┌──────────┘
                │    │    │
                │    │    │
    ┌─────────────────────────────────┐   batches are
    │       RepartitionExec(3)        │   repartitioned
    │           RoundRobin            │
    │                                 │
    └─────────────────────────────────┘
                ▲         ▲
                │         │
          ┌─────┘         └─────┐
          │                     │
          │                     │
          │                     │
    ┌───────────┐         ┌───────────┐
    │           │         │           │
    │ batch A1  │         │ batch B1  │
    │           │         │           │
    ├───────────┤         ├───────────┤
    │           │         │           │
    │ batch A2  │         │ batch B2  │
    │           │         │           │
    ├───────────┤         ├───────────┤
    │           │         │           │
    │ batch A3  │         │ batch B3  │
    │           │         │           │
    └───────────┘         └───────────┘


     Input                 Input
       A                     B
```

The `EnforceDistribution` rule

- is idempotent; i.e. it can be applied multiple times, each time producing the same result.
- always produces a valid plan in terms of distribution requirements. Its input plan can be valid or invalid with respect to distribution requirements, but the output plan will always be valid.
- produces a valid plan in terms of ordering requirements, *if* its input is a valid plan in terms of ordering requirements. If the input plan is invalid, this rule does not attempt to fix it as doing so is the responsibility of the `EnforceSorting` rule.

Note that distribution requirements are met in the strictest way. This may result in more than strictly necessary [`RepartitionExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html "struct datafusion::physical_plan::repartition::RepartitionExec")s in the plan, but meeting the requirements in the strictest way may help avoid possible data skew in joins.

For example for a hash join with keys (a, b, c), the required Distribution(a, b, c) can be satisfied by several alternative partitioning ways: (a, b, c), (a, b), (a, c), (b, c), (a), (b), (c) and ( ).

This rule only chooses the exact match and satisfies the Distribution(a, b, c) by a HashPartition(a, b, c).

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#impl-EnforceDistribution" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html" class="struct" title="struct datafusion::physical_optimizer::enforce_distribution::EnforceDistribution">EnforceDistribution</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html" class="struct" title="struct datafusion::physical_optimizer::enforce_distribution::EnforceDistribution">EnforceDistribution</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#impl-Debug-for-EnforceDistribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html" class="struct" title="struct datafusion::physical_optimizer::enforce_distribution::EnforceDistribution">EnforceDistribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#impl-Default-for-EnforceDistribution" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html" class="struct" title="struct datafusion::physical_optimizer::enforce_distribution::EnforceDistribution">EnforceDistribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html" class="struct" title="struct datafusion::physical_optimizer::enforce_distribution::EnforceDistribution">EnforceDistribution</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#impl-PhysicalOptimizerRule-for-EnforceDistribution" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html" class="trait" title="trait datafusion::physical_optimizer::PhysicalOptimizerRule">PhysicalOptimizerRule</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html" class="struct" title="struct datafusion::physical_optimizer::enforce_distribution::EnforceDistribution">EnforceDistribution</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#method.optimize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.optimize" class="fn">optimize</a>( &self, plan: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Rewrite `plan` to an optimized form

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

A human readable name for this optimizer rule

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#method.schema_check" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html#tymethod.schema_check" class="fn">schema_check</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

A flag to indicate whether the physical planner should validate that the rule will not change the schema of the plan after the rewriting. Some of the optimization rules might change the nullable properties of the schema and should disable the schema check.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_distribution/struct.EnforceDistribution.html#blanket-implementations" class="anchor">§</a>
