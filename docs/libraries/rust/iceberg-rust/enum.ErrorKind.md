# Enum ErrorKind Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/error.rs.html#30-66" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum ErrorKind {
    PreconditionFailed,
    Unexpected,
    DataInvalid,
    NamespaceAlreadyExists,
    TableAlreadyExists,
    NamespaceNotFound,
    TableNotFound,
    FeatureUnsupported,
    CatalogCommitConflicts,
}
```

Expand description

ErrorKind is all kinds of Error of iceberg.

## Variants (Non-exhaustive)<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.PreconditionFailed" class="anchor">§</a>

### PreconditionFailed

The operation was rejected because the system is not in a state required for the operation’s execution.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.Unexpected" class="anchor">§</a>

### Unexpected

Iceberg don’t know what happened here, and no actions other than just returning it back. For example, iceberg returns an internal service error.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.DataInvalid" class="anchor">§</a>

### DataInvalid

Iceberg data is invalid.

This error is returned when we try to read a table from iceberg but failed to parse its metadata or data file correctly.

The table could be invalid or corrupted.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.NamespaceAlreadyExists" class="anchor">§</a>

### NamespaceAlreadyExists

Iceberg namespace already exists at creation.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.TableAlreadyExists" class="anchor">§</a>

### TableAlreadyExists

Iceberg table already exists at creation.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.NamespaceNotFound" class="anchor">§</a>

### NamespaceNotFound

Iceberg namespace does not exist.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.TableNotFound" class="anchor">§</a>

### TableNotFound

Iceberg table does not exist.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.FeatureUnsupported" class="anchor">§</a>

### FeatureUnsupported

Iceberg feature is not supported.

This error is returned when given iceberg feature is not supported.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#variant.CatalogCommitConflicts" class="anchor">§</a>

### CatalogCommitConflicts

Catalog commit failed due to outdated metadata

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-ErrorKind" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#method.into_static" class="fn">into_static</a>(self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Convert self into static str.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-Clone-for-ErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-Debug-for-ErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-Display-for-ErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-From%3CErrorKind%3E-for-%26str" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-PartialEq-for-ErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-Copy-for-ErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-Eq-for-ErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#impl-StructuralPartialEq-for-ErrorKind" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html" class="enum" title="enum iceberg::ErrorKind">ErrorKind</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/enum.ErrorKind.html#blanket-implementations" class="anchor">§</a>
