# Struct CloudLocation Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/glob.rs.html#100" class="src">Source</a>

``` rust
pub struct CloudLocation {
    pub scheme: PlSmallStr,
    pub bucket: PlSmallStr,
    pub prefix: String,
    pub expansion: Option<PlSmallStr>,
}
```

Available on **crate feature `polars-io`** only.

Expand description

A location on cloud storage, may have wildcards.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#structfield.scheme" class="anchor field">§</a>`scheme: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>

The scheme (s3, …).

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#structfield.bucket" class="anchor field">§</a>`bucket: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>

The bucket name.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#structfield.prefix" class="anchor field">§</a>`prefix: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The prefix inside the bucket, this will be the full key when wildcards are not used.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#structfield.expansion" class="anchor field">§</a>`expansion: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`>`

The path components that need to be expanded.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#impl-CloudLocation" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#method.from_url" class="fn">from_url</a>(parsed: &<a href="https://docs.rs/url/2.5.4/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, glob: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#method.new" class="fn">new</a>(url: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, glob: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Parse a CloudLocation from an url.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#impl-Debug-for-CloudLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#impl-Default-for-CloudLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#impl-PartialEq-for-CloudLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#impl-StructuralPartialEq-for-CloudLocation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html" class="struct" title="struct polars::prelude::cloud::CloudLocation">CloudLocation</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudLocation.html#blanket-implementations" class="anchor">§</a>
