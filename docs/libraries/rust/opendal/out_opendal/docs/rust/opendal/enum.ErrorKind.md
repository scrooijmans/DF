# Enum ErrorKind Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/error.rs.html#51-89" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum ErrorKind {
    Unexpected,
    Unsupported,
    ConfigInvalid,
    NotFound,
    PermissionDenied,
    IsADirectory,
    NotADirectory,
    AlreadyExists,
    RateLimited,
    IsSameFile,
    ConditionNotMatch,
    RangeNotSatisfied,
}
```

Expand description

ErrorKind is all kinds of Error of opendal.

## Variants (Non-exhaustive)<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variants" class="anchor">Â§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.Unexpected" class="anchor">Â§</a>

### Unexpected

OpenDAL donâ€™t know what happened here, and no actions other than just returning it back. For example, s3 returns an internal service error.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.Unsupported" class="anchor">Â§</a>

### Unsupported

Underlying service doesnâ€™t support this operation.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.ConfigInvalid" class="anchor">Â§</a>

### ConfigInvalid

The config for backend is invalid.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.NotFound" class="anchor">Â§</a>

### NotFound

The given path is not found.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.PermissionDenied" class="anchor">Â§</a>

### PermissionDenied

The given path doesnâ€™t have enough permission for this operation

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.IsADirectory" class="anchor">Â§</a>

### IsADirectory

The given path is a directory.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.NotADirectory" class="anchor">Â§</a>

### NotADirectory

The given path is not a directory.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.AlreadyExists" class="anchor">Â§</a>

### AlreadyExists

The given path already exists thus we failed to the specified operation on it.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.RateLimited" class="anchor">Â§</a>

### RateLimited

Requests that sent to this path is over the limit, please slow down.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.IsSameFile" class="anchor">Â§</a>

### IsSameFile

The given file paths are same.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.ConditionNotMatch" class="anchor">Â§</a>

### ConditionNotMatch

The condition of this operation is not match.

The `condition` itself is context based.

For example, in S3, the `condition` can be:

1.  writing a file with If-Match header but the fileâ€™s ETag is not match (will get a 412 Precondition Failed).
2.  reading a file with If-None-Match header but the fileâ€™s ETag is match (will get a 304 Not Modified).

As OpenDAL cannot handle the `condition not match` error, it will always return this error to users. So users could to handle this error by themselves.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.RangeNotSatisfied" class="anchor">Â§</a>

### RangeNotSatisfied

The range of the content is not satisfied.

OpenDAL returns this error to indicate that the range of the read request is not satisfied.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.into_static" class="fn">into_static</a>(self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Convert self into static str.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-Clone-for-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-Debug-for-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-Display-for-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-From%3CErrorKind%3E-for-%26str" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-Hash-for-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.hash" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.hash_slice" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-PartialEq-for-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-Copy-for-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-Eq-for-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#impl-StructuralPartialEq-for-ErrorKind" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind">ErrorKind</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#blanket-implementations" class="anchor">Â§</a>
