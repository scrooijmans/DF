# Struct PrimitiveArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#566" class="src">Source</a>

``` rust
pub struct PrimitiveArray<T>where
    T: ArrowPrimitiveType,{ /* private fields */ }
```

Expand description

An array of primitive values, of type [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType")

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-from-a-vec" class="doc-anchor">§</a>Example: From a Vec

``` rust
let arr: PrimitiveArray<Int32Type> = vec![1, 2, 3, 4].into();
assert_eq!(4, arr.len());
assert_eq!(0, arr.null_count());
assert_eq!(arr.values(), &[1, 2, 3, 4])
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-from-an-optional-vec" class="doc-anchor">§</a>Example: From an optional Vec

``` rust
let arr: PrimitiveArray<Int32Type> = vec![Some(1), None, Some(3), None].into();
assert_eq!(4, arr.len());
assert_eq!(2, arr.null_count());
// Note: values for null indexes are arbitrary
assert_eq!(arr.values(), &[1, 0, 3, 0])
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-from-an-iterator-of-values" class="doc-anchor">§</a>Example: From an iterator of values

``` rust
let arr: PrimitiveArray<Int32Type> = (0..10).map(|x| x + 1).collect();
assert_eq!(10, arr.len());
assert_eq!(0, arr.null_count());
for i in 0..10i32 {
    assert_eq!(i + 1, arr.value(i as usize));
}
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-from-an-iterator-of-option" class="doc-anchor">§</a>Example: From an iterator of option

``` rust
let arr: PrimitiveArray<Int32Type> = (0..10).map(|x| (x % 2 == 0).then_some(x)).collect();
assert_eq!(10, arr.len());
assert_eq!(5, arr.null_count());
// Note: values for null indexes are arbitrary
assert_eq!(arr.values(), &[0, 0, 2, 0, 4, 0, 6, 0, 8, 0])
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-using-builder" class="doc-anchor">§</a>Example: Using Builder

``` rust
let mut builder = PrimitiveBuilder::<Int32Type>::new();
builder.append_value(1);
builder.append_null();
builder.append_value(2);
let array = builder.finish();
// Note: values for null indexes are arbitrary
assert_eq!(array.values(), &[1, 0, 2]);
assert!(array.is_null(1));
```

## <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-get-a-primitivearray-from-an-arrayref" class="doc-anchor">§</a>Example: Get a `PrimitiveArray` from an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef")

``` rust
// will panic if the array is not a Float32Array
assert_eq!(&DataType::Float32, array.data_type());
let f32_array: Float32Array  = array.as_primitive().clone();
assert_eq!(f32_array, Float32Array::from(vec![1.2, 2.3]));
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.new" class="fn">new</a>( values: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Create a new [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") from the provided values and nulls

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#panics" class="doc-anchor">§</a>Panics

Panics if [`Self::try_new`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.try_new "associated function arrow::array::PrimitiveArray::try_new") returns an error

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example" class="doc-anchor">§</a>Example

Creating a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") directly from a [`ScalarBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html "struct arrow::buffer::ScalarBuffer") and [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") using this constructor is the most performant approach, avoiding any additional allocations

``` rust
// [1, 2, 3, 4]
let array = Int32Array::new(vec![1, 2, 3, 4].into(), None);
// [1, null, 3, 4]
let nulls = NullBuffer::from(vec![true, false, true, true]);
let array = Int32Array::new(vec![1, 2, 3, 4].into(), Some(nulls));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.new_null" class="fn">new_null</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Create a new [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of the given length where all values are null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.try_new" class="fn">try_new</a>( values: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Create a new [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") from the provided values and nulls

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#errors" class="doc-anchor">§</a>Errors

Errors if:

- `values.len() != nulls.len()`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.new_scalar" class="fn">new_scalar</a>( value: \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html" class="struct" title="struct arrow::array::Scalar">Scalar</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\>

Create a new [`Scalar`](https://docs.rs/arrow/latest/arrow/array/struct.Scalar.html "struct arrow::array::Scalar") from `value`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.into_parts" class="fn">into_parts</a>( self, ) -\> (<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>)

Deconstruct this array into its constituent parts

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.with_data_type" class="fn">with_data_type</a>(self, data_type: <a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Overrides the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

Prefer using [`Self::with_timezone`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.with_timezone "method arrow::array::PrimitiveArray::with_timezone") or [`Self::with_precision_and_scale`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.with_precision_and_scale "method arrow::array::PrimitiveArray::with_precision_and_scale") where the primitive type is suitably constrained, as these cannot panic

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#panics-1" class="doc-anchor">§</a>Panics

Panics if \![Self::is_compatible\]

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of this array.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.values" class="fn">values</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

Returns the values of this array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.builder" class="fn">builder</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

Returns a new primitive array builder

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.is_compatible" class="fn">is_compatible</a>(data_type: &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns if this [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") is compatible with the provided [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType")

This is equivalent to `data_type == T::DATA_TYPE`, however ignores timestamp timezones and decimal precision and scale

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value_unchecked" class="fn">value_unchecked</a>( &self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns the primitive value at index `i`.

Note: This method does not check for nulls and the value is arbitrary if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.is_null "method arrow::array::PrimitiveArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#safety" class="doc-anchor">§</a>Safety

caller must ensure that the passed in offset is less than the array len()

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value" class="fn">value</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

Returns the primitive value at index `i`.

Note: This method does not check for nulls and the value is arbitrary if [`is_null`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.is_null "method arrow::array::PrimitiveArray::is_null") returns true for the index.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#panics-2" class="doc-anchor">§</a>Panics

Panics if index `i` is out of bounds

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from_iter_values" class="fn">from_iter_values</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>,

Creates a PrimitiveArray based on an iterator of values without nulls

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from_iter_values_with_nulls" class="fn">from_iter_values_with_nulls</a>\<I\>( iter: I, nulls: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>,

Creates a PrimitiveArray based on an iterator of values with provided nulls

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from_value" class="fn">from_value</a>( value: \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Creates a PrimitiveArray based on a constant value with `count` elements

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.take_iter" class="fn">take_iter</a>\<'a\>( &'a self, indexes: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> + 'a, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> + 'a

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.take_iter_unchecked" class="fn">take_iter_unchecked</a>\<'a\>( &'a self, indexes: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> + 'a, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> + 'a

Returns an iterator that returns the values of `array.value(i)` for an iterator with each element `i`

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#safety-1" class="doc-anchor">§</a>Safety

caller must ensure that the offsets in the iterator are less than the array len()

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Returns a zero-copy slice of this array with the indicated offset and length.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.reinterpret_cast" class="fn">reinterpret_cast</a>\<K\>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\<Native = \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>,

Reinterprets this array’s contents as a different data type without copying

This can be used to efficiently convert between primitive arrays with the same underlying representation

Note: this will not modify the underlying values, and therefore may change the semantic values of the array, e.g. 100 milliseconds in a [`TimestampNanosecondArray`](https://docs.rs/arrow/latest/arrow/array/type.TimestampNanosecondArray.html "type arrow::array::TimestampNanosecondArray") will become 100 seconds in a [`TimestampSecondArray`](https://docs.rs/arrow/latest/arrow/array/type.TimestampSecondArray.html "type arrow::array::TimestampSecondArray").

For casts that preserve the semantic value, check out the [compute kernels](https://docs.rs/arrow/latest/arrow/compute/kernels/cast/index.html).

``` rust
let a = Int64Array::from_iter_values([1, 2, 3, 4]);
let b: TimestampNanosecondArray = a.reinterpret_cast();
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.unary" class="fn">unary</a>\<F, O\>(&self, op: F) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>) -\> \<O as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>,

Applies a unary infallible function to a primitive array, producing a new array of potentially different type.

This is the fastest way to perform an operation on a primitive array when the benefits of a vectorized operation outweigh the cost of branching nulls and non-nulls.

See also

- [`Self::unary_mut`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary_mut "method arrow::array::PrimitiveArray::unary_mut") for in place modification.
- [`Self::try_unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.try_unary "method arrow::array::PrimitiveArray::try_unary") for fallible operations.
- [`arrow::compute::binary`](https://docs.rs/arrow/latest/arrow/compute/fn.binary.html) for binary operations

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#null-handling" class="doc-anchor">§</a>Null Handling

Applies the function for all values, including those on null slots. This will often allow the compiler to generate faster vectorized code, but requires that the operation must be infallible (not error/panic) for any value of the corresponding type or this function may panic.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-1" class="doc-anchor">§</a>Example

``` rust
let array = Int32Array::from(vec![Some(5), Some(7), None]);
// Create a new array with the value of applying sqrt
let c = array.unary(|x| f32::sqrt(x as f32));
assert_eq!(c, Float32Array::from(vec![Some(2.236068), Some(2.6457512), None]));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.unary_mut" class="fn">unary_mut</a>\<F\>(self, op: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>, <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>) -\> \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>,

Applies a unary and infallible function to the array in place if possible.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#buffer-reuse" class="doc-anchor">§</a>Buffer Reuse

If the underlying buffers are not shared with other arrays, mutates the underlying buffer in place, without allocating.

If the underlying buffer is shared, returns Err(self)

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#null-handling-1" class="doc-anchor">§</a>Null Handling

See [`Self::unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary "method arrow::array::PrimitiveArray::unary") for more information on null handling.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-2" class="doc-anchor">§</a>Example

``` rust
let array = Int32Array::from(vec![Some(5), Some(7), None]);
// Apply x*2+1 to the data in place, no allocations
let c = array.unary_mut(|x| x * 2 + 1).unwrap();
assert_eq!(c, Int32Array::from(vec![Some(11), Some(15), None]));
```

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-modify-arrayref-in-place-if-not-shared" class="doc-anchor">§</a>Example: modify [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") in place, if not shared

It is also possible to modify an [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") if there are no other references to the underlying buffer.

``` rust
// Convert to Int32Array (panic's if array.data_type is not Int32)
let a = array.as_primitive::<Int32Type>().clone();
// Try to apply x*2+1 to the data in place, fails because array is still shared
a.unary_mut(|x| x * 2 + 1).unwrap_err();
// Try again, this time dropping the last remaining reference
let a = array.as_primitive::<Int32Type>().clone();
drop(array);
// Now we can apply the operation in place
let c = a.unary_mut(|x| x * 2 + 1).unwrap();
assert_eq!(c, Int32Array::from(vec![Some(11), Some(15), None]));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.try_unary" class="fn">try_unary</a>\<F, O, E\>(&self, op: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<O\>, E\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<O as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, E\>,

Applies a unary fallible function to all valid values in a primitive array, producing a new array of potentially different type.

Applies `op` to only rows that are valid, which is often significantly slower than [`Self::unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary "method arrow::array::PrimitiveArray::unary"), which should be preferred if `op` is fallible.

Note: LLVM is currently unable to effectively vectorize fallible operations

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.try_unary_mut" class="fn">try_unary_mut</a>\<F, E\>( self, op: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>, E\>, <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>, E\>,

Applies a unary fallible function to all valid values in a mutable primitive array.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#null-handling-2" class="doc-anchor">§</a>Null Handling

See [`Self::try_unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.try_unary "method arrow::array::PrimitiveArray::try_unary") for more information on null handling.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#buffer-reuse-1" class="doc-anchor">§</a>Buffer Reuse

See [`Self::unary_mut`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary_mut "method arrow::array::PrimitiveArray::unary_mut") for more information on buffer reuse.

This returns an `Err` when the input array is shared buffer with other array. In the case, returned `Err` wraps input array. If the function encounters an error during applying on values. In the case, this returns an `Err` within an `Ok` which wraps the actual error.

Note: LLVM is currently unable to effectively vectorize fallible operations

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.unary_opt" class="fn">unary_opt</a>\<F, O\>(&self, op: F) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<O as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>,

Applies a unary and nullable function to all valid values in a primitive array

Applies `op` to only rows that are valid, which is often significantly slower than [`Self::unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary "method arrow::array::PrimitiveArray::unary"), which should be preferred if `op` is fallible.

Note: LLVM is currently unable to effectively vectorize fallible operations

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from_unary" class="fn">from_unary</a>\<U, F\>(left: U, op: F) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where U: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(\<U as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>) -\> \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>,

Applies a unary infallible function to each value in an array, producing a new primitive array.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#null-handling-3" class="doc-anchor">§</a>Null Handling

See [`Self::unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary "method arrow::array::PrimitiveArray::unary") for more information on null handling.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#example-create-an-int16array-from-an-arrayaccessor-with-item-type-u8" class="doc-anchor">§</a>Example: create an [`Int16Array`](https://docs.rs/arrow/latest/arrow/array/type.Int16Array.html "type arrow::array::Int16Array") from an [`ArrayAccessor`](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html "trait arrow::array::ArrayAccessor") with item type `&[u8]`

``` rust
use arrow_array::{Array, FixedSizeBinaryArray, Int16Array};
let input_arg = vec![ vec![1, 0], vec![2, 0], vec![3, 0] ];
let arr = FixedSizeBinaryArray::try_from_iter(input_arg.into_iter()).unwrap();
let c = Int16Array::from_unary(&arr, |x| i16::from_le_bytes(x[..2].try_into().unwrap()));
assert_eq!(c, Int16Array::from(vec![Some(1i16), Some(2i16), Some(3i16)]));
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.into_builder" class="fn">into_builder</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>, <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\>

Returns a `PrimitiveBuilder` for this array, suitable for mutating values in place.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#buffer-reuse-2" class="doc-anchor">§</a>Buffer Reuse

If the underlying data buffer has no other outstanding references, the buffer is used without copying.

If the underlying data buffer does have outstanding references, returns `Err(self)`

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-PrimitiveArray%3CT%3E-1" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTemporalType.html" class="trait" title="trait arrow::datatypes::ArrowTemporalType">ArrowTemporalType</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value_as_datetime" class="fn">value_as_datetime</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html" class="struct" title="struct chrono::naive::datetime::NaiveDateTime">NaiveDateTime</a>\>

Returns value as a chrono `NaiveDateTime`, handling time resolution

If a data type cannot be converted to `NaiveDateTime`, a `None` is returned. A valid value is expected, thus the user should first check for validity.

See notes on [`PrimitiveArray::value`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.value "method arrow::array::PrimitiveArray::value") regarding nulls and panics

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value_as_datetime_with_tz" class="fn">value_as_datetime_with_tz</a>( &self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, tz: <a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/timezone/struct.Tz.html" class="struct" title="struct arrow::array::timezone::Tz">Tz</a>\>\>

Returns value as a chrono `NaiveDateTime`, handling time resolution with the provided tz

functionally it is same as `value_as_datetime`, however it adds the passed tz to the to-be-returned NaiveDateTime

See notes on [`PrimitiveArray::value`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.value "method arrow::array::PrimitiveArray::value") regarding nulls and panics

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value_as_date" class="fn">value_as_date</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html" class="struct" title="struct chrono::naive::date::NaiveDate">NaiveDate</a>\>

Returns value as a chrono `NaiveDate` by using `Self::datetime()`

If a data type cannot be converted to `NaiveDate`, a `None` is returned

See notes on [`PrimitiveArray::value`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.value "method arrow::array::PrimitiveArray::value") regarding nulls and panics

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value_as_time" class="fn">value_as_time</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html" class="struct" title="struct chrono::naive::time::NaiveTime">NaiveTime</a>\>

Returns a value as a chrono `NaiveTime`

`Date32` and `Date64` return UTC midnight as they do not have time resolution

See notes on [`PrimitiveArray::value`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.value "method arrow::array::PrimitiveArray::value") regarding nulls and panics

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value_as_duration" class="fn">value_as_duration</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/time_delta/struct.TimeDelta.html" class="struct" title="struct chrono::time_delta::TimeDelta">TimeDelta</a>\>

Returns a value as a chrono `Duration`

If a data type cannot be converted to `Duration`, a `None` is returned

See notes on [`PrimitiveArray::value`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.value "method arrow::array::PrimitiveArray::value") regarding nulls and panics

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-PrimitiveArray%3CT%3E-2" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.iter" class="fn">iter</a>(&'a self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\> <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#" class="tooltip" data-notable-ty="ArrayIter&lt;&amp;&#39;a PrimitiveArray&lt;T&gt;&gt;">ⓘ</a>

constructs a new iterator

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-PrimitiveArray%3CT%3E-3" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from_trusted_len_iter" class="fn">from_trusted_len_iter</a>\<I, P\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where P: <a href="https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html" class="trait" title="trait core::borrow::Borrow">Borrow</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = P\>,

Creates a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") from an iterator of trusted length.

##### <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#safety-2" class="doc-anchor">§</a>Safety

The iterator must be [`TrustedLen`](https://doc.rust-lang.org/std/iter/trait.TrustedLen.html). I.e. that `size_hint().1` correctly reports its length.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-PrimitiveArray%3CT%3E-4" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowTimestampType.html" class="trait" title="trait arrow::datatypes::ArrowTimestampType">ArrowTimestampType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.timezone" class="fn">timezone</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the timezone of this array if any

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.with_timezone" class="fn">with_timezone</a>(self, timezone: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Construct a timestamp array with new timezone

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.with_timezone_utc" class="fn">with_timezone_utc</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Construct a timestamp array with UTC

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.with_timezone_opt" class="fn">with_timezone_opt</a>\<S\>(self, timezone: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<S\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>,

Construct a timestamp array with an optional timezone

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-PrimitiveArray%3CT%3E-5" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.DecimalType.html" class="trait" title="trait arrow::datatypes::DecimalType">DecimalType</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.with_precision_and_scale" class="fn">with_precision_and_scale</a>( self, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, scale: <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Returns a Decimal array with the same data as self, with the specified precision and scale.

See [`validate_decimal_precision_and_scale`](https://docs.rs/arrow/latest/arrow/datatypes/fn.validate_decimal_precision_and_scale.html "fn arrow::datatypes::validate_decimal_precision_and_scale")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.validate_decimal_precision" class="fn">validate_decimal_precision</a>( &self, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Validates values in this array can be properly interpreted with the specified precision.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.null_if_overflow_precision" class="fn">null_if_overflow_precision</a>(&self, precision: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Validates the Decimal Array, if the value of slot is overflow for the specified precision, and will be casted to Null

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value_as_string" class="fn">value_as_string</a>(&self, row: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns [`Self::value`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.value "method arrow::array::PrimitiveArray::value") formatted as a string

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.precision" class="fn">precision</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Returns the decimal precision of this array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.scale" class="fn">scale</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive">i8</a>

Returns the decimal scale of this array

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-Array-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.to_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.to_data" class="fn">to_data</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.into_data" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data" class="fn">into_data</a>(self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Returns the underlying data of this array [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.into_data)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

Returns a reference to the [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.data_type)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Returns a zero-copy slice of this array with the indicated offset and length. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.slice)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.len-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length (i.e., number of elements) of this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.len)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.is_empty-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether this array is empty. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.is_empty)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrinks the capacity of any exclusively owned buffer as much as possible [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.shrink_to_fit)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the offset into the underlying data used by this array(-slice). Note that the underlying data can be shared by many arrays. This defaults to `0`. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.offset)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls" class="fn">nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns the null buffer of this array if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.logical_null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count" class="fn">logical_null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of logical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.get_buffer_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_buffer_memory_size" class="fn">get_buffer_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory pointed to by this array. The buffers store bytes in the Arrow memory format, and include the data as well as the validity map. Note that this does not always correspond to the exact memory usage of an array, since multiple arrays can share the same buffers or slices thereof.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.get_array_memory_size" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.get_array_memory_size" class="fn">get_array_memory_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of bytes of memory occupied physically by this array. This value will always be greater than returned by `get_buffer_memory_size()` and includes the overhead of the data structures that contain the pointers to the various buffers.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.logical_nulls" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls" class="fn">logical_nulls</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct arrow::buffer::NullBuffer">NullBuffer</a>\>

Returns a potentially computed [`NullBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.NullBuffer.html "struct arrow::buffer::NullBuffer") that represents the logical null values of this array, if any. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.logical_nulls)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.is_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null" class="fn">is_null</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is null according to [`Array::nulls`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#tymethod.nulls "method arrow::array::Array::nulls") [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the element at `index` is *not* null, the opposite of [`Self::is_null`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null::is_null"). [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_valid)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.null_count" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total number of physical null values in this array. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.null_count)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.is_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable" class="fn">is_nullable</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `false` if the array is guaranteed to not contain any logical nulls [Read more](https://docs.rs/arrow/latest/arrow/array/trait.Array.html#method.is_nullable)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-ArrayAccessor-for-%26PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = \<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>

The Arrow type of the element being accessed.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value" class="fn">value</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.value_unchecked-1" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked" class="fn">value_unchecked</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i` [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-Clone-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-Debug-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CArrayData%3E-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Constructs a `PrimitiveArray` from an array data reference.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-65" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(data: <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CPrimitiveArray%3CT%3E%3E-for-ArrayData" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(array: <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDate32Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDate32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-31" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDate64Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDate64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-33" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDecimal128Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDecimal128Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-27" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDecimal256Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDecimal256Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-29" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDecimal32Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDecimal32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-23" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDecimal64Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDecimal64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-25" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDurationMicrosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDurationMicrosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-53" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDurationMillisecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDurationMillisecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-51" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDurationNanosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDurationNanosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-55" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CDurationSecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CDurationSecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-49" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CFloat16Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CFloat16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-17" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CFloat32Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CFloat32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-19" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CFloat64Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CFloat64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-21" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CInt16Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CInt16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CInt32Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CInt64Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CInt8Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CInt8Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CIntervalDayTimeType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CIntervalDayTimeType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-45" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CIntervalMonthDayNanoType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CIntervalMonthDayNanoType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-47" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CIntervalYearMonthType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CIntervalYearMonthType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-43" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CTime32MillisecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CTime32MillisecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-37" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CTime32SecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CTime32SecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-35" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CTime64MicrosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CTime64MicrosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-39" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CTime64NanosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CTime64NanosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-41" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CTimestampMicrosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CTimestampMicrosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-61" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CTimestampMillisecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CTimestampMillisecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-59" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CTimestampNanosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CTimestampNanosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-63" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CTimestampSecondType+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CTimestampSecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-57" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CUInt16Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CUInt16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CUInt32Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CUInt32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-13" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CUInt64Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CUInt64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-15" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3C%3CUInt8Type+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveArray%3CUInt8Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDate32Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDate32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-32" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date32Type.html" class="struct" title="struct arrow::datatypes::Date32Type">Date32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDate64Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDate64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-34" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Date64Type.html" class="struct" title="struct arrow::datatypes::Date64Type">Date64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDecimal128Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDecimal128Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-28" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal128Type.html" class="struct" title="struct arrow::datatypes::Decimal128Type">Decimal128Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDecimal256Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDecimal256Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-30" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal256Type.html" class="struct" title="struct arrow::datatypes::Decimal256Type">Decimal256Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDecimal32Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDecimal32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-24" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal32Type.html" class="struct" title="struct arrow::datatypes::Decimal32Type">Decimal32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDecimal64Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDecimal64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-26" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Decimal64Type.html" class="struct" title="struct arrow::datatypes::Decimal64Type">Decimal64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDurationMicrosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDurationMicrosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-54" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMicrosecondType.html" class="struct" title="struct arrow::datatypes::DurationMicrosecondType">DurationMicrosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDurationMillisecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDurationMillisecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-52" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationMillisecondType.html" class="struct" title="struct arrow::datatypes::DurationMillisecondType">DurationMillisecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDurationNanosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDurationNanosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-56" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationNanosecondType.html" class="struct" title="struct arrow::datatypes::DurationNanosecondType">DurationNanosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CDurationSecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CDurationSecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-50" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.DurationSecondType.html" class="struct" title="struct arrow::datatypes::DurationSecondType">DurationSecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CFloat16Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CFloat16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-18" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float16Type.html" class="struct" title="struct arrow::datatypes::Float16Type">Float16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CFloat32Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CFloat32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-20" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float32Type.html" class="struct" title="struct arrow::datatypes::Float32Type">Float32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CFloat64Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CFloat64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-22" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Float64Type.html" class="struct" title="struct arrow::datatypes::Float64Type">Float64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CInt16Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CInt16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int16Type.html" class="struct" title="struct arrow::datatypes::Int16Type">Int16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CInt32Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int32Type.html" class="struct" title="struct arrow::datatypes::Int32Type">Int32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CInt64Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int64Type.html" class="struct" title="struct arrow::datatypes::Int64Type">Int64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CInt8Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CInt8Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Int8Type.html" class="struct" title="struct arrow::datatypes::Int8Type">Int8Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CIntervalDayTimeType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CIntervalDayTimeType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-46" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html" class="struct" title="struct arrow::datatypes::IntervalDayTimeType">IntervalDayTimeType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CIntervalMonthDayNanoType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CIntervalMonthDayNanoType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-48" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html" class="struct" title="struct arrow::datatypes::IntervalMonthDayNanoType">IntervalMonthDayNanoType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CIntervalYearMonthType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CIntervalYearMonthType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-44" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html" class="struct" title="struct arrow::datatypes::IntervalYearMonthType">IntervalYearMonthType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CTime32MillisecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CTime32MillisecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-38" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32MillisecondType.html" class="struct" title="struct arrow::datatypes::Time32MillisecondType">Time32MillisecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CTime32SecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CTime32SecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-36" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time32SecondType.html" class="struct" title="struct arrow::datatypes::Time32SecondType">Time32SecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CTime64MicrosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CTime64MicrosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-40" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64MicrosecondType.html" class="struct" title="struct arrow::datatypes::Time64MicrosecondType">Time64MicrosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CTime64NanosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CTime64NanosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-42" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Time64NanosecondType.html" class="struct" title="struct arrow::datatypes::Time64NanosecondType">Time64NanosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CTimestampMicrosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CTimestampMicrosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-62" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMicrosecondType.html" class="struct" title="struct arrow::datatypes::TimestampMicrosecondType">TimestampMicrosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CTimestampMillisecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CTimestampMillisecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-60" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampMillisecondType.html" class="struct" title="struct arrow::datatypes::TimestampMillisecondType">TimestampMillisecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CTimestampNanosecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CTimestampNanosecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-64" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampNanosecondType.html" class="struct" title="struct arrow::datatypes::TimestampNanosecondType">TimestampNanosecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CTimestampSecondType+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CTimestampSecondType%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-58" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.TimestampSecondType.html" class="struct" title="struct arrow::datatypes::TimestampSecondType">TimestampSecondType</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CUInt16Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CUInt16Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-12" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt16Type.html" class="struct" title="struct arrow::datatypes::UInt16Type">UInt16Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CUInt32Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CUInt32Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-14" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt32Type.html" class="struct" title="struct arrow::datatypes::UInt32Type">UInt32Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CUInt64Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CUInt64Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-16" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt64Type.html" class="struct" title="struct arrow::datatypes::UInt64Type">UInt64Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-From%3CVec%3COption%3C%3CUInt8Type+as+ArrowPrimitiveType%3E::Native%3E%3E%3E-for-PrimitiveArray%3CUInt8Type%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a> as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.UInt8Type.html" class="struct" title="struct arrow::datatypes::UInt8Type">UInt8Type</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-FromIterator%3CPtr%3E-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T, Ptr\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<Ptr\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, Ptr: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.NativeAdapter.html" class="struct" title="struct arrow::array::NativeAdapter">NativeAdapter</a>\<T\>\>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = Ptr\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-IntoIterator-for-%26PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<T as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>

The type of the elements being iterated over.

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayIter.html" class="struct" title="struct arrow::array::ArrayIter">ArrayIter</a>\<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> \<&'a <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#impl-PartialEq-for-PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/array/struct.PrimitiveArray.html#blanket-implementations" class="anchor">§</a>
