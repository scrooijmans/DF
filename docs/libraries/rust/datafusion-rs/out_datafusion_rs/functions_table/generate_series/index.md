# Module generate_series Copy item path

<a href="https://docs.rs/datafusion-functions-table/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_table/lib.rs.html#27" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.Empty.html" class="struct" title="struct datafusion::functions_table::generate_series::Empty">Empty</a>

Empty generator that produces no rows - used when series arguments contain null values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.GenerateSeriesFunc.html" class="struct" title="struct datafusion::functions_table::generate_series::GenerateSeriesFunc">GenerateSeriesFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.GenerateSeriesTable.html" class="struct" title="struct datafusion::functions_table::generate_series::GenerateSeriesTable">GenerateSeriesTable</a>

Table that generates a series of integers/timestamps from `start`(inclusive) to `end`, incrementing by step

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.GenericSeriesState.html" class="struct" title="struct datafusion::functions_table::generate_series::GenericSeriesState">GenericSeriesState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.RangeFunc.html" class="struct" title="struct datafusion::functions_table::generate_series::RangeFunc">RangeFunc</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/struct.TimestampValue.html" class="struct" title="struct datafusion::functions_table::generate_series::TimestampValue">TimestampValue</a>

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/enum.GenSeriesArgs.html" class="enum" title="enum datafusion::functions_table::generate_series::GenSeriesArgs">GenSeriesArgs</a>  
Indicates the arguments used for generating a series.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_table/generate_series/trait.SeriesValue.html" class="trait" title="trait datafusion::functions_table::generate_series::SeriesValue">SeriesValue</a>  
Trait for values that can be generated in a series
