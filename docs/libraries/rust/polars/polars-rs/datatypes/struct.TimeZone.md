# Struct TimeZone Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/temporal/time_zone.rs.html#9" class="src">Source</a>

``` rust
pub struct TimeZone { /* private fields */ }
```

## Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-TimeZone" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#associatedconstant.UTC" class="constant">UTC</a>: <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

#### pub const unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.from_static" class="fn">from_static</a>(tz: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

Construct from a static string.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#safety" class="doc-anchor">§</a>Safety

This does not perform any validation, the caller is responsible for ensuring they pass a valid timezone.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.new_unchecked" class="fn">new_unchecked</a>(zone_str: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#safety-1" class="doc-anchor">§</a>Safety

This does not perform any validation, the caller is responsible for ensuring they pass a valid timezone.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.opt_try_new" class="fn">opt_try_new</a>( zone_str: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Converts timezones to canonical form.

If the “timezones” feature is enabled, additionally performs validation and converts to Etc/GMT form where applicable.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.eq_none_as_utc" class="fn">eq_none_as_utc</a>(this: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>, other: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Equality where `None` is treated as UTC.

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method._canonical_timezone_impl" class="fn">_canonical_timezone_impl</a>(tz: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.from_chrono" class="fn">from_chrono</a>(tz: &<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_chrono" class="fn">to_chrono</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html" class="enum" title="enum chrono_tz::timezones::Tz">Tz</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.validate_time_zone" class="fn">validate_time_zone</a>(tz: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Methods from <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\<Target = <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\><a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#deref-methods-PlSmallStr" class="anchor">§</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#associatedconstant.EMPTY" class="constant">EMPTY</a>: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

#### pub const <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#associatedconstant.EMPTY_REF" class="constant">EMPTY_REF</a>: &'static <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.as_str" class="fn">as_str</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_string" class="fn">to_string</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

## Methods from <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\<Target = <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\><a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#deref-methods-str" class="anchor">§</a>

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#141" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the length of `self`.

This length is in bytes, not [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s or graphemes. In other words, it might not be what a human considers the length of the string.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples" class="doc-anchor">§</a>Examples

``` rust
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#161" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if `self` has a length of zero bytes.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

1.9.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#361" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.is_char_boundary" class="fn">is_char_boundary</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks that `index`-th byte is the first byte in a UTF-8 code point sequence or the end of the string.

The start and end of the string (when `index == self.len()`) are considered to be boundaries.

Returns `false` if `index` is greater than `self.len()`.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
```

1.92.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#410" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.floor_char_boundary" class="fn">floor_char_boundary</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Finds the closest `x` not exceeding `index` where [`is_char_boundary(x)`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") is `true`.

This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t exceed a given number of bytes. Note that this is done purely at the character level and can still visually split graphemes, even though the underlying characters aren’t split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only includes 🧑 (person) instead.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.floor_char_boundary(13);
assert_eq!(closest, 10);
assert_eq!(&s[..closest], "❤️🧡");
```

1.92.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#453" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.ceil_char_boundary" class="fn">ceil_char_boundary</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Finds the closest `x` not below `index` where [`is_char_boundary(x)`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") is `true`.

If `index` is greater than the length of the string, this returns the length of the string.

This method is the natural complement to [`floor_char_boundary`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.floor_char_boundary "method str::floor_char_boundary"). See that method for more details.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.ceil_char_boundary(13);
assert_eq!(closest, 14);
assert_eq!(&s[..closest], "❤️🧡💛");
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#486" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.as_bytes" class="fn">as_bytes</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#" class="tooltip" data-notable-ty="&amp;[u8]">ⓘ</a>

Converts a string slice to a byte slice. To convert the byte slice back into a string slice, use the [`from_utf8`](https://doc.rust-lang.org/nightly/core/str/converts/fn.from_utf8.html "fn core::str::converts::from_utf8") function.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-5" class="doc-anchor">§</a>Examples

``` rust
let bytes = "bors".as_bytes();
assert_eq!(b"bors", bytes);
```

1.20.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#531" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.as_bytes_mut" class="fn">as_bytes_mut</a>(&mut self) -\> &mut \[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\] <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#" class="tooltip" data-notable-ty="&amp;mut [u8]">ⓘ</a>

Converts a mutable string slice to a mutable byte slice.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#safety-2" class="doc-anchor">§</a>Safety

The caller must ensure that the content of the slice is valid UTF-8 before the borrow ends and the underlying `str` is used.

Use of a `str` whose contents are not valid UTF-8 is undefined behavior.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-6" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let mut s = String::from("Hello");
let bytes = unsafe { s.as_bytes_mut() };

assert_eq!(b"Hello", bytes);
```

Mutability:

``` rust
let mut s = String::from("🗻∈🌏");

unsafe {
    let bytes = s.as_bytes_mut();

    bytes[0] = 0xF0;
    bytes[1] = 0x9F;
    bytes[2] = 0x8D;
    bytes[3] = 0x94;
}

assert_eq!("🍔∈🌏", s);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#562" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.as_ptr" class="fn">as_ptr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const</a> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Converts a string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8"). This pointer will be pointing to the first byte of the string slice.

The caller must ensure that the returned pointer is never written to. If you need to mutate the contents of the string slice, use [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.as_mut_ptr "method str::as_mut_ptr").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-7" class="doc-anchor">§</a>Examples

``` rust
let s = "Hello";
let ptr = s.as_ptr();
```

1.36.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#580" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.as_mut_ptr" class="fn">as_mut_ptr</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*mut</a> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Converts a mutable string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8"). This pointer will be pointing to the first byte of the string slice.

It is your responsibility to make sure that the string slice only gets modified in a way that it remains valid UTF-8.

1.20.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#606" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.get" class="fn">get</a>\<I\>(&self, i: I) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Returns a subslice of `str`.

This is the non-panicking alternative to indexing the `str`. Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") whenever equivalent indexing operation would panic.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-8" class="doc-anchor">§</a>Examples

``` rust
let v = String::from("🗻∈🌏");

assert_eq!(Some("🗻"), v.get(0..4));

// indices not on UTF-8 sequence boundaries
assert!(v.get(1..).is_none());
assert!(v.get(..8).is_none());

// out of bounds
assert!(v.get(..42).is_none());
```

1.20.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#639" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.get_mut" class="fn">get_mut</a>\<I\>( &mut self, i: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&mut \<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Returns a mutable subslice of `str`.

This is the non-panicking alternative to indexing the `str`. Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") whenever equivalent indexing operation would panic.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-9" class="doc-anchor">§</a>Examples

``` rust
let mut v = String::from("hello");
// correct length
assert!(v.get_mut(0..5).is_some());
// out of bounds
assert!(v.get_mut(..42).is_none());
assert_eq!(Some("he"), v.get_mut(0..2).map(|v| &*v));

assert_eq!("hello", v);
{
    let s = v.get_mut(0..2);
    let s = s.map(|s| {
        s.make_ascii_uppercase();
        &*s
    });
    assert_eq!(Some("HE"), s);
}
assert_eq!("HEllo", v);
```

1.20.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#671" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.get_unchecked" class="fn">get_unchecked</a>\<I\>(&self, i: I) -\> &\<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Returns an unchecked subslice of `str`.

This is the unchecked alternative to indexing the `str`.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#safety-3" class="doc-anchor">§</a>Safety

Callers of this function are responsible that these preconditions are satisfied:

- The starting index must not exceed the ending index;
- Indexes must be within bounds of the original slice;
- Indexes must lie on UTF-8 sequence boundaries.

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the `str` type.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-10" class="doc-anchor">§</a>Examples

``` rust
let v = "🗻∈🌏";
unsafe {
    assert_eq!("🗻", v.get_unchecked(0..4));
    assert_eq!("∈", v.get_unchecked(4..7));
    assert_eq!("🌏", v.get_unchecked(7..11));
}
```

1.20.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#706" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.get_unchecked_mut" class="fn">get_unchecked_mut</a>\<I\>( &mut self, i: I, ) -\> &mut \<I as <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output" class="associatedtype" title="type core::slice::index::SliceIndex::Output">Output</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html" class="trait" title="trait core::slice::index::SliceIndex">SliceIndex</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Returns a mutable, unchecked subslice of `str`.

This is the unchecked alternative to indexing the `str`.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#safety-4" class="doc-anchor">§</a>Safety

Callers of this function are responsible that these preconditions are satisfied:

- The starting index must not exceed the ending index;
- Indexes must be within bounds of the original slice;
- Indexes must lie on UTF-8 sequence boundaries.

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the `str` type.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-11" class="doc-anchor">§</a>Examples

``` rust
let mut v = String::from("🗻∈🌏");
unsafe {
    assert_eq!("🗻", v.get_unchecked_mut(0..4));
    assert_eq!("∈", v.get_unchecked_mut(4..7));
    assert_eq!("🌏", v.get_unchecked_mut(7..11));
}
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#757" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.slice_unchecked" class="fn">slice_unchecked</a>(&self, begin: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, end: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.29.0: use `get_unchecked(begin..end)` instead

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "primitive str") and [`Index`](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index").

This new slice goes from `begin` to `end`, including `begin` but excluding `end`.

To get a mutable string slice instead, see the [`slice_mut_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.slice_mut_unchecked "method str::slice_mut_unchecked") method.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#safety-5" class="doc-anchor">§</a>Safety

Callers of this function are responsible that three preconditions are satisfied:

- `begin` must not exceed `end`.
- `begin` and `end` must be byte positions within the string slice.
- `begin` and `end` must lie on UTF-8 sequence boundaries.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-12" class="doc-anchor">§</a>Examples

``` rust
let s = "Löwe 老虎 Léopard";

unsafe {
    assert_eq!("Löwe 老虎 Léopard", s.slice_unchecked(0, 21));
}

let s = "Hello, world!";

unsafe {
    assert_eq!("world", s.slice_unchecked(7, 12));
}
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#791" class="src">Source</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.slice_mut_unchecked" class="fn">slice_mut_unchecked</a>( &mut self, begin: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, end: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.29.0: use `get_unchecked_mut(begin..end)` instead

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "primitive str") and [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut").

This new slice goes from `begin` to `end`, including `begin` but excluding `end`.

To get an immutable string slice instead, see the [`slice_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.slice_unchecked "method str::slice_unchecked") method.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#safety-6" class="doc-anchor">§</a>Safety

Callers of this function are responsible that three preconditions are satisfied:

- `begin` must not exceed `end`.
- `begin` and `end` must be byte positions within the string slice.
- `begin` and `end` must lie on UTF-8 sequence boundaries.

1.4.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#831" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_at" class="fn">split_at</a>(&self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Divides one string slice into two at an index.

The argument, `mid`, should be a byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut "method str::split_at_mut") method.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#panics" class="doc-anchor">§</a>Panics

Panics if `mid` is not on a UTF-8 code point boundary, or if it is past the end of the last code point of the string slice. For a non-panicking alternative see [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_checked "method str::split_at_checked").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-13" class="doc-anchor">§</a>Examples

``` rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at(3);

assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);
```

1.4.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#872" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_at_mut" class="fn">split_at_mut</a>(&mut self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Divides one mutable string slice into two at an index.

The argument, `mid`, should be a byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get immutable string slices instead, see the [`split_at`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at "method str::split_at") method.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#panics-1" class="doc-anchor">§</a>Panics

Panics if `mid` is not on a UTF-8 code point boundary, or if it is past the end of the last code point of the string slice. For a non-panicking alternative see [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut_checked "method str::split_at_mut_checked").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-14" class="doc-anchor">§</a>Examples

``` rust
let mut s = "Per Martin-Löf".to_string();
{
    let (first, last) = s.split_at_mut(3);
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#912" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_at_checked" class="fn">split_at_checked</a>(&self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)\>

Divides one string slice into two at an index.

The argument, `mid`, should be a valid byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point. The method returns `None` if that’s not the case.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut_checked "method str::split_at_mut_checked") method.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-15" class="doc-anchor">§</a>Examples

``` rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at_checked(3).unwrap();
assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);

assert_eq!(None, s.split_at_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_checked(16));  // Beyond the string length
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#953" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_at_mut_checked" class="fn">split_at_mut_checked</a>( &mut self, mid: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&mut <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, &mut <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)\>

Divides one mutable string slice into two at an index.

The argument, `mid`, should be a valid byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point. The method returns `None` if that’s not the case.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get immutable string slices instead, see the [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_checked "method str::split_at_checked") method.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-16" class="doc-anchor">§</a>Examples

``` rust
let mut s = "Per Martin-Löf".to_string();
if let Some((first, last)) = s.split_at_mut_checked(3) {
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);

assert_eq!(None, s.split_at_mut_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_mut_checked(16));  // Beyond the string length
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1050" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.chars" class="fn">chars</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html" class="struct" title="struct core::str::iter::Chars">Chars</a>\<'\_\>

Returns an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of a string slice.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns such an iterator.

It’s important to remember that [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") represents a Unicode Scalar Value, and might not match your idea of what a ‘character’ is. Iteration over grapheme clusters may be what you actually want. This functionality is not provided by Rust’s standard library, check crates.io instead.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-17" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let word = "goodbye";

let count = word.chars().count();
assert_eq!(7, count);

let mut chars = word.chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

``` rust
let y = "y̆";

let mut chars = y.chars();

assert_eq!(Some('y'), chars.next()); // not 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1107" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.char_indices" class="fn">char_indices</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html" class="struct" title="struct core::str::iter::CharIndices">CharIndices</a>\<'\_\>

Returns an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of a string slice, and their positions.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns an iterator of both these [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, as well as their byte positions.

The iterator yields tuples. The position is first, the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") is second.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-18" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let word = "goodbye";

let count = word.char_indices().count();
assert_eq!(7, count);

let mut char_indices = word.char_indices();

assert_eq!(Some((0, 'g')), char_indices.next());
assert_eq!(Some((1, 'o')), char_indices.next());
assert_eq!(Some((2, 'o')), char_indices.next());
assert_eq!(Some((3, 'd')), char_indices.next());
assert_eq!(Some((4, 'b')), char_indices.next());
assert_eq!(Some((5, 'y')), char_indices.next());
assert_eq!(Some((6, 'e')), char_indices.next());

assert_eq!(None, char_indices.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

``` rust
let yes = "y̆es";

let mut char_indices = yes.char_indices();

assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
assert_eq!(Some((1, '\u{0306}')), char_indices.next());

// note the 3 here - the previous character took up two bytes
assert_eq!(Some((3, 'e')), char_indices.next());
assert_eq!(Some((4, 's')), char_indices.next());

assert_eq!(None, char_indices.next());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1130" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.bytes" class="fn">bytes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html" class="struct" title="struct core::str::iter::Bytes">Bytes</a>\<'\_\>

Returns an iterator over the bytes of a string slice.

As a string slice consists of a sequence of bytes, we can iterate through a string slice by byte. This method returns such an iterator.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-19" class="doc-anchor">§</a>Examples

``` rust
let mut bytes = "bors".bytes();

assert_eq!(Some(b'b'), bytes.next());
assert_eq!(Some(b'o'), bytes.next());
assert_eq!(Some(b'r'), bytes.next());
assert_eq!(Some(b's'), bytes.next());

assert_eq!(None, bytes.next());
```

1.1.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1182" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_whitespace" class="fn">split_whitespace</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html" class="struct" title="struct core::str::iter::SplitWhitespace">SplitWhitespace</a>\<'\_\>

Splits a string slice by whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of whitespace.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`. If you only want to split on ASCII whitespace instead, use [`split_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_ascii_whitespace "method str::split_ascii_whitespace").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-20" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let mut iter = "A few words".split_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

All kinds of whitespace are considered:

``` rust
let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all whitespace, the iterator yields no string slices:

``` rust
assert_eq!("".split_whitespace().next(), None);
assert_eq!("   ".split_whitespace().next(), None);
```

1.34.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1233" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_ascii_whitespace" class="fn">split_ascii_whitespace</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html" class="struct" title="struct core::str::iter::SplitAsciiWhitespace">SplitAsciiWhitespace</a>\<'\_\>

Splits a string slice by ASCII whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of ASCII whitespace.

This uses the same definition as [`char::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace "method char::is_ascii_whitespace"). To split by Unicode `Whitespace` instead, use [`split_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace "method str::split_whitespace").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-21" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let mut iter = "A few words".split_ascii_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

Various kinds of ASCII whitespace are considered (see [`char::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace "method char::is_ascii_whitespace")):

``` rust
let mut iter = " Mary   had\ta little  \n\t lamb".split_ascii_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all ASCII whitespace, the iterator yields no string slices:

``` rust
assert_eq!("".split_ascii_whitespace().next(), None);
assert_eq!("   ".split_ascii_whitespace().next(), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1286" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.lines" class="fn">lines</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html" class="struct" title="struct core::str::iter::Lines">Lines</a>\<'\_\>

Returns an iterator over the lines of a string, as string slices.

Lines are split at line endings that are either newlines (`\n`) or sequences of a carriage return followed by a line feed (`\r\n`).

Line terminators are not included in the lines returned by the iterator.

Note that any carriage return (`\r`) not immediately followed by a line feed (`\n`) does not split a line. These carriage returns are thereby included in the produced lines.

The final line ending is optional. A string that ends with a final line ending will return the same lines as an otherwise identical string without a final line ending.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-22" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let text = "foo\r\nbar\n\nbaz\r";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
// Trailing carriage return is included in the last line
assert_eq!(Some("baz\r"), lines.next());

assert_eq!(None, lines.next());
```

The final line does not require any ending:

``` rust
let text = "foo\nbar\n\r\nbaz";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
assert_eq!(Some("baz"), lines.next());

assert_eq!(None, lines.next());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1295" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.lines_any" class="fn">lines_any</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html" class="struct" title="struct core::str::iter::LinesAny">LinesAny</a>\<'\_\>

👎Deprecated since 1.4.0: use lines() instead now

Returns an iterator over the lines of a string.

1.8.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1315" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.encode_utf16" class="fn">encode_utf16</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html" class="struct" title="struct core::str::iter::EncodeUtf16">EncodeUtf16</a>\<'\_\>

Returns an iterator of `u16` over the string encoded as native endian UTF-16 (without byte-order mark).

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-23" class="doc-anchor">§</a>Examples

``` rust
let text = "Zażółć gęślą jaźń";

let utf8_len = text.len();
let utf16_len = text.encode_utf16().count();

assert!(utf16_len <= utf8_len);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1340" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.contains" class="fn">contains</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns `true` if the given pattern matches a sub-slice of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-24" class="doc-anchor">§</a>Examples

``` rust
let bananas = "bananas";

assert!(bananas.contains("nana"));
assert!(!bananas.contains("apples"));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1378" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.starts_with" class="fn">starts_with</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns `true` if the given pattern matches a prefix of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, in which case this function will return true if the `&str` is a prefix of this string slice.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can also be a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches. These will only be checked against the first character of this string slice. Look at the second example below regarding behavior for slices of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-25" class="doc-anchor">§</a>Examples

``` rust
let bananas = "bananas";

assert!(bananas.starts_with("bana"));
assert!(!bananas.starts_with("nana"));
```

``` rust
let bananas = "bananas";

// Note that both of these assert successfully.
assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1403-1405" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.ends_with" class="fn">ends_with</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns `true` if the given pattern matches a suffix of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-26" class="doc-anchor">§</a>Examples

``` rust
let bananas = "bananas";

assert!(bananas.ends_with("anas"));
assert!(!bananas.ends_with("nana"));
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1454" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.find" class="fn">find</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns the byte index of the first character of this string slice that matches the pattern.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the pattern doesn’t match.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-27" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.find('L'), Some(0));
assert_eq!(s.find('é'), Some(14));
assert_eq!(s.find("pard"), Some(17));
```

More complex patterns using point-free style and closures:

``` rust
let s = "Löwe 老虎 Léopard";

assert_eq!(s.find(char::is_whitespace), Some(5));
assert_eq!(s.find(char::is_lowercase), Some(1));
assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

Not finding the pattern:

``` rust
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.find(x), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1500-1502" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.rfind" class="fn">rfind</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns the byte index for the first character of the last match of the pattern in this string slice.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the pattern doesn’t match.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-28" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.rfind('L'), Some(13));
assert_eq!(s.rfind('é'), Some(14));
assert_eq!(s.rfind("pard"), Some(24));
```

More complex patterns with closures:

``` rust
let s = "Löwe 老虎 Léopard";

assert_eq!(s.rfind(char::is_whitespace), Some(12));
assert_eq!(s.rfind(char::is_lowercase), Some(20));
```

Not finding the pattern:

``` rust
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.rfind(x), None);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1628" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split" class="fn">split</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html" class="struct" title="struct core::str::iter::Split">Split</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

If there are no matches the full string slice is returned as the only item in the iterator.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior" class="doc-anchor">§</a>Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit "method str::rsplit") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-29" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let v: Vec<&str> = "".split('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
assert_eq!(v, ["lion", "", "tiger", "leopard"]);

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

let v: Vec<&str> = "AABBCC".split("DD").collect();
assert_eq!(v, ["AABBCC"]);

let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
assert_eq!(v, ["abc", "def", "ghi"]);

let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);
```

If the pattern is a slice of chars, split on each occurrence of any of the characters:

``` rust
let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
assert_eq!(v, ["2020", "11", "03", "23", "59"]);
```

A more complex pattern, using a closure:

``` rust
let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "def", "ghi"]);
```

If a string contains multiple contiguous separators, you will end up with empty strings in the output:

``` rust
let x = "||||a||b|c".to_string();
let d: Vec<_> = x.split('|').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

Contiguous separators are separated by the empty string.

``` rust
let x = "(///)".to_string();
let d: Vec<_> = x.split('/').collect();

assert_eq!(d, &["(", "", "", ")"]);
```

Separators at the start or end of a string are neighbored by empty strings.

``` rust
let d: Vec<_> = "010".split("0").collect();
assert_eq!(d, &["", "1", ""]);
```

When the empty string is used as a separator, it separates every character in the string, along with the beginning and end of the string.

``` rust
let f: Vec<_> = "rust".split("").collect();
assert_eq!(f, &["", "r", "u", "s", "t", ""]);
```

Contiguous separators can lead to possibly surprising behavior when whitespace is used as the separator. This code is correct:

``` rust
let x = "    a  b c".to_string();
let d: Vec<_> = x.split(' ').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

It does *not* give you:

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
assert_eq!(d, &["a", "b", "c"]);
```

Use [`split_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace "method str::split_whitespace") for this behavior.

1.51.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1669" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_inclusive" class="fn">split_inclusive</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html" class="struct" title="struct core::str::iter::SplitInclusive">SplitInclusive</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

Differs from the iterator produced by `split` in that `split_inclusive` leaves the matched part as the terminator of the substring.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-30" class="doc-anchor">§</a>Examples

``` rust
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
```

If the last element of the string is matched, that element will be considered the terminator of the preceding substring. That substring will be the last item returned by the iterator.

``` rust
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb.\n"]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1724-1726" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.rsplit" class="fn">rsplit</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html" class="struct" title="struct core::str::iter::RSplit">RSplit</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern and yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-1" class="doc-anchor">§</a>Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-31" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
let v: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
assert_eq!(v, ["lamb", "little", "a", "had", "Mary"]);

let v: Vec<&str> = "".rsplit('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplit('X').collect();
assert_eq!(v, ["leopard", "tiger", "", "lion"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplit("::").collect();
assert_eq!(v, ["leopard", "tiger", "lion"]);
```

A more complex pattern, using a closure:

``` rust
let v: Vec<&str> = "abc1defXghi".rsplit(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "def", "abc"]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1773" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_terminator" class="fn">split_terminator</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html" class="struct" title="struct core::str::iter::SplitTerminator">SplitTerminator</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

Equivalent to [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split"), except that the trailing substring is skipped if empty.

This method can be used for string data that is *terminated*, rather than *separated* by a pattern.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-2" class="doc-anchor">§</a>Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit_terminator`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit_terminator "method str::rsplit_terminator") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-32" class="doc-anchor">§</a>Examples

``` rust
let v: Vec<&str> = "A.B.".split_terminator('.').collect();
assert_eq!(v, ["A", "B"]);

let v: Vec<&str> = "A..B..".split_terminator(".").collect();
assert_eq!(v, ["A", "", "B", ""]);

let v: Vec<&str> = "A.B:C.D".split_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["A", "B", "C", "D"]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1819-1821" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.rsplit_terminator" class="fn">rsplit_terminator</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html" class="struct" title="struct core::str::iter::RSplitTerminator">RSplitTerminator</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns an iterator over substrings of `self`, separated by characters matched by a pattern and yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

Equivalent to [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split"), except that the trailing substring is skipped if empty.

This method can be used for string data that is *terminated*, rather than *separated* by a pattern.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-3" class="doc-anchor">§</a>Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be double ended if a forward/reverse search yields the same elements.

For iterating from the front, the [`split_terminator`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_terminator "method str::split_terminator") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-33" class="doc-anchor">§</a>Examples

``` rust
let v: Vec<&str> = "A.B.".rsplit_terminator('.').collect();
assert_eq!(v, ["B", "A"]);

let v: Vec<&str> = "A..B..".rsplit_terminator(".").collect();
assert_eq!(v, ["", "B", "", "A"]);

let v: Vec<&str> = "A.B:C.D".rsplit_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["D", "C", "B", "A"]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1874" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.splitn" class="fn">splitn</a>\<P\>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html" class="struct" title="struct core::str::iter::SplitN">SplitN</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns an iterator over substrings of the given string slice, separated by a pattern, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-4" class="doc-anchor">§</a>Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

If the pattern allows a reverse search, the [`rsplitn`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplitn "method str::rsplitn") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-34" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
assert_eq!(v, ["Mary", "had", "a little lambda"]);

let v: Vec<&str> = "lionXXtigerXleopard".splitn(3, "X").collect();
assert_eq!(v, ["lion", "", "tigerXleopard"]);

let v: Vec<&str> = "abcXdef".splitn(1, 'X').collect();
assert_eq!(v, ["abcXdef"]);

let v: Vec<&str> = "".splitn(1, 'X').collect();
assert_eq!(v, [""]);
```

A more complex pattern, using a closure:

``` rust
let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "defXghi"]);
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1923-1925" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.rsplitn" class="fn">rsplitn</a>\<P\>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html" class="struct" title="struct core::str::iter::RSplitN">RSplitN</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns an iterator over substrings of this string slice, separated by a pattern, starting from the end of the string, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-5" class="doc-anchor">§</a>Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

For splitting from the front, the [`splitn`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.splitn "method str::splitn") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-35" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
let v: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
assert_eq!(v, ["lamb", "little", "Mary had a"]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(3, 'X').collect();
assert_eq!(v, ["leopard", "tiger", "lionX"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplitn(2, "::").collect();
assert_eq!(v, ["leopard", "lion::tiger"]);
```

A more complex pattern, using a closure:

``` rust
let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "abc1def"]);
```

1.52.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1943" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.split_once" class="fn">split_once</a>\<P\>(&self, delimiter: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Splits the string on the first occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-36" class="doc-anchor">§</a>Examples

``` rust
assert_eq!("cfg".split_once('='), None);
assert_eq!("cfg=".split_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".split_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".split_once('='), Some(("cfg", "foo=bar")));
```

1.52.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1961-1963" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.rsplit_once" class="fn">rsplit_once</a>\<P\>(&self, delimiter: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Splits the string on the last occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-37" class="doc-anchor">§</a>Examples

``` rust
assert_eq!("cfg".rsplit_once('='), None);
assert_eq!("cfg=foo".rsplit_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".rsplit_once('='), Some(("cfg=foo", "bar")));
```

1.2.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2001" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.matches" class="fn">matches</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html" class="struct" title="struct core::str::iter::Matches">Matches</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns an iterator over the disjoint matches of a pattern within the given string slice.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-6" class="doc-anchor">§</a>Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatches "method str::rmatches") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-38" class="doc-anchor">§</a>Examples

``` rust
let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
assert_eq!(v, ["1", "2", "3"]);
```

1.2.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2035-2037" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.rmatches" class="fn">rmatches</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html" class="struct" title="struct core::str::iter::RMatches">RMatches</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns an iterator over the disjoint matches of a pattern within this string slice, yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-7" class="doc-anchor">§</a>Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.matches "method str::matches") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-39" class="doc-anchor">§</a>Examples

``` rust
let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
assert_eq!(v, ["3", "2", "1"]);
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2079" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.match_indices" class="fn">match_indices</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html" class="struct" title="struct core::str::iter::MatchIndices">MatchIndices</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns an iterator over the disjoint matches of a pattern within this string slice as well as the index that the match starts at.

For matches of `pat` within `self` that overlap, only the indices corresponding to the first match are returned.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-8" class="doc-anchor">§</a>Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatch_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatch_indices "method str::rmatch_indices") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-40" class="doc-anchor">§</a>Examples

``` rust
let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);

let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
assert_eq!(v, [(1, "abc"), (4, "abc")]);

let v: Vec<_> = "ababa".match_indices("aba").collect();
assert_eq!(v, [(0, "aba")]); // only the first `aba`
```

1.5.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2119-2121" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.rmatch_indices" class="fn">rmatch_indices</a>\<P\>(&self, pat: P) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html" class="struct" title="struct core::str::iter::RMatchIndices">RMatchIndices</a>\<'\_, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns an iterator over the disjoint matches of a pattern within `self`, yielded in reverse order along with the index of the match.

For matches of `pat` within `self` that overlap, only the indices corresponding to the last match are returned.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#iterator-behavior-9" class="doc-anchor">§</a>Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`match_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.match_indices "method str::match_indices") method can be used.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-41" class="doc-anchor">§</a>Examples

``` rust
let v: Vec<_> = "abcXXXabcYYYabc".rmatch_indices("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);

let v: Vec<_> = "1abcabc2".rmatch_indices("abc").collect();
assert_eq!(v, [(4, "abc"), (1, "abc")]);

let v: Vec<_> = "ababa".rmatch_indices("aba").collect();
assert_eq!(v, [(2, "aba")]); // only the last `aba`
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2143" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim" class="fn">trim</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns a string slice with leading and trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-42" class="doc-anchor">§</a>Examples

``` rust
let s = "\n Hello\tworld\t\n";

assert_eq!("Hello\tworld", s.trim());
```

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2182" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_start" class="fn">trim_start</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#text-directionality" class="doc-anchor">§</a>Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-43" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let s = "\n Hello\tworld\t\n";
assert_eq!("Hello\tworld\t\n", s.trim_start());
```

Directionality:

``` rust
let s = "  English  ";
assert!(Some('E') == s.trim_start().chars().next());

let s = "  עברית  ";
assert!(Some('ע') == s.trim_start().chars().next());
```

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2221" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_end" class="fn">trim_end</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#text-directionality-1" class="doc-anchor">§</a>Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-44" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let s = "\n Hello\tworld\t\n";
assert_eq!("\n Hello\tworld", s.trim_end());
```

Directionality:

``` rust
let s = "  English  ";
assert!(Some('h') == s.trim_end().chars().rev().next());

let s = "  עברית  ";
assert!(Some('ת') == s.trim_end().chars().rev().next());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2261" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_left" class="fn">trim_left</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.33.0: superseded by `trim_start`

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#text-directionality-2" class="doc-anchor">§</a>Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the *right* side, not the left.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-45" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let s = " Hello\tworld\t";

assert_eq!("Hello\tworld\t", s.trim_left());
```

Directionality:

``` rust
let s = "  English";
assert!(Some('E') == s.trim_left().chars().next());

let s = "  עברית";
assert!(Some('ע') == s.trim_left().chars().next());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2301" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_right" class="fn">trim_right</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.33.0: superseded by `trim_end`

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#text-directionality-3" class="doc-anchor">§</a>Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the *left* side, not the right.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-46" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let s = " Hello\tworld\t";

assert_eq!(" Hello\tworld", s.trim_right());
```

Directionality:

``` rust
let s = "English  ";
assert!(Some('h') == s.trim_right().chars().rev().next());

let s = "עברית  ";
assert!(Some('ת') == s.trim_right().chars().rev().next());
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2334-2336" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_matches" class="fn">trim_matches</a>\<P\>(&self, pat: P) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.DoubleEndedSearcher.html" class="trait" title="trait core::str::pattern::DoubleEndedSearcher">DoubleEndedSearcher</a>\<'a\>,

Returns a string slice with all prefixes and suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-47" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
```

A more complex pattern, using a closure:

``` rust
assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");
```

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2381" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_start_matches" class="fn">trim_start_matches</a>\<P\>(&self, pat: P) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#text-directionality-4" class="doc-anchor">§</a>Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-48" class="doc-anchor">§</a>Examples

``` rust
assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
```

1.45.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2415" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.strip_prefix" class="fn">strip_prefix</a>\<P\>(&self, prefix: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Returns a string slice with the prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix, wrapped in `Some`. Unlike [`trim_start_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start_matches "method str::trim_start_matches"), this method removes the prefix exactly once.

If the string does not start with `prefix`, returns `None`.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-49" class="doc-anchor">§</a>Examples

``` rust
assert_eq!("foo:bar".strip_prefix("foo:"), Some("bar"));
assert_eq!("foo:bar".strip_prefix("bar"), None);
assert_eq!("foofoo".strip_prefix("foo"), Some("foo"));
```

1.45.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2443-2445" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.strip_suffix" class="fn">strip_suffix</a>\<P\>(&self, suffix: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns a string slice with the suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix, wrapped in `Some`. Unlike [`trim_end_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end_matches "method str::trim_end_matches"), this method removes the suffix exactly once.

If the string does not end with `suffix`, returns `None`.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-50" class="doc-anchor">§</a>Examples

``` rust
assert_eq!("bar:foo".strip_suffix(":foo"), Some("bar"));
assert_eq!("bar:foo".strip_suffix("bar"), None);
assert_eq!("foofoo".strip_suffix("foo"), Some("foo"));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_prefix" class="fn">trim_prefix</a>\<P\>(&self, prefix: P) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix. Unlike [`strip_prefix`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_prefix "method str::strip_prefix"), this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

If the string does not start with `prefix`, returns the original string unchanged.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-51" class="doc-anchor">§</a>Examples

``` rust
#![feature(trim_prefix_suffix)]

// Prefix present - removes it
assert_eq!("foo:bar".trim_prefix("foo:"), "bar");
assert_eq!("foofoo".trim_prefix("foo"), "foo");

// Prefix absent - returns original string
assert_eq!("foo:bar".trim_prefix("bar"), "foo:bar");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_suffix" class="fn">trim_suffix</a>\<P\>(&self, suffix: P) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix. Unlike [`strip_suffix`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_suffix "method str::strip_suffix"), this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

If the string does not end with `suffix`, returns the original string unchanged.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-52" class="doc-anchor">§</a>Examples

``` rust
#![feature(trim_prefix_suffix)]

// Suffix present - removes it
assert_eq!("bar:foo".trim_suffix(":foo"), "bar");
assert_eq!("foofoo".trim_suffix("foo"), "foo");

// Suffix absent - returns original string
assert_eq!("bar:foo".trim_suffix("bar"), "bar:foo");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

1.30.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2563-2565" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_end_matches" class="fn">trim_end_matches</a>\<P\>(&self, pat: P) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#text-directionality-5" class="doc-anchor">§</a>Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-53" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

``` rust
assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2607" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_left_matches" class="fn">trim_left_matches</a>\<P\>(&self, pat: P) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

👎Deprecated since 1.33.0: superseded by `trim_start_matches`

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#text-directionality-6" class="doc-anchor">§</a>Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the *right* side, not the left.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-54" class="doc-anchor">§</a>Examples

``` rust
assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_left_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2650-2652" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_right_matches" class="fn">trim_right_matches</a>\<P\>(&self, pat: P) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>, \<P as <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher" class="associatedtype" title="type core::str::pattern::Pattern::Searcher">Searcher</a>\<'a\>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html" class="trait" title="trait core::str::pattern::ReverseSearcher">ReverseSearcher</a>\<'a\>,

👎Deprecated since 1.33.0: superseded by `trim_end_matches`

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#text-directionality-7" class="doc-anchor">§</a>Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the *left* side, not the right.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-55" class="doc-anchor">§</a>Examples

Simple patterns:

``` rust
assert_eq!("11foo1bar11".trim_right_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_right_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_right_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

``` rust
assert_eq!("1fooX".trim_right_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2701" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.parse" class="fn">parse</a>\<F\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<F, \<F as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>,

Parses this string slice into another type.

Because `parse` is so general, it can cause problems with type inference. As such, `parse` is one of the few times you’ll see the syntax affectionately known as the ‘turbofish’: `::<>`. This helps the inference algorithm understand specifically which type you’re trying to parse into.

`parse` can parse into any type that implements the [`FromStr`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") trait.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#errors" class="doc-anchor">§</a>Errors

Will return [`Err`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "associated type core::str::traits::FromStr::Err") if it’s not possible to parse this string slice into the desired type.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-56" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let four: u32 = "4".parse().unwrap();

assert_eq!(4, four);
```

Using the ‘turbofish’ instead of annotating `four`:

``` rust
let four = "4".parse::<u32>();

assert_eq!(Ok(4), four);
```

Failing to parse:

``` rust
let nope = "j".parse::<u32>();

assert!(nope.is_err());
```

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2720" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.is_ascii" class="fn">is_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks if all characters in this string are within the ASCII range.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-57" class="doc-anchor">§</a>Examples

``` rust
let ascii = "hello!\n";
let non_ascii = "Grüße, Jürgen ❤";

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.as_ascii" class="fn">as_ascii</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html" class="enum" title="enum core::ascii::ascii_char::AsciiChar">AsciiChar</a>\]\>

🔬This is a nightly-only experimental API. (`ascii_char`)

If this string slice [`is_ascii`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_ascii "method str::is_ascii"), returns it as a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), otherwise returns `None`.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.as_ascii_unchecked" class="fn">as_ascii_unchecked</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html" class="enum" title="enum core::ascii::ascii_char::AsciiChar">AsciiChar</a>\]

🔬This is a nightly-only experimental API. (`ascii_char`)

Converts this string slice into a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), without checking whether they are valid.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#safety-7" class="doc-anchor">§</a>Safety

Every character in this string must be ASCII, or else this is UB.

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2774" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.eq_ignore_ascii_case" class="fn">eq_ignore_ascii_case</a>(&self, other: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks that two strings are an ASCII case-insensitive match.

Same as `to_ascii_lowercase(a) == to_ascii_lowercase(b)`, but without allocating and copying temporaries.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-58" class="doc-anchor">§</a>Examples

``` rust
assert!("Ferris".eq_ignore_ascii_case("FERRIS"));
assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));
```

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2800" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.make_ascii_uppercase" class="fn">make_ascii_uppercase</a>(&mut self)

Converts this string to its ASCII upper case equivalent in-place.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To return a new uppercased value without modifying the existing one, use [`to_ascii_uppercase()`](https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_ascii_uppercase).

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-59" class="doc-anchor">§</a>Examples

``` rust
let mut s = String::from("Grüße, Jürgen ❤");

s.make_ascii_uppercase();

assert_eq!("GRüßE, JüRGEN ❤", s);
```

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2828" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.make_ascii_lowercase" class="fn">make_ascii_lowercase</a>(&mut self)

Converts this string to its ASCII lower case equivalent in-place.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To return a new lowercased value without modifying the existing one, use [`to_ascii_lowercase()`](https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_ascii_lowercase).

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-60" class="doc-anchor">§</a>Examples

``` rust
let mut s = String::from("GRÜßE, JÜRGEN ❤");

s.make_ascii_lowercase();

assert_eq!("grÜße, jÜrgen ❤", s);
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2853" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_ascii_start" class="fn">trim_ascii_start</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns a string slice with leading ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-61" class="doc-anchor">§</a>Examples

``` rust
assert_eq!(" \t \u{3000}hello world\n".trim_ascii_start(), "\u{3000}hello world\n");
assert_eq!("  ".trim_ascii_start(), "");
assert_eq!("".trim_ascii_start(), "");
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2878" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_ascii_end" class="fn">trim_ascii_end</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns a string slice with trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-62" class="doc-anchor">§</a>Examples

``` rust
assert_eq!("\r hello world\u{3000}\n ".trim_ascii_end(), "\r hello world\u{3000}");
assert_eq!("  ".trim_ascii_end(), "");
assert_eq!("".trim_ascii_end(), "");
```

1.80.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2904" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.trim_ascii" class="fn">trim_ascii</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns a string slice with leading and trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-63" class="doc-anchor">§</a>Examples

``` rust
assert_eq!("\r hello world\n ".trim_ascii(), "hello world");
assert_eq!("  ".trim_ascii(), "");
assert_eq!("".trim_ascii(), "");
```

1.34.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2947" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.escape_debug" class="fn">escape_debug</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html" class="struct" title="struct core::str::iter::EscapeDebug">EscapeDebug</a>\<'\_\>

Returns an iterator that escapes each char in `self` with [`char::escape_debug`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_debug "method char::escape_debug").

Note: only extended grapheme codepoints that begin the string will be escaped.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-64" class="doc-anchor">§</a>Examples

As an iterator:

``` rust
for c in "❤\n!".escape_debug() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

``` rust
println!("{}", "❤\n!".escape_debug());
```

Both are equivalent to:

``` rust
println!("❤\\n!");
```

Using `to_string`:

``` rust
assert_eq!("❤\n!".escape_debug().to_string(), "❤\\n!");
```

1.34.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2993" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.escape_default" class="fn">escape_default</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html" class="struct" title="struct core::str::iter::EscapeDefault">EscapeDefault</a>\<'\_\>

Returns an iterator that escapes each char in `self` with [`char::escape_default`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_default "method char::escape_default").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-65" class="doc-anchor">§</a>Examples

As an iterator:

``` rust
for c in "❤\n!".escape_default() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

``` rust
println!("{}", "❤\n!".escape_default());
```

Both are equivalent to:

``` rust
println!("\\u{{2764}}\\n!");
```

Using `to_string`:

``` rust
assert_eq!("❤\n!".escape_default().to_string(), "\\u{2764}\\n!");
```

1.34.0 · <a href="https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3031" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.escape_unicode" class="fn">escape_unicode</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html" class="struct" title="struct core::str::iter::EscapeUnicode">EscapeUnicode</a>\<'\_\>

Returns an iterator that escapes each char in `self` with [`char::escape_unicode`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_unicode "method char::escape_unicode").

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-66" class="doc-anchor">§</a>Examples

As an iterator:

``` rust
for c in "❤\n!".escape_unicode() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

``` rust
println!("{}", "❤\n!".escape_unicode());
```

Both are equivalent to:

``` rust
println!("\\u{{2764}}\\u{{a}}\\u{{21}}");
```

Using `to_string`:

``` rust
assert_eq!("❤\n!".escape_unicode().to_string(), "\\u{2764}\\u{a}\\u{21}");
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.substr_range" class="fn">substr_range</a>(&self, substr: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the range that a substring points to.

Returns `None` if `substr` does not point within `self`.

Unlike [`str::find`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.find "method str::find"), **this does not search through the string**. Instead, it uses pointer arithmetic to find where in the string `substr` is derived from.

This is useful for extending [`str::split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split") and similar methods.

Note that this method may return false positives (typically either `Some(0..0)` or `Some(self.len()..self.len())`) if `substr` is a zero-length `str` that points at the beginning or end of another, independent, `str`.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-67" class="doc-anchor">§</a>Examples

``` rust
#![feature(substr_range)]

let data = "a, b, b, a";
let mut iter = data.split(", ").map(|s| data.substr_range(s).unwrap());

assert_eq!(iter.next(), Some(0..1));
assert_eq!(iter.next(), Some(3..4));
assert_eq!(iter.next(), Some(6..7));
assert_eq!(iter.next(), Some(9..10));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.as_str-1" class="fn">as_str</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

🔬This is a nightly-only experimental API. (`str_as_str`)

Returns the same string as a string slice `&str`.

This method is redundant when used directly on `&str`, but it helps dereferencing other string-like types to string slices, for example references to `Box<str>` or `Arc<str>`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#268" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.replace" class="fn">replace</a>\<P\>(&self, from: P, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Replaces all matches of a pattern with another string.

`replace` creates a new [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String"), and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-68" class="doc-anchor">§</a>Examples

``` rust
let s = "this is old";

assert_eq!("this is new", s.replace("old", "new"));
assert_eq!("than an old", s.replace("is", "an"));
```

When the pattern doesn’t match, it returns this string slice as [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String"):

``` rust
let s = "this is old";
assert_eq!(s, s.replace("cookie monster", "little lamb"));
```

1.16.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#323" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.replacen" class="fn">replacen</a>\<P\>(&self, pat: P, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html" class="trait" title="trait core::str::pattern::Pattern">Pattern</a>,

Replaces first N matches of a pattern with another string.

`replacen` creates a new [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String"), and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice at most `count` times.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-69" class="doc-anchor">§</a>Examples

``` rust
let s = "foo foo 123 foo";
assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
```

When the pattern doesn’t match, it returns this string slice as [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String"):

``` rust
let s = "this is old";
assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
```

1.2.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#380" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_lowercase" class="fn">to_lowercase</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns the lowercase equivalent of this string slice, as a new [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String").

‘Lowercase’ is defined according to the terms of the Unicode Derived Core Property `Lowercase`.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String") instead of modifying the parameter in-place.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-70" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let s = "HELLO";

assert_eq!("hello", s.to_lowercase());
```

A tricky example, with sigma:

``` rust
let sigma = "Σ";

assert_eq!("σ", sigma.to_lowercase());

// but at the end of a word, it's ς, not σ:
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());
```

Languages without case are not changed:

``` rust
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_lowercase());
```

1.2.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#466" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_uppercase" class="fn">to_uppercase</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns the uppercase equivalent of this string slice, as a new [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String").

‘Uppercase’ is defined according to the terms of the Unicode Derived Core Property `Uppercase`.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String") instead of modifying the parameter in-place.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-71" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
let s = "hello";

assert_eq!("HELLO", s.to_uppercase());
```

Scripts without case are not changed:

``` rust
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_uppercase());
```

One character can become multiple:

``` rust
let s = "tschüß";

assert_eq!("TSCHÜSS", s.to_uppercase());
```

1.16.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#530" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.repeat" class="fn">repeat</a>(&self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Creates a new [`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String") by repeating a string `n` times.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#panics-2" class="doc-anchor">§</a>Panics

This function will panic if the capacity would overflow.

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-72" class="doc-anchor">§</a>Examples

Basic usage:

``` rust
assert_eq!("abc".repeat(4), String::from("abcabcabcabc"));
```

A panic upon overflow:

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#" class="tooltip" title="This example panics">ⓘ</a>

``` rust
// this will panic at runtime
let huge = "0123456789abcdef".repeat(usize::MAX);
```

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#560" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_ascii_uppercase" class="fn">to_ascii_uppercase</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns a copy of this string where each character is mapped to its ASCII upper case equivalent.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_uppercase "method str::make_ascii_uppercase").

To uppercase ASCII characters in addition to non-ASCII characters, use [`to_uppercase`](https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_uppercase).

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-73" class="doc-anchor">§</a>Examples

``` rust
let s = "Grüße, Jürgen ❤";

assert_eq!("GRüßE, JüRGEN ❤", s.to_ascii_uppercase());
```

1.23.0 · <a href="https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#592" class="src">Source</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_ascii_lowercase" class="fn">to_ascii_lowercase</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns a copy of this string where each character is mapped to its ASCII lower case equivalent.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_lowercase "method str::make_ascii_lowercase").

To lowercase ASCII characters in addition to non-ASCII characters, use [`to_lowercase`](https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.to_lowercase).

##### <a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#examples-74" class="doc-anchor">§</a>Examples

``` rust
let s = "Grüße, Jürgen ❤";

assert_eq!("grüße, jürgen ❤", s.to_ascii_lowercase());
```

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-Clone-for-TimeZone" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-Debug-for-TimeZone" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-Deref-for-TimeZone" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>

The resulting type after dereferencing.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Dereferences the value.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-Deserialize%3C&#39;de%3E-for-TimeZone" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-Display-for-TimeZone" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-Hash-for-TimeZone" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-PartialEq-for-TimeZone" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-Serialize-for-TimeZone" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-Eq-for-TimeZone" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#impl-StructuralPartialEq-for-TimeZone" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.TimeZone.html" class="struct" title="struct polars::prelude::TimeZone">TimeZone</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/datatypes/struct.TimeZone.html#blanket-implementations" class="anchor">§</a>
