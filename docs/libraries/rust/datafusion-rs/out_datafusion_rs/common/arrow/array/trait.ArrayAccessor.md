# Trait ArrayAccessor Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/array/mod.rs.html#574" class="src">Source</a>

``` rust
pub trait ArrayAccessor: Array {
    type Item: Send + Sync;

    // Required methods
    fn value(&self, index: usize) -> Self::Item;
    unsafe fn value_unchecked(&self, index: usize) -> Self::Item;
}
```

Expand description

A generic trait for accessing the values of an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array")

This trait helps write specialized implementations of algorithms for different array types. Specialized implementations allow the compiler to optimize the code for the specific array type, which can lead to significant performance improvements.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#example" class="doc-anchor">§</a>Example

For example, to write three different implementations of a string length function for [`StringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringArray.html "type datafusion::common::arrow::array::StringArray"), [`LargeStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeStringArray.html "type datafusion::common::arrow::array::LargeStringArray"), and [`StringViewArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.StringViewArray.html "type datafusion::common::arrow::array::StringViewArray"), you can write

``` rust
/// This function takes a dynamically typed `ArrayRef` and calls
/// calls one of three specialized implementations
fn character_length(arg: ArrayRef) -> Result<ArrayRef, ArrowError> {
    match arg.data_type() {
        DataType::Utf8 => {
            // downcast the ArrayRef to a StringArray and call the specialized implementation
            let string_array = arg.as_string::<i32>();
            character_length_general::<Int32Type, _>(string_array)
        }
        DataType::LargeUtf8 => {
            character_length_general::<Int64Type, _>(arg.as_string::<i64>())
        }
        DataType::Utf8View => {
            character_length_general::<Int32Type, _>(arg.as_string_view())
        }
        _ => Err(ArrowError::InvalidArgumentError("Unsupported data type".to_string())),
    }
}

/// A generic implementation of the character_length function
/// This function uses the `ArrayAccessor` trait to access the values of the array
/// so the compiler can generated specialized implementations for different array types
///
/// Returns a new array with the length of each string in the input array
/// * Int32Array for Utf8 and Utf8View arrays (lengths are 32-bit integers)
/// * Int64Array for LargeUtf8 arrays (lengths are 64-bit integers)
///
/// This is generic on the type of the primitive array (different string arrays have
/// different lengths) and the type of the array accessor (different string arrays
/// have different ways to access the values)
fn character_length_general<'a, T: ArrowPrimitiveType, V: ArrayAccessor<Item = &'a str>>(
    array: V,
) -> Result<ArrayRef, ArrowError>
where
    T::Native: OffsetSizeTrait,
{
    let iter = ArrayIter::new(array);
    // Create a Int32Array / Int64Array with the length of each string
    let result = iter
        .map(|string| {
            string.map(|string: &str| {
                T::Native::from_usize(string.chars().count())
                    .expect("should not fail as string.chars will always return integer")
            })
        })
        .collect::<PrimitiveArray<T>>();

    /// Return the result as a new ArrayRef (dynamically typed)
    Ok(Arc::new(result) as ArrayRef)
}
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#validity" class="doc-anchor">§</a>Validity

An [`ArrayAccessor`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html "trait datafusion::common::arrow::array::ArrayAccessor") must always return a well-defined value for an index that is within the bounds `0..Array::len`, including for null indexes where [`Array::is_null`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html#method.is_null "method datafusion::common::arrow::array::Array::is_null") is true.

The value at null indexes is unspecified, and implementations must not rely on a specific value such as [`Default::default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default") being returned, however, it must not be undefined

## Required Associated Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>

The Arrow type of the element being accessed.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value" class="fn">value</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#panics" class="doc-anchor">§</a>Panics

Panics if the value is outside the bounds of the array

#### unsafe fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#tymethod.value_unchecked" class="fn">value_unchecked</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

Returns the element at index `i`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#safety" class="doc-anchor">§</a>Safety

Caller is responsible for ensuring that the index is within the bounds of the array

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26BooleanArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.BooleanArray.html" class="struct" title="struct datafusion::common::arrow::array::BooleanArray">BooleanArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26FixedSizeListArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct datafusion::common::arrow::array::FixedSizeListArray">FixedSizeListArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26MapArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.MapArray.html" class="struct" title="struct datafusion::common::arrow::array::MapArray">MapArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.StructArray.html" class="struct" title="struct datafusion::common::arrow::array::StructArray">StructArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-FixedSizeListArray" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct datafusion::common::arrow::array::FixedSizeListArray">FixedSizeListArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-4" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26FixedSizeBinaryArray" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct datafusion::common::arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-5" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-TypedDictionaryArray%3C&#39;a,+K,+V%3E" class="anchor">§</a>

### impl\<'a, K, V\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.TypedDictionaryArray.html" class="struct" title="struct datafusion::common::arrow::array::TypedDictionaryArray">TypedDictionaryArray</a>\<'a, K, V\>

where K: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, V: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>, \<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-6" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-TypedRunArray%3C&#39;a,+R,+V%3E" class="anchor">§</a>

### impl\<'a, R, V\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.TypedRunArray.html" class="struct" title="struct datafusion::common::arrow::array::TypedRunArray">TypedRunArray</a>\<'a, R, V\>

where R: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait datafusion::common::arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a>: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>, \<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-7" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = \<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype" title="type datafusion::common::arrow::array::ArrayAccessor::Item">Item</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26GenericByteArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-8" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = &'a \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteArrayType::Native">Native</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26GenericByteViewArray%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-9" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = &'a \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait datafusion::common::arrow::datatypes::ByteViewType">ByteViewType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/trait.ByteViewType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::datatypes::ByteViewType::Native">Native</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26GenericListArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListArray">GenericListArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-10" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26GenericListViewArray%3COffsetSize%3E" class="anchor">§</a>

### impl\<OffsetSize\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericListViewArray">GenericListViewArray</a>\<OffsetSize\>

where OffsetSize: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait datafusion::common::arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-11" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#impl-ArrayAccessor-for-%26PrimitiveArray%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html" class="trait" title="trait datafusion::common::arrow::array::ArrayAccessor">ArrayAccessor</a> for &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct datafusion::common::arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item-12" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html#associatedtype.Item" class="associatedtype">Item</a> = \<T as <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait datafusion::common::arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type datafusion::common::arrow::array::ArrowPrimitiveType::Native">Native</a>
