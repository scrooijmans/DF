# geoarrow_array::array - Rust

Expand description

The concrete array definitions.

All arrays implement the core [GeoArrowArray](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") trait.

[GenericWkbArray](struct.GenericWkbArray.html "struct geoarrow_array::array::GenericWkbArray")

An immutable array of WKB geometries.

[GenericWktArray](struct.GenericWktArray.html "struct geoarrow_array::array::GenericWktArray")

An immutable array of WKT geometries using GeoArrow’s in-memory representation.

[GeometryArray](struct.GeometryArray.html "struct geoarrow_array::array::GeometryArray")

An immutable array of geometries of unknown geometry type and dimension.

[GeometryCollectionArray](struct.GeometryCollectionArray.html "struct geoarrow_array::array::GeometryCollectionArray")

An immutable array of GeometryCollection geometries.

[InterleavedCoordBuffer](struct.InterleavedCoordBuffer.html "struct geoarrow_array::array::InterleavedCoordBuffer")

An array of coordinates stored interleaved in a single buffer.

[LineStringArray](struct.LineStringArray.html "struct geoarrow_array::array::LineStringArray")

An immutable array of LineString geometries.

[MultiLineStringArray](struct.MultiLineStringArray.html "struct geoarrow_array::array::MultiLineStringArray")

An immutable array of MultiLineString geometries.

[MultiPointArray](struct.MultiPointArray.html "struct geoarrow_array::array::MultiPointArray")

An immutable array of MultiPoint geometries.

[MultiPolygonArray](struct.MultiPolygonArray.html "struct geoarrow_array::array::MultiPolygonArray")

An immutable array of MultiPolygon geometries.

[PointArray](struct.PointArray.html "struct geoarrow_array::array::PointArray")

An immutable array of Point geometries.

[PolygonArray](struct.PolygonArray.html "struct geoarrow_array::array::PolygonArray")

An immutable array of Polygon geometries using GeoArrow’s in-memory representation.

[RectArray](struct.RectArray.html "struct geoarrow_array::array::RectArray")

An immutable array of Rect or Box geometries.

[SeparatedCoordBuffer](struct.SeparatedCoordBuffer.html "struct geoarrow_array::array::SeparatedCoordBuffer")

An array of coordinates stored in separate buffers of the same length.

[WkbViewArray](struct.WkbViewArray.html "struct geoarrow_array::array::WkbViewArray")

An immutable array of WKB geometries.

[WktViewArray](struct.WktViewArray.html "struct geoarrow_array::array::WktViewArray")

An immutable array of WKT geometries.

[CoordBuffer](enum.CoordBuffer.html "enum geoarrow_array::array::CoordBuffer")

An Arrow representation of an array of coordinates.

[GenericWkbArrayType](trait.GenericWkbArrayType.html "trait geoarrow_array::array::GenericWkbArrayType")

A trait for GeoArrow arrays that can hold WKB data.

[GenericWktArrayType](trait.GenericWktArrayType.html "trait geoarrow_array::array::GenericWktArrayType")

A trait for GeoArrow arrays that can hold WKT data.

[from_arrow_array](fn.from_arrow_array.html "fn geoarrow_array::array::from_arrow_array")

Construct a new [GeoArrowArray](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") from an Arrow [Array](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/array/trait.Array.html "trait arrow_array::array::Array") and [Field](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field").

[LargeWkbArray](type.LargeWkbArray.html "type geoarrow_array::array::LargeWkbArray")

A [`GenericWkbArray`](struct.GenericWkbArray.html "struct geoarrow_array::array::GenericWkbArray") using `i64` offsets

[LargeWktArray](type.LargeWktArray.html "type geoarrow_array::array::LargeWktArray")

A [`GenericWktArray`](struct.GenericWktArray.html "struct geoarrow_array::array::GenericWktArray") using `i64` offsets

[WkbArray](type.WkbArray.html "type geoarrow_array::array::WkbArray")

A [`GenericWkbArray`](struct.GenericWkbArray.html "struct geoarrow_array::array::GenericWkbArray") using `i32` offsets

[WktArray](type.WktArray.html "type geoarrow_array::array::WktArray")

A [`GenericWktArray`](struct.GenericWktArray.html "struct geoarrow_array::array::GenericWktArray") using `i32` offsets
