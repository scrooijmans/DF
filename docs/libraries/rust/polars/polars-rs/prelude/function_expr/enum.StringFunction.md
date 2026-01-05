# Enum StringFunction Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/function_expr/strings.rs.html#9" class="src">Source</a>

``` rust
pub enum StringFunction {
Show 36 variants    ConcatHorizontal {
        delimiter: PlSmallStr,
        ignore_nulls: bool,
    },
    ConcatVertical {
        delimiter: PlSmallStr,
        ignore_nulls: bool,
    },
    Contains {
        literal: bool,
        strict: bool,
    },
    CountMatches(bool),
    EndsWith,
    Extract(usize),
    ExtractAll,
    ExtractGroups {
        dtype: DataType,
        pat: PlSmallStr,
    },
    Find {
        literal: bool,
        strict: bool,
    },
    ToInteger {
        dtype: Option<DataType>,
        strict: bool,
    },
    LenBytes,
    LenChars,
    Lowercase,
    Replace {
        n: i64,
        literal: bool,
    },
    Reverse,
    Slice,
    Head,
    Tail,
    HexEncode,
    HexDecode(bool),
    Base64Encode,
    Base64Decode(bool),
    StartsWith,
    StripChars,
    StripCharsStart,
    StripCharsEnd,
    StripPrefix,
    StripSuffix,
    SplitExact {
        n: usize,
        inclusive: bool,
    },
    SplitN(usize),
    Strptime(DataTypeExpr, StrptimeOptions),
    Split(bool),
    ToDecimal {
        scale: usize,
    },
    Titlecase,
    Uppercase,
    EscapeRegex,
}
```

Available on **crate feature `lazy`** only.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ConcatHorizontal" class="anchor">§</a>

### ConcatHorizontal

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ConcatHorizontal.field.delimiter" class="anchor field">§</a>`delimiter: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ConcatHorizontal.field.ignore_nulls" class="anchor field">§</a>`ignore_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ConcatVertical" class="anchor">§</a>

### ConcatVertical

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ConcatVertical.field.delimiter" class="anchor field">§</a>`delimiter: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ConcatVertical.field.ignore_nulls" class="anchor field">§</a>`ignore_nulls: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Contains" class="anchor">§</a>

### Contains

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Contains.field.literal" class="anchor field">§</a>`literal: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Contains.field.strict" class="anchor field">§</a>`strict: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.CountMatches" class="anchor">§</a>

### CountMatches(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.EndsWith" class="anchor">§</a>

### EndsWith

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Extract" class="anchor">§</a>

### Extract(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ExtractAll" class="anchor">§</a>

### ExtractAll

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ExtractGroups" class="anchor">§</a>

### ExtractGroups

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ExtractGroups.field.dtype" class="anchor field">§</a>`dtype: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ExtractGroups.field.pat" class="anchor field">§</a>`pat: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Find" class="anchor">§</a>

### Find

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Find.field.literal" class="anchor field">§</a>`literal: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Find.field.strict" class="anchor field">§</a>`strict: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ToInteger" class="anchor">§</a>

### ToInteger

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ToInteger.field.dtype" class="anchor field">§</a>`dtype: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>`

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ToInteger.field.strict" class="anchor field">§</a>`strict: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.LenBytes" class="anchor">§</a>

### LenBytes

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.LenChars" class="anchor">§</a>

### LenChars

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Lowercase" class="anchor">§</a>

### Lowercase

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Replace" class="anchor">§</a>

### Replace

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Replace.field.n" class="anchor field">§</a>`n: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Replace.field.literal" class="anchor field">§</a>`literal: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Reverse" class="anchor">§</a>

### Reverse

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Slice" class="anchor">§</a>

### Slice

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Head" class="anchor">§</a>

### Head

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Tail" class="anchor">§</a>

### Tail

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.HexEncode" class="anchor">§</a>

### HexEncode

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.HexDecode" class="anchor">§</a>

### HexDecode(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Base64Encode" class="anchor">§</a>

### Base64Encode

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Base64Decode" class="anchor">§</a>

### Base64Decode(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.StartsWith" class="anchor">§</a>

### StartsWith

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.StripChars" class="anchor">§</a>

### StripChars

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.StripCharsStart" class="anchor">§</a>

### StripCharsStart

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.StripCharsEnd" class="anchor">§</a>

### StripCharsEnd

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.StripPrefix" class="anchor">§</a>

### StripPrefix

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.StripSuffix" class="anchor">§</a>

### StripSuffix

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.SplitExact" class="anchor">§</a>

### SplitExact

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.SplitExact.field.n" class="anchor field">§</a>`n: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.SplitExact.field.inclusive" class="anchor field">§</a>`inclusive: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.SplitN" class="anchor">§</a>

### SplitN(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Strptime" class="anchor">§</a>

### Strptime(<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataTypeExpr.html" class="enum" title="enum polars::prelude::DataTypeExpr">DataTypeExpr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.StrptimeOptions.html" class="struct" title="struct polars::prelude::StrptimeOptions">StrptimeOptions</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Split" class="anchor">§</a>

### Split(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ToDecimal" class="anchor">§</a>

### ToDecimal

#### Fields

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.ToDecimal.field.scale" class="anchor field">§</a>`scale: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Titlecase" class="anchor">§</a>

### Titlecase

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.Uppercase" class="anchor">§</a>

### Uppercase

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#variant.EscapeRegex" class="anchor">§</a>

### EscapeRegex

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-Clone-for-StringFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-Debug-for-StringFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-Deserialize%3C&#39;de%3E-for-StringFunction" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-Display-for-StringFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-From%3CStringFunction%3E-for-FunctionExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.FunctionExpr.html" class="enum" title="enum polars::prelude::FunctionExpr">FunctionExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-Hash-for-StringFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-PartialEq-for-StringFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-Serialize-for-StringFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#impl-StructuralPartialEq-for-StringFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/enum.StringFunction.html" class="enum" title="enum polars::prelude::StringFunction">StringFunction</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/function_expr/enum.StringFunction.html#blanket-implementations" class="anchor">§</a>
