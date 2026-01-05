# Enum Pattern Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/chunkedarray/string/patterns.rs.html#220" class="src">Source</a>

``` rust
pub enum Pattern {
    DateDMY,
    DateYMD,
    DatetimeYMD,
    DatetimeDMY,
    DatetimeYMDZ,
    Time,
}
```

Available on **crate feature `temporal`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#variant.DateDMY" class="anchor">§</a>

### DateDMY

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#variant.DateYMD" class="anchor">§</a>

### DateYMD

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#variant.DatetimeYMD" class="anchor">§</a>

### DatetimeYMD

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#variant.DatetimeDMY" class="anchor">§</a>

### DatetimeDMY

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#variant.DatetimeYMDZ" class="anchor">§</a>

### DatetimeYMDZ

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#variant.Time" class="anchor">§</a>

### Time

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-Pattern" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.is_inferable" class="fn">is_inferable</a>(&self, val: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-Clone-for-Pattern" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-Debug-for-Pattern" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-Hash-for-Pattern" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-PartialEq-for-Pattern" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-TryFromWithUnit%3CPattern%3E-for-DatetimeInfer%3CInt32Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::TryFromWithUnit">TryFromWithUnit</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.try_from_with_unit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#tymethod.try_from_with_unit" class="fn">try_from_with_unit</a>( value: <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>, \_time_unit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-TryFromWithUnit%3CPattern%3E-for-DatetimeInfer%3CInt64Type%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html" class="trait" title="trait polars::prelude::chunkedarray::string::infer::TryFromWithUnit">TryFromWithUnit</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#method.try_from_with_unit" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/trait.TryFromWithUnit.html#tymethod.try_from_with_unit" class="fn">try_from_with_unit</a>( value: <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>, time_unit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.TimeUnit.html" class="enum" title="enum polars::prelude::TimeUnit">TimeUnit</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/infer/struct.DatetimeInfer.html" class="struct" title="struct polars::prelude::chunkedarray::string::infer::DatetimeInfer">DatetimeInfer</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-Copy-for-Pattern" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-Eq-for-Pattern" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#impl-StructuralPartialEq-for-Pattern" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html" class="enum" title="enum polars::prelude::chunkedarray::string::Pattern">Pattern</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/chunkedarray/string/enum.Pattern.html#blanket-implementations" class="anchor">§</a>
