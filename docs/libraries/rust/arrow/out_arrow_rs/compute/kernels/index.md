# Module kernels Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/compute/kernels.rs.html#18-34" class="src">Source</a>

Expand description

Computation kernels on Arrow Arrays

## Modules<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/aggregate/index.html" class="mod" title="mod arrow::compute::kernels::aggregate">aggregate</a>  
Defines aggregations over Arrow arrays.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/arity/index.html" class="mod" title="mod arrow::compute::kernels::arity">arity</a>  
Kernels for operating on [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")s

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/bitwise/index.html" class="mod" title="mod arrow::compute::kernels::bitwise">bitwise</a>  
Module contains bitwise operations on arrays

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/boolean/index.html" class="mod" title="mod arrow::compute::kernels::boolean">boolean</a>  
Defines boolean kernels on Arrow `BooleanArray`’s, e.g. `AND`, `OR` and `NOT`.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast/index.html" class="mod" title="mod arrow::compute::kernels::cast">cast</a>  
Cast kernels to convert [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") between supported datatypes.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/index.html" class="mod" title="mod arrow::compute::kernels::cast_utils">cast_utils</a>  
[`Parser`](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/trait.Parser.html "trait arrow::compute::kernels::cast_utils::Parser") implementations for converting strings to Arrow types

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cmp/index.html" class="mod" title="mod arrow::compute::kernels::cmp">cmp</a>  
Comparison kernels for `Array`s.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/index.html" class="mod" title="mod arrow::compute::kernels::coalesce">coalesce</a>  
[`BatchCoalescer`](https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html "struct arrow::compute::BatchCoalescer") concatenates multiple [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es after operations such as [`filter`](https://docs.rs/arrow/latest/arrow/compute/fn.filter.html "fn arrow::compute::filter") and [`take`](https://docs.rs/arrow/latest/arrow/compute/fn.take.html "fn arrow::compute::take").

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/index.html" class="mod" title="mod arrow::compute::kernels::comparison">comparison</a>  
Comparison kernels for `Array`s.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/concat/index.html" class="mod" title="mod arrow::compute::kernels::concat">concat</a>  
Defines concat kernel for `ArrayRef`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/concat_elements/index.html" class="mod" title="mod arrow::compute::kernels::concat_elements">concat_elements</a>  
Provides utility functions for concatenation of elements in arrays.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/filter/index.html" class="mod" title="mod arrow::compute::kernels::filter">filter</a>  
Defines filter kernels

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/interleave/index.html" class="mod" title="mod arrow::compute::kernels::interleave">interleave</a>  
Interleave elements from multiple arrays

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/length/index.html" class="mod" title="mod arrow::compute::kernels::length">length</a>  
Defines kernel for length of string arrays and binary arrays

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/nullif/index.html" class="mod" title="mod arrow::compute::kernels::nullif">nullif</a>  
Implements the `nullif` function for Arrow arrays.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/index.html" class="mod" title="mod arrow::compute::kernels::numeric">numeric</a>  
Defines numeric arithmetic kernels on [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray"), such as [`add`](https://docs.rs/arrow/latest/arrow/compute/kernels/numeric/fn.add.html "fn arrow::compute::kernels::numeric::add")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/partition/index.html" class="mod" title="mod arrow::compute::kernels::partition">partition</a>  
Defines partition kernel for `ArrayRef`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/rank/index.html" class="mod" title="mod arrow::compute::kernels::rank">rank</a>  
Provides `rank` function to assign a rank to each value in an array

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/regexp/index.html" class="mod" title="mod arrow::compute::kernels::regexp">regexp</a>  
Defines kernel to extract substrings based on a regular expression of a \[Large\]StringArray

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/index.html" class="mod" title="mod arrow::compute::kernels::sort">sort</a>  
Defines sort kernel for `ArrayRef`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/substring/index.html" class="mod" title="mod arrow::compute::kernels::substring">substring</a>  
Defines kernel to extract a substring of an Array Supported array types: [GenericStringArray](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray"), [GenericBinaryArray](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray"), [FixedSizeBinaryArray](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray"), [DictionaryArray](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/take/index.html" class="mod" title="mod arrow::compute::kernels::take">take</a>  
Defines take kernel for [Array](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/temporal/index.html" class="mod" title="mod arrow::compute::kernels::temporal">temporal</a>  
Defines temporal kernels for time and date related functions.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/union_extract/index.html" class="mod" title="mod arrow::compute::kernels::union_extract">union_extract</a>  
Defines union_extract kernel for [UnionArray](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/window/index.html" class="mod" title="mod arrow::compute::kernels::window">window</a>  
Defines windowing functions, like `shift`ing

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/zip/index.html" class="mod" title="mod arrow::compute::kernels::zip">zip</a>  
[`zip`](https://docs.rs/arrow/latest/arrow/compute/kernels/zip/fn.zip.html "fn arrow::compute::kernels::zip::zip"): Combine values from two arrays based on boolean mask

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/fn.cast.html" class="fn" title="fn arrow::compute::kernels::cast">cast</a>  
Cast `array` to the provided data type and return a new Array with type `to_type`, if possible.
