# Struct SerializeOptions Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/write/options.rs.html#34" class="src">Source</a>

``` rust
pub struct SerializeOptions {
    pub date_format: Option<String>,
    pub time_format: Option<String>,
    pub datetime_format: Option<String>,
    pub float_scientific: Option<bool>,
    pub float_precision: Option<usize>,
    pub decimal_comma: bool,
    pub separator: u8,
    pub quote_char: u8,
    pub null: String,
    pub line_terminator: String,
    pub quote_style: QuoteStyle,
}
```

Available on **crate feature `polars-io`** only.

Expand description

Options to serialize logical types to CSV.

The default is to format times and dates as `chrono` crate formats them.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.date_format" class="anchor field">§</a>`date_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Used for [`DataType::Date`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Date "variant polars::prelude::DataType::Date").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.time_format" class="anchor field">§</a>`time_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Used for [`DataType::Time`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Time "variant polars::prelude::DataType::Time").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.datetime_format" class="anchor field">§</a>`datetime_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Used for [`DataType::Datetime`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Datetime "variant polars::prelude::DataType::Datetime").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.float_scientific" class="anchor field">§</a>`float_scientific: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Used for [`DataType::Float64`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Float64 "variant polars::prelude::DataType::Float64") and [`DataType::Float32`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.Float32 "variant polars::prelude::DataType::Float32").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.float_precision" class="anchor field">§</a>`float_precision: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.decimal_comma" class="anchor field">§</a>`decimal_comma: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Use comma as the decimal separator.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.separator" class="anchor field">§</a>`separator: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>

Used as separator.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.quote_char" class="anchor field">§</a>`quote_char: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>

Quoting character.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.null" class="anchor field">§</a>`null: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

Null value representation.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.line_terminator" class="anchor field">§</a>`line_terminator: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

String appended after every row.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#structfield.quote_style" class="anchor field">§</a>`quote_style: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.QuoteStyle.html" class="enum" title="enum polars::prelude::QuoteStyle"><code>QuoteStyle</code></a>

When to insert quotes.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-Clone-for-SerializeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-Debug-for-SerializeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-Default-for-SerializeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-Deserialize%3C&#39;de%3E-for-SerializeOptions" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-Hash-for-SerializeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-PartialEq-for-SerializeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-Serialize-for-SerializeOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-Eq-for-SerializeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#impl-StructuralPartialEq-for-SerializeOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html" class="struct" title="struct polars::prelude::SerializeOptions">SerializeOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.SerializeOptions.html#blanket-implementations" class="anchor">§</a>
