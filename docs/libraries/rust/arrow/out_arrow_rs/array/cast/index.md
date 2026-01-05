# Module cast Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/lib.rs.html#250" class="src">Source</a>

Expand description

Defines helper functions for downcasting [`dyn Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") to concrete types

## Traits<a href="https://docs.rs/arrow/latest/arrow/array/cast/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/cast/trait.AsArray.html" class="trait" title="trait arrow::array::cast::AsArray">AsArray</a>  
An extension trait for `dyn Array` that provides ergonomic downcasting

## Functions<a href="https://docs.rs/arrow/latest/arrow/array/cast/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_boolean_array.html" class="fn" title="fn arrow::array::cast::as_boolean_array">as_boolean_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`BooleanArray`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_dictionary_array.html" class="fn" title="fn arrow::array::cast::as_dictionary_array">as_dictionary_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`DictionaryArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray"), panic’ing on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_fixed_size_list_array.html" class="fn" title="fn arrow::array::cast::as_fixed_size_list_array">as_fixed_size_list_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_generic_binary_array.html" class="fn" title="fn arrow::array::cast::as_generic_binary_array">as_generic_binary_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`GenericBinaryArray<S>`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_generic_list_array.html" class="fn" title="fn arrow::array::cast::as_generic_list_array">as_generic_list_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`GenericListArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_large_list_array.html" class="fn" title="fn arrow::array::cast::as_large_list_array">as_large_list_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`LargeListArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html "type arrow::array::LargeListArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_largestring_array.html" class="fn" title="fn arrow::array::cast::as_largestring_array">as_largestring_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_list_array.html" class="fn" title="fn arrow::array::cast::as_list_array">as_list_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_map_array.html" class="fn" title="fn arrow::array::cast::as_map_array">as_map_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_null_array.html" class="fn" title="fn arrow::array::cast::as_null_array">as_null_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`NullArray`](https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html "struct arrow::array::NullArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_primitive_array.html" class="fn" title="fn arrow::array::cast::as_primitive_array">as_primitive_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef"), to [`PrimitiveArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray"), panic’ing on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_run_array.html" class="fn" title="fn arrow::array::cast::as_run_array">as_run_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`RunArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray"), panic’ing on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_string_array.html" class="fn" title="fn arrow::array::cast::as_string_array">as_string_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_struct_array.html" class="fn" title="fn arrow::array::cast::as_struct_array">as_struct_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.as_union_array.html" class="fn" title="fn arrow::array::cast::as_union_array">as_union_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/cast/fn.downcast_array.html" class="fn" title="fn arrow::array::cast::downcast_array">downcast_array</a>  
Downcasts a `dyn Array` to a concrete type
