# Struct TagSet Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/tags.rs.html#25" class="src">Source</a>

``` rust
pub struct TagSet(/* private fields */);
```

Expand description

A collection of key value pairs used to annotate objects

<https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-tagging.html> <https://learn.microsoft.com/en-us/rest/api/storageservices/set-blob-tags>

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-TagSet" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.push" class="fn">push</a>(&mut self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Append a key value pair to this [`TagSet`](https://docs.rs/object_store/latest/object_store/struct.TagSet.html "struct object_store::TagSet")

Stores have different restrictions on what characters are permitted, for portability it is recommended applications use no more than 10 tags, and stick to alphanumeric characters, and `+ - = . _ : /`

<https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObjectTagging.html> <https://learn.microsoft.com/en-us/rest/api/storageservices/set-blob-tags?tabs=azure-ad#request-body>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.encoded" class="fn">encoded</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return this [`TagSet`](https://docs.rs/object_store/latest/object_store/struct.TagSet.html "struct object_store::TagSet") as a URL-encoded string

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-Clone-for-TagSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-Debug-for-TagSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-Default-for-TagSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-From%3CTagSet%3E-for-PutMultipartOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutMultipartOptions.html" class="struct" title="struct object_store::PutMultipartOptions">PutMultipartOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(tags: <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-From%3CTagSet%3E-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(tags: <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-PartialEq-for-TagSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-Eq-for-TagSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#impl-StructuralPartialEq-for-TagSet" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html" class="struct" title="struct object_store::TagSet">TagSet</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.TagSet.html#blanket-implementations" class="anchor">§</a>
