# Module ensure_coop Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#33" class="src">Source</a>

Expand description

The [`EnsureCooperative`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/ensure_coop/struct.EnsureCooperative.html "struct datafusion::physical_optimizer::ensure_coop::EnsureCooperative") optimizer rule inspects the physical plan to find all portions of the plan that will not yield cooperatively. It will insert `CooperativeExec` nodes where appropriate to ensure execution plans always yield cooperatively.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/ensure_coop/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/ensure_coop/struct.EnsureCooperative.html" class="struct" title="struct datafusion::physical_optimizer::ensure_coop::EnsureCooperative">EnsureCooperative</a>  
`EnsureCooperative` is a [`PhysicalOptimizerRule`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/trait.PhysicalOptimizerRule.html "trait datafusion::physical_optimizer::PhysicalOptimizerRule") that inspects the physical plan for sub plans that do not participate in cooperative scheduling. The plan is subdivided into sub plans on eager evaluation boundaries. Leaf nodes and eager evaluation roots are checked to see if they participate in cooperative scheduling. Those that do no are wrapped in a [`CooperativeExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeExec.html "struct datafusion::physical_plan::coop::CooperativeExec") parent.
