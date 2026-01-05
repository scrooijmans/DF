# Module oio Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/mod.rs.html#18-40" class="src">Source</a>

Expand description

`oio` provides OpenDALâ€™s raw traits and types that opendal returns as output.

Those types should only be used internally and we donâ€™t want users to depend on them.

## Structs<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/index.html#structs" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html" class="struct" title="struct opendal::raw::oio::AppendWriter">AppendWriter</a>  
AppendWriter will implements [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on append object.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html" class="struct" title="struct opendal::raw::oio::BatchDeleteResult">BatchDeleteResult</a>  
BatchDeleteResult is the result of batch delete operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html" class="struct" title="struct opendal::raw::oio::BatchDeleter">BatchDeleter</a>  
BatchDeleter is used to implement [`oio::Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete") based on batch delete.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BlockWriter.html" class="struct" title="struct opendal::raw::oio::BlockWriter">BlockWriter</a>  
BlockWriter will implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on block uploads.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.Entry.html" class="struct" title="struct opendal::raw::oio::Entry">Entry</a>  
Entry is returned by `Page` or `BlockingPage` during list operations.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlatLister.html" class="struct" title="struct opendal::raw::oio::FlatLister">FlatLister</a>  
FlatLister will walk dir in bottom up way:

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.FlexBuf.html" class="struct" title="struct opendal::raw::oio::FlexBuf">FlexBuf</a>  
FlexBuf is a buffer that support frozen bytes and reuse existing allocated memory.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.HierarchyLister.html" class="struct" title="struct opendal::raw::oio::HierarchyLister">HierarchyLister</a>  
ToHierarchyLister will convert a flat list to hierarchy by filter not needed entries.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartPart.html" class="struct" title="struct opendal::raw::oio::MultipartPart">MultipartPart</a>  
The result of [`MultipartWrite::write_part`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.MultipartWrite.html#tymethod.write_part "method opendal::raw::oio::MultipartWrite::write_part").

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html" class="struct" title="struct opendal::raw::oio::MultipartWriter">MultipartWriter</a>  
MultipartWriter will implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on multipart uploads.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html" class="struct" title="struct opendal::raw::oio::OneShotDeleter">OneShotDeleter</a>  
OneShotDelete is used to implement [`oio::Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete") based on one shot.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html" class="struct" title="struct opendal::raw::oio::OneShotWriter">OneShotWriter</a>  
OneShotWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on one shot.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageContext.html" class="struct" title="struct opendal::raw::oio::PageContext">PageContext</a>  
PageContext is the context passing between `PageList`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PageLister.html" class="struct" title="struct opendal::raw::oio::PageLister">PageLister</a>  
PageLister implements [`oio::List`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html "trait opendal::raw::oio::List") based on [`PageList`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html "trait opendal::raw::oio::PageList").

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PooledBuf.html" class="struct" title="struct opendal::raw::oio::PooledBuf">PooledBuf</a>  
PooledBuf is a buffer pool that designed for reusing already allocated bufs.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html" class="struct" title="struct opendal::raw::oio::PositionWriter">PositionWriter</a>  
PositionWriter will implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on position write.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PrefixLister.html" class="struct" title="struct opendal::raw::oio::PrefixLister">PrefixLister</a>  
PrefixLister is used to filter entries by prefix.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.QueueBuf.html" class="struct" title="struct opendal::raw::oio::QueueBuf">QueueBuf</a>  
QueueBuf is a queue of [`Buffer`](https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html "struct opendal::Buffer").

## Traits<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/index.html#traits" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html" class="trait" title="trait opendal::raw::oio::AppendWrite">AppendWrite</a>  
AppendWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on append object. By implementing AppendWrite, services donâ€™t need to care about the details of buffering and uploading parts.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html" class="trait" title="trait opendal::raw::oio::BatchDelete">BatchDelete</a>  
BatchDelete is used to implement [`oio::Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete") based on batch delete operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BlockWrite.html" class="trait" title="trait opendal::raw::oio::BlockWrite">BlockWrite</a>  
BlockWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on block uploads. By implementing BlockWrite, services donâ€™t need to care about the details of uploading blocks.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a>  
The Delete trait defines interfaces for performing deletion operations.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html" class="trait" title="trait opendal::raw::oio::DeleteDyn">DeleteDyn</a>  
The dyn version of [`Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete")

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html" class="trait" title="trait opendal::raw::oio::List">List</a>  
Page trait is used by [`raw::Accessor`](https://opendal.apache.org/docs/rust/opendal/raw/type.Accessor.html "type opendal::raw::Accessor") to implement `list` operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ListDyn.html" class="trait" title="trait opendal::raw::oio::ListDyn">ListDyn</a>  
ListDyn is the dyn version of [`List`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html "trait opendal::raw::oio::List"). Makes it possible to use as `Box<dyn ListDyn>`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.MultipartWrite.html" class="trait" title="trait opendal::raw::oio::MultipartWrite">MultipartWrite</a>  
MultipartWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on multipart uploads. By implementing MultipartWrite, services donâ€™t need to care about the details of uploading parts.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotDelete.html" class="trait" title="trait opendal::raw::oio::OneShotDelete">OneShotDelete</a>  
OneShotDelete is used to implement [`oio::Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete") based on one shot operation.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html" class="trait" title="trait opendal::raw::oio::OneShotWrite">OneShotWrite</a>  
OneShotWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on one shot operation. By implementing OneShotWrite, services donâ€™t need to care about the details.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PageList.html" class="trait" title="trait opendal::raw::oio::PageList">PageList</a>  
PageList is used to implement [`oio::List`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html "trait opendal::raw::oio::List") based on API supporting pagination. By implementing PageList, services donâ€™t need to care about the details of page list.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html" class="trait" title="trait opendal::raw::oio::PositionWrite">PositionWrite</a>  
PositionWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on position write.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>  
Read is the internal trait used by OpenDAL to read data from storage.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html" class="trait" title="trait opendal::raw::oio::ReadDyn">ReadDyn</a>  
ReadDyn is the dyn version of [`Read`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html "trait opendal::raw::oio::Read") make it possible to use as `Box<dyn ReadDyn>`.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>  
Write is the trait that OpenDAL returns to callers.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html" class="trait" title="trait opendal::raw::oio::WriteDyn">WriteDyn</a>  
WriteDyn is the dyn version of [`Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") make it possible to use as `Box<dyn WriteDyn>`.

## Type Aliases<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/index.html#types" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Deleter.html" class="type" title="type opendal::raw::oio::Deleter">Deleter</a>  
Deleter is a type erased [`Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete")

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Lister.html" class="type" title="type opendal::raw::oio::Lister">Lister</a>  
The boxed version of [`List`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.List.html "trait opendal::raw::oio::List")

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Reader.html" class="type" title="type opendal::raw::oio::Reader">Reader</a>  
Reader is a type erased [`Read`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html "trait opendal::raw::oio::Read").

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/type.Writer.html" class="type" title="type opendal::raw::oio::Writer">Writer</a>  
Writer is a type erased [`Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write")
