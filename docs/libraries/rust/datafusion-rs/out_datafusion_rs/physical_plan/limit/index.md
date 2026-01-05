# Module limit Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#76" class="src">Source</a>

Expand description

Defines the LIMIT plan

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.GlobalLimitExec.html" class="struct" title="struct datafusion::physical_plan::limit::GlobalLimitExec">GlobalLimitExec</a>  
Limit execution plan

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.LimitStream.html" class="struct" title="struct datafusion::physical_plan::limit::LimitStream">LimitStream</a>  
A Limit stream skips `skip` rows, and then fetch up to `fetch` rows.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.LocalLimitExec.html" class="struct" title="struct datafusion::physical_plan::limit::LocalLimitExec">LocalLimitExec</a>  
LocalLimitExec applies a limit to a single partition
