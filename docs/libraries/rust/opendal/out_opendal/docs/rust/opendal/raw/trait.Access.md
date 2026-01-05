# Trait Access Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/accessor.rs.html#60-260" class="src">Source</a>

``` rust
pub trait Access:
    Send
    + Sync
    + Debug
    + Unpin
    + 'static {
    type Reader: Read;
    type Writer: Write;
    type Lister: List;
    type Deleter: Delete;

    // Required method
    fn info(&self) -> Arc<AccessorInfo>;

    // Provided methods
    fn create_dir(
        &self,
        path: &str,
        args: OpCreateDir,
    ) -> impl Future<Output = Result<RpCreateDir>> + MaybeSend { ... }
    fn stat(
        &self,
        path: &str,
        args: OpStat,
    ) -> impl Future<Output = Result<RpStat>> + MaybeSend { ... }
    fn read(
        &self,
        path: &str,
        args: OpRead,
    ) -> impl Future<Output = Result<(RpRead, Self::Reader)>> + MaybeSend { ... }
    fn write(
        &self,
        path: &str,
        args: OpWrite,
    ) -> impl Future<Output = Result<(RpWrite, Self::Writer)>> + MaybeSend { ... }
    fn delete(
        &self,
    ) -> impl Future<Output = Result<(RpDelete, Self::Deleter)>> + MaybeSend { ... }
    fn list(
        &self,
        path: &str,
        args: OpList,
    ) -> impl Future<Output = Result<(RpList, Self::Lister)>> + MaybeSend { ... }
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
    fn presign(
        &self,
        path: &str,
        args: OpPresign,
    ) -> impl Future<Output = Result<RpPresign>> + MaybeSend { ... }
}
```

Expand description

Underlying trait of all backends for implementers.

The actual data access of storage service happens in Accessor layer. Every storage supported by OpenDAL must implement [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") but not all methods of [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") will be implemented according to how the storage service is.

For example, user can not modify the content from one HTTP file server directly. So [`Http`](https://opendal.apache.org/docs/rust/opendal/services/struct.Http.html "struct opendal::services::Http") implements and provides only read related actions.

[`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") gives default implementation for all methods which will raise [`ErrorKind::Unsupported`](https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.Unsupported "variant opendal::ErrorKind::Unsupported") error. And what action this [`Access`](https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html "trait opendal::raw::Access") supports will be pointed out in [`AccessorInfo`](https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html "struct opendal::raw::AccessorInfo").

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#note" class="doc-anchor">Â§</a>Note

Visit [`internals`](https://opendal.apache.org/docs/rust/opendal/docs/internals/index.html "mod opendal::docs::internals") for more tutorials.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#operations" class="doc-anchor">Â§</a>Operations

- Path in args will all be normalized into the same style, services should handle them based on servicesâ€™ requirement.
  - Path that ends with `/` means itâ€™s Dir, otherwise, itâ€™s File.
  - Root dir is `/`
  - Path will never be empty.
- Operations without capability requirement like `metadata`, `create` are basic operations.
  - All services must implement them.
  - Use `unimplemented!()` if not implemented or canâ€™t implement.
- Operations with capability requirement like `presign` are optional operations.
  - Services can implement them based on services capabilities.
  - The default implementation should return [`ErrorKind::Unsupported`](https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html#variant.Unsupported "variant opendal::ErrorKind::Unsupported").

## Required Associated Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#required-associated-types" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>

Reader is the associated reader returned in `read` operation.

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>

Writer is the associated writer returned in `write` operation.

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>

Lister is the associated lister returned in `list` operation.

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a>: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a>

Deleter is the associated deleter returned in `delete` operation.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#tymethod.info" class="fn">info</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>

Invoke the `info` operation to get metadata of accessor.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#notes" class="doc-anchor">Â§</a>Notes

This function is required to be implemented.

By returning AccessorInfo, underlying services can declare some useful information about itself.

- scheme: declare the scheme of backend.
- capabilities: declare the capabilities of current backend.

## Provided Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#provided-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.create_dir" class="fn">create_dir</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCreateDir.html" class="struct" title="struct opendal::raw::OpCreateDir">OpCreateDir</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCreateDir.html" class="struct" title="struct opendal::raw::RpCreateDir">RpCreateDir</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `create` operation on the specified path

Require [`Capability::create_dir`](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.create_dir "field opendal::Capability::create_dir")

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#behavior" class="doc-anchor">Â§</a>Behavior

- Input path MUST match with EntryMode, DONâ€™T NEED to check mode.
- Create on existing dir SHOULD succeed.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.stat" class="fn">stat</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpStat.html" class="struct" title="struct opendal::raw::RpStat">RpStat</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `stat` operation on the specified path.

Require [`Capability::stat`](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.stat "field opendal::Capability::stat")

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#behavior-1" class="doc-anchor">Â§</a>Behavior

- `stat` empty path means stat backendâ€™s root path.
- `stat` a path endswith â€œ/â€? means stating a dir.
- `mode` and `content_length` must be set.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.read" class="fn">read</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::Access::Reader">Reader</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `read` operation on the specified path, returns a [`Reader`](https://opendal.apache.org/docs/rust/opendal/struct.Reader.html "struct opendal::Reader") if operate successful.

Require [`Capability::read`](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.read "field opendal::Capability::read")

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#behavior-2" class="doc-anchor">Â§</a>Behavior

- Input path MUST be file path, DONâ€™T NEED to check mode.
- The returning content length may be smaller than the range specified.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.write" class="fn">write</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpWrite.html" class="struct" title="struct opendal::raw::RpWrite">RpWrite</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::Access::Writer">Writer</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `write` operation on the specified path, returns a written size if operate successful.

Require [`Capability::write`](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.write "field opendal::Capability::write")

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#behavior-3" class="doc-anchor">Â§</a>Behavior

- Input path MUST be file path, DONâ€™T NEED to check mode.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.delete" class="fn">delete</a>( &self, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpDelete.html" class="struct" title="struct opendal::raw::RpDelete">RpDelete</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::Access::Deleter">Deleter</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `delete` operation on the specified path.

Require [`Capability::delete`](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.delete "field opendal::Capability::delete")

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#behavior-4" class="doc-anchor">Â§</a>Behavior

- `delete` is an idempotent operation, itâ€™s safe to call `Delete` on the same path multiple times.
- `delete` SHOULD return `Ok(())` if the path is deleted successfully or not exist.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.list" class="fn">list</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpList.html" class="struct" title="struct opendal::raw::RpList">RpList</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::Access::Lister">Lister</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `list` operation on the specified path.

Require [`Capability::list`](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.list "field opendal::Capability::list")

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#behavior-5" class="doc-anchor">Â§</a>Behavior

- Input path MUST be dir path, DONâ€™T NEED to check mode.
- List non-exist dir should return Empty.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.copy" class="fn">copy</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCopy.html" class="struct" title="struct opendal::raw::OpCopy">OpCopy</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCopy.html" class="struct" title="struct opendal::raw::RpCopy">RpCopy</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `copy` operation on the specified `from` path and `to` path.

Require [Capability::copy](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.copy "field opendal::Capability::copy")

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#behaviour" class="doc-anchor">Â§</a>Behaviour

- `from` and `to` MUST be file path, DONâ€™T NEED to check mode.
- Copy on existing file SHOULD succeed.
- Copy on existing file SHOULD overwrite and truncate.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.rename" class="fn">rename</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRename.html" class="struct" title="struct opendal::raw::OpRename">OpRename</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRename.html" class="struct" title="struct opendal::raw::RpRename">RpRename</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `rename` operation on the specified `from` path and `to` path.

Require [Capability::rename](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.rename "field opendal::Capability::rename")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.presign" class="fn">presign</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Invoke the `presign` operation on the specified path.

Require [`Capability::presign`](https://opendal.apache.org/docs/rust/opendal/struct.Capability.html#structfield.presign "field opendal::Capability::presign")

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#behavior-6" class="doc-anchor">Â§</a>Behavior

- This API is optional, return [`std::io::ErrorKind::Unsupported`](https://doc.rust-lang.org/nightly/std/io/error/enum.ErrorKind.html#variant.Unsupported "variant std::io::error::ErrorKind::Unsupported") if not supported.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#impl-Access-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

Dummy implementation of accessor.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter-1" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.info" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#tymethod.info" class="fn">info</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#impl-Access-for-Arc%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>

All functions in `Accessor` only requires `&self`, so itâ€™s safe to implement `Accessor` for `Arc<impl Access>`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader-2" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a> = \<T as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::Access::Reader">Reader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer-2" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a> = \<T as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::Access::Writer">Writer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister-2" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a> = \<T as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::Access::Lister">Lister</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter-2" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = \<T as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::Access::Deleter">Deleter</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.info-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#tymethod.info" class="fn">info</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.create_dir-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.create_dir" class="fn">create_dir</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCreateDir.html" class="struct" title="struct opendal::raw::OpCreateDir">OpCreateDir</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCreateDir.html" class="struct" title="struct opendal::raw::RpCreateDir">RpCreateDir</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.stat-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.stat" class="fn">stat</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpStat.html" class="struct" title="struct opendal::raw::OpStat">OpStat</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpStat.html" class="struct" title="struct opendal::raw::RpStat">RpStat</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.read-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.read" class="fn">read</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRead.html" class="struct" title="struct opendal::raw::OpRead">OpRead</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRead.html" class="struct" title="struct opendal::raw::RpRead">RpRead</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::Access::Reader">Reader</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.write-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.write" class="fn">write</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpWrite.html" class="struct" title="struct opendal::raw::OpWrite">OpWrite</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpWrite.html" class="struct" title="struct opendal::raw::RpWrite">RpWrite</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::Access::Writer">Writer</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.delete-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.delete" class="fn">delete</a>( &self, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpDelete.html" class="struct" title="struct opendal::raw::RpDelete">RpDelete</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::Access::Deleter">Deleter</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.list-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.list" class="fn">list</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpList.html" class="struct" title="struct opendal::raw::OpList">OpList</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpList.html" class="struct" title="struct opendal::raw::RpList">RpList</a>, Self::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::Access::Lister">Lister</a>)\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.copy-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.copy" class="fn">copy</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpCopy.html" class="struct" title="struct opendal::raw::OpCopy">OpCopy</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpCopy.html" class="struct" title="struct opendal::raw::RpCopy">RpCopy</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.rename-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.rename" class="fn">rename</a>( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpRename.html" class="struct" title="struct opendal::raw::OpRename">OpRename</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpRename.html" class="struct" title="struct opendal::raw::RpRename">RpRename</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.presign-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#method.presign" class="fn">presign</a>( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpPresign.html" class="struct" title="struct opendal::raw::OpPresign">OpPresign</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.RpPresign.html" class="struct" title="struct opendal::raw::RpPresign">RpPresign</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#impl-Access-for-dyn+AccessDyn" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> for dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.AccessDyn.html" class="trait" title="trait opendal::raw::AccessDyn">AccessDyn</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader-3" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html" class="trait" title="trait opendal::raw::oio::ReadDyn">ReadDyn</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer-3" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html" class="trait" title="trait opendal::raw::oio::WriteDyn">WriteDyn</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter-3" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html" class="trait" title="trait opendal::raw::oio::DeleteDyn">DeleteDyn</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister-3" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a> = <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html" class="trait" title="trait opendal::raw::oio::ListDyn">ListDyn</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#impl-Access-for-L" class="anchor">Â§</a>

### impl\<L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html" class="trait" title="trait opendal::raw::LayeredAccess">LayeredAccess</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> for L

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader-4" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a> = \<L as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html" class="trait" title="trait opendal::raw::LayeredAccess">LayeredAccess</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Reader" class="associatedtype" title="type opendal::raw::LayeredAccess::Reader">Reader</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer-4" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a> = \<L as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html" class="trait" title="trait opendal::raw::LayeredAccess">LayeredAccess</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Writer" class="associatedtype" title="type opendal::raw::LayeredAccess::Writer">Writer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister-4" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a> = \<L as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html" class="trait" title="trait opendal::raw::LayeredAccess">LayeredAccess</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Lister" class="associatedtype" title="type opendal::raw::LayeredAccess::Lister">Lister</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter-4" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = \<L as <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html" class="trait" title="trait opendal::raw::LayeredAccess">LayeredAccess</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.LayeredAccess.html#associatedtype.Deleter" class="associatedtype" title="type opendal::raw::LayeredAccess::Deleter">Deleter</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#impl-Access-for-Backend%3CS%3E" class="anchor">Â§</a>

### impl\<S: <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::kv::Adapter">Adapter</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> for opendal::raw::adapters::kv::<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::kv::Backend">Backend</a>\<S\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader-5" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a> = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer-5" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a> = KvWriter\<S\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister-5" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a> = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html" class="struct" title="struct opendal::raw::oio::HierarchyLister">HierarchyLister</a>\<KvLister\<\<S as <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::kv::Adapter">Adapter</a>\>::<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/kv/trait.Adapter.html#associatedtype.Scanner" class="associatedtype" title="type opendal::raw::adapters::kv::Adapter::Scanner">Scanner</a>\>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter-5" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html" class="struct" title="struct opendal::raw::oio::OneShotDeleter">OneShotDeleter</a>\<KvDeleter\<S\>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#impl-Access-for-Backend%3CS%3E-1" class="anchor">Â§</a>

### impl\<S: <a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/trait.Adapter.html" class="trait" title="trait opendal::raw::adapters::typed_kv::Adapter">Adapter</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a> for opendal::raw::adapters::typed_kv::<a href="https://opendal.apache.org/docs/rust/opendal/raw/adapters/typed_kv/struct.Backend.html" class="struct" title="struct opendal::raw::adapters::typed_kv::Backend">Backend</a>\<S\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader-6" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Reader" class="associatedtype">Reader</a> = <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer-6" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Writer" class="associatedtype">Writer</a> = KvWriter\<S\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister-6" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Lister" class="associatedtype">Lister</a> = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html" class="struct" title="struct opendal::raw::oio::HierarchyLister">HierarchyLister</a>\<KvLister\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter-6" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html#associatedtype.Deleter" class="associatedtype">Deleter</a> = <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html" class="struct" title="struct opendal::raw::oio::OneShotDeleter">OneShotDeleter</a>\<KvDeleter\<S\>\>
