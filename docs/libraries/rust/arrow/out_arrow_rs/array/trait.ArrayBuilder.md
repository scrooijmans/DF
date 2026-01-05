# Trait ArrayBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/mod.rs.html#329" class="src">Source</a>

``` rust
pub trait ArrayBuilder:
    Any
    + Send
    + Sync {
    // Required methods
    fn len(&self) -> usize;
    fn finish(&mut self) -> Arc<dyn Array>;
    fn finish_cloned(&self) -> Arc<dyn Array>;
    fn as_any(&self) -> &(dyn Any + 'static);
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static);
    fn into_box_any(self: Box<Self>) -> Box<dyn Any>;

    // Provided method
    fn is_empty(&self) -> bool { ... }
}
```

Expand description

Trait for dealing with different array builders at runtime

## <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#example" class="doc-anchor">§</a>Example

``` rust
// Create

let mut data_builders: Vec<Box<dyn ArrayBuilder>> = vec![
    Box::new(Float64Builder::new()),
    Box::new(Int64Builder::new()),
    Box::new(StringBuilder::new()),
];

// Fill
data_builders[0]
    .as_any_mut()
    .downcast_mut::<Float64Builder>()
    .unwrap()
    .append_value(3.14);
data_builders[1]
    .as_any_mut()
    .downcast_mut::<Int64Builder>()
    .unwrap()
    .append_value(-1);
data_builders[2]
    .as_any_mut()
    .downcast_mut::<StringBuilder>()
    .unwrap()
    .append_value("🍎");

// Finish
let array_refs: Vec<ArrayRef> = data_builders
    .iter_mut()
    .map(|builder| builder.finish())
    .collect();
assert_eq!(array_refs[0].len(), 1);
assert_eq!(array_refs[1].is_null(0), false);
assert_eq!(
    array_refs[2]
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap()
        .value(0),
    "🍎"
);
```

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the underlying builder.

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

This is most useful when one wants to call non-mutable APIs on a specific builder type. In this case, one can first cast this into a `Any`, and then use `downcast_ref` to get a reference on the specific builder.

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

This is most useful when one wants to call mutable APIs on a specific builder type. In this case, one can first cast this into a `Any`, and then use `downcast_mut` to get a reference on the specific builder.

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<Self\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

## Provided Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-Box%3Cdyn+ArrayBuilder%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.finish_cloned" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the underlying builder.

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any)

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference. [Read more](https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut)

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-Box%3Cdyn+ArrayBuilder%3E-1" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.len-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.as_any-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.as_any_mut-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.into_box_any-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-BooleanBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanBuilder.html" class="struct" title="struct arrow::array::BooleanBuilder">BooleanBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-FixedSizeBinaryBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-NullBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.NullBuilder.html" class="struct" title="struct arrow::array::NullBuilder">NullBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-StructBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.StructBuilder.html" class="struct" title="struct arrow::array::StructBuilder">StructBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-UnionBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionBuilder.html" class="struct" title="struct arrow::array::UnionBuilder">UnionBuilder</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-FixedSizeBinaryDictionaryBuilder%3CK%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryDictionaryBuilder.html" class="struct" title="struct arrow::array::FixedSizeBinaryDictionaryBuilder">FixedSizeBinaryDictionaryBuilder</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-GenericByteDictionaryBuilder%3CK,+T%3E" class="anchor">§</a>

### impl\<K, T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteDictionaryBuilder.html" class="struct" title="struct arrow::array::GenericByteDictionaryBuilder">GenericByteDictionaryBuilder</a>\<K, T\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-MapBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.MapBuilder.html" class="struct" title="struct arrow::array::MapBuilder">MapBuilder</a>\<K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-PrimitiveDictionaryBuilder%3CK,+V%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveDictionaryBuilder.html" class="struct" title="struct arrow::array::PrimitiveDictionaryBuilder">PrimitiveDictionaryBuilder</a>\<K, V\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-GenericListBuilder%3COffsetSize,+T%3E" class="anchor">§</a>

### impl\<OffsetSize, T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListBuilder.html" class="struct" title="struct arrow::array::GenericListBuilder">GenericListBuilder</a>\<OffsetSize, T\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>, T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> + 'static,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-GenericListViewBuilder%3COffsetSize,+T%3E" class="anchor">§</a>

### impl\<OffsetSize, T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewBuilder.html" class="struct" title="struct arrow::array::GenericListViewBuilder">GenericListViewBuilder</a>\<OffsetSize, T\>

where OffsetSize: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>, T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-GenericByteRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-PrimitiveRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-FixedSizeListBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListBuilder.html" class="struct" title="struct arrow::array::FixedSizeListBuilder">FixedSizeListBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> + 'static,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-GenericByteBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteBuilder.html" class="struct" title="struct arrow::array::GenericByteBuilder">GenericByteBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-GenericByteViewBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewBuilder.html" class="struct" title="struct arrow::array::GenericByteViewBuilder">GenericByteViewBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#impl-ArrayBuilder-for-PrimitiveBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveBuilder.html" class="struct" title="struct arrow::array::PrimitiveBuilder">PrimitiveBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,
