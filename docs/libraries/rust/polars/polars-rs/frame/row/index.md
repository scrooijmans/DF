# Module row Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/mod.rs.html#33" class="src">Source</a>

## Structs<a href="https://docs.rs/polars/latest/polars/frame/row/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>

## Enums<a href="https://docs.rs/polars/latest/polars/frame/row/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBuffer.html" class="enum" title="enum polars::frame::row::AnyValueBuffer">AnyValueBuffer</a>  
<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html" class="enum" title="enum polars::frame::row::AnyValueBufferTrusted">AnyValueBufferTrusted</a>  
An [`AnyValueBuffer`](https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBuffer.html "enum polars::frame::row::AnyValueBuffer") that should be used when we trust the builder

## Functions<a href="https://docs.rs/polars/latest/polars/frame/row/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/fn.coerce_dtype.html" class="fn" title="fn polars::frame::row::coerce_dtype">coerce_dtype</a>  
Coerces a slice of datatypes into a single supertype.

<a href="https://docs.rs/polars/latest/polars/frame/row/fn.infer_schema.html" class="fn" title="fn polars::frame::row::infer_schema">infer_schema</a>  
<a href="https://docs.rs/polars/latest/polars/frame/row/fn.rows_to_schema_first_non_null.html" class="fn" title="fn polars::frame::row::rows_to_schema_first_non_null">rows_to_schema_first_non_null</a>  
Infer schema from rows and set the first no null type as column data type.

<a href="https://docs.rs/polars/latest/polars/frame/row/fn.rows_to_schema_supertypes.html" class="fn" title="fn polars::frame::row::rows_to_schema_supertypes">rows_to_schema_supertypes</a>  
Infer the schema of rows by determining the supertype of the values.

<a href="https://docs.rs/polars/latest/polars/frame/row/fn.rows_to_supertypes.html" class="fn" title="fn polars::frame::row::rows_to_supertypes">rows_to_supertypes</a>  
Infer the schema data types of rows by determining the supertype of the values.
