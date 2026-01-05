# Module substring Copy item path

<a href="https://docs.rs/arrow-string/56.2.0/x86_64-unknown-linux-gnu/src/arrow_string/lib.rs.html#34" class="src">Source</a>

Expand description

Defines kernel to extract a substring of an Array Supported array types: [GenericStringArray](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray"), [GenericBinaryArray](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray"), [FixedSizeBinaryArray](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray"), [DictionaryArray](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray")

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring.html" class="fn" title="fn arrow::compute::kernels::substring::substring">substring</a>  
Returns an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") with substrings of all the elements in `array`.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/fn.substring_by_char.html" class="fn" title="fn arrow::compute::kernels::substring::substring_by_char">substring_by_char</a>  
Substrings based on character index
