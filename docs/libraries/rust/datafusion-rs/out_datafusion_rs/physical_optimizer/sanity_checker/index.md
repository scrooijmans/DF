# Module sanity_checker Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#43" class="src">Source</a>

Expand description

The [SanityCheckPlan](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/struct.SanityCheckPlan.html "struct datafusion::physical_optimizer::sanity_checker::SanityCheckPlan") rule ensures that a given plan can accommodate its infinite sources, if there are any. It will reject non-runnable query plans that use pipeline-breaking operators on infinite input(s). In addition, it will check if all order and distribution requirements of a plan are satisfied by its children.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/struct.SanityCheckPlan.html" class="struct" title="struct datafusion::physical_optimizer::sanity_checker::SanityCheckPlan">SanityCheckPlan</a>  
The SanityCheckPlan rule rejects the following query plans:

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/fn.check_finiteness_requirements.html" class="fn" title="fn datafusion::physical_optimizer::sanity_checker::check_finiteness_requirements">check_finiteness_requirements</a>  
This function propagates finiteness information and rejects any plan with pipeline-breaking operators acting on infinite inputs.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/sanity_checker/fn.check_plan_sanity.html" class="fn" title="fn datafusion::physical_optimizer::sanity_checker::check_plan_sanity">check_plan_sanity</a>  
Ensures that the plan is pipeline friendly and the order and distribution requirements from its children are satisfied.
