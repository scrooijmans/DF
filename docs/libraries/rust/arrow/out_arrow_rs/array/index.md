# Module array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/array/mod.rs.html#18-38" class="src">Source</a>

Expand description

Statically typed implementations of Arrow Arrays

**See [arrow_array](https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") for examples and usage instructions**

## Modules<a href="https://docs.rs/arrow/latest/arrow/array/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/index.html" class="mod" title="mod arrow::array::array">array</a>  
The concrete array definitions

<a href="https://docs.rs/arrow/latest/arrow/array/builder/index.html" class="mod" title="mod arrow::array::builder">builder</a>  
Defines push-based APIs for constructing arrays

<a href="https://docs.rs/arrow/latest/arrow/array/cast/index.html" class="mod" title="mod arrow::array::cast">cast</a>  
Defines helper functions for downcasting [`dyn Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") to concrete types

<a href="https://docs.rs/arrow/latest/arrow/array/ffi/index.html" class="mod" title="mod arrow::array::ffi">ffi</a>  
Contains declarations to bind to the [C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html).

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/index.html" class="mod" title="mod arrow::array::ffi_stream">ffi_stream</a>  
Contains declarations to bind to the [C Stream Interface](https://arrow.apache.org/docs/format/CStreamInterface.html).

<a href="https://docs.rs/arrow/latest/arrow/array/iterator/index.html" class="mod" title="mod arrow::array::iterator">iterator</a>  
Idiomatic iterators for [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

<a href="https://docs.rs/arrow/latest/arrow/array/run_iterator/index.html" class="mod" title="mod arrow::array::run_iterator">run_iterator</a>  
Idiomatic iterator for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray")

<a href="https://docs.rs/arrow/latest/arrow/array/temporal_conversions/index.html" class="mod" title="mod arrow::array::temporal_conversions">temporal_conversions</a>  
Conversion methods for dates and times.

<a href="https://docs.rs/arrow/latest/arrow/array/timezone/index.html" class="mod" title="mod arrow::array::timezone">timezone</a>  
Timezone for timestamp arrays

<a href="https://docs.rs/arrow/latest/arrow/array/types/index.html" class="mod" title="mod arrow::array::types">types</a>  
Zero-sized types used to parameterize generic array implementations

## Macros<a href="https://docs.rs/arrow/latest/arrow/array/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/macro.create_array.html" class="macro" title="macro arrow::array::create_array">create_array</a>  
Creates an array from a literal slice of values, suitable for rapid testing and development.

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_dictionary_array.html" class="macro" title="macro arrow::array::downcast_dictionary_array">downcast_dictionary_array</a>  
Downcast an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") to a [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") based on its [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType"), accepts a number of subsequent patterns to match the data type

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_integer.html" class="macro" title="macro arrow::array::downcast_integer">downcast_integer</a>  
Given one or more expressions evaluating to an integer [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") invokes the provided macro `m` with the corresponding integer [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType"), followed by any additional arguments

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_integer_array.html" class="macro" title="macro arrow::array::downcast_integer_array">downcast_integer_array</a>  
Given one or more expressions evaluating to an integer [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") invokes the provided macro with the corresponding array, along with match statements for any non integer array types

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_primitive.html" class="macro" title="macro arrow::array::downcast_primitive">downcast_primitive</a>  
Given one or more expressions evaluating to primitive [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") invokes the provided macro `m` with the corresponding [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType"), followed by any additional arguments

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_primitive_array.html" class="macro" title="macro arrow::array::downcast_primitive_array">downcast_primitive_array</a>  
Downcast an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") to a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") based on its [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") accepts a number of subsequent patterns to match the data type

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_run_array.html" class="macro" title="macro arrow::array::downcast_run_array">downcast_run_array</a>  
Downcast an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") to a [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") based on its [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType"), accepts a number of subsequent patterns to match the data type

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_run_end_index.html" class="macro" title="macro arrow::array::downcast_run_end_index">downcast_run_end_index</a>  
Given one or more expressions evaluating to an integer [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") invokes the provided macro `m` with the corresponding integer [`RunEndIndexType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html "trait arrow::datatypes::RunEndIndexType"), followed by any additional arguments

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_temporal.html" class="macro" title="macro arrow::array::downcast_temporal">downcast_temporal</a>  
Given one or more expressions evaluating to primitive [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") invokes the provided macro `m` with the corresponding [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType"), followed by any additional arguments

<a href="https://docs.rs/arrow/latest/arrow/array/macro.downcast_temporal_array.html" class="macro" title="macro arrow::array::downcast_temporal_array">downcast_temporal_array</a>  
Downcast an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") to a temporal [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") based on its [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") accepts a number of subsequent patterns to match the data type

<a href="https://docs.rs/arrow/latest/arrow/array/macro.record_batch.html" class="macro" title="macro arrow::array::record_batch">record_batch</a>  
Creates a record batch from literal slice of values, suitable for rapid testing and development.

## Structs<a href="https://docs.rs/arrow/latest/arrow/array/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>  
A generic representation of Arrow array data which encapsulates common attributes and operations for Arrow array.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayDataBuilder.html" class="struct" title="struct arrow::array::ArrayDataBuilder">ArrayDataBuilder</a>  
Builder for [`ArrayData`](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") type

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>  
An iterator that returns Some(T) or None, that can be used on any [`ArrayAccessor`](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html "trait arrow::array::ArrayAccessor")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>  
An array of [boolean values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBufferBuilder.html" class="struct" title="struct arrow::array::BooleanBufferBuilder">BooleanBufferBuilder</a>  
Builder for [`BooleanBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.BooleanBuffer.html "struct arrow::buffer::BooleanBuffer")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBuilder.html" class="struct" title="struct arrow::array::BooleanBuilder">BooleanBuilder</a>  
Builder for [`BooleanArray`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>  
Builder for creating a [Buffer](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") object.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>  
Helper to access views of [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html) (`StringViewArray` and `BinaryViewArray`) where the length is greater than 12 bytes.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html" class="struct" title="struct arrow::array::DataTypeLayout">DataTypeLayout</a>  
Layout specification for a data type

<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>  
An array of [dictionary encoded values](https://arrow.apache.org/docs/format/Columnar.html#dictionary-encoded-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>  
An array of [fixed size binary arrays](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>  
Builder for [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>  
An array of \[fixed length lists\], similar to JSON arrays (e.g. `["A", "B"]`).

<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>  
Builder for [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>  
An array of [variable length byte arrays](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>  
Builder for [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteDictionaryBuilder.html" class="struct" title="struct arrow::array::GenericByteDictionaryBuilder">GenericByteDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>  
[Variable-size Binary View Layout](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-view-layout): An array of variable length bytes views.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>  
A builder for [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>  
An array of [variable length lists](https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout), similar to JSON arrays (e.g. `["A", "B", "C"]`). This struct specifically represents the [list layout](https://arrow.apache.org/docs/format/Columnar.html#list-layout). Refer to [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") for the [list-view layout](https://arrow.apache.org/docs/format/Columnar.html#listview-layout).

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>  
Builder for [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>  
An array of [variable length lists](https://arrow.apache.org/docs/format/Columnar.html#variable-size-list-layout), specifically in the [list-view layout](https://arrow.apache.org/docs/format/Columnar.html#listview-layout).

<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewBuilder.html" class="struct" title="struct arrow::array::GenericListViewBuilder">GenericListViewBuilder</a>  
Builder for [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>  
An array of key-value maps

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>  
Builder for [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapFieldNames.html" class="struct" title="struct arrow::array::MapFieldNames">MapFieldNames</a>  
The [`Field`](https://docs.rs/arrow/latest/arrow/datatypes/struct.Field.html "struct arrow::datatypes::Field") names for a [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.MutableArrayData.html" class="struct" title="struct arrow::array::MutableArrayData">MutableArrayData</a>  
Efficiently create an [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData") from one or more existing [ArrayData](https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html "struct arrow::array::ArrayData")s by copying chunks.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>  
An optional primitive value

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html" class="struct" title="struct arrow::array::NullArray">NullArray</a>  
An array of [null values](https://arrow.apache.org/docs/format/Columnar.html#null-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBufferBuilder.html" class="struct" title="struct arrow::array::NullBufferBuilder">NullBufferBuilder</a>  
Builder for creating [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBuilder.html" class="struct" title="struct arrow::array::NullBuilder">NullBuilder</a>  
Builder for [`NullArray`](https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html "struct arrow::array::NullArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.OffsetBufferBuilder.html" class="struct" title="struct arrow::array::OffsetBufferBuilder">OffsetBufferBuilder</a>  
Builder of [`OffsetBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.OffsetBuffer.html "struct arrow::buffer::OffsetBuffer")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>  
An array of primitive values, of type [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>  
Builder for [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>  
A two-dimensional batch of column-oriented data with a defined [schema](https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html "struct arrow::datatypes::Schema").

<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchIterator.html" class="struct" title="struct arrow::array::RecordBatchIterator">RecordBatchIterator</a>  
Generic implementation of [RecordBatchReader](https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html "trait arrow::array::RecordBatchReader") that wraps an iterator.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchOptions.html" class="struct" title="struct arrow::array::RecordBatchOptions">RecordBatchOptions</a>  
Options that control the behaviour used when creating a [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").

<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>  
An array of [run-end encoded values](https://arrow.apache.org/docs/format/Columnar.html#run-end-encoded-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html" class="struct" title="struct arrow::array::Scalar">Scalar</a>  
A wrapper around a single value [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array") that implements [`Datum`](https://docs.rs/arrow/latest/arrow/array/trait.Datum.html "trait arrow::array::Datum") and indicates [compute](https://docs.rs/arrow/latest/arrow/compute/index.html) kernels should treat this array as a scalar value (a single value).

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>  
An array of [structs](https://arrow.apache.org/docs/format/Columnar.html#struct-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html" class="struct" title="struct arrow::array::StructBuilder">StructBuilder</a>  
Builder for [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray")

<a href="https://docs.rs/arrow/latest/arrow/array/struct.TypedDictionaryArray.html" class="struct" title="struct arrow::array::TypedDictionaryArray">TypedDictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") typed on its child values array

<a href="https://docs.rs/arrow/latest/arrow/array/struct.TypedRunArray.html" class="struct" title="struct arrow::array::TypedRunArray">TypedRunArray</a>  
A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") typed typed on its child values array

<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>  
An array of [values of varying types](https://arrow.apache.org/docs/format/Columnar.html#union-layout)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>  
Builder for [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray")

## Enums<a href="https://docs.rs/arrow/latest/arrow/array/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/enum.BufferSpec.html" class="enum" title="enum arrow::array::BufferSpec">BufferSpec</a>  
Layout specification for a single data type buffer

<a href="https://docs.rs/arrow/latest/arrow/array/enum.Capacities.html" class="enum" title="enum arrow::array::Capacities">Capacities</a>  
Define capacities to pre-allocate for child data or data buffers.

## Traits<a href="https://docs.rs/arrow/latest/arrow/array/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::AnyDictionaryArray">AnyDictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") with the key type erased

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>  
An array in the [arrow columnar format](https://arrow.apache.org/docs/format/Columnar.html)

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>  
A generic trait for accessing the values of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>  
Trait for dealing with different array builders at runtime

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNativeTypeOp.html" class="trait" title="trait arrow::array::ArrowNativeTypeOp">ArrowNativeTypeOp</a>  
Trait for [`ArrowNativeType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html "trait arrow::datatypes::ArrowNativeType") that adds checked and unchecked arithmetic operations, and totally ordered comparison operations

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowNumericType.html" class="trait" title="trait arrow::array::ArrowNumericType">ArrowNumericType</a>  
A subtype of primitive type that represents numeric values.

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>  
Trait for [primitive values](https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout).

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html" class="trait" title="trait arrow::array::AsArray">AsArray</a>  
An extension trait for `dyn Array` that provides ergonomic downcasting

<a href="https://docs.rs/arrow/latest/arrow/array/trait.BinaryArrayType.html" class="trait" title="trait arrow::array::BinaryArrayType">BinaryArrayType</a>  
A trait for Arrow String Arrays, currently three types are supported:

<a href="https://docs.rs/arrow/latest/arrow/array/trait.Datum.html" class="trait" title="trait arrow::array::Datum">Datum</a>  
A possibly [`Scalar`](https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html "struct arrow::array::Scalar") [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

<a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>  
A type that can be used within a variable-size array to encode offset information

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a>  
Trait for types that can read `RecordBatch`’s.

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchWriter.html" class="trait" title="trait arrow::array::RecordBatchWriter">RecordBatchWriter</a>  
Trait for types that can write `RecordBatch`’s.

<a href="https://docs.rs/arrow/latest/arrow/array/trait.StringArrayType.html" class="trait" title="trait arrow::array::StringArrayType">StringArrayType</a>  
A trait for Arrow String Arrays, currently three types are supported:

## Functions<a href="https://docs.rs/arrow/latest/arrow/array/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_boolean_array.html" class="fn" title="fn arrow::array::as_boolean_array">as_boolean_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`BooleanArray`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_dictionary_array.html" class="fn" title="fn arrow::array::as_dictionary_array">as_dictionary_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`DictionaryArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray"), panic’ing on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_fixed_size_list_array.html" class="fn" title="fn arrow::array::as_fixed_size_list_array">as_fixed_size_list_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_generic_binary_array.html" class="fn" title="fn arrow::array::as_generic_binary_array">as_generic_binary_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`GenericBinaryArray<S>`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_generic_list_array.html" class="fn" title="fn arrow::array::as_generic_list_array">as_generic_list_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`GenericListArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_large_list_array.html" class="fn" title="fn arrow::array::as_large_list_array">as_large_list_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`LargeListArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html "type arrow::array::LargeListArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_largestring_array.html" class="fn" title="fn arrow::array::as_largestring_array">as_largestring_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_list_array.html" class="fn" title="fn arrow::array::as_list_array">as_list_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_map_array.html" class="fn" title="fn arrow::array::as_map_array">as_map_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_null_array.html" class="fn" title="fn arrow::array::as_null_array">as_null_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`NullArray`](https://docs.rs/arrow/latest/arrow/array/struct.NullArray.html "struct arrow::array::NullArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_primitive_array.html" class="fn" title="fn arrow::array::as_primitive_array">as_primitive_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef"), to [`PrimitiveArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray"), panic’ing on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_run_array.html" class="fn" title="fn arrow::array::as_run_array">as_run_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`RunArray<T>`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray"), panic’ing on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_string_array.html" class="fn" title="fn arrow::array::as_string_array">as_string_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_struct_array.html" class="fn" title="fn arrow::array::as_struct_array">as_struct_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.as_union_array.html" class="fn" title="fn arrow::array::as_union_array">as_union_array</a>  
Force downcast of an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array"), such as an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") to [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray"), panicking on failure.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.downcast_array.html" class="fn" title="fn arrow::array::downcast_array">downcast_array</a>  
Downcasts a `dyn Array` to a concrete type

<a href="https://docs.rs/arrow/latest/arrow/array/fn.export_array_into_raw.html" class="fn" title="fn arrow::array::export_array_into_raw">export_array_into_raw</a><sup>⚠</sup>Deprecated`ffi`  
Exports an array to raw pointers of the C Data Interface provided by the consumer.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.layout.html" class="fn" title="fn arrow::array::layout">layout</a>  
Return the expected [`DataTypeLayout`](https://docs.rs/arrow/latest/arrow/array/struct.DataTypeLayout.html "struct arrow::array::DataTypeLayout") Arrays of this data type are expected to have

<a href="https://docs.rs/arrow/latest/arrow/array/fn.make_array.html" class="fn" title="fn arrow::array::make_array">make_array</a>  
Constructs an array using the input `data`. Returns a reference-counted `Array` instance.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.make_builder.html" class="fn" title="fn arrow::array::make_builder">make_builder</a>  
Returns a builder with capacity for `capacity` elements of datatype `DataType`.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.make_comparator.html" class="fn" title="fn arrow::array::make_comparator">make_comparator</a>  
Returns a comparison function that compares two values at two different positions between the two arrays.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.make_view.html" class="fn" title="fn arrow::array::make_view">make_view</a>  
Create a view based on the given data, block id and offset.

<a href="https://docs.rs/arrow/latest/arrow/array/fn.new_empty_array.html" class="fn" title="fn arrow::array::new_empty_array">new_empty_array</a>  
Creates a new empty array

<a href="https://docs.rs/arrow/latest/arrow/array/fn.new_null_array.html" class="fn" title="fn arrow::array::new_null_array">new_null_array</a>  
Creates a new array of `data_type` of length `length` filled entirely of `NULL` values

## Type Aliases<a href="https://docs.rs/arrow/latest/arrow/array/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/type.ArrayDataRef.html" class="type" title="type arrow::array::ArrayDataRef">ArrayDataRef</a>  
A thread-safe, shared reference to the Arrow array data.

<a href="https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html" class="type" title="type arrow::array::ArrayRef">ArrayRef</a>  
A reference-counted reference to a generic `Array`

<a href="https://docs.rs/arrow/latest/arrow/array/type.BinaryArray.html" class="type" title="type arrow::array::BinaryArray">BinaryArray</a>  
A [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") of `[u8]` using `i32` offsets

<a href="https://docs.rs/arrow/latest/arrow/array/type.BinaryBuilder.html" class="type" title="type arrow::array::BinaryBuilder">BinaryBuilder</a>  
Builder for [`BinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryArray.html "type arrow::array::BinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.BinaryDictionaryBuilder.html" class="type" title="type arrow::array::BinaryDictionaryBuilder">BinaryDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`BinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryArray.html "type arrow::array::BinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.BinaryRunBuilder.html" class="type" title="type arrow::array::BinaryRunBuilder">BinaryRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`BinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryArray.html "type arrow::array::BinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.BinaryViewArray.html" class="type" title="type arrow::array::BinaryViewArray">BinaryViewArray</a>  
A [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") of `[u8]`

<a href="https://docs.rs/arrow/latest/arrow/array/type.BinaryViewBuilder.html" class="type" title="type arrow::array::BinaryViewBuilder">BinaryViewBuilder</a>  
Array builder for [`BinaryViewArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryViewArray.html "type arrow::array::BinaryViewArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.BooleanIter.html" class="type" title="type arrow::array::BooleanIter">BooleanIter</a>  
an iterator that returns Some(T) or None, that can be used on any BooleanArray

<a href="https://docs.rs/arrow/latest/arrow/array/type.Date32Array.html" class="type" title="type arrow::array::Date32Array">Date32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of days since UNIX epoch stored as `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Date32BufferBuilder.html" class="type" title="type arrow::array::Date32BufferBuilder">Date32BufferBuilder</a>  
Buffer builder for 32-bit date type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Date32Builder.html" class="type" title="type arrow::array::Date32Builder">Date32Builder</a>  
A 32-bit date array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Date64Array.html" class="type" title="type arrow::array::Date64Array">Date64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of milliseconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Date64BufferBuilder.html" class="type" title="type arrow::array::Date64BufferBuilder">Date64BufferBuilder</a>  
Buffer builder for 64-bit date type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Date64Builder.html" class="type" title="type arrow::array::Date64Builder">Date64Builder</a>  
A 64-bit date array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal32Array.html" class="type" title="type arrow::array::Decimal32Array">Decimal32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 32-bit fixed point decimals

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal32BufferBuilder.html" class="type" title="type arrow::array::Decimal32BufferBuilder">Decimal32BufferBuilder</a>  
Buffer builder for 32-bit decimal type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal32Builder.html" class="type" title="type arrow::array::Decimal32Builder">Decimal32Builder</a>  
A decimal 32 array builder

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal64Array.html" class="type" title="type arrow::array::Decimal64Array">Decimal64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 64-bit fixed point decimals

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal64BufferBuilder.html" class="type" title="type arrow::array::Decimal64BufferBuilder">Decimal64BufferBuilder</a>  
Buffer builder for 64-bit decimal type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal64Builder.html" class="type" title="type arrow::array::Decimal64Builder">Decimal64Builder</a>  
A decimal 64 array builder

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal128Array.html" class="type" title="type arrow::array::Decimal128Array">Decimal128Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 128-bit fixed point decimals

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal128BufferBuilder.html" class="type" title="type arrow::array::Decimal128BufferBuilder">Decimal128BufferBuilder</a>  
Buffer builder for 128-bit decimal type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal128Builder.html" class="type" title="type arrow::array::Decimal128Builder">Decimal128Builder</a>  
A decimal 128 array builder

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal256Array.html" class="type" title="type arrow::array::Decimal256Array">Decimal256Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of 256-bit fixed point decimals

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal256BufferBuilder.html" class="type" title="type arrow::array::Decimal256BufferBuilder">Decimal256BufferBuilder</a>  
Buffer builder for 256-bit decimal type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Decimal256Builder.html" class="type" title="type arrow::array::Decimal256Builder">Decimal256Builder</a>  
A decimal 256 array builder

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationMicrosecondArray.html" class="type" title="type arrow::array::DurationMicrosecondArray">DurationMicrosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of elapsed durations in microseconds

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationMicrosecondBufferBuilder.html" class="type" title="type arrow::array::DurationMicrosecondBufferBuilder">DurationMicrosecondBufferBuilder</a>  
Buffer builder for elaspsed time of microseconds unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationMicrosecondBuilder.html" class="type" title="type arrow::array::DurationMicrosecondBuilder">DurationMicrosecondBuilder</a>  
An elapsed time in microseconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationMillisecondArray.html" class="type" title="type arrow::array::DurationMillisecondArray">DurationMillisecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of elapsed durations in milliseconds

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationMillisecondBufferBuilder.html" class="type" title="type arrow::array::DurationMillisecondBufferBuilder">DurationMillisecondBufferBuilder</a>  
Buffer builder for elaspsed time of milliseconds unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationMillisecondBuilder.html" class="type" title="type arrow::array::DurationMillisecondBuilder">DurationMillisecondBuilder</a>  
An elapsed time in milliseconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationNanosecondArray.html" class="type" title="type arrow::array::DurationNanosecondArray">DurationNanosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of elapsed durations in nanoseconds

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationNanosecondBufferBuilder.html" class="type" title="type arrow::array::DurationNanosecondBufferBuilder">DurationNanosecondBufferBuilder</a>  
Buffer builder for elaspsed time of nanoseconds unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationNanosecondBuilder.html" class="type" title="type arrow::array::DurationNanosecondBuilder">DurationNanosecondBuilder</a>  
An elapsed time in nanoseconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationSecondArray.html" class="type" title="type arrow::array::DurationSecondArray">DurationSecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of elapsed durations in seconds

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationSecondBufferBuilder.html" class="type" title="type arrow::array::DurationSecondBufferBuilder">DurationSecondBufferBuilder</a>  
Buffer builder for elaspsed time of second unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.DurationSecondBuilder.html" class="type" title="type arrow::array::DurationSecondBuilder">DurationSecondBuilder</a>  
An elapsed time in seconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.DynComparator.html" class="type" title="type arrow::array::DynComparator">DynComparator</a>  
Compare the values at two arbitrary indices in two arrays.

<a href="https://docs.rs/arrow/latest/arrow/array/type.FixedSizeBinaryIter.html" class="type" title="type arrow::array::FixedSizeBinaryIter">FixedSizeBinaryIter</a>  
an iterator that returns Some(T) or None, that can be used on any FixedSizeBinaryArray

<a href="https://docs.rs/arrow/latest/arrow/array/type.FixedSizeListIter.html" class="type" title="type arrow::array::FixedSizeListIter">FixedSizeListIter</a>  
an iterator that returns Some(T) or None, that can be used on any FixedSizeListArray

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float16Array.html" class="type" title="type arrow::array::Float16Array">Float16Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `f16`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float16BufferBuilder.html" class="type" title="type arrow::array::Float16BufferBuilder">Float16BufferBuilder</a>  
Buffer builder for 16-bit floating point type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float16Builder.html" class="type" title="type arrow::array::Float16Builder">Float16Builder</a>  
A 16-bit floating point array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float32Array.html" class="type" title="type arrow::array::Float32Array">Float32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `f32`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float32BufferBuilder.html" class="type" title="type arrow::array::Float32BufferBuilder">Float32BufferBuilder</a>  
Buffer builder for 32-bit floating point type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float32Builder.html" class="type" title="type arrow::array::Float32Builder">Float32Builder</a>  
A 32-bit floating point array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float64Array.html" class="type" title="type arrow::array::Float64Array">Float64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `f64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float64BufferBuilder.html" class="type" title="type arrow::array::Float64BufferBuilder">Float64BufferBuilder</a>  
Buffer builder for 64-bit floating point type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Float64Builder.html" class="type" title="type arrow::array::Float64Builder">Float64Builder</a>  
A 64-bit floating point array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html" class="type" title="type arrow::array::GenericBinaryArray">GenericBinaryArray</a>  
A [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") for storing `[u8]`

<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryBuilder.html" class="type" title="type arrow::array::GenericBinaryBuilder">GenericBinaryBuilder</a>  
Array builder for [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryIter.html" class="type" title="type arrow::array::GenericBinaryIter">GenericBinaryIter</a>  
an iterator that returns Some(T) or None, that can be used on any BinaryArray

<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericListArrayIter.html" class="type" title="type arrow::array::GenericListArrayIter">GenericListArrayIter</a>  
an iterator that returns Some(T) or None, that can be used on any ListArray

<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericListViewArrayIter.html" class="type" title="type arrow::array::GenericListViewArrayIter">GenericListViewArrayIter</a>  
an iterator that returns Some(T) or None, that can be used on any ListArray

<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html" class="type" title="type arrow::array::GenericStringArray">GenericStringArray</a>  
A [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") for storing `str`

<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericStringBuilder.html" class="type" title="type arrow::array::GenericStringBuilder">GenericStringBuilder</a>  
Array builder for [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.GenericStringIter.html" class="type" title="type arrow::array::GenericStringIter">GenericStringIter</a>  
an iterator that returns Some(T) or None, that can be used on any Utf8Array

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int8Array.html" class="type" title="type arrow::array::Int8Array">Int8Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i8`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int8BufferBuilder.html" class="type" title="type arrow::array::Int8BufferBuilder">Int8BufferBuilder</a>  
Buffer builder for signed 8-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int8Builder.html" class="type" title="type arrow::array::Int8Builder">Int8Builder</a>  
A signed 8-bit integer array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int8DictionaryArray.html" class="type" title="type arrow::array::Int8DictionaryArray">Int8DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `i8`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int16Array.html" class="type" title="type arrow::array::Int16Array">Int16Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i16`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int16BufferBuilder.html" class="type" title="type arrow::array::Int16BufferBuilder">Int16BufferBuilder</a>  
Buffer builder for signed 16-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int16Builder.html" class="type" title="type arrow::array::Int16Builder">Int16Builder</a>  
A signed 16-bit integer array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int16DictionaryArray.html" class="type" title="type arrow::array::Int16DictionaryArray">Int16DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `i16`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int16RunArray.html" class="type" title="type arrow::array::Int16RunArray">Int16RunArray</a>  
A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") with `i16` run ends

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int32Array.html" class="type" title="type arrow::array::Int32Array">Int32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int32BufferBuilder.html" class="type" title="type arrow::array::Int32BufferBuilder">Int32BufferBuilder</a>  
Buffer builder for signed 32-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int32Builder.html" class="type" title="type arrow::array::Int32Builder">Int32Builder</a>  
A signed 32-bit integer array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int32DictionaryArray.html" class="type" title="type arrow::array::Int32DictionaryArray">Int32DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int32RunArray.html" class="type" title="type arrow::array::Int32RunArray">Int32RunArray</a>  
A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") with `i32` run ends

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int64Array.html" class="type" title="type arrow::array::Int64Array">Int64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int64BufferBuilder.html" class="type" title="type arrow::array::Int64BufferBuilder">Int64BufferBuilder</a>  
Buffer builder for signed 64-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int64Builder.html" class="type" title="type arrow::array::Int64Builder">Int64Builder</a>  
A signed 64-bit integer array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int64DictionaryArray.html" class="type" title="type arrow::array::Int64DictionaryArray">Int64DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Int64RunArray.html" class="type" title="type arrow::array::Int64RunArray">Int64RunArray</a>  
A [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") with `i64` run ends

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalDayTimeArray.html" class="type" title="type arrow::array::IntervalDayTimeArray">IntervalDayTimeArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of “calendar” intervals in days and milliseconds

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalDayTimeBufferBuilder.html" class="type" title="type arrow::array::IntervalDayTimeBufferBuilder">IntervalDayTimeBufferBuilder</a>  
Buffer builder for “calendar” interval in days and milliseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalDayTimeBuilder.html" class="type" title="type arrow::array::IntervalDayTimeBuilder">IntervalDayTimeBuilder</a>  
A “calendar” interval in days and milliseconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalMonthDayNanoArray.html" class="type" title="type arrow::array::IntervalMonthDayNanoArray">IntervalMonthDayNanoArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of “calendar” intervals in months, days, and nanoseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalMonthDayNanoBufferBuilder.html" class="type" title="type arrow::array::IntervalMonthDayNanoBufferBuilder">IntervalMonthDayNanoBufferBuilder</a>  
Buffer builder “calendar” interval in months, days, and nanoseconds.

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalMonthDayNanoBuilder.html" class="type" title="type arrow::array::IntervalMonthDayNanoBuilder">IntervalMonthDayNanoBuilder</a>  
A “calendar” interval in months, days, and nanoseconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalYearMonthArray.html" class="type" title="type arrow::array::IntervalYearMonthArray">IntervalYearMonthArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of “calendar” intervals in whole months

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalYearMonthBufferBuilder.html" class="type" title="type arrow::array::IntervalYearMonthBufferBuilder">IntervalYearMonthBufferBuilder</a>  
Buffer builder for “calendar” interval in months.

<a href="https://docs.rs/arrow/latest/arrow/array/type.IntervalYearMonthBuilder.html" class="type" title="type arrow::array::IntervalYearMonthBuilder">IntervalYearMonthBuilder</a>  
A “calendar” interval in months array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryArray.html" class="type" title="type arrow::array::LargeBinaryArray">LargeBinaryArray</a>  
A [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") of `[u8]` using `i64` offsets

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryBuilder.html" class="type" title="type arrow::array::LargeBinaryBuilder">LargeBinaryBuilder</a>  
Builder for [`LargeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryArray.html "type arrow::array::LargeBinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryDictionaryBuilder.html" class="type" title="type arrow::array::LargeBinaryDictionaryBuilder">LargeBinaryDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`LargeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryArray.html "type arrow::array::LargeBinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryRunBuilder.html" class="type" title="type arrow::array::LargeBinaryRunBuilder">LargeBinaryRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`LargeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeBinaryArray.html "type arrow::array::LargeBinaryArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html" class="type" title="type arrow::array::LargeListArray">LargeListArray</a>  
A [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") of variable size lists, storing offsets as `i64`.

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeListBuilder.html" class="type" title="type arrow::array::LargeListBuilder">LargeListBuilder</a>  
Builder for [`LargeListArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeListArray.html "type arrow::array::LargeListArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeListViewArray.html" class="type" title="type arrow::array::LargeListViewArray">LargeListViewArray</a>  
A [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") of variable size lists, storing offsets as `i64`.

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeListViewBuilder.html" class="type" title="type arrow::array::LargeListViewBuilder">LargeListViewBuilder</a>  
Builder for [`LargeListViewArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeListViewArray.html "type arrow::array::LargeListViewArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html" class="type" title="type arrow::array::LargeStringArray">LargeStringArray</a>  
A [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") of `str` using `i64` offsets

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeStringBuilder.html" class="type" title="type arrow::array::LargeStringBuilder">LargeStringBuilder</a>  
Builder for [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeStringDictionaryBuilder.html" class="type" title="type arrow::array::LargeStringDictionaryBuilder">LargeStringDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.LargeStringRunBuilder.html" class="type" title="type arrow::array::LargeStringRunBuilder">LargeStringRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`LargeStringArray`](https://docs.rs/arrow/latest/arrow/array/type.LargeStringArray.html "type arrow::array::LargeStringArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.ListArray.html" class="type" title="type arrow::array::ListArray">ListArray</a>  
A [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") of variable size lists, storing offsets as `i32`.

<a href="https://docs.rs/arrow/latest/arrow/array/type.ListBuilder.html" class="type" title="type arrow::array::ListBuilder">ListBuilder</a>  
Builder for [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html "type arrow::array::ListArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.ListViewArray.html" class="type" title="type arrow::array::ListViewArray">ListViewArray</a>  
A [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") of variable size lists, storing offsets as `i32`.

<a href="https://docs.rs/arrow/latest/arrow/array/type.ListViewBuilder.html" class="type" title="type arrow::array::ListViewBuilder">ListViewBuilder</a>  
Builder for [`ListViewArray`](https://docs.rs/arrow/latest/arrow/array/type.ListViewArray.html "type arrow::array::ListViewArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.MapArrayIter.html" class="type" title="type arrow::array::MapArrayIter">MapArrayIter</a>  
an iterator that returns Some(T) or None, that can be used on any MapArray

<a href="https://docs.rs/arrow/latest/arrow/array/type.PrimitiveIter.html" class="type" title="type arrow::array::PrimitiveIter">PrimitiveIter</a>  
an iterator that returns Some(T) or None, that can be used on any PrimitiveArray

<a href="https://docs.rs/arrow/latest/arrow/array/type.StringArray.html" class="type" title="type arrow::array::StringArray">StringArray</a>  
A [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") of `str` using `i32` offsets

<a href="https://docs.rs/arrow/latest/arrow/array/type.StringBuilder.html" class="type" title="type arrow::array::StringBuilder">StringBuilder</a>  
Builder for [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.StringDictionaryBuilder.html" class="type" title="type arrow::array::StringDictionaryBuilder">StringDictionaryBuilder</a>  
Builder for [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") of [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.StringRunBuilder.html" class="type" title="type arrow::array::StringRunBuilder">StringRunBuilder</a>  
Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`StringArray`](https://docs.rs/arrow/latest/arrow/array/type.StringArray.html "type arrow::array::StringArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html" class="type" title="type arrow::array::StringViewArray">StringViewArray</a>  
A [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") that stores utf8 data

<a href="https://docs.rs/arrow/latest/arrow/array/type.StringViewBuilder.html" class="type" title="type arrow::array::StringViewBuilder">StringViewBuilder</a>  
Array builder for [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray")

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time32MillisecondArray.html" class="type" title="type arrow::array::Time32MillisecondArray">Time32MillisecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of milliseconds since midnight stored as `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time32MillisecondBufferBuilder.html" class="type" title="type arrow::array::Time32MillisecondBufferBuilder">Time32MillisecondBufferBuilder</a>  
Buffer builder for 32-bit elaspsed time since midnight of millisecond unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time32MillisecondBuilder.html" class="type" title="type arrow::array::Time32MillisecondBuilder">Time32MillisecondBuilder</a>  
A 32-bit elaspsed time in milliseconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time32SecondArray.html" class="type" title="type arrow::array::Time32SecondArray">Time32SecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of seconds since midnight stored as `i32`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time32SecondBufferBuilder.html" class="type" title="type arrow::array::Time32SecondBufferBuilder">Time32SecondBufferBuilder</a>  
Buffer builder for 32-bit elaspsed time since midnight of second unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time32SecondBuilder.html" class="type" title="type arrow::array::Time32SecondBuilder">Time32SecondBuilder</a>  
A 32-bit elaspsed time in seconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time64MicrosecondArray.html" class="type" title="type arrow::array::Time64MicrosecondArray">Time64MicrosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of microseconds since midnight stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time64MicrosecondBufferBuilder.html" class="type" title="type arrow::array::Time64MicrosecondBufferBuilder">Time64MicrosecondBufferBuilder</a>  
Buffer builder for 64-bit elaspsed time since midnight of microsecond unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time64MicrosecondBuilder.html" class="type" title="type arrow::array::Time64MicrosecondBuilder">Time64MicrosecondBuilder</a>  
A 64-bit elaspsed time in microseconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time64NanosecondArray.html" class="type" title="type arrow::array::Time64NanosecondArray">Time64NanosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of nanoseconds since midnight stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time64NanosecondBufferBuilder.html" class="type" title="type arrow::array::Time64NanosecondBufferBuilder">Time64NanosecondBufferBuilder</a>  
Buffer builder for 64-bit elaspsed time since midnight of nanosecond unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.Time64NanosecondBuilder.html" class="type" title="type arrow::array::Time64NanosecondBuilder">Time64NanosecondBuilder</a>  
A 64-bit elaspsed time in nanoseconds array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampMicrosecondArray.html" class="type" title="type arrow::array::TimestampMicrosecondArray">TimestampMicrosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of microseconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampMicrosecondBufferBuilder.html" class="type" title="type arrow::array::TimestampMicrosecondBufferBuilder">TimestampMicrosecondBufferBuilder</a>  
Buffer builder for timestamp type of microsecond unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampMicrosecondBuilder.html" class="type" title="type arrow::array::TimestampMicrosecondBuilder">TimestampMicrosecondBuilder</a>  
A timestamp microsecond array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampMillisecondArray.html" class="type" title="type arrow::array::TimestampMillisecondArray">TimestampMillisecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of milliseconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampMillisecondBufferBuilder.html" class="type" title="type arrow::array::TimestampMillisecondBufferBuilder">TimestampMillisecondBufferBuilder</a>  
Buffer builder for timestamp type of millisecond unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampMillisecondBuilder.html" class="type" title="type arrow::array::TimestampMillisecondBuilder">TimestampMillisecondBuilder</a>  
A timestamp millisecond array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampNanosecondArray.html" class="type" title="type arrow::array::TimestampNanosecondArray">TimestampNanosecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of nanoseconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampNanosecondBufferBuilder.html" class="type" title="type arrow::array::TimestampNanosecondBufferBuilder">TimestampNanosecondBufferBuilder</a>  
Buffer builder for timestamp type of nanosecond unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampNanosecondBuilder.html" class="type" title="type arrow::array::TimestampNanosecondBuilder">TimestampNanosecondBuilder</a>  
A timestamp nanosecond array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampSecondArray.html" class="type" title="type arrow::array::TimestampSecondArray">TimestampSecondArray</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of seconds since UNIX epoch stored as `i64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampSecondBufferBuilder.html" class="type" title="type arrow::array::TimestampSecondBufferBuilder">TimestampSecondBufferBuilder</a>  
Buffer builder for timestamp type of second unit.

<a href="https://docs.rs/arrow/latest/arrow/array/type.TimestampSecondBuilder.html" class="type" title="type arrow::array::TimestampSecondBuilder">TimestampSecondBuilder</a>  
A timestamp second array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt8Array.html" class="type" title="type arrow::array::UInt8Array">UInt8Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u8`

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt8BufferBuilder.html" class="type" title="type arrow::array::UInt8BufferBuilder">UInt8BufferBuilder</a>  
Buffer builder for usigned 8-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt8Builder.html" class="type" title="type arrow::array::UInt8Builder">UInt8Builder</a>  
An usigned 8-bit integer array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt8DictionaryArray.html" class="type" title="type arrow::array::UInt8DictionaryArray">UInt8DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u8`

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt16Array.html" class="type" title="type arrow::array::UInt16Array">UInt16Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u16`

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt16BufferBuilder.html" class="type" title="type arrow::array::UInt16BufferBuilder">UInt16BufferBuilder</a>  
Buffer builder for usigned 16-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt16Builder.html" class="type" title="type arrow::array::UInt16Builder">UInt16Builder</a>  
An usigned 16-bit integer array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt16DictionaryArray.html" class="type" title="type arrow::array::UInt16DictionaryArray">UInt16DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u16`

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt32Array.html" class="type" title="type arrow::array::UInt32Array">UInt32Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u32`

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt32BufferBuilder.html" class="type" title="type arrow::array::UInt32BufferBuilder">UInt32BufferBuilder</a>  
Buffer builder for usigned 32-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt32Builder.html" class="type" title="type arrow::array::UInt32Builder">UInt32Builder</a>  
An usigned 32-bit integer array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt32DictionaryArray.html" class="type" title="type arrow::array::UInt32DictionaryArray">UInt32DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u32`

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt64Array.html" class="type" title="type arrow::array::UInt64Array">UInt64Array</a>  
A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of `u64`

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt64BufferBuilder.html" class="type" title="type arrow::array::UInt64BufferBuilder">UInt64BufferBuilder</a>  
Buffer builder for usigned 64-bit integer type.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt64Builder.html" class="type" title="type arrow::array::UInt64Builder">UInt64Builder</a>  
An usigned 64-bit integer array builder.

<a href="https://docs.rs/arrow/latest/arrow/array/type.UInt64DictionaryArray.html" class="type" title="type arrow::array::UInt64DictionaryArray">UInt64DictionaryArray</a>  
A [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") indexed by `u64`
