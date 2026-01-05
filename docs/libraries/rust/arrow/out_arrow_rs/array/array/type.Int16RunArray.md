# Type Alias Int16RunArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/run_array.rs.html#474" class="src">Source</a>

``` rust
pub type Int16RunArray = RunArray<Int16Type>;
```

Expand description

A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") with `i16` run ends

## <a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int16RunArray.html#example-using-collect" class="doc-anchor">§</a>Example: Using `collect`

``` rust

let array: Int16RunArray = vec!["a", "a", "b", "c", "c"].into_iter().collect();
let values: Arc<dyn Array> = Arc::new(StringArray::from(vec!["a", "b", "c"]));
assert_eq!(array.run_ends().values(), &[2, 3, 5]);
assert_eq!(array.values(), &values);
```

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int16RunArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Int16RunArray { /* private fields */ }
```
