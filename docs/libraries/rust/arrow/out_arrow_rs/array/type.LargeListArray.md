# Type Alias LargeListArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/list_array.rs.html#627" class="src">Source</a>

``` rust
pub type LargeListArray = GenericListArray<i64>;
```

Expand description

A [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") of variable size lists, storing offsets as `i64`.

See [`LargeListBuilder`](https://docs.rs/arrow/latest/arrow/array/type.LargeListBuilder.html "type arrow::array::LargeListBuilder") for how to construct a [`LargeListArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html "type arrow::array::LargeListArray")

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct LargeListArray { /* private fields */ }
```
