# Module aggregate Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/lib.rs.html#26" class="src">Source</a>

Expand description

Defines aggregations over Arrow arrays.

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.bit_and.html" class="fn" title="fn arrow::compute::kernels::aggregate::bit_and">bit_and</a>  
Returns the bitwise and of all non-null input values.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.bit_or.html" class="fn" title="fn arrow::compute::kernels::aggregate::bit_or">bit_or</a>  
Returns the bitwise or of all non-null input values.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.bit_xor.html" class="fn" title="fn arrow::compute::kernels::aggregate::bit_xor">bit_xor</a>  
Returns the bitwise xor of all non-null input values.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.bool_and.html" class="fn" title="fn arrow::compute::kernels::aggregate::bool_and">bool_and</a>  
Returns true if all non-null input values are true, otherwise false.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.bool_or.html" class="fn" title="fn arrow::compute::kernels::aggregate::bool_or">bool_or</a>  
Returns true if any non-null input value is true, otherwise false.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.max.html" class="fn" title="fn arrow::compute::kernels::aggregate::max">max</a>  
Returns the maximum value in the array, according to the natural order. For floating point arrays any NaN values are considered to be greater than any other non-null value

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.max_array.html" class="fn" title="fn arrow::compute::kernels::aggregate::max_array">max_array</a>  
Returns the max of values in the array of `ArrowNumericType` type, or dictionary array with value of `ArrowNumericType` type.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.max_binary.html" class="fn" title="fn arrow::compute::kernels::aggregate::max_binary">max_binary</a>  
Returns the maximum value in the binary array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.max_binary_view.html" class="fn" title="fn arrow::compute::kernels::aggregate::max_binary_view">max_binary_view</a>  
Returns the maximum value in the binary view array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.max_boolean.html" class="fn" title="fn arrow::compute::kernels::aggregate::max_boolean">max_boolean</a>  
Returns the maximum value in the boolean array

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.max_fixed_size_binary.html" class="fn" title="fn arrow::compute::kernels::aggregate::max_fixed_size_binary">max_fixed_size_binary</a>  
Returns the maximum value in the fixed size binary array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.max_string.html" class="fn" title="fn arrow::compute::kernels::aggregate::max_string">max_string</a>  
Returns the maximum value in the string array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.max_string_view.html" class="fn" title="fn arrow::compute::kernels::aggregate::max_string_view">max_string_view</a>  
Returns the maximum value in the string view array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.min.html" class="fn" title="fn arrow::compute::kernels::aggregate::min">min</a>  
Returns the minimum value in the array, according to the natural order. For floating point arrays any NaN values are considered to be greater than any other non-null value

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.min_array.html" class="fn" title="fn arrow::compute::kernels::aggregate::min_array">min_array</a>  
Returns the min of values in the array of `ArrowNumericType` type, or dictionary array with value of `ArrowNumericType` type.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.min_binary.html" class="fn" title="fn arrow::compute::kernels::aggregate::min_binary">min_binary</a>  
Returns the minimum value in the binary array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.min_binary_view.html" class="fn" title="fn arrow::compute::kernels::aggregate::min_binary_view">min_binary_view</a>  
Returns the minimum value in the binary view array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.min_boolean.html" class="fn" title="fn arrow::compute::kernels::aggregate::min_boolean">min_boolean</a>  
Returns the minimum value in the boolean array.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.min_fixed_size_binary.html" class="fn" title="fn arrow::compute::kernels::aggregate::min_fixed_size_binary">min_fixed_size_binary</a>  
Returns the minimum value in the fixed size binary array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.min_string.html" class="fn" title="fn arrow::compute::kernels::aggregate::min_string">min_string</a>  
Returns the minimum value in the string array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.min_string_view.html" class="fn" title="fn arrow::compute::kernels::aggregate::min_string_view">min_string_view</a>  
Returns the minimum value in the string view array, according to the natural order.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.sum.html" class="fn" title="fn arrow::compute::kernels::aggregate::sum">sum</a>  
Returns the sum of values in the primitive array.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.sum_array.html" class="fn" title="fn arrow::compute::kernels::aggregate::sum_array">sum_array</a>  
Returns the sum of values in the array.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.sum_array_checked.html" class="fn" title="fn arrow::compute::kernels::aggregate::sum_array_checked">sum_array_checked</a>  
Returns the sum of values in the array.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/fn.sum_checked.html" class="fn" title="fn arrow::compute::kernels::aggregate::sum_checked">sum_checked</a>  
Returns the sum of values in the primitive array.
