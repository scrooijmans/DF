# Enum Checksum Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/aws/checksum.rs.html#24-27" class="src">Source</a>

``` rust
pub enum Checksum {
    SHA256,
}
```

Available on **crate feature `aws`** only.

Expand description

Enum representing checksum algorithm supported by S3.

## Variants<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#variants" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#variant.SHA256" class="anchor">§</a>

### SHA256

SHA-256 algorithm.

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-Clone-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-Debug-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-Display-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-FromStr-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, Self::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-PartialEq-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-TryFrom%3C%26String%3E-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(value: &<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, Self::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-Copy-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-Eq-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#impl-StructuralPartialEq-for-Checksum" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html" class="enum" title="enum object_store::aws::Checksum">Checksum</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/aws/enum.Checksum.html#blanket-implementations" class="anchor">§</a>
