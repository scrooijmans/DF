# Function create_random_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/data_gen.rs.html#66-234" class="src">Source</a>

``` rust
pub fn create_random_array(
    field: &Field,
    size: usize,
    null_density: f32,
    true_density: f32,
) -> Result<ArrayRef>
```

Available on **crate feature `test_utils`** only.

Expand description

Create a random [ArrayRef](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") from a [DataType](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") with a length, null density and true density (for [BooleanArray](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray")).

## <a href="https://docs.rs/arrow/latest/arrow/util/data_gen/fn.create_random_array.html#arguments" class="doc-anchor">§</a>Arguments

- `field` - The field containing the data type for which to create a random array
- `size` - The number of elements in the generated array
- `null_density` - The approximate fraction of null values in the resulting array (0.0 to 1.0)
- `true_density` - The approximate fraction of true values in boolean arrays (0.0 to 1.0)
