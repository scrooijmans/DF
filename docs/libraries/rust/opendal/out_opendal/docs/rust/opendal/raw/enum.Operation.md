# Enum Operation Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/operation.rs.html#30-52" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum Operation {
    Info,
    CreateDir,
    Read,
    Write,
    Copy,
    Rename,
    Stat,
    Delete,
    List,
    Presign,
}
```

Expand description

Operation is the name of the operation that is being performed.

Most operations can be mapped to the methods of the `Access` trait, but we modify the names to make them more readable and clear for users.

The same operation might have different meanings and costs in different storage services.

## Variants (Non-exhaustive)<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variants" class="anchor">Â§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.Info" class="anchor">Â§</a>

### Info

Operation to retrieve information about the specified storage services.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.CreateDir" class="anchor">Â§</a>

### CreateDir

Operation to create a directory.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.Read" class="anchor">Â§</a>

### Read

Operation to read a file.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.Write" class="anchor">Â§</a>

### Write

Operation to write to a file.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.Copy" class="anchor">Â§</a>

### Copy

Operation to copy a file.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.Rename" class="anchor">Â§</a>

### Rename

Operation to rename a file.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.Stat" class="anchor">Â§</a>

### Stat

Operation to stat a file or a directory.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.Delete" class="anchor">Â§</a>

### Delete

Operation to delete files.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.List" class="anchor">Â§</a>

### List

Operation to get the next file from the list.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#variant.Presign" class="anchor">Â§</a>

### Presign

Operation to generate a presigned URL.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-Operation" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.into_static" class="fn">into_static</a>(self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Convert self into static str.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-Clone-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-Debug-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-Default-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-Display-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-From%3COperation%3E-for-%26str" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-From%3COperation%3E-for-String" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>\> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.from-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(v: <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>) -\> Self

Converts to this type from the input type.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-Hash-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.hash" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.hash_slice" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-PartialEq-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-Copy-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-Eq-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#impl-StructuralPartialEq-for-Operation" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html#blanket-implementations" class="anchor">Â§</a>
