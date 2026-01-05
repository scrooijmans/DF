# Module work_table Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#91" class="src">Source</a>

Expand description

Defines the work table query plan

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/work_table/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/work_table/struct.WorkTable.html" class="struct" title="struct datafusion::physical_plan::work_table::WorkTable">WorkTable</a>  
The name is from PostgreSQL’s terminology. See <https://wiki.postgresql.org/wiki/CTEReadme#How_Recursion_Works> This table serves as a mirror or buffer between each iteration of a recursive query.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/work_table/struct.WorkTableExec.html" class="struct" title="struct datafusion::physical_plan::work_table::WorkTableExec">WorkTableExec</a>  
A temporary “working table” operation where the input data will be taken from the named handle during the execution and will be re-published as is (kind of like a mirror).
