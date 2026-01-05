# Enum CloudType Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/options.rs.html#152" class="src">Source</a>

``` rust
pub enum CloudType {
    Aws,
    Azure,
    File,
    Gcp,
    Http,
    Hf,
}
```

Available on **crate feature `polars-io`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#variant.Aws" class="anchor">§</a>

### Aws

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#variant.Azure" class="anchor">§</a>

### Azure

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#variant.File" class="anchor">§</a>

### File

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#variant.Gcp" class="anchor">§</a>

### Gcp

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#variant.Http" class="anchor">§</a>

### Http

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#variant.Hf" class="anchor">§</a>

### Hf

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#impl-Clone-for-CloudType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#impl-Debug-for-CloudType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#impl-FromStr-for-CloudType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(url: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>, \<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#impl-PartialEq-for-CloudType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#impl-StructuralPartialEq-for-CloudType" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/enum.CloudType.html" class="enum" title="enum polars::prelude::cloud::CloudType">CloudType</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/options/enum.CloudType.html#blanket-implementations" class="anchor">§</a>
