# Enum BooleanFunction Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/function_expr/boolean.rs.html#9" class="src">Source</a>

``` rust
pub enum BooleanFunction {
Show 16 variants    Any {
        ignore_nulls: bool,
    },
    All {
        ignore_nulls: bool,
    },
    IsNull,
    IsNotNull,
    IsFinite,
    IsInfinite,
    IsNan,
    IsNotNan,
    IsFirstDistinct,
    IsLastDistinct,
    IsBetween {
        closed: ClosedInterval,
    },
    IsIn {
        nulls_equal: bool,
    },
    IsClose {
        abs_tol: TotalOrdWrap<f64>,
        rel_tol: TotalOrdWrap<f64>,
        nans_equal: bool,
    },
    AllHorizontal,
    AnyHorizontal,
    Not,
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.Any" class="anchor">§</a>

### Any

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.Any.field.ignore_nulls" class="anchor field">§</a>`ignore_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.All" class="anchor">§</a>

### All

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.All.field.ignore_nulls" class="anchor field">§</a>`ignore_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsNull" class="anchor">§</a>

### IsNull

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsNotNull" class="anchor">§</a>

### IsNotNull

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsFinite" class="anchor">§</a>

### IsFinite

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsInfinite" class="anchor">§</a>

### IsInfinite

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsNan" class="anchor">§</a>

### IsNan

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsNotNan" class="anchor">§</a>

### IsNotNan

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsFirstDistinct" class="anchor">§</a>

### IsFirstDistinct

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsLastDistinct" class="anchor">§</a>

### IsLastDistinct

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsBetween" class="anchor">§</a>

### IsBetween

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsBetween.field.closed" class="anchor field">§</a>`closed: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.ClosedInterval.html" class="enum" title="enum polars::prelude::ClosedInterval"><code>ClosedInterval</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsIn" class="anchor">§</a>

### IsIn

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsIn.field.nulls_equal" class="anchor field">§</a>`nulls_equal: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsClose" class="anchor">§</a>

### IsClose

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsClose.field.abs_tol" class="anchor field">§</a>`abs_tol: `<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/struct.TotalOrdWrap.html" class="struct" title="struct polars_utils::total_ord::TotalOrdWrap"><code>TotalOrdWrap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive"><code>f64</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsClose.field.rel_tol" class="anchor field">§</a>`rel_tol: `<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/total_ord/struct.TotalOrdWrap.html" class="struct" title="struct polars_utils::total_ord::TotalOrdWrap"><code>TotalOrdWrap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive"><code>f64</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.IsClose.field.nans_equal" class="anchor field">§</a>`nans_equal: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.AllHorizontal" class="anchor">§</a>

### AllHorizontal

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.AnyHorizontal" class="anchor">§</a>

### AnyHorizontal

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#variant.Not" class="anchor">§</a>

### Not

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-Clone-for-BooleanFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-Debug-for-BooleanFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-Deserialize%3C&#39;de%3E-for-BooleanFunction" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-Display-for-BooleanFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-From%3CBooleanFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-Hash-for-BooleanFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-PartialEq-for-BooleanFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-Serialize-for-BooleanFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-Eq-for-BooleanFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#impl-StructuralPartialEq-for-BooleanFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.BooleanFunction.html" class="enum" title="enum polars::prelude::BooleanFunction">BooleanFunction</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.BooleanFunction.html#blanket-implementations" class="anchor">§</a>
