# Module bench_util Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#18-589" class="src">Source</a>

Available on **crate feature `test_utils`** only.

Expand description

Utils to make benchmarking easier

## Functions<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_binary_array.html" class="fn" title="fn arrow::util::bench_util::create_binary_array">create_binary_array</a>  
Creates an random (but fixed-seeded) binary array of a given size and null density

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_boolean_array.html" class="fn" title="fn arrow::util::bench_util::create_boolean_array">create_boolean_array</a>  
Creates a random (but fixed-seeded) array of a given size and null density

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_dict_from_values.html" class="fn" title="fn arrow::util::bench_util::create_dict_from_values">create_dict_from_values</a>  
Creates a random (but fixed-seeded) dictionary array of a given size and null density with the provided values array

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_f16_array.html" class="fn" title="fn arrow::util::bench_util::create_f16_array">create_f16_array</a>  
Creates a random (but fixed-seeded) f16 array of a given size and nan-value density

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_f32_array.html" class="fn" title="fn arrow::util::bench_util::create_f32_array">create_f32_array</a>  
Creates a random (but fixed-seeded) f32 array of a given size and nan-value density

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_f64_array.html" class="fn" title="fn arrow::util::bench_util::create_f64_array">create_f64_array</a>  
Creates a random (but fixed-seeded) f64 array of a given size and nan-value density

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_fsb_array.html" class="fn" title="fn arrow::util::bench_util::create_fsb_array">create_fsb_array</a>  
Creates an random (but fixed-seeded) array of a given size and null density

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_longer_string_array_with_same_prefix.html" class="fn" title="fn arrow::util::bench_util::create_longer_string_array_with_same_prefix">create_longer_string_array_with_same_prefix</a>  
Creates longer string array with same prefix, the prefix should be larger than 4 bytes, and the string length should be larger than 12 bytes so that we can compare the performance with StringViewArray, because StringViewArray has 4 bytes inline for view

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_longer_string_view_array_with_same_prefix.html" class="fn" title="fn arrow::util::bench_util::create_longer_string_view_array_with_same_prefix">create_longer_string_view_array_with_same_prefix</a>  
Creates longer string view array with same prefix, the prefix should be larger than 4 bytes, and the string length should be larger than 12 bytes so that we can compare the StringArray performance with StringViewArray, because StringViewArray has 4 bytes inline for view

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_month_day_nano_array_with_seed.html" class="fn" title="fn arrow::util::bench_util::create_month_day_nano_array_with_seed">create_month_day_nano_array_with_seed</a>  
Creates a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of a given `size` and `null_density` filling it with random [`IntervalMonthDayNano`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html "struct arrow::datatypes::IntervalMonthDayNano") generated using the provided `seed`.

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_primitive_array.html" class="fn" title="fn arrow::util::bench_util::create_primitive_array">create_primitive_array</a>  
Creates an random (but fixed-seeded) array of a given size and null density

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_primitive_array_with_seed.html" class="fn" title="fn arrow::util::bench_util::create_primitive_array_with_seed">create_primitive_array_with_seed</a>  
Creates a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of a given `size` and `null_density` filling it with random numbers generated using the provided `seed`.

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_primitive_run_array.html" class="fn" title="fn arrow::util::bench_util::create_primitive_run_array">create_primitive_run_array</a>  
Create primitive run array for given logical and physical array lengths

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_sparse_dict_from_values.html" class="fn" title="fn arrow::util::bench_util::create_sparse_dict_from_values">create_sparse_dict_from_values</a>  
Creates a random (but fixed-seeded) dictionary array of a given size and null density with the provided values array and key range

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_array.html" class="fn" title="fn arrow::util::bench_util::create_string_array">create_string_array</a>  
Creates a random (but fixed-seeded) string array of a given size and null density.

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_array_for_runs.html" class="fn" title="fn arrow::util::bench_util::create_string_array_for_runs">create_string_array_for_runs</a>  
Create string array to be used by run array builder. The string array will result in run array with physical length of `physical_array_len` and logical length of `logical_array_len`

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_array_with_len.html" class="fn" title="fn arrow::util::bench_util::create_string_array_with_len">create_string_array_with_len</a>  
Creates a random (but fixed-seeded) array of a given size, null density and length

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_array_with_max_len.html" class="fn" title="fn arrow::util::bench_util::create_string_array_with_max_len">create_string_array_with_max_len</a>  
Creates a random (but fixed-seeded) array of rand size with a given max size, null density and length

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_dict_array.html" class="fn" title="fn arrow::util::bench_util::create_string_dict_array">create_string_dict_array</a>  
Creates an random (but fixed-seeded) array of a given size and null density consisting of random 4 character alphanumeric strings

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_view_array.html" class="fn" title="fn arrow::util::bench_util::create_string_view_array">create_string_view_array</a>  
Creates a random (but fixed-seeded) string view array of a given size and null density.

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_view_array_with_fixed_len.html" class="fn" title="fn arrow::util::bench_util::create_string_view_array_with_fixed_len">create_string_view_array_with_fixed_len</a>  
Creates a random (but fixed-seeded) array of a given size, null density and length

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_view_array_with_len.html" class="fn" title="fn arrow::util::bench_util::create_string_view_array_with_len">create_string_view_array_with_len</a>  
Creates a random (but fixed-seeded) array of a given size, null density and length

<a href="https://docs.rs/arrow/latest/arrow/util/bench_util/fn.create_string_view_array_with_max_len.html" class="fn" title="fn arrow::util::bench_util::create_string_view_array_with_max_len">create_string_view_array_with_max_len</a>  
Creates a random (but fixed-seeded) array of rand size with a given max size, null density and length
