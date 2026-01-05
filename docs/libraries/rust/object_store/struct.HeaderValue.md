# Struct HeaderValue Copy item path

<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/src/http/header/value.rs.html#22" class="src">Source</a>

``` rust
pub struct HeaderValue { /* private fields */ }
```

Expand description

Represents an HTTP header field value.

In practice, HTTP header field values are usually valid ASCII. However, the HTTP spec allows for a header value to contain opaque bytes as well. In this case, the header field value is not able to be represented as a string.

To handle this, the `HeaderValue` is useable as a type and can be compared with strings and implements `Debug`. A `to_str` fn is provided that returns an `Err` if the header value contains non visible ascii characters.

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-HeaderValue" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

#### pub const fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from_static" class="fn">from_static</a>(src: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Convert a static string to a `HeaderValue`.

This function will not perform any copying, however the string is checked to ensure that no invalid characters are present. Only visible ASCII characters (32-127) are permitted.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#panics" class="doc-anchor">§</a>Panics

This function panics if the argument contains invalid header value characters.

Until [Allow panicking in constants](https://github.com/rust-lang/rfcs/pull/2345) makes its way into stable, the panic message at compile-time is going to look cryptic, but should at least point at your header value:

``` text
error: any use of this value will cause an error
  --> http/src/header/value.rs:67:17
   |
67 |                 ([] as [u8; 0])[0]; // Invalid header value
   |                 ^^^^^^^^^^^^^^^^^^
   |                 |
   |                 index out of bounds: the length is 0 but the index is 0
   |                 inside `HeaderValue::from_static` at http/src/header/value.rs:67:17
   |                 inside `INVALID_HEADER` at src/main.rs:73:33
   |
  ::: src/main.rs:73:1
   |
73 | const INVALID_HEADER: HeaderValue = HeaderValue::from_static("жsome value");
   | ----------------------------------------------------------------------------
```

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples" class="doc-anchor">§</a>Examples

``` rust
let val = HeaderValue::from_static("hello");
assert_eq!(val, "hello");
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from_str" class="fn">from_str</a>(src: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>\>

Attempt to convert a string to a `HeaderValue`.

If the argument contains invalid header value characters, an error is returned. Only visible ASCII characters (32-127) are permitted. Use `from_bytes` to create a `HeaderValue` that includes opaque octets (128-255).

This function is intended to be replaced in the future by a `TryFrom` implementation once the trait is stabilized in std.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
let val = HeaderValue::from_str("hello").unwrap();
assert_eq!(val, "hello");
```

An invalid value

``` rust
let val = HeaderValue::from_str("\n");
assert!(val.is_err());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from_name" class="fn">from_name</a>(name: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts a HeaderName into a HeaderValue

Since every valid HeaderName is a valid HeaderValue this is done infallibly.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
let val = HeaderValue::from_name(ACCEPT);
assert_eq!(val, HeaderValue::from_bytes(b"accept").unwrap());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from_bytes" class="fn">from_bytes</a>(src: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>\>

Attempt to convert a byte slice to a `HeaderValue`.

If the argument contains invalid header value bytes, an error is returned. Only byte values between 32 and 255 (inclusive) are permitted, excluding byte 127 (DEL).

This function is intended to be replaced in the future by a `TryFrom` implementation once the trait is stabilized in std.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
let val = HeaderValue::from_bytes(b"hello\xfa").unwrap();
assert_eq!(val, &b"hello\xfa"[..]);
```

An invalid value

``` rust
let val = HeaderValue::from_bytes(b"\n");
assert!(val.is_err());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from_maybe_shared" class="fn">from_maybe_shared</a>\<T\>(src: T) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> + 'static,

Attempt to convert a `Bytes` buffer to a `HeaderValue`.

This will try to prevent a copy if the type passed is the type used internally, and will copy the data if it is not.

#### pub unsafe fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from_maybe_shared_unchecked" class="fn">from_maybe_shared_unchecked</a>\<T\>(src: T) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> + 'static,

Convert a `Bytes` directly into a `HeaderValue` without validating.

This function does NOT validate that illegal bytes are not contained within the buffer.

###### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#panics-1" class="doc-anchor">§</a>Panics

In a debug build this will panic if `src` is not valid UTF-8.

###### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#safety" class="doc-anchor">§</a>Safety

`src` must contain valid UTF-8. In a release build it is undefined behaviour to call this with `src` that is not valid UTF-8.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.to_str" class="fn">to_str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.ToStrError.html" class="struct" title="struct http::header::value::ToStrError">ToStrError</a>\>

Yields a `&str` slice if the `HeaderValue` only contains visible ASCII chars.

This function will perform a scan of the header value, checking all the characters.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
let val = HeaderValue::from_static("hello");
assert_eq!(val.to_str().unwrap(), "hello");
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of `self`.

This length is in bytes.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-5" class="doc-anchor">§</a>Examples

``` rust
let val = HeaderValue::from_static("hello");
assert_eq!(val.len(), 5);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the `HeaderValue` has a length of zero bytes.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-6" class="doc-anchor">§</a>Examples

``` rust
let val = HeaderValue::from_static("");
assert!(val.is_empty());

let val = HeaderValue::from_static("hello");
assert!(!val.is_empty());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.as_bytes" class="fn">as_bytes</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Converts a `HeaderValue` to a byte slice.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-7" class="doc-anchor">§</a>Examples

``` rust
let val = HeaderValue::from_static("hello");
assert_eq!(val.as_bytes(), b"hello");
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.set_sensitive" class="fn">set_sensitive</a>(&mut self, val: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

Mark that the header value represents sensitive information.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-8" class="doc-anchor">§</a>Examples

``` rust
let mut val = HeaderValue::from_static("my secret");

val.set_sensitive(true);
assert!(val.is_sensitive());

val.set_sensitive(false);
assert!(!val.is_sensitive());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.is_sensitive" class="fn">is_sensitive</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the value represents sensitive data.

Sensitive data could represent passwords or other data that should not be stored on disk or in memory. By marking header values as sensitive, components using this crate can be instructed to treat them with special care for security reasons. For example, caches can avoid storing sensitive values, and HPACK encoders used by HTTP/2.0 implementations can choose not to compress them.

Additionally, sensitive values will be masked by the `Debug` implementation of `HeaderValue`.

Note that sensitivity is not factored into equality or ordering.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#examples-9" class="doc-anchor">§</a>Examples

``` rust
let mut val = HeaderValue::from_static("my secret");

val.set_sensitive(true);
assert!(val.is_sensitive());

val.set_sensitive(false);
assert!(!val.is_sensitive());
```

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-AsRef%3C%5Bu8%5D%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-Clone-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-Debug-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3C%26HeaderValue%3E-for-HeaderValue" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'a <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(t: &'a <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3CHeaderName%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(h: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3Ci16%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.i16.html" class="primitive">i16</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3Ci32%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3Ci64%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3Cisize%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.isize.html" class="primitive">isize</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3Cu16%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive">u16</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3Cu32%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3Cu64%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-From%3Cusize%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(num: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-FromStr-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.from_str-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, \<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-Hash-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-Ord-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3C%26T%3E-for-HeaderValue" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

where <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<T\>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3C%5Bu8%5D%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3CHeaderValue%3E-for-%26HeaderValue" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for &'a <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3CHeaderValue%3E-for-%26str" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3CHeaderValue%3E-for-%5Bu8%5D" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3CHeaderValue%3E-for-String" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3CHeaderValue%3E-for-str" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3CString%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq%3Cstr%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialEq-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3C%26T%3E-for-HeaderValue" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

where <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<T\>, T: ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3C%5Bu8%5D%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3CHeaderValue%3E-for-%26HeaderValue" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for &'a <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3CHeaderValue%3E-for-%26str" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3CHeaderValue%3E-for-%5Bu8%5D" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3CHeaderValue%3E-for-String" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3CHeaderValue%3E-for-str" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>\> for <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3CString%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd%3Cstr%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-PartialOrd-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-TryFrom%3C%26%5Bu8%5D%3E-for-HeaderValue" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#associatedtype.Error-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.try_from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( t: &'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, \<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&'a \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-TryFrom%3C%26String%3E-for-HeaderValue" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&'a <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.try_from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( s: &'a <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, \<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&'a <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-TryFrom%3C%26str%3E-for-HeaderValue" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( t: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, \<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-TryFrom%3CString%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#associatedtype.Error-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.try_from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( t: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, \<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-TryFrom%3CVec%3Cu8%3E%3E-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#associatedtype.Error-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/value/struct.InvalidHeaderValue.html" class="struct" title="struct http::header::value::InvalidHeaderValue">InvalidHeaderValue</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#method.try_from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( vec: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>, \<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#impl-Eq-for-HeaderValue" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html" class="struct" title="struct object_store::HeaderValue">HeaderValue</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderValue.html#blanket-implementations" class="anchor">§</a>
