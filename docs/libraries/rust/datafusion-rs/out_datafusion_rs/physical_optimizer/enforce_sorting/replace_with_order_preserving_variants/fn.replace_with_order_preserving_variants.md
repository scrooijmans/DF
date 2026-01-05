# Function replace_with_order_preserving_variants Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/enforce_sorting/replace_with_order_preserving_variants.rs.html#246-259" class="src">Source</a>

``` rust
pub fn replace_with_order_preserving_variants(
    requirements: PlanContext<bool>,
    is_spr_better: bool,
    is_spm_better: bool,
    config: &ConfigOptions,
) -> Result<Transformed<PlanContext<bool>>, DataFusionError>
```

Expand description

The `replace_with_order_preserving_variants` optimizer sub-rule tries to remove `SortExec`s from the physical plan by replacing operators that do not preserve ordering with their order-preserving variants; i.e. by replacing ordinary `RepartitionExec`s with their sort-preserving variants or by replacing `CoalescePartitionsExec`s with `SortPreservingMergeExec`s.

If this replacement is helpful for removing a `SortExec`, it updates the plan. Otherwise, it leaves the plan unchanged.

NOTE: This optimizer sub-rule will only produce sort-preserving `RepartitionExec`s if the query is bounded or if the config option `prefer_existing_sort` is set to `true`.

The algorithm flow is simply like this:

1.  Visit nodes of the physical plan bottom-up and look for `SortExec` nodes. During the traversal, keep track of operators that maintain ordering (or can maintain ordering when replaced by an order-preserving variant) until a `SortExec` is found.
2.  When a `SortExec` is found, update the child of the `SortExec` by replacing operators that do not preserve ordering in the tree with their order preserving variants.
3.  Check if the `SortExec` is still necessary in the updated plan by comparing its input ordering with the output ordering it imposes. We do this because replacing operators that lose ordering with their order-preserving variants enables us to preserve the previously lost ordering at the input of `SortExec`.
4.  If the `SortExec` in question turns out to be unnecessary, remove it and use updated plan. Otherwise, use the original plan.
5.  Continue the bottom-up traversal until another `SortExec` is seen, or the traversal is complete.
