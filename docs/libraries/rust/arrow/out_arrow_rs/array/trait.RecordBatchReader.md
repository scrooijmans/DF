# Trait RecordBatchReader Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/record_batch.rs.html#30" class="src">Source</a>

``` rust
pub trait RecordBatchReader: Iterator<Item = Result<RecordBatch, ArrowError>> {
    // Required method
    fn schema(&self) -> Arc<Schema>;
}
```

Expand description

Trait for types that can read `RecordBatch`’s.

To create from an iterator, see [RecordBatchIterator](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchIterator.html "struct arrow::array::RecordBatchIterator").

## Required Methods<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>\>

Returns the schema of this `RecordBatchReader`.

Implementation of this trait should guarantee that all `RecordBatch`’s returned by this reader should have the same schema as returned from this method.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#impl-IntoPyArrow-for-Box%3Cdyn+RecordBatchReader+%2B+Send%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.IntoPyArrow.html" class="trait" title="trait arrow_pyarrow::IntoPyArrow">IntoPyArrow</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>

Convert a [`RecordBatchReader`](https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html "trait arrow::array::RecordBatchReader") into a `pyarrow.RecordBatchReader`.

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#method.into_pyarrow" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow-pyarrow/56.2.0/x86_64-unknown-linux-gnu/arrow_pyarrow/trait.IntoPyArrow.html#tymethod.into_pyarrow" class="fn">into_pyarrow</a>(self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert the implemented type into a Python object while consuming it.

## Implementations on Foreign Types<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#impl-RecordBatchReader-for-Box%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<R\>

where R: <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#impl-RecordBatchReader-for-BufReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a> for <a href="https://docs.rs/arrow-csv/56.2.0/x86_64-unknown-linux-gnu/arrow_csv/reader/struct.BufReader.html" class="struct" title="struct arrow_csv::reader::BufReader">BufReader</a>\<R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html" class="trait" title="trait std::io::BufRead">BufRead</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#method.schema-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#impl-RecordBatchReader-for-FileReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a> for <a href="https://docs.rs/arrow-ipc/56.2.0/x86_64-unknown-linux-gnu/arrow_ipc/reader/struct.FileReader.html" class="struct" title="struct arrow_ipc::reader::FileReader">FileReader</a>\<R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a> + <a href="https://doc.rust-lang.org/nightly/std/io/trait.Seek.html" class="trait" title="trait std::io::Seek">Seek</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#method.schema-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#impl-RecordBatchReader-for-StreamReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a> for <a href="https://docs.rs/arrow-ipc/56.2.0/x86_64-unknown-linux-gnu/arrow_ipc/reader/struct.StreamReader.html" class="struct" title="struct arrow_ipc::reader::StreamReader">StreamReader</a>\<R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#method.schema-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>\>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#impl-RecordBatchReader-for-Reader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a> for <a href="https://docs.rs/arrow-json/56.2.0/x86_64-unknown-linux-gnu/arrow_json/reader/struct.Reader.html" class="struct" title="struct arrow_json::reader::Reader">Reader</a>\<R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html" class="trait" title="trait std::io::BufRead">BufRead</a>,

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#method.schema-4" class="anchor">§</a>

#### fn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>\>

## Implementors<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#impl-RecordBatchReader-for-ArrowArrayStreamReader" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a> for <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.ArrowArrayStreamReader.html" class="struct" title="struct arrow::ffi_stream::ArrowArrayStreamReader">ArrowArrayStreamReader</a>

<a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html#impl-RecordBatchReader-for-RecordBatchIterator%3CI%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a> for <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatchIterator.html" class="struct" title="struct arrow::array::RecordBatchIterator">RecordBatchIterator</a>\<I\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>\>,
