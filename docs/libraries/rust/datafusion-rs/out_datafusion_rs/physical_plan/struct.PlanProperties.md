# Struct PlanProperties Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#970" class="src">Source</a>

``` rust
pub struct PlanProperties {
    pub eq_properties: EquivalenceProperties,
    pub partitioning: Partitioning,
    pub emission_type: EmissionType,
    pub boundedness: Boundedness,
    pub evaluation_type: EvaluationType,
    pub scheduling_type: SchedulingType,
    /* private fields */
}
```

Expand description

Stores certain, often expensive to compute, plan properties used in query optimization.

These properties are stored a single structure to permit this information to be computed once and then those cached results used multiple times without recomputation (aka a cache)

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#structfield.eq_properties" class="anchor field">§</a>`eq_properties: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties"><code>EquivalenceProperties</code></a>

See [ExecutionPlanProperties::equivalence_properties](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.equivalence_properties "method datafusion::physical_plan::ExecutionPlanProperties::equivalence_properties")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#structfield.partitioning" class="anchor field">§</a>`partitioning: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning"><code>Partitioning</code></a>

See [ExecutionPlanProperties::output_partitioning](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_partitioning "method datafusion::physical_plan::ExecutionPlanProperties::output_partitioning")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#structfield.emission_type" class="anchor field">§</a>`emission_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType"><code>EmissionType</code></a>

See [ExecutionPlanProperties::pipeline_behavior](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.pipeline_behavior "method datafusion::physical_plan::ExecutionPlanProperties::pipeline_behavior")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#structfield.boundedness" class="anchor field">§</a>`boundedness: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness"><code>Boundedness</code></a>

See [ExecutionPlanProperties::boundedness](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.boundedness "method datafusion::physical_plan::ExecutionPlanProperties::boundedness")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#structfield.evaluation_type" class="anchor field">§</a>`evaluation_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EvaluationType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EvaluationType"><code>EvaluationType</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#structfield.scheduling_type" class="anchor field">§</a>`scheduling_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.SchedulingType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::SchedulingType"><code>SchedulingType</code></a>

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#impl-PlanProperties" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.new" class="fn">new</a>( eq_properties: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>, partitioning: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>, emission_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>, boundedness: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Construct a new `PlanPropertiesCache` from the

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.with_partitioning" class="fn">with_partitioning</a>(self, partitioning: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Overwrite output partitioning with its new value.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.with_eq_properties" class="fn">with_eq_properties</a>( self, eq_properties: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Overwrite equivalence properties with its new value.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.with_boundedness" class="fn">with_boundedness</a>(self, boundedness: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Overwrite boundedness with its new value.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.with_emission_type" class="fn">with_emission_type</a>(self, emission_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Overwrite emission type with its new value.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.with_scheduling_type" class="fn">with_scheduling_type</a>( self, scheduling_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.SchedulingType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::SchedulingType">SchedulingType</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Set the [`SchedulingType`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.SchedulingType.html "enum datafusion::physical_plan::execution_plan::SchedulingType").

Defaults to [`SchedulingType::NonCooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.SchedulingType.html#variant.NonCooperative "variant datafusion::physical_plan::execution_plan::SchedulingType::NonCooperative")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.with_evaluation_type" class="fn">with_evaluation_type</a>(self, drive_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EvaluationType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EvaluationType">EvaluationType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Set the [`EvaluationType`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EvaluationType.html "enum datafusion::physical_plan::execution_plan::EvaluationType").

Defaults to [`EvaluationType::Lazy`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EvaluationType.html#variant.Lazy "variant datafusion::physical_plan::execution_plan::EvaluationType::Lazy")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.with_constraints" class="fn">with_constraints</a>(self, constraints: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Overwrite constraints with its new value.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.equivalence_properties" class="fn">equivalence_properties</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.output_partitioning" class="fn">output_partitioning</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.output_ordering" class="fn">output_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#impl-Clone-for-PlanProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#impl-Debug-for-PlanProperties" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html" class="struct" title="struct datafusion::physical_plan::PlanProperties">PlanProperties</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.PlanProperties.html#blanket-implementations" class="anchor">§</a>
