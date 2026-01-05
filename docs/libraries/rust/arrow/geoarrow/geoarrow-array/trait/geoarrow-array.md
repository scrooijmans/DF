# GeoArrowArray in geoarrow_array - Rust

[![](https://github.com/geoarrow.png)](../geoarrow_array/index.html)

## Trait GeoArrowArray

[Source](about:blank/src/geoarrow_array/trait_.rs.html#38-255)

```
pub trait GeoArrowArray:
    Debug
    + Send
    + Sync {
    // Required methods
    fn as_any(&self) -> &dyn Any;
    fn data_type(&self) -> GeoArrowType;
    fn into_array_ref(self) -> ArrayRef;
    fn to_array_ref(&self) -> ArrayRef;
    fn len(&self) -> usize;
    fn logical_nulls(&self) -> Option<NullBuffer>;
    fn logical_null_count(&self) -> usize;
    fn is_null(&self, i: usize) -> bool;
    fn slice(&self, offset: usize, length: usize) -> Arc<dyn GeoArrowArray>;
    fn with_metadata(self, metadata: Arc<Metadata>) -> Arc<dyn GeoArrowArray>;

    // Provided methods
    fn is_empty(&self) -> bool { ... }
    fn is_valid(&self, i: usize) -> bool { ... }
}
```

Expand description

A base trait for all GeoArrow arrays.

This is a geospatial corollary to the upstream [`Array`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html "trait arrow_array::array::Array") trait.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#43)

Returns the array as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcasted to a specific implementation.

Prefer using [`AsGeoArrowArray`](cast/trait.AsGeoArrowArray.html "trait geoarrow_array::cast::AsGeoArrowArray") instead of calling this method and manually downcasting.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#59)

Returns the [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType") of this array.

##### [§](#examples)Examples

```
let point = geo_types::point!(x: 1., y: 2.);
let point_type = PointType::new(Dimension::XY, Default::default());
let point_array = PointBuilder::from_points([point].iter(), point_type.clone()).finish();
assert_eq!(point_array.data_type(), GeoArrowType::Point(point_type));
```

[Source](about:blank/src/geoarrow_array/trait_.rs.html#84)

Converts this array into an `Arc`ed [`arrow`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") array, consuming the original array.

This is `O(1)`.

Note that **this will omit any spatial extension information**. You must separately store the spatial information in a [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field") derived from [`Self::data_type`](about:blank/trait.GeoArrowArray.html#tymethod.data_type).

##### [§](#examples-1)Examples

```
let point = geo_types::point!(x: 1., y: 2.);
let point_type = PointType::new(Dimension::XY, Default::default());
let point_array = PointBuilder::from_points([point].iter(), point_type.clone()).finish();
let array_ref: ArrayRef = point_array.into_array_ref();
```

[Source](about:blank/src/geoarrow_array/trait_.rs.html#108)

Converts this array into an `Arc`ed [`arrow`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/index.html "mod arrow_array") array.

This is `O(1)`.

Note that **this will omit any spatial extension information**. You must separately store the spatial information in a [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field") derived from [`Self::data_type`](about:blank/trait.GeoArrowArray.html#tymethod.data_type).

##### [§](#examples-2)Examples

```
let point = geo_types::point!(x: 1., y: 2.);
let point_type = PointType::new(Dimension::XY, Default::default());
let point_array = PointBuilder::from_points([point].iter(), point_type.clone()).finish();
let array_ref: ArrayRef = point_array.to_array_ref();
```

[Source](about:blank/src/geoarrow_array/trait_.rs.html#125)

The number of geometries contained in this array.

##### [§](#examples-3)Examples

```
let point = geo_types::point!(x: 1., y: 2.);
let point_type = PointType::new(Dimension::XY, Default::default());
let point_array = PointBuilder::from_points([point].iter(), point_type.clone()).finish();
assert_eq!(point_array.len(), 1);
```

[Source](about:blank/src/geoarrow_array/trait_.rs.html#157)

Returns a potentially computed [\`NullBuffer\`\`](https://docs.rs/arrow-buffer/56.0.0/x86_64-unknown-linux-gnu/arrow_buffer/buffer/null/struct.NullBuffer.html "struct arrow_buffer::buffer::null::NullBuffer") that represents the logical null values of this array, if any.

Logical nulls represent the values that are null in the array, regardless of the underlying physical arrow representation.

For most array types, this is equivalent to the “physical” nulls returned by [`Array::nulls`](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html#tymethod.nulls "method arrow_array::array::Array::nulls"). However it is different for union arrays, including our [`GeometryArray`](array/struct.GeometryArray.html "struct geoarrow_array::array::GeometryArray") and [`GeometryCollectionArray`](array/struct.GeometryCollectionArray.html "struct geoarrow_array::array::GeometryCollectionArray") types, because the unions aren’t encoded in a single null buffer.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#176)

Returns the number of null slots in this array.

This is `O(1)` since the number of null elements is pre-computed.

##### [§](#examples-4)Examples

```
let point = geo_types::point!(x: 1., y: 2.);
let point_type = PointType::new(Dimension::XY, Default::default());
let point_array =
    PointBuilder::from_nullable_points([Some(&point), None].into_iter(), point_type.clone()).finish();
assert_eq!(point_array.logical_null_count(), 1);
```

[Source](about:blank/src/geoarrow_array/trait_.rs.html#198)

Returns whether slot `i` is null.

##### [§](#examples-5)Examples

```
let point = geo_types::point!(x: 1., y: 2.);

let point_type = PointType::new(Dimension::XY, Default::default());
let point_array =
    PointBuilder::from_nullable_points([Some(&point), None].into_iter(), point_type.clone()).finish();
assert!(point_array.is_null(1));
```

##### [§](#panics)Panics

Panics iff `i >= self.len()`.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#251)

Returns a zero-copy slice of this array with the indicated offset and length.

##### [§](#examples-6)Examples

```
let point1 = geo_types::point!(x: 1., y: 2.);
let point2 = geo_types::point!(x: 3., y: 4.);

let point_type = PointType::new(Dimension::XY, Default::default());
let point_array =
    Arc::new(PointBuilder::from_points([point1, point2].iter(), point_type.clone()).finish())
        as Arc<dyn GeoArrowArray>;
let sliced_array = point_array.slice(1, 1);
assert_eq!(sliced_array.len(), 1);
```

##### [§](#panics-1)Panics

This function panics iff `offset + length > self.len()`.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#254)

Change the [`Metadata`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/metadata/struct.Metadata.html "struct geoarrow_schema::metadata::Metadata") of this array.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#142-144)

Returns `true` if the array is empty.

##### [§](#examples-7)Examples

```
let point = geo_types::point!(x: 1., y: 2.);
let point_type = PointType::new(Dimension::XY, Default::default());
let point_array = PointBuilder::from_points([point].iter(), point_type.clone()).finish();
assert!(!point_array.is_empty());
```

[Source](about:blank/src/geoarrow_array/trait_.rs.html#221-223)

Returns whether slot `i` is valid.

##### [§](#examples-8)Examples

```
let point = geo_types::point!(x: 1., y: 2.);

let point_type = PointType::new(Dimension::XY, Default::default());
let point_array =
    PointBuilder::from_nullable_points([Some(&point), None].into_iter(), point_type.clone()).finish();
assert!(point_array.is_valid(0));
```

##### [§](#panics-2)Panics

Panics iff `i >= self.len()`.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#258-302)
[§](#impl-GeoArrowArray-for-Arc%3Cdyn+GeoArrowArray%3E)

Ergonomics: Allow use of an `Arc<dyn GeoArrowArray>` as an `&dyn GeoArrowArray`

[Source](about:blank/src/geoarrow_array/trait_.rs.html#259-261)
[§](#method.as_any)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#263-265)
[§](#method.data_type)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#267-269)
[§](#method.into_array_ref)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#271-273)
[§](#method.to_array_ref)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#275-277)
[§](#method.len)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#279-281)
[§](#method.logical_nulls)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#283-285)
[§](#method.logical_null_count)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#287-289)
[§](#method.is_null)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#291-293)
[§](#method.slice)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#295-301)
[§](#method.with_metadata)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#304-348)
[§](#impl-GeoArrowArray-for-%26T)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#305-307)
[§](#method.as_any-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#309-311)
[§](#method.data_type-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#313-315)
[§](#method.into_array_ref-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#317-319)
[§](#method.to_array_ref-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#321-323)
[§](#method.len-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#325-327)
[§](#method.logical_nulls-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#329-331)
[§](#method.logical_null_count-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#333-335)
[§](#method.is_null-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#337-339)
[§](#method.slice-1)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#341-347)
[§](#method.with_metadata-1)

[Source](about:blank/src/geoarrow_array/array/geometry.rs.html#319-376)
[§](#impl-GeoArrowArray-for-GeometryArray)

[Source](about:blank/src/geoarrow_array/array/geometrycollection.rs.html#115-162)
[§](#impl-GeoArrowArray-for-GeometryCollectionArray)

[Source](about:blank/src/geoarrow_array/array/linestring.rs.html#165-211)
[§](#impl-GeoArrowArray-for-LineStringArray)

[Source](about:blank/src/geoarrow_array/array/multilinestring.rs.html#199-246)
[§](#impl-GeoArrowArray-for-MultiLineStringArray)

[Source](about:blank/src/geoarrow_array/array/multipoint.rs.html#169-216)
[§](#impl-GeoArrowArray-for-MultiPointArray)

[Source](about:blank/src/geoarrow_array/array/multipolygon.rs.html#234-281)
[§](#impl-GeoArrowArray-for-MultiPolygonArray)

[Source](about:blank/src/geoarrow_array/array/point.rs.html#130-177)
[§](#impl-GeoArrowArray-for-PointArray)

[Source](about:blank/src/geoarrow_array/array/polygon.rs.html#201-248)
[§](#impl-GeoArrowArray-for-PolygonArray)

[Source](about:blank/src/geoarrow_array/array/rect.rs.html#101-148)
[§](#impl-GeoArrowArray-for-RectArray)

[Source](about:blank/src/geoarrow_array/array/wkb_view.rs.html#66-110)
[§](#impl-GeoArrowArray-for-WkbViewArray)

[Source](about:blank/src/geoarrow_array/array/wkt_view.rs.html#72-116)
[§](#impl-GeoArrowArray-for-WktViewArray)

[Source](about:blank/src/geoarrow_array/array/wkb.rs.html#95-143)
[§](#impl-GeoArrowArray-for-GenericWkbArray%3CO%3E)

[Source](about:blank/src/geoarrow_array/array/wkt.rs.html#76-124)
[§](#impl-GeoArrowArray-for-GenericWktArray%3CO%3E)
