# Enum Ambiguous Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/kernels/time.rs.html#14" class="src">Source</a>

``` rust
pub enum Ambiguous {
    Earliest,
    Latest,
    Null,
    Raise,
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#variant.Earliest" class="anchor">§</a>

### Earliest

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#variant.Latest" class="anchor">§</a>

### Latest

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#variant.Null" class="anchor">§</a>

### Null

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#variant.Raise" class="anchor">§</a>

### Raise

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#impl-FromStr-for-Ambiguous" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html" class="enum" title="enum polars::prelude::Ambiguous">Ambiguous</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html" class="enum" title="enum polars::prelude::Ambiguous">Ambiguous</a>, \<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html" class="enum" title="enum polars::prelude::Ambiguous">Ambiguous</a> as <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a>\>::<a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype" title="type core::str::traits::FromStr::Err">Err</a>\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.Ambiguous.html#blanket-implementations" class="anchor">§</a>
