# geoarrow_schema::crs - Rust

Expand description

Defines GeoArrow CRS metadata and CRS transforms used for writing GeoArrow data to file formats that require different CRS representations.

## Structs[§](#structs)

[Crs](struct.Crs.html "struct geoarrow_schema::crs::Crs")

Coordinate Reference System information.

[DefaultCrsTransform](struct.DefaultCrsTransform.html "struct geoarrow_schema::crs::DefaultCrsTransform")

A default implementation for [CrsTransform](trait.CrsTransform.html "trait geoarrow_schema::crs::CrsTransform") which does not do any CRS conversion.

## Enums[§](#enums)

[CrsType](enum.CrsType.html "enum geoarrow_schema::crs::CrsType")

An optional string disambiguating the value of the `crs` field.

## Traits[§](#traits)

[CrsTransform](trait.CrsTransform.html "trait geoarrow_schema::crs::CrsTransform")

CRS transforms used for writing GeoArrow data to file formats that require different CRS representations.
