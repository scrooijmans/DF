# Struct NullBuilder Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/builder/null_builder.rs.html#46" class="src">Source</a>

``` rust
pub struct NullBuilder { /* private fields */ }
```

Expand description

Builder for [`NullArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html "struct datafusion::common::arrow::array::NullArray")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#example" class="doc-anchor">§</a>Example

Create a `NullArray` from a `NullBuilder`

``` rust


let mut b = NullBuilder::new();
b.append_empty_value();
b.append_null();
b.append_nulls(3);
b.append_empty_values(3);
let arr = b.finish();

assert_eq!(8, arr.len());
assert_eq!(0, arr.null_count());
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#impl-NullBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::NullBuilder">NullBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::NullBuilder">NullBuilder</a>

Creates a new null builder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a null slot into the builder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.append_nulls" class="fn">append_nulls</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `null`s into the builder.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.append_empty_value" class="fn">append_empty_value</a>(&mut self)

Appends a null slot into the builder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.append_empty_values" class="fn">append_empty_values</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n` `null`s into the builder.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

Builds the [NullArray](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html "struct datafusion::common::arrow::array::NullArray") and reset this builder.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html" class="struct" title="struct datafusion::common::arrow::array::NullArray">NullArray</a>

Builds the [NullArray](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullArray.html "struct datafusion::common::arrow::array::NullArray") without resetting the builder.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#impl-ArrayBuilder-for-NullBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayBuilder.html" class="trait" title="trait datafusion::common::arrow::array::ArrayBuilder">ArrayBuilder</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::NullBuilder">NullBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayBuilder.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a non-mutable `Any` reference.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.as_any_mut" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayBuilder.html#tymethod.as_any_mut" class="fn">as_any_mut</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the builder as a mutable `Any` reference.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.into_box_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayBuilder.html#tymethod.into_box_any" class="fn">into_box_any</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::NullBuilder">NullBuilder</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a>\>

Returns the boxed builder as a box of `Any`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayBuilder.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of array slots in the builder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.finish-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayBuilder.html#tymethod.finish" class="fn">finish</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Builds the array and reset this builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.finish_cloned-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayBuilder.html#tymethod.finish_cloned" class="fn">finish_cloned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html" class="trait" title="trait datafusion::common::arrow::array::Array">Array</a>\>

Builds the array without resetting the builder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether number of array slots is zero

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#impl-Debug-for-NullBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::NullBuilder">NullBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#impl-Default-for-NullBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::NullBuilder">NullBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::array::NullBuilder">NullBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/struct.NullBuilder.html#blanket-implementations" class="anchor">§</a>
