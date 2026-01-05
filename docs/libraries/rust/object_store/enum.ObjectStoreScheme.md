# Enum ObjectStoreScheme Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/parse.rs.html#66-79" class="src">Source</a>

``` rust
#[non_exhaustive]pub enum ObjectStoreScheme {
    Local,
    Memory,
    AmazonS3,
    GoogleCloudStorage,
    MicrosoftAzure,
    Http,
}
```

Expand description

Recognizes various URL formats, identifying the relevant [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore")

See [`ObjectStoreScheme::parse`](https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#method.parse "associated function object_store::ObjectStoreScheme::parse") for more details

## <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#supported-formats" class="doc-anchor">§</a>Supported formats:

- `file:///path/to/my/file` -\> [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem")
- `memory:///` -\> [`InMemory`](https://docs.rs/object_store/latest/object_store/memory/struct.InMemory.html "struct object_store::memory::InMemory")
- `s3://bucket/path` -\> [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3") (also supports `s3a`)
- `gs://bucket/path` -\> [`GoogleCloudStorage`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html "struct object_store::gcp::GoogleCloudStorage")
- `az://account/container/path` -\> [`MicrosoftAzure`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html "struct object_store::azure::MicrosoftAzure") (also supports `adl`, `azure`, `abfs`, `abfss`)
- `http://mydomain/path` -\> [`HttpStore`](https://docs.rs/object_store/latest/object_store/http/struct.HttpStore.html "struct object_store::http::HttpStore")
- `https://mydomain/path` -\> [`HttpStore`](https://docs.rs/object_store/latest/object_store/http/struct.HttpStore.html "struct object_store::http::HttpStore")

There are also special cases for AWS and Azure for `https://{host?}/path` paths:

- `dfs.core.windows.net`, `blob.core.windows.net`, `dfs.fabric.microsoft.com`, `blob.fabric.microsoft.com` -\> [`MicrosoftAzure`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html "struct object_store::azure::MicrosoftAzure")
- `amazonaws.com` -\> [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3")
- `r2.cloudflarestorage.com` -\> [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3")

## Variants (Non-exhaustive)<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#variants" class="anchor">§</a>

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#variant.Local" class="anchor">§</a>

### Local

Url corresponding to [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem")

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#variant.Memory" class="anchor">§</a>

### Memory

Url corresponding to [`InMemory`](https://docs.rs/object_store/latest/object_store/memory/struct.InMemory.html "struct object_store::memory::InMemory")

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#variant.AmazonS3" class="anchor">§</a>

### AmazonS3

Url corresponding to [`AmazonS3`](https://docs.rs/object_store/latest/object_store/aws/struct.AmazonS3.html "struct object_store::aws::AmazonS3")

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#variant.GoogleCloudStorage" class="anchor">§</a>

### GoogleCloudStorage

Url corresponding to [`GoogleCloudStorage`](https://docs.rs/object_store/latest/object_store/gcp/struct.GoogleCloudStorage.html "struct object_store::gcp::GoogleCloudStorage")

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#variant.MicrosoftAzure" class="anchor">§</a>

### MicrosoftAzure

Url corresponding to [`MicrosoftAzure`](https://docs.rs/object_store/latest/object_store/azure/struct.MicrosoftAzure.html "struct object_store::azure::MicrosoftAzure")

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#variant.Http" class="anchor">§</a>

### Http

Url corresponding to [`HttpStore`](https://docs.rs/object_store/latest/object_store/http/struct.HttpStore.html "struct object_store::http::HttpStore")

## Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#impl-ObjectStoreScheme" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#method.parse" class="fn">parse</a>(url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(Self, <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>), Error\>

Create an [`ObjectStoreScheme`](https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html "enum object_store::ObjectStoreScheme") from the provided [`Url`](https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html "struct url::Url")

Returns the [`ObjectStoreScheme`](https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html "enum object_store::ObjectStoreScheme") and the remaining [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path")

##### <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#example" class="doc-anchor">§</a>Example

``` rust
let url: Url = "file:///path/to/my/file".parse().unwrap();
let (scheme, path) = ObjectStoreScheme::parse(&url).unwrap();
assert_eq!(scheme, ObjectStoreScheme::Local);
assert_eq!(path.as_ref(), "path/to/my/file");

let url: Url = "https://blob.core.windows.net/container/path/to/my/file".parse().unwrap();
let (scheme, path) = ObjectStoreScheme::parse(&url).unwrap();
assert_eq!(scheme, ObjectStoreScheme::MicrosoftAzure);
assert_eq!(path.as_ref(), "path/to/my/file");

let url: Url = "https://example.com/path/to/my/file".parse().unwrap();
let (scheme, path) = ObjectStoreScheme::parse(&url).unwrap();
assert_eq!(scheme, ObjectStoreScheme::Http);
assert_eq!(path.as_ref(), "path/to/my/file");
```

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#impl-Clone-for-ObjectStoreScheme" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#impl-Debug-for-ObjectStoreScheme" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#impl-PartialEq-for-ObjectStoreScheme" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#impl-Eq-for-ObjectStoreScheme" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>

<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#impl-StructuralPartialEq-for-ObjectStoreScheme" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html" class="enum" title="enum object_store::ObjectStoreScheme">ObjectStoreScheme</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html#blanket-implementations" class="anchor">§</a>
