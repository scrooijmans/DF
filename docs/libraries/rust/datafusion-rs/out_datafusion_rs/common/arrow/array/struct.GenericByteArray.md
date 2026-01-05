# Struct GenericByteArray Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/byte_array.rs.html#87" class="src">Source</a>

``` rust
pub struct GenericByteArray<T>where
    T: ByteArrayType,{ /* private fields */ }
```

Expand description

An array of [variable length byte arrays](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)

See [`StringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringArray.html "type datafusion::common::arrow::array::StringArray") and [`LargeStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeStringArray.html "type datafusion::common::arrow::array::LargeStringArray") for storing utf8 encoded string data

See [`BinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.BinaryArray.html "type datafusion::common::arrow::array::BinaryArray") and [`LargeBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeBinaryArray.html "type datafusion::common::arrow::array::LargeBinaryArray") for storing arbitrary bytes

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#example-from-a-vec" class="doc-anchor">§</a>Example: From a Vec

``` rust
let arr: GenericByteArray<Utf8Type> = vec!["hello", "world", ""].into();
assert_eq!(arr.value_data(), b"helloworld");
assert_eq!(arr.value_offsets(), &[0, 5, 10, 10]);
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("hello"), Some("world"), Some("")]);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#example-from-an-optional-vec" class="doc-anchor">§</a>Example: From an optional Vec

``` rust
let arr: GenericByteArray<Utf8Type> = vec![Some("hello"), Some("world"), Some(""), None].into();
assert_eq!(arr.value_data(), b"helloworld");
assert_eq!(arr.value_offsets(), &[0, 5, 10, 10, 10]);
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("hello"), Some("world"), Some(""), None]);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#example-from-an-iterator-of-option" class="doc-anchor">§</a>Example: From an iterator of option

``` rust
let arr: GenericByteArray<Utf8Type> = (0..5).map(|x| (x % 2 == 0).then(|| x.to_string())).collect();
let values: Vec<_> = arr.iter().collect();
assert_eq!(values, &[Some("0"), None, Some("2"), None, Some("4")]);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#example-using-builder" class="doc-anchor">§</a>Example: Using Builder

``` rust
let mut builder = GenericByteBuilder::<Utf8Type>::new();
builder.append_value("hello");
builder.append_null();
builder.append_value("world");
let array = builder.finish();
let values: Vec<_> = array.iter().collect();
assert_eq!(values, &[Some("hello"), None, Some("world")]);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-GenericByteArray%3CGenericBinaryType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from_vec" class="fn">from_vec</a>( v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

Creates a [GenericBinaryArray](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericBinaryArray.html "type datafusion::common::arrow::array::GenericBinaryArray") from a vector of byte slices

See also [`Self::from_iter_values`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from_iter_values "associated function datafusion::common::arrow::array::GenericByteArray::from_iter_values")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from_opt_vec" class="fn">from_opt_vec</a>( v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

Creates a [GenericBinaryArray](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericBinaryArray.html "type datafusion::common::arrow::array::GenericBinaryArray") from a vector of Optional (null) byte slices

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.take_iter" class="fn">take_iter</a>\<'a\>( &'a self, indexes: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> + 'a, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.take_iter_unchecked" class="fn">take_iter_unchecked</a>\<'a\>( &'a self, indexes: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> + 'a, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#safety" class="doc-anchor">§</a>Safety

caller must ensure that the indexes in the iterator are less than the `array.len()`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#associatedconstant.DATA_TYPE" class="constant">DATA_TYPE</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a> = T::DATA_TYPE

Data type of the array.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.new" class="fn">new</a>( offsets: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>\>, values: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Create a new [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray") from the provided parts, panicking on failure

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`GenericByteArray::try_new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.try_new "associated function datafusion::common::arrow::array::GenericByteArray::try_new") returns an error

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.try_new" class="fn">try_new</a>( offsets: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>\>, values: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Create a new [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray") from the provided parts, returning an error on failure

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#errors" class="doc-anchor">§</a>Errors

- `offsets.len() - 1 != nulls.len()`
- Any consecutive pair of `offsets` does not denote a valid slice of `values`

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.new_unchecked" class="fn">new_unchecked</a>( offsets: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>\>, values: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Create a new [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray") from the provided parts, without validation

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#safety-1" class="doc-anchor">§</a>Safety

Safe if [`Self::try_new`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.try_new "associated function datafusion::common::arrow::array::GenericByteArray::try_new") would not error

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.new_null" class="fn">new_null</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Create a new [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray") of length `len` where all values are null

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.new_scalar" class="fn">new_scalar</a>( value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html" class="struct" title="struct datafusion::common::arrow::array::Scalar">Scalar</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\>

Create a new [`Scalar`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.Scalar.html "struct datafusion::common::arrow::array::Scalar") from `v`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from_iter_values" class="fn">from_iter_values</a>\<Ptr, I\>(iter: I) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where Ptr: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Ptr\>,

Creates a [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray") based on an iterator of values without nulls

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.into_parts" class="fn">into_parts</a>( self, ) -\> (<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value_length" class="fn">value_length</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>

Returns the length for value at index `i`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#panics-1" class="doc-anchor">§</a>Panics

Panics if index `i` is out of bounds.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.offsets" class="fn">offsets</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::OffsetBuffer">OffsetBuffer</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>\>

Returns a reference to the offsets of this array

Unlike [`Self::value_offsets`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value_offsets "method datafusion::common::arrow::array::GenericByteArray::value_offsets") this returns the [`OffsetBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.OffsetBuffer.html "struct datafusion::common::arrow::buffer::OffsetBuffer") allowing for zero-copy cloning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::buffer::Buffer">Buffer</a>

Returns the values of this array

Unlike [`Self::value_data`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value_data "method datafusion::common::arrow::array::GenericByteArray::value_data") this returns the [`Buffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.Buffer.html "struct datafusion::common::arrow::buffer::Buffer") allowing for zero-copy cloning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value_data" class="fn">value_data</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Returns the raw value data

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.is_ascii" class="fn">is_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if all data within this array is ASCII

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value_offsets" class="fn">value_offsets</a>(&self) -\> &\[\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>\]

Returns the offset values in the offsets buffer

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value_unchecked" class="fn">value_unchecked</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>

Returns the element at index `i`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#safety-2" class="doc-anchor">§</a>Safety

Caller is responsible for ensuring that the index is within the bounds of the array

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value" class="fn">value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>

Returns the element at index `i`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#panics-2" class="doc-anchor">§</a>Panics

Panics if index `i` is out of bounds.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;GenericByteArray&lt;T&gt;&gt;">ⓘ</a>

constructs a new iterator

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Returns a zero-copy slice of this array with the indicated offset and length.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.into_builder" class="fn">into_builder</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\>

Returns `GenericByteBuilder` of this byte array for mutating its values if the underlying offset and data buffers are not shared by others.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.num_chars" class="fn">num_chars</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of `Unicode Scalar Value` in the string at index `i`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#performance" class="doc-anchor">§</a>Performance

This function has `O(n)` time complexity where `n` is the string length. If you can make sure that all chars in the string are in the range `U+0x0000` ~ `U+0x007F`, please use the function [`value_length`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value_length) which has O(1) time complexity.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.take_iter-1" class="fn">take_iter</a>\<'a\>( &'a self, indexes: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> + 'a, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

#### pub unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.take_iter_unchecked-1" class="fn">take_iter_unchecked</a>\<'a\>( &'a self, indexes: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> + 'a, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#safety-3" class="doc-anchor">§</a>Safety

caller must ensure that the indexes in the iterator are less than the `array.len()`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.try_from_binary" class="fn">try_from_binary</a>( v: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Fallibly creates a [`GenericStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericStringArray.html "type datafusion::common::arrow::array::GenericStringArray") from a [`GenericBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericBinaryArray.html "type datafusion::common::arrow::array::GenericBinaryArray") returning an error if [`GenericBinaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.GenericBinaryArray.html "type datafusion::common::arrow::array::GenericBinaryArray") contains invalid UTF-8 data

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-Array-for-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html "struct datafusion::common::arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#tymethod.nulls "method datafusion::common::arrow::array::Array::nulls") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-ArrayAccessor-for-%26GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = &'a \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>

The Arrow type of the element being accessed.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value" class="fn">value</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked" class="fn">value_unchecked</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-BinaryArrayType%3C&#39;a%3E-for-%26GenericByteArray%3CGenericBinaryType%3CO%3E%3E" class="anchor">§</a>

### impl\<'a, O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.BinaryArrayType.html" class="trait" title="trait datafusion::common::arrow::array::BinaryArrayType">BinaryArrayType</a>\<'a\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>\>

where O: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.iter-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.BinaryArrayType.html#tymethod.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>\>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;&#39;a GenericByteArray&lt;GenericBinaryType&lt;O&gt;&gt;&gt;">ⓘ</a>

Constructs a new iterator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-Clone-for-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-Debug-for-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3C%26GenericByteArray%3CFROM%3E%3E-for-GenericByteViewArray%3CV%3E" class="anchor">§</a>

### impl\<FROM, V\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<FROM\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<V\>

where FROM: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>, \<FROM as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Offset" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Offset">Offset</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a> + <a href="https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/cast/trait.ToPrimitive.html" class="trait" title="trait num_traits::cast::ToPrimitive">ToPrimitive</a>, V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\<Native = \<FROM as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>\>,

Efficiently convert a [`GenericByteArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html "struct datafusion::common::arrow::array::GenericByteArray") to a [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray")

For example this method can convert a [`StringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringArray.html "type datafusion::common::arrow::array::StringArray") to a [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray").

If the offsets are all less than u32::MAX, the new [`GenericByteViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html "struct datafusion::common::arrow::array::GenericByteViewArray") is built without copying the underlying string data (views are created directly into the existing buffer)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(byte_array: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<FROM\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<V\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CArrayData%3E-for-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CGenericByteArray%3CGenericBinaryType%3COffsetSize%3E%3E%3E-for-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( v: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CGenericByteArray%3CGenericStringType%3COffsetSize%3E%3E%3E-for-GenericByteArray%3CGenericBinaryType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CGenericByteArray%3CT%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CGenericListArray%3COffsetSize%3E%3E-for-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( v: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CGenericListArray%3CT%3E%3E-for-GenericByteArray%3CGenericBinaryType%3CT%3E%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<T\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<T\>\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<T\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<T\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CVec%3C%26%5Bu8%5D%3E%3E-for-GenericByteArray%3CGenericBinaryType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CVec%3C%26str%3E%3E-for-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CVec%3COption%3C%26%5Bu8%5D%3E%3E%3E-for-GenericByteArray%3CGenericBinaryType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CVec%3COption%3C%26str%3E%3E%3E-for-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CVec%3COption%3CString%3E%3E%3E-for-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-From%3CVec%3CString%3E%3E-for-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-FromIterator%3C%26Option%3CPtr%3E%3E-for-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<'a, Ptr, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<&'a <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ptr\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>, Ptr: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>\> + 'a,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = &'a <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ptr\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-FromIterator%3COption%3CPtr%3E%3E-for-GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<Ptr, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ptr\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>, Ptr: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Ptr\>\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-IntoIterator-for-%26GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>\>

The type of the elements being iterated over.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-PartialEq-for-GenericByteArray%3CGenericBinaryType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.eq-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<OffsetSize\>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.ne-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-PartialEq-for-GenericByteArray%3CGenericStringType%3COffsetSize%3E%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<OffsetSize\>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#impl-StringArrayType%3C&#39;a%3E-for-%26GenericByteArray%3CGenericStringType%3CO%3E%3E" class="anchor">§</a>

### impl\<'a, O\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.StringArrayType.html" class="trait" title="trait datafusion::common::arrow::array::StringArrayType">StringArrayType</a>\<'a\> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>

where O: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.is_ascii-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.StringArrayType.html#tymethod.is_ascii" class="fn">is_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if all data within this string array is ASCII

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#method.iter-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.StringArrayType.html#tymethod.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayIter.html" class="struct" title="struct datafusion::common::arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;&#39;a GenericByteArray&lt;GenericStringType&lt;O&gt;&gt;&gt;">ⓘ</a>

Constructs a new iterator

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html#blanket-implementations" class="anchor">§</a>
