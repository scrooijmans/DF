# Function infer_schema Copy item path

<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/src/polars_parquet/arrow/read/schema/mod.rs.html#44" class="src">Source</a>

``` rust
pub fn infer_schema(
    file_metadata: &FileMetadata,
) -> Result<Schema<Field>, PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Infers a [`ArrowSchema`](https://docs.rs/polars/latest/polars/prelude/type.ArrowSchema.html "type polars::prelude::ArrowSchema") from parquet’s [`FileMetadata`](https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html "struct polars::prelude::FileMetadata").

This first looks for the metadata key `"ARROW:schema"`; if it does not exist, it converts the Parquet types declared in the file’s Parquet schema to Arrow’s equivalent.

## <a href="https://docs.rs/polars/latest/polars/prelude/fn.infer_schema.html#error" class="doc-anchor">§</a>Error

This function errors iff the key `"ARROW:schema"` exists but is not correctly encoded, indicating that the file’s arrow metadata was incorrectly written.
