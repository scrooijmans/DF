# Module comparison Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/compute/kernels.rs.html#30" class="src">Source</a>

Expand description

Comparison kernels for `Array`s.

## Traits<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/trait.StringArrayType.html" class="trait" title="trait arrow::compute::kernels::comparison::StringArrayType">StringArrayType</a>  
A trait for Arrow String Arrays, currently three types are supported:

## Functions<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.contains.html" class="fn" title="fn arrow::compute::kernels::comparison::contains">contains</a>  
Perform SQL `CONTAINS(left, right)`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.ends_with.html" class="fn" title="fn arrow::compute::kernels::comparison::ends_with">ends_with</a>  
Perform SQL `ENDSWITH(left, right)`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.ilike.html" class="fn" title="fn arrow::compute::kernels::comparison::ilike">ilike</a>  
Perform SQL `left ILIKE right`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.in_list.html" class="fn" title="fn arrow::compute::kernels::comparison::in_list">in_list</a>  
Checks if a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") contains a value in the [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.in_list_utf8.html" class="fn" title="fn arrow::compute::kernels::comparison::in_list_utf8">in_list_utf8</a>  
Checks if a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") contains a value in the [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray")

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.like.html" class="fn" title="fn arrow::compute::kernels::comparison::like">like</a>  
Perform SQL `left LIKE right`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.nilike.html" class="fn" title="fn arrow::compute::kernels::comparison::nilike">nilike</a>  
Perform SQL `left NOT ILIKE right`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.nlike.html" class="fn" title="fn arrow::compute::kernels::comparison::nlike">nlike</a>  
Perform SQL `left NOT LIKE right`

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.regexp_is_match.html" class="fn" title="fn arrow::compute::kernels::comparison::regexp_is_match">regexp_is_match</a>  
Return BooleanArray indicating which strings in an array match an array of regular expressions.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.regexp_is_match_scalar.html" class="fn" title="fn arrow::compute::kernels::comparison::regexp_is_match_scalar">regexp_is_match_scalar</a>  
Return BooleanArray indicating which strings in an array match a single regular expression.

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/comparison/fn.starts_with.html" class="fn" title="fn arrow::compute::kernels::comparison::starts_with">starts_with</a>  
Perform SQL `STARTSWITH(left, right)`
