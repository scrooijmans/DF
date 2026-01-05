# Trait LayeredAccess Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/layer.rs.html#109-180" class="src">Source</a>

``` rust
pub trait LayeredAccess:
    Send
    + Sync
    + Debug
    + Unpin
    + 'static {
    type Inner: Access;
    type Reader: Read;
    type Writer: Write;
    type Lister: List;
    type Deleter: Delete;

    // Required methods
    fn inner(&self) -> &Self::Inner;
    fn read(
        &self,
        path: &str,
        args: OpRead,
    ) -> impl Future<Output = Result<(RpRead, Self::Reader)>> + MaybeSend;
    fn write(
        &self,
        path: &str,
        args: OpWrite,
    ) -> impl Future<Output = Result<(RpWrite, Self::Writer)>> + MaybeSend;
    fn delete(
        &self,
    ) -> impl Future<Output = Result<(RpDelete, Self::Deleter)>> + MaybeSend;
    fn list(
        &self,
        path: &str,
        args: OpList,
    ) -> impl Future<Output = Result<(RpList, Self::Lister)>> + MaybeSend;

    // Provided methods
    fn info(&self) -> Arc<AccessorInfo> { ... }
    fn create_dir(
        &self,
        path: &str,
        args: OpCreateDir,
    ) -> impl Future<Output = Result<RpCreateDir>> + MaybeSend { ... }
    fn copy(
        &self,
        from: &str,
        to: &str,
        args: OpCopy,
    ) -> impl Future<Output = Result<RpCopy>> + MaybeSend { ... }
    fn rename(
        &self,
        from: &str,
        to: &str,
        args: OpRename,
    ) -> impl Future<Output = Result<RpRename>> + MaybeSend { ... }
    fn stat(
        &self,
        path: &str,
        args: OpStat,
    ) -> impl Future<Output = Result<RpStat>> + MaybeSend { ... }
    fn presign(
        &self,
        path: &str,
        args: OpPresign,
    ) -> impl Future<Output = Result<RpPresign>> + MaybeSend { ... }
}
```

Expand description

LayeredAccess is layered accessor that forward all not implemented method to inner.

## Required Associated Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#required-associated-types" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Inner" class="associatedtype">Inner</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Reader" class="associatedtype">Reader</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Writer" class="associatedtype">Writer</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Lister" class="associatedtype">Lister</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Deleter" class="associatedtype">Deleter</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a>

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.inner" class="fn">inner</a>(&self) -\> &Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Inner" class="associatedtype" title="type opendal::raw::LayeredAccess::Inner">Inner</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.read" class="fn">read</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::LayeredAccess::Reader">Reader</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.write" class="fn">write</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpWrite.html" class="struct" title="struct opendal::raw::RpWrite">RpWrite</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::LayeredAccess::Writer">Writer</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.delete" class="fn">delete</a>( &self, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpDelete.html" class="struct" title="struct opendal::raw::RpDelete">RpDelete</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::LayeredAccess::Deleter">Deleter</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#tymethod.list" class="fn">list</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpList.html" class="struct" title="struct opendal::raw::RpList">RpList</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::LayeredAccess::Lister">Lister</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

## Provided Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#provided-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.info" class="fn">info</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.create_dir" class="fn">create_dir</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCreateDir.html" class="struct" title="struct opendal::raw::OpCreateDir">OpCreateDir</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCreateDir.html" class="struct" title="struct opendal::raw::RpCreateDir">RpCreateDir</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.copy" class="fn">copy</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCopy.html" class="struct" title="struct opendal::raw::OpCopy">OpCopy</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCopy.html" class="struct" title="struct opendal::raw::RpCopy">RpCopy</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.rename" class="fn">rename</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRename.html" class="struct" title="struct opendal::raw::OpRename">OpRename</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRename.html" class="struct" title="struct opendal::raw::RpRename">RpRename</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.stat" class="fn">stat</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpStat.html" class="struct" title="struct opendal::raw::RpStat">RpStat</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#method.presign" class="fn">presign</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#impl-LayeredAccess-for-MetricsAccessor%3CA,+I%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>, I: <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/trait.MetricsIntercept.html" class="trait" title="trait opendal::layers::observe::MetricsIntercept">MetricsIntercept</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html" class="trait" title="trait opendal::raw::LayeredAccess">LayeredAccess</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricsAccessor.html" class="struct" title="struct opendal::layers::observe::MetricsAccessor">MetricsAccessor</a>\<A, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Inner-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Inner" class="associatedtype">Inner</a> = A

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Reader-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Reader" class="associatedtype">Reader</a> = MetricsWrapper\<\<A as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::Access::Reader">Reader</a>, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Writer-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Writer" class="associatedtype">Writer</a> = MetricsWrapper\<\<A as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::Access::Writer">Writer</a>, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Lister-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Lister" class="associatedtype">Lister</a> = MetricsWrapper\<\<A as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::Access::Lister">Lister</a>, I\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Deleter-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = MetricsWrapper\<\<A as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::Access::Deleter">Deleter</a>, I\>
