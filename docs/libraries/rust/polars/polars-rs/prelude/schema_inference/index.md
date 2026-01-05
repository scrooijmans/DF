# Module schema_inference Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/mod.rs.html#24" class="src">Source</a>

Available on **crate feature `polars-io`** only.

## Structs<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>

## Functions<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/fn.finish_infer_field_schema.html" class="fn" title="fn polars::prelude::schema_inference::finish_infer_field_schema">finish_infer_field_schema</a>  
<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/fn.infer_field_schema.html" class="fn" title="fn polars::prelude::schema_inference::infer_field_schema">infer_field_schema</a>  
Infer the data type of a record

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/fn.infer_file_schema.html" class="fn" title="fn polars::prelude::schema_inference::infer_file_schema">infer_file_schema</a>  
Infer the schema of a CSV file by reading through the first n rows of the file, with `max_read_rows` controlling the maximum number of rows to read.
