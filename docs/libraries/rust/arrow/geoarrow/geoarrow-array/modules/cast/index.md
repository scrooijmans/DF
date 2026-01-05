# geoarrow_array::cast - Rust

Expand description

Helper functions for downcasting [`dyn GeoArrowArray`](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to concrete types and for converting between GeoArrow array representations.

[AsGeoArrowArray](trait.AsGeoArrowArray.html "trait geoarrow_array::cast::AsGeoArrowArray")

Helpers for downcasting a [`GeoArrowArray`](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to a concrete implementation.

[from_wkb](fn.from_wkb.html "fn geoarrow_array::cast::from_wkb")

Parse a [`GenericWkbArray`](../array/struct.GenericWkbArray.html "struct geoarrow_array::array::GenericWkbArray") or [`WkbViewArray`](../array/struct.WkbViewArray.html "struct geoarrow_array::array::WkbViewArray") to a [`GeoArrowArray`](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") with the designated [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType").

[from_wkt](fn.from_wkt.html "fn geoarrow_array::cast::from_wkt")

Parse a [`GenericWktArray`](../array/struct.GenericWktArray.html "struct geoarrow_array::array::GenericWktArray") or [`WktViewArray`](../array/struct.WktViewArray.html "struct geoarrow_array::array::WktViewArray") to a [`GeoArrowArray`](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") with the designated [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType").

[to_wkb](fn.to_wkb.html "fn geoarrow_array::cast::to_wkb")

Convert a [GeoArrowArray](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to a [`GenericWkbArray`](../array/struct.GenericWkbArray.html "struct geoarrow_array::array::GenericWkbArray").

[to_wkb_view](fn.to_wkb_view.html "fn geoarrow_array::cast::to_wkb_view")

Convert a [GeoArrowArray](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to a [`WkbViewArray`](../array/struct.WkbViewArray.html "struct geoarrow_array::array::WkbViewArray").

[to_wkt](fn.to_wkt.html "fn geoarrow_array::cast::to_wkt")

Convert a [GeoArrowArray](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to a [`GenericWktArray`](../array/struct.GenericWktArray.html "struct geoarrow_array::array::GenericWktArray").

[to_wkt_view](fn.to_wkt_view.html "fn geoarrow_array::cast::to_wkt_view")

Convert a [GeoArrowArray](../trait.GeoArrowArray.html "trait geoarrow_array::GeoArrowArray") to a [`WktViewArray`](../array/struct.WktViewArray.html "struct geoarrow_array::array::WktViewArray").
