# Module utils Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#61" class="src">Source</a>

Expand description

This module provides the bisect function, which implements binary search.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/datafusion_strsim/index.html" class="mod" title="mod datafusion::common::utils::datafusion_strsim">datafusion_strsim</a>  
Adopted from strsim-rs for string similarity metrics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/expr/index.html" class="mod" title="mod datafusion::common::utils::expr">expr</a>  
Expression utilities

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/memory/index.html" class="mod" title="mod datafusion::common::utils::memory">memory</a>  
This module provides a function to estimate the memory size of a HashTable prior to allocation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/proxy/index.html" class="mod" title="mod datafusion::common::utils::proxy">proxy</a>  
[`VecAllocExt`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.VecAllocExt.html "trait datafusion::execution::memory_pool::proxy::VecAllocExt") and [`RawTableAllocExt`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/trait.RawTableAllocExt.html "trait datafusion::execution::memory_pool::proxy::RawTableAllocExt") to help tracking of memory allocations

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/string_utils/index.html" class="mod" title="mod datafusion::common::utils::string_utils">string_utils</a>  
Utilities for working with strings

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/struct.SingleRowListArrayBuilder.html" class="struct" title="struct datafusion::common::utils::SingleRowListArrayBuilder">SingleRowListArrayBuilder</a>  
Creates single element [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray"), [`LargeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListArray.html "type datafusion::common::arrow::array::LargeListArray") and [`FixedSizeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html "struct datafusion::common::arrow::array::FixedSizeListArray") from other arrays

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/enum.ListCoercion.html" class="enum" title="enum datafusion::common::utils::ListCoercion">ListCoercion</a>  
Information about how to coerce lists.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.arrays_into_list_array.html" class="fn" title="fn datafusion::common::utils::arrays_into_list_array">arrays_into_list_array</a>  
Wrap arrays into a single element `ListArray`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.base_type.html" class="fn" title="fn datafusion::common::utils::base_type">base_type</a>  
Get the base type of a data type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.bisect.html" class="fn" title="fn datafusion::common::utils::bisect">bisect</a>  
This function searches for a tuple of given values (`target`) among the given rows (`item_columns`) using the bisection algorithm. It assumes that `item_columns` is sorted according to `sort_options` and returns the insertion index of `target`. Template argument `SIDE` being `true`/`false` means left/right insertion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.coerced_fixed_size_list_to_list.html" class="fn" title="fn datafusion::common::utils::coerced_fixed_size_list_to_list">coerced_fixed_size_list_to_list</a>  
Recursively coerce and `FixedSizeList` elements to `List`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.coerced_type_with_base_type_only.html" class="fn" title="fn datafusion::common::utils::coerced_type_with_base_type_only">coerced_type_with_base_type_only</a>  
A helper function to coerce base type in List.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.combine_limit.html" class="fn" title="fn datafusion::common::utils::combine_limit">combine_limit</a>  
Computes the `skip` and `fetch` parameters of a single limit that would be equivalent to two consecutive limits with the given `skip`/`fetch` parameters.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.compare_rows.html" class="fn" title="fn datafusion::common::utils::compare_rows">compare_rows</a>  
This function compares two tuples depending on the given sort options.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.evaluate_partition_ranges.html" class="fn" title="fn datafusion::common::utils::evaluate_partition_ranges">evaluate_partition_ranges</a>  
Given a list of 0 or more already sorted columns, finds the partition ranges that would partition equally across columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.extract_row_at_idx_to_buf.html" class="fn" title="fn datafusion::common::utils::extract_row_at_idx_to_buf">extract_row_at_idx_to_buf</a>  
Extracts a row at the specified index from a set of columns and stores it in the provided buffer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.find_bisect_point.html" class="fn" title="fn datafusion::common::utils::find_bisect_point">find_bisect_point</a>  
This function searches for a tuple of given values (`target`) among a slice of the given rows (`item_columns`) using the bisection algorithm. The slice starts at the index `low` and ends at the index `high`. The boolean-valued function `compare_fn` specifies whether we bisect on the left (by returning `false`), or on the right (by returning `true`) when we compare the target value with the current value as we iteratively bisect the input.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.find_indices.html" class="fn" title="fn datafusion::common::utils::find_indices">find_indices</a>  
Find indices of each element in `targets` inside `items`. If one of the elements is absent in `items`, returns an error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.fixed_size_list_to_arrays.html" class="fn" title="fn datafusion::common::utils::fixed_size_list_to_arrays">fixed_size_list_to_arrays</a>  
Helper function to convert a FixedSizeListArray into a vector of ArrayRefs.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.get_at_indices.html" class="fn" title="fn datafusion::common::utils::get_at_indices">get_at_indices</a>  
This function “takes” the elements at `indices` from the slice `items`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.get_available_parallelism.html" class="fn" title="fn datafusion::common::utils::get_available_parallelism">get_available_parallelism</a>  
Returns the estimated number of threads available for parallel execution.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.get_row_at_idx.html" class="fn" title="fn datafusion::common::utils::get_row_at_idx">get_row_at_idx</a>  
Given column vectors, returns row at `idx`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.linear_search.html" class="fn" title="fn datafusion::common::utils::linear_search">linear_search</a>  
This function searches for a tuple of given values (`target`) among the given rows (`item_columns`) via a linear scan. It assumes that `item_columns` is sorted according to `sort_options` and returns the insertion index of `target`. Template argument `SIDE` being `true`/`false` means left/right insertion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.list_ndims.html" class="fn" title="fn datafusion::common::utils::list_ndims">list_ndims</a>  
Compute the number of dimensions in a list data type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.list_to_arrays.html" class="fn" title="fn datafusion::common::utils::list_to_arrays">list_to_arrays</a>  
Helper function to convert a ListArray into a vector of ArrayRefs.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.longest_consecutive_prefix.html" class="fn" title="fn datafusion::common::utils::longest_consecutive_prefix">longest_consecutive_prefix</a>  
This function finds the longest prefix of the form 0, 1, 2, … within the collection `sequence`. Examples:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.merge_and_order_indices.html" class="fn" title="fn datafusion::common::utils::merge_and_order_indices">merge_and_order_indices</a>  
Merges collections `first` and `second`, removes duplicates and sorts the result, returning it as a [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.project_schema.html" class="fn" title="fn datafusion::common::utils::project_schema">project_schema</a>  
Applies an optional projection to a [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef"), returning the projected schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.quote_identifier.html" class="fn" title="fn datafusion::common::utils::quote_identifier">quote_identifier</a>  
Wraps identifier string in double quotes, escaping any double quotes in the identifier by replacing it with two double quotes

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.search_in_slice.html" class="fn" title="fn datafusion::common::utils::search_in_slice">search_in_slice</a>  
This function searches for a tuple of given values (`target`) among a slice of the given rows (`item_columns`) via a linear scan. The slice starts at the index `low` and ends at the index `high`. The boolean-valued function `compare_fn` specifies the stopping criterion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.set_difference.html" class="fn" title="fn datafusion::common::utils::set_difference">set_difference</a>  
Calculates the set difference between sequences `first` and `second`, returning the result as a [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"). Preserves the ordering of `first`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.take_function_args.html" class="fn" title="fn datafusion::common::utils::take_function_args">take_function_args</a>  
Converts a collection of function arguments into a fixed-size array of length N producing a reasonable error message in case of unexpected number of arguments.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/fn.transpose.html" class="fn" title="fn datafusion::common::utils::transpose">transpose</a>  
Transposes the given vector of vectors.
