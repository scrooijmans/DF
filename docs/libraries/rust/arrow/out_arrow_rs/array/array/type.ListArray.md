# Type Alias ListArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/list_array.rs.html#622" class="src">Source</a>

``` rust
pub type ListArray = GenericListArray<i32>;
```

Expand description

A [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") of variable size lists, storing offsets as `i32`.

See [`ListBuilder`](https://docs.rs/arrow/latest/arrow/array/type.ListBuilder.html "type arrow::array::ListBuilder") for how to construct a [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray")

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.ListArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct ListArray { /* private fields */ }
```
