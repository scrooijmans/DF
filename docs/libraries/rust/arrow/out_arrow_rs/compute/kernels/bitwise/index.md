# Module bitwise Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/lib.rs.html#30" class="src">Source</a>

Expand description

Module contains bitwise operations on arrays

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_and.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_and">bitwise_and</a>  
Perform `left & right` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_and_not.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_and_not">bitwise_and_not</a>  
Perform `left & !right` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_and_scalar.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_and_scalar">bitwise_and_scalar</a>  
Perform bitwise `and` every value in an array with the scalar. If any value in the array is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_not.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_not">bitwise_not</a>  
Perform `!array` operation on array. If array value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_or.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_or">bitwise_or</a>  
Perform `left | right` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_or_scalar.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_or_scalar">bitwise_or_scalar</a>  
Perform bitwise `or` every value in an array with the scalar. If any value in the array is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_shift_left.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_shift_left">bitwise_shift_left</a>  
Perform bitwise `left << right` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_shift_left_scalar.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_shift_left_scalar">bitwise_shift_left_scalar</a>  
Perform bitwise `left << right` every value in an array with the scalar. If any value in the array is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_shift_right.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_shift_right">bitwise_shift_right</a>  
Perform bitwise `left >> right` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_shift_right_scalar.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_shift_right_scalar">bitwise_shift_right_scalar</a>  
Perform bitwise `left >> right` every value in an array with the scalar. If any value in the array is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_xor.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_xor">bitwise_xor</a>  
Perform `left ^ right` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/fn.bitwise_xor_scalar.html" class="fn" title="fn arrow::compute::kernels::bitwise::bitwise_xor_scalar">bitwise_xor_scalar</a>  
Perform bitwise `xor` every value in an array with the scalar. If any value in the array is null then the result is also null.
