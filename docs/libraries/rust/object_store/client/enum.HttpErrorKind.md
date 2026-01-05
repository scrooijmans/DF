# Enum HttpErrorKind Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/http/connection.rs.html#47-72" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum HttpErrorKind {
    Connect,
    Request,
    Timeout,
    Interrupted,
    Decode,
    Unknown,
}
```

Available on **crate feature `cloud`** only.

Expand description

Identifies the kind of [`HttpError`](https://docs.rs/object_store/latest/object_store/client/struct.HttpError.html "struct object_store::client::HttpError")

This is used, among other things, to determine if a request can be retried

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#variant.Connect" class="anchor">§</a>

### Connect

An error occurred whilst connecting to the remote

Will be automatically retried

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#variant.Request" class="anchor">§</a>

### Request

An error occurred whilst making the request

Will be automatically retried

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#variant.Timeout" class="anchor">§</a>

### Timeout

Request timed out

Will be automatically retried if the request is idempotent

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#variant.Interrupted" class="anchor">§</a>

### Interrupted

The request was aborted

Will be automatically retried if the request is idempotent

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#variant.Decode" class="anchor">§</a>

### Decode

An error occurred whilst decoding the response

Will not be automatically retried

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#variant.Unknown" class="anchor">§</a>

### Unknown

An unknown error occurred

Will not be automatically retried

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#impl-Clone-for-HttpErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#impl-Debug-for-HttpErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#impl-PartialEq-for-HttpErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#impl-Copy-for-HttpErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#impl-Eq-for-HttpErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>

<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#impl-StructuralPartialEq-for-HttpErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html" class="enum" title="enum object_store::client::HttpErrorKind">HttpErrorKind</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/client/enum.HttpErrorKind.html#blanket-implementations" class="anchor">§</a>
