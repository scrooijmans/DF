# Struct JoinOptionsIR Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/options/mod.rs.html#115" class="src">Source</a>

``` rust
pub struct JoinOptionsIR {
    pub allow_parallel: bool,
    pub force_parallel: bool,
    pub args: JoinArgs,
    pub options: Option<JoinTypeOptionsIR>,
    pub rows_left: (Option<usize>, usize),
    pub rows_right: (Option<usize>, usize),
}
```

Available on **crate feature `lazy`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#structfield.allow_parallel" class="anchor field">§</a>`allow_parallel: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#structfield.force_parallel" class="anchor field">§</a>`force_parallel: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#structfield.args" class="anchor field">§</a>`args: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinArgs.html" class="struct" title="struct polars::prelude::JoinArgs"><code>JoinArgs</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#structfield.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR"><code>JoinTypeOptionsIR</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#structfield.rows_left" class="anchor field">§</a>`rows_left: (`<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>, `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`)`

Proxy of the number of rows in both sides of the joins Holds `(Option<known_size>, estimated_size)`

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#structfield.rows_right" class="anchor field">§</a>`rows_right: (`<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>, `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`)`

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#impl-Clone-for-JoinOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#impl-Debug-for-JoinOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#impl-From%3CJoinOptions%3E-for-JoinOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptions.html" class="struct" title="struct polars::prelude::JoinOptions">JoinOptions</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(opts: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptions.html" class="struct" title="struct polars::prelude::JoinOptions">JoinOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#impl-From%3CJoinOptionsIR%3E-for-JoinOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptions.html" class="struct" title="struct polars::prelude::JoinOptions">JoinOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(opts: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptions.html" class="struct" title="struct polars::prelude::JoinOptions">JoinOptions</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#impl-Hash-for-JoinOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#impl-PartialEq-for-JoinOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#impl-StructuralPartialEq-for-JoinOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html" class="struct" title="struct polars::prelude::JoinOptionsIR">JoinOptionsIR</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinOptionsIR.html#blanket-implementations" class="anchor">§</a>
