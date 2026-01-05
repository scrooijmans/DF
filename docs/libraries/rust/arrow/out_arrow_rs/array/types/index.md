# Module types Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/lib.rs.html#261" class="src">Source</a>

Expand description

Zero-sized types used to parameterize generic array implementations

## Structs<a href="https://docs.rs/arrow/latest/arrow/array/types/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BinaryViewType.html" class="struct" title="struct arrow::array::types::BinaryViewType">BinaryViewType</a>  
[`BinaryViewType`](https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html "struct arrow::datatypes::BinaryViewType") for string arrays

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.BooleanType.html" class="struct" title="struct arrow::array::types::BooleanType">BooleanType</a>  
A boolean datatype

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date32Type.html" class="struct" title="struct arrow::array::types::Date32Type">Date32Type</a>  
32-bit date type: the elapsed time since UNIX epoch in days (32 bits).

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Date64Type.html" class="struct" title="struct arrow::array::types::Date64Type">Date64Type</a>  
64-bit date type: the elapsed time since UNIX epoch in milliseconds (64 bits). Values must be divisible by `86_400_000`. See [`DataType::Date64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Date64 "variant arrow::datatypes::DataType::Date64") for more details.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal32Type.html" class="struct" title="struct arrow::array::types::Decimal32Type">Decimal32Type</a>  
The decimal type for a Decimal32Array

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal64Type.html" class="struct" title="struct arrow::array::types::Decimal64Type">Decimal64Type</a>  
The decimal type for a Decimal64Array

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal128Type.html" class="struct" title="struct arrow::array::types::Decimal128Type">Decimal128Type</a>  
The decimal type for a Decimal128Array

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Decimal256Type.html" class="struct" title="struct arrow::array::types::Decimal256Type">Decimal256Type</a>  
The decimal type for a Decimal256Array

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::array::types::DurationMicrosecondType">DurationMicrosecondType</a>  
Elapsed time type: microseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationMillisecondType.html" class="struct" title="struct arrow::array::types::DurationMillisecondType">DurationMillisecondType</a>  
Elapsed time type: milliseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationNanosecondType.html" class="struct" title="struct arrow::array::types::DurationNanosecondType">DurationNanosecondType</a>  
Elapsed time type: nanoseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.DurationSecondType.html" class="struct" title="struct arrow::array::types::DurationSecondType">DurationSecondType</a>  
Elapsed time type: seconds.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Float16Type.html" class="struct" title="struct arrow::array::types::Float16Type">Float16Type</a>  
16-bit floating point number type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Float32Type.html" class="struct" title="struct arrow::array::types::Float32Type">Float32Type</a>  
32-bit floating point number type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Float64Type.html" class="struct" title="struct arrow::array::types::Float64Type">Float64Type</a>  
64-bit floating point number type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericBinaryType.html" class="struct" title="struct arrow::array::types::GenericBinaryType">GenericBinaryType</a>  
[`ByteArrayType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html "trait arrow::datatypes::ByteArrayType") for binary arrays

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.GenericStringType.html" class="struct" title="struct arrow::array::types::GenericStringType">GenericStringType</a>  
[`ByteArrayType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html "trait arrow::datatypes::ByteArrayType") for string arrays

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Int8Type.html" class="struct" title="struct arrow::array::types::Int8Type">Int8Type</a>  
A signed 8-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Int16Type.html" class="struct" title="struct arrow::array::types::Int16Type">Int16Type</a>  
Signed 16-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Int32Type.html" class="struct" title="struct arrow::array::types::Int32Type">Int32Type</a>  
Signed 32-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Int64Type.html" class="struct" title="struct arrow::array::types::Int64Type">Int64Type</a>  
Signed 64-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.IntervalDayTime.html" class="struct" title="struct arrow::array::types::IntervalDayTime">IntervalDayTime</a>  
Value of an IntervalDayTime array

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::array::types::IntervalDayTimeType">IntervalDayTimeType</a>  
“Calendar” interval type: days and milliseconds. See [`IntervalDayTime`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTime.html "struct arrow::datatypes::IntervalDayTime") for more details.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.IntervalMonthDayNano.html" class="struct" title="struct arrow::array::types::IntervalMonthDayNano">IntervalMonthDayNano</a>  
Value of an IntervalMonthDayNano array

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::array::types::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a>  
“Calendar” interval type: months, days, and nanoseconds. See [`IntervalMonthDayNano`](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNano.html "struct arrow::datatypes::IntervalMonthDayNano") for more details.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::array::types::IntervalYearMonthType">IntervalYearMonthType</a>  
32-bit “calendar” interval type: the number of whole months.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.StringViewType.html" class="struct" title="struct arrow::array::types::StringViewType">StringViewType</a>  
[`ByteViewType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html "trait arrow::datatypes::ByteViewType") for string arrays

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Time32MillisecondType.html" class="struct" title="struct arrow::array::types::Time32MillisecondType">Time32MillisecondType</a>  
32-bit time type: the elapsed time since midnight in milliseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Time32SecondType.html" class="struct" title="struct arrow::array::types::Time32SecondType">Time32SecondType</a>  
32-bit time type: the elapsed time since midnight in seconds.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::array::types::Time64MicrosecondType">Time64MicrosecondType</a>  
64-bit time type: the elapsed time since midnight in microseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.Time64NanosecondType.html" class="struct" title="struct arrow::array::types::Time64NanosecondType">Time64NanosecondType</a>  
64-bit time type: the elapsed time since midnight in nanoseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::array::types::TimestampMicrosecondType">TimestampMicrosecondType</a>  
Timestamp microsecond type with an optional timezone.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::array::types::TimestampMillisecondType">TimestampMillisecondType</a>  
Timestamp millisecond type with an optional timezone.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::array::types::TimestampNanosecondType">TimestampNanosecondType</a>  
Timestamp nanosecond type with an optional timezone.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.TimestampSecondType.html" class="struct" title="struct arrow::array::types::TimestampSecondType">TimestampSecondType</a>  
Timestamp second type with an optional timezone.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.UInt8Type.html" class="struct" title="struct arrow::array::types::UInt8Type">UInt8Type</a>  
Unsigned 8-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.UInt16Type.html" class="struct" title="struct arrow::array::types::UInt16Type">UInt16Type</a>  
Unsigned 16-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.UInt32Type.html" class="struct" title="struct arrow::array::types::UInt32Type">UInt32Type</a>  
Unsigned 32-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/types/struct.UInt64Type.html" class="struct" title="struct arrow::array::types::UInt64Type">UInt64Type</a>  
Unsigned 64-bit integer type.

## Traits<a href="https://docs.rs/arrow/latest/arrow/array/types/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::array::types::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>  
A subtype of primitive type that represents legal dictionary keys. See <https://arrow.apache.org/docs/format/Columnar.html>

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::types::ArrowPrimitiveType">ArrowPrimitiveType</a>  
Trait for [primitive values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout).

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ArrowTemporalType.html" class="trait" title="trait arrow::array::types::ArrowTemporalType">ArrowTemporalType</a>  
A subtype of primitive type that represents temporal values.

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ArrowTimestampType.html" class="trait" title="trait arrow::array::types::ArrowTimestampType">ArrowTimestampType</a>  
A timestamp type allows us to create array builders that take a timestamp.

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteArrayType.html" class="trait" title="trait arrow::array::types::ByteArrayType">ByteArrayType</a>  
A trait over the variable-size byte array types

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.ByteViewType.html" class="trait" title="trait arrow::array::types::ByteViewType">ByteViewType</a>  
A trait over the variable length bytes view array types

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.DecimalType.html" class="trait" title="trait arrow::array::types::DecimalType">DecimalType</a>  
A trait over the decimal types, used by [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") to provide a generic implementation across the various decimal types

<a href="https://docs.rs/arrow/latest/arrow/array/types/trait.RunEndIndexType.html" class="trait" title="trait arrow::array::types::RunEndIndexType">RunEndIndexType</a>  
A subtype of primitive type that is used as run-ends index in `RunArray`. See <https://arrow.apache.org/docs/format/Columnar.html>

## Functions<a href="https://docs.rs/arrow/latest/arrow/array/types/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/fn.validate_decimal_precision_and_scale.html" class="fn" title="fn arrow::array::types::validate_decimal_precision_and_scale">validate_decimal_precision_and_scale</a>  
Validate that `precision` and `scale` are valid for `T`

## Type Aliases<a href="https://docs.rs/arrow/latest/arrow/array/types/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/types/type.BinaryType.html" class="type" title="type arrow::array::types::BinaryType">BinaryType</a>  
An arrow binary array with i32 offsets

<a href="https://docs.rs/arrow/latest/arrow/array/types/type.LargeBinaryType.html" class="type" title="type arrow::array::types::LargeBinaryType">LargeBinaryType</a>  
An arrow binary array with i64 offsets

<a href="https://docs.rs/arrow/latest/arrow/array/types/type.LargeUtf8Type.html" class="type" title="type arrow::array::types::LargeUtf8Type">LargeUtf8Type</a>  
An arrow utf8 array with i64 offsets

<a href="https://docs.rs/arrow/latest/arrow/array/types/type.Utf8Type.html" class="type" title="type arrow::array::types::Utf8Type">Utf8Type</a>  
An arrow utf8 array with i32 offsets
