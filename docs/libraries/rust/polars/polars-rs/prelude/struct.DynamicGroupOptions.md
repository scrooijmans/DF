# Struct DynamicGroupOptions Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/group_by/dynamic.rs.html#22" class="src">Source</a>

``` rust
pub struct DynamicGroupOptions {
    pub index_column: PlSmallStr,
    pub every: Duration,
    pub period: Duration,
    pub offset: Duration,
    pub label: Label,
    pub include_boundaries: bool,
    pub closed_window: ClosedWindow,
    pub start_by: StartBy,
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#structfield.index_column" class="anchor field">§</a>`index_column: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>

Time or index column.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#structfield.every" class="anchor field">§</a>`every: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

Start a window at this interval.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#structfield.period" class="anchor field">§</a>`period: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

Window duration.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#structfield.offset" class="anchor field">§</a>`offset: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration"><code>Duration</code></a>

Offset window boundaries.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#structfield.label" class="anchor field">§</a>`label: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.Label.html" class="enum" title="enum polars::prelude::Label"><code>Label</code></a>

Truncate the time column values to the window.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#structfield.include_boundaries" class="anchor field">§</a>`include_boundaries: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Add the boundaries to the DataFrame.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#structfield.closed_window" class="anchor field">§</a>`closed_window: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedWindow.html" class="enum" title="enum polars::prelude::ClosedWindow"><code>ClosedWindow</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#structfield.start_by" class="anchor field">§</a>`start_by: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.StartBy.html" class="enum" title="enum polars::prelude::StartBy"><code>StartBy</code></a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-Clone-for-DynamicGroupOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-Debug-for-DynamicGroupOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-Default-for-DynamicGroupOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-Deserialize%3C&#39;de%3E-for-DynamicGroupOptions" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-Hash-for-DynamicGroupOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-PartialEq-for-DynamicGroupOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-Serialize-for-DynamicGroupOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-Eq-for-DynamicGroupOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#impl-StructuralPartialEq-for-DynamicGroupOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html#blanket-implementations" class="anchor">§</a>
