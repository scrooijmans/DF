# Trait IntoPyArrow Copy item path

<a href="https://docs.rs/arrow-pyarrow/56.0.0/x86_64-unknown-linux-gnu/src/arrow_pyarrow/lib.rs.html#102" class="src">Source</a>

``` rust
pub trait IntoPyArrow {
    // Required method
    fn into_pyarrow(self, py: Python<'_>) -> Result<Py<PyAny>, PyErr>;
}
```

Expand description

Convert an arrow-rs type into a PyArrow object.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#tymethod.into_pyarrow" class="fn">into_pyarrow</a>(self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert the implemented type into a Python object while consuming it.

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#impl-IntoPyArrow-for-Box%3Cdyn+RecordBatchReader+%2B+Send%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::IntoPyArrow">IntoPyArrow</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait datafusion::common::arrow::array::RecordBatchReader">RecordBatchReader</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>

Convert a [`RecordBatchReader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.RecordBatchReader.html "trait datafusion::common::arrow::array::RecordBatchReader") into a `pyarrow.RecordBatchReader`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#method.into_pyarrow" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#tymethod.into_pyarrow" class="fn">into_pyarrow</a>(self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#impl-IntoPyArrow-for-ArrowArrayStreamReader" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::IntoPyArrow">IntoPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi_stream/struct.ArrowArrayStreamReader.html" class="struct" title="struct datafusion::common::arrow::array::ffi_stream::ArrowArrayStreamReader">ArrowArrayStreamReader</a>

Convert a [`ArrowArrayStreamReader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi_stream/struct.ArrowArrayStreamReader.html "struct datafusion::common::arrow::array::ffi_stream::ArrowArrayStreamReader") into a `pyarrow.RecordBatchReader`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html#impl-IntoPyArrow-for-T" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::IntoPyArrow">IntoPyArrow</a> for T

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a>,
