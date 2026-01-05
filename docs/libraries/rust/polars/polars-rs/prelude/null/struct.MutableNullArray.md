# Struct MutableNullArray Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/null.rs.html#8" class="src">Source</a>

``` rust
pub struct MutableNullArray { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#impl-MutableNullArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html" class="struct" title="struct polars::prelude::null::MutableNullArray">MutableNullArray</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.new" class="fn">new</a>(len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html" class="struct" title="struct polars::prelude::null::MutableNullArray">MutableNullArray</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.extend_nulls" class="fn">extend_nulls</a>(&mut self, null_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#impl-Clone-for-MutableNullArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html" class="struct" title="struct polars::prelude::null::MutableNullArray">MutableNullArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html" class="struct" title="struct polars::prelude::null::MutableNullArray">MutableNullArray</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#impl-Debug-for-MutableNullArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html" class="struct" title="struct polars::prelude::null::MutableNullArray">MutableNullArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#impl-Default-for-MutableNullArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html" class="struct" title="struct polars::prelude::null::MutableNullArray">MutableNullArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html" class="struct" title="struct polars::prelude::null::MutableNullArray">MutableNullArray</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#impl-MutableArray-for-MutableNullArray" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html" class="trait" title="trait polars_arrow::array::MutableArray">MutableArray</a> for <a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html" class="struct" title="struct polars::prelude::null::MutableNullArray">MutableNullArray</a>

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.dtype" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.dtype" class="fn">dtype</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html" class="enum" title="enum polars::prelude::ArrowDataType">ArrowDataType</a>

The [`ArrowDataType`](https://docs.rs/polars/latest/polars/prelude/enum.ArrowDataType.html "enum polars::prelude::ArrowDataType") of the array.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The length of the array.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.validity" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.validity" class="fn">validity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/mutable/struct.MutableBitmap.html" class="struct" title="struct polars_arrow::bitmap::mutable::MutableBitmap">MutableBitmap</a>\>

The optional validity of the array.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.as_box" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.as_box" class="fn">as_box</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Convert itself to an (immutable) [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array").

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Convert to `Any`, to enable dynamic casting.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.as_mut_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.as_mut_any" class="fn">as_mut_any</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Convert to mutable `Any`, to enable dynamic casting.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.push_null" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.push_null" class="fn">push_null</a>(&mut self)

Adds a new null element to the array.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.reserve" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.reserve" class="fn">reserve</a>(&mut self, \_additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves additional slots to its capacity.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.shrink_to_fit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#tymethod.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrink the array to fit its length.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the array is empty.

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.as_arc" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#method.as_arc" class="fn">as_arc</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>

Convert itself to an (immutable) atomically reference counted [`Array`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html "trait polars_arrow::array::Array").

<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#method.is_valid" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#method.is_valid" class="fn">is_valid</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether `index` is valid / set. [Read more](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.MutableArray.html#method.is_valid)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/null/struct.MutableNullArray.html#blanket-implementations" class="anchor">§</a>
