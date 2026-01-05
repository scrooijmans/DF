# Crate arrow Copy item path

<a href="https://docs.rs/arrow/56.0.0/x86_64-unknown-linux-gnu/src/arrow/lib.rs.html#18-411" class="src">Source</a>

Expand description

A complete, safe, native Rust implementation of [Apache Arrow](https://arrow.apache.org), a cross-language development platform for in-memory data.

Please see the [arrow crates.io](https://crates.io/crates/arrow) page for feature flags and tips to improve performance.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#columnar-format" class="doc-anchor">§</a>Columnar Format

The [`array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/index.html "mod datafusion::common::arrow::array") module provides statically typed implementations of all the array types as defined by the [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html)

For example, an [`Int32Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.Int32Array.html "type datafusion::common::arrow::array::Int32Array") represents a nullable array of `i32`

``` rust
let array = Int32Array::from(vec![Some(1), None, Some(3)]);
assert_eq!(array.len(), 3);
assert_eq!(array.value(0), 1);
assert_eq!(array.is_null(1), true);

let collected: Vec<_> = array.iter().collect();
assert_eq!(collected, vec![Some(1), None, Some(3)]);
assert_eq!(array.values(), &[1, 0, 3])
```

It is also possible to write generic code for different concrete types. For example, since the following function is generic over all primitively typed arrays, when invoked the Rust compiler will generate specialized implementations with optimized code for each concrete type.

``` rust
fn sum<T: ArrowPrimitiveType>(array: &PrimitiveArray<T>) -> T::Native
where
    T: ArrowPrimitiveType,
    T::Native: Sum
{
    array.iter().map(|v| v.unwrap_or_default()).sum()
}

assert_eq!(sum(&Float32Array::from(vec![1.1, 2.9, 3.])), 7.);
assert_eq!(sum(&TimestampNanosecondArray::from(vec![1, 2, 3])), 6);
```

And the following uses [`ArrayAccessor`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.ArrayAccessor.html "trait datafusion::common::arrow::array::ArrayAccessor") to implement a generic function over all arrays with comparable values.

``` rust
fn min<T: ArrayAccessor>(array: T) -> Option<T::Item>
where
    T::Item: Ord
{
    ArrayIter::new(array).filter_map(|v| v).min()
}

assert_eq!(min(&Int32Array::from(vec![4, 2, 1, 6])), Some(1));
assert_eq!(min(&StringArray::from(vec!["b", "a", "c"])), Some("a"));
```

**For more examples, and details consult the [arrow_array](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") docs.**

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#type-erasure--trait-objects" class="doc-anchor">§</a>Type Erasure / Trait Objects

It is common to write code that handles any type of array, without necessarily knowing its concrete type. This is done using the [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/index.html "mod datafusion::common::arrow::array") trait and using [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") to determine the appropriate `downcast_ref`.

``` rust
fn impl_string(array: &StringArray) {}
fn impl_f32(array: &Float32Array) {}

fn impl_dyn(array: &dyn Array) {
    match array.data_type() {
        // downcast `dyn Array` to concrete `StringArray`
        DataType::Utf8 => impl_string(array.as_any().downcast_ref().unwrap()),
        // downcast `dyn Array` to concrete `Float32Array`
        DataType::Float32 => impl_f32(array.as_any().downcast_ref().unwrap()),
        _ => unimplemented!()
    }
}
```

You can use the [`AsArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.AsArray.html "trait datafusion::common::arrow::array::AsArray") extension trait to facilitate downcasting:

``` rust
fn impl_string(array: &StringArray) {}
fn impl_f32(array: &Float32Array) {}

fn impl_dyn(array: &dyn Array) {
    match array.data_type() {
        DataType::Utf8 => impl_string(array.as_string()),
        DataType::Float32 => impl_f32(array.as_primitive()),
        _ => unimplemented!()
    }
}
```

It is also common to want to write a function that returns one of a number of possible array implementations. [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") is a type-alias for [`Arc<dyn Array>`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") which is frequently used for this purpose

``` rust
fn parse_to_primitive<'a, T, I>(iter: I) -> PrimitiveArray<T>
where
    T: ArrowPrimitiveType,
    T::Native: FromStr,
    I: IntoIterator<Item=&'a str>,
{
    PrimitiveArray::from_iter(iter.into_iter().map(|val| T::Native::from_str(val).ok()))
}

fn parse_strings<'a, I>(iter: I, to_data_type: DataType) -> ArrayRef
where
    I: IntoIterator<Item=&'a str>,
{
   match to_data_type {
       DataType::Int32 => Arc::new(parse_to_primitive::<Int32Type, _>(iter)) as _,
       DataType::UInt32 => Arc::new(parse_to_primitive::<UInt32Type, _>(iter)) as _,
       _ => unimplemented!()
   }
}

let array = parse_strings(["1", "2", "3"], DataType::Int32);
let integers = array.as_any().downcast_ref::<Int32Array>().unwrap();
assert_eq!(integers.values(), &[1, 2, 3])
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#compute-kernels" class="doc-anchor">§</a>Compute Kernels

The [`compute`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/index.html "mod datafusion::common::arrow::compute") module provides optimised implementations of many common operations, for example the `parse_strings` operation above could also be implemented as follows:

``` rust
fn parse_strings<'a, I>(iter: I, to_data_type: &DataType) -> Result<ArrayRef>
where
    I: IntoIterator<Item=&'a str>,
{
    let array = StringArray::from_iter(iter.into_iter().map(Some));
    arrow::compute::cast(&array, to_data_type)
}

let array = parse_strings(["1", "2", "3"], &DataType::UInt32).unwrap();
let integers = array.as_any().downcast_ref::<UInt32Array>().unwrap();
assert_eq!(integers.values(), &[1, 2, 3])
```

This module also implements many common vertical operations:

- All mathematical binary operators, such as [`sub`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/numeric/fn.sub.html "fn datafusion::common::arrow::compute::kernels::numeric::sub")
- All boolean binary operators such as [`equality`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/cmp/fn.eq.html "fn datafusion::common::arrow::compute::kernels::cmp::eq")
- [`cast`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.cast.html "fn datafusion::common::arrow::compute::cast")
- [`filter`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.filter.html "fn datafusion::common::arrow::compute::filter")
- [`take`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.take.html "fn datafusion::common::arrow::compute::take")
- [`sort`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sort.html "fn datafusion::common::arrow::compute::sort")
- some string operators such as [`substring`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/substring/fn.substring.html "fn datafusion::common::arrow::compute::kernels::substring::substring") and [`length`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/length/fn.length.html "fn datafusion::common::arrow::compute::kernels::length::length")

``` rust
let array = Int32Array::from_iter(0..100);
// Create a 32-bit integer scalar (single) value:
let scalar = Int32Array::new_scalar(60);
// find all rows in the array that are greater than 60
let predicate = gt(&array, &scalar).unwrap();
// copy all matching rows into a new array
let filtered = filter(&array, &predicate).unwrap();

let expected = Int32Array::from_iter(61..100);
assert_eq!(&expected, filtered.as_primitive::<Int32Type>());
```

As well as some horizontal operations, such as:

- [`min`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.min.html "fn datafusion::common::arrow::compute::min") and [`max`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.max.html "fn datafusion::common::arrow::compute::max")
- [`sum`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.sum.html "fn datafusion::common::arrow::compute::sum")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#tabular-representation" class="doc-anchor">§</a>Tabular Representation

It is common to want to group one or more columns together into a tabular representation. This is provided by [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") which combines a [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema") and a corresponding list of [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef").

``` rust
let col_1 = Arc::new(Int32Array::from_iter([1, 2, 3])) as _;
let col_2 = Arc::new(Float32Array::from_iter([1., 6.3, 4.])) as _;

let batch = RecordBatch::try_from_iter([("col1", col_1), ("col_2", col_2)]).unwrap();
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#pretty-printing" class="doc-anchor">§</a>Pretty Printing

See the [`util::pretty`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/pretty/index.html "mod datafusion::common::arrow::util::pretty") module (requires the `prettyprint` crate feature)

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#io" class="doc-anchor">§</a>IO

This crate provides readers and writers for various formats to/from [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

- JSON: [`Reader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Reader.html "struct datafusion::common::arrow::json::Reader") and [`Writer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html "struct datafusion::common::arrow::json::Writer")
- CSV: [`Reader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/type.Reader.html "type datafusion::common::arrow::csv::Reader") and [`Writer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html "struct datafusion::common::arrow::csv::Writer")
- IPC: [`Reader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/reader/struct.StreamReader.html "struct datafusion::common::arrow::ipc::reader::StreamReader") and [`Writer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html "struct datafusion::common::arrow::ipc::writer::FileWriter")

Parquet is published as a [separate crate](https://crates.io/crates/parquet)

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#serde-compatibility" class="doc-anchor">§</a>Serde Compatibility

[`arrow_json::reader::Decoder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html "struct datafusion::common::arrow::json::reader::Decoder") provides a mechanism to convert arbitrary, serde-compatible structures into [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch").

Whilst likely less performant than implementing a custom builder, as described in [arrow_array::builder](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/builder/index.html "mod datafusion::common::arrow::array::builder"), this provides a simple mechanism to get up and running quickly

``` rust
#[derive(Serialize)]
struct MyStruct {
    int32: i32,
    string: String,
}

let schema = Schema::new(vec![
    Field::new("int32", DataType::Int32, false),
    Field::new("string", DataType::Utf8, false),
]);

let rows = vec![
    MyStruct{ int32: 5, string: "bar".to_string() },
    MyStruct{ int32: 8, string: "foo".to_string() },
];

let mut decoder = ReaderBuilder::new(Arc::new(schema)).build_decoder().unwrap();
decoder.serialize(&rows).unwrap();

let batch = decoder.flush().unwrap().unwrap();

// Expect batch containing two columns
let int32 = batch.column(0).as_primitive::<Int32Type>();
assert_eq!(int32.values(), &[5, 8]);

let string = batch.column(1).as_string::<i32>();
assert_eq!(string.value(0), "bar");
assert_eq!(string.value(1), "foo");
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#crate-topology" class="doc-anchor">§</a>Crate Topology

The [`arrow`](https://github.com/apache/arrow-rs) project is implemented as multiple sub-crates, which are then re-exported by this top-level crate.

Crate authors can choose to depend on this top-level crate, or just the sub-crates they need.

The current list of sub-crates is:

- [`arrow-arith`](https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/arrow_arith/index.html "mod arrow_arith") - arithmetic kernels
- [`arrow-array`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") - type-safe arrow array abstractions
- [`arrow-buffer`](https://docs.rs/arrow-buffer/56.0.0/x86_64-unknown-linux-gnu/arrow_buffer/index.html "mod arrow_buffer") - buffer abstractions for arrow arrays
- [`arrow-cast`](https://docs.rs/arrow-cast/56.0.0/x86_64-unknown-linux-gnu/arrow_cast/index.html "mod arrow_cast") - cast kernels for arrow arrays
- [`arrow-csv`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/index.html "mod datafusion::common::arrow::csv") - read/write CSV to arrow format
- [`arrow-data`](https://docs.rs/arrow-data/56.0.0/x86_64-unknown-linux-gnu/arrow_data/index.html "mod arrow_data") - the underlying data of arrow arrays
- [`arrow-ipc`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/index.html "mod datafusion::common::arrow::ipc") - read/write IPC to arrow format
- [`arrow-json`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/index.html "mod datafusion::common::arrow::json") - read/write JSON to arrow format
- [`arrow-ord`](https://docs.rs/arrow-ord/56.0.0/x86_64-unknown-linux-gnu/arrow_ord/index.html "mod arrow_ord") - ordering kernels for arrow arrays
- [`arrow-row`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/row/index.html "mod datafusion::common::arrow::row") - comparable row format
- [`arrow-schema`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/index.html "mod arrow_schema") - the logical types for arrow arrays
- [`arrow-select`](https://docs.rs/arrow-select/56.0.0/x86_64-unknown-linux-gnu/arrow_select/index.html "mod arrow_select") - selection kernels for arrow arrays
- [`arrow-string`](https://docs.rs/arrow-string/56.0.0/x86_64-unknown-linux-gnu/arrow_string/index.html "mod arrow_string") - string kernels for arrow arrays

Some functionality is also distributed independently of this crate:

- [`arrow-flight`](https://docs.rs/arrow-flight/latest/arrow_flight/) - support for [Arrow Flight RPC](https://arrow.apache.org/docs/format/Flight.html)
- [`arrow-integration-test`](https://docs.rs/arrow-integration-test/latest/arrow_integration_test/) - support for [Arrow JSON Test Format](https://github.com/apache/arrow/blob/master/docs/source/format/Integration.rst#json-test-data-format)
- [`parquet`](https://docs.rs/parquet/latest/parquet/) - support for [Apache Parquet](https://parquet.apache.org/)

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#safety-and-security" class="doc-anchor">§</a>Safety and Security

Like many crates, this crate makes use of unsafe where prudent. However, it endeavours to be sound. Specifically, **it should not be possible to trigger undefined behaviour using safe APIs.**

If you think you have found an instance where this is possible, please file a ticket in our [issue tracker](https://github.com/apache/arrow-rs/issues) and it will be triaged and fixed. For more information on arrow’s use of unsafe, see [here](https://github.com/apache/arrow-rs/tree/main/arrow#safety).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#higher-level-processing" class="doc-anchor">§</a>Higher-level Processing

This crate aims to provide reusable, low-level primitives for operating on columnar data. For more sophisticated query processing workloads, consider checking out [DataFusion](https://github.com/apache/arrow-datafusion). This orchestrates the primitives exported by this crate into an embeddable query engine, with SQL and DataFrame frontends, and heavily influences this crate’s roadmap.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/index.html" class="mod" title="mod datafusion::common::arrow::alloc">alloc</a>  
Defines the low-level [`Allocation`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/alloc/trait.Allocation.html "trait datafusion::common::arrow::alloc::Allocation") API for shared memory regions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/index.html" class="mod" title="mod datafusion::common::arrow::array">array</a>  
Statically typed implementations of Arrow Arrays

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/index.html" class="mod" title="mod datafusion::common::arrow::buffer">buffer</a>  
Types of shared memory region

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/index.html" class="mod" title="mod datafusion::common::arrow::compute">compute</a>  
Computation kernels on Arrow Arrays

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/index.html" class="mod" title="mod datafusion::common::arrow::csv">csv</a>  
Transfer data between the Arrow memory format and CSV (comma-separated values).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/index.html" class="mod" title="mod datafusion::common::arrow::datatypes">datatypes</a>  
Defines the logical data types of Arrow arrays.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/index.html" class="mod" title="mod datafusion::common::arrow::error">error</a>  
Defines `ArrowError` for representing failures in various Arrow operations.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/index.html" class="mod" title="mod datafusion::common::arrow::ipc">ipc</a>  
Support for the [Arrow IPC Format](https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/index.html" class="mod" title="mod datafusion::common::arrow::json">json</a>  
Transfer data between the Arrow memory format and JSON line-delimited records.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/pyarrow/index.html" class="mod" title="mod datafusion::common::arrow::pyarrow">pyarrow</a>  
Pass Arrow objects from and to PyArrow, using Arrow’s [C Data Interface](https://arrow.apache.org/docs/format/CDataInterface.html) and [pyo3](https://docs.rs/pyo3/latest/pyo3/).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/record_batch/index.html" class="mod" title="mod datafusion::common::arrow::record_batch">record_batch</a>  
Contains the `RecordBatch` type and associated traits

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/row/index.html" class="mod" title="mod datafusion::common::arrow::row">row</a>  
A comparable row-oriented representation of a collection of [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/tensor/index.html" class="mod" title="mod datafusion::common::arrow::tensor">tensor</a>  
Arrow Tensor Type, defined in [`format/Tensor.fbs`](https://github.com/apache/arrow/blob/master/format/Tensor.fbs).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/index.html" class="mod" title="mod datafusion::common::arrow::util">util</a>  
Utility functions for working with Arrow data

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/macro.downcast_dictionary_array.html" class="macro" title="macro datafusion::common::arrow::downcast_dictionary_array">downcast_dictionary_array</a>  
Downcast an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") to a [`DictionaryArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.DictionaryArray.html "struct datafusion::common::arrow::array::DictionaryArray") based on its [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType"), accepts a number of subsequent patterns to match the data type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/macro.downcast_primitive_array.html" class="macro" title="macro datafusion::common::arrow::downcast_primitive_array">downcast_primitive_array</a>  
Downcast an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") to a [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") based on its [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") accepts a number of subsequent patterns to match the data type

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/constant.ARROW_VERSION.html" class="constant" title="constant datafusion::common::arrow::ARROW_VERSION">ARROW_VERSION</a>  
Arrow crate version
