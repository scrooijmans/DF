# Module builder Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/lib.rs.html#249" class="src">Source</a>

Expand description

Defines push-based APIs for constructing arrays

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#basic-usage" class="doc-anchor">§</a>Basic Usage

Builders can be used to build simple, non-nested arrays

``` rust
let mut a = Int32Builder::new();
a.append_value(1);
a.append_null();
a.append_value(2);
let a = a.finish();

assert_eq!(a, PrimitiveArray::from(vec![Some(1), None, Some(2)]));
```

``` rust
let mut a = StringBuilder::new();
a.append_value("foo");
a.append_value("bar");
a.append_null();
let a = a.finish();

assert_eq!(a, StringArray::from_iter([Some("foo"), Some("bar"), None]));
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#nested-usage" class="doc-anchor">§</a>Nested Usage

Builders can also be used to build more complex nested arrays, such as lists

``` rust
let mut a = ListBuilder::new(Int32Builder::new());
// [1, 2]
a.values().append_value(1);
a.values().append_value(2);
a.append(true);
// null
a.append(false);
// []
a.append(true);
// [3, null]
a.values().append_value(3);
a.values().append_null();
a.append(true);

// [[1, 2], null, [], [3, null]]
let a = a.finish();

assert_eq!(a, ListArray::from_iter_primitive::<Int32Type, _, _>([
    Some(vec![Some(1), Some(2)]),
    None,
    Some(vec![]),
    Some(vec![Some(3), None])]
))
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#using-the-extend-trait-to-append-values-from-an-iterable" class="doc-anchor">§</a>Using the [`Extend`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend") trait to append values from an iterable:

``` rust

let mut builder = StringBuilder::new();
builder.extend(vec![Some("🍐"), Some("🍎"), None]);
assert_eq!(builder.finish().len(), 3);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#using-the-extend-trait-to-write-generic-functions" class="doc-anchor">§</a>Using the [`Extend`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend") trait to write generic functions:

``` rust

// For generic methods that fill a list of values for an [`ArrayBuilder`], use the [`Extend`] trait.
fn filter_and_fill<V, I: IntoIterator<Item = V>>(builder: &mut impl Extend<V>, values: I, filter: V)
where V: PartialEq
{
    builder.extend(values.into_iter().filter(|v| *v == filter));
}
let mut string_builder = StringBuilder::new();
filter_and_fill(
    &mut string_builder,
    vec![Some("🍐"), Some("🍎"), None],
    Some("🍎"),
);
assert_eq!(string_builder.finish().len(), 1);

let mut int_builder = Int32Builder::new();
filter_and_fill(
    &mut int_builder,
    vec![Some(11), Some(42), None],
    Some(42),
);
assert_eq!(int_builder.finish().len(), 1);

// For generic methods that fill lists-of-lists for an [`ArrayBuilder`], use the [`Extend`] trait.
fn filter_and_fill_if_contains<T, V, I: IntoIterator<Item = Option<V>>>(
    list_builder: &mut impl Extend<Option<V>>,
    values: I,
    filter: Option<T>,
) where
    T: PartialEq,
    for<'a> &'a V: IntoIterator<Item = &'a Option<T>>,
{
    list_builder.extend(values.into_iter().filter(|string: &Option<V>| {
        string
            .as_ref()
            .map(|str: &V| str.into_iter().any(|ch: &Option<T>| ch == &filter))
            .unwrap_or(false)
    }));
 }
let builder = StringBuilder::new();
let mut list_builder = ListBuilder::new(builder);
let pear_pear = vec![Some("🍐"),Some("🍐")];
let pear_app = vec![Some("🍐"),Some("🍎")];
filter_and_fill_if_contains(
    &mut list_builder,
    vec![Some(pear_pear), Some(pear_app), None],
    Some("🍎"),
);
assert_eq!(list_builder.finish().len(), 1);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#custom-builders" class="doc-anchor">§</a>Custom Builders

It is common to have a collection of statically defined Rust types that you want to convert to Arrow arrays.

An example of doing so is below

``` rust
/// A custom row representation
struct MyRow {
    i32: i32,
    optional_i32: Option<i32>,
    string: Option<String>,
    i32_list: Option<Vec<Option<i32>>>,
}

/// Converts `Vec<Row>` into `StructArray`
#[derive(Debug, Default)]
struct MyRowBuilder {
    i32: Int32Builder,
    string: StringBuilder,
    i32_list: ListBuilder<Int32Builder>,
}

impl MyRowBuilder {
    fn append(&mut self, row: &MyRow) {
        self.i32.append_value(row.i32);
        self.string.append_option(row.string.as_ref());
        self.i32_list.append_option(row.i32_list.as_ref().map(|x| x.iter().copied()));
    }

    /// Note: returns StructArray to allow nesting within another array if desired
    fn finish(&mut self) -> StructArray {
        let i32 = Arc::new(self.i32.finish()) as ArrayRef;
        let i32_field = Arc::new(Field::new("i32", DataType::Int32, false));

        let string = Arc::new(self.string.finish()) as ArrayRef;
        let string_field = Arc::new(Field::new("i32", DataType::Utf8, false));

        let i32_list = Arc::new(self.i32_list.finish()) as ArrayRef;
        let value_field = Arc::new(Field::new_list_field(DataType::Int32, true));
        let i32_list_field = Arc::new(Field::new("i32_list", DataType::List(value_field), true));

        StructArray::from(vec![
            (i32_field, i32),
            (string_field, string),
            (i32_list_field, i32_list),
        ])
    }
}

/// For building arrays in generic code, use Extend instead of the append_* methods
/// e.g. append_value, append_option, append_null
impl<'a> Extend<&'a MyRow> for MyRowBuilder {
    fn extend<T: IntoIterator<Item = &'a MyRow>>(&mut self, iter: T) {
        iter.into_iter().for_each(|row| self.append(row));
    }
}

/// Converts a slice of [`MyRow`] to a [`RecordBatch`]
fn rows_to_batch(rows: &[MyRow]) -> RecordBatch {
    let mut builder = MyRowBuilder::default();
    builder.extend(rows);
    RecordBatch::from(&builder.finish())
}
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#null--validity-masks" class="doc-anchor">§</a>Null / Validity Masks

The [`NullBufferBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBufferBuilder.html "struct datafusion::common::arrow::array::NullBufferBuilder") is optimized for creating the null mask for an array.

``` rust
let mut builder = NullBufferBuilder::new(8);
let mut builder = NullBufferBuilder::new(8);
builder.append_n_non_nulls(7);
builder.append_null();
let buffer = builder.finish().unwrap();
assert_eq!(buffer.len(), 8);
assert_eq!(buffer.iter().collect::<Vec<_>>(), vec![true, true, true, true, true, true, true, false]);
```

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.BooleanBufferBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::BooleanBufferBuilder">BooleanBufferBuilder</a>  
Builder for [`BooleanBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.BooleanBuffer.html "struct datafusion::common::arrow::buffer::BooleanBuffer")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.BooleanBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::BooleanBuilder">BooleanBuilder</a>  
Builder for [`BooleanArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html "struct datafusion::common::arrow::array::BooleanArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.BufferBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::BufferBuilder">BufferBuilder</a>  
Builder for creating a [Buffer](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html "struct datafusion::common::arrow::buffer::Buffer") object.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>  
Builder for [`FixedSizeBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeBinaryArray.html "struct datafusion::common::arrow::array::FixedSizeBinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.DictionaryArray.html "struct datafusion::common::arrow::array::DictionaryArray") of [`FixedSizeBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeBinaryArray.html "struct datafusion::common::arrow::array::FixedSizeBinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.FixedSizeListBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::FixedSizeListBuilder">FixedSizeListBuilder</a>  
Builder for [`FixedSizeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html "struct datafusion::common::arrow::array::FixedSizeListArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.GenericByteBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::GenericByteBuilder">GenericByteBuilder</a>  
Builder for [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.GenericByteDictionaryBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::GenericByteDictionaryBuilder">GenericByteDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.DictionaryArray.html "struct datafusion::common::arrow::array::DictionaryArray") of [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.GenericByteRunBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::GenericByteRunBuilder">GenericByteRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RunArray.html "struct datafusion::common::arrow::array::RunArray") of [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.GenericByteViewBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::GenericByteViewBuilder">GenericByteViewBuilder</a>  
A builder for [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.GenericListBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::GenericListBuilder">GenericListBuilder</a>  
Builder for [`GenericListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html "struct datafusion::common::arrow::array::GenericListArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.GenericListViewBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::GenericListViewBuilder">GenericListViewBuilder</a>  
Builder for [`GenericListViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListViewArray.html "struct datafusion::common::arrow::array::GenericListViewArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.MapBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::MapBuilder">MapBuilder</a>  
Builder for [`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.MapFieldNames.html" class="struct" title="struct datafusion::common::arrow::array::builder::MapFieldNames">MapFieldNames</a>  
The [`Field`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") names for a [`MapArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html "struct datafusion::common::arrow::array::MapArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBufferBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::NullBufferBuilder">NullBufferBuilder</a>  
Builder for creating [`NullBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html "struct datafusion::common::arrow::buffer::NullBuffer")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::NullBuilder">NullBuilder</a>  
Builder for [`NullArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html "struct datafusion::common::arrow::array::NullArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.OffsetBufferBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::OffsetBufferBuilder">OffsetBufferBuilder</a>  
Builder of [`OffsetBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html "struct datafusion::common::arrow::buffer::OffsetBuffer")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.PrimitiveBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::PrimitiveBuilder">PrimitiveBuilder</a>  
Builder for [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.DictionaryArray.html "struct datafusion::common::arrow::array::DictionaryArray") of [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.PrimitiveRunBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::PrimitiveRunBuilder">PrimitiveRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RunArray.html "struct datafusion::common::arrow::array::RunArray") of [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.StructBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::StructBuilder">StructBuilder</a>  
Builder for [`StructArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html "struct datafusion::common::arrow::array::StructArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.UnionBuilder.html" class="struct" title="struct datafusion::common::arrow::array::builder::UnionBuilder">UnionBuilder</a>  
Builder for [`UnionArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.UnionArray.html "struct datafusion::common::arrow::array::UnionArray")

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/trait.ArrayBuilder.html" class="trait" title="trait datafusion::common::arrow::array::builder::ArrayBuilder">ArrayBuilder</a>  
Trait for dealing with different array builders at runtime

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/fn.make_builder.html" class="fn" title="fn datafusion::common::arrow::array::builder::make_builder">make_builder</a>  
Returns a builder with capacity for `capacity` elements of datatype `DataType`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/fn.make_view.html" class="fn" title="fn datafusion::common::arrow::array::builder::make_view">make_view</a>  
Create a view based on the given data, block id and offset.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.BinaryBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::BinaryBuilder">BinaryBuilder</a>  
Builder for [`BinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryArray.html "type datafusion::common::arrow::array::BinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.BinaryDictionaryBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::BinaryDictionaryBuilder">BinaryDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.DictionaryArray.html "struct datafusion::common::arrow::array::DictionaryArray") of [`BinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryArray.html "type datafusion::common::arrow::array::BinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.BinaryRunBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::BinaryRunBuilder">BinaryRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RunArray.html "struct datafusion::common::arrow::array::RunArray") of [`BinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryArray.html "type datafusion::common::arrow::array::BinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.BinaryViewBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::BinaryViewBuilder">BinaryViewBuilder</a>  
Array builder for [`BinaryViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryViewArray.html "type datafusion::common::arrow::array::BinaryViewArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Date32BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Date32BufferBuilder">Date32BufferBuilder</a>  
Buffer builder for 32-bit date type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Date32Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Date32Builder">Date32Builder</a>  
A 32-bit date array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Date64BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Date64BufferBuilder">Date64BufferBuilder</a>  
Buffer builder for 64-bit date type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Date64Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Date64Builder">Date64Builder</a>  
A 64-bit date array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Decimal32BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Decimal32BufferBuilder">Decimal32BufferBuilder</a>  
Buffer builder for 32-bit decimal type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Decimal32Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Decimal32Builder">Decimal32Builder</a>  
A decimal 32 array builder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Decimal64BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Decimal64BufferBuilder">Decimal64BufferBuilder</a>  
Buffer builder for 64-bit decimal type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Decimal64Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Decimal64Builder">Decimal64Builder</a>  
A decimal 64 array builder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Decimal128BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Decimal128BufferBuilder">Decimal128BufferBuilder</a>  
Buffer builder for 128-bit decimal type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Decimal128Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Decimal128Builder">Decimal128Builder</a>  
A decimal 128 array builder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Decimal256BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Decimal256BufferBuilder">Decimal256BufferBuilder</a>  
Buffer builder for 256-bit decimal type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Decimal256Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Decimal256Builder">Decimal256Builder</a>  
A decimal 256 array builder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.DurationMicrosecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::DurationMicrosecondBufferBuilder">DurationMicrosecondBufferBuilder</a>  
Buffer builder for elaspsed time of microseconds unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.DurationMicrosecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::DurationMicrosecondBuilder">DurationMicrosecondBuilder</a>  
An elapsed time in microseconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.DurationMillisecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::DurationMillisecondBufferBuilder">DurationMillisecondBufferBuilder</a>  
Buffer builder for elaspsed time of milliseconds unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.DurationMillisecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::DurationMillisecondBuilder">DurationMillisecondBuilder</a>  
An elapsed time in milliseconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.DurationNanosecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::DurationNanosecondBufferBuilder">DurationNanosecondBufferBuilder</a>  
Buffer builder for elaspsed time of nanoseconds unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.DurationNanosecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::DurationNanosecondBuilder">DurationNanosecondBuilder</a>  
An elapsed time in nanoseconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.DurationSecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::DurationSecondBufferBuilder">DurationSecondBufferBuilder</a>  
Buffer builder for elaspsed time of second unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.DurationSecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::DurationSecondBuilder">DurationSecondBuilder</a>  
An elapsed time in seconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Float16BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Float16BufferBuilder">Float16BufferBuilder</a>  
Buffer builder for 16-bit floating point type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Float16Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Float16Builder">Float16Builder</a>  
A 16-bit floating point array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Float32BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Float32BufferBuilder">Float32BufferBuilder</a>  
Buffer builder for 32-bit floating point type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Float32Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Float32Builder">Float32Builder</a>  
A 32-bit floating point array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Float64BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Float64BufferBuilder">Float64BufferBuilder</a>  
Buffer builder for 64-bit floating point type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Float64Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Float64Builder">Float64Builder</a>  
A 64-bit floating point array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.GenericBinaryBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::GenericBinaryBuilder">GenericBinaryBuilder</a>  
Array builder for [`GenericBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericBinaryArray.html "type datafusion::common::arrow::array::GenericBinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.GenericStringBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::GenericStringBuilder">GenericStringBuilder</a>  
Array builder for [`GenericStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericStringArray.html "type datafusion::common::arrow::array::GenericStringArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Int8BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Int8BufferBuilder">Int8BufferBuilder</a>  
Buffer builder for signed 8-bit integer type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Int8Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Int8Builder">Int8Builder</a>  
A signed 8-bit integer array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Int16BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Int16BufferBuilder">Int16BufferBuilder</a>  
Buffer builder for signed 16-bit integer type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Int16Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Int16Builder">Int16Builder</a>  
A signed 16-bit integer array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Int32BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Int32BufferBuilder">Int32BufferBuilder</a>  
Buffer builder for signed 32-bit integer type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Int32Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Int32Builder">Int32Builder</a>  
A signed 32-bit integer array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Int64BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Int64BufferBuilder">Int64BufferBuilder</a>  
Buffer builder for signed 64-bit integer type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Int64Builder.html" class="type" title="type datafusion::common::arrow::array::builder::Int64Builder">Int64Builder</a>  
A signed 64-bit integer array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.IntervalDayTimeBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::IntervalDayTimeBufferBuilder">IntervalDayTimeBufferBuilder</a>  
Buffer builder for “calendar” interval in days and milliseconds.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.IntervalDayTimeBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::IntervalDayTimeBuilder">IntervalDayTimeBuilder</a>  
A “calendar” interval in days and milliseconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.IntervalMonthDayNanoBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::IntervalMonthDayNanoBufferBuilder">IntervalMonthDayNanoBufferBuilder</a>  
Buffer builder “calendar” interval in months, days, and nanoseconds.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.IntervalMonthDayNanoBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::IntervalMonthDayNanoBuilder">IntervalMonthDayNanoBuilder</a>  
A “calendar” interval in months, days, and nanoseconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.IntervalYearMonthBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::IntervalYearMonthBufferBuilder">IntervalYearMonthBufferBuilder</a>  
Buffer builder for “calendar” interval in months.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.IntervalYearMonthBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::IntervalYearMonthBuilder">IntervalYearMonthBuilder</a>  
A “calendar” interval in months array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.LargeBinaryBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::LargeBinaryBuilder">LargeBinaryBuilder</a>  
Builder for [`LargeBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeBinaryArray.html "type datafusion::common::arrow::array::LargeBinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.LargeBinaryDictionaryBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::LargeBinaryDictionaryBuilder">LargeBinaryDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.DictionaryArray.html "struct datafusion::common::arrow::array::DictionaryArray") of [`LargeBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeBinaryArray.html "type datafusion::common::arrow::array::LargeBinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.LargeBinaryRunBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::LargeBinaryRunBuilder">LargeBinaryRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RunArray.html "struct datafusion::common::arrow::array::RunArray") of [`LargeBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeBinaryArray.html "type datafusion::common::arrow::array::LargeBinaryArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.LargeListBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::LargeListBuilder">LargeListBuilder</a>  
Builder for [`LargeListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListArray.html "type datafusion::common::arrow::array::LargeListArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.LargeListViewBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::LargeListViewBuilder">LargeListViewBuilder</a>  
Builder for [`LargeListViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeListViewArray.html "type datafusion::common::arrow::array::LargeListViewArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.LargeStringBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::LargeStringBuilder">LargeStringBuilder</a>  
Builder for [`LargeStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeStringArray.html "type datafusion::common::arrow::array::LargeStringArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.LargeStringDictionaryBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::LargeStringDictionaryBuilder">LargeStringDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.DictionaryArray.html "struct datafusion::common::arrow::array::DictionaryArray") of [`LargeStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeStringArray.html "type datafusion::common::arrow::array::LargeStringArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.LargeStringRunBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::LargeStringRunBuilder">LargeStringRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RunArray.html "struct datafusion::common::arrow::array::RunArray") of [`LargeStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeStringArray.html "type datafusion::common::arrow::array::LargeStringArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.ListBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::ListBuilder">ListBuilder</a>  
Builder for [`ListArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListArray.html "type datafusion::common::arrow::array::ListArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.ListViewBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::ListViewBuilder">ListViewBuilder</a>  
Builder for [`ListViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ListViewArray.html "type datafusion::common::arrow::array::ListViewArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.StringBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::StringBuilder">StringBuilder</a>  
Builder for [`StringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringArray.html "type datafusion::common::arrow::array::StringArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.StringDictionaryBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::StringDictionaryBuilder">StringDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.DictionaryArray.html "struct datafusion::common::arrow::array::DictionaryArray") of [`StringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringArray.html "type datafusion::common::arrow::array::StringArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.StringRunBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::StringRunBuilder">StringRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RunArray.html "struct datafusion::common::arrow::array::RunArray") of [`StringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringArray.html "type datafusion::common::arrow::array::StringArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.StringViewBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::StringViewBuilder">StringViewBuilder</a>  
Array builder for [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Time32MillisecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Time32MillisecondBufferBuilder">Time32MillisecondBufferBuilder</a>  
Buffer builder for 32-bit elaspsed time since midnight of millisecond unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Time32MillisecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Time32MillisecondBuilder">Time32MillisecondBuilder</a>  
A 32-bit elaspsed time in milliseconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Time32SecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Time32SecondBufferBuilder">Time32SecondBufferBuilder</a>  
Buffer builder for 32-bit elaspsed time since midnight of second unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Time32SecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Time32SecondBuilder">Time32SecondBuilder</a>  
A 32-bit elaspsed time in seconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Time64MicrosecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Time64MicrosecondBufferBuilder">Time64MicrosecondBufferBuilder</a>  
Buffer builder for 64-bit elaspsed time since midnight of microsecond unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Time64MicrosecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Time64MicrosecondBuilder">Time64MicrosecondBuilder</a>  
A 64-bit elaspsed time in microseconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Time64NanosecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Time64NanosecondBufferBuilder">Time64NanosecondBufferBuilder</a>  
Buffer builder for 64-bit elaspsed time since midnight of nanosecond unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.Time64NanosecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::Time64NanosecondBuilder">Time64NanosecondBuilder</a>  
A 64-bit elaspsed time in nanoseconds array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.TimestampMicrosecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::TimestampMicrosecondBufferBuilder">TimestampMicrosecondBufferBuilder</a>  
Buffer builder for timestamp type of microsecond unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.TimestampMicrosecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::TimestampMicrosecondBuilder">TimestampMicrosecondBuilder</a>  
A timestamp microsecond array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.TimestampMillisecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::TimestampMillisecondBufferBuilder">TimestampMillisecondBufferBuilder</a>  
Buffer builder for timestamp type of millisecond unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.TimestampMillisecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::TimestampMillisecondBuilder">TimestampMillisecondBuilder</a>  
A timestamp millisecond array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.TimestampNanosecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::TimestampNanosecondBufferBuilder">TimestampNanosecondBufferBuilder</a>  
Buffer builder for timestamp type of nanosecond unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.TimestampNanosecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::TimestampNanosecondBuilder">TimestampNanosecondBuilder</a>  
A timestamp nanosecond array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.TimestampSecondBufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::TimestampSecondBufferBuilder">TimestampSecondBufferBuilder</a>  
Buffer builder for timestamp type of second unit.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.TimestampSecondBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::TimestampSecondBuilder">TimestampSecondBuilder</a>  
A timestamp second array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.UInt8BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::UInt8BufferBuilder">UInt8BufferBuilder</a>  
Buffer builder for usigned 8-bit integer type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.UInt8Builder.html" class="type" title="type datafusion::common::arrow::array::builder::UInt8Builder">UInt8Builder</a>  
An usigned 8-bit integer array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.UInt16BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::UInt16BufferBuilder">UInt16BufferBuilder</a>  
Buffer builder for usigned 16-bit integer type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.UInt16Builder.html" class="type" title="type datafusion::common::arrow::array::builder::UInt16Builder">UInt16Builder</a>  
An usigned 16-bit integer array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.UInt32BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::UInt32BufferBuilder">UInt32BufferBuilder</a>  
Buffer builder for usigned 32-bit integer type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.UInt32Builder.html" class="type" title="type datafusion::common::arrow::array::builder::UInt32Builder">UInt32Builder</a>  
An usigned 32-bit integer array builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.UInt64BufferBuilder.html" class="type" title="type datafusion::common::arrow::array::builder::UInt64BufferBuilder">UInt64BufferBuilder</a>  
Buffer builder for usigned 64-bit integer type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/type.UInt64Builder.html" class="type" title="type datafusion::common::arrow::array::builder::UInt64Builder">UInt64Builder</a>  
An usigned 64-bit integer array builder.
