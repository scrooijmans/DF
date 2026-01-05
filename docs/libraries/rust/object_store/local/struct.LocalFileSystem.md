# Struct LocalFileSystem Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/local.rs.html#197-201" class="src">Source</a>

``` rust
pub struct LocalFileSystem { /* private fields */ }
```

Available on **crate feature `fs` and non-WebAssembly** only.

Expand description

Local filesystem storage providing an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") interface to files on local disk. Can optionally be created with a directory prefix

## <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#path-semantics" class="doc-anchor">§</a>Path Semantics

This implementation follows the [file URI](https://en.wikipedia.org/wiki/File_URI_scheme) scheme outlined in [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986). In particular paths are delimited by `/`

## <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#path-semantics-1" class="doc-anchor">§</a>Path Semantics

[`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem") will expose the path semantics of the underlying filesystem, which may have additional restrictions beyond those enforced by [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path").

For example:

- Windows forbids certain filenames, e.g. `COM0`,
- Windows forbids folders with trailing `.`
- Windows forbids certain ASCII characters, e.g. `<` or `|`
- OS X forbids filenames containing `:`
- Leading `-` are discouraged on Unix systems where they may be interpreted as CLI flags
- Filesystems may have restrictions on the maximum path or path segment length
- Filesystem support for non-ASCII characters is inconsistent

Additionally some filesystems, such as NTFS, are case-insensitive, whilst others like FAT don’t preserve case at all. Further some filesystems support non-unicode character sequences, such as unpaired UTF-16 surrogates, and [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem") will error on encountering such sequences.

Finally, filenames matching the regex `/.*#\d+/`, e.g. `foo.parquet#123`, are not supported by [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem") as they are used to provide atomic writes. Such files will be ignored for listing operations, and attempting to address such a file will error.

## <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#tokio-compatibility" class="doc-anchor">§</a>Tokio Compatibility

Tokio discourages performing blocking IO on a tokio worker thread, however, no major operating systems have stable async file APIs. Therefore if called from a tokio context, this will use [`tokio::runtime::Handle::spawn_blocking`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html#method.spawn_blocking "method tokio::runtime::handle::Handle::spawn_blocking") to dispatch IO to a blocking thread pool, much like `tokio::fs` does under-the-hood.

If not called from a tokio context, this will perform IO on the current thread with no additional complexity or overheads

## <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#symlinks" class="doc-anchor">§</a>Symlinks

[`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem") will follow symlinks as normal, however, it is worth noting:

- Broken symlinks will be silently ignored by listing operations
- No effort is made to prevent breaking symlinks when deleting files
- Symlinks that resolve to paths outside the root **will** be followed
- Mutating a file through one or more symlinks will mutate the underlying file
- Deleting a path that resolves to a symlink will only delete the symlink

## <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#cross-filesystem-copy" class="doc-anchor">§</a>Cross-Filesystem Copy

[`LocalFileSystem::copy`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.copy "method object_store::local::LocalFileSystem::copy") is implemented using [`std::fs::hard_link`](https://doc.rust-lang.org/nightly/std/fs/fn.hard_link.html "fn std::fs::hard_link"), and therefore does not support copying across filesystem boundaries.

## Implementations<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#impl-LocalFileSystem" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html" class="struct" title="struct object_store::local::LocalFileSystem">LocalFileSystem</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.new" class="fn">new</a>() -\> Self

Create new filesystem storage with no prefix

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.new_with_prefix" class="fn">new_with_prefix</a>(prefix: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>\>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<Self\>

Create new filesystem storage with `prefix` applied to all paths

Returns an error if the path does not exist

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.path_to_filesystem" class="fn">path_to_filesystem</a>(&self, location: &<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>

Return an absolute filesystem path of the given file location

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.with_automatic_cleanup" class="fn">with_automatic_cleanup</a>(self, automatic_cleanup: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Enable automatic cleanup of empty directories when deleting files

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#impl-Debug-for-LocalFileSystem" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html" class="struct" title="struct object_store::local::LocalFileSystem">LocalFileSystem</a>

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#impl-Default-for-LocalFileSystem" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html" class="struct" title="struct object_store::local::LocalFileSystem">LocalFileSystem</a>

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#impl-Display-for-LocalFileSystem" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html" class="struct" title="struct object_store::local::LocalFileSystem">LocalFileSystem</a>

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#impl-ObjectStore-for-LocalFileSystem" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a> for <a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html" class="struct" title="struct object_store::local::LocalFileSystem">LocalFileSystem</a>

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.put_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.put_opts" class="fn">put_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, payload: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>, opts: <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Save the provided `payload` to `location` with the given options

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.put_multipart_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.put_multipart_opts" class="fn">put_multipart_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, opts: <a href="https://docs.rs/object_store/latest/object_store/struct.PutMultipartOptions.html" class="struct" title="struct object_store::PutMultipartOptions">PutMultipartOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a multipart upload with options [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.put_multipart_opts)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.get_opts" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.get_opts" class="fn">get_opts</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, options: <a href="https://docs.rs/object_store/latest/object_store/struct.GetOptions.html" class="struct" title="struct object_store::GetOptions">GetOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a get request with options

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.get_range" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_range" class="fn">get_range</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, range: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Return the bytes that are stored at the specified location in the given byte range. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_range)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.get_ranges" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get_ranges" class="fn">get_ranges</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ranges: &'life2 \[<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Return the bytes that are stored at the specified location in the given byte ranges

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.delete" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.delete" class="fn">delete</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Delete the object at the specified location.

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.list" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list" class="fn">list</a>(&self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'static, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\>

List all the objects with the given prefix. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.list_with_offset" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.list_with_offset" class="fn">list_with_offset</a>( &self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>, offset: &<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'static, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\>

List all the objects with the given prefix and a location greater than `offset` [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.list_with_offset)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.list_with_delimiter" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list_with_delimiter" class="fn">list_with_delimiter</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.ListResult.html" class="struct" title="struct object_store::ListResult">ListResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

List objects with the given prefix and an implementation specific delimiter. Returns common prefixes (directories) in addition to object metadata. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.list_with_delimiter)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.copy" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy" class="fn">copy</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Copy an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.rename" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.rename" class="fn">rename</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Move an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.rename)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.copy_if_not_exists" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists" class="fn">copy_if_not_exists</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Copy an object from one path to another, only if destination is empty. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#tymethod.copy_if_not_exists)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.put" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put" class="fn">put</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, payload: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutResult.html" class="struct" title="struct object_store::PutResult">PutResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Save the provided bytes to the specified location [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.put_multipart" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart" class="fn">put_multipart</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.MultipartUpload.html" class="trait" title="trait object_store::MultipartUpload">MultipartUpload</a>\>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Perform a multipart upload [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.put_multipart)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.get" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.get" class="fn">get</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Return the bytes that are stored at the specified location.

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.head" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.head" class="fn">head</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, location: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Return the metadata for the specified location

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.delete_stream" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.delete_stream" class="fn">delete_stream</a>\<'a\>( &'a self, locations: <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'a, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>\>, ) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'a, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>\>\>

Delete all the objects at the specified locations [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.delete_stream)

<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#method.rename_if_not_exists" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.rename_if_not_exists" class="fn">rename_if_not_exists</a>\<'life0, 'life1, 'life2, 'async_trait\>( &'life0 self, from: &'life1 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, to: &'life2 <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait,

Move an object from one path to another in the same object store. [Read more](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html#method.rename_if_not_exists)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html#blanket-implementations" class="anchor">§</a>
