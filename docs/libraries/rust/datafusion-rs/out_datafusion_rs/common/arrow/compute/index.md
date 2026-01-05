# Module compute Copy item path

<a href="https://docs.rs/arrow/56.0.0/x86_64-unknown-linux-gnu/src/arrow/lib.rs.html#386" class="src">Source</a>

Expand description

Computation kernels on Arrow Arrays

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/index.html" class="mod" title="mod datafusion::common::arrow::compute::kernels">kernels</a>  
Computation kernels on Arrow Arrays

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.BatchCoalescer.html" class="struct" title="struct datafusion::common::arrow::compute::BatchCoalescer">BatchCoalescer</a>  
Concatenate multiple [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.CastOptions.html" class="struct" title="struct datafusion::common::arrow::compute::CastOptions">CastOptions</a>  
CastOptions provides a way to override the default cast behaviors

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.FilterBuilder.html" class="struct" title="struct datafusion::common::arrow::compute::FilterBuilder">FilterBuilder</a>  
A builder to construct [`FilterPredicate`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.FilterPredicate.html "struct datafusion::common::arrow::compute::FilterPredicate")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.FilterPredicate.html" class="struct" title="struct datafusion::common::arrow::compute::FilterPredicate">FilterPredicate</a>  
A filtering predicate that can be applied to an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.FixedLexicographicalComparator.html" class="struct" title="struct datafusion::common::arrow::compute::FixedLexicographicalComparator">FixedLexicographicalComparator</a>  
A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data at given two indices. This version of the comparator is for compile-time constant number of columns. The lifetime is the same at the data wrapped.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.LexicographicalComparator.html" class="struct" title="struct datafusion::common::arrow::compute::LexicographicalComparator">LexicographicalComparator</a>  
A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data at given two indices. The lifetime is the same at the data wrapped.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.Partitions.html" class="struct" title="struct datafusion::common::arrow::compute::Partitions">Partitions</a>  
A computed set of partitions, see [`partition`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.partition.html "fn datafusion::common::arrow::compute::partition")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SlicesIterator.html" class="struct" title="struct datafusion::common::arrow::compute::SlicesIterator">SlicesIterator</a>  
An iterator of `(usize, usize)` each representing an interval `[start, end)` whose slots of a bitmap [Buffer](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html "struct datafusion::common::arrow::buffer::Buffer") are true.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortColumn.html" class="struct" title="struct datafusion::common::arrow::compute::SortColumn">SortColumn</a>  
One column to be used in lexicographical sort

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.SortOptions.html" class="struct" title="struct datafusion::common::arrow::compute::SortOptions">SortOptions</a>  
Options that define the sort order of a given column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.TakeOptions.html" class="struct" title="struct datafusion::common::arrow::compute::TakeOptions">TakeOptions</a>  
Options that define how `take` should behave

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/enum.DatePart.html" class="enum" title="enum datafusion::common::arrow::compute::DatePart">DatePart</a>  
Valid parts to extract from date/time/timestamp arrays.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/trait.StringArrayType.html" class="trait" title="trait datafusion::common::arrow::compute::StringArrayType">StringArrayType</a>  
A trait for Arrow String Arrays, currently three types are supported:

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.and.html" class="fn" title="fn datafusion::common::arrow::compute::and">and</a>  
Performs `AND` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.and_kleene.html" class="fn" title="fn datafusion::common::arrow::compute::and_kleene">and_kleene</a>  
Logical ‘and’ boolean values with Kleene logic

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.and_not.html" class="fn" title="fn datafusion::common::arrow::compute::and_not">and_not</a>  
Performs `AND_NOT` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.binary.html" class="fn" title="fn datafusion::common::arrow::compute::binary">binary</a>  
Allies a binary infallable function to two [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray")s, producing a new [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.binary_mut.html" class="fn" title="fn datafusion::common::arrow::compute::binary_mut">binary_mut</a>  
Applies a binary and infallible function to values in two arrays, replacing the values in the first array in place.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.bit_and.html" class="fn" title="fn datafusion::common::arrow::compute::bit_and">bit_and</a>  
Returns the bitwise and of all non-null input values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.bit_or.html" class="fn" title="fn datafusion::common::arrow::compute::bit_or">bit_or</a>  
Returns the bitwise or of all non-null input values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.bit_xor.html" class="fn" title="fn datafusion::common::arrow::compute::bit_xor">bit_xor</a>  
Returns the bitwise xor of all non-null input values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.bool_and.html" class="fn" title="fn datafusion::common::arrow::compute::bool_and">bool_and</a>  
Returns true if all non-null input values are true, otherwise false.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.bool_or.html" class="fn" title="fn datafusion::common::arrow::compute::bool_or">bool_or</a>  
Returns true if any non-null input value is true, otherwise false.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.can_cast_types.html" class="fn" title="fn datafusion::common::arrow::compute::can_cast_types">can_cast_types</a>  
Return true if a value of type `from_type` can be cast into a value of `to_type`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.cast.html" class="fn" title="fn datafusion::common::arrow::compute::cast">cast</a>  
Cast `array` to the provided data type and return a new Array with type `to_type`, if possible.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.cast_with_options.html" class="fn" title="fn datafusion::common::arrow::compute::cast_with_options">cast_with_options</a>  
Try to cast `array` to `to_type` if possible.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.concat.html" class="fn" title="fn datafusion::common::arrow::compute::concat">concat</a>  
Concatenate multiple [Array](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") of the same type into a single [ArrayRef](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.concat_batches.html" class="fn" title="fn datafusion::common::arrow::compute::concat_batches">concat_batches</a>  
Concatenates `batches` together into a single [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.contains.html" class="fn" title="fn datafusion::common::arrow::compute::contains">contains</a>  
Perform SQL `CONTAINS(left, right)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.date_part.html" class="fn" title="fn datafusion::common::arrow::compute::date_part">date_part</a>  
Given an array, return a new array with the extracted [`DatePart`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/enum.DatePart.html "enum datafusion::common::arrow::compute::DatePart") as signed 32-bit integer values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.ends_with.html" class="fn" title="fn datafusion::common::arrow::compute::ends_with">ends_with</a>  
Perform SQL `ENDSWITH(left, right)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.filter.html" class="fn" title="fn datafusion::common::arrow::compute::filter">filter</a>  
Returns a filtered `values` [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") where the corresponding elements of `predicate` are `true`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.filter_record_batch.html" class="fn" title="fn datafusion::common::arrow::compute::filter_record_batch">filter_record_batch</a>  
Returns a filtered [RecordBatch](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") where the corresponding elements of `predicate` are true.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.ilike.html" class="fn" title="fn datafusion::common::arrow::compute::ilike">ilike</a>  
Perform SQL `left ILIKE right`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.in_list.html" class="fn" title="fn datafusion::common::arrow::compute::in_list">in_list</a>  
Checks if a [`GenericListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html "struct datafusion::common::arrow::array::GenericListArray") contains a value in the [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.in_list_utf8.html" class="fn" title="fn datafusion::common::arrow::compute::in_list_utf8">in_list_utf8</a>  
Checks if a [`GenericListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html "struct datafusion::common::arrow::array::GenericListArray") contains a value in the [`GenericStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericStringArray.html "type datafusion::common::arrow::array::GenericStringArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.interleave.html" class="fn" title="fn datafusion::common::arrow::compute::interleave">interleave</a>  
Takes elements by index from a list of [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array"), creating a new [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") from those values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.interleave_record_batch.html" class="fn" title="fn datafusion::common::arrow::compute::interleave_record_batch">interleave_record_batch</a>  
Interleave rows by index from multiple [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") instances and return a new [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.is_not_null.html" class="fn" title="fn datafusion::common::arrow::compute::is_not_null">is_not_null</a>  
Returns a non-null [BooleanArray](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html "struct datafusion::common::arrow::array::BooleanArray") with whether each value of the array is not null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.is_null.html" class="fn" title="fn datafusion::common::arrow::compute::is_null">is_null</a>  
Returns a non-null [BooleanArray](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html "struct datafusion::common::arrow::array::BooleanArray") with whether each value of the array is null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.lexsort.html" class="fn" title="fn datafusion::common::arrow::compute::lexsort">lexsort</a>  
Sort a list of `ArrayRef` using `SortOptions` provided for each array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.lexsort_to_indices.html" class="fn" title="fn datafusion::common::arrow::compute::lexsort_to_indices">lexsort_to_indices</a>  
Sort elements lexicographically from a list of `ArrayRef` into an unsigned integer (`UInt32Array`) of indices.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.like.html" class="fn" title="fn datafusion::common::arrow::compute::like">like</a>  
Perform SQL `left LIKE right`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max.html" class="fn" title="fn datafusion::common::arrow::compute::max">max</a>  
Returns the maximum value in the array, according to the natural order. For floating point arrays any NaN values are considered to be greater than any other non-null value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max_array.html" class="fn" title="fn datafusion::common::arrow::compute::max_array">max_array</a>  
Returns the max of values in the array of `ArrowNumericType` type, or dictionary array with value of `ArrowNumericType` type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max_binary.html" class="fn" title="fn datafusion::common::arrow::compute::max_binary">max_binary</a>  
Returns the maximum value in the binary array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max_binary_view.html" class="fn" title="fn datafusion::common::arrow::compute::max_binary_view">max_binary_view</a>  
Returns the maximum value in the binary view array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max_boolean.html" class="fn" title="fn datafusion::common::arrow::compute::max_boolean">max_boolean</a>  
Returns the maximum value in the boolean array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max_fixed_size_binary.html" class="fn" title="fn datafusion::common::arrow::compute::max_fixed_size_binary">max_fixed_size_binary</a>  
Returns the maximum value in the fixed size binary array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max_string.html" class="fn" title="fn datafusion::common::arrow::compute::max_string">max_string</a>  
Returns the maximum value in the string array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max_string_view.html" class="fn" title="fn datafusion::common::arrow::compute::max_string_view">max_string_view</a>  
Returns the maximum value in the string view array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min.html" class="fn" title="fn datafusion::common::arrow::compute::min">min</a>  
Returns the minimum value in the array, according to the natural order. For floating point arrays any NaN values are considered to be greater than any other non-null value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min_array.html" class="fn" title="fn datafusion::common::arrow::compute::min_array">min_array</a>  
Returns the min of values in the array of `ArrowNumericType` type, or dictionary array with value of `ArrowNumericType` type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min_binary.html" class="fn" title="fn datafusion::common::arrow::compute::min_binary">min_binary</a>  
Returns the minimum value in the binary array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min_binary_view.html" class="fn" title="fn datafusion::common::arrow::compute::min_binary_view">min_binary_view</a>  
Returns the minimum value in the binary view array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min_boolean.html" class="fn" title="fn datafusion::common::arrow::compute::min_boolean">min_boolean</a>  
Returns the minimum value in the boolean array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min_fixed_size_binary.html" class="fn" title="fn datafusion::common::arrow::compute::min_fixed_size_binary">min_fixed_size_binary</a>  
Returns the minimum value in the fixed size binary array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min_string.html" class="fn" title="fn datafusion::common::arrow::compute::min_string">min_string</a>  
Returns the minimum value in the string array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min_string_view.html" class="fn" title="fn datafusion::common::arrow::compute::min_string_view">min_string_view</a>  
Returns the minimum value in the string view array, according to the natural order.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.multiply_fixed_point.html" class="fn" title="fn datafusion::common::arrow::compute::multiply_fixed_point">multiply_fixed_point</a>  
Perform `left * right` operation on two decimal arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.multiply_fixed_point_checked.html" class="fn" title="fn datafusion::common::arrow::compute::multiply_fixed_point_checked">multiply_fixed_point_checked</a>  
Perform `left * right` operation on two decimal arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.multiply_fixed_point_dyn.html" class="fn" title="fn datafusion::common::arrow::compute::multiply_fixed_point_dyn">multiply_fixed_point_dyn</a>  
Perform `left * right` operation on two decimal arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.nilike.html" class="fn" title="fn datafusion::common::arrow::compute::nilike">nilike</a>  
Perform SQL `left NOT ILIKE right`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.nlike.html" class="fn" title="fn datafusion::common::arrow::compute::nlike">nlike</a>  
Perform SQL `left NOT LIKE right`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.not.html" class="fn" title="fn datafusion::common::arrow::compute::not">not</a>  
Performs unary `NOT` operation on an arrays. If value is null then the result is also null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.nullif.html" class="fn" title="fn datafusion::common::arrow::compute::nullif">nullif</a>  
Returns a new array with the same values and the validity bit to false where the corresponding element of`right` is true.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.or.html" class="fn" title="fn datafusion::common::arrow::compute::or">or</a>  
Performs `OR` operation on two arrays. If either left or right value is null then the result is also null.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.or_kleene.html" class="fn" title="fn datafusion::common::arrow::compute::or_kleene">or_kleene</a>  
Logical ‘or’ boolean values with Kleene logic

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.partial_sort.html" class="fn" title="fn datafusion::common::arrow::compute::partial_sort">partial_sort</a>  
It’s unstable_sort, may not preserve the order of equal elements

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.partition.html" class="fn" title="fn datafusion::common::arrow::compute::partition">partition</a>  
Given a list of lexicographically sorted columns, computes the [`Partitions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.Partitions.html "struct datafusion::common::arrow::compute::Partitions"), where a partition consists of the set of consecutive rows with equal values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.partition_validity.html" class="fn" title="fn datafusion::common::arrow::compute::partition_validity">partition_validity</a>  
Partition indices of an Arrow array into two categories:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.prep_null_mask_filter.html" class="fn" title="fn datafusion::common::arrow::compute::prep_null_mask_filter">prep_null_mask_filter</a>  
Remove null values by do a bitmask AND operation with null bits and the boolean bits.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.rank.html" class="fn" title="fn datafusion::common::arrow::compute::rank">rank</a>  
Assigns a rank to each value in `array` based on its position in the sorted order

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.regexp_is_match.html" class="fn" title="fn datafusion::common::arrow::compute::regexp_is_match">regexp_is_match</a>  
Return BooleanArray indicating which strings in an array match an array of regular expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.regexp_is_match_scalar.html" class="fn" title="fn datafusion::common::arrow::compute::regexp_is_match_scalar">regexp_is_match_scalar</a>  
Return BooleanArray indicating which strings in an array match a single regular expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.regexp_match.html" class="fn" title="fn datafusion::common::arrow::compute::regexp_match">regexp_match</a>  
Extract all groups matched by a regular expression for a given String array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.shift.html" class="fn" title="fn datafusion::common::arrow::compute::shift">shift</a>  
Shifts array by defined number of items (to left or right) A positive value for `offset` shifts the array to the right a negative value shifts the array to the left.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sort.html" class="fn" title="fn datafusion::common::arrow::compute::sort">sort</a>  
Sort the `ArrayRef` using `SortOptions`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sort_limit.html" class="fn" title="fn datafusion::common::arrow::compute::sort_limit">sort_limit</a>  
Sort the `ArrayRef` partially.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sort_to_indices.html" class="fn" title="fn datafusion::common::arrow::compute::sort_to_indices">sort_to_indices</a>  
Sort elements from `ArrayRef` into an unsigned integer (`UInt32Array`) of indices. Floats are sorted using IEEE 754 totalOrder. `limit` is an option for [partial_sort](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.partial_sort.html "fn datafusion::common::arrow::compute::partial_sort").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.starts_with.html" class="fn" title="fn datafusion::common::arrow::compute::starts_with">starts_with</a>  
Perform SQL `STARTSWITH(left, right)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sum.html" class="fn" title="fn datafusion::common::arrow::compute::sum">sum</a>  
Returns the sum of values in the primitive array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sum_array.html" class="fn" title="fn datafusion::common::arrow::compute::sum_array">sum_array</a>  
Returns the sum of values in the array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sum_array_checked.html" class="fn" title="fn datafusion::common::arrow::compute::sum_array_checked">sum_array_checked</a>  
Returns the sum of values in the array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sum_checked.html" class="fn" title="fn datafusion::common::arrow::compute::sum_checked">sum_checked</a>  
Returns the sum of values in the primitive array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.take.html" class="fn" title="fn datafusion::common::arrow::compute::take">take</a>  
Take elements by index from [Array](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array"), creating a new [Array](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") from those indexes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.take_arrays.html" class="fn" title="fn datafusion::common::arrow::compute::take_arrays">take_arrays</a>  
For each [ArrayRef](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") in the [`Vec<ArrayRef>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"), take elements by index and create a new [`Vec<ArrayRef>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") from those indices.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.take_record_batch.html" class="fn" title="fn datafusion::common::arrow::compute::take_record_batch">take_record_batch</a>  
Take rows by index from [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") and returns a new [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") from those indexes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.try_binary.html" class="fn" title="fn datafusion::common::arrow::compute::try_binary">try_binary</a>  
Applies the provided fallible binary operation across `a` and `b`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.try_binary_mut.html" class="fn" title="fn datafusion::common::arrow::compute::try_binary_mut">try_binary_mut</a>  
Applies the provided fallible binary operation across `a` and `b` by mutating the mutable [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") `a` with the results.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.try_unary.html" class="fn" title="fn datafusion::common::arrow::compute::try_unary">try_unary</a>  
See [`PrimitiveArray::try_unary`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html#method.try_unary "method datafusion::common::arrow::array::PrimitiveArray::try_unary")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.try_unary_mut.html" class="fn" title="fn datafusion::common::arrow::compute::try_unary_mut">try_unary_mut</a>  
See [`PrimitiveArray::try_unary_mut`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html#method.try_unary_mut "method datafusion::common::arrow::array::PrimitiveArray::try_unary_mut")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.unary.html" class="fn" title="fn datafusion::common::arrow::compute::unary">unary</a>  
See [`PrimitiveArray::unary`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html#method.unary "method datafusion::common::arrow::array::PrimitiveArray::unary")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.unary_mut.html" class="fn" title="fn datafusion::common::arrow::compute::unary_mut">unary_mut</a>  
See [`PrimitiveArray::unary_mut`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html#method.unary_mut "method datafusion::common::arrow::array::PrimitiveArray::unary_mut")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.union_extract.html" class="fn" title="fn datafusion::common::arrow::compute::union_extract">union_extract</a>  
Returns the value of the target field when selected, or NULL otherwise.
