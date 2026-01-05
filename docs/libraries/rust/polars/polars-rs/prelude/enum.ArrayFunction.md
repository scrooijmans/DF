# Enum ArrayFunction Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/function_expr/array.rs.html#10" class="src">Source</a>

``` rust
pub enum ArrayFunction {
Show 22 variants    Length,
    Slice(i64, i64),
    Min,
    Max,
    Sum,
    ToList,
    Unique(bool),
    NUnique,
    Std(u8),
    Var(u8),
    Mean,
    Median,
    Sort(SortOptions),
    Reverse,
    ArgMin,
    ArgMax,
    Get(bool),
    Join(bool),
    Contains {
        nulls_equal: bool,
    },
    Shift,
    Explode {
        skip_empty: bool,
    },
    Concat,
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Length" class="anchor">§</a>

### Length

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Slice" class="anchor">§</a>

### Slice(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Min" class="anchor">§</a>

### Min

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Max" class="anchor">§</a>

### Max

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Sum" class="anchor">§</a>

### Sum

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.ToList" class="anchor">§</a>

### ToList

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Unique" class="anchor">§</a>

### Unique(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.NUnique" class="anchor">§</a>

### NUnique

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Std" class="anchor">§</a>

### Std(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Var" class="anchor">§</a>

### Var(<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Mean" class="anchor">§</a>

### Mean

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Median" class="anchor">§</a>

### Median

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Sort" class="anchor">§</a>

### Sort(<a href="https://docs.rs/polars/latest/polars/prelude/struct.SortOptions.html" class="struct" title="struct polars::prelude::SortOptions">SortOptions</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Reverse" class="anchor">§</a>

### Reverse

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.ArgMin" class="anchor">§</a>

### ArgMin

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.ArgMax" class="anchor">§</a>

### ArgMax

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Get" class="anchor">§</a>

### Get(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Join" class="anchor">§</a>

### Join(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Contains" class="anchor">§</a>

### Contains

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Contains.field.nulls_equal" class="anchor field">§</a>`nulls_equal: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Shift" class="anchor">§</a>

### Shift

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Explode" class="anchor">§</a>

### Explode

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Explode.field.skip_empty" class="anchor field">§</a>`skip_empty: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#variant.Concat" class="anchor">§</a>

### Concat

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-Clone-for-ArrayFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-Debug-for-ArrayFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-Deserialize%3C&#39;de%3E-for-ArrayFunction" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-Display-for-ArrayFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-From%3CArrayFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-Hash-for-ArrayFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-PartialEq-for-ArrayFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-Serialize-for-ArrayFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-Eq-for-ArrayFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#impl-StructuralPartialEq-for-ArrayFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html" class="enum" title="enum polars::prelude::ArrayFunction">ArrayFunction</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.ArrayFunction.html#blanket-implementations" class="anchor">§</a>
