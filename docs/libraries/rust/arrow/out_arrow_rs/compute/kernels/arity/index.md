# Module arity Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/lib.rs.html#29" class="src">Source</a>

Expand description

Kernels for operating on [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")s

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.binary.html" class="fn" title="fn arrow::compute::kernels::arity::binary">binary</a>  
Allies a binary infallable function to two [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")s, producing a new [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.binary_mut.html" class="fn" title="fn arrow::compute::kernels::arity::binary_mut">binary_mut</a>  
Applies a binary and infallible function to values in two arrays, replacing the values in the first array in place.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.try_binary.html" class="fn" title="fn arrow::compute::kernels::arity::try_binary">try_binary</a>  
Applies the provided fallible binary operation across `a` and `b`.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.try_binary_mut.html" class="fn" title="fn arrow::compute::kernels::arity::try_binary_mut">try_binary_mut</a>  
Applies the provided fallible binary operation across `a` and `b` by mutating the mutable [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") `a` with the results.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.try_unary.html" class="fn" title="fn arrow::compute::kernels::arity::try_unary">try_unary</a>  
See [`PrimitiveArray::try_unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.try_unary "method arrow::array::PrimitiveArray::try_unary")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.try_unary_mut.html" class="fn" title="fn arrow::compute::kernels::arity::try_unary_mut">try_unary_mut</a>  
See [`PrimitiveArray::try_unary_mut`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.try_unary_mut "method arrow::array::PrimitiveArray::try_unary_mut")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.unary.html" class="fn" title="fn arrow::compute::kernels::arity::unary">unary</a>  
See [`PrimitiveArray::unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary "method arrow::array::PrimitiveArray::unary")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/fn.unary_mut.html" class="fn" title="fn arrow::compute::kernels::arity::unary_mut">unary_mut</a>  
See [`PrimitiveArray::unary_mut`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary_mut "method arrow::array::PrimitiveArray::unary_mut")
