# Trait AsArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#826" class="src">Source</a>

``` rust
pub trait AsArray: Sealed {
Show 36 methods    // Required methods
    fn as_boolean_opt(&self) -> Option<&BooleanArray>;
    fn as_primitive_opt<T>(&self) -> Option<&PrimitiveArray<T>>
       where T: ArrowPrimitiveType;
    fn as_bytes_opt<T>(&self) -> Option<&GenericByteArray<T>>
       where T: ByteArrayType;
    fn as_byte_view_opt<T>(&self) -> Option<&GenericByteViewArray<T>>
       where T: ByteViewType;
    fn as_struct_opt(&self) -> Option<&StructArray>;
    fn as_union_opt(&self) -> Option<&UnionArray>;
    fn as_list_opt<O>(&self) -> Option<&GenericListArray<O>>
       where O: OffsetSizeTrait;
    fn as_list_view_opt<O>(&self) -> Option<&GenericListViewArray<O>>
       where O: OffsetSizeTrait;
    fn as_fixed_size_binary_opt(&self) -> Option<&FixedSizeBinaryArray>;
    fn as_fixed_size_list_opt(&self) -> Option<&FixedSizeListArray>;
    fn as_map_opt(&self) -> Option<&MapArray>;
    fn as_dictionary_opt<K>(&self) -> Option<&DictionaryArray<K>>
       where K: ArrowDictionaryKeyType;
    fn as_run_opt<K>(&self) -> Option<&RunArray<K>>
       where K: RunEndIndexType;
    fn as_any_dictionary_opt(&self) -> Option<&dyn AnyDictionaryArray>;

    // Provided methods
    fn as_boolean(&self) -> &BooleanArray { ... }
    fn as_primitive<T>(&self) -> &PrimitiveArray<T>
       where T: ArrowPrimitiveType { ... }
    fn as_bytes<T>(&self) -> &GenericByteArray<T>
       where T: ByteArrayType { ... }
    fn as_string_opt<O>(
        &self,
    ) -> Option<&GenericByteArray<GenericStringType<O>>>
       where O: OffsetSizeTrait { ... }
    fn as_string<O>(&self) -> &GenericByteArray<GenericStringType<O>>
       where O: OffsetSizeTrait { ... }
    fn as_binary_opt<O>(
        &self,
    ) -> Option<&GenericByteArray<GenericBinaryType<O>>>
       where O: OffsetSizeTrait { ... }
    fn as_binary<O>(&self) -> &GenericByteArray<GenericBinaryType<O>>
       where O: OffsetSizeTrait { ... }
    fn as_string_view_opt(
        &self,
    ) -> Option<&GenericByteViewArray<StringViewType>> { ... }
    fn as_string_view(&self) -> &GenericByteViewArray<StringViewType> { ... }
    fn as_binary_view_opt(
        &self,
    ) -> Option<&GenericByteViewArray<BinaryViewType>> { ... }
    fn as_binary_view(&self) -> &GenericByteViewArray<BinaryViewType> { ... }
    fn as_byte_view<T>(&self) -> &GenericByteViewArray<T>
       where T: ByteViewType { ... }
    fn as_struct(&self) -> &StructArray { ... }
    fn as_union(&self) -> &UnionArray { ... }
    fn as_list<O>(&self) -> &GenericListArray<O>
       where O: OffsetSizeTrait { ... }
    fn as_list_view<O>(&self) -> &GenericListViewArray<O>
       where O: OffsetSizeTrait { ... }
    fn as_fixed_size_binary(&self) -> &FixedSizeBinaryArray { ... }
    fn as_fixed_size_list(&self) -> &FixedSizeListArray { ... }
    fn as_map(&self) -> &MapArray { ... }
    fn as_dictionary<K>(&self) -> &DictionaryArray<K>
       where K: ArrowDictionaryKeyType { ... }
    fn as_run<K>(&self) -> &RunArray<K>
       where K: RunEndIndexType { ... }
    fn as_any_dictionary(&self) -> &dyn AnyDictionaryArray { ... }
}
```

Expand description

An extension trait for `dyn Array` that provides ergonomic downcasting

``` rust
let col = Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef;
assert_eq!(col.as_primitive::<Int32Type>().values(), &[1, 2, 3]);
```

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_boolean_opt" class="fn">as_boolean_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>\>

Downcast this to a [`BooleanArray`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_primitive_opt" class="fn">as_primitive_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Downcast this to a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_bytes_opt" class="fn">as_bytes_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

Downcast this to a [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_byte_view_opt" class="fn">as_byte_view_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>,

Downcast this to a [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_struct_opt" class="fn">as_struct_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>\>

Downcast this to a [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_union_opt" class="fn">as_union_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>\>

Downcast this to a [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_list_opt" class="fn">as_list_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_list_view_opt" class="fn">as_list_view_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_fixed_size_binary_opt" class="fn">as_fixed_size_binary_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>\>

Downcast this to a [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_fixed_size_list_opt" class="fn">as_fixed_size_list_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>\>

Downcast this to a [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_map_opt" class="fn">as_map_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>\>

Downcast this to a [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_dictionary_opt" class="fn">as_dictionary_opt</a>\<K\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

Downcast this to a [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_run_opt" class="fn">as_run_opt</a>\<K\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<K\>\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

Downcast this to a [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_any_dictionary_opt" class="fn">as_any_dictionary_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::AnyDictionaryArray">AnyDictionaryArray</a>\>

Downcasts this to a [`AnyDictionaryArray`](https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html "trait arrow::array::AnyDictionaryArray") returning `None` if not possible

## Provided Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_boolean" class="fn">as_boolean</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>

Downcast this to a [`BooleanArray`](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_primitive" class="fn">as_primitive</a>\<T\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

Downcast this to a [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_bytes" class="fn">as_bytes</a>\<T\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

Downcast this to a [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string_opt" class="fn">as_string_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string" class="fn">as_string</a>\<O\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_binary_opt" class="fn">as_binary_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_binary" class="fn">as_binary</a>\<O\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericBinaryType.html" class="struct" title="struct arrow::datatypes::GenericBinaryType">GenericBinaryType</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericBinaryArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericBinaryArray.html "type arrow::array::GenericBinaryArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string_view_opt" class="fn">as_string_view_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct arrow::datatypes::StringViewType">StringViewType</a>\>\>

Downcast this to a [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string_view" class="fn">as_string_view</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct arrow::datatypes::StringViewType">StringViewType</a>\>

Downcast this to a [`StringViewArray`](https://docs.rs/arrow/latest/arrow/array/type.StringViewArray.html "type arrow::array::StringViewArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_binary_view_opt" class="fn">as_binary_view_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>\>\>

Downcast this to a [`BinaryViewArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryViewArray.html "type arrow::array::BinaryViewArray") returning `None` if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_binary_view" class="fn">as_binary_view</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.BinaryViewType.html" class="struct" title="struct arrow::datatypes::BinaryViewType">BinaryViewType</a>\>

Downcast this to a [`BinaryViewArray`](https://docs.rs/arrow/latest/arrow/array/type.BinaryViewArray.html "type arrow::array::BinaryViewArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_byte_view" class="fn">as_byte_view</a>\<T\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>,

Downcast this to a [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_struct" class="fn">as_struct</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>

Downcast this to a [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html "struct arrow::array::StructArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_union" class="fn">as_union</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>

Downcast this to a [`UnionArray`](https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html "struct arrow::array::UnionArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_list" class="fn">as_list</a>\<O\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_list_view" class="fn">as_list_view</a>\<O\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<O\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

Downcast this to a [`GenericListViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html "struct arrow::array::GenericListViewArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_fixed_size_binary" class="fn">as_fixed_size_binary</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>

Downcast this to a [`FixedSizeBinaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html "struct arrow::array::FixedSizeBinaryArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_fixed_size_list" class="fn">as_fixed_size_list</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>

Downcast this to a [`FixedSizeListArray`](https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html "struct arrow::array::FixedSizeListArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_map" class="fn">as_map</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>

Downcast this to a [`MapArray`](https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html "struct arrow::array::MapArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_dictionary" class="fn">as_dictionary</a>\<K\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

Downcast this to a [`DictionaryArray`](https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html "struct arrow::array::DictionaryArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_run" class="fn">as_run</a>\<K\>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<K\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

Downcast this to a [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") panicking if not possible

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_any_dictionary" class="fn">as_any_dictionary</a>(&self) -\> &dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::AnyDictionaryArray">AnyDictionaryArray</a>

Downcasts this to a [`AnyDictionaryArray`](https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html "trait arrow::array::AnyDictionaryArray") panicking if not possible

## Dyn Compatibility<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#impl-AsArray-for-Arc%3Cdyn+Array%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html" class="trait" title="trait arrow::array::AsArray">AsArray</a> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_boolean_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_boolean_opt" class="fn">as_boolean_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_primitive_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_primitive_opt" class="fn">as_primitive_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html" class="struct" title="struct arrow::array::PrimitiveArray">PrimitiveArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_bytes_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_bytes_opt" class="fn">as_bytes_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_byte_view_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_byte_view_opt" class="fn">as_byte_view_opt</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<T\>\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteViewType.html" class="trait" title="trait arrow::datatypes::ByteViewType">ByteViewType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_struct_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_struct_opt" class="fn">as_struct_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html" class="struct" title="struct arrow::array::StructArray">StructArray</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_union_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_union_opt" class="fn">as_union_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.UnionArray.html" class="struct" title="struct arrow::array::UnionArray">UnionArray</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_list_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_list_opt" class="fn">as_list_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html" class="struct" title="struct arrow::array::GenericListArray">GenericListArray</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_list_view_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_list_view_opt" class="fn">as_list_view_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericListViewArray.html" class="struct" title="struct arrow::array::GenericListViewArray">GenericListViewArray</a>\<O\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_fixed_size_binary_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_fixed_size_binary_opt" class="fn">as_fixed_size_binary_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeBinaryArray.html" class="struct" title="struct arrow::array::FixedSizeBinaryArray">FixedSizeBinaryArray</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_fixed_size_list_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_fixed_size_list_opt" class="fn">as_fixed_size_list_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.FixedSizeListArray.html" class="struct" title="struct arrow::array::FixedSizeListArray">FixedSizeListArray</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_map_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_map_opt" class="fn">as_map_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.MapArray.html" class="struct" title="struct arrow::array::MapArray">MapArray</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_dictionary_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_dictionary_opt" class="fn">as_dictionary_opt</a>\<K\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.DictionaryArray.html" class="struct" title="struct arrow::array::DictionaryArray">DictionaryArray</a>\<K\>\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowDictionaryKeyType.html" class="trait" title="trait arrow::datatypes::ArrowDictionaryKeyType">ArrowDictionaryKeyType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_any_dictionary_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_any_dictionary_opt" class="fn">as_any_dictionary_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AnyDictionaryArray.html" class="trait" title="trait arrow::array::AnyDictionaryArray">AnyDictionaryArray</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_run_opt" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#tymethod.as_run_opt" class="fn">as_run_opt</a>\<K\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<K\>\>

where K: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string_opt-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#method.as_string_opt" class="fn">as_string_opt</a>\<O\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html" class="struct" title="struct arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct arrow::datatypes::GenericStringType">GenericStringType</a>\<O\>\>\>

where O: <a href="https://docs.rs/arrow/latest/arrow/array/trait.OffsetSizeTrait.html" class="trait" title="trait arrow::array::OffsetSizeTrait">OffsetSizeTrait</a>,

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html#impl-AsArray-for-dyn+Array" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.AsArray.html" class="trait" title="trait arrow::array::AsArray">AsArray</a> for dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a> + '\_
