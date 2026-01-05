# Function max_array Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#638-641" class="src">Source</a>

``` rust
pub fn max_array<T, A>(array: A) -> Option<<T as ArrowPrimitiveType>::Native>where
    A: ArrayAccessor<Item = <T as ArrowPrimitiveType>::Native>,
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: ArrowNativeTypeOp,
```

Expand description

Returns the max of values in the array of `ArrowNumericType` type, or dictionary array with value of `ArrowNumericType` type.
