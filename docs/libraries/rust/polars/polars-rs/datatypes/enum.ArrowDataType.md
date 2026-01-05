# Enum ArrowDataType Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/datatypes/mod.rs.html#39" class="src">Source</a>

``` rust
pub enum ArrowDataType {
Show 41 variants    Null,
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float16,
    Float32,
    Float64,
    Timestamp(TimeUnit, Option<PlSmallStr>),
    Date32,
    Date64,
    Time32(TimeUnit),
    Time64(TimeUnit),
    Duration(TimeUnit),
    Interval(IntervalUnit),
    Binary,
    FixedSizeBinary(usize),
    LargeBinary,
    Utf8,
    LargeUtf8,
    List(Box<Field>),
    FixedSizeList(Box<Field>, usize),
    LargeList(Box<Field>),
    Struct(Vec<Field>),
    Map(Box<Field>, bool),
    Dictionary(IntegerType, Box<ArrowDataType>, bool),
    Decimal(usize, usize),
    Decimal32(usize, usize),
    Decimal64(usize, usize),
    Decimal256(usize, usize),
    Extension(Box<ExtensionType>),
    BinaryView,
    Utf8View,
    Unknown,
    Union(Box<UnionType>),
}
```

Expand description

The set of supported logical types in this crate.

Each variant uniquely identifies a logical type, which define specific semantics to the data (e.g. how it should be represented). Each variant has a corresponding [`PhysicalType`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.PhysicalType.html "enum polars_arrow::datatypes::physical_type::PhysicalType"), obtained via [`ArrowDataType::to_physical_type`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html#method.to_physical_type "method polars::prelude::ArrowDataType::to_physical_type"), which declares the in-memory representation of data. The [`ArrowDataType::Extension`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html#variant.Extension "variant polars::prelude::ArrowDataType::Extension") is special in that it augments a [`ArrowDataType`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html "enum polars::prelude::ArrowDataType") with metadata to support custom types. Use `to_logical_type` to desugar such type and return its corresponding logical type.

## Variants<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Null" class="anchor">§</a>

### Null

Null type

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Boolean" class="anchor">§</a>

### Boolean

`true` and `false`.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Int8" class="anchor">§</a>

### Int8

An [`i8`](https://doc.rust-lang.org/nightly/std/primitive.i8.html "primitive i8")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Int16" class="anchor">§</a>

### Int16

An [`i16`](https://doc.rust-lang.org/nightly/std/primitive.i16.html "primitive i16")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Int32" class="anchor">§</a>

### Int32

An [`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html "primitive i32")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Int64" class="anchor">§</a>

### Int64

An [`i64`](https://doc.rust-lang.org/nightly/std/primitive.i64.html "primitive i64")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Int128" class="anchor">§</a>

### Int128

An [`i128`](https://doc.rust-lang.org/nightly/std/primitive.i128.html "primitive i128")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.UInt8" class="anchor">§</a>

### UInt8

An [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.UInt16" class="anchor">§</a>

### UInt16

An [`u16`](https://doc.rust-lang.org/nightly/std/primitive.u16.html "primitive u16")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.UInt32" class="anchor">§</a>

### UInt32

An [`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html "primitive u32")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.UInt64" class="anchor">§</a>

### UInt64

An [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html "primitive u64")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Float16" class="anchor">§</a>

### Float16

An 16-bit float

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Float32" class="anchor">§</a>

### Float32

A [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Float64" class="anchor">§</a>

### Float64

A [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64")

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Timestamp" class="anchor">§</a>

### Timestamp(<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html" class="enum" title="enum polars::prelude::ArrowTimeUnit">TimeUnit</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>)

A [`i64`](https://doc.rust-lang.org/nightly/std/primitive.i64.html "primitive i64") representing a timestamp measured in [`TimeUnit`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html "enum polars::prelude::ArrowTimeUnit") with an optional timezone.

Time is measured as a Unix epoch, counting the seconds from 00:00:00.000 on 1 January 1970, excluding leap seconds, as a 64-bit signed integer.

The time zone is a string indicating the name of a time zone, one of:

- As used in the Olson time zone database (the “tz database” or “tzdata”), such as “America/New_York”
- An absolute time zone offset of the form +XX:XX or -XX:XX, such as +07:30

When the timezone is not specified, the timestamp is considered to have no timezone and is represented *as is*

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Date32" class="anchor">§</a>

### Date32

An [`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html "primitive i32") representing the elapsed time since UNIX epoch (1970-01-01) in days.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Date64" class="anchor">§</a>

### Date64

An [`i64`](https://doc.rust-lang.org/nightly/std/primitive.i64.html "primitive i64") representing the elapsed time since UNIX epoch (1970-01-01) in milliseconds. Values are evenly divisible by 86400000.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Time32" class="anchor">§</a>

### Time32(<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html" class="enum" title="enum polars::prelude::ArrowTimeUnit">TimeUnit</a>)

A 32-bit time representing the elapsed time since midnight in the unit of `TimeUnit`. Only [`TimeUnit::Second`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html#variant.Second "variant polars::prelude::ArrowTimeUnit::Second") and [`TimeUnit::Millisecond`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html#variant.Millisecond "variant polars::prelude::ArrowTimeUnit::Millisecond") are supported on this variant.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Time64" class="anchor">§</a>

### Time64(<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html" class="enum" title="enum polars::prelude::ArrowTimeUnit">TimeUnit</a>)

A 64-bit time representing the elapsed time since midnight in the unit of `TimeUnit`. Only [`TimeUnit::Microsecond`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html#variant.Microsecond "variant polars::prelude::ArrowTimeUnit::Microsecond") and [`TimeUnit::Nanosecond`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html#variant.Nanosecond "variant polars::prelude::ArrowTimeUnit::Nanosecond") are supported on this variant.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Duration" class="anchor">§</a>

### Duration(<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowTimeUnit.html" class="enum" title="enum polars::prelude::ArrowTimeUnit">TimeUnit</a>)

Measure of elapsed time. This elapsed time is a physical duration (i.e. 1s as defined in S.I.)

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Interval" class="anchor">§</a>

### Interval(<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/enum.IntervalUnit.html" class="enum" title="enum polars_arrow::datatypes::IntervalUnit">IntervalUnit</a>)

A “calendar” interval modeling elapsed time that takes into account calendar shifts. For example an interval of 1 day may represent more than 24 hours.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Binary" class="anchor">§</a>

### Binary

Opaque binary data of variable length whose offsets are represented as [`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html "primitive i32").

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.FixedSizeBinary" class="anchor">§</a>

### FixedSizeBinary(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Opaque binary data of fixed size. Enum parameter specifies the number of bytes per value.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.LargeBinary" class="anchor">§</a>

### LargeBinary

Opaque binary data of variable length whose offsets are represented as [`i64`](https://doc.rust-lang.org/nightly/std/primitive.i64.html "primitive i64").

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Utf8" class="anchor">§</a>

### Utf8

A variable-length UTF-8 encoded string whose offsets are represented as [`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html "primitive i32").

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.LargeUtf8" class="anchor">§</a>

### LargeUtf8

A variable-length UTF-8 encoded string whose offsets are represented as [`i64`](https://doc.rust-lang.org/nightly/std/primitive.i64.html "primitive i64").

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>)

A list of some logical data type whose offsets are represented as [`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html "primitive i32").

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.FixedSizeList" class="anchor">§</a>

### FixedSizeList(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

A list of some logical data type with a fixed number of elements.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.LargeList" class="anchor">§</a>

### LargeList(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>)

A list of some logical data type whose offsets are represented as [`i64`](https://doc.rust-lang.org/nightly/std/primitive.i64.html "primitive i64").

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>)

A nested [`ArrowDataType`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html "enum polars::prelude::ArrowDataType") with a given number of [`Field`](https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html "struct polars::prelude::ArrowField")s.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Map" class="anchor">§</a>

### Map(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

A nested type that is represented as

List\<entries: Struct\<key: K, value: V\>\>

In this layout, the keys and values are each respectively contiguous. We do not constrain the key and value types, so the application is responsible for ensuring that the keys are hashable and unique. Whether the keys are sorted may be set in the metadata for this field.

In a field with Map type, the field has a child Struct field, which then has two children: key type and the second the value type. The names of the child fields may be respectively “entries”, “key”, and “value”, but this is not enforced.

Map

``` text
  - child[0] entries: Struct
    - child[0] key: K
    - child[1] value: V
```

Neither the “entries” field nor the “key” field may be nullable.

The metadata is structured so that Arrow systems without special handling for Map can make Map an alias for List. The “layout” attribute for the Map field must have the same contents as a List.

- Field
- ordered

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Dictionary" class="anchor">§</a>

### Dictionary(<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.IntegerType.html" class="enum" title="enum polars_arrow::datatypes::physical_type::IntegerType">IntegerType</a>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

A dictionary encoded array (`key_type`, `value_type`), where each array element is an index of `key_type` into an associated dictionary of `value_type`.

Dictionary arrays are used to store columns of `value_type` that contain many repeated values using less memory, but with a higher CPU overhead for some operations.

This type mostly used to represent low cardinality string arrays or a limited set of primitive types as integers.

The `bool` value indicates the `Dictionary` is sorted if set to `true`.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Decimal" class="anchor">§</a>

### Decimal(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Decimal value with precision and scale precision is the number of digits in the number and scale is the number of decimal places. The number 999.99 has a precision of 5 and scale of 2.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Decimal32" class="anchor">§</a>

### Decimal32(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Decimal backed by 32 bits

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Decimal64" class="anchor">§</a>

### Decimal64(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Decimal backed by 64 bits

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Decimal256" class="anchor">§</a>

### Decimal256(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Decimal backed by 256 bits

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Extension" class="anchor">§</a>

### Extension(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/struct.ExtensionType.html" class="struct" title="struct polars_arrow::datatypes::ExtensionType">ExtensionType</a>\>)

Extension type.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.BinaryView" class="anchor">§</a>

### BinaryView

A binary type that inlines small values and can intern bytes.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Utf8View" class="anchor">§</a>

### Utf8View

A string type that inlines small values and can intern strings.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Unknown" class="anchor">§</a>

### Unknown

A type unknown to Arrow.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#variant.Union" class="anchor">§</a>

### Union(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/struct.UnionType.html" class="struct" title="struct polars_arrow::datatypes::UnionType">UnionType</a>\>)

A nested datatype that can represent slots of differing types. Third argument represents mode

## Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-ArrowDataType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#associatedconstant.IDX_DTYPE" class="constant">IDX_DTYPE</a>: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a> = { { ArrowDataType::UInt32 } }

Polars IdxSize type, dependent on bigidx feature

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.to_physical_type" class="fn">to_physical_type</a>(&self) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.PhysicalType.html" class="enum" title="enum polars_arrow::datatypes::physical_type::PhysicalType">PhysicalType</a>

the [`PhysicalType`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.PhysicalType.html "enum polars_arrow::datatypes::physical_type::PhysicalType") of this [`ArrowDataType`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html "enum polars::prelude::ArrowDataType").

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.underlying_physical_type" class="fn">underlying_physical_type</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.to_logical_type" class="fn">to_logical_type</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

Returns `&self` for all but [`ArrowDataType::Extension`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html#variant.Extension "variant polars::prelude::ArrowDataType::Extension"). For [`ArrowDataType::Extension`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html#variant.Extension "variant polars::prelude::ArrowDataType::Extension"), (recursively) returns the inner [`ArrowDataType`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html "enum polars::prelude::ArrowDataType"). Never returns the variant [`ArrowDataType::Extension`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html#variant.Extension "variant polars::prelude::ArrowDataType::Extension").

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.inner_dtype" class="fn">inner_dtype</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.is_nested" class="fn">is_nested</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.is_view" class="fn">is_view</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.is_numeric" class="fn">is_numeric</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.to_fixed_size_list" class="fn">to_fixed_size_list</a>(self, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, is_nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.contains_dictionary" class="fn">contains_dictionary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check (recursively) whether datatype contains an [`ArrowDataType::Dictionary`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html#variant.Dictionary "variant polars::prelude::ArrowDataType::Dictionary") type.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-Clone-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-Debug-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-Default-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-Deserialize%3C&#39;de%3E-for-ArrowDataType" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-From%3CIntegerType%3E-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.IntegerType.html" class="enum" title="enum polars_arrow::datatypes::physical_type::IntegerType">IntegerType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(item: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/datatypes/physical_type/enum.IntegerType.html" class="enum" title="enum polars_arrow::datatypes::physical_type::IntegerType">IntegerType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-From%3CPrimitiveType%3E-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/types/enum.PrimitiveType.html" class="enum" title="enum polars_arrow::types::PrimitiveType">PrimitiveType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(item: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/types/enum.PrimitiveType.html" class="enum" title="enum polars_arrow::types::PrimitiveType">PrimitiveType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-Hash-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-PartialEq-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-Serialize-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-Eq-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#impl-StructuralPartialEq-for-ArrowDataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/enum.ArrowDataType.html#blanket-implementations" class="anchor">§</a>
