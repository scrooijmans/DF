# Struct PrimitiveRunBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/primitive_run_builder.rs.html#61" class="src">Source</a>

``` rust
pub struct PrimitiveRunBuilder<R, V>where
    R: RunEndIndexType,
    V: ArrowPrimitiveType,{ /* private fields */ }
```

Expand description

Builder for [`RunArray`](https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html "struct arrow::array::RunArray") of [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#example" class="doc-anchor">§</a>Example:

``` rust


let mut builder =
PrimitiveRunBuilder::<Int16Type, UInt32Type>::new();
builder.append_value(1234);
builder.append_value(1234);
builder.append_value(1234);
builder.append_null();
builder.append_value(5678);
builder.append_value(5678);
let array = builder.finish();

assert_eq!(array.run_ends().values(), &[3, 4, 6]);

let av = array.values();

assert!(!av.is_null(0));
assert!(av.is_null(1));
assert!(!av.is_null(2));

// Values are polymorphic and so require a downcast.
let ava: &UInt32Array = av.as_primitive::<UInt32Type>();

assert_eq!(ava, &UInt32Array::from(vec![Some(1234), None, Some(5678)]));
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#impl-PrimitiveRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

Creates a new `PrimitiveRunBuilder`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

Creates a new `PrimitiveRunBuilder` with the provided capacity

`capacity`: the expected number of run-end encoded values.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#impl-PrimitiveRunBuilder%3CR,+V%3E-1" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.append_option" class="fn">append_option</a>( &mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>, )

Appends optional value to the logical array encoded by the RunArray.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.append_value" class="fn">append_value</a>(&mut self, value: \<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>)

Appends value to the logical array encoded by the run-ends array.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends null to the logical array encoded by the run-ends array.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

Creates the RunArray and resets the builder. Panics if RunArray cannot be built.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.RunArray.html" class="struct" title="struct arrow::array::RunArray">RunArray</a>\<R\>

Creates the RunArray and without resetting the builder. Panics if RunArray cannot be built.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#impl-ArrayBuilder-for-PrimitiveRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of logical array encoded by the eventual runs array.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#impl-Debug-for-PrimitiveRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

where R: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>, \<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#impl-Default-for-PrimitiveRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#impl-Extend%3COption%3C%3CV+as+ArrowPrimitiveType%3E::Native%3E%3E-for-PrimitiveRunBuilder%3CR,+V%3E" class="anchor">§</a>

### impl\<R, V\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html" class="struct" title="struct arrow::array::PrimitiveRunBuilder">PrimitiveRunBuilder</a>\<R, V\>

where R: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html" class="trait" title="trait arrow::datatypes::RunEndIndexType">RunEndIndexType</a>, V: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<T\>(&mut self, iter: T)

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<V as <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\>::<a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html#associatedtype.Native" class="associatedtype" title="type arrow::array::ArrowPrimitiveType::Native">Native</a>\>\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveRunBuilder.html#blanket-implementations" class="anchor">§</a>
