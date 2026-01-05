# Function deserialize Copy item path

<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/src/polars_parquet/arrow/read/statistics.rs.html#549-552" class="src">Source</a>

``` rust
pub fn deserialize<'a>(
    field: &Field,
    columns: &mut impl ExactSizeIterator<Item = &'a ColumnChunkMetadata>,
) -> Result<Option<Statistics>, ParquetError>
```

Available on **crate feature `polars-io`** only.

Expand description

Deserializes the statistics in the column chunks from a single `row_group` into [`Statistics`](https://docs.rs/polars/latest/polars/prelude/enum.ParquetStatistics.html "enum polars::prelude::ParquetStatistics") associated from `field`’s name.

## <a href="https://docs.rs/polars/latest/polars/prelude/fn.deserialize.html#errors" class="doc-anchor">§</a>Errors

This function errors if the deserialization of the statistics fails (e.g. invalid utf8)
