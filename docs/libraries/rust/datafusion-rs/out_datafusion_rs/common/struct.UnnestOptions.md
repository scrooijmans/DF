# Struct UnnestOptions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/unnest.rs.html#70" class="src">Source</a>

``` rust
pub struct UnnestOptions {
    pub preserve_nulls: bool,
    pub recursions: Vec<RecursionUnnestOption>,
}
```

Expand description

Options for unnesting a column that contains a list type, replicating values in the other, non nested rows.

Conceptually this operation is like joining each row with all the values in the list column.

If `preserve_nulls` is false, nulls and empty lists from the input column are not carried through to the output. This is the default behavior for other systems such as ClickHouse and DuckDB

If `preserve_nulls` is true (the default), nulls from the input column are carried through to the output.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#examples" class="doc-anchor">§</a>Examples

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#unnestc1-preserve_nulls-false" class="doc-anchor">§</a>`Unnest(c1)`, preserve_nulls: false

``` text
     ┌─────────┐ ┌─────┐                ┌─────────┐ ┌─────┐
     │ {1, 2}  │ │  A  │   Unnest       │    1    │ │  A  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │  null   │ │  B  │                │    2    │ │  A  │
     ├─────────┤ ├─────┤ ────────────▶  ├─────────┤ ├─────┤
     │   {}    │ │  D  │                │    3    │ │  E  │
     ├─────────┤ ├─────┤                └─────────┘ └─────┘
     │   {3}   │ │  E  │                    c1        c2
     └─────────┘ └─────┘
       c1         c2
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#unnestc1-preserve_nulls-true" class="doc-anchor">§</a>`Unnest(c1)`, preserve_nulls: true

``` text
     ┌─────────┐ ┌─────┐                ┌─────────┐ ┌─────┐
     │ {1, 2}  │ │  A  │   Unnest       │    1    │ │  A  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │  null   │ │  B  │                │    2    │ │  A  │
     ├─────────┤ ├─────┤ ────────────▶  ├─────────┤ ├─────┤
     │   {}    │ │  D  │                │  null   │ │  B  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │   {3}   │ │  E  │                │    3    │ │  E  │
     └─────────┘ └─────┘                └─────────┘ └─────┘
       c1         c2                        c1        c2
```

`recursions` instruct how a column should be unnested (e.g unnesting a column multiple time, with depth = 1 and depth = 2). Any unnested column not being mentioned inside this options is inferred to be unnested with depth = 1

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#structfield.preserve_nulls" class="anchor field">§</a>`preserve_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Should nulls in the input be preserved? Defaults to true

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#structfield.recursions" class="anchor field">§</a>`recursions: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.RecursionUnnestOption.html" class="struct" title="struct datafusion::common::RecursionUnnestOption"><code>RecursionUnnestOption</code></a>`>`

If specific columns need to be unnested multiple times (e.g at different depth), declare them here. Any unnested columns not being mentioned inside this option will be unnested with depth = 1

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-UnnestOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

Create a new [`UnnestOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html "struct datafusion::common::UnnestOptions") with default values

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.with_preserve_nulls" class="fn">with_preserve_nulls</a>(self, preserve_nulls: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

Set the behavior with nulls in the input as described on [`Self`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html "struct datafusion::common::UnnestOptions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.with_recursions" class="fn">with_recursions</a>(self, recursion: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.RecursionUnnestOption.html" class="struct" title="struct datafusion::common::RecursionUnnestOption">RecursionUnnestOption</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

Set the recursions for the unnest operation

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-Clone-for-UnnestOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-Debug-for-UnnestOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-Default-for-UnnestOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-Hash-for-UnnestOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-PartialEq-for-UnnestOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-PartialOrd-for-UnnestOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-Eq-for-UnnestOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#impl-StructuralPartialEq-for-UnnestOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html" class="struct" title="struct datafusion::common::UnnestOptions">UnnestOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.UnnestOptions.html#blanket-implementations" class="anchor">§</a>
