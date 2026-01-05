# Struct ObjectStoreService Copy item path

<a href="https://opendal.apache.org/docs/object-store-opendal/src/object_store_opendal/service/mod.rs.html#81-83" class="src">Source</a>

``` rust
pub struct ObjectStoreService { /* private fields */ }
```

Expand description

ObjectStore backend

## Trait Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#impl-Access-for-ObjectStoreService" class="anchor">Â§</a>

### impl Access for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html" class="struct" title="struct object_store_opendal::ObjectStoreService">ObjectStoreService</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#associatedtype.Reader" class="anchor">Â§</a>

#### type Reader = ObjectStoreReader

Reader is the associated reader returned in `read` operation.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#associatedtype.Writer" class="anchor">Â§</a>

#### type Writer = MultipartWriter\<ObjectStoreWriter\>

Writer is the associated writer returned in `write` operation.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#associatedtype.Lister" class="anchor">Â§</a>

#### type Lister = ObjectStoreLister

Lister is the associated lister returned in `list` operation.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#associatedtype.Deleter" class="anchor">Â§</a>

#### type Deleter = BatchDeleter\<ObjectStoreDeleter\>

Deleter is the associated deleter returned in `delete` operation.

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.info" class="anchor">Â§</a>

#### fn info(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<AccessorInfo\>

Invoke the `info` operation to get metadata of accessor. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.stat" class="anchor">Â§</a>

#### async fn stat(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: OpStat) -\> Result\<RpStat\>

Invoke the `stat` operation on the specified path. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.read" class="anchor">Â§</a>

#### async fn read(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: OpRead) -\> Result\<(RpRead, Self::Reader)\>

Invoke the `read` operation on the specified path, returns a \[`Reader`\]\[crate::Reader\] if operate successful. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.write" class="anchor">Â§</a>

#### async fn write( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: OpWrite, ) -\> Result\<(RpWrite, Self::Writer)\>

Invoke the `write` operation on the specified path, returns a written size if operate successful. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.delete" class="anchor">Â§</a>

#### async fn delete(&self) -\> Result\<(RpDelete, Self::Deleter)\>

Invoke the `delete` operation on the specified path. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.list" class="anchor">Â§</a>

#### async fn list(&self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: OpList) -\> Result\<(RpList, Self::Lister)\>

Invoke the `list` operation on the specified path. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.create_dir" class="anchor">Â§</a>

#### fn create_dir( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: OpCreateDir, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<RpCreateDir, Error\>\> + MaybeSend

Invoke the `create` operation on the specified path Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.copy" class="anchor">Â§</a>

#### fn copy( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: OpCopy, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<RpCopy, Error\>\> + MaybeSend

Invoke the `copy` operation on the specified `from` path and `to` path. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.rename" class="anchor">Â§</a>

#### fn rename( &self, from: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, to: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: OpRename, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<RpRename, Error\>\> + MaybeSend

Invoke the `rename` operation on the specified `from` path and `to` path. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.presign" class="anchor">Â§</a>

#### fn presign( &self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: OpPresign, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<RpPresign, Error\>\> + MaybeSend

Invoke the `presign` operation on the specified path. Read more

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#impl-Debug-for-ObjectStoreService" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html" class="struct" title="struct object_store_opendal::ObjectStoreService">ObjectStoreService</a>

<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/object-store-opendal/object_store_opendal/struct.ObjectStoreService.html#blanket-implementations" class="anchor">Â§</a>
