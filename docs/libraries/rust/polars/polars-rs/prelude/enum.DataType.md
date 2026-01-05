# Enum DataType Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/dtype.rs.html#88" class="src">Source</a>

``` rust
pub enum DataType {
Show 28 variants    Boolean,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Float32,
    Float64,
    Decimal(Option<usize>, Option<usize>),
    String,
    Binary,
    BinaryOffset,
    Date,
    Datetime(TimeUnit, Option<TimeZone>),
    Duration(TimeUnit),
    Time,
    Array(Box<DataType>, usize),
    List(Box<DataType>),
    Object(&'static str),
    Null,
    Categorical(Arc<Categories>, Arc<CategoricalMapping>),
    Enum(Arc<FrozenCategories>, Arc<CategoricalMapping>),
    Struct(Vec<Field>),
    Unknown(UnknownKind),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Boolean" class="anchor">§</a>

### Boolean

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt8" class="anchor">§</a>

### UInt8

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt16" class="anchor">§</a>

### UInt16

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt32" class="anchor">§</a>

### UInt32

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.UInt64" class="anchor">§</a>

### UInt64

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int8" class="anchor">§</a>

### Int8

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int16" class="anchor">§</a>

### Int16

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int32" class="anchor">§</a>

### Int32

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int64" class="anchor">§</a>

### Int64

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Int128" class="anchor">§</a>

### Int128

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Float32" class="anchor">§</a>

### Float32

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Float64" class="anchor">§</a>

### Float64

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Decimal" class="anchor">§</a>

### Decimal(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Fixed point decimal type optional precision and non-negative scale. This is backed by a signed 128-bit integer which allows for up to 38 significant digits. Meaning max precision is 38.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.String" class="anchor">§</a>

### String

String data

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Binary" class="anchor">§</a>

### Binary

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.BinaryOffset" class="anchor">§</a>

### BinaryOffset

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Date" class="anchor">§</a>

### Date

A 32-bit date representing the elapsed time since UNIX epoch (1970-01-01) in days (32 bits).

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Datetime" class="anchor">§</a>

### Datetime(<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>)

A 64-bit date representing the elapsed time since UNIX epoch (1970-01-01) in the given timeunit (64 bits).

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Duration" class="anchor">§</a>

### Duration(<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>)

64-bit integer representing difference between times in milliseconds or nanoseconds

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Time" class="anchor">§</a>

### Time

A 64-bit time representing the elapsed time since midnight in nanoseconds

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Array" class="anchor">§</a>

### Array(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

A nested list with a fixed size in each row

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>)

A nested list with a variable size in each row

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Object" class="anchor">§</a>

### Object(&'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

A generic type that can be used in a `Series` &’static str can be used to determine/set inner type

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Null" class="anchor">§</a>

### Null

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Categorical" class="anchor">§</a>

### Categorical(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Enum" class="anchor">§</a>

### Enum(<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FrozenCategories.html" class="struct" title="struct polars::prelude::FrozenCategories">FrozenCategories</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Unknown" class="anchor">§</a>

### Unknown(<a href="https://docs.rs/polars/latest/polars/prelude/enum.UnknownKind.html" class="enum" title="enum polars::prelude::UnknownKind">UnknownKind</a>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-DataType" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#associatedconstant.IDX_DTYPE" class="constant">IDX_DTYPE</a>: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a> = { { DataType::UInt32 } }

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.value_within_range" class="fn">value_within_range</a>(&self, other: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method._month_days_ns_struct_type" class="fn">_month_days_ns_struct_type</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Struct representation of the arrow `month_day_nano_interval` type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_known" class="fn">is_known</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if the whole dtype is known.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.materialize_unknown" class="fn">materialize_unknown</a>( self, allow_unknown: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Materialize this datatype if it is unknown. All other datatypes are left unchanged.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.get_shape" class="fn">get_shape</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

Get the full shape of a multidimensional array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.inner_dtype" class="fn">inner_dtype</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

Get the inner data type of a nested type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.into_inner_dtype" class="fn">into_inner_dtype</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

Get the inner data type of a nested type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.try_into_inner_dtype" class="fn">try_into_inner_dtype</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the inner data type of a nested type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.leaf_dtype" class="fn">leaf_dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Get the absolute inner data type of a nested type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.array_leaf_dtype" class="fn">array_leaf_dtype</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

Get the inner data type of a multidimensional array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.cast_leaf" class="fn">cast_leaf</a>(&self, to: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Cast the leaf types of Lists/Arrays and keep the nesting.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.can_cast_to" class="fn">can_cast_to</a>(&self, to: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Return whether the cast to `to` makes sense.

If it `None`, we are not sure.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.implode" class="fn">implode</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.to_physical" class="fn">to_physical</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Convert to the physical data type

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_supported_list_arithmetic_input" class="fn">is_supported_list_arithmetic_input</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_logical" class="fn">is_logical</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is a logical type

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_temporal" class="fn">is_temporal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is a temporal type

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_primitive" class="fn">is_primitive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if datatype is a primitive type. By that we mean that it is not a nested or logical type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_primitive_numeric" class="fn">is_primitive_numeric</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is a primitive numeric type (excludes Decimal).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_bool" class="fn">is_bool</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is a boolean.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_list" class="fn">is_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is a list.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_array" class="fn">is_array</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is an array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_nested" class="fn">is_nested</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_struct" class="fn">is_struct</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is a struct

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_binary" class="fn">is_binary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_date" class="fn">is_date</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_datetime" class="fn">is_datetime</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_duration" class="fn">is_duration</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_object" class="fn">is_object</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_null" class="fn">is_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.contains_views" class="fn">contains_views</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.contains_categoricals" class="fn">contains_categoricals</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.contains_objects" class="fn">contains_objects</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.contains_list_recursive" class="fn">contains_list_recursive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.contains_unknown" class="fn">contains_unknown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_ord" class="fn">is_ord</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if type is sortable

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_decimal" class="fn">is_decimal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is a Decimal type (of any scale/precision).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_float" class="fn">is_float</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is a basic floating point type (excludes Decimal). Note, this also includes `Unknown(UnknownKind::Float)`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_integer" class="fn">is_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType") is an integer. Note, this also includes `Unknown(UnknownKind::Int)`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_signed_integer" class="fn">is_signed_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_unsigned_integer" class="fn">is_unsigned_integer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_string" class="fn">is_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_categorical" class="fn">is_categorical</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_enum" class="fn">is_enum</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.to_arrow_field" class="fn">to_arrow_field</a>( &self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>

Convert to an Arrow Field.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.max" class="fn">max</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Try to get the maximum value for this datatype.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.min" class="fn">min</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html" class="struct" title="struct polars::prelude::Scalar">Scalar</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Try to get the minimum value for this datatype.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.to_arrow" class="fn">to_arrow</a>(&self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

Convert to an Arrow data type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.try_to_arrow" class="fn">try_to_arrow</a>( &self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_nested_null" class="fn">is_nested_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.matches_schema_type" class="fn">matches_schema_type</a>( &self, schema_type: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Answers if this type matches the given type of a schema.

Allows (nested) Null types in this type to match any type in the schema, but not vice versa. In such a case Ok(true) is returned, because a cast is necessary. If no cast is necessary Ok(false) is returned, and an error is returned if the types are incompatible.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_unknown" class="fn">is_unknown</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.nesting_level" class="fn">nesting_level</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.cat_physical" class="fn">cat_physical</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.CategoricalPhysical.html" class="enum" title="enum polars::prelude::CategoricalPhysical">CategoricalPhysical</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

If this dtype is a Categorical or Enum, returns the physical backing type.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.cat_mapping" class="fn">cat_mapping</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CategoricalMapping.html" class="struct" title="struct polars::prelude::CategoricalMapping">CategoricalMapping</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

If this dtype is a Categorical or Enum, returns the underlying mapping.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from_categories" class="fn">from_categories</a>(cats: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Categories.html" class="struct" title="struct polars::prelude::Categories">Categories</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from_frozen_categories" class="fn">from_frozen_categories</a>(fcats: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FrozenCategories.html" class="struct" title="struct polars::prelude::FrozenCategories">FrozenCategories</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.is_numeric" class="fn">is_numeric</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-DataType-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.boxed" class="fn">boxed</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from_arrow_field" class="fn">from_arrow_field</a>(field: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from_arrow_dtype" class="fn">from_arrow_dtype</a>(dt: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from_arrow" class="fn">from_arrow</a>( dt: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>, md: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-AsRef%3CDataType%3E-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.as_ref-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-AsRef%3CDataType%3E-for-Field" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-Clone-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-Debug-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-Default-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-Deserialize%3C&#39;a%3E-for-DataType" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'a\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>( deserializer: D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, \<D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'a\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'a\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-Display-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-From%3C%26AnyValue%3C&#39;a%3E%3E-for-DataType" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-From%3CAnyValue%3C&#39;_%3E%3E-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-From%3CDataType%3E-for-DataTypeExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-From%3CSerializableDataType%3E-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<SerializableDataType\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(dt: SerializableDataType) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-Hash-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-PartialEq-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-Serialize-for-DataType" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>( &self, serializer: S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#impl-Eq-for-DataType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#blanket-implementations" class="anchor">§</a>
