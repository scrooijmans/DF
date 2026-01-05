# Module union Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#88" class="src">Source</a>

Expand description

The Union operator combines multiple inputs with the same schema

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/struct.InterleaveExec.html" class="struct" title="struct datafusion::physical_plan::union::InterleaveExec">InterleaveExec</a>  
Combines multiple input streams by interleaving them.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/struct.UnionExec.html" class="struct" title="struct datafusion::physical_plan::union::UnionExec">UnionExec</a>  
`UnionExec`: `UNION ALL` execution plan.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/fn.can_interleave.html" class="fn" title="fn datafusion::physical_plan::union::can_interleave">can_interleave</a>  
If all the input partitions have the same Hash partition spec with the first_input_partition The InterleaveExec is partition aware.
