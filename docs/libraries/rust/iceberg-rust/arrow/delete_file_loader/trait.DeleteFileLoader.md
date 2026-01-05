# Trait DeleteFileLoader Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/delete_file_loader.rs.html#32-41" class="src">Source</a>

``` rust
pub trait DeleteFileLoader {
    // Required method
    fn read_delete_file<'life0, 'life1, 'async_trait>(
        &'life0 self,
        task: &'life1 FileScanTaskDeleteFile,
        schema: SchemaRef,
    ) -> Pin<Box<dyn Future<Output = Result<ArrowRecordBatchStream>> + Send + 'async_trait>>
       where Self: 'async_trait,
             'life0: 'async_trait,
             'life1: 'async_trait;
}
```

Expand description

Delete File Loader

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/delete_file_loader/trait.DeleteFileLoader.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/delete_file_loader/trait.DeleteFileLoader.html#tymethod.read_delete_file" class="fn">read_delete_file</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, task: &'life1 <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTaskDeleteFile.html" class="struct" title="struct iceberg::scan::FileScanTaskDeleteFile">FileScanTaskDeleteFile</a>, schema: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.ArrowRecordBatchStream.html" class="type" title="type iceberg::scan::ArrowRecordBatchStream">ArrowRecordBatchStream</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Read the delete file referred to in the task

Returns the contents of the delete file as a RecordBatch stream. Applies schema evolution.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/delete_file_loader/trait.DeleteFileLoader.html#implementors" class="anchor">§</a>
