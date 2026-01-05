# Module cast Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#39" class="src">Source</a>

Expand description

This module provides DataFusion specific casting functions that provide error handling. They are intended to “never fail” but provide an error message rather than a panic, as the corresponding kernels in arrow-rs such as `as_boolean_array` do.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_binary_array.html" class="fn" title="fn datafusion::common::cast::as_binary_array">as_binary_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_binary_view_array.html" class="fn" title="fn datafusion::common::cast::as_binary_view_array">as_binary_view_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_boolean_array.html" class="fn" title="fn datafusion::common::cast::as_boolean_array">as_boolean_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_date32_array.html" class="fn" title="fn datafusion::common::cast::as_date32_array">as_date32_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_date64_array.html" class="fn" title="fn datafusion::common::cast::as_date64_array">as_date64_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_decimal128_array.html" class="fn" title="fn datafusion::common::cast::as_decimal128_array">as_decimal128_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_decimal256_array.html" class="fn" title="fn datafusion::common::cast::as_decimal256_array">as_decimal256_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_dictionary_array.html" class="fn" title="fn datafusion::common::cast::as_dictionary_array">as_dictionary_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_duration_microsecond_array.html" class="fn" title="fn datafusion::common::cast::as_duration_microsecond_array">as_duration_microsecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_duration_millisecond_array.html" class="fn" title="fn datafusion::common::cast::as_duration_millisecond_array">as_duration_millisecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_duration_nanosecond_array.html" class="fn" title="fn datafusion::common::cast::as_duration_nanosecond_array">as_duration_nanosecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_duration_second_array.html" class="fn" title="fn datafusion::common::cast::as_duration_second_array">as_duration_second_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_fixed_size_binary_array.html" class="fn" title="fn datafusion::common::cast::as_fixed_size_binary_array">as_fixed_size_binary_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_fixed_size_list_array.html" class="fn" title="fn datafusion::common::cast::as_fixed_size_list_array">as_fixed_size_list_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_float16_array.html" class="fn" title="fn datafusion::common::cast::as_float16_array">as_float16_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_float32_array.html" class="fn" title="fn datafusion::common::cast::as_float32_array">as_float32_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_float64_array.html" class="fn" title="fn datafusion::common::cast::as_float64_array">as_float64_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_generic_binary_array.html" class="fn" title="fn datafusion::common::cast::as_generic_binary_array">as_generic_binary_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_generic_list_array.html" class="fn" title="fn datafusion::common::cast::as_generic_list_array">as_generic_list_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_generic_string_array.html" class="fn" title="fn datafusion::common::cast::as_generic_string_array">as_generic_string_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_int8_array.html" class="fn" title="fn datafusion::common::cast::as_int8_array">as_int8_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_int16_array.html" class="fn" title="fn datafusion::common::cast::as_int16_array">as_int16_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_int32_array.html" class="fn" title="fn datafusion::common::cast::as_int32_array">as_int32_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_int64_array.html" class="fn" title="fn datafusion::common::cast::as_int64_array">as_int64_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_interval_dt_array.html" class="fn" title="fn datafusion::common::cast::as_interval_dt_array">as_interval_dt_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_interval_mdn_array.html" class="fn" title="fn datafusion::common::cast::as_interval_mdn_array">as_interval_mdn_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_interval_ym_array.html" class="fn" title="fn datafusion::common::cast::as_interval_ym_array">as_interval_ym_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_large_binary_array.html" class="fn" title="fn datafusion::common::cast::as_large_binary_array">as_large_binary_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_large_list_array.html" class="fn" title="fn datafusion::common::cast::as_large_list_array">as_large_list_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_large_string_array.html" class="fn" title="fn datafusion::common::cast::as_large_string_array">as_large_string_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_list_array.html" class="fn" title="fn datafusion::common::cast::as_list_array">as_list_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_map_array.html" class="fn" title="fn datafusion::common::cast::as_map_array">as_map_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_null_array.html" class="fn" title="fn datafusion::common::cast::as_null_array">as_null_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_primitive_array.html" class="fn" title="fn datafusion::common::cast::as_primitive_array">as_primitive_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_string_array.html" class="fn" title="fn datafusion::common::cast::as_string_array">as_string_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_string_view_array.html" class="fn" title="fn datafusion::common::cast::as_string_view_array">as_string_view_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_struct_array.html" class="fn" title="fn datafusion::common::cast::as_struct_array">as_struct_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_time32_millisecond_array.html" class="fn" title="fn datafusion::common::cast::as_time32_millisecond_array">as_time32_millisecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_time32_second_array.html" class="fn" title="fn datafusion::common::cast::as_time32_second_array">as_time32_second_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_time64_microsecond_array.html" class="fn" title="fn datafusion::common::cast::as_time64_microsecond_array">as_time64_microsecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_time64_nanosecond_array.html" class="fn" title="fn datafusion::common::cast::as_time64_nanosecond_array">as_time64_nanosecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_timestamp_microsecond_array.html" class="fn" title="fn datafusion::common::cast::as_timestamp_microsecond_array">as_timestamp_microsecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_timestamp_millisecond_array.html" class="fn" title="fn datafusion::common::cast::as_timestamp_millisecond_array">as_timestamp_millisecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_timestamp_nanosecond_array.html" class="fn" title="fn datafusion::common::cast::as_timestamp_nanosecond_array">as_timestamp_nanosecond_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_timestamp_second_array.html" class="fn" title="fn datafusion::common::cast::as_timestamp_second_array">as_timestamp_second_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_uint8_array.html" class="fn" title="fn datafusion::common::cast::as_uint8_array">as_uint8_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_uint16_array.html" class="fn" title="fn datafusion::common::cast::as_uint16_array">as_uint16_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_uint32_array.html" class="fn" title="fn datafusion::common::cast::as_uint32_array">as_uint32_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_uint64_array.html" class="fn" title="fn datafusion::common::cast::as_uint64_array">as_uint64_array</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cast/fn.as_union_array.html" class="fn" title="fn datafusion::common::cast::as_union_array">as_union_array</a>
