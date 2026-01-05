# Type Alias ArrowSchema Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/datatypes/schema.rs.html#10" class="src">Source</a>

``` rust
pub type ArrowSchema = Schema<Field>;
```

Expand description

An ordered sequence of [`Field`](https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html "struct polars::prelude::ArrowField")s

[`ArrowSchema`](https://docs.rs/polars/latest/polars/prelude/type.ArrowSchema.html "type polars::prelude::ArrowSchema") is an abstraction used to read from, and write to, Arrow IPC format, Apache Parquet, and Apache Avro. All these formats have a concept of a schema with fields and metadata.

## Aliased Type<a href="https://docs.rs/polars/latest/polars/prelude/type.ArrowSchema.html#aliased-type" class="anchor">§</a>

``` rust
pub struct ArrowSchema { /* private fields */ }
```
