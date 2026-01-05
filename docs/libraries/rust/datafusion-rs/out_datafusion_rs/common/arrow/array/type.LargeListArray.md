# Type Alias LargeListArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/list_array.rs.html#617" class="src">Source</a>

``` rust
pub type LargeListArray = GenericListArray<i64>;
```

Expand description

A [`GenericListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html "struct datafusion::common::arrow::array::GenericListArray") of variable size lists, storing offsets as `i64`.

See [`LargeListBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListBuilder.html "type datafusion::common::arrow::array::LargeListBuilder") for how to construct a [`LargeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListArray.html "type datafusion::common::arrow::array::LargeListArray")

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct LargeListArray { /* private fields */ }
```
