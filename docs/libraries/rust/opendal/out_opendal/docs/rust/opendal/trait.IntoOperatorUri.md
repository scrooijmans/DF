# Trait IntoOperatorUri Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/operator/uri.rs.html#144-147" class="src">Source</a>

``` rust
pub trait IntoOperatorUri {
    // Required method
    fn into_operator_uri(self) -> Result<OperatorUri>;
}
```

Expand description

Conversion trait that builds [`OperatorUri`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html "struct opendal::OperatorUri") from various inputs.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

Convert the input into an [`OperatorUri`](https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html "struct opendal::OperatorUri").

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-%26str" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#method.into_operator_uri" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-%26Uri" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for &Uri

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#method.into_operator_uri-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-String" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#method.into_operator_uri-2" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-Uri" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for Uri

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#method.into_operator_uri-3" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-(%26str,+O)" class="anchor">Â§</a>

### impl\<O, K, V\> <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for (&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, O)

where O: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>, K: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, V: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>,

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#method.into_operator_uri-4" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-(%26Uri,+O)" class="anchor">Â§</a>

### impl\<O, K, V\> <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for (&Uri, O)

where O: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>, K: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, V: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>,

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#method.into_operator_uri-5" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-(String,+O)" class="anchor">Â§</a>

### impl\<O, K, V\> <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for (<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, O)

where O: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>, K: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, V: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>,

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#method.into_operator_uri-6" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-(Uri,+O)" class="anchor">Â§</a>

### impl\<O, K, V\> <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for (Uri, O)

where O: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>, K: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, V: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>,

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#method.into_operator_uri-7" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#tymethod.into_operator_uri" class="fn">into_operator_uri</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>\>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-%26OperatorUri" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for &<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>

<a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html#impl-IntoOperatorUri-for-OperatorUri" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.IntoOperatorUri.html" class="trait" title="trait opendal::IntoOperatorUri">IntoOperatorUri</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorUri.html" class="struct" title="struct opendal::OperatorUri">OperatorUri</a>
