# Struct Path Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/path/mod.rs.html#154-157" class="src">Source</a>

``` rust
pub struct Path { /* private fields */ }
```

Expand description

A parsed path representation that can be safely written to object storage

A [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") maintains the following invariants:

- Paths are delimited by `/`
- Paths do not contain leading or trailing `/`
- Paths do not contain relative path segments, i.e. `.` or `..`
- Paths do not contain empty path segments
- Paths do not contain any ASCII control characters

There are no enforced restrictions on path length, however, it should be noted that most object stores do not permit paths longer than 1024 bytes, and many filesystems do not support path segments longer than 255 bytes.

## <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#encode" class="doc-anchor">§</a>Encode

In theory object stores support any UTF-8 character sequence, however, certain character sequences cause compatibility problems with some applications and protocols. Additionally some filesystems may impose character restrictions, see [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem"). As such the naming guidelines for [S3](https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html), [GCS](https://cloud.google.com/storage/docs/naming-objects) and [Azure Blob Storage](https://docs.microsoft.com/en-us/rest/api/storageservices/Naming-and-Referencing-Containers--Blobs--and-Metadata#blob-names) all recommend sticking to a limited character subset.

A string containing potentially problematic path segments can therefore be encoded to a [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") using [`Path::from`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from "associated function object_store::path::Path::from") or [`Path::from_iter`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from_iter "associated function object_store::path::Path::from_iter"). This will percent encode any problematic segments according to [RFC 1738](https://www.ietf.org/rfc/rfc1738.txt).

``` rust
assert_eq!(Path::from("foo/bar").as_ref(), "foo/bar");
assert_eq!(Path::from("foo//bar").as_ref(), "foo/bar");
assert_eq!(Path::from("foo/../bar").as_ref(), "foo/%2E%2E/bar");
assert_eq!(Path::from("/").as_ref(), "");
assert_eq!(Path::from_iter(["foo", "foo/bar"]).as_ref(), "foo/foo%2Fbar");
```

Note: if provided with an already percent encoded string, this will encode it again

``` rust
assert_eq!(Path::from("foo/foo%2Fbar").as_ref(), "foo/foo%252Fbar");
```

## <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#parse" class="doc-anchor">§</a>Parse

Alternatively a [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") can be parsed from an existing string, returning an error if it is invalid. Unlike the encoding methods above, this will permit arbitrary unicode, including percent encoded sequences.

``` rust
assert_eq!(Path::parse("/foo/foo%2Fbar").unwrap().as_ref(), "foo/foo%2Fbar");
Path::parse("..").unwrap_err(); // Relative path segments are disallowed
Path::parse("/foo//").unwrap_err(); // Empty path segments are disallowed
Path::parse("\x00").unwrap_err(); // ASCII control characters are disallowed
```

## Implementations<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-Path" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.parse" class="fn">parse</a>(path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>\>

Parse a string as a [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path"), returning a [`Error`](https://docs.rs/object_store/latest/object_store/path/enum.Error.html "enum object_store::path::Error") if invalid, as defined on the docstring for [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path")

Note: this will strip any leading `/` or trailing `/`

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from_filesystem_path" class="fn">from_filesystem_path</a>(path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>\>

Available on **non-WebAssembly** only.

Convert a filesystem path to a [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") relative to the filesystem root

This will return an error if the path contains illegal character sequences as defined on the docstring for [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") or does not exist

Note: this will canonicalize the provided path, resolving any symlinks

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from_absolute_path" class="fn">from_absolute_path</a>(path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>\>

Available on **non-WebAssembly** only.

Convert an absolute filesystem path to a [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") relative to the filesystem root

This will return an error if the path contains illegal character sequences, as defined on the docstring for [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path"), or `base` is not an absolute path

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from_url_path" class="fn">from_url_path</a>(path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/object_store/latest/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>\>

Parse a url encoded string as a [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path"), returning a [`Error`](https://docs.rs/object_store/latest/object_store/path/enum.Error.html "enum object_store::path::Error") if invalid

This will return an error if the path contains illegal character sequences as defined on the docstring for [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.parts" class="fn">parts</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html" class="struct" title="struct object_store::path::PathPart">PathPart</a>\<'\_\>\>

Returns the [`PathPart`](https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html "struct object_store::path::PathPart") of this [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.filename" class="fn">filename</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the last path segment containing the filename stored in this [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.extension" class="fn">extension</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the extension of the file stored in this [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path"), if any

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.prefix_match" class="fn">prefix_match</a>( &self, prefix: &Self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html" class="struct" title="struct object_store::path::PathPart">PathPart</a>\<'\_\>\> + '\_\>

Returns an iterator of the [`PathPart`](https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html "struct object_store::path::PathPart") of this [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") after `prefix`

Returns `None` if the prefix does not match

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.prefix_matches" class="fn">prefix_matches</a>(&self, prefix: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") starts with `prefix`

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.child" class="fn">child</a>\<'a\>(&self, child: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html" class="struct" title="struct object_store::path::PathPart">PathPart</a>\<'a\>\>) -\> Self

Creates a new child of this [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path")

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-AsRef%3Cstr%3E-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-Clone-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-Debug-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-Default-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-Display-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-From%3C%26str%3E-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-From%3CPath%3E-for-String" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(path: <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-From%3CString%3E-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(path: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-FromIterator%3CI%3E-for-Path" class="anchor">§</a>

### impl\<'a, I\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<I\> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/struct.PathPart.html" class="struct" title="struct object_store::path::PathPart">PathPart</a>\<'a\>\>,

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = I\>\>(iter: T) -\> Self

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-Hash-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-Ord-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-PartialEq-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-PartialOrd-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-Eq-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#impl-StructuralPartialEq-for-Path" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html#blanket-implementations" class="anchor">§</a>
