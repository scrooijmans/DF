# geoarrow_array - Rust

## Crate geoarrow_array

[Source](about:blank/src/geoarrow_array/lib.rs.html#1-27)

Expand description

## [§](#geoarrow-array)geoarrow-array

GeoArrow array definitions.

The central type in Apache Arrow are arrays, which are a known-length sequence of values all having the same type. This crate provides concrete implementations of each type defined in the [GeoArrow specification](https://github.com/geoarrow/geoarrow), as well as a [GeoArrowArray](trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") trait that can be used for type-erasure.

In order to minimize overhead of dynamic downcasting, the array types in this crate are defined “natively” and there’s a `O(1)` conversion process that needs to happen to convert between a GeoArrow array type and an [`arrow`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") array type.

### [§](#building-a-geoarrow-array)Building a GeoArrow Array

Use [builders](builder/index.html "mod geoarrow_array::builder") to construct GeoArrow arrays. These builders offer a push-based interface to construct arrays from a series of objects that implement [`geo-traits`](https://docs.rs/geo-traits/0.3.0/x86_64-unknown-linux-gnu/geo_traits/index.html "mod geo_traits").

```
let point_type = PointType::new(Dimension::XY, Default::default());
let mut builder = PointBuilder::new(point_type);

builder.push_point(Some(&geo_types::point!(x: 0., y: 1.)));
builder.push_point(Some(&geo_types::point!(x: 2., y: 3.)));
builder.push_point(Some(&geo_types::point!(x: 4., y: 5.)));

let array: PointArray = builder.finish();

let point_0: Point<'_> = array.get(0).unwrap().unwrap();
assert_eq!(point_0.coord().unwrap().x_y(), (0., 1.));
```

Converting a builder to an array via `finish()` is always `O(1)`.

### [§](#converting-to-and-from-arrow-arrays)Converting to and from [`arrow`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") Arrays

The `geoarrow` crates depend on and are designed to be used in combination with the upstream [Arrow](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") crates. As such, we have easy integration to convert between representations of each crate.

Note that an [`Array`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html "trait arrow_array::array::Array") or [`ArrayRef`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html "type arrow_array::array::ArrayRef") only maintains information about the physical [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType") and will lose any extension type information. Because of this, it’s **imperative to store an [`Array`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html "trait arrow_array::array::Array") and [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field") together** since the [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field") persists the Arrow [extension metadata](https://arrow.apache.org/docs/format/Columnar.html#format-metadata-extension-types). A [`RecordBatch`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/record_batch/struct.RecordBatch.html "struct arrow_array::record_batch::RecordBatch") holds an [`Array`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html "trait arrow_array::array::Array") and [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field") together for each column, so a [`RecordBatch`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/record_batch/struct.RecordBatch.html "struct arrow_array::record_batch::RecordBatch") will persist extension metadata.

#### [§](#converting-to-geoarrow-arrays)Converting to GeoArrow Arrays

If you have an [`Array`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html "trait arrow_array::array::Array") and [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field") but don’t know the geometry type of the array, you can use [`from_arrow_array`](array/fn.from_arrow_array.html "fn geoarrow_array::array::from_arrow_array"):

```
fn use_from_arrow_array(array: &dyn Array, field: &Field) {
    let geoarrow_array: Arc<dyn GeoArrowArray> = from_arrow_array(array, field).unwrap();
    match geoarrow_array.data_type() {
        GeoArrowType::Point(_) => {
            let array: &PointArray = geoarrow_array.as_point();
        }
        _ => todo!("handle other geometry types"),
    }
}
```

If you know the geometry type of your array, you can use one of its `TryFrom` implementations to convert directly to that type. This means you don’t have to downcast on the GeoArrow side from an `Arc<dyn GeoArrowArray>`.

```
fn convert_to_point_array(array: &dyn Array, field: &Field) {
    let point_array = PointArray::try_from((array, field)).unwrap();
}
```

#### [§](#converting-to-arrow-arrays)Converting to [arrow](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") Arrays

You can use the [`to_array_ref`](about:blank/trait.GeoArrowArray.html#tymethod.to_array_ref "method geoarrow_array::GeoArrowArray::to_array_ref") or [`into_array_ref`](about:blank/trait.GeoArrowArray.html#tymethod.into_array_ref "method geoarrow_array::GeoArrowArray::into_array_ref") methods on [`GeoArrowArray`](trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to convert to an [`ArrayRef`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/type.ArrayRef.html "type arrow_array::array::ArrayRef").

Alternatively, if you have a concrete GeoArrow array type, you can use [`IntoArray`](trait.IntoArrow.html "trait geoarrow_array::IntoArrow") to convert to a concrete arrow array type.

The easiest way today to access an arrow [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field") is to use [`IntoArray::extension_type`](about:blank/trait.IntoArrow.html#tymethod.extension_type "method geoarrow_array::IntoArrow::extension_type") and then call `to_field` on the result. We like to make this process simpler in the future.

### [§](#downcasting-a-geoarrow-array)Downcasting a GeoArrow array

Arrays are often passed around as a dynamically typed `&dyn GeoArrowArray` or [`Arc<dyn GeoArrowArray>`](trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray").

While these arrays can be passed directly to compute functions, it is often the case that you wish to interact with the concrete arrays directly.

This requires downcasting to the concrete type of the array. Use the [`cast::AsGeoArrowArray`](cast/trait.AsGeoArrowArray.html "trait geoarrow_array::cast::AsGeoArrowArray") extension trait to do this ergonomically.

```
use geoarrow_array::cast::AsGeoArrowArray;
use geoarrow_array::{GeoArrowArrayAccessor, GeoArrowArray};

fn iter_line_string_array(array: &dyn GeoArrowArray) {
    for row in array.as_line_string().iter() {
        // do something with each row
    }
}
```

[array](array/index.html "mod geoarrow_array::array")

The concrete array definitions.

[builder](builder/index.html "mod geoarrow_array::builder")

Push-based APIs for constructing arrays.

[capacity](capacity/index.html "mod geoarrow_array::capacity")

Counters for managing buffer lengths for each geometry array type.

[cast](cast/index.html "mod geoarrow_array::cast")

Helper functions for downcasting [`dyn GeoArrowArray`](trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to concrete types and for converting between GeoArrow array representations.

[geozero](geozero/index.html "mod geoarrow_array::geozero")`geozero`

Implements the geometry and dataset conversion APIs defined by the [`geozero`](https://docs.rs/geozero/0.14.0/x86_64-unknown-linux-gnu/geozero/index.html "mod geozero") crate.

[scalar](scalar/index.html "mod geoarrow_array::scalar")

Scalar references onto a parent GeoArrow array.

[test](test/index.html "mod geoarrow_array::test")`test-data`

[downcast_geoarrow_array](macro.downcast_geoarrow_array.html "macro geoarrow_array::downcast_geoarrow_array")

Downcast a [GeoArrowArray](trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to a concrete-typed array based on its [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType").

[GeoArrowArrayIterator](struct.GeoArrowArrayIterator.html "struct geoarrow_array::GeoArrowArrayIterator")

An iterator of [`Arc<dyn GeoArrowArray>`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc") with an attached [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType")

[GeoArrowArray](trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray")

A base trait for all GeoArrow arrays.

[GeoArrowArrayAccessor](trait.GeoArrowArrayAccessor.html "trait geoarrow_array::GeoArrowArrayAccessor")

A trait for accessing the values of a [`GeoArrowArray`](trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray").

[GeoArrowArrayReader](trait.GeoArrowArrayReader.html "trait geoarrow_array::GeoArrowArrayReader")

Trait for types that can read `Arc<dyn GeoArrowArray>`’s.

[IntoArrow](trait.IntoArrow.html "trait geoarrow_array::IntoArrow")

Convert GeoArrow arrays into their respective [arrow](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") arrays.
