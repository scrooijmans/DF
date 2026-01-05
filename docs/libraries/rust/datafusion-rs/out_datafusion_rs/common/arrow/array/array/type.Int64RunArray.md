# Type Alias Int64RunArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/run_array.rs.html#504" class="src">Source</a>

``` rust
pub type Int64RunArray = RunArray<Int64Type>;
```

Expand description

A [`RunArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RunArray.html "struct datafusion::common::arrow::array::RunArray") with `i64` run ends

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/type.Int64RunArray.html#example-using-collect" class="doc-anchor">§</a>Example: Using `collect`

``` rust

let array: Int64RunArray = vec!["a", "a", "b", "c", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.run_ends().values(), &[2, 3, 5]);
assert_eq!(array.values(), &values);
```

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/array/type.Int64RunArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Int64RunArray { /* private fields */ }
```
