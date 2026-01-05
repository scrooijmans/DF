# Trait ToPyArrow Copy item path

<a href="https://docs.rs/arrow-pyarrow/56.0.0/x86_64-unknown-linux-gnu/src/arrow_pyarrow/lib.rs.html#96" class="src">Source</a>

``` rust
pub trait ToPyArrow {
    // Required method
    fn to_pyarrow(&self, py: Python<'_>) -> Result<Py<PyAny>, PyErr>;
}
```

Expand description

Create a new PyArrow object from a arrow-rs type.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#tymethod.to_pyarrow" class="fn">to_pyarrow</a>(&self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

Convert the implemented type into a Python object without consuming it.

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#impl-ToPyArrow-for-Vec%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#method.to_pyarrow" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#tymethod.to_pyarrow" class="fn">to_pyarrow</a>(&self, py: <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/marker/struct.Python.html" class="struct" title="struct pyo3::marker::Python">Python</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/instance/struct.Py.html" class="struct" title="struct pyo3::instance::Py">Py</a>\<<a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/types/any/struct.PyAny.html" class="struct" title="struct pyo3::types::any::PyAny">PyAny</a>\>, <a href="https://docs.rs/pyo3/0.25.1/x86_64-unknown-linux-gnu/pyo3/err/struct.PyErr.html" class="struct" title="struct pyo3::err::PyErr">PyErr</a>\>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#impl-ToPyArrow-for-ScalarValue" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#impl-ToPyArrow-for-DataType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#impl-ToPyArrow-for-ArrayData" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.ArrayData.html" class="struct" title="struct datafusion::common::arrow::array::ArrayData">ArrayData</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#impl-ToPyArrow-for-RecordBatch" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#impl-ToPyArrow-for-Field" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html#impl-ToPyArrow-for-Schema" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/trait.ToPyArrow.html" class="trait" title="trait datafusion::common::arrow::pyarrow::ToPyArrow">ToPyArrow</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>
