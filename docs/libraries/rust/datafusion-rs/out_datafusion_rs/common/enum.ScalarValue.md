# Enum ScalarValue Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/scalar/mod.rs.html#223" class="src">Source</a>

``` rust
pub enum ScalarValue {
Show 46 variants    Null,
    Boolean(Option<bool>),
    Float16(Option<f16>),
    Float32(Option<f32>),
    Float64(Option<f64>),
    Decimal128(Option<i128>, u8, i8),
    Decimal256(Option<i256>, u8, i8),
    Int8(Option<i8>),
    Int16(Option<i16>),
    Int32(Option<i32>),
    Int64(Option<i64>),
    UInt8(Option<u8>),
    UInt16(Option<u16>),
    UInt32(Option<u32>),
    UInt64(Option<u64>),
    Utf8(Option<String>),
    Utf8View(Option<String>),
    LargeUtf8(Option<String>),
    Binary(Option<Vec<u8>>),
    BinaryView(Option<Vec<u8>>),
    FixedSizeBinary(i32, Option<Vec<u8>>),
    LargeBinary(Option<Vec<u8>>),
    FixedSizeList(Arc<FixedSizeListArray>),
    List(Arc<GenericListArray<i32>>),
    LargeList(Arc<GenericListArray<i64>>),
    Struct(Arc<StructArray>),
    Map(Arc<MapArray>),
    Date32(Option<i32>),
    Date64(Option<i64>),
    Time32Second(Option<i32>),
    Time32Millisecond(Option<i32>),
    Time64Microsecond(Option<i64>),
    Time64Nanosecond(Option<i64>),
    TimestampSecond(Option<i64>, Option<Arc<str>>),
    TimestampMillisecond(Option<i64>, Option<Arc<str>>),
    TimestampMicrosecond(Option<i64>, Option<Arc<str>>),
    TimestampNanosecond(Option<i64>, Option<Arc<str>>),
    IntervalYearMonth(Option<i32>),
    IntervalDayTime(Option<IntervalDayTime>),
    IntervalMonthDayNano(Option<IntervalMonthDayNano>),
    DurationSecond(Option<i64>),
    DurationMillisecond(Option<i64>),
    DurationMicrosecond(Option<i64>),
    DurationNanosecond(Option<i64>),
    Union(Option<(i8, Box<ScalarValue>)>, UnionFields, UnionMode),
    Dictionary(Box<DataType>, Box<ScalarValue>),
}
```

Expand description

A dynamically typed, nullable single value.

While an arrow [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array")) stores one or more values of the same type, in a single column, a `ScalarValue` stores a single value of a single type, the equivalent of 1 row and one column.

``` text
 ┌────────┐
 │ value1 │
 │ value2 │                  ┌────────┐
 │ value3 │                  │ value2 │
 │  ...   │                  └────────┘
 │ valueN │
 └────────┘

   Array                     ScalarValue

stores multiple,             stores a single,
possibly null, values of     possible null, value
the same type
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#performance" class="doc-anchor">§</a>Performance

In general, performance will be better using arrow [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array")s rather than [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue"), as it is far more efficient to process multiple values at once (vectorized processing).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#example" class="doc-anchor">§</a>Example

``` rust
// Create single scalar value for an Int32 value
let s1 = ScalarValue::Int32(Some(10));

// You can also create values using the From impl:
let s2 = ScalarValue::from(10i32);
assert_eq!(s1, s2);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#null-handling" class="doc-anchor">§</a>Null Handling

`ScalarValue` represents null values in the same way as Arrow. Nulls are “typed” in the sense that a null value in an [`Int32Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.Int32Array.html "type datafusion::common::arrow::array::Int32Array") is different from a null value in a [`Float64Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.Float64Array.html "type datafusion::common::arrow::array::Float64Array"), and is different from the values in a [`NullArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html "struct datafusion::common::arrow::array::NullArray").

``` rust
// You can create a 'null' Int32 value directly:
let s1 = ScalarValue::Int32(None);

// You can also create a null value for a given datatype:
let s2 = ScalarValue::try_from(&DataType::Int32)?;
assert_eq!(s1, s2);

// Note that this is DIFFERENT than a `ScalarValue::Null`
let s3 = ScalarValue::Null;
assert_ne!(s1, s3);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#nested-types" class="doc-anchor">§</a>Nested Types

`List` / `LargeList` / `FixedSizeList` / `Struct` / `Map` are represented as a single element array of the corresponding type.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#example-creating-scalarvaluestruct-using-scalarstructbuilder" class="doc-anchor">§</a>Example: Creating [`ScalarValue::Struct`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Struct "variant datafusion::scalar::ScalarValue::Struct") using [`ScalarStructBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html "struct datafusion::common::scalar::ScalarStructBuilder")

``` rust
// Build a struct like: {a: 1, b: "foo"}
let field_a = Field::new("a", DataType::Int32, false);
let field_b = Field::new("b", DataType::Utf8, false);

let s1 = ScalarStructBuilder::new()
   .with_scalar(field_a, ScalarValue::from(1i32))
   .with_scalar(field_b, ScalarValue::from("foo"))
   .build();
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#example-creating-a-null-scalarvaluestruct-using-scalarstructbuilder" class="doc-anchor">§</a>Example: Creating a null [`ScalarValue::Struct`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Struct "variant datafusion::scalar::ScalarValue::Struct") using [`ScalarStructBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/scalar/struct.ScalarStructBuilder.html "struct datafusion::common::scalar::ScalarStructBuilder")

``` rust
// Build a struct representing a NULL value
let fields = vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, false),
];

let s1 = ScalarStructBuilder::new_null(fields);
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#example-creating-scalarvaluestruct-directly" class="doc-anchor">§</a>Example: Creating [`ScalarValue::Struct`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Struct "variant datafusion::scalar::ScalarValue::Struct") directly

``` rust
// Build a struct like: {a: 1, b: "foo"}
// Field description
let fields = Fields::from(vec![
  Field::new("a", DataType::Int32, false),
  Field::new("b", DataType::Utf8, false),
]);
// one row arrays for each field
let arrays: Vec<ArrayRef> = vec![
  Arc::new(Int32Array::from(vec![1])),
  Arc::new(StringArray::from(vec!["foo"])),
];
// no nulls for this array
let nulls = None;
let arr = StructArray::new(fields, arrays, nulls);

// Create a ScalarValue::Struct directly
let s1 = ScalarValue::Struct(Arc::new(arr));
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#further-reading" class="doc-anchor">§</a>Further Reading

See [datatypes](https://arrow.apache.org/docs/python/api/datatypes.html) for details on datatypes and the [format](https://github.com/apache/arrow/blob/master/format/Schema.fbs#L354-L375) for the definitive reference.

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Null" class="anchor">§</a>

### Null

represents `DataType::Null` (castable to/from any other type)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Boolean" class="anchor">§</a>

### Boolean(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>)

true or false value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Float16" class="anchor">§</a>

### Float16(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/half/2.6.0/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html" class="struct" title="struct half::binary16::f16">f16</a>\>)

16bit float

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Float32" class="anchor">§</a>

### Float32(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>)

32bit float

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Float64" class="anchor">§</a>

### Float64(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>)

64bit float

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Decimal128" class="anchor">§</a>

### Decimal128(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>)

128bit decimal, using the i128 to represent the decimal, precision scale

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Decimal256" class="anchor">§</a>

### Decimal256(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>\>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>)

256bit decimal, using the i256 to represent the decimal, precision scale

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Int8" class="anchor">§</a>

### Int8(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>)

signed 8bit int

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Int16" class="anchor">§</a>

### Int16(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>)

signed 16bit int

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Int32" class="anchor">§</a>

### Int32(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>)

signed 32bit int

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Int64" class="anchor">§</a>

### Int64(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>)

signed 64bit int

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.UInt8" class="anchor">§</a>

### UInt8(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>)

unsigned 8bit int

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.UInt16" class="anchor">§</a>

### UInt16(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>)

unsigned 16bit int

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.UInt32" class="anchor">§</a>

### UInt32(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>)

unsigned 32bit int

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.UInt64" class="anchor">§</a>

### UInt64(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>)

unsigned 64bit int

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Utf8" class="anchor">§</a>

### Utf8(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

utf-8 encoded string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Utf8View" class="anchor">§</a>

### Utf8View(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

utf-8 encoded string but from view types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.LargeUtf8" class="anchor">§</a>

### LargeUtf8(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

utf-8 encoded string representing a LargeString’s arrow type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Binary" class="anchor">§</a>

### Binary(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>)

binary

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.BinaryView" class="anchor">§</a>

### BinaryView(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>)

binary but from view types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.FixedSizeBinary" class="anchor">§</a>

### FixedSizeBinary(<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>)

fixed size binary

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.LargeBinary" class="anchor">§</a>

### LargeBinary(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>)

large binary

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.FixedSizeList" class="anchor">§</a>

### FixedSizeList(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct datafusion::common::arrow::array::FixedSizeListArray">FixedSizeListArray</a>\>)

Fixed size list scalar.

The array must be a FixedSizeListArray with length 1.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.List" class="anchor">§</a>

### List(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>)

Represents a single element of a [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray") as an [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef")

The array must be a ListArray with length 1.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.LargeList" class="anchor">§</a>

### LargeList(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>)

The array must be a LargeListArray with length 1.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>\>)

Represents a single element [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray") as an [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef"). See [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") for examples of how to create instances of this type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Map" class="anchor">§</a>

### Map(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>\>)

Represents a single element [`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray") as an [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Date32" class="anchor">§</a>

### Date32(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>)

Date stored as a signed 32bit int days since UNIX epoch 1970-01-01

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Date64" class="anchor">§</a>

### Date64(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>)

Date stored as a signed 64bit int milliseconds since UNIX epoch 1970-01-01

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Time32Second" class="anchor">§</a>

### Time32Second(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>)

Time stored as a signed 32bit int as seconds since midnight

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Time32Millisecond" class="anchor">§</a>

### Time32Millisecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>)

Time stored as a signed 32bit int as milliseconds since midnight

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Time64Microsecond" class="anchor">§</a>

### Time64Microsecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>)

Time stored as a signed 64bit int as microseconds since midnight

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Time64Nanosecond" class="anchor">§</a>

### Time64Nanosecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>)

Time stored as a signed 64bit int as nanoseconds since midnight

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.TimestampSecond" class="anchor">§</a>

### TimestampSecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>)

Timestamp Second

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.TimestampMillisecond" class="anchor">§</a>

### TimestampMillisecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>)

Timestamp Milliseconds

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.TimestampMicrosecond" class="anchor">§</a>

### TimestampMicrosecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>)

Timestamp Microseconds

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.TimestampNanosecond" class="anchor">§</a>

### TimestampNanosecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>)

Timestamp Nanoseconds

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.IntervalYearMonth" class="anchor">§</a>

### IntervalYearMonth(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>)

Number of elapsed whole months

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.IntervalDayTime" class="anchor">§</a>

### IntervalDayTime(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalDayTime.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalDayTime">IntervalDayTime</a>\>)

Number of elapsed days and milliseconds (no leap seconds) stored as 2 contiguous 32-bit signed integers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.IntervalMonthDayNano" class="anchor">§</a>

### IntervalMonthDayNano(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.IntervalMonthDayNano.html" class="struct" title="struct datafusion::common::arrow::datatypes::IntervalMonthDayNano">IntervalMonthDayNano</a>\>)

A triple of the number of elapsed months, days, and nanoseconds. Months and days are encoded as 32-bit signed integers. Nanoseconds is encoded as a 64-bit signed integer (no leap seconds).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.DurationSecond" class="anchor">§</a>

### DurationSecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>)

Duration in seconds

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.DurationMillisecond" class="anchor">§</a>

### DurationMillisecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>)

Duration in milliseconds

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.DurationMicrosecond" class="anchor">§</a>

### DurationMicrosecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>)

Duration in microseconds

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.DurationNanosecond" class="anchor">§</a>

### DurationNanosecond(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>)

Duration in nanoseconds

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Union" class="anchor">§</a>

### Union(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>)\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.UnionFields.html" class="struct" title="struct datafusion::common::arrow::datatypes::UnionFields">UnionFields</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.UnionMode.html" class="enum" title="enum datafusion::common::arrow::datatypes::UnionMode">UnionMode</a>)

A nested datatype that can represent slots of differing types. Components: `.0`: a tuple of union `type_id` and the single value held by this Scalar `.1`: the list of fields, zero-to-one of which will by set in `.0` `.2`: the physical storage of the source/destination UnionArray from which this Scalar came

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#variant.Dictionary" class="anchor">§</a>

### Dictionary(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>)

Dictionary type: index type and value

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-ScalarValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_primitive" class="fn">new_primitive</a>\<T\>( a: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>\>, d: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Create a [`Result<ScalarValue>`](https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html "type datafusion::error::Result") with the provided value and datatype

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#panics" class="doc-anchor">§</a>Panics

Panics if d is not compatible with T

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_new_decimal128" class="fn">try_new_decimal128</a>( value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i128.html" class="primitive">i128</a>, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, scale: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a decimal Scalar from value/precision and scale.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_new_null" class="fn">try_new_null</a>( data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a Null instance of ScalarValue for this datatype

Example

``` rust
use datafusion_common::ScalarValue;
use arrow::datatypes::DataType;

let scalar = ScalarValue::try_new_null(&DataType::Int32).unwrap();
assert_eq!(scalar.is_null(), true);
assert_eq!(scalar.data_type(), DataType::Int32);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_utf8" class="fn">new_utf8</a>(val: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Returns a [`ScalarValue::Utf8`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Utf8 "variant datafusion::scalar::ScalarValue::Utf8") representing `val`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_utf8view" class="fn">new_utf8view</a>(val: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Returns a [`ScalarValue::Utf8View`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Utf8View "variant datafusion::scalar::ScalarValue::Utf8View") representing `val`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_interval_ym" class="fn">new_interval_ym</a>(years: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, months: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Returns a [`ScalarValue::IntervalYearMonth`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.IntervalYearMonth "variant datafusion::scalar::ScalarValue::IntervalYearMonth") representing `years` years and `months` months

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_interval_dt" class="fn">new_interval_dt</a>(days: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, millis: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Returns a [`ScalarValue::IntervalDayTime`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.IntervalDayTime "variant datafusion::scalar::ScalarValue::IntervalDayTime") representing `days` days and `millis` milliseconds

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_interval_mdn" class="fn">new_interval_mdn</a>(months: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, days: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, nanos: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Returns a [`ScalarValue::IntervalMonthDayNano`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.IntervalMonthDayNano "variant datafusion::scalar::ScalarValue::IntervalMonthDayNano") representing `months` months and `days` days, and `nanos` nanoseconds

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_timestamp" class="fn">new_timestamp</a>\<T\>( value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>, tz_opt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a>,

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing `value` and `tz_opt` timezone

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_pi" class="fn">new_pi</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing PI

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_pi_upper" class="fn">new_pi_upper</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing PI’s upper bound

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_negative_pi_lower" class="fn">new_negative_pi_lower</a>( datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing -PI’s lower bound

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_frac_pi_2_upper" class="fn">new_frac_pi_2_upper</a>( datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing FRAC_PI_2’s upper bound

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_neg_frac_pi_2_lower" class="fn">new_neg_frac_pi_2_lower</a>( datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_negative_pi" class="fn">new_negative_pi</a>( datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing -PI

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_frac_pi_2" class="fn">new_frac_pi_2</a>( datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing PI/2

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_neg_frac_pi_2" class="fn">new_neg_frac_pi_2</a>( datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing -PI/2

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_infinity" class="fn">new_infinity</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing infinity

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_neg_infinity" class="fn">new_neg_infinity</a>( datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") representing negative infinity

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_zero" class="fn">new_zero</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a zero value in the given type.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_default" class="fn">new_default</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a default value for the given `DataType`.

This function is useful when you need to initialize a column with non-null values in a DataFrame or when you need a “zero” value for a specific data type.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#default-values" class="doc-anchor">§</a>Default Values

- **Numeric types**: Returns zero (via [`new_zero`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#method.new_zero "associated function datafusion::scalar::ScalarValue::new_zero"))
- **String types**: Returns empty string (`""`)
- **Binary types**: Returns empty byte array
- **Temporal types**: Returns zero/epoch value
- **List types**: Returns empty list
- **Struct types**: Returns struct with all fields set to their defaults
- **Dictionary types**: Returns dictionary with default value
- **Map types**: Returns empty map
- **Union types**: Returns first variant with default value

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#errors" class="doc-anchor">§</a>Errors

Returns an error for data types that don’t have a clear default value or are not yet supported (e.g., `RunEndEncoded`).

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_one" class="fn">new_one</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create an one value in the given type.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_negative_one" class="fn">new_negative_one</a>( datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a negative one value in the given type.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_ten" class="fn">new_ten</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.data_type" class="fn">data_type</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

return the [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") of this `ScalarValue`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.arithmetic_negate" class="fn">arithmetic_negate</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Calculate arithmetic negation for a scalar value

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.add" class="fn">add</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>,

Wrapping addition of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code should operate on Arrays directly, using vectorized array kernels

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.add_checked" class="fn">add_checked</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>,

Checked addition of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code should operate on Arrays directly, using vectorized array kernels

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.sub" class="fn">sub</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>,

Wrapping subtraction of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code should operate on Arrays directly, using vectorized array kernels

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.sub_checked" class="fn">sub_checked</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>,

Checked subtraction of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code should operate on Arrays directly, using vectorized array kernels

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.mul" class="fn">mul</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>,

Wrapping multiplication of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code should operate on Arrays directly, using vectorized array kernels.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.mul_checked" class="fn">mul_checked</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>,

Checked multiplication of `ScalarValue`

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code should operate on Arrays directly, using vectorized array kernels.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.div" class="fn">div</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>,

Performs `lhs / rhs`

Overflow or division by zero will result in an error, with exception to floating point numbers, which instead follow the IEEE 754 rules.

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code should operate on Arrays directly, using vectorized array kernels.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.rem" class="fn">rem</a>\<T\>(&self, other: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>,

Performs `lhs % rhs`

Overflow or division by zero will result in an error, with exception to floating point numbers, which instead follow the IEEE 754 rules.

NB: operating on `ScalarValue` directly is not efficient, performance sensitive code should operate on Arrays directly, using vectorized array kernels.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.is_unsigned" class="fn">is_unsigned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.is_null" class="fn">is_null</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

whether this value is null or not.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.distance" class="fn">distance</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Absolute distance between two numeric values (of the same type). This method will return None if either one of the arguments are null. It might also return None if the resulting distance is greater than [`usize::MAX`](https://doc.rust-lang.org/nightly/std/primitive.usize.html#associatedconstant.MAX "associated constant usize::MAX"). If the type is a float, then the distance will be rounded to the nearest integer.

Note: the datatype itself must support subtraction.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.to_array" class="fn">to_array</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts a scalar value into an 1-row array.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#errors-1" class="doc-anchor">§</a>Errors

Errors if the ScalarValue cannot be converted into a 1-row array

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.to_scalar" class="fn">to_scalar</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts a scalar into an arrow [`Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html "struct datafusion::common::arrow::array::Scalar") (which implements the [`Datum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Datum.html "trait datafusion::common::arrow::array::Datum") interface).

This can be used to call arrow compute kernels such as `lt`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#errors-2" class="doc-anchor">§</a>Errors

Errors if the ScalarValue cannot be converted into a 1-row array

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#example-1" class="doc-anchor">§</a>Example

``` rust
use datafusion_common::ScalarValue;
use arrow::array::{BooleanArray, Int32Array};

let arr = Int32Array::from(vec![Some(1), None, Some(10)]);
let five = ScalarValue::Int32(Some(5));

let result = arrow::compute::kernels::cmp::lt(
  &arr,
  &five.to_scalar().unwrap(),
).unwrap();

let expected = BooleanArray::from(vec![
    Some(true),
    None,
    Some(false)
  ]
);

assert_eq!(&result, &expected);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.iter_to_array" class="fn">iter_to_array</a>( scalars: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts an iterator of references [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") into an [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") corresponding to those values. For example, an iterator of [`ScalarValue::Int32`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Int32 "variant datafusion::scalar::ScalarValue::Int32") would be converted to an [`Int32Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.Int32Array.html "type datafusion::common::arrow::array::Int32Array").

Returns an error if the iterator is empty or if the [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue")s are not all the same type

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#example-2" class="doc-anchor">§</a>Example

``` rust
use datafusion_common::ScalarValue;
use arrow::array::{ArrayRef, BooleanArray};

let scalars = vec![
  ScalarValue::Boolean(Some(true)),
  ScalarValue::Boolean(None),
  ScalarValue::Boolean(Some(false)),
];

// Build an Array from the list of ScalarValues
let array = ScalarValue::iter_to_array(scalars.into_iter())
  .unwrap();

let expected: ArrayRef = std::sync::Arc::new(
  BooleanArray::from(vec![
    Some(true),
    None,
    Some(false)
  ]
));

assert_eq!(&array, &expected);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_list" class="fn">new_list</a>( values: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\], data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>

Converts `Vec<ScalarValue>` where each element has type corresponding to `data_type`, to a single element [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray").

Example

``` rust
use datafusion_common::ScalarValue;
use arrow::array::{ListArray, Int32Array};
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::cast::as_list_array;

let scalars = vec![
   ScalarValue::Int32(Some(1)),
   ScalarValue::Int32(None),
   ScalarValue::Int32(Some(2))
];

let result = ScalarValue::new_list(&scalars, &DataType::Int32, true);

let expected = ListArray::from_iter_primitive::<Int32Type, _, _>(
    vec![
       Some(vec![Some(1), None, Some(2)])
    ]);

assert_eq!(*result, expected);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_list_nullable" class="fn">new_list_nullable</a>( values: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\], data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>

Same as [`ScalarValue::new_list`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#method.new_list "associated function datafusion::scalar::ScalarValue::new_list") but with nullable set to true.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_null_list" class="fn">new_null_list</a>( data_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, null_len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Create ListArray with Null with specific data type

- new_null_list(i32, nullable, 1): `ListArray[NULL]`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_list_from_iter" class="fn">new_list_from_iter</a>( values: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>, data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, nullable: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>

Converts `IntoIterator<Item = ScalarValue>` where each element has type corresponding to `data_type`, to a [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray").

Example

``` rust
use datafusion_common::ScalarValue;
use arrow::array::{ListArray, Int32Array};
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::cast::as_list_array;

let scalars = vec![
   ScalarValue::Int32(Some(1)),
   ScalarValue::Int32(None),
   ScalarValue::Int32(Some(2))
];

let result = ScalarValue::new_list_from_iter(scalars.into_iter(), &DataType::Int32, true);

let expected = ListArray::from_iter_primitive::<Int32Type, _, _>(
    vec![
       Some(vec![Some(1), None, Some(2)])
    ]);

assert_eq!(*result, expected);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.new_large_list" class="fn">new_large_list</a>( values: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\], data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>

Converts `Vec<ScalarValue>` where each element has type corresponding to `data_type`, to a [`LargeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListArray.html "type datafusion::common::arrow::array::LargeListArray").

Example

``` rust
use datafusion_common::ScalarValue;
use arrow::array::{LargeListArray, Int32Array};
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::cast::as_large_list_array;

let scalars = vec![
   ScalarValue::Int32(Some(1)),
   ScalarValue::Int32(None),
   ScalarValue::Int32(Some(2))
];

let result = ScalarValue::new_large_list(&scalars, &DataType::Int32);

let expected = LargeListArray::from_iter_primitive::<Int32Type, _, _>(
    vec![
       Some(vec![Some(1), None, Some(2)])
    ]);

assert_eq!(*result, expected);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.to_array_of_size" class="fn">to_array_of_size</a>( &self, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts a scalar value into an array of `size` rows.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#errors-3" class="doc-anchor">§</a>Errors

Errors if `self` is

- a decimal that fails be converted to a decimal array of size
- a `FixedsizeList` that fails to be concatenated into an array of size
- a `List` that fails to be concatenated into an array of size
- a `Dictionary` that fails be converted to a dictionary array of size

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.convert_array_to_scalar_vec" class="fn">convert_array_to_scalar_vec</a>( array: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Retrieve ScalarValue for each row in `array`

Example 1: Array (ScalarValue::Int32)

``` rust
use datafusion_common::ScalarValue;
use arrow::array::ListArray;
use arrow::datatypes::{DataType, Int32Type};

// Equivalent to [[1,2,3], [4,5]]
let list_arr = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![
   Some(vec![Some(1), Some(2), Some(3)]),
   Some(vec![Some(4), Some(5)])
]);

// Convert the array into Scalar Values for each row
let scalar_vec = ScalarValue::convert_array_to_scalar_vec(&list_arr).unwrap();

let expected = vec![
vec![
    ScalarValue::Int32(Some(1)),
    ScalarValue::Int32(Some(2)),
    ScalarValue::Int32(Some(3)),
],
vec![
   ScalarValue::Int32(Some(4)),
   ScalarValue::Int32(Some(5)),
],
];

assert_eq!(scalar_vec, expected);
```

Example 2: Nested array (ScalarValue::List)

``` rust
use datafusion_common::ScalarValue;
use arrow::array::ListArray;
use arrow::datatypes::{DataType, Int32Type};
use datafusion_common::utils::SingleRowListArrayBuilder;
use std::sync::Arc;

let list_arr = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![
   Some(vec![Some(1), Some(2), Some(3)]),
   Some(vec![Some(4), Some(5)])
]);

// Wrap into another layer of list, we got nested array as [ [[1,2,3], [4,5]] ]
let list_arr = SingleRowListArrayBuilder::new(Arc::new(list_arr)).build_list_array();

// Convert the array into Scalar Values for each row, we got 1D arrays in this example
let scalar_vec = ScalarValue::convert_array_to_scalar_vec(&list_arr).unwrap();

let l1 = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![
    Some(vec![Some(1), Some(2), Some(3)]),
]);
let l2 = ListArray::from_iter_primitive::<Int32Type, _, _>(vec![
    Some(vec![Some(4), Some(5)]),
]);

let expected = vec![
  vec![
    ScalarValue::List(Arc::new(l1)),
    ScalarValue::List(Arc::new(l2)),
  ],
];

assert_eq!(scalar_vec, expected);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.raw_data" class="fn">raw_data</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 46.0.0: This function is obsolete. Use `to_array` instead

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_from_array" class="fn">try_from_array</a>( array: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Converts a value in `array` at `index` into a ScalarValue

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_from_string" class="fn">try_from_string</a>( value: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, target_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to parse `value` into a ScalarValue of type `target_type`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_as_str" class="fn">try_as_str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>

Returns the Some(`&str`) representation of `ScalarValue` of logical string type

Returns `None` if this `ScalarValue` is not a logical string type or the `ScalarValue` represents the `NULL` value.

Note you can use [`Option::flatten`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#method.flatten "method core::option::Option::flatten") to check for non null logical strings.

For example, [`ScalarValue::Utf8`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Utf8 "variant datafusion::scalar::ScalarValue::Utf8"), [`ScalarValue::LargeUtf8`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.LargeUtf8 "variant datafusion::scalar::ScalarValue::LargeUtf8"), and [`ScalarValue::Dictionary`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#variant.Dictionary "variant datafusion::scalar::ScalarValue::Dictionary") with a logical string value and store strings and can be accessed as `&str` using this method.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#example-logical-strings" class="doc-anchor">§</a>Example: logical strings

``` rust
/// non strings return None
let scalar = ScalarValue::from(42);
assert_eq!(scalar.try_as_str(), None);
// Non null logical string returns Some(Some(&str))
let scalar = ScalarValue::from("hello");
assert_eq!(scalar.try_as_str(), Some(Some("hello")));
// Null logical string returns Some(None)
let scalar = ScalarValue::Utf8(None);
assert_eq!(scalar.try_as_str(), Some(None));
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#example-use-optionflatten-to-check-for-non-null-logical-strings" class="doc-anchor">§</a>Example: use [`Option::flatten`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#method.flatten "method core::option::Option::flatten") to check for non-null logical strings

``` rust
// Non null logical string returns Some(Some(&str))
let scalar = ScalarValue::from("hello");
assert_eq!(scalar.try_as_str().flatten(), Some("hello"));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.cast_to" class="fn">cast_to</a>( &self, target_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to cast this value to a ScalarValue of type `data_type`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.cast_to_with_options" class="fn">cast_to_with_options</a>( &self, target_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, cast_options: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.CastOptions.html" class="struct" title="struct datafusion::common::arrow::compute::CastOptions">CastOptions</a>\<'static\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to cast this value to a ScalarValue of type `data_type` with [`CastOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/struct.CastOptions.html "struct datafusion::common::arrow::compute::CastOptions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.eq_array" class="fn">eq_array</a>( &self, array: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Compares a single row of array @ index for equality with self, in an optimized fashion.

This method implements an optimized version of:

``` text
    let arr_scalar = Self::try_from_array(array, index).unwrap();
    arr_scalar.eq(self)
```

*Performance note*: the arrow compute kernels should be preferred over this function if at all possible as they can be vectorized and are generally much faster.

This function has a few narrow use cases such as hash table key comparisons where comparing a single row at a time is necessary.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#errors-4" class="doc-anchor">§</a>Errors

Errors if

- it fails to downcast `array` to the data type of `self`
- `self` is a `Struct`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#panics-1" class="doc-anchor">§</a>Panics

Panics if `self` is a dictionary with invalid key type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_cmp" class="fn">try_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Compare `self` with `other` and return an `Ordering`.

This is the same as [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd") except that it returns `Err` if the values cannot be compared, e.g., they have incompatible data types.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Estimate size if bytes including `Self`. For values with internal containers such as `String` includes the allocated size (`capacity`) rather than the current length (`len`)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.size_of_vec" class="fn">size_of_vec</a>(vec: &<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Estimates [size](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#method.size "method datafusion::scalar::ScalarValue::size") of [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") in bytes.

Includes the size of the [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") container itself.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.size_of_vec_deque" class="fn">size_of_vec_deque</a>(vec_deque: &<a href="https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html" class="struct" title="struct alloc::collections::vec_deque::VecDeque">VecDeque</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Estimates [size](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#method.size "method datafusion::scalar::ScalarValue::size") of [`VecDeque`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque") in bytes.

Includes the size of the [`VecDeque`](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque") container itself.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.size_of_hashset" class="fn">size_of_hashset</a>\<S\>(set: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, S\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Estimates [size](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#method.size "method datafusion::scalar::ScalarValue::size") of [`HashSet`](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet") in bytes.

Includes the size of the [`HashSet`](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet") container itself.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.compact" class="fn">compact</a>(&mut self)

Compacts the allocation referenced by `self` to the minimum, copying the data if necessary.

This can be relevant when `self` is a list or contains a list as a nested value, as a single list holds an Arc to its entire original array buffer.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.compacted" class="fn">compacted</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Compacts ([ScalarValue::compact](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html#method.compact "method datafusion::scalar::ScalarValue::compact")) the current [ScalarValue](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") and returns it.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.min" class="fn">min</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>

Returns the minimum value for the given numeric `DataType`.

This function returns the smallest representable value for numeric and temporal data types. For non-numeric types, it returns `None`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#supported-types" class="doc-anchor">§</a>Supported Types

- **Integer types**: `i8::MIN`, `i16::MIN`, etc.
- **Unsigned types**: Always 0 (`u8::MIN`, `u16::MIN`, etc.)
- **Float types**: Negative infinity (IEEE 754)
- **Decimal types**: Smallest value based on precision
- **Temporal types**: Minimum timestamp/date values
- **Time types**: 0 (midnight)
- **Duration types**: `i64::MIN`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.max" class="fn">max</a>(datatype: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>

Returns the maximum value for the given numeric `DataType`.

This function returns the largest representable value for numeric and temporal data types. For non-numeric types, it returns `None`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#supported-types-1" class="doc-anchor">§</a>Supported Types

- **Integer types**: `i8::MAX`, `i16::MAX`, etc.
- **Unsigned types**: `u8::MAX`, `u16::MAX`, etc.
- **Float types**: Positive infinity (IEEE 754)
- **Decimal types**: Largest value based on precision
- **Temporal types**: Maximum timestamp/date values
- **Time types**: Maximum time in the day (1 day - 1 unit)
- **Duration types**: `i64::MAX`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-Clone-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-Debug-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-Display-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3C%26HyperLogLog%3CT%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&HyperLogLog\<T\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-31" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: &HyperLogLog\<T\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3C%26ScalarValue%3E-for-Interval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-29" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3C%26str%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-22" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3C%26str%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-23" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3CString%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-26" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Cbool%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-13" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Cf32%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Cf64%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Ci16%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Ci32%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Ci64%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Ci8%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Cu16%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-17" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Cu32%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-19" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Cu64%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-21" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3COption%3Cu8%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-15" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3CScalarValue%3E-for-ColumnarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-27" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ColumnarValue.html" class="enum" title="enum datafusion::logical_expr::ColumnarValue">ColumnarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3CScalarValue%3E-for-Interval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-28" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/struct.Interval.html" class="struct" title="struct datafusion::logical_expr::interval_arithmetic::Interval">Interval</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3CScalarValue%3E-for-NullableInterval" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-30" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/interval_arithmetic/enum.NullableInterval.html" class="enum" title="enum datafusion::logical_expr::interval_arithmetic::NullableInterval">NullableInterval</a>

Create an interval that represents a single value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3CString%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-25" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3CVec%3C(%26str,+ScalarValue)%3E%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>)\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Wrapper to create ScalarValue::Struct for convenience

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-24" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>)\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Cbool%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Cf32%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.f32.html" class="primitive">f32</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Cf64%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Ci16%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Ci32%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Ci64%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Ci8%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Cu16%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-16" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Cu32%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-18" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Cu64%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-20" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-From%3Cu8%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from-14" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-FromPyArrow-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.FromPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::FromPyArrow">FromPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from_pyarrow_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.FromPyArrow.html#tymethod.from_pyarrow_bound" class="fn">from_pyarrow_bound</a>(value: &<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'\_, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert a Python object to an arrow-rs type. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.FromPyArrow.html#tymethod.from_pyarrow_bound)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-FromPyObject%3C&#39;source%3E-for-ScalarValue" class="anchor">§</a>

### impl\<'source\> <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.FromPyObject.html" class="trait" title="trait pyo3::conversion::FromPyObject">FromPyObject</a>\<'source\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.extract_bound" class="anchor">§</a>

#### fn <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.FromPyObject.html#tymethod.extract_bound" class="fn">extract_bound</a>(value: &<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'source, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Extracts `Self` from the bound smart pointer `obj`. [Read more](https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.FromPyObject.html#tymethod.extract_bound)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-FromStr-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html" class="enum" title="enum core::convert::Infallible">Infallible</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-Hash-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-IntoPyObject%3C&#39;source%3E-for-ScalarValue" class="anchor">§</a>

### impl\<'source\> <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html" class="trait" title="trait pyo3::conversion::IntoPyObject">IntoPyObject</a>\<'source\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html#associatedtype.Target" class="associatedtype">Target</a> = <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>

The Python output type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Bound.html" class="struct" title="struct pyo3::instance::Bound">Bound</a>\<'source, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a> as <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html" class="trait" title="trait pyo3::conversion::IntoPyObject">IntoPyObject</a>\<'source\>\>::<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html#associatedtype.Target" class="associatedtype" title="type pyo3::conversion::IntoPyObject::Target">Target</a>\>

The smart pointer type to use. [Read more](https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html#associatedtype.Output)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#associatedtype.Error-4" class="anchor">§</a>

#### type <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.into_pyobject" class="anchor">§</a>

#### fn <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html#tymethod.into_pyobject" class="fn">into_pyobject</a>( self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'source\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a> as <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html" class="trait" title="trait pyo3::conversion::IntoPyObject">IntoPyObject</a>\<'source\>\>::<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html#associatedtype.Output" class="associatedtype" title="type pyo3::conversion::IntoPyObject::Output">Output</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a> as <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html" class="trait" title="trait pyo3::conversion::IntoPyObject">IntoPyObject</a>\<'source\>\>::<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/conversion/trait.IntoPyObject.html#associatedtype.Error" class="associatedtype" title="type pyo3::conversion::IntoPyObject::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-Literal-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.lit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html#tymethod.lit" class="fn">lit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

convert the value to a Literal expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-PartialEq-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-PartialOrd-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-ToPyArrow-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.to_pyarrow" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#tymethod.to_pyarrow" class="fn">to_pyarrow</a>(&self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert the implemented type into a Python object without consuming it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-TryFrom%3C%26DataType%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(data_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a Null instance of ScalarValue for this datatype

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#associatedtype.Error-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-TryFrom%3CDataType%3E-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(datatype: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a Null instance of ScalarValue for this datatype

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#associatedtype.Error-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-TryFrom%3CScalarValue%3E-for-i256" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.i256.html" class="struct" title="struct datafusion::common::arrow::datatypes::i256">i256</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-TryFrom%3CScalarValue%3E-for-u32" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#method.try_from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#impl-Eq-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.ScalarValue.html#blanket-implementations" class="anchor">§</a>
