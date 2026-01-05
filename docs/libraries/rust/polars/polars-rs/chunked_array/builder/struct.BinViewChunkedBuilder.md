# Struct BinViewChunkedBuilder Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/builder/string.rs.html#3" class="src">Source</a>

``` rust
pub struct BinViewChunkedBuilder<T>where
    T: ViewType + ?Sized,{ /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#impl-BinViewChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::BinViewChunkedBuilder">BinViewChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/trait.ViewType.html" class="trait" title="trait polars_arrow::array::binview::ViewType">ViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#method.new" class="fn">new</a>(name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::BinViewChunkedBuilder">BinViewChunkedBuilder</a>\<T\>

Create a new BinViewChunkedBuilder

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#arguments" class="doc-anchor">§</a>Arguments

- `capacity` - Number of string elements in the final array.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#method.append_value" class="fn">append_value</a>\<S\>(&mut self, v: S)

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<T\>,

Appends a value of type `T` into the builder

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#method.append_null" class="fn">append_null</a>(&mut self)

Appends a null slot into the builder

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#method.append_option" class="fn">append_option</a>\<S\>(&mut self, opt: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<S\>)

where S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<T\>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#impl-BinViewChunkedBuilder%3Cstr%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::BinViewChunkedBuilder">BinViewChunkedBuilder</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#impl-BinViewChunkedBuilder%3C%5Bu8%5D%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::BinViewChunkedBuilder">BinViewChunkedBuilder</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#method.finish-1" class="fn">finish</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#impl-Clone-for-BinViewChunkedBuilder%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::BinViewChunkedBuilder">BinViewChunkedBuilder</a>\<T\>

where T: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/binview/trait.ViewType.html" class="trait" title="trait polars_arrow::array::binview::ViewType">ViewType</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::BinViewChunkedBuilder">BinViewChunkedBuilder</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html#blanket-implementations" class="anchor">§</a>
