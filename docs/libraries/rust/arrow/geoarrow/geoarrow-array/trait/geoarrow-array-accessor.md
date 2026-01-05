# GeoArrowArrayAccessor in geoarrow_array - Rust

[![](https://github.com/geoarrow.png)](../geoarrow_array/index.html)

## Trait GeoArrowArrayAccessor

[Source](about:blank/src/geoarrow_array/trait_.rs.html#373-510)

```
pub trait GeoArrowArrayAccessor<'a>: GeoArrowArray {
    type Item: Send + Sync + GeometryTrait<T = f64>;

    // Required method
    unsafe fn value_unchecked(
        &'a self,
        index: usize,
    ) -> GeoArrowResult<Self::Item>;

    // Provided methods
    fn value(&'a self, index: usize) -> GeoArrowResult<Self::Item> { ... }
    fn get(&'a self, index: usize) -> GeoArrowResult<Option<Self::Item>> { ... }
    unsafe fn get_unchecked(
        &'a self,
        index: usize,
    ) -> Option<GeoArrowResult<Self::Item>> { ... }
    fn iter(
        &'a self,
    ) -> impl ExactSizeIterator<Item = Option<GeoArrowResult<Self::Item>>> + 'a { ... }
    fn iter_values(
        &'a self,
    ) -> impl ExactSizeIterator<Item = GeoArrowResult<Self::Item>> + 'a { ... }
}
```

Expand description

A trait for accessing the values of a [`GeoArrowArray`](trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray").

## [§](#performance)Performance

Accessing a geometry from a “native” array, such as `PointArray`, `MultiPolygonArray` or `GeometryArray` will always be constant-time and zero-copy.

Accessing a geometry from a “serialized” array such as `GenericWkbArray` or `GenericWktArray` will trigger some amount of parsing. In the case of `GenericWkbArray`, accessing an item will read the WKB header and scan the buffer if needed to find internal geometry offsets, but will not copy any internal coordinates. This allows for later access to be constant-time (though not necessarily zero-copy, since WKB is not byte-aligned). In the case of `GenericWktArray`, accessing a geometry will fully parse the WKT string and copy coordinates to a separate representation. This means that calling `.iter()` on a `GenericWktArray` will transparently fully parse every row.

## [§](#validity)Validity

A [`GeoArrowArrayAccessor`](trait.GeoArrowArrayAccessor.html "trait geoarrow_array::GeoArrowArrayAccessor") must always return a well-defined value for an index that is within the bounds `0..Array::len`, including for null indexes where [`Array::is_null`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html#method.is_null "method arrow_array::array::Array::is_null") is true.

The value at null indexes is unspecified, and implementations must not rely on a specific value such as [`Default::default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default") being returned, however, it must not be undefined.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#443)

Returns the element at index `i`, not considering validity.

##### [§](#examples)Examples

```
use geo_traits::{CoordTrait, PointTrait};

let point1 = geo_types::point!(x: 1., y: 2.);

let point_type = PointType::new(Dimension::XY, Default::default());
let point_array =
    PointBuilder::from_nullable_points([Some(&point1), None].into_iter(), point_type.clone())
        .finish();

let coord = unsafe { point_array.value_unchecked(0) }
    .unwrap()
    .coord()
    .unwrap();
assert_eq!(coord.x(), 1.);
assert_eq!(coord.y(), 2.);
```

##### [§](#errors)Errors

Errors for invalid WKT and WKB geometries. Will never error for native arrays.

##### [§](#safety)Safety

Caller is responsible for ensuring that the index is within the bounds of the array

[Source](about:blank/src/geoarrow_array/trait_.rs.html#406-409)

Returns the element at index `i`, not considering validity.

##### [§](#examples-1)Examples

```
use geo_traits::{CoordTrait, PointTrait};

let point1 = geo_types::point!(x: 1., y: 2.);

let point_type = PointType::new(Dimension::XY, Default::default());
let point_array =
    PointBuilder::from_nullable_points([Some(&point1), None].into_iter(), point_type.clone())
        .finish();

let coord = point_array.value(0).unwrap().coord().unwrap();
assert_eq!(coord.x(), 1.);
assert_eq!(coord.y(), 2.);
```

##### [§](#errors-1)Errors

Errors for invalid WKT and WKB geometries. Will never error for native arrays.

##### [§](#panics)Panics

Panics if the value is outside the bounds of the array.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#468-474)

Returns the value at slot `i` as an Arrow scalar, considering validity.

##### [§](#examples-2)Examples

```
let point1 = geo_types::point!(x: 1., y: 2.);

let point_type = PointType::new(Dimension::XY, Default::default());
let point_array =
    PointBuilder::from_nullable_points([Some(&point1), None].into_iter(), point_type.clone())
        .finish();

assert!(point_array.get(0).unwrap().is_some());
assert!(point_array.get(1).unwrap().is_none());
```

##### [§](#errors-2)Errors

Errors for invalid WKT and WKB geometries. Will never error for native arrays.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#485-491)

Returns the value at slot `i` as an Arrow scalar, considering validity.

##### [§](#errors-3)Errors

Errors for invalid WKT and WKB geometries. Will never error for native arrays.

##### [§](#safety-1)Safety

Caller is responsible for ensuring that the index is within the bounds of the array

[Source](about:blank/src/geoarrow_array/trait_.rs.html#498-500)

Iterates over this array’s geoarrow scalar values, considering validity.

##### [§](#errors-4)Errors

Errors for invalid WKT and WKB geometries. Will never error for native arrays.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#507-509)

Iterator over geoarrow scalar values, not considering validity.

##### [§](#errors-5)Errors

Errors for invalid WKT and WKB geometries. Will never error for native arrays.

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe._
