# Module frame Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/lib.rs.html#19" class="src">Source</a>

Expand description

DataFrame module.

## Modules<a href="https://docs.rs/polars/latest/polars/frame/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/builder/index.html" class="mod" title="mod polars::frame::builder">builder</a>

<a href="https://docs.rs/polars/latest/polars/frame/column/index.html" class="mod" title="mod polars::frame::column">column</a>

<a href="https://docs.rs/polars/latest/polars/frame/explode/index.html" class="mod" title="mod polars::frame::explode">explode</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/index.html" class="mod" title="mod polars::frame::group_by">group_by</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/index.html" class="mod" title="mod polars::frame::row">row</a>

## Structs<a href="https://docs.rs/polars/latest/polars/frame/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html" class="struct" title="struct polars::frame::DataFrame">DataFrame</a>

A contiguous growable collection of `Series` that have the same length.

<a href="https://docs.rs/polars/latest/polars/frame/struct.PhysRecordBatchIter.html" class="struct" title="struct polars::frame::PhysRecordBatchIter">PhysRecordBatchIter</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.RecordBatchIter.html" class="struct" title="struct polars::frame::RecordBatchIter">RecordBatchIter</a>

## Enums<a href="https://docs.rs/polars/latest/polars/frame/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/enum.UniqueKeepStrategy.html" class="enum" title="enum polars::frame::UniqueKeepStrategy">UniqueKeepStrategy</a>

## Functions<a href="https://docs.rs/polars/latest/polars/frame/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/fn.chunk_df_for_writing.html" class="fn" title="fn polars::frame::chunk_df_for_writing">chunk_df_for_writing</a>  
Split DataFrame into chunks in preparation for writing. The chunks have a maximum number of rows per chunk to ensure reasonable memory efficiency when reading the resulting file, and a minimum size per chunk to ensure reasonable performance when writing.
