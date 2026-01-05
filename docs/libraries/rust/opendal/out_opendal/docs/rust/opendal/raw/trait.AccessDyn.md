# Trait AccessDyn Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/accessor.rs.html#264-315" class="src">Source</a>

``` rust
pub trait AccessDyn:
    Send
    + Sync
    + Debug
    + Unpin {
    // Required methods
    fn info_dyn(&self) -> Arc<AccessorInfo>;
    fn create_dir_dyn<'a>(
        &'a self,
        path: &'a str,
        args: OpCreateDir,
    ) -> BoxedFuture<'a, Result<RpCreateDir>>;
    fn stat_dyn<'a>(
        &'a self,
        path: &'a str,
        args: OpStat,
    ) -> BoxedFuture<'a, Result<RpStat>>;
    fn read_dyn<'a>(
        &'a self,
        path: &'a str,
        args: OpRead,
    ) -> BoxedFuture<'a, Result<(RpRead, Reader)>>;
    fn write_dyn<'a>(
        &'a self,
        path: &'a str,
        args: OpWrite,
    ) -> BoxedFuture<'a, Result<(RpWrite, Writer)>>;
    fn delete_dyn(&self) -> BoxedFuture<'_, Result<(RpDelete, Deleter)>>;
    fn list_dyn<'a>(
        &'a self,
        path: &'a str,
        args: OpList,
    ) -> BoxedFuture<'a, Result<(RpList, Lister)>>;
    fn copy_dyn<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        args: OpCopy,
    ) -> BoxedFuture<'a, Result<RpCopy>>;
    fn rename_dyn<'a>(
        &'a self,
        from: &'a str,
        to: &'a str,
        args: OpRename,
    ) -> BoxedFuture<'a, Result<RpRename>>;
    fn presign_dyn<'a>(
        &'a self,
        path: &'a str,
        args: OpPresign,
    ) -> BoxedFuture<'a, Result<RpPresign>>;
}
```

Expand description

`AccessDyn` is the dyn version of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") make it possible to use as `Box<dyn AccessDyn>`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.info_dyn" class="fn">info_dyn</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>

Dyn version of [`Accessor::info`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.info "method alloc::sync::Arc::info")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.create_dir_dyn" class="fn">create_dir_dyn</a>\<'a\>( &'a self, path: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCreateDir.html" class="struct" title="struct opendal::raw::OpCreateDir">OpCreateDir</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'a, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCreateDir.html" class="struct" title="struct opendal::raw::RpCreateDir">RpCreateDir</a>\>\>

Dyn version of [`Accessor::create_dir`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.create_dir "method alloc::sync::Arc::create_dir")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.stat_dyn" class="fn">stat_dyn</a>\<'a\>( &'a self, path: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'a, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpStat.html" class="struct" title="struct opendal::raw::RpStat">RpStat</a>\>\>

Dyn version of [`Accessor::stat`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.stat "method alloc::sync::Arc::stat")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.read_dyn" class="fn">read_dyn</a>\<'a\>( &'a self, path: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'a, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Reader.html" class="type" title="type opendal::raw::oio::Reader">Reader</a>)\>\>

Dyn version of [`Accessor::read`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.read "method alloc::sync::Arc::read")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.write_dyn" class="fn">write_dyn</a>\<'a\>( &'a self, path: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'a, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpWrite.html" class="struct" title="struct opendal::raw::RpWrite">RpWrite</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Writer.html" class="type" title="type opendal::raw::oio::Writer">Writer</a>)\>\>

Dyn version of [`Accessor::write`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.write "method alloc::sync::Arc::write")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.delete_dyn" class="fn">delete_dyn</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'\_, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpDelete.html" class="struct" title="struct opendal::raw::RpDelete">RpDelete</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Deleter.html" class="type" title="type opendal::raw::oio::Deleter">Deleter</a>)\>\>

Dyn version of [`Accessor::delete`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.delete "method alloc::sync::Arc::delete")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.list_dyn" class="fn">list_dyn</a>\<'a\>( &'a self, path: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'a, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpList.html" class="struct" title="struct opendal::raw::RpList">RpList</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Lister.html" class="type" title="type opendal::raw::oio::Lister">Lister</a>)\>\>

Dyn version of [`Accessor::list`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.list "method alloc::sync::Arc::list")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.copy_dyn" class="fn">copy_dyn</a>\<'a\>( &'a self, from: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCopy.html" class="struct" title="struct opendal::raw::OpCopy">OpCopy</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'a, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCopy.html" class="struct" title="struct opendal::raw::RpCopy">RpCopy</a>\>\>

Dyn version of [`Accessor::copy`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.copy "method alloc::sync::Arc::copy")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.rename_dyn" class="fn">rename_dyn</a>\<'a\>( &'a self, from: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRename.html" class="struct" title="struct opendal::raw::OpRename">OpRename</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'a, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRename.html" class="struct" title="struct opendal::raw::RpRename">RpRename</a>\>\>

Dyn version of [`Accessor::rename`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.rename "method alloc::sync::Arc::rename")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#tymethod.presign_dyn" class="fn">presign_dyn</a>\<'a\>( &'a self, path: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'a, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>\>\>

Dyn version of [`Accessor::presign`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html#method.presign "method alloc::sync::Arc::presign")

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#impl-Access-for-dyn+AccessDyn" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> for dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html" class="trait" title="trait opendal::raw::AccessDyn">AccessDyn</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#associatedtype.Reader" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html" class="trait" title="trait opendal::raw::oio::ReadDyn">ReadDyn</a>\>

Reader is the associated reader returned in `read` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#associatedtype.Writer" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html" class="trait" title="trait opendal::raw::oio::WriteDyn">WriteDyn</a>\>

Writer is the associated writer returned in `write` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#associatedtype.Deleter" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html" class="trait" title="trait opendal::raw::oio::DeleteDyn">DeleteDyn</a>\>

Deleter is the associated deleter returned in `delete` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#associatedtype.Lister" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html" class="trait" title="trait opendal::raw::oio::ListDyn">ListDyn</a>\>

Lister is the associated lister returned in `list` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.info" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#tymethod.info" class="fn">info</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>

Invoke the `info` operation to get metadata of accessor. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#tymethod.info)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.create_dir" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.create_dir" class="fn">create_dir</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCreateDir.html" class="struct" title="struct opendal::raw::OpCreateDir">OpCreateDir</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCreateDir.html" class="struct" title="struct opendal::raw::RpCreateDir">RpCreateDir</a>\>

Invoke the `create` operation on the specified path [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.create_dir)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.stat" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.stat" class="fn">stat</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpStat.html" class="struct" title="struct opendal::raw::RpStat">RpStat</a>\>

Invoke the `stat` operation on the specified path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.stat)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.read" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.read" class="fn">read</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::Access::Reader">Reader</a>)\>

Invoke the `read` operation on the specified path, returns a [`Reader`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html "struct opendal::Reader") if operate successful. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.read)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.write" class="fn">write</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpWrite.html" class="struct" title="struct opendal::raw::RpWrite">RpWrite</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::Access::Writer">Writer</a>)\>

Invoke the `write` operation on the specified path, returns a written size if operate successful. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.write)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.delete" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.delete" class="fn">delete</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpDelete.html" class="struct" title="struct opendal::raw::RpDelete">RpDelete</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::Access::Deleter">Deleter</a>)\>

Invoke the `delete` operation on the specified path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.delete)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.list" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.list" class="fn">list</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpList.html" class="struct" title="struct opendal::raw::RpList">RpList</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::Access::Lister">Lister</a>)\>

Invoke the `list` operation on the specified path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.list)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.copy" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.copy" class="fn">copy</a>(&self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCopy.html" class="struct" title="struct opendal::raw::OpCopy">OpCopy</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCopy.html" class="struct" title="struct opendal::raw::RpCopy">RpCopy</a>\>

Invoke the `copy` operation on the specified `from` path and `to` path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.copy)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.rename" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.rename" class="fn">rename</a>(&self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRename.html" class="struct" title="struct opendal::raw::OpRename">OpRename</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRename.html" class="struct" title="struct opendal::raw::RpRename">RpRename</a>\>

Invoke the `rename` operation on the specified `from` path and `to` path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.rename)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#method.presign" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.presign" class="fn">presign</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>\>

Invoke the `presign` operation on the specified path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.presign)

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html#impl-AccessDyn-for-A" class="anchor">Â§</a>

### impl\<A\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html" class="trait" title="trait opendal::raw::AccessDyn">AccessDyn</a> for A

where A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\<Reader = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Reader.html" class="type" title="type opendal::raw::oio::Reader">Reader</a>, Writer = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Writer.html" class="type" title="type opendal::raw::oio::Writer">Writer</a>, Lister = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Lister.html" class="type" title="type opendal::raw::oio::Lister">Lister</a>, Deleter = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Deleter.html" class="type" title="type opendal::raw::oio::Deleter">Deleter</a>\> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,
