# Crate pyarrow Copy item path

<a href="https://docs.rs/arrow-pyarrow/56.0.0/x86_64-unknown-linux-gnu/src/arrow_pyarrow/lib.rs.html#18-520" class="src">Source</a>

Expand description

Pass Arrow objects from and to PyArrow, using Arrow’s [C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html) and [pyo3](https://docs.rs/pyo3/latest/pyo3/).

For underlying implementation, see the [ffi](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi/index.html "mod datafusion::common::arrow::array::ffi") module.

One can use these to write Python functions that take and return PyArrow objects, with automatic conversion to corresponding arrow-rs types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/index.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
#[pyfunction]
fn double_array(array: PyArrowType<ArrayData>) -> PyResult<PyArrowType<ArrayData>> {
    let array = array.0; // Extract from PyArrowType wrapper
    let array: Arc<dyn Array> = make_array(array); // Convert ArrayData to ArrayRef
    let array: &Int32Array = array.as_any().downcast_ref()
        .ok_or_else(|| PyValueError::new_err("expected int32 array"))?;
    let array: Int32Array = array.iter().map(|x| x.map(|x| x * 2)).collect();
    Ok(PyArrowType(array.into_data()))
}
```

| pyarrow type | arrow-rs type |
|----|----|
| `pyarrow.DataType` | [DataType](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") |
| `pyarrow.Field` | [Field](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") |
| `pyarrow.Schema` | [Schema](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema") |
| `pyarrow.Array` | [ArrayData](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html "struct datafusion::common::arrow::array::ArrayData") |
| `pyarrow.RecordBatch` | [RecordBatch](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") |
| `pyarrow.RecordBatchReader` | [ArrowArrayStreamReader](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi_stream/struct.ArrowArrayStreamReader.html "struct datafusion::common::arrow::array::ffi_stream::ArrowArrayStreamReader") / `Box<dyn RecordBatchReader + Send>` (1) |

\(1\) `pyarrow.RecordBatchReader` can be imported as [ArrowArrayStreamReader](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi_stream/struct.ArrowArrayStreamReader.html "struct datafusion::common::arrow::array::ffi_stream::ArrowArrayStreamReader"). Either [ArrowArrayStreamReader](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi_stream/struct.ArrowArrayStreamReader.html "struct datafusion::common::arrow::array::ffi_stream::ArrowArrayStreamReader") or `Box<dyn RecordBatchReader + Send>` can be exported as `pyarrow.RecordBatchReader`. (`Box<dyn RecordBatchReader + Send>` is typically easier to create.)

PyArrow has the notion of chunked arrays and tables, but arrow-rs doesn’t have these same concepts. A chunked table is instead represented with `Vec<RecordBatch>`. A `pyarrow.Table` can be imported to Rust by calling [pyarrow.Table.to_reader()](https://arrow.apache.org/docs/python/generated/pyarrow.Table.html#pyarrow.Table.to_reader) and then importing the reader as a [ArrowArrayStreamReader](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/ffi_stream/struct.ArrowArrayStreamReader.html "struct datafusion::common::arrow::array::ffi_stream::ArrowArrayStreamReader").

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/struct.ArrowException.html" class="struct" title="struct datafusion::common::arrow::pyarrow::ArrowException">ArrowException</a>  
A Rust type representing an exception defined in Python code.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/struct.PyArrowType.html" class="struct" title="struct datafusion::common::arrow::pyarrow::PyArrowType">PyArrowType</a>  
A newtype wrapper for types implementing [`FromPyArrow`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.FromPyArrow.html "trait datafusion::common::arrow::pyarrow::FromPyArrow") or [`IntoPyArrow`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html "trait datafusion::common::arrow::pyarrow::IntoPyArrow").

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.FromPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::FromPyArrow">FromPyArrow</a>  
Trait for converting Python objects to arrow-rs types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.IntoPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::IntoPyArrow">IntoPyArrow</a>  
Convert an arrow-rs type into a PyArrow object.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a>  
Create a new PyArrow object from a arrow-rs type.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/type.PyArrowException.html" class="type" title="type datafusion::common::arrow::pyarrow::PyArrowException">PyArrowException</a>  
Represents an exception raised by PyArrow.
