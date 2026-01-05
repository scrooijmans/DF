# Struct OperatorUri Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/operator/uri.rs.html#28-36" class="src">Source</a>

``` rust
pub struct OperatorUri { /* private fields */ }
```

Expand description

Parsed representation of an operator URI with normalized components.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#impl-OperatorUri" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.new" class="fn">new</a>( base: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, extra_options: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)\>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<Self\>

Build [`OperatorUri`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html "struct opendal::OperatorUri") from a URI string plus additional options.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.scheme" class="fn">scheme</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Normalized scheme in lowercase.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.name" class="fn">name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Name extracted from the URI authority, if present.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.authority" class="fn">authority</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Authority extracted from the URI, if present (host with optional port).

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.username" class="fn">username</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Username extracted from the URI, if present.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.password" class="fn">password</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Password extracted from the URI, if present.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.root" class="fn">root</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Root path (without leading slash) extracted from the URI path, if present.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.options" class="fn">options</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Normalized option map merged from query string and extra options (excluding reserved keys).

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.option" class="fn">option</a>(&self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Retrieve a specific option by key (case-insensitive).

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#impl-Clone-for-OperatorUri" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#impl-Debug-for-OperatorUri" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#impl-IntoOperatorUri-for-%26OperatorUri" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for &<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.into_operator_uri-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

Convert the input into an [`OperatorUri`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html "struct opendal::OperatorUri").

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#impl-IntoOperatorUri-for-OperatorUri" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.into_operator_uri" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

Convert the input into an [`OperatorUri`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html "struct opendal::OperatorUri").

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#impl-PartialEq-for-OperatorUri" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#impl-Eq-for-OperatorUri" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#impl-StructuralPartialEq-for-OperatorUri" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html#blanket-implementations" class="anchor">Â§</a>
