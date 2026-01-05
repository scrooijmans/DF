# Struct BufferBuilder Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/builder/mod.rs.html#51" class="src">Source</a>

``` rust
pub struct BufferBuilder<T>where
    T: ArrowNativeType,{ /* private fields */ }
```

Expand description

Builder for creating a [Buffer](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") object.

A [Buffer](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer") is the underlying data structure of Arrow’s Arrays.

For all supported types, there are type definitions for the generic version of `BufferBuilder<T>`, e.g. `BufferBuilder`.

## <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(100);
builder.append_slice(&[42, 43, 44]);
builder.append(45);
let buffer = builder.finish();

assert_eq!(unsafe { buffer.typed_data::<u8>() }, &[42, 43, 44, 45]);
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#impl-BufferBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.new" class="fn">new</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

Creates a new builder with initial capacity for *at least* `capacity` elements of type `T`.

The capacity can later be manually adjusted with the [`reserve()`](https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html#method.reserve "method arrow::array::BufferBuilder::reserve") method. Also the [`append()`](https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html#method.append "method arrow::array::BufferBuilder::append"), [`append_slice()`](https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html#method.append_slice "method arrow::array::BufferBuilder::append_slice") and [`advance()`](https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html#method.advance "method arrow::array::BufferBuilder::advance") methods automatically increase the capacity if needed.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-1" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);

assert!(builder.capacity() >= 10);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.new_from_buffer" class="fn">new_from_buffer</a>(buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html" class="struct" title="struct arrow::buffer::MutableBuffer">MutableBuffer</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

Creates a new builder from a [`MutableBuffer`](https://docs.rs/arrow/latest/arrow/buffer/struct.MutableBuffer.html "struct arrow::buffer::MutableBuffer")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the current number of array elements in the internal buffer.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-2" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);
builder.append(42);

assert_eq!(builder.len(), 1);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns whether the internal buffer is empty.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-3" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);
builder.append(42);

assert_eq!(builder.is_empty(), false);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the actual capacity (number of elements) of the internal buffer.

Note: the internal capacity returned by this method might be larger than what you’d expect after setting the capacity in the `new()` or `reserve()` functions.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.advance" class="fn">advance</a>(&mut self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Increases the number of elements in the internal buffer by `n` and resizes the buffer as needed.

The values of the newly added elements are 0. This method is usually used when appending `NULL` values to the buffer as they still require physical memory space.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-4" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);
builder.advance(2);

assert_eq!(builder.len(), 2);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.reserve" class="fn">reserve</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves memory for *at least* `n` more elements of type `T`.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-5" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);
builder.reserve(10);

assert!(builder.capacity() >= 20);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.append" class="fn">append</a>(&mut self, v: T)

Appends a value of type `T` into the builder, growing the internal buffer as needed.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-6" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);
builder.append(42);

assert_eq!(builder.len(), 1);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.append_n" class="fn">append_n</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, v: T)

Appends a value of type `T` into the builder N times, growing the internal buffer as needed.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-7" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);
builder.append_n(10, 42);

assert_eq!(builder.len(), 10);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.append_n_zeroed" class="fn">append_n_zeroed</a>(&mut self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Appends `n`, zero-initialized values

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-8" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u32>::new(10);
builder.append_n_zeroed(3);

assert_eq!(builder.len(), 3);
assert_eq!(builder.as_slice(), &[0, 0, 0])
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.append_slice" class="fn">append_slice</a>(&mut self, slice: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>)

Appends a slice of type `T`, growing the internal buffer as needed.

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-9" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);
builder.append_slice(&[42, 44, 46]);

assert_eq!(builder.len(), 3);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.as_slice" class="fn">as_slice</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

View the contents of this buffer as a slice

``` rust

let mut builder = BufferBuilder::<f64>::new(10);
builder.append(1.3);
builder.append_n(2, 2.3);

assert_eq!(builder.as_slice(), &[1.3, 2.3, 2.3]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.as_slice_mut" class="fn">as_slice_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

View the contents of this buffer as a mutable slice

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-10" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<f32>::new(10);

builder.append_slice(&[1., 2., 3.4]);
assert_eq!(builder.as_slice(), &[1., 2., 3.4]);

builder.as_slice_mut()[1] = 4.2;
assert_eq!(builder.as_slice(), &[1., 4.2, 3.4]);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.truncate" class="fn">truncate</a>(&mut self, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Shorten this BufferBuilder to `len` items

If `len` is greater than the builder’s current length, this has no effect

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-11" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u16>::new(10);

builder.append_slice(&[42, 44, 46]);
assert_eq!(builder.as_slice(), &[42, 44, 46]);

builder.truncate(2);
assert_eq!(builder.as_slice(), &[42, 44]);

builder.append(12);
assert_eq!(builder.as_slice(), &[42, 44, 12]);
```

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.append_trusted_len_iter" class="fn">append_trusted_len_iter</a>( &mut self, iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>, )

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#safety" class="doc-anchor">§</a>Safety

This requires the iterator be a trusted length. This could instead require the iterator implement `TrustedLen` once that is stabilized.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.finish" class="fn">finish</a>(&mut self) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Resets this builder and returns an immutable [Buffer](https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html "struct arrow::buffer::Buffer").

##### <a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#example-12" class="doc-anchor">§</a>Example:

``` rust

let mut builder = BufferBuilder::<u8>::new(10);
builder.append_slice(&[42, 44, 46]);

let buffer = builder.finish();

assert_eq!(unsafe { buffer.typed_data::<u8>() }, &[42, 44, 46]);
```

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#impl-Debug-for-BufferBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#impl-Default-for-BufferBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#impl-Extend%3CT%3E-for-BufferBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<T\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#impl-From%3CBufferBuilder%3CT%3E%3E-for-Buffer" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#impl-From%3CBufferBuilder%3CT%3E%3E-for-ScalarBuffer%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.ScalarBuffer.html" class="struct" title="struct arrow::buffer::ScalarBuffer">ScalarBuffer</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#impl-From%3CVec%3CT%3E%3E-for-BufferBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#impl-FromIterator%3CT%3E-for-BufferBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<T\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.BufferBuilder.html" class="struct" title="struct arrow::array::BufferBuilder">BufferBuilder</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = T\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/builder/struct.BufferBuilder.html#blanket-implementations" class="anchor">§</a>
