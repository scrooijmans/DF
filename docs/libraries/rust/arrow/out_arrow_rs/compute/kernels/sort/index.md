# Module sort Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/lib.rs.html#58" class="src">Source</a>

Expand description

Defines sort kernel for `ArrayRef`

## Structs<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.FixedLexicographicalComparator.html" class="struct" title="struct arrow::compute::kernels::sort::FixedLexicographicalComparator">FixedLexicographicalComparator</a>  
A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data at given two indices. This version of the comparator is for compile-time constant number of columns. The lifetime is the same at the data wrapped.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.LexicographicalComparator.html" class="struct" title="struct arrow::compute::kernels::sort::LexicographicalComparator">LexicographicalComparator</a>  
A lexicographical comparator that wraps given array data (columns) and can lexicographically compare data at given two indices. The lifetime is the same at the data wrapped.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortColumn.html" class="struct" title="struct arrow::compute::kernels::sort::SortColumn">SortColumn</a>  
One column to be used in lexicographical sort

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/struct.SortOptions.html" class="struct" title="struct arrow::compute::kernels::sort::SortOptions">SortOptions</a>  
Options that define the sort order of a given column

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/fn.lexsort.html" class="fn" title="fn arrow::compute::kernels::sort::lexsort">lexsort</a>  
Sort a list of `ArrayRef` using `SortOptions` provided for each array.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/fn.lexsort_to_indices.html" class="fn" title="fn arrow::compute::kernels::sort::lexsort_to_indices">lexsort_to_indices</a>  
Sort elements lexicographically from a list of `ArrayRef` into an unsigned integer (`UInt32Array`) of indices.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/fn.partial_sort.html" class="fn" title="fn arrow::compute::kernels::sort::partial_sort">partial_sort</a>  
It’s unstable_sort, may not preserve the order of equal elements

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/fn.partition_validity.html" class="fn" title="fn arrow::compute::kernels::sort::partition_validity">partition_validity</a>  
Partition indices of an Arrow array into two categories:

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/fn.sort.html" class="fn" title="fn arrow::compute::kernels::sort::sort">sort</a>  
Sort the `ArrayRef` using `SortOptions`.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/fn.sort_limit.html" class="fn" title="fn arrow::compute::kernels::sort::sort_limit">sort_limit</a>  
Sort the `ArrayRef` partially.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/sort/fn.sort_to_indices.html" class="fn" title="fn arrow::compute::kernels::sort::sort_to_indices">sort_to_indices</a>  
Sort elements from `ArrayRef` into an unsigned integer (`UInt32Array`) of indices. Floats are sorted using IEEE 754 totalOrder. `limit` is an option for [partial_sort](https://docs.rs/arrow/latest/arrow/compute/fn.partial_sort.html "fn arrow::compute::partial_sort").
