# Struct ByteView Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/byte_view.rs.html#70" class="src">Source</a>

``` rust
#[repr(C)]pub struct ByteView {
    pub length: u32,
    pub prefix: u32,
    pub buffer_index: u32,
    pub offset: u32,
}
```

Expand description

Helper to access views of [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html) (`StringViewArray` and `BinaryViewArray`) where the length is greater than 12 bytes.

See Also:

- [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html) for more information on the layout of the views.
- [`validate_binary_view`](https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/fn.validate_binary_view.html "fn arrow_data::byte_view::validate_binary_view") and [`validate_string_view`](https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/fn.validate_string_view.html "fn arrow_data::byte_view::validate_string_view") to validate

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#example-create-a-new-u128-view" class="doc-anchor">§</a>Example: Create a new u128 view

``` rust
// Create a view for a string of length 20
// first four bytes are "Rust"
// stored in buffer 3
// at offset 42
let prefix = "Rust";
let view = ByteView::new(20, prefix.as_bytes())
  .with_buffer_index(3)
  .with_offset(42);

// create the final u128
let v = view.as_u128();
assert_eq!(v, 0x2a000000037473755200000014);
```

## <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#example-decode-a-u128-into-its-constituent-fields" class="doc-anchor">§</a>Example: decode a `u128` into its constituent fields

``` rust
// Convert a u128 to a ByteView
// See validate_{string,binary}_view functions to validate
let v = ByteView::from(0x2a000000037473755200000014);

assert_eq!(v.length, 20);
assert_eq!(v.prefix, 0x74737552);
assert_eq!(v.buffer_index, 3);
assert_eq!(v.offset, 42);
```

## Fields<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#fields" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#structfield.length" class="anchor field">§</a>`length: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

The length of the string/bytes.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#structfield.prefix" class="anchor field">§</a>`prefix: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

First 4 bytes of string/bytes data.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#structfield.buffer_index" class="anchor field">§</a>`buffer_index: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

The buffer index.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#structfield.offset" class="anchor field">§</a>`offset: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>

The offset into the buffer.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#impl-ByteView" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.new" class="fn">new</a>(length: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, prefix: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

Construct a [`ByteView`](https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html "struct arrow::array::ByteView") for data `length` of bytes with the specified prefix.

See example on [`ByteView`](https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html "struct arrow::array::ByteView") docs

Notes:

- the length should always be greater than [`MAX_INLINE_VIEW_LEN`](https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/arrow_data/byte_view/constant.MAX_INLINE_VIEW_LEN.html "constant arrow_data::byte_view::MAX_INLINE_VIEW_LEN") (Data less than 12 bytes is stored as an inline view)
- buffer and offset are set to `0`

##### <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#panics" class="doc-anchor">§</a>Panics

If the prefix is not exactly 4 bytes

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.with_buffer_index" class="fn">with_buffer_index</a>(self, buffer_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

Set the [`Self::buffer_index`](https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#structfield.buffer_index "field arrow::array::ByteView::buffer_index") field

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.with_offset" class="fn">with_offset</a>(self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

Set the [`Self::offset`](https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#structfield.offset "field arrow::array::ByteView::offset") field

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.as_u128" class="fn">as_u128</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>

Convert `ByteView` to `u128` by concatenating the fields

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#impl-Clone-for-ByteView" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#impl-Debug-for-ByteView" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#impl-Default-for-ByteView" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#impl-From%3Cu128%3E-for-ByteView" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>\> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u128.html" class="primitive">u128</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

Converts to this type from the input type.

<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#impl-Copy-for-ByteView" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html" class="struct" title="struct arrow::array::ByteView">ByteView</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/struct.ByteView.html#blanket-implementations" class="anchor">§</a>
