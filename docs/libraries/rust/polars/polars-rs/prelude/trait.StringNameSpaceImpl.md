# Trait StringNameSpaceImpl Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/chunked_array/strings/namespace.rs.html#105" class="src">Source</a>

``` rust
pub trait StringNameSpaceImpl: AsString {
Show 42 methods    // Provided methods
    fn hex_decode(
        &self,
        strict: bool,
    ) -> Result<ChunkedArray<BinaryType>, PolarsError> { ... }
    fn hex_encode(&self) -> ChunkedArray<StringType> { ... }
    fn base64_decode(
        &self,
        strict: bool,
    ) -> Result<ChunkedArray<BinaryType>, PolarsError> { ... }
    fn base64_encode(&self) -> ChunkedArray<StringType> { ... }
    fn to_integer(
        &self,
        base: &ChunkedArray<UInt32Type>,
        dtype: Option<DataType>,
        strict: bool,
    ) -> Result<Series, PolarsError> { ... }
    fn contains_chunked(
        &self,
        pat: &ChunkedArray<StringType>,
        literal: bool,
        strict: bool,
    ) -> Result<ChunkedArray<BooleanType>, PolarsError> { ... }
    fn find_chunked(
        &self,
        pat: &ChunkedArray<StringType>,
        literal: bool,
        strict: bool,
    ) -> Result<ChunkedArray<UInt32Type>, PolarsError> { ... }
    fn str_len_chars(&self) -> ChunkedArray<UInt32Type> { ... }
    fn str_len_bytes(&self) -> ChunkedArray<UInt32Type> { ... }
    fn contains(
        &self,
        pat: &str,
        strict: bool,
    ) -> Result<ChunkedArray<BooleanType>, PolarsError> { ... }
    fn contains_literal(
        &self,
        lit: &str,
    ) -> Result<ChunkedArray<BooleanType>, PolarsError> { ... }
    fn find_literal(
        &self,
        lit: &str,
    ) -> Result<ChunkedArray<UInt32Type>, PolarsError> { ... }
    fn find(
        &self,
        pat: &str,
        strict: bool,
    ) -> Result<ChunkedArray<UInt32Type>, PolarsError> { ... }
    fn replace<'a>(
        &'a self,
        pat: &str,
        val: &str,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn replace_literal<'a>(
        &'a self,
        pat: &str,
        val: &str,
        n: usize,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn replace_all(
        &self,
        pat: &str,
        val: &str,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn replace_literal_all<'a>(
        &'a self,
        pat: &str,
        val: &str,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn extract(
        &self,
        pat: &ChunkedArray<StringType>,
        group_index: usize,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn extract_all(
        &self,
        pat: &str,
    ) -> Result<ChunkedArray<ListType>, PolarsError> { ... }
    fn strip_chars(
        &self,
        pat: &Column,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn strip_chars_start(
        &self,
        pat: &Column,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn strip_chars_end(
        &self,
        pat: &Column,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn strip_prefix(
        &self,
        prefix: &ChunkedArray<StringType>,
    ) -> ChunkedArray<StringType> { ... }
    fn strip_suffix(
        &self,
        suffix: &ChunkedArray<StringType>,
    ) -> ChunkedArray<StringType> { ... }
    fn split_exact(
        &self,
        by: &ChunkedArray<StringType>,
        n: usize,
    ) -> Result<ChunkedArray<StructType>, PolarsError> { ... }
    fn split_exact_inclusive(
        &self,
        by: &ChunkedArray<StringType>,
        n: usize,
    ) -> Result<ChunkedArray<StructType>, PolarsError> { ... }
    fn splitn(
        &self,
        by: &ChunkedArray<StringType>,
        n: usize,
    ) -> Result<ChunkedArray<StructType>, PolarsError> { ... }
    fn split(
        &self,
        by: &ChunkedArray<StringType>,
    ) -> Result<ChunkedArray<ListType>, PolarsError> { ... }
    fn split_inclusive(
        &self,
        by: &ChunkedArray<StringType>,
    ) -> Result<ChunkedArray<ListType>, PolarsError> { ... }
    fn extract_all_many(
        &self,
        pat: &ChunkedArray<StringType>,
    ) -> Result<ChunkedArray<ListType>, PolarsError> { ... }
    fn extract_groups(
        &self,
        pat: &str,
        dtype: &DataType,
    ) -> Result<Series, PolarsError> { ... }
    fn count_matches(
        &self,
        pat: &str,
        literal: bool,
    ) -> Result<ChunkedArray<UInt32Type>, PolarsError> { ... }
    fn count_matches_many(
        &self,
        pat: &ChunkedArray<StringType>,
        literal: bool,
    ) -> Result<ChunkedArray<UInt32Type>, PolarsError> { ... }
    fn to_lowercase(&self) -> ChunkedArray<StringType> { ... }
    fn to_uppercase(&self) -> ChunkedArray<StringType> { ... }
    fn to_titlecase(&self) -> ChunkedArray<StringType> { ... }
    fn concat(
        &self,
        other: &ChunkedArray<StringType>,
    ) -> ChunkedArray<StringType> { ... }
    fn str_reverse(&self) -> ChunkedArray<StringType> { ... }
    fn str_slice(
        &self,
        offset: &Column,
        length: &Column,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn str_head(
        &self,
        n: &Column,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn str_tail(
        &self,
        n: &Column,
    ) -> Result<ChunkedArray<StringType>, PolarsError> { ... }
    fn str_escape_regex(&self) -> ChunkedArray<StringType> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.hex_decode" class="fn">hex_decode</a>( &self, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.hex_encode" class="fn">hex_encode</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.base64_decode" class="fn">base64_decode</a>( &self, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BinaryType.html" class="struct" title="struct polars::prelude::BinaryType">BinaryType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.base64_encode" class="fn">base64_encode</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.to_integer" class="fn">to_integer</a>( &self, base: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, dtype: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.contains_chunked" class="fn">contains_chunked</a>( &self, pat: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, literal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.find_chunked" class="fn">find_chunked</a>( &self, pat: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, literal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.str_len_chars" class="fn">str_len_chars</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

Get the length of the string values as number of chars.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.str_len_bytes" class="fn">str_len_bytes</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>

Get the length of the string values as number of bytes.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.contains" class="fn">contains</a>( &self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check if strings contain a regex pattern.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.contains_literal" class="fn">contains_literal</a>( &self, lit: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check if strings contain a given literal

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.find_literal" class="fn">find_literal</a>( &self, lit: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Return the index position of a literal substring in the target string.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.find" class="fn">find</a>( &self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Return the index position of a regular expression substring in the target string.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.replace" class="fn">replace</a>\<'a\>( &'a self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, val: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Replace the leftmost regex-matched (sub)string with another string

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.replace_literal" class="fn">replace_literal</a>\<'a\>( &'a self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, val: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Replace the leftmost literal (sub)string with another string

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.replace_all" class="fn">replace_all</a>( &self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, val: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Replace all regex-matched (sub)strings with another string

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.replace_literal_all" class="fn">replace_literal_all</a>\<'a\>( &'a self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, val: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Replace all matching literal (sub)strings with another string

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.extract" class="fn">extract</a>( &self, pat: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, group_index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract the nth capture group from pattern.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.extract_all" class="fn">extract_all</a>(&self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract each successive non-overlapping regex match in an individual string as an array.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.strip_chars" class="fn">strip_chars</a>( &self, pat: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.strip_chars_start" class="fn">strip_chars_start</a>( &self, pat: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.strip_chars_end" class="fn">strip_chars_end</a>( &self, pat: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.strip_prefix" class="fn">strip_prefix</a>( &self, prefix: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.strip_suffix" class="fn">strip_suffix</a>( &self, suffix: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.split_exact" class="fn">split_exact</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.split_exact_inclusive" class="fn">split_exact_inclusive</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.splitn" class="fn">splitn</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.split" class="fn">split</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.split_inclusive" class="fn">split_inclusive</a>( &self, by: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.extract_all_many" class="fn">extract_all_many</a>( &self, pat: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ListType.html" class="struct" title="struct polars::prelude::ListType">ListType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract each successive non-overlapping regex match in an individual string as an array.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.extract_groups" class="fn">extract_groups</a>( &self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extract all capture groups from pattern and return as a struct.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.count_matches" class="fn">count_matches</a>( &self, pat: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, literal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Count all successive non-overlapping regex matches.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.count_matches_many" class="fn">count_matches_many</a>( &self, pat: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, literal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Count all successive non-overlapping regex matches.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.to_lowercase" class="fn">to_lowercase</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

Modify the strings to their lowercase equivalent.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.to_uppercase" class="fn">to_uppercase</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

Modify the strings to their uppercase equivalent.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.to_titlecase" class="fn">to_titlecase</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

Modify the strings to their titlecase equivalent.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.concat" class="fn">concat</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

Concat with the values from a second StringChunked.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.str_reverse" class="fn">str_reverse</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

Reverses the string values

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.str_slice" class="fn">str_slice</a>( &self, offset: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, length: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Slice the string values.

Determines a substring starting from `offset` and with length `length` of each of the elements in `array`. `offset` can be negative, in which case the start counts from the end of the string.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.str_head" class="fn">str_head</a>(&self, n: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Slice the first `n` values of the string.

Determines a substring starting at the beginning of the string up to offset `n` of each element in `array`. `n` can be negative, in which case the slice ends `n` characters from the end of the string.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.str_tail" class="fn">str_tail</a>(&self, n: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Slice the last `n` values of the string.

Determines a substring starting at offset `n` of each element in `array`. `n` can be negative, in which case the slice begins `n` characters from the start of the string.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#method.str_escape_regex" class="fn">str_escape_regex</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>

Escapes all regular expression meta characters in the string.

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html#impl-StringNameSpaceImpl-for-ChunkedArray%3CStringType%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.StringNameSpaceImpl.html" class="trait" title="trait polars::prelude::StringNameSpaceImpl">StringNameSpaceImpl</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StringType.html" class="struct" title="struct polars::prelude::StringType">StringType</a>\>
