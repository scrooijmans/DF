# Trait ExecutionPlanProperties Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#694" class="src">Source</a>

``` rust
pub trait ExecutionPlanProperties {
    // Required methods
    fn output_partitioning(&self) -> &Partitioning;
    fn output_ordering(&self) -> Option<&LexOrdering>;
    fn boundedness(&self) -> Boundedness;
    fn pipeline_behavior(&self) -> EmissionType;
    fn equivalence_properties(&self) -> &EquivalenceProperties;
}
```

Expand description

Extension trait provides an easy API to fetch various properties of [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") objects based on [`ExecutionPlan::properties`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#tymethod.properties "method datafusion::physical_plan::ExecutionPlan::properties").

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_partitioning" class="fn">output_partitioning</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

Specifies how the output of this `ExecutionPlan` is split into partitions.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_ordering" class="fn">output_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>

If the output of this `ExecutionPlan` within each partition is sorted, returns `Some(keys)` describing the ordering. A `None` return value indicates no assumptions should be made on the output ordering.

For example, `SortExec` (obviously) produces sorted output as does `SortPreservingMergeStream`. Less obviously, `Projection` produces sorted output if its input is sorted as it does not reorder the input rows.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.boundedness" class="fn">boundedness</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

Boundedness information of the stream corresponding to this `ExecutionPlan`. For more details, see [`Boundedness`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html "enum datafusion::physical_plan::execution_plan::Boundedness").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.pipeline_behavior" class="fn">pipeline_behavior</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

Indicates how the stream of this `ExecutionPlan` emits its results. For more details, see [`EmissionType`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html "enum datafusion::physical_plan::execution_plan::EmissionType").

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.equivalence_properties" class="fn">equivalence_properties</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

Get the [`EquivalenceProperties`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html "struct datafusion::physical_expr::EquivalenceProperties") within the plan.

Equivalence properties tell DataFusion what columns are known to be equal, during various optimization passes. By default, this returns “no known equivalences” which is always correct, but may cause DataFusion to unnecessarily resort data.

If this ExecutionPlan makes no changes to the schema of the rows flowing through it or how columns within each row relate to each other, it should return the equivalence properties of its input. For example, since [`FilterExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/struct.FilterExec.html "struct datafusion::physical_plan::filter::FilterExec") may remove rows from its input, but does not otherwise modify them, it preserves its input equivalence properties. However, since `ProjectionExec` may calculate derived expressions, it needs special handling.

See also [`ExecutionPlan::maintains_input_order`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html#method.maintains_input_order "method datafusion::physical_plan::ExecutionPlan::maintains_input_order") and [`Self::output_ordering`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_ordering "method datafusion_physical_plan::execution_plan::ExecutionPlanProperties::output_ordering::output_ordering") for related concepts.

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#impl-ExecutionPlanProperties-for-Arc%3Cdyn+ExecutionPlan%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlanProperties">ExecutionPlanProperties</a> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#method.output_partitioning" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_partitioning" class="fn">output_partitioning</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/enum.Partitioning.html" class="enum" title="enum datafusion::physical_expr::Partitioning">Partitioning</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#method.output_ordering" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.output_ordering" class="fn">output_ordering</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.LexOrdering.html" class="struct" title="struct datafusion::physical_expr::LexOrdering">LexOrdering</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#method.boundedness" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.boundedness" class="fn">boundedness</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.Boundedness.html" class="enum" title="enum datafusion::physical_plan::execution_plan::Boundedness">Boundedness</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#method.pipeline_behavior" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.pipeline_behavior" class="fn">pipeline_behavior</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/execution_plan/enum.EmissionType.html" class="enum" title="enum datafusion::physical_plan::execution_plan::EmissionType">EmissionType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#method.equivalence_properties" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#tymethod.equivalence_properties" class="fn">equivalence_properties</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/struct.EquivalenceProperties.html" class="struct" title="struct datafusion::physical_expr::EquivalenceProperties">EquivalenceProperties</a>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html#impl-ExecutionPlanProperties-for-%26dyn+ExecutionPlan" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlanProperties.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlanProperties">ExecutionPlanProperties</a> for &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan">ExecutionPlan</a>
