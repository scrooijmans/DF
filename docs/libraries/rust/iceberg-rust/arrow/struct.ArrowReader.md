# Struct ArrowReader Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/reader.rs.html#126-136" class="src">Source</a>

``` rust
pub struct ArrowReader { /* private fields */ }
```

Expand description

Reads data from Parquet files

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#impl-ArrowReader" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html" class="struct" title="struct iceberg::arrow::ArrowReader">ArrowReader</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#method.read" class="fn">read</a>(self, tasks: <a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.FileScanTaskStream.html" class="type" title="type iceberg::scan::FileScanTaskStream">FileScanTaskStream</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.ArrowRecordBatchStream.html" class="type" title="type iceberg::scan::ArrowRecordBatchStream">ArrowRecordBatchStream</a>\>

Take a stream of FileScanTasks and reads all the files. Returns a stream of Arrow RecordBatches containing the data from the files

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#impl-Clone-for-ArrowReader" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html" class="struct" title="struct iceberg::arrow::ArrowReader">ArrowReader</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html" class="struct" title="struct iceberg::arrow::ArrowReader">ArrowReader</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html#blanket-implementations" class="anchor">§</a>
