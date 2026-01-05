# Struct Backend Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/adapters/typed_kv/backend.rs.html#30-34" class="src">Source</a>

``` rust
pub struct Backend<S: Adapter> { /* private fields */ }
```

Expand description

The typed kv backend which implements Accessor for typed kv adapter.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#impl-Backend%3CS%3E" class="anchor">Â§</a>

### impl\<S\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Backend">Backend</a>\<S\>

where S: <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::typed_kv::Adapter">Adapter</a>,

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.new" class="fn">new</a>(kv: S) -\> Self

Create a new kv backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.with_root" class="fn">with_root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Configure root within this backend.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#impl-Access-for-Backend%3CS%3E" class="anchor">Â§</a>

### impl\<S: <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::typed_kv::Adapter">Adapter</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Backend">Backend</a>\<S\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#associatedtype.Reader" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a> = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

Reader is the associated reader returned in `read` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#associatedtype.Writer" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a> = KvWriter\<S\>

Writer is the associated writer returned in `write` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#associatedtype.Lister" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a> = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html" class="struct" title="struct opendal::raw::oio::HierarchyLister">HierarchyLister</a>\<KvLister\>

Lister is the associated lister returned in `list` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#associatedtype.Deleter" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html" class="struct" title="struct opendal::raw::oio::OneShotDeleter">OneShotDeleter</a>\<KvDeleter\<S\>\>

Deleter is the associated deleter returned in `delete` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.info" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#tymethod.info" class="fn">info</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>

Invoke the `info` operation to get metadata of accessor. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#tymethod.info)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.stat" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.stat" class="fn">stat</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpStat.html" class="struct" title="struct opendal::raw::RpStat">RpStat</a>\>

Invoke the `stat` operation on the specified path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.stat)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.read" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.read" class="fn">read</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::Access::Reader">Reader</a>)\>

Invoke the `read` operation on the specified path, returns a [`Reader`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html "struct opendal::Reader") if operate successful. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.read)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.write" class="fn">write</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>, ) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpWrite.html" class="struct" title="struct opendal::raw::RpWrite">RpWrite</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::Access::Writer">Writer</a>)\>

Invoke the `write` operation on the specified path, returns a written size if operate successful. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.write)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.delete" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.delete" class="fn">delete</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpDelete.html" class="struct" title="struct opendal::raw::RpDelete">RpDelete</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::Access::Deleter">Deleter</a>)\>

Invoke the `delete` operation on the specified path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.delete)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.list" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.list" class="fn">list</a>(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpList.html" class="struct" title="struct opendal::raw::RpList">RpList</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::Access::Lister">Lister</a>)\>

Invoke the `list` operation on the specified path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.list)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.create_dir" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.create_dir" class="fn">create_dir</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCreateDir.html" class="struct" title="struct opendal::raw::OpCreateDir">OpCreateDir</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCreateDir.html" class="struct" title="struct opendal::raw::RpCreateDir">RpCreateDir</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `create` operation on the specified path [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.create_dir)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.copy" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.copy" class="fn">copy</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCopy.html" class="struct" title="struct opendal::raw::OpCopy">OpCopy</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCopy.html" class="struct" title="struct opendal::raw::RpCopy">RpCopy</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `copy` operation on the specified `from` path and `to` path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.copy)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.rename" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.rename" class="fn">rename</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRename.html" class="struct" title="struct opendal::raw::OpRename">OpRename</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRename.html" class="struct" title="struct opendal::raw::RpRename">RpRename</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `rename` operation on the specified `from` path and `to` path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.rename)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.presign" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.presign" class="fn">presign</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `presign` operation on the specified path. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.presign)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#impl-Clone-for-Backend%3CS%3E" class="anchor">Â§</a>

### impl\<S: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::typed_kv::Adapter">Adapter</a>\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Backend">Backend</a>\<S\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Backend">Backend</a>\<S\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#impl-Debug-for-Backend%3CS%3E" class="anchor">Â§</a>

### impl\<S: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::typed_kv::Adapter">Adapter</a>\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Backend">Backend</a>\<S\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html#blanket-implementations" class="anchor">Â§</a>
