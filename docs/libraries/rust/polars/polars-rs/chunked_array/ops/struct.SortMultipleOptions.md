# Struct SortMultipleOptions Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/sort/options.rs.html#85" class="src">Source</a>

``` rust
pub struct SortMultipleOptions {
    pub descending: Vec<bool>,
    pub nulls_last: Vec<bool>,
    pub multithreaded: bool,
    pub maintain_order: bool,
    pub limit: Option<u32>,
}
```

Expand description

Sort options for multi-series sorting.

Indicating the order of sorting, nulls position, multithreading, and maintaining order.

## <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#example" class="doc-anchor">§</a>Example

``` rust

let df = df! {
    "a" => [Some(1), Some(2), None, Some(4), None],
    "b" => [Some(5), None, Some(3), Some(2), Some(1)]
}?;

let out = df
    .sort(
        ["a", "b"],
        SortMultipleOptions::default()
            .with_maintain_order(true)
            .with_multithreaded(false)
            .with_order_descending_multi([false, true])
            .with_nulls_last(true),
    )?;

let expected = df! {
    "a" => [Some(1), Some(2), Some(4), None, None],
    "b" => [Some(5), None, Some(2), Some(3), Some(1)]
}?;

assert_eq!(out, expected);
```

## Fields<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#structfield.descending" class="anchor field">§</a>`descending: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Order of the columns. Default all \`false\`\`.

If only one value is given, it will broadcast to all columns.

Use [`SortMultipleOptions::with_order_descending_multi`](https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html#method.with_order_descending_multi "method polars::prelude::SortMultipleOptions::with_order_descending_multi") or [`SortMultipleOptions::with_order_descending`](https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html#method.with_order_descending "method polars::prelude::SortMultipleOptions::with_order_descending") to modify.

### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#safety" class="doc-anchor">§</a>Safety

Len must match the number of columns, or equal 1.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#structfield.nulls_last" class="anchor field">§</a>`nulls_last: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Whether place null values last. Default `false`.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#structfield.multithreaded" class="anchor field">§</a>`multithreaded: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether sort in multiple threads. Default `true`.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#structfield.maintain_order" class="anchor field">§</a>`maintain_order: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether maintain the order of equal elements. Default `false`.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#structfield.limit" class="anchor field">§</a>`limit: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive"><code>u32</code></a>`>`

Limit a sort output, this is for optimization purposes and might be ignored.

## Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Create `SortMultipleOptions` with default values.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.with_order_descending_multi" class="fn">with_order_descending_multi</a>( self, descending: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Specify order for each column. Defaults all `false`.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#safety-1" class="doc-anchor">§</a>Safety

Len must match the number of columns, or be equal to 1.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.with_order_descending" class="fn">with_order_descending</a>(self, descending: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Sort order for all columns. Default `false` which is ascending.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.with_nulls_last_multi" class="fn">with_nulls_last_multi</a>( self, nulls_last: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Specify whether to place nulls last, per-column. Defaults all `false`.

##### <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#safety-2" class="doc-anchor">§</a>Safety

Len must match the number of columns, or be equal to 1.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.with_nulls_last" class="fn">with_nulls_last</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Whether to place null values last. Default `false`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.with_multithreaded" class="fn">with_multithreaded</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Whether to sort in multiple threads. Default `true`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.with_maintain_order" class="fn">with_maintain_order</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Whether to maintain the order of equal elements. Default `false`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.with_order_reversed" class="fn">with_order_reversed</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Reverse the order of sorting for each column.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-Clone-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-Debug-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-Default-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-Deserialize%3C&#39;de%3E-for-SortMultipleOptions" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-From%3C%26SortMultipleOptions%3E-for-SortOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-From%3C%26SortOptions%3E-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-Hash-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-PartialEq-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-Serialize-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-Eq-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#impl-StructuralPartialEq-for-SortMultipleOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/chunked_array/ops/struct.SortMultipleOptions.html#blanket-implementations" class="anchor">§</a>
