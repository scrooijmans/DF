# Struct GenericByteRunBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/generic_byte_run_builder.rs.html#65" class="src">Source</a>

``` rust
pub struct GenericByteRunBuilder<R, V>where
    R: ArrowPrimitiveType,
    V: ByteArrayType,{ /* private fields */ }
```

Expand description

Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`GenericByteArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteArray.html "struct arrow::array::GenericByteArray")

## <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#example" class="doc-anchor">§</a>Example:

``` rust


let mut builder =
GenericByteRunBuilder::<Int16Type, BinaryType>::new();
builder.extend([Some(b"abc"), Some(b"abc"), None, Some(b"def")].into_iter());
builder.append_value(b"def");
builder.append_null();
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[2, 3, 5, 6]);

let av = array.values();

assert!(!av.is_null(0));
assert!(av.is_null(1));
assert!(!av.is_null(2));
assert!(av.is_null(3));

// Values are polymorphic and so require a downcast.
let ava: &BinaryArray = av.as_binary();

assert_eq!(ava.value(0), b"abc");
assert_eq!(ava.value(2), b"def");
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#impl-GenericByteRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

Creates a new `GenericByteRunBuilder`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.with_capacity" class="fn">with_capacity</a>( capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, data_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

Creates a new `GenericByteRunBuilder` with the provided capacity

`capacity`: the expected number of run-end encoded values. `data_capacity`: the expected number of bytes of run end encoded values

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#impl-GenericByteRunBuilder%3CR,+V%3E-1" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.append_option" class="fn">append_option</a>( &mut self, input_value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<V as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Native">Native</a>\>\>, )

Appends optional value to the logical array encoded by the RunArray.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.append_value" class="fn">append_value</a>( &mut self, input_value: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<V as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Native">Native</a>\>, )

Appends value to the logical array encoded by the RunArray.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends null to the logical array encoded by the RunArray.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

Creates the RunArray and resets the builder. Panics if RunArray cannot be built.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

Creates the RunArray and without resetting the builder. Panics if RunArray cannot be built.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#impl-ArrayBuilder-for-GenericByteRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of logical array encoded by the eventual runs array.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#impl-Debug-for-GenericByteRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

where R: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, V: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#impl-Default-for-GenericByteRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#impl-Extend%3COption%3CS%3E%3E-for-GenericByteRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V, S\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<S\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.GenericByteRunBuilder.html" class="struct" title="struct arrow::array::GenericByteRunBuilder">GenericByteRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\<V as <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html" class="trait" title="trait arrow::datatypes::ByteArrayType">ByteArrayType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ByteArrayType.html#associatedtype.Native" class="associatedtype" title="type arrow::datatypes::ByteArrayType::Native">Native</a>\>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<S\>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.GenericByteRunBuilder.html#blanket-implementations" class="anchor">§</a>
