# Type Alias ArrayWriter Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/writer/mod.rs.html#196" class="src">Source</a>

``` rust
pub type ArrayWriter<W> = Writer<W, JsonArray>;
```

Expand description

A JSON writer which serializes [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to JSON arrays.

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/type.ArrayWriter.html#aliased-type" class="anchor">§</a>

``` rust
pub struct ArrayWriter<W> { /* private fields */ }
```
