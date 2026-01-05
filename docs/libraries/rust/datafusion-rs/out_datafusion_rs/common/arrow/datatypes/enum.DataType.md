# Enum DataType Copy item path

<a href="https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/src/arrow_schema/datatype.rs.html#97" class="src">Source</a>

``` rust
pub enum DataType {
Show 41 variants    Null,
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float16,
    Float32,
    Float64,
    Timestamp(TimeUnit, Option<Arc<str>>),
    Date32,
    Date64,
    Time32(TimeUnit),
    Time64(TimeUnit),
    Duration(TimeUnit),
    Interval(IntervalUnit),
    Binary,
    FixedSizeBinary(i32),
    LargeBinary,
    BinaryView,
    Utf8,
    LargeUtf8,
    Utf8View,
    List(Arc<Field>),
    ListView(Arc<Field>),
    FixedSizeList(Arc<Field>, i32),
    LargeList(Arc<Field>),
    LargeListView(Arc<Field>),
    Struct(Fields),
    Union(UnionFields, UnionMode),
    Dictionary(Box<DataType>, Box<DataType>),
    Decimal32(u8, i8),
    Decimal64(u8, i8),
    Decimal128(u8, i8),
    Decimal256(u8, i8),
    Map(Arc<Field>, bool),
    RunEndEncoded(Arc<Field>, Arc<Field>),
}
```

Expand description

Datatypes supported by this implementation of Apache Arrow.

The variants of this enum include primitive fixed size types as well as parametric or nested types. See [`Schema.fbs`](https://github.com/apache/arrow/blob/main/format/Schema.fbs) for Arrow’s specification.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#examples" class="doc-anchor">§</a>Examples

Primitive types

``` rust
// create a new 32-bit signed integer
let data_type = DataType::Int32;
```

Nested Types

``` rust
// create a new list of 32-bit signed integers directly
let list_data_type = DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
// Create the same list type with constructor
let list_data_type2 = DataType::new_list(DataType::Int32, true);
assert_eq!(list_data_type, list_data_type2);
```

Dictionary Types

``` rust
// String Dictionary (key type Int32 and value type Utf8)
let data_type = DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8));
```

Timestamp Types

``` rust
// timestamp with millisecond precision without timezone specified
let data_type = DataType::Timestamp(TimeUnit::Millisecond, None);
// timestamp with nanosecond precision in UTC timezone
let data_type = DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into()));
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#display-and-fromstr" class="doc-anchor">§</a>Display and FromStr

The `Display` and `FromStr` implementations for `DataType` are human-readable, parseable, and reversible.

``` rust
let data_type = DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8));
let data_type_string = data_type.to_string();
assert_eq!(data_type_string, "Dictionary(Int32, Utf8)");
// display can be parsed back into the original type
let parsed_data_type: DataType = data_type.to_string().parse().unwrap();
assert_eq!(data_type, parsed_data_type);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#nested-support" class="doc-anchor">§</a>Nested Support

Currently, the Rust implementation supports the following nested types:

- `List<T>`
- `LargeList<T>`
- `FixedSizeList<T>`
- `Struct<T, U, V, ...>`
- `Union<T, U, V, ...>`
- `Map<K, V>`

Nested types can themselves be nested within other arrays. For more information on these types please see [the physical memory layout of Apache Arrow](https://arrow.apache.org/docs/format/Columnar.html#physical-memory-layout)

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Null" class="anchor">§</a>

### Null

Null type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Boolean" class="anchor">§</a>

### Boolean

A boolean datatype representing the values `true` and `false`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Int8" class="anchor">§</a>

### Int8

A signed 8-bit integer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Int16" class="anchor">§</a>

### Int16

A signed 16-bit integer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Int32" class="anchor">§</a>

### Int32

A signed 32-bit integer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Int64" class="anchor">§</a>

### Int64

A signed 64-bit integer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.UInt8" class="anchor">§</a>

### UInt8

An unsigned 8-bit integer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.UInt16" class="anchor">§</a>

### UInt16

An unsigned 16-bit integer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.UInt32" class="anchor">§</a>

### UInt32

An unsigned 32-bit integer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.UInt64" class="anchor">§</a>

### UInt64

An unsigned 64-bit integer.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Float16" class="anchor">§</a>

### Float16

A 16-bit floating point number.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Float32" class="anchor">§</a>

### Float32

A 32-bit floating point number.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Float64" class="anchor">§</a>

### Float64

A 64-bit floating point number.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Timestamp" class="anchor">§</a>

### Timestamp(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum datafusion::common::arrow::datatypes::TimeUnit">TimeUnit</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>)

A timestamp with an optional timezone.

Time is measured as a Unix epoch, counting the seconds from 00:00:00.000 on 1 January 1970, excluding leap seconds, as a signed 64-bit integer.

The time zone is a string indicating the name of a time zone, one of:

- As used in the Olson time zone database (the “tz database” or “tzdata”), such as “America/New_York”
- An absolute time zone offset of the form +XX:XX or -XX:XX, such as +07:30

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#timestamps-with-a-non-empty-timezone" class="doc-anchor">§</a>Timestamps with a non-empty timezone

If a Timestamp column has a non-empty timezone value, its epoch is 1970-01-01 00:00:00 (January 1st 1970, midnight) in the *UTC* timezone (the Unix epoch), regardless of the Timestamp’s own timezone.

Therefore, timestamp values with a non-empty timezone correspond to physical points in time together with some additional information about how the data was obtained and/or how to display it (the timezone).

For example, the timestamp value 0 with the timezone string “Europe/Paris” corresponds to “January 1st 1970, 00h00” in the UTC timezone, but the application may prefer to display it as “January 1st 1970, 01h00” in the Europe/Paris timezone (which is the same physical point in time).

One consequence is that timestamp values with a non-empty timezone can be compared and ordered directly, since they all share the same well-known point of reference (the Unix epoch).

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#timestamps-with-an-unset--empty-timezone" class="doc-anchor">§</a>Timestamps with an unset / empty timezone

If a Timestamp column has no timezone value, its epoch is 1970-01-01 00:00:00 (January 1st 1970, midnight) in an *unknown* timezone.

Therefore, timestamp values without a timezone cannot be meaningfully interpreted as physical points in time, but only as calendar / clock indications (“wall clock time”) in an unspecified timezone.

For example, the timestamp value 0 with an empty timezone string corresponds to “January 1st 1970, 00h00” in an unknown timezone: there is not enough information to interpret it as a well-defined physical point in time.

One consequence is that timestamp values without a timezone cannot be reliably compared or ordered, since they may have different points of reference. In particular, it is *not* possible to interpret an unset or empty timezone as the same as “UTC”.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#conversion-between-timezones" class="doc-anchor">§</a>Conversion between timezones

If a Timestamp column has a non-empty timezone, changing the timezone to a different non-empty value is a metadata-only operation: the timestamp values need not change as their point of reference remains the same (the Unix epoch).

However, if a Timestamp column has no timezone value, changing it to a non-empty value requires to think about the desired semantics. One possibility is to assume that the original timestamp values are relative to the epoch of the timezone being set; timestamp values should then adjusted to the Unix epoch (for example, changing the timezone from empty to “Europe/Paris” would require converting the timestamp values from “Europe/Paris” to “UTC”, which seems counter-intuitive but is nevertheless correct).

``` rust
DataType::Timestamp(TimeUnit::Second, None);
DataType::Timestamp(TimeUnit::Second, Some("literal".into()));
DataType::Timestamp(TimeUnit::Second, Some("string".to_string().into()));
```

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#timezone-representation" class="doc-anchor">§</a>Timezone representation

------------------------------------------------------------------------

It is possible to use either the timezone string representation, such as “UTC”, or the absolute time zone offset “+00:00”. For timezones with fixed offsets, such as “UTC” or “JST”, the offset representation is recommended, as it is more explicit and less ambiguous.

Most arrow-rs functionalities use the absolute offset representation, such as [`PrimitiveArray::with_timezone_utc`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.with_timezone_utc) that applies a UTC timezone to timestamp arrays.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#timezone-string-parsing" class="doc-anchor">§</a>Timezone string parsing

When feature `chrono-tz` is not enabled, allowed timezone strings are fixed offsets of the form “+09:00”, “-09” or “+0930”.

When feature `chrono-tz` is enabled, additional strings supported by [chrono_tz](https://docs.rs/chrono-tz/latest/chrono_tz/) are also allowed, which include [IANA database](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones) timezones.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Date32" class="anchor">§</a>

### Date32

A signed 32-bit date representing the elapsed time since UNIX epoch (1970-01-01) in days.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Date64" class="anchor">§</a>

### Date64

A signed 64-bit date representing the elapsed time since UNIX epoch (1970-01-01) in milliseconds.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#valid-ranges" class="doc-anchor">§</a>Valid Ranges

According to the Arrow specification ([Schema.fbs](https://github.com/apache/arrow/blob/main/format/Schema.fbs)), values of Date64 are treated as the number of *days*, in milliseconds, since the UNIX epoch. Therefore, values of this type must be evenly divisible by `86_400_000`, the number of milliseconds in a standard day.

It is not valid to store milliseconds that do not represent an exact day. The reason for this restriction is compatibility with other language’s native libraries (specifically Java), which historically lacked a dedicated date type and only supported timestamps.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#validation" class="doc-anchor">§</a>Validation

This library does not validate or enforce that Date64 values are evenly divisible by `86_400_000` for performance and usability reasons. Date64 values are treated similarly to `Timestamp(TimeUnit::Millisecond, None)`: values will be displayed with a time of day if the value does not represent an exact day, and arithmetic will be done at the millisecond granularity.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#recommendation" class="doc-anchor">§</a>Recommendation

Users should prefer [`Date32`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Date32 "variant datafusion::common::arrow::datatypes::DataType::Date32") to cleanly represent the number of days, or one of the Timestamp variants to include time as part of the representation, depending on their use case.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#further-reading" class="doc-anchor">§</a>Further Reading

For more details, see [\#5288](https://github.com/apache/arrow-rs/issues/5288).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Time32" class="anchor">§</a>

### Time32(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum datafusion::common::arrow::datatypes::TimeUnit">TimeUnit</a>)

A signed 32-bit time representing the elapsed time since midnight in the unit of `TimeUnit`. Must be either seconds or milliseconds.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Time64" class="anchor">§</a>

### Time64(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum datafusion::common::arrow::datatypes::TimeUnit">TimeUnit</a>)

A signed 64-bit time representing the elapsed time since midnight in the unit of `TimeUnit`. Must be either microseconds or nanoseconds.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Duration" class="anchor">§</a>

### Duration(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.TimeUnit.html" class="enum" title="enum datafusion::common::arrow::datatypes::TimeUnit">TimeUnit</a>)

Measure of elapsed time in either seconds, milliseconds, microseconds or nanoseconds.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Interval" class="anchor">§</a>

### Interval(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.IntervalUnit.html" class="enum" title="enum datafusion::common::arrow::datatypes::IntervalUnit">IntervalUnit</a>)

A “calendar” interval which models types that don’t necessarily have a precise duration without the context of a base timestamp (e.g. days can differ in length during day light savings time transitions).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Binary" class="anchor">§</a>

### Binary

Opaque binary data of variable length.

A single Binary array can store up to [`i32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i32.html#associatedconstant.MAX "associated constant i32::MAX") bytes of binary data in total.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.FixedSizeBinary" class="anchor">§</a>

### FixedSizeBinary(<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>)

Opaque binary data of fixed size. Enum parameter specifies the number of bytes per value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.LargeBinary" class="anchor">§</a>

### LargeBinary

Opaque binary data of variable length and 64-bit offsets.

A single LargeBinary array can store up to [`i64::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i64.html#associatedconstant.MAX "associated constant i64::MAX") bytes of binary data in total.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.BinaryView" class="anchor">§</a>

### BinaryView

Opaque binary data of variable length.

Logically the same as [`Binary`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Binary "variant datafusion::common::arrow::datatypes::DataType::Binary"), but the internal representation uses a view struct that contains the string length and either the string’s entire data inline (for small strings) or an inlined prefix, an index of another buffer, and an offset pointing to a slice in that buffer (for non-small strings).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Utf8" class="anchor">§</a>

### Utf8

A variable-length string in Unicode with UTF-8 encoding.

A single Utf8 array can store up to [`i32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i32.html#associatedconstant.MAX "associated constant i32::MAX") bytes of string data in total.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.LargeUtf8" class="anchor">§</a>

### LargeUtf8

A variable-length string in Unicode with UFT-8 encoding and 64-bit offsets.

A single LargeUtf8 array can store up to [`i64::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i64.html#associatedconstant.MAX "associated constant i64::MAX") bytes of string data in total.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Utf8View" class="anchor">§</a>

### Utf8View

A variable-length string in Unicode with UTF-8 encoding

Logically the same as [`Utf8`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Utf8 "variant datafusion::common::arrow::datatypes::DataType::Utf8"), but the internal representation uses a view struct that contains the string length and either the string’s entire data inline (for small strings) or an inlined prefix, an index of another buffer, and an offset pointing to a slice in that buffer (for non-small strings).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)

A list of some logical data type with variable length.

A single List array can store up to [`i32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i32.html#associatedconstant.MAX "associated constant i32::MAX") elements in total.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.ListView" class="anchor">§</a>

### ListView(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)

(NOT YET FULLY SUPPORTED) A list of some logical data type with variable length.

Logically the same as [`List`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.List "variant datafusion::common::arrow::datatypes::DataType::List"), but the internal representation differs in how child data is referenced, allowing flexibility in how data is layed out.

Note this data type is not yet fully supported. Using it with arrow APIs may result in `panic`s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.FixedSizeList" class="anchor">§</a>

### FixedSizeList(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>)

A list of some logical data type with fixed length.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.LargeList" class="anchor">§</a>

### LargeList(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)

A list of some logical data type with variable length and 64-bit offsets.

A single LargeList array can store up to [`i64::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i64.html#associatedconstant.MAX "associated constant i64::MAX") elements in total.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.LargeListView" class="anchor">§</a>

### LargeListView(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)

(NOT YET FULLY SUPPORTED) A list of some logical data type with variable length and 64-bit offsets.

Logically the same as [`LargeList`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.LargeList "variant datafusion::common::arrow::datatypes::DataType::LargeList"), but the internal representation differs in how child data is referenced, allowing flexibility in how data is layed out.

Note this data type is not yet fully supported. Using it with arrow APIs may result in `panic`s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>)

A nested datatype that contains a number of sub-fields.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Union" class="anchor">§</a>

### Union(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UnionFields.html" class="struct" title="struct datafusion::common::arrow::datatypes::UnionFields">UnionFields</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.UnionMode.html" class="enum" title="enum datafusion::common::arrow::datatypes::UnionMode">UnionMode</a>)

A nested datatype that can represent slots of differing types. Components:

1.  [`UnionFields`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UnionFields.html "struct datafusion::common::arrow::datatypes::UnionFields")
2.  The type of union (Sparse or Dense)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Dictionary" class="anchor">§</a>

### Dictionary(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>)

A dictionary encoded array (`key_type`, `value_type`), where each array element is an index of `key_type` into an associated dictionary of `value_type`.

Dictionary arrays are used to store columns of `value_type` that contain many repeated values using less memory, but with a higher CPU overhead for some operations.

This type mostly used to represent low cardinality string arrays or a limited set of primitive types as integers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Decimal32" class="anchor">§</a>

### Decimal32(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>)

Exact 32-bit width decimal value with precision and scale

- precision is the total number of digits
- scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.

For example the number 12300 could be treated as a decimal has precision 3 and scale -2.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Decimal64" class="anchor">§</a>

### Decimal64(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>)

Exact 64-bit width decimal value with precision and scale

- precision is the total number of digits
- scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.

For example the number 12300 could be treated as a decimal has precision 3 and scale -2.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Decimal128" class="anchor">§</a>

### Decimal128(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>)

Exact 128-bit width decimal value with precision and scale

- precision is the total number of digits
- scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.

For example the number 12300 could be treated as a decimal has precision 3 and scale -2.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Decimal256" class="anchor">§</a>

### Decimal256(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>)

Exact 256-bit width decimal value with precision and scale

- precision is the total number of digits
- scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.

For example the number 12300 could be treated as a decimal has precision 3 and scale -2.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Map" class="anchor">§</a>

### Map(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

A Map is a logical nested type that is represented as

`List<entries: Struct<key: K, value: V>>`

The keys and values are each respectively contiguous. The key and value types are not constrained, but keys should be hashable and unique. Whether the keys are sorted can be set in the `bool` after the `Field`.

In a field with Map type, the field has a child Struct field, which then has two children: key type and the second the value type. The names of the child fields may be respectively “entries”, “key”, and “value”, but this is not enforced.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.RunEndEncoded" class="anchor">§</a>

### RunEndEncoded(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)

A run-end encoding (REE) is a variation of run-length encoding (RLE). These encodings are well-suited for representing data containing sequences of the same value, called runs. Each run is represented as a value and an integer giving the index in the array where the run ends.

A run-end encoded array has no buffers by itself, but has two child arrays. The first child array, called the run ends array, holds either 16, 32, or 64-bit signed integers. The actual values of each run are held in the second child array.

These child arrays are prescribed the standard names of “run_ends” and “values” respectively.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-DataType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_primitive" class="fn">is_primitive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the type is primitive: (numeric, temporal).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_numeric" class="fn">is_numeric</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is numeric: (UInt\*, Int\*, Float\*, Decimal\*).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_temporal" class="fn">is_temporal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is temporal: (Date\*, Time\*, Duration, or Interval).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_floating" class="fn">is_floating</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is floating: (Float\*).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_integer" class="fn">is_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is integer: (Int\*, UInt\*).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_signed_integer" class="fn">is_signed_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is signed integer: (Int\*).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_unsigned_integer" class="fn">is_unsigned_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is unsigned integer: (UInt\*).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_dictionary_key_type" class="fn">is_dictionary_key_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is valid as a dictionary key

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_run_ends_type" class="fn">is_run_ends_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is valid for run-ends array in RunArray

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_nested" class="fn">is_nested</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is nested (List, FixedSizeList, LargeList, ListView. LargeListView, Struct, Union, or Map), or a dictionary of a nested type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_null" class="fn">is_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this type is DataType::Null.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.equals_datatype" class="fn">equals_datatype</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Compares the datatype with another, ignoring nested field names and metadata.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.primitive_width" class="fn">primitive_width</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Returns the byte width of this type if it is a primitive type

Returns `None` if not a primitive type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return size of this instance in bytes.

Includes the size of `Self`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.contains" class="fn">contains</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check to see if `self` is a superset of `other`

If DataType is a nested type, then it will check to see if the nested type is a superset of the other nested type else it will check to see if the DataType is equal to the other DataType

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.new_list" class="fn">new_list</a>(data_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Create a [`DataType::List`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.List "variant datafusion::common::arrow::datatypes::DataType::List") with elements of the specified type and nullability, and conventionally named inner [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") (`"item"`).

To specify field level metadata, construct the inner [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") directly via [`Field::new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.new "associated function datafusion::common::arrow::datatypes::Field::new") or [`Field::new_list_field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.new_list_field "associated function datafusion::common::arrow::datatypes::Field::new_list_field").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.new_large_list" class="fn">new_large_list</a>(data_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Create a [`DataType::LargeList`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.LargeList "variant datafusion::common::arrow::datatypes::DataType::LargeList") with elements of the specified type and nullability, and conventionally named inner [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") (`"item"`).

To specify field level metadata, construct the inner [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") directly via [`Field::new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.new "associated function datafusion::common::arrow::datatypes::Field::new") or [`Field::new_list_field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.new_list_field "associated function datafusion::common::arrow::datatypes::Field::new_list_field").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.new_fixed_size_list" class="fn">new_fixed_size_list</a>( data_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Create a [`DataType::FixedSizeList`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.FixedSizeList "variant datafusion::common::arrow::datatypes::DataType::FixedSizeList") with elements of the specified type, size and nullability, and conventionally named inner [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") (`"item"`).

To specify field level metadata, construct the inner [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") directly via [`Field::new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.new "associated function datafusion::common::arrow::datatypes::Field::new") or [`Field::new_list_field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html#method.new_list_field "associated function datafusion::common::arrow::datatypes::Field::new_list_field").

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-Clone-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-Debug-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-Deserialize%3C&#39;de%3E-for-DataType" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-Display-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-From%3C%26DataType%3E-for-NativeType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-From%3CDataType%3E-for-NativeType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-FromPyArrow-for-DataType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.FromPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::FromPyArrow">FromPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.from_pyarrow_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.FromPyArrow.html#tymethod.from_pyarrow_bound" class="fn">from_pyarrow_bound</a>(value: &<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'\_, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert a Python object to an arrow-rs type. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.FromPyArrow.html#tymethod.from_pyarrow_bound)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-FromStr-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Parses `str` into a `DataType`.

This is the reverse of [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType")’s `Display` impl, and maintains the invariant that `DataType::try_from(&data_type.to_string()).unwrap() == data_type`

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#example" class="doc-anchor">§</a>Example

``` rust
use arrow_schema::DataType;

let data_type: DataType = "Int32".parse().unwrap();
assert_eq!(data_type, DataType::Int32);
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-Hash-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-Ord-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-PartialEq-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-PartialOrd-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-Serialize-for-DataType" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-ToPyArrow-for-DataType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.to_pyarrow" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#tymethod.to_pyarrow" class="fn">to_pyarrow</a>(&self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert the implemented type into a Python object without consuming it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-TryFrom%3C%26DataType%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.try_from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(dtype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

See [CDataInterface docs](https://arrow.apache.org/docs/format/CDataInterface.html#data-type-description-format-strings)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#associatedtype.Error-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-TryFrom%3C%26DataType%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.try_from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a Null instance of ScalarValue for this datatype

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-TryFrom%3C%26FFI_ArrowSchema%3E-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.try_from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(c_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

See [CDataInterface docs](https://arrow.apache.org/docs/format/CDataInterface.html#data-type-description-format-strings)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#associatedtype.Error-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-TryFrom%3C%26str%3E-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#associatedtype.Error-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.try_from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-TryFrom%3CDataType%3E-for-FFI_ArrowSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#associatedtype.Error-5" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.try_from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(dtype: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct datafusion::common::arrow::array::ffi::FFI_ArrowSchema">FFI_ArrowSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-TryFrom%3CDataType%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(datatype: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a Null instance of ScalarValue for this datatype

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-Eq-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#impl-StructuralPartialEq-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#blanket-implementations" class="anchor">§</a>
