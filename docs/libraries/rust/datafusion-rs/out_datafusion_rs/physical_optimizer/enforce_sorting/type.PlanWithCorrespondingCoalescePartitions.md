# Type Alias PlanWithCorrespondingCoalescePartitions Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_sorting/mod.rs.html#147" class="src">Source</a>

``` rust
pub type PlanWithCorrespondingCoalescePartitions = PlanContext<bool>;
```

Expand description

This object is used within the [`EnforceSorting`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/struct.EnforceSorting.html "struct datafusion::physical_optimizer::enforce_sorting::EnforceSorting") rule to track the closest [`CoalescePartitionsExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html "struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec") descendant(s) for every child of a plan. The data attribute stores whether the plan is a `CoalescePartitionsExec` or is connected to a `CoalescePartitionsExec` via its children.

The tracker halts at each [`SortExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html "struct datafusion::physical_plan::sorts::sort::SortExec") (where the SPM will act to replace the coalesce).

This requires a bottom-up traversal was previously performed, updating the children previously.

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/type.PlanWithCorrespondingCoalescePartitions.html#aliased-type" class="anchor">§</a>

``` rust
pub struct PlanWithCorrespondingCoalescePartitions {
    pub plan: Arc<dyn ExecutionPlan>,
    pub data: bool,
    pub children: Vec<PlanContext<bool>>,
}
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/type.PlanWithCorrespondingCoalescePartitions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/type.PlanWithCorrespondingCoalescePartitions.html#structfield.plan" class="anchor field">§</a>`plan: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html" class="trait" title="trait datafusion::physical_plan::ExecutionPlan"><code>ExecutionPlan</code></a>`>`

The execution plan associated with this context.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/type.PlanWithCorrespondingCoalescePartitions.html#structfield.data" class="anchor field">§</a>`data: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Custom data payload of the node.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/enforce_sorting/type.PlanWithCorrespondingCoalescePartitions.html#structfield.children" class="anchor field">§</a>`children: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/tree_node/struct.PlanContext.html" class="struct" title="struct datafusion::physical_plan::tree_node::PlanContext"><code>PlanContext</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>>`

Child contexts of this node.
