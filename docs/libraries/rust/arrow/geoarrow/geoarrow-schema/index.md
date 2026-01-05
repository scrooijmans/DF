# geoarrow_schema - Rust

## Crate geoarrow_schema

[Source](about:blank/src/geoarrow_schema/lib.rs.html#1-28)

Expand description

## [§](#geoarrow-schema)geoarrow-schema

GeoArrow geometry type and metadata definitions.

All geometry type definitions, such as [`PointType`](struct.PointType.html "struct geoarrow_schema::PointType"), [`GeometryType`](struct.GeometryType.html "struct geoarrow_schema::GeometryType"), or [`WkbType`](struct.WkbType.html "struct geoarrow_schema::WkbType") implement the upstream [`ExtensionType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/extension/trait.ExtensionType.html "trait arrow_schema::extension::ExtensionType") trait.

Instances of type definitions are included within the variants on the [`GeoArrowType`](enum.GeoArrowType.html "enum geoarrow_schema::GeoArrowType") enum.

`pub use crs::[Crs](crs/struct.Crs.html "struct geoarrow_schema::crs::Crs");`

`pub use crs::[CrsType](crs/enum.CrsType.html "enum geoarrow_schema::crs::CrsType");`

[crs](crs/index.html "mod geoarrow_schema::crs")

Defines GeoArrow CRS metadata and CRS transforms used for writing GeoArrow data to file formats that require different CRS representations.

[error](error/index.html "mod geoarrow_schema::error")

Defines [`GeoArrowError`](error/enum.GeoArrowError.html "enum geoarrow_schema::error::GeoArrowError"), representing all errors returned by this crate.

[BoxType](struct.BoxType.html "struct geoarrow_schema::BoxType")

A GeoArrow “Box” or “Rect” type.

[GeometryCollectionType](struct.GeometryCollectionType.html "struct geoarrow_schema::GeometryCollectionType")

A GeoArrow GeometryCollection type.

[GeometryType](struct.GeometryType.html "struct geoarrow_schema::GeometryType")

A GeoArrow Geometry type.

[LineStringType](struct.LineStringType.html "struct geoarrow_schema::LineStringType")

A GeoArrow LineString type.

[Metadata](struct.Metadata.html "struct geoarrow_schema::Metadata")

GeoArrow extension metadata.

[MultiLineStringType](struct.MultiLineStringType.html "struct geoarrow_schema::MultiLineStringType")

A GeoArrow MultiLineString type.

[MultiPointType](struct.MultiPointType.html "struct geoarrow_schema::MultiPointType")

A GeoArrow MultiPoint type.

[MultiPolygonType](struct.MultiPolygonType.html "struct geoarrow_schema::MultiPolygonType")

A GeoArrow MultiPolygon type.

[PointType](struct.PointType.html "struct geoarrow_schema::PointType")

A GeoArrow Point type.

[PolygonType](struct.PolygonType.html "struct geoarrow_schema::PolygonType")

A GeoArrow Polygon type.

[WkbType](struct.WkbType.html "struct geoarrow_schema::WkbType")

A GeoArrow WKB type.

[WktType](struct.WktType.html "struct geoarrow_schema::WktType")

A GeoArrow WKT type.

[CoordType](enum.CoordType.html "enum geoarrow_schema::CoordType")

The permitted GeoArrow coordinate representations.

[Dimension](enum.Dimension.html "enum geoarrow_schema::Dimension")

The dimension of the geometry array.

[Edges](enum.Edges.html "enum geoarrow_schema::Edges")

The edge interpretation between explicitly defined vertices.

[GeoArrowType](enum.GeoArrowType.html "enum geoarrow_schema::GeoArrowType")

Geospatial data types supported by GeoArrow.
