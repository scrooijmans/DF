# Struct Tensor Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/tensor.rs.html#74-81" class="src">Source</a>

``` rust
pub struct Tensor<'a, T: ArrowPrimitiveType> { /* private fields */ }
```

Expand description

Tensor of primitive types

## Implementations<a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#impl-Tensor%3C&#39;a,+T%3E" class="anchor">§</a>

### impl\<'a, T: <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\> <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html" class="struct" title="struct arrow::tensor::Tensor">Tensor</a>\<'a, T\>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.try_new" class="fn">try_new</a>( buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, shape: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, strides: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, names: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html" class="type" title="type arrow::error::Result">Result</a>\<Self\>

Creates a new `Tensor`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.new_row_major" class="fn">new_row_major</a>( buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, shape: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, names: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html" class="type" title="type arrow::error::Result">Result</a>\<Self\>

Creates a new Tensor using row major memory layout

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.new_column_major" class="fn">new_column_major</a>( buffer: <a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>, shape: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, names: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html" class="type" title="type arrow::error::Result">Result</a>\<Self\>

Creates a new Tensor using column major memory layout

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.data_type" class="fn">data_type</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html" class="enum" title="enum arrow::datatypes::DataType">DataType</a>

The data type of the `Tensor`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.shape" class="fn">shape</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

The sizes of the dimensions

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.data" class="fn">data</a>(&self) -\> &<a href="https://docs.rs/arrow/latest/arrow/buffer/struct.Buffer.html" class="struct" title="struct arrow::buffer::Buffer">Buffer</a>

Returns a reference to the underlying `Buffer`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.strides" class="fn">strides</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

The number of bytes between elements in each dimension

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.names" class="fn">names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>

The names of the dimensions

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.ndim" class="fn">ndim</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The number of dimensions

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.dim_name" class="fn">dim_name</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

The name of dimension i

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The total number of elements in the `Tensor`

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.is_contiguous" class="fn">is_contiguous</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html" class="type" title="type arrow::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Indicates if the data is laid out contiguously in memory

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.is_row_major" class="fn">is_row_major</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html" class="type" title="type arrow::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Indicates if the memory layout row major

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.is_column_major" class="fn">is_column_major</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/error/type.Result.html" class="type" title="type arrow::error::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Indicates if the memory layout column major

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#impl-Debug-for-Tensor%3C&#39;a,+T%3E" class="anchor">§</a>

### impl\<'a, T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html" class="trait" title="trait arrow::array::ArrowPrimitiveType">ArrowPrimitiveType</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html" class="struct" title="struct arrow::tensor::Tensor">Tensor</a>\<'a, T\>

<a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/tensor/struct.Tensor.html#blanket-implementations" class="anchor">§</a>
