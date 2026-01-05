# Trait ToByteSlice Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/native.rs.html#268" class="src">Source</a>

``` rust
pub trait ToByteSlice {
    // Required method
    fn to_byte_slice(&self) -> &[u8] ⓘ;
}
```

Expand description

Allows conversion from supported Arrow types to a byte slice.

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#tymethod.to_byte_slice" class="fn">to_byte_slice</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Converts this instance into a byte slice

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#impl-ToByteSlice-for-%5BT%5D" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html" class="trait" title="trait arrow::datatypes::ToByteSlice">ToByteSlice</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#method.to_byte_slice" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#tymethod.to_byte_slice" class="fn">to_byte_slice</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

## Implementors<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html#impl-ToByteSlice-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ToByteSlice.html" class="trait" title="trait arrow::datatypes::ToByteSlice">ToByteSlice</a> for T

where T: <a href="https://docs.rs/arrow/latest/arrow/datatypes/trait.ArrowNativeType.html" class="trait" title="trait arrow::datatypes::ArrowNativeType">ArrowNativeType</a>,
