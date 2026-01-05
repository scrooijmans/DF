# geoarrow_array::capacity - Rust

Expand description

Counters for managing buffer lengths for each geometry array type.

The most memory-efficient way to construct an array from a set of geometries is to make a first pass over these geometries to count exactly how big each underlying buffer of the Arrow array must be, allocate _once_ for exactly what you need, and then fill those buffers in a second pass. Capacity counters help with this process.

[GeometryCapacity](struct.GeometryCapacity.html "struct geoarrow_array::capacity::GeometryCapacity")

A counter for the buffer sizes of a [`GeometryArray`](../array/struct.GeometryArray.html "struct geoarrow_array::array::GeometryArray").

[GeometryCollectionCapacity](struct.GeometryCollectionCapacity.html "struct geoarrow_array::capacity::GeometryCollectionCapacity")

A counter for the buffer sizes of a [`GeometryCollectionArray`](../array/struct.GeometryCollectionArray.html "struct geoarrow_array::array::GeometryCollectionArray").

[LineStringCapacity](struct.LineStringCapacity.html "struct geoarrow_array::capacity::LineStringCapacity")

A counter for the buffer sizes of a [`LineStringArray`](../array/struct.LineStringArray.html "struct geoarrow_array::array::LineStringArray").

[MultiLineStringCapacity](struct.MultiLineStringCapacity.html "struct geoarrow_array::capacity::MultiLineStringCapacity")

A counter for the buffer sizes of a [`MultiLineStringArray`](../array/struct.MultiLineStringArray.html "struct geoarrow_array::array::MultiLineStringArray").

[MultiPointCapacity](struct.MultiPointCapacity.html "struct geoarrow_array::capacity::MultiPointCapacity")

A counter for the buffer sizes of a [`MultiPointArray`](../array/struct.MultiPointArray.html "struct geoarrow_array::array::MultiPointArray").

[MultiPolygonCapacity](struct.MultiPolygonCapacity.html "struct geoarrow_array::capacity::MultiPolygonCapacity")

A counter for the buffer sizes of a [`MultiPolygonArray`](../array/struct.MultiPolygonArray.html "struct geoarrow_array::array::MultiPolygonArray").

[PolygonCapacity](struct.PolygonCapacity.html "struct geoarrow_array::capacity::PolygonCapacity")

A counter for the buffer sizes of a [`PolygonArray`](../array/struct.PolygonArray.html "struct geoarrow_array::array::PolygonArray").

[WkbCapacity](struct.WkbCapacity.html "struct geoarrow_array::capacity::WkbCapacity")

A counter for the buffer sizes of a [`GenericWkbArray`](../array/struct.GenericWkbArray.html "struct geoarrow_array::array::GenericWkbArray").
