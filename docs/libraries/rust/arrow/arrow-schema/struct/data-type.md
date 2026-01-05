# DataType in arrow_schema - Rust

```
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
    List(FieldRef),
    ListView(FieldRef),
    FixedSizeList(FieldRef, i32),
    LargeList(FieldRef),
    LargeListView(FieldRef),
    Struct(Fields),
    Union(UnionFields, UnionMode),
    Dictionary(Box<DataType>, Box<DataType>),
    Decimal32(u8, i8),
    Decimal64(u8, i8),
    Decimal128(u8, i8),
    Decimal256(u8, i8),
    Map(FieldRef, bool),
    RunEndEncoded(FieldRef, FieldRef),
}
```

Expand description

Datatypes supported by this implementation of Apache Arrow.

The variants of this enum include primitive fixed size types as well as parametric or nested types. See [`Schema.fbs`](https://github.com/apache/arrow/blob/main/format/Schema.fbs) for Arrow’s specification.

## [§](#examples)Examples

Primitive types

```
// create a new 32-bit signed integer
let data_type = DataType::Int32;
```

Nested Types

```
// create a new list of 32-bit signed integers directly
let list_data_type = DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
// Create the same list type with constructor
let list_data_type2 = DataType::new_list(DataType::Int32, true);
assert_eq!(list_data_type, list_data_type2);
```

Dictionary Types

```
// String Dictionary (key type Int32 and value type Utf8)
let data_type = DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8));
```

Timestamp Types

```
// timestamp with millisecond precision without timezone specified
let data_type = DataType::Timestamp(TimeUnit::Millisecond, None);
// timestamp with nanosecond precision in UTC timezone
let data_type = DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into()));
```

## [§](#display-and-fromstr)Display and FromStr

The `Display` and `FromStr` implementations for `DataType` are human-readable, parseable, and reversible.

```
let data_type = DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8));
let data_type_string = data_type.to_string();
assert_eq!(data_type_string, "Dictionary(Int32, Utf8)");
// display can be parsed back into the original type
let parsed_data_type: DataType = data_type.to_string().parse().unwrap();
assert_eq!(data_type, parsed_data_type);
```

## [§](#nested-support)Nested Support

Currently, the Rust implementation supports the following nested types:

- `List<T>`
- `LargeList<T>`
- `FixedSizeList<T>`
- `Struct<T, U, V, ...>`
- `Union<T, U, V, ...>`
- `Map<K, V>`

Nested types can themselves be nested within other arrays. For more information on these types please see [the physical memory layout of Apache Arrow](https://arrow.apache.org/docs/format/Columnar.html#physical-memory-layout)

[§](#variant.Null)

Null type

[§](#variant.Boolean)

A boolean datatype representing the values `true` and `false`.

[§](#variant.Int8)

A signed 8-bit integer.

[§](#variant.Int16)

A signed 16-bit integer.

[§](#variant.Int32)

A signed 32-bit integer.

[§](#variant.Int64)

A signed 64-bit integer.

[§](#variant.UInt8)

An unsigned 8-bit integer.

[§](#variant.UInt16)

An unsigned 16-bit integer.

[§](#variant.UInt32)

An unsigned 32-bit integer.

[§](#variant.UInt64)

An unsigned 64-bit integer.

[§](#variant.Float16)

A 16-bit floating point number.

[§](#variant.Float32)

A 32-bit floating point number.

[§](#variant.Float64)

A 64-bit floating point number.

[§](#variant.Timestamp)

A timestamp with an optional timezone.

Time is measured as a Unix epoch, counting the seconds from 00:00:00.000 on 1 January 1970, excluding leap seconds, as a signed 64-bit integer.

The time zone is a string indicating the name of a time zone, one of:

- As used in the Olson time zone database (the “tz database” or “tzdata”), such as “America/New_York”
- An absolute time zone offset of the form +XX:XX or -XX:XX, such as +07:30

##### [§](#timestamps-with-a-non-empty-timezone)Timestamps with a non-empty timezone

If a Timestamp column has a non-empty timezone value, its epoch is 1970-01-01 00:00:00 (January 1st 1970, midnight) in the _UTC_ timezone (the Unix epoch), regardless of the Timestamp’s own timezone.

Therefore, timestamp values with a non-empty timezone correspond to physical points in time together with some additional information about how the data was obtained and/or how to display it (the timezone).

For example, the timestamp value 0 with the timezone string “Europe/Paris” corresponds to “January 1st 1970, 00h00” in the UTC timezone, but the application may prefer to display it as “January 1st 1970, 01h00” in the Europe/Paris timezone (which is the same physical point in time).

One consequence is that timestamp values with a non-empty timezone can be compared and ordered directly, since they all share the same well-known point of reference (the Unix epoch).

##### [§](#timestamps-with-an-unset--empty-timezone)Timestamps with an unset / empty timezone

If a Timestamp column has no timezone value, its epoch is 1970-01-01 00:00:00 (January 1st 1970, midnight) in an _unknown_ timezone.

Therefore, timestamp values without a timezone cannot be meaningfully interpreted as physical points in time, but only as calendar / clock indications (“wall clock time”) in an unspecified timezone.

For example, the timestamp value 0 with an empty timezone string corresponds to “January 1st 1970, 00h00” in an unknown timezone: there is not enough information to interpret it as a well-defined physical point in time.

One consequence is that timestamp values without a timezone cannot be reliably compared or ordered, since they may have different points of reference. In particular, it is _not_ possible to interpret an unset or empty timezone as the same as “UTC”.

##### [§](#conversion-between-timezones)Conversion between timezones

If a Timestamp column has a non-empty timezone, changing the timezone to a different non-empty value is a metadata-only operation: the timestamp values need not change as their point of reference remains the same (the Unix epoch).

However, if a Timestamp column has no timezone value, changing it to a non-empty value requires to think about the desired semantics. One possibility is to assume that the original timestamp values are relative to the epoch of the timezone being set; timestamp values should then adjusted to the Unix epoch (for example, changing the timezone from empty to “Europe/Paris” would require converting the timestamp values from “Europe/Paris” to “UTC”, which seems counter-intuitive but is nevertheless correct).

```
DataType::Timestamp(TimeUnit::Second, None);
DataType::Timestamp(TimeUnit::Second, Some("literal".into()));
DataType::Timestamp(TimeUnit::Second, Some("string".to_string().into()));
```

#### [§](#timezone-representation)Timezone representation

---

It is possible to use either the timezone string representation, such as “UTC”, or the absolute time zone offset “+00:00”. For timezones with fixed offsets, such as “UTC” or “JST”, the offset representation is recommended, as it is more explicit and less ambiguous.

Most arrow-rs functionalities use the absolute offset representation, such as [`PrimitiveArray::with_timezone_utc`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.with_timezone_utc) that applies a UTC timezone to timestamp arrays.

##### [§](#timezone-string-parsing)Timezone string parsing

When feature `chrono-tz` is not enabled, allowed timezone strings are fixed offsets of the form “+09:00”, “-09” or “+0930”.

When feature `chrono-tz` is enabled, additional strings supported by [chrono_tz](https://docs.rs/chrono-tz/latest/chrono_tz/) are also allowed, which include [IANA database](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones) timezones.

[§](#variant.Date32)

A signed 32-bit date representing the elapsed time since UNIX epoch (1970-01-01) in days.

[§](#variant.Date64)

A signed 64-bit date representing the elapsed time since UNIX epoch (1970-01-01) in milliseconds.

#### [§](#valid-ranges)Valid Ranges

According to the Arrow specification ([Schema.fbs](https://github.com/apache/arrow/blob/main/format/Schema.fbs)), values of Date64 are treated as the number of _days_, in milliseconds, since the UNIX epoch. Therefore, values of this type must be evenly divisible by `86_400_000`, the number of milliseconds in a standard day.

It is not valid to store milliseconds that do not represent an exact day. The reason for this restriction is compatibility with other language’s native libraries (specifically Java), which historically lacked a dedicated date type and only supported timestamps.

#### [§](#validation)Validation

This library does not validate or enforce that Date64 values are evenly divisible by `86_400_000` for performance and usability reasons. Date64 values are treated similarly to `Timestamp(TimeUnit::Millisecond, None)`: values will be displayed with a time of day if the value does not represent an exact day, and arithmetic will be done at the millisecond granularity.

#### [§](#recommendation)Recommendation

Users should prefer [`Date32`](about:blank/enum.DataType.html#variant.Date32 "variant arrow_schema::DataType::Date32") to cleanly represent the number of days, or one of the Timestamp variants to include time as part of the representation, depending on their use case.

#### [§](#further-reading)Further Reading

For more details, see [#5288](https://github.com/apache/arrow-rs/issues/5288).

[§](#variant.Time32)

A signed 32-bit time representing the elapsed time since midnight in the unit of `TimeUnit`. Must be either seconds or milliseconds.

[§](#variant.Time64)

A signed 64-bit time representing the elapsed time since midnight in the unit of `TimeUnit`. Must be either microseconds or nanoseconds.

[§](#variant.Duration)

Measure of elapsed time in either seconds, milliseconds, microseconds or nanoseconds.

[§](#variant.Interval)

A “calendar” interval which models types that don’t necessarily have a precise duration without the context of a base timestamp (e.g. days can differ in length during day light savings time transitions).

[§](#variant.Binary)

Opaque binary data of variable length.

A single Binary array can store up to [`i32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i32.html#associatedconstant.MAX "associated constant i32::MAX") bytes of binary data in total.

[§](#variant.FixedSizeBinary)

Opaque binary data of fixed size. Enum parameter specifies the number of bytes per value.

[§](#variant.LargeBinary)

Opaque binary data of variable length and 64-bit offsets.

A single LargeBinary array can store up to [`i64::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i64.html#associatedconstant.MAX "associated constant i64::MAX") bytes of binary data in total.

[§](#variant.BinaryView)

Opaque binary data of variable length.

Logically the same as [`Binary`](about:blank/enum.DataType.html#variant.Binary "variant arrow_schema::DataType::Binary"), but the internal representation uses a view struct that contains the string length and either the string’s entire data inline (for small strings) or an inlined prefix, an index of another buffer, and an offset pointing to a slice in that buffer (for non-small strings).

[§](#variant.Utf8)

A variable-length string in Unicode with UTF-8 encoding.

A single Utf8 array can store up to [`i32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i32.html#associatedconstant.MAX "associated constant i32::MAX") bytes of string data in total.

[§](#variant.LargeUtf8)

A variable-length string in Unicode with UFT-8 encoding and 64-bit offsets.

A single LargeUtf8 array can store up to [`i64::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i64.html#associatedconstant.MAX "associated constant i64::MAX") bytes of string data in total.

[§](#variant.Utf8View)

A variable-length string in Unicode with UTF-8 encoding

Logically the same as [`Utf8`](about:blank/enum.DataType.html#variant.Utf8 "variant arrow_schema::DataType::Utf8"), but the internal representation uses a view struct that contains the string length and either the string’s entire data inline (for small strings) or an inlined prefix, an index of another buffer, and an offset pointing to a slice in that buffer (for non-small strings).

[§](#variant.List)

A list of some logical data type with variable length.

A single List array can store up to [`i32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i32.html#associatedconstant.MAX "associated constant i32::MAX") elements in total.

[§](#variant.ListView)

(NOT YET FULLY SUPPORTED) A list of some logical data type with variable length.

Logically the same as [`List`](about:blank/enum.DataType.html#variant.List "variant arrow_schema::DataType::List"), but the internal representation differs in how child data is referenced, allowing flexibility in how data is layed out.

Note this data type is not yet fully supported. Using it with arrow APIs may result in `panic`s.

[§](#variant.FixedSizeList)

A list of some logical data type with fixed length.

[§](#variant.LargeList)

A list of some logical data type with variable length and 64-bit offsets.

A single LargeList array can store up to [`i64::MAX`](https://doc.rust-lang.org/nightly/std/primitive.i64.html#associatedconstant.MAX "associated constant i64::MAX") elements in total.

[§](#variant.LargeListView)

(NOT YET FULLY SUPPORTED) A list of some logical data type with variable length and 64-bit offsets.

Logically the same as [`LargeList`](about:blank/enum.DataType.html#variant.LargeList "variant arrow_schema::DataType::LargeList"), but the internal representation differs in how child data is referenced, allowing flexibility in how data is layed out.

Note this data type is not yet fully supported. Using it with arrow APIs may result in `panic`s.

[§](#variant.Struct)

A nested datatype that contains a number of sub-fields.

[§](#variant.Union)

A nested datatype that can represent slots of differing types. Components:

1.  [`UnionFields`](struct.UnionFields.html "struct arrow_schema::UnionFields")
2.  The type of union (Sparse or Dense)

[§](#variant.Dictionary)

A dictionary encoded array (`key_type`, `value_type`), where each array element is an index of `key_type` into an associated dictionary of `value_type`.

Dictionary arrays are used to store columns of `value_type` that contain many repeated values using less memory, but with a higher CPU overhead for some operations.

This type mostly used to represent low cardinality string arrays or a limited set of primitive types as integers.

[§](#variant.Decimal32)

Exact 32-bit width decimal value with precision and scale

- precision is the total number of digits
- scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.

For example the number 12300 could be treated as a decimal has precision 3 and scale -2.

[§](#variant.Decimal64)

Exact 64-bit width decimal value with precision and scale

- precision is the total number of digits
- scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.

For example the number 12300 could be treated as a decimal has precision 3 and scale -2.

[§](#variant.Decimal128)

Exact 128-bit width decimal value with precision and scale

- precision is the total number of digits
- scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.

For example the number 12300 could be treated as a decimal has precision 3 and scale -2.

[§](#variant.Decimal256)

Exact 256-bit width decimal value with precision and scale

- precision is the total number of digits
- scale is the number of digits past the decimal

For example the number 123.45 has precision 5 and scale 2.

In certain situations, scale could be negative number. For negative scale, it is the number of padding 0 to the right of the digits.

For example the number 12300 could be treated as a decimal has precision 3 and scale -2.

[§](#variant.Map)

A Map is a logical nested type that is represented as

`List<entries: Struct<key: K, value: V>>`

The keys and values are each respectively contiguous. The key and value types are not constrained, but keys should be hashable and unique. Whether the keys are sorted can be set in the `bool` after the `Field`.

In a field with Map type, the field has a child Struct field, which then has two children: key type and the second the value type. The names of the child fields may be respectively “entries”, “key”, and “value”, but this is not enforced.

[§](#variant.RunEndEncoded)

A run-end encoding (REE) is a variation of run-length encoding (RLE). These encodings are well-suited for representing data containing sequences of the same value, called runs. Each run is represented as a value and an integer giving the index in the array where the run ends.

A run-end encoded array has no buffers by itself, but has two child arrays. The first child array, called the run ends array, holds either 16, 32, or 64-bit signed integers. The actual values of each run are held in the second child array.

These child arrays are prescribed the standard names of “run_ends” and “values” respectively.

[Source](about:blank/src/arrow_schema/datatype.rs.html#537-850)
[§](#impl-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#540-542)

Returns true if the type is primitive: (numeric, temporal).

[Source](about:blank/src/arrow_schema/datatype.rs.html#546-566)

Returns true if this type is numeric: (UInt\*, Int\*, Float\*, Decimal\*).

[Source](about:blank/src/arrow_schema/datatype.rs.html#570-576)

Returns true if this type is temporal: (Date\*, Time\*, Duration, or Interval).

[Source](about:blank/src/arrow_schema/datatype.rs.html#580-583)

Returns true if this type is floating: (Float\*).

[Source](about:blank/src/arrow_schema/datatype.rs.html#587-589)

Returns true if this type is integer: (Int\*, UInt\*).

[Source](about:blank/src/arrow_schema/datatype.rs.html#593-596)

Returns true if this type is signed integer: (Int\*).

[Source](about:blank/src/arrow_schema/datatype.rs.html#600-603)

Returns true if this type is unsigned integer: (UInt\*).

[Source](about:blank/src/arrow_schema/datatype.rs.html#607-609)

Returns true if this type is valid as a dictionary key

[Source](about:blank/src/arrow_schema/datatype.rs.html#613-616)

Returns true if this type is valid for run-ends array in RunArray

[Source](about:blank/src/arrow_schema/datatype.rs.html#621-636)

Returns true if this type is nested (List, FixedSizeList, LargeList, ListView. LargeListView, Struct, Union, or Map), or a dictionary of a nested type

[Source](about:blank/src/arrow_schema/datatype.rs.html#640-643)

Returns true if this type is DataType::Null.

[Source](about:blank/src/arrow_schema/datatype.rs.html#647-702)

Compares the datatype with another, ignoring nested field names and metadata.

[Source](about:blank/src/arrow_schema/datatype.rs.html#708-741)

Returns the byte width of this type if it is a primitive type

Returns `None` if not a primitive type

[Source](about:blank/src/arrow_schema/datatype.rs.html#746-794)

Return size of this instance in bytes.

Includes the size of `Self`.

[Source](about:blank/src/arrow_schema/datatype.rs.html#800-822)

Check to see if `self` is a superset of `other`

If DataType is a nested type, then it will check to see if the nested type is a superset of the other nested type else it will check to see if the DataType is equal to the other DataType

[Source](about:blank/src/arrow_schema/datatype.rs.html#829-831)

[Source](about:blank/src/arrow_schema/datatype.rs.html#838-840)

[Source](about:blank/src/arrow_schema/datatype.rs.html#847-849)

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#impl-Clone-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#impl-Debug-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#96)
[§](#impl-Deserialize%3C'de%3E-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#487-506)
[§](#impl-Display-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#521-527)
[§](#impl-FromStr-for-DataType)

Parses `str` into a `DataType`.

This is the reverse of [`DataType`](enum.DataType.html "enum arrow_schema::DataType")’s `Display` impl, and maintains the invariant that `DataType::try_from(&data_type.to_string()).unwrap() == data_type`

#### [§](#example)Example

```
use arrow_schema::DataType;

let data_type: DataType = "Int32".parse().unwrap();
assert_eq!(data_type, DataType::Int32);
```

[Source](about:blank/src/arrow_schema/datatype.rs.html#522)
[§](#associatedtype.Err)

The associated error which can be returned from parsing.

[Source](about:blank/src/arrow_schema/datatype.rs.html#524-526)
[§](#method.from_str)

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#impl-Hash-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#impl-Ord-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#impl-PartialEq-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#impl-PartialOrd-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#method.partial_cmp)

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398)
[§](#method.lt)

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416)
[§](#method.le)

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434)
[§](#method.gt)

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452)
[§](#method.ge)

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

[Source](about:blank/src/arrow_schema/datatype.rs.html#96)
[§](#impl-Serialize-for-DataType)

[Source](about:blank/src/arrow_schema/ffi.rs.html#645-686)
[§](#impl-TryFrom%3C%26DataType%3E-for-FFI_ArrowSchema)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#649-685)
[§](#method.try_from-2)

[Source](about:blank/src/arrow_schema/ffi.rs.html#646)
[§](#associatedtype.Error-2)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/ffi.rs.html#413-616)
[§](#impl-TryFrom%3C%26FFI_ArrowSchema%3E-for-DataType)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#417-615)
[§](#method.try_from-1)

[Source](about:blank/src/arrow_schema/ffi.rs.html#414)
[§](#associatedtype.Error-1)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/datatype.rs.html#529-535)
[§](#impl-TryFrom%3C%26str%3E-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#530)
[§](#associatedtype.Error)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/datatype.rs.html#532-534)
[§](#method.try_from)

Performs the conversion.

[Source](about:blank/src/arrow_schema/ffi.rs.html#803-809)
[§](#impl-TryFrom%3CDataType%3E-for-FFI_ArrowSchema)

Available on **crate feature `ffi`** only.

[Source](about:blank/src/arrow_schema/ffi.rs.html#804)
[§](#associatedtype.Error-3)

The type returned in the event of a conversion error.

[Source](about:blank/src/arrow_schema/ffi.rs.html#806-808)
[§](#method.try_from-3)

Performs the conversion.

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#impl-Eq-for-DataType)

[Source](about:blank/src/arrow_schema/datatype.rs.html#95)
[§](#impl-StructuralPartialEq-for-DataType)

[§](#impl-Freeze-for-DataType)

[§](#impl-RefUnwindSafe-for-DataType)

[§](#impl-Send-for-DataType)

[§](#impl-Sync-for-DataType)

[§](#impl-Unpin-for-DataType)

[§](#impl-UnwindSafe-for-DataType)
