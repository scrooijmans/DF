# Struct ListingTableUrl Copy item path

<a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource/url.rs.html#37" class="src">Source</a>

``` rust
pub struct ListingTableUrl { /* private fields */ }
```

Expand description

A parsed URL identifying files for a listing table, see [`ListingTableUrl::parse`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.parse "associated function datafusion::datasource::listing::ListingTableUrl::parse") for more information on the supported expressions

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.parse" class="fn">parse</a>(s: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Parse a provided string as a `ListingTableUrl`

A URL can either refer to a single object, or a collection of objects with a common prefix, with the presence of a trailing `/` indicating a collection.

For example, `file:///foo.txt` refers to the file at `/foo.txt`, whereas `file:///foo/` refers to all the files under the directory `/foo` and its subdirectories.

Similarly `s3://BUCKET/blob.csv` refers to `blob.csv` in the S3 bucket `BUCKET`, whereas `s3://BUCKET/foo/` refers to all objects with the prefix `foo/` in the S3 bucket `BUCKET`

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#url-encoding" class="doc-anchor">§</a>URL Encoding

URL paths are expected to be URL-encoded. That is, the URL for a file named `bar%2Efoo` would be `file:///bar%252Efoo`, as per the [URL](https://url.spec.whatwg.org/) specification.

It should be noted that some tools, such as the AWS CLI, take a different approach and instead interpret the URL path verbatim. For example the object `bar%2Efoo` would be addressed as `s3://BUCKET/bar%252Efoo` using [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") but `s3://BUCKET/bar%2Efoo` when using the aws-cli.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#paths-without-a-scheme" class="doc-anchor">§</a>Paths without a Scheme

If no scheme is provided, or the string is an absolute filesystem path as determined by [`std::path::Path::is_absolute`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html#method.is_absolute "method std::path::Path::is_absolute"), the string will be interpreted as a path on the local filesystem using the operating system’s standard path delimiter, i.e. `\` on Windows, `/` on Unix.

If the path contains any of `'?', '*', '['`, it will be considered a glob expression and resolved as described in the section below.

Otherwise, the path will be resolved to an absolute path based on the current working directory, and converted to a [file URI](https://en.wikipedia.org/wiki/File_URI_scheme).

If the path already exists in the local filesystem this will be used to determine if this [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") refers to a collection or a single object, otherwise the presence of a trailing path delimiter will be used to indicate a directory. For the avoidance of ambiguity it is recommended users always include trailing `/` when intending to refer to a directory.

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#glob-file-paths" class="doc-anchor">§</a>Glob File Paths

If no scheme is provided, and the path contains a glob expression, it will be resolved as follows.

The string up to the first path segment containing a glob expression will be extracted, and resolved in the same manner as a normal scheme-less path above.

The remaining string will be interpreted as a [`glob::Pattern`](https://docs.rs/glob/0.3.3/x86_64-unknown-linux-gnu/glob/struct.Pattern.html "struct glob::Pattern") and used as a filter when listing files from object storage

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.try_new" class="fn">try_new</a>( url: <a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, glob: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/glob/0.3.3/x86_64-unknown-linux-gnu/glob/struct.Pattern.html" class="struct" title="struct glob::Pattern">Pattern</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Creates a new [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") from a url and optional glob expression

[`Self::parse`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.parse "associated function datafusion::datasource::listing::ListingTableUrl::parse") supports glob expression only for file system paths. However, some applications may want to support glob expression for URLs with a scheme. The application can split the URL into a base URL and a glob expression and use this method to create a [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.scheme" class="fn">scheme</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the URL scheme

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.prefix" class="fn">prefix</a>(&self) -\> &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>

Return the URL path not excluding any glob expression

If [`Self::is_collection`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.is_collection "method datafusion::datasource::listing::ListingTableUrl::is_collection"), this is the listing prefix Otherwise, this is the path to the object

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.contains" class="fn">contains</a>(&self, path: &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ignore_subdirectory: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if `path` matches this [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.is_collection" class="fn">is_collection</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if `path` refers to a collection of objects

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.file_extension" class="fn">file_extension</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Returns the file extension of the last path segment if it exists

Examples:

``` rust
use datafusion_datasource::ListingTableUrl;
let url = ListingTableUrl::parse("file:///foo/bar.csv").unwrap();
assert_eq!(url.file_extension(), Some("csv"));
let url = ListingTableUrl::parse("file:///foo/bar").unwrap();
assert_eq!(url.file_extension(), None);
let url = ListingTableUrl::parse("file:///foo/bar.").unwrap();
assert_eq!(url.file_extension(), None);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.strip_prefix" class="fn">strip_prefix</a>\<'a, 'b\>( &'a self, path: &'b <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &'b <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> + 'a\>

where 'b: 'a,

Strips the prefix of this [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") from the provided path, returning an iterator of the remaining path segments

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.list_all_files" class="fn">list_all_files</a>\<'a\>( &'a self, ctx: &'a dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, store: &'a (dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a> + 'static), file_extension: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

List all files identified by this [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") for the provided `file_extension`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.as_str" class="fn">as_str</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns this [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") as a string

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.object_store" class="fn">object_store</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.ObjectStoreUrl.html" class="struct" title="struct datafusion::datasource::object_store::ObjectStoreUrl">ObjectStoreUrl</a>

Return the [`ObjectStoreUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.ObjectStoreUrl.html "struct datafusion::datasource::object_store::ObjectStoreUrl") for this [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.is_folder" class="fn">is_folder</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") points to the folder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.get_url" class="fn">get_url</a>(&self) -\> &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>

Return the `url` for [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.get_glob" class="fn">get_glob</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/glob/0.3.3/x86_64-unknown-linux-gnu/glob/struct.Pattern.html" class="struct" title="struct glob::Pattern">Pattern</a>\>

Return the `glob` for [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.with_glob" class="fn">with_glob</a>(self, glob: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a copy of current [`ListingTableUrl`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html "struct datafusion::datasource::listing::ListingTableUrl") with a specified `glob`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-AsRef%3CUrl%3E-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.as_ref-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-AsRef%3Cstr%3E-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-Clone-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-Debug-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-Display-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-Hash-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-PartialEq-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-Eq-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#impl-StructuralPartialEq-for-ListingTableUrl" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html#blanket-implementations" class="anchor">§</a>
