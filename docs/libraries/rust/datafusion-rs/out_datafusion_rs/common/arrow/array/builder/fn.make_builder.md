# Function make_builder Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/mod.rs.html#432" class="src">Source</a>

``` rust
pub fn make_builder(
    datatype: &DataType,
    capacity: usize,
) -> Box<dyn ArrayBuilder>
```

Expand description

Returns a builder with capacity for `capacity` elements of datatype `DataType`.

This function is useful to construct arrays from an arbitrary vectors with known/expected schema.

See comments on [StructBuilder](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructBuilder.html "struct datafusion::common::arrow::array::StructBuilder") for retrieving collection builders built by make_builder.
