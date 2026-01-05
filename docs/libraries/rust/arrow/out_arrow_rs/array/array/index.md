# Module array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/lib.rs.html#232" class="src">Source</a>

Expand description

The concrete array definitions

## Structs<a href="https://docs.rs/arrow/latest/arrow/array/array/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::array::BooleanArray">BooleanArray</a>  
An array of [boolean values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::array::DictionaryArray">DictionaryArray</a>  
An array of [dictionary encoded values](https://arrow.apache.org/docs/format/Columnar.html#dictionary-encoded-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>  
An array of [fixed size binary arrays](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::array::FixedSizeListArray">FixedSizeListArray</a>  
An array of \[fixed length lists\], similar to JSON arrays (e.g. `["A", "B"]`).

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::array::GenericByteArray">GenericByteArray</a>  
An array of [variable length byte arrays](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::array::GenericByteViewArray">GenericByteViewArray</a>  
[Variable-size Binary View Layout](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout): An array of variable length bytes views.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::array::GenericListArray">GenericListArray</a>  
An array of [variable length lists](https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout), similar to JSON arrays (e.g. `["A", "B", "C"]`). This struct specifically represents the [list layout](https://arrow.apache.org/docs/format/Columnar.html#list-layout). Refer to [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") for the [list-view layout](https://arrow.apache.org/docs/format/Columnar.html#listview-layout).

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::array::GenericListViewArray">GenericListViewArray</a>  
An array of [variable length lists](https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout), specifically in the [list-view layout](https://arrow.apache.org/docs/format/Columnar.html#listview-layout).

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.MapArray.html" class="struct" title="struct arrow::array::array::MapArray">MapArray</a>  
An array of key-value maps

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::array::NativeAdapter">NativeAdapter</a>  
An optional primitive value

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.NullArray.html" class="struct" title="struct arrow::array::array::NullArray">NullArray</a>  
An array of [null values](https://arrow.apache.org/docs/format/Columnar.html#null-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::array::PrimitiveArray">PrimitiveArray</a>  
An array of primitive values, of type [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType")

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.RunArray.html" class="struct" title="struct arrow::array::array::RunArray">RunArray</a>  
An array of [run-end encoded values](https://arrow.apache.org/docs/format/Columnar.html#run-end-encoded-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.StructArray.html" class="struct" title="struct arrow::array::array::StructArray">StructArray</a>  
An array of [structs](https://arrow.apache.org/docs/format/Columnar.html#struct-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.TypedDictionaryArray.html" class="struct" title="struct arrow::array::array::TypedDictionaryArray">TypedDictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") typed on its child values array

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.TypedRunArray.html" class="struct" title="struct arrow::array::array::TypedRunArray">TypedRunArray</a>  
A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") typed typed on its child values array

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.UnionArray.html" class="struct" title="struct arrow::array::array::UnionArray">UnionArray</a>  
An array of [values of varying types](https://arrow.apache.org/docs/format/Columnar.html#union-layout)

## Traits<a href="https://docs.rs/arrow/latest/arrow/array/array/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::array::AnyDictionaryArray">AnyDictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") with the key type erased

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.Array.html" class="trait" title="trait arrow::array::array::Array">Array</a>  
An array in the [arrow columnar format](https://arrow.apache.org/docs/format/Columnar.html)

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::array::ArrayAccessor">ArrayAccessor</a>  
A generic trait for accessing the values of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::array::ArrowPrimitiveType">ArrowPrimitiveType</a>  
Trait for [primitive values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout).

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.BinaryArrayType.html" class="trait" title="trait arrow::array::array::BinaryArrayType">BinaryArrayType</a>  
A trait for Arrow String Arrays, currently three types are supported:

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::array::OffsetSizeTrait">OffsetSizeTrait</a>  
A type that can be used within a variable-size array to encode offset information

<a href="https://docs.rs/arrow/latest/arrow/array/array/trait.StringArrayType.html" class="trait" title="trait arrow::array::array::StringArrayType">StringArrayType</a>  
A trait for Arrow String Arrays, currently three types are supported:

## Functions<a href="https://docs.rs/arrow/latest/arrow/array/array/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/fn.make_array.html" class="fn" title="fn arrow::array::array::make_array">make_array</a>  
Constructs an array using the input `data`. Returns a reference-counted `Array` instance.

<a href="https://docs.rs/arrow/latest/arrow/array/array/fn.new_empty_array.html" class="fn" title="fn arrow::array::array::new_empty_array">new_empty_array</a>  
Creates a new empty array

<a href="https://docs.rs/arrow/latest/arrow/array/array/fn.new_null_array.html" class="fn" title="fn arrow::array::array::new_null_array">new_null_array</a>  
Creates a new array of `data_type` of length `length` filled entirely of `NULL` values

## Type Aliases<a href="https://docs.rs/arrow/latest/arrow/array/array/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.ArrayRef.html" class="type" title="type arrow::array::array::ArrayRef">ArrayRef</a>  
A reference-counted reference to a generic `Array`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.BinaryArray.html" class="type" title="type arrow::array::array::BinaryArray">BinaryArray</a>  
A [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") of `[u8]` using `i32` offsets

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.BinaryViewArray.html" class="type" title="type arrow::array::array::BinaryViewArray">BinaryViewArray</a>  
A [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") of `[u8]`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Date32Array.html" class="type" title="type arrow::array::array::Date32Array">Date32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of days since UNIX epoch stored as `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Date64Array.html" class="type" title="type arrow::array::array::Date64Array">Date64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of milliseconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Decimal32Array.html" class="type" title="type arrow::array::array::Decimal32Array">Decimal32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 32-bit fixed point decimals

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Decimal64Array.html" class="type" title="type arrow::array::array::Decimal64Array">Decimal64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 64-bit fixed point decimals

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Decimal128Array.html" class="type" title="type arrow::array::array::Decimal128Array">Decimal128Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 128-bit fixed point decimals

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Decimal256Array.html" class="type" title="type arrow::array::array::Decimal256Array">Decimal256Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 256-bit fixed point decimals

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.DurationMicrosecondArray.html" class="type" title="type arrow::array::array::DurationMicrosecondArray">DurationMicrosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of elapsed durations in microseconds

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.DurationMillisecondArray.html" class="type" title="type arrow::array::array::DurationMillisecondArray">DurationMillisecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of elapsed durations in milliseconds

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.DurationNanosecondArray.html" class="type" title="type arrow::array::array::DurationNanosecondArray">DurationNanosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of elapsed durations in nanoseconds

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.DurationSecondArray.html" class="type" title="type arrow::array::array::DurationSecondArray">DurationSecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of elapsed durations in seconds

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Float16Array.html" class="type" title="type arrow::array::array::Float16Array">Float16Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `f16`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Float32Array.html" class="type" title="type arrow::array::array::Float32Array">Float32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `f32`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Float64Array.html" class="type" title="type arrow::array::array::Float64Array">Float64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `f64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.GenericBinaryArray.html" class="type" title="type arrow::array::array::GenericBinaryArray">GenericBinaryArray</a>  
A [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") for storing `[u8]`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.GenericStringArray.html" class="type" title="type arrow::array::array::GenericStringArray">GenericStringArray</a>  
A [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") for storing `str`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int8Array.html" class="type" title="type arrow::array::array::Int8Array">Int8Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i8`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int8DictionaryArray.html" class="type" title="type arrow::array::array::Int8DictionaryArray">Int8DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `i8`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int16Array.html" class="type" title="type arrow::array::array::Int16Array">Int16Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i16`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int16DictionaryArray.html" class="type" title="type arrow::array::array::Int16DictionaryArray">Int16DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `i16`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int16RunArray.html" class="type" title="type arrow::array::array::Int16RunArray">Int16RunArray</a>  
A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") with `i16` run ends

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int32Array.html" class="type" title="type arrow::array::array::Int32Array">Int32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int32DictionaryArray.html" class="type" title="type arrow::array::array::Int32DictionaryArray">Int32DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int32RunArray.html" class="type" title="type arrow::array::array::Int32RunArray">Int32RunArray</a>  
A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") with `i32` run ends

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int64Array.html" class="type" title="type arrow::array::array::Int64Array">Int64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int64DictionaryArray.html" class="type" title="type arrow::array::array::Int64DictionaryArray">Int64DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Int64RunArray.html" class="type" title="type arrow::array::array::Int64RunArray">Int64RunArray</a>  
A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") with `i64` run ends

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.IntervalDayTimeArray.html" class="type" title="type arrow::array::array::IntervalDayTimeArray">IntervalDayTimeArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of “calendar” intervals in days and milliseconds

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.IntervalMonthDayNanoArray.html" class="type" title="type arrow::array::array::IntervalMonthDayNanoArray">IntervalMonthDayNanoArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of “calendar” intervals in months, days, and nanoseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.IntervalYearMonthArray.html" class="type" title="type arrow::array::array::IntervalYearMonthArray">IntervalYearMonthArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of “calendar” intervals in whole months

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.LargeBinaryArray.html" class="type" title="type arrow::array::array::LargeBinaryArray">LargeBinaryArray</a>  
A [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") of `[u8]` using `i64` offsets

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.LargeListArray.html" class="type" title="type arrow::array::array::LargeListArray">LargeListArray</a>  
A [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") of variable size lists, storing offsets as `i64`.

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.LargeListViewArray.html" class="type" title="type arrow::array::array::LargeListViewArray">LargeListViewArray</a>  
A [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") of variable size lists, storing offsets as `i64`.

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.LargeStringArray.html" class="type" title="type arrow::array::array::LargeStringArray">LargeStringArray</a>  
A [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") of `str` using `i64` offsets

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.ListArray.html" class="type" title="type arrow::array::array::ListArray">ListArray</a>  
A [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") of variable size lists, storing offsets as `i32`.

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.ListViewArray.html" class="type" title="type arrow::array::array::ListViewArray">ListViewArray</a>  
A [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") of variable size lists, storing offsets as `i32`.

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.StringArray.html" class="type" title="type arrow::array::array::StringArray">StringArray</a>  
A [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") of `str` using `i32` offsets

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.StringViewArray.html" class="type" title="type arrow::array::array::StringViewArray">StringViewArray</a>  
A [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") that stores utf8 data

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Time32MillisecondArray.html" class="type" title="type arrow::array::array::Time32MillisecondArray">Time32MillisecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of milliseconds since midnight stored as `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Time32SecondArray.html" class="type" title="type arrow::array::array::Time32SecondArray">Time32SecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of seconds since midnight stored as `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Time64MicrosecondArray.html" class="type" title="type arrow::array::array::Time64MicrosecondArray">Time64MicrosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of microseconds since midnight stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Time64NanosecondArray.html" class="type" title="type arrow::array::array::Time64NanosecondArray">Time64NanosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of nanoseconds since midnight stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.TimestampMicrosecondArray.html" class="type" title="type arrow::array::array::TimestampMicrosecondArray">TimestampMicrosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of microseconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.TimestampMillisecondArray.html" class="type" title="type arrow::array::array::TimestampMillisecondArray">TimestampMillisecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of milliseconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.TimestampNanosecondArray.html" class="type" title="type arrow::array::array::TimestampNanosecondArray">TimestampNanosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of nanoseconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.TimestampSecondArray.html" class="type" title="type arrow::array::array::TimestampSecondArray">TimestampSecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of seconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt8Array.html" class="type" title="type arrow::array::array::UInt8Array">UInt8Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u8`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt8DictionaryArray.html" class="type" title="type arrow::array::array::UInt8DictionaryArray">UInt8DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u8`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt16Array.html" class="type" title="type arrow::array::array::UInt16Array">UInt16Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u16`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt16DictionaryArray.html" class="type" title="type arrow::array::array::UInt16DictionaryArray">UInt16DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u16`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt32Array.html" class="type" title="type arrow::array::array::UInt32Array">UInt32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u32`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt32DictionaryArray.html" class="type" title="type arrow::array::array::UInt32DictionaryArray">UInt32DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u32`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt64Array.html" class="type" title="type arrow::array::array::UInt64Array">UInt64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u64`

<a href="https://docs.rs/arrow/latest/arrow/array/array/type.UInt64DictionaryArray.html" class="type" title="type arrow::array::array::UInt64DictionaryArray">UInt64DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u64`
