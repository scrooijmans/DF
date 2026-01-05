https://docs.rs/geoarrow-cast/latest/geoarrow_cast/cast/fn.cast.html

# cast in geoarrow_cast::cast - Rust

```
pub fn cast(
    array: &dyn GeoArrowArray,
    to_type: &GeoArrowType,
) -> GeoArrowResult<Arc<dyn GeoArrowArray>>
```

Expand description

Cast a [`GeoArrowArray`](https://docs.rs/geoarrow-array/0.5.0/x86_64-unknown-linux-gnu/geoarrow_array/trait_/trait.GeoArrowArray.html "trait geoarrow_array::trait_::GeoArrowArray") to another [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType").

#### [§](#criteria)Criteria:

- Dimension must be compatible:
  - If the source array and destination type are both dimension-aware, then their dimensions must match.
  - Casts from dimension-aware to dimensionless arrays (`GeometryArray`, `WkbArray`, `WkbViewArray`, `WktArray`, `WktViewArray`) are always allowed.
- GeoArrow [`Metadata`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/metadata/struct.Metadata.html "struct geoarrow_schema::metadata::Metadata") on the [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType") must match. Use [`GeoArrowArray::with_metadata`](https://docs.rs/geoarrow-array/0.5.0/x86_64-unknown-linux-gnu/geoarrow_array/trait_/trait.GeoArrowArray.html#tymethod.with_metadata "method geoarrow_array::trait_::GeoArrowArray::with_metadata") to change the metadata on an array.

#### [§](#infallible-casts)Infallible casts:

As long as the above criteria are met, these casts will always succeed without erroring.

- The same geometry type with different coord types.
- Any source array type to `Geometry`, `Wkb`, `LargeWkb`, `WkbView`, `Wkt`, `LargeWkt`, or `WktView`.
- `Point` to `MultiPoint`
- `LineString` to `MultiLineString`
- `Polygon` to `MultiPolygon`

#### [§](#fallible-casts)Fallible casts:

- `Geometry` to any other native type.
- Parsing `WKB` or `WKT` to any native type other than `Geometry`.
- `MultiPoint` to `Point`
- `MultiLineString` to `LineString`
- `MultiPolygon` to `Polygon`
