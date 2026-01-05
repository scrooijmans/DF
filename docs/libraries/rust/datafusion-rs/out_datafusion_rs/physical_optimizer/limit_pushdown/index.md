# Module limit_pushdown Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#36" class="src">Source</a>

Expand description

[`LimitPushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/struct.LimitPushdown.html "struct datafusion::physical_optimizer::limit_pushdown::LimitPushdown") pushes `LIMIT` down through `ExecutionPlan`s to reduce data transfer as much as possible.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/struct.GlobalRequirements.html" class="struct" title="struct datafusion::physical_optimizer::limit_pushdown::GlobalRequirements">GlobalRequirements</a>  
This is a “data class” we use within the [`LimitPushdown`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/struct.LimitPushdown.html "struct datafusion::physical_optimizer::limit_pushdown::LimitPushdown") rule to push down [`LimitExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/enum.LimitExec.html "enum datafusion::physical_optimizer::limit_pushdown::LimitExec") in the plan. GlobalRequirements are hold as a rule-wide state and holds the fetch and skip information. The struct also has a field named satisfied which means if the “current” plan is valid in terms of limits or not.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/struct.LimitPushdown.html" class="struct" title="struct datafusion::physical_optimizer::limit_pushdown::LimitPushdown">LimitPushdown</a>  
This rule inspects [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan")’s and pushes down the fetch limit from the parent to the child if applicable.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/enum.LimitExec.html" class="enum" title="enum datafusion::physical_optimizer::limit_pushdown::LimitExec">LimitExec</a>  
This enumeration makes `skip` and `fetch` calculations easier by providing a single API for both local and global limit operators.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/limit_pushdown/fn.pushdown_limit_helper.html" class="fn" title="fn datafusion::physical_optimizer::limit_pushdown::pushdown_limit_helper">pushdown_limit_helper</a>  
This function is the main helper function of the `LimitPushDown` rule. The helper takes an `ExecutionPlan` and a global (algorithm) state which is an instance of `GlobalRequirements` and modifies these parameters while checking if the limits can be pushed down or not.
