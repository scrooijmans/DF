# Struct CsvParseOptions Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/options.rs.html#50" class="src">Source</a>

``` rust
pub struct CsvParseOptions {
    pub separator: u8,
    pub quote_char: Option<u8>,
    pub eol_char: u8,
    pub encoding: CsvEncoding,
    pub null_values: Option<NullValues>,
    pub missing_is_null: bool,
    pub truncate_ragged_lines: bool,
    pub comment_prefix: Option<CommentPrefix>,
    pub try_parse_dates: bool,
    pub decimal_comma: bool,
}
```

Available on **crate feature `polars-io`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.separator" class="anchor field">§</a>`separator: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.quote_char" class="anchor field">§</a>`quote_char: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.eol_char" class="anchor field">§</a>`eol_char: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.encoding" class="anchor field">§</a>`encoding: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.CsvEncoding.html" class="enum" title="enum polars::prelude::CsvEncoding"><code>CsvEncoding</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.null_values" class="anchor field">§</a>`null_values: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.NullValues.html" class="enum" title="enum polars::prelude::NullValues"><code>NullValues</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.missing_is_null" class="anchor field">§</a>`missing_is_null: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.truncate_ragged_lines" class="anchor field">§</a>`truncate_ragged_lines: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.comment_prefix" class="anchor field">§</a>`comment_prefix: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.CommentPrefix.html" class="enum" title="enum polars::prelude::CommentPrefix"><code>CommentPrefix</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.try_parse_dates" class="anchor field">§</a>`try_parse_dates: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#structfield.decimal_comma" class="anchor field">§</a>`decimal_comma: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_separator" class="fn">with_separator</a>(self, separator: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

The character used to separate fields in the CSV file. This is most often a comma ‘,’.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_quote_char" class="fn">with_quote_char</a>(self, quote_char: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Set the character used for field quoting. This is most often double quotes ‘“’. Set this to [None](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") to disable quote parsing.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_eol_char" class="fn">with_eol_char</a>(self, eol_char: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Set the character used to indicate an end-of-line (eol).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_encoding" class="fn">with_encoding</a>(self, encoding: <a href="https://docs.rs/polars/latest/polars/prelude/enum.CsvEncoding.html" class="enum" title="enum polars::prelude::CsvEncoding">CsvEncoding</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Set the encoding used by the file.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_null_values" class="fn">with_null_values</a>( self, null_values: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.NullValues.html" class="enum" title="enum polars::prelude::NullValues">NullValues</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Set values that will be interpreted as missing/null.

Note: These values are matched before quote-parsing, so if the null values are quoted then those quotes also need to be included here.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_missing_is_null" class="fn">with_missing_is_null</a>(self, missing_is_null: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Treat missing fields as null.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_truncate_ragged_lines" class="fn">with_truncate_ragged_lines</a>( self, truncate_ragged_lines: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Truncate lines that are longer than the schema.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_comment_prefix" class="fn">with_comment_prefix</a>\<T\>( self, comment_prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.CommentPrefix.html" class="enum" title="enum polars::prelude::CommentPrefix">CommentPrefix</a>\>,

Sets the comment prefix for this instance. Lines starting with this prefix will be ignored.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_try_parse_dates" class="fn">with_try_parse_dates</a>(self, try_parse_dates: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Automatically try to parse dates/datetimes and time. If parsing fails, columns remain of dtype [`DataType::String`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.String "variant polars::prelude::DataType::String").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.with_decimal_comma" class="fn">with_decimal_comma</a>(self, decimal_comma: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Parse floats with a comma as decimal separator.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-Clone-for-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-Debug-for-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-Default-for-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Options related to parsing the CSV format.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-Deserialize%3C&#39;de%3E-for-CsvParseOptions" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-Hash-for-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-PartialEq-for-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-Serialize-for-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-Eq-for-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#impl-StructuralPartialEq-for-CsvParseOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html#blanket-implementations" class="anchor">§</a>
