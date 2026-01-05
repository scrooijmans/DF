# Module cast Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/lib.rs.html#26" class="src">Source</a>

Expand description

Cast kernels to convert [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") between supported datatypes.

See [`cast_with_options`](https://docs.rs/arrow/latest/arrow/compute/fn.cast_with_options.html "fn arrow::compute::cast_with_options") for more information on specific conversions.

Example:

``` rust
// int32 to float64
let a = Int32Array::from(vec![5, 6, 7]);
let b = cast(&a, &DataType::Float64).unwrap();
let c = b.as_primitive::<Float64Type>();
assert_eq!(5.0, c.value(0));
assert_eq!(6.0, c.value(1));
assert_eq!(7.0, c.value(2));
```

## Structs<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast/struct.CastOptions.html" class="struct" title="struct arrow::compute::kernels::cast::CastOptions">CastOptions</a>  
CastOptions provides a way to override the default cast behaviors

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast/fn.can_cast_types.html" class="fn" title="fn arrow::compute::kernels::cast::can_cast_types">can_cast_types</a>  
Return true if a value of type `from_type` can be cast into a value of `to_type`.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast/fn.cast.html" class="fn" title="fn arrow::compute::kernels::cast::cast">cast</a>  
Cast `array` to the provided data type and return a new Array with type `to_type`, if possible.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast/fn.cast_with_options.html" class="fn" title="fn arrow::compute::kernels::cast::cast_with_options">cast_with_options</a>  
Try to cast `array` to `to_type` if possible.
