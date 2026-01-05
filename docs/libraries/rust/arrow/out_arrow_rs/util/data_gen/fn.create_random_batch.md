# Function create_random_batch Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/data_gen.rs.html#37-54" class="src">Source</a>

``` rust
pub fn create_random_batch(
    schema: SchemaRef,
    size: usize,
    null_density: f32,
    true_density: f32,
) -> Result<RecordBatch>
```

Available on **crate feature `test_utils`** only.

Expand description

Create a random [RecordBatch](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") from a schema
