# Module unnest Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#89" class="src">Source</a>

Expand description

Define a plan for unnesting values in columns that contain a list type.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/unnest/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/unnest/struct.ListUnnest.html" class="struct" title="struct datafusion::physical_plan::unnest::ListUnnest">ListUnnest</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/unnest/struct.UnnestExec.html" class="struct" title="struct datafusion::physical_plan::unnest::UnnestExec">UnnestExec</a>  
Unnest the given columns (either with type struct or list) For list unnesting, each rows is vertically transformed into multiple rows For struct unnesting, each columns is horizontally transformed into multiple columns, Thus the original RecordBatch with dimension (n x m) may have new dimension (n’ x m’)
