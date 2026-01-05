# Enum JoinTypeOptionsIR Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/options/mod.rs.html#75" class="src">Source</a>

``` rust
pub enum JoinTypeOptionsIR {
    IEJoin(IEJoinOptions),
    CrossAndFilter {
        predicate: ExprIR,
    },
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#variant.IEJoin" class="anchor">§</a>

### IEJoin(<a href="https://docs.rs/polars/latest/polars/prelude/struct.IEJoinOptions.html" class="struct" title="struct polars::prelude::IEJoinOptions">IEJoinOptions</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#variant.CrossAndFilter" class="anchor">§</a>

### CrossAndFilter

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#variant.CrossAndFilter.field.predicate" class="anchor field">§</a>`predicate: `<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/expr_ir/struct.ExprIR.html" class="struct" title="struct polars_plan::plans::expr_ir::ExprIR"><code>ExprIR</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-JoinTypeOptionsIR" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.compile" class="fn">compile</a>\<C\>(self, plan: C) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptions.html" class="enum" title="enum polars::prelude::JoinTypeOptions">JoinTypeOptions</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where C: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/expr_ir/struct.ExprIR.html" class="struct" title="struct polars_plan::plans::expr_ir::ExprIR">ExprIR</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/trait.CrossJoinFilter.html" class="trait" title="trait polars::prelude::CrossJoinFilter">CrossJoinFilter</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-Clone-for-JoinTypeOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-Debug-for-JoinTypeOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-From%3C%26JoinTypeOptionsIR%3E-for-%26str" class="anchor">§</a>

### impl\<'\_derivative_strum\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&'\_derivative_strum <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(x: &'\_derivative_strum <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-From%3CJoinTypeOptionsIR%3E-for-%26str" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>\> for &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(x: <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-Hash-for-JoinTypeOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-PartialEq-for-JoinTypeOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-Eq-for-JoinTypeOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#impl-StructuralPartialEq-for-JoinTypeOptionsIR" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html" class="enum" title="enum polars::prelude::JoinTypeOptionsIR">JoinTypeOptionsIR</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptionsIR.html#blanket-implementations" class="anchor">§</a>
