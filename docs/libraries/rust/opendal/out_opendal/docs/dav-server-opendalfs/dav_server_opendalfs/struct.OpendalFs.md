# Struct OpendalFs Copy item path

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/src/dav_server_opendalfs/fs.rs.html#60-62" class="src">Source</a>

``` rust
pub struct OpendalFs {
    pub op: Operator,
}
```

Expand description

OpendalFs is a `DavFileSystem` implementation for opendal.

``` rust
use anyhow::Result;
use dav_server::davpath::DavPath;
use dav_server::fs::DavFileSystem;
use dav_server_opendalfs::OpendalFs;
use opendal::services::Memory;
use opendal::Operator;

#[tokio::test]
async fn test() -> Result<()> {
    let op = Operator::new(Memory::default())?.finish();

    let webdavfs = OpendalFs::new(op);

    let metadata = webdavfs
        .metadata(&DavPath::new("/").unwrap())
        .await
        .unwrap();
    println!("{}", metadata.is_dir());

    Ok(())
}
```

## Fields<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#structfield.op" class="anchor field">Â§</a>`op: Operator`

## Implementations<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#impl-OpendalFs" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html" class="struct" title="struct dav_server_opendalfs::OpendalFs">OpendalFs</a>

#### pub fn <a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.new" class="fn">new</a>(op: Operator) -\> <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html" class="struct" title="struct dav_server_opendalfs::OpendalFs">OpendalFs</a>\>

Create a new `OpendalFs` instance.

## Trait Implementations<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#impl-Clone-for-OpendalFs" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html" class="struct" title="struct dav_server_opendalfs::OpendalFs">OpendalFs</a>

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html" class="struct" title="struct dav_server_opendalfs::OpendalFs">OpendalFs</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#impl-DavFileSystem-for-OpendalFs" class="anchor">Â§</a>

### impl DavFileSystem for <a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html" class="struct" title="struct dav_server_opendalfs::OpendalFs">OpendalFs</a>

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.open" class="anchor">Â§</a>

#### fn open\<'a\>( &'a self, path: &'a DavPath, options: OpenOptions, ) -\> FsFuture\<'a, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn DavFile\>\>

Open a file.

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.read_dir" class="anchor">Â§</a>

#### fn read_dir\<'a\>( &'a self, path: &'a DavPath, \_meta: ReadDirMeta, ) -\> FsFuture\<'a, FsStream\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn DavDirEntry\>\>\>

Lists entries within a directory.

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.metadata" class="anchor">Â§</a>

#### fn metadata\<'a\>( &'a self, path: &'a DavPath, ) -\> FsFuture\<'a, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn DavMetaData\>\>

Return the metadata of a file or directory.

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.create_dir" class="anchor">Â§</a>

#### fn create_dir\<'a\>(&'a self, path: &'a DavPath) -\> FsFuture\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Create a directory. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.remove_dir" class="anchor">Â§</a>

#### fn remove_dir\<'a\>(&'a self, path: &'a DavPath) -\> FsFuture\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Remove a directory. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.remove_file" class="anchor">Â§</a>

#### fn remove_file\<'a\>(&'a self, path: &'a DavPath) -\> FsFuture\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Remove a file. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.rename" class="anchor">Â§</a>

#### fn rename\<'a\>(&'a self, from: &'a DavPath, to: &'a DavPath) -\> FsFuture\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Rename a file or directory. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.copy" class="anchor">Â§</a>

#### fn copy\<'a\>(&'a self, from: &'a DavPath, to: &'a DavPath) -\> FsFuture\<'a, <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Copy a file. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.symlink_metadata" class="anchor">Â§</a>

#### fn symlink_metadata\<'a\>( &'a self, path: &'a DavPath, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn DavMetaData\>, FsError\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>

Return the metadata of a file, directory or symbolic link. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.have_props" class="anchor">Â§</a>

#### fn have_props\<'a\>( &'a self, path: &'a DavPath, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>

Indicator that tells if this filesystem driver supports DAV properties. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.patch_props" class="anchor">Â§</a>

#### fn patch_props\<'a\>( &'a self, path: &'a DavPath, patch: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, DavProp)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(StatusCode, DavProp)\>, FsError\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>

Patch the DAV properties of a node (add/remove props). Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.get_props" class="anchor">Â§</a>

#### fn get_props\<'a\>( &'a self, path: &'a DavPath, do_content: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<DavProp\>, FsError\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>

List/get the DAV properties of a node. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.get_prop" class="anchor">Â§</a>

#### fn get_prop\<'a\>( &'a self, path: &'a DavPath, prop: DavProp, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>, FsError\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'a\>\>

Get one specific named property of a node. Read more

<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#method.get_quota" class="anchor">Â§</a>

#### fn get_quota( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>), FsError\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + '\_\>\>

Get quota of this filesystem (used/total space). Read more

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/dav-server-opendalfs/dav_server_opendalfs/struct.OpendalFs.html#blanket-implementations" class="anchor">Â§</a>
