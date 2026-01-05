# GeoArrowType in geoarrow_schema - Rust

[![](https://github.com/geoarrow.png)](../geoarrow_schema/index.html)

```
pub enum GeoArrowType {
Show 15 variants    Point(PointType),
    LineString(LineStringType),
    Polygon(PolygonType),
    MultiPoint(MultiPointType),
    MultiLineString(MultiLineStringType),
    MultiPolygon(MultiPolygonType),
    GeometryCollection(GeometryCollectionType),
    Rect(BoxType),
    Geometry(GeometryType),
    Wkb(WkbType),
    LargeWkb(WkbType),
    WkbView(WkbType),
    Wkt(WktType),
    LargeWkt(WktType),
    WktView(WktType),
}
```

Expand description

Geospatial data types supported by GeoArrow.

The variants of this enum include all possible GeoArrow geometry types, including both “native” and “serialized” encodings.

Each variant uniquely identifies the physical buffer layout for the respective array type.

[§](#variant.Point)

A Point.

[§](#variant.LineString)

A LineString.

[§](#variant.Polygon)

A Polygon.

[§](#variant.MultiPoint)

A MultiPoint.

[§](#variant.MultiLineString)

A MultiLineString.

[§](#variant.MultiPolygon)

A MultiPolygon.

[§](#variant.GeometryCollection)

A GeometryCollection.

[§](#variant.Rect)

A Rect.

[§](#variant.Geometry)

A Geometry with unknown types or dimensions.

[§](#variant.Wkb)

A WKB stored in a `BinaryArray` with `i32` offsets.

[§](#variant.LargeWkb)

A WKB stored in a `LargeBinaryArray` with `i64` offsets.

[§](#variant.WkbView)

A WKB stored in a `BinaryViewArray`.

[§](#variant.Wkt)

A WKT stored in a `StringArray` with `i32` offsets.

[§](#variant.LargeWkt)

A WKT stored in a `LargeStringArray` with `i64` offsets.

[§](#variant.WktView)

A WKT stored in a `StringViewArray`.

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#76-394)
[§](#impl-GeoArrowType)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#80-94)

Get the [`CoordType`](enum.CoordType.html "enum geoarrow_schema::CoordType") of this data type.

WKB and WKT variants will return `None`.

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#99-114)

Get the [`Dimension`](enum.Dimension.html "enum geoarrow_schema::Dimension") of this data type, if it has one.

[`Geometry`](about:blank/enum.GeoArrowType.html#variant.Geometry "variant geoarrow_schema::GeoArrowType::Geometry") and WKB and WKT variants will return `None`.

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#117-132)

Returns the [Metadata](struct.Metadata.html "struct geoarrow_schema::Metadata") contained within this type.

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#148-167)

Converts a [`GeoArrowType`](enum.GeoArrowType.html "enum geoarrow_schema::GeoArrowType") into the relevant arrow [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

Note that an arrow [`DataType`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType") will lose the accompanying GeoArrow metadata if it is not part of a [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field") with GeoArrow extension metadata in its field metadata.

##### [§](#examples)Examples

```
let point_type = PointType::new(Dimension::XY, Default::default());
let data_type = GeoArrowType::Point(point_type).to_data_type();
assert!(matches!(data_type, DataType::Struct(_)));
```

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#184-203)

Converts this [`GeoArrowType`](enum.GeoArrowType.html "enum geoarrow_schema::GeoArrowType") into an arrow [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field"), maintaining GeoArrow extension metadata.

##### [§](#examples-1)Examples

```
let point_type = PointType::new(Dimension::XY, Default::default());
let geoarrow_type = GeoArrowType::Point(point_type);
let field = geoarrow_type.to_field("geometry", true);
assert_eq!(field.name(), "geometry");
assert!(field.is_nullable());
assert_eq!(field.metadata()
["ARROW:extension:name"], "geoarrow.point");
```

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#221-235)

Applies the provided [CoordType](enum.CoordType.html "enum geoarrow_schema::CoordType") onto self.

[`Rect`](about:blank/enum.GeoArrowType.html#variant.Rect "variant geoarrow_schema::GeoArrowType::Rect") and WKB and WKT variants will return the same type as they do not have a parameterized coordinate types.

##### [§](#examples-2)Examples

```
let point_type = PointType::new(Dimension::XY, Default::default());
let geoarrow_type = GeoArrowType::Point(point_type);
let new_type = geoarrow_type.with_coord_type(CoordType::Separated);

assert_eq!(new_type.coord_type(), Some(CoordType::Separated));
```

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#253-267)

Applies the provided [Dimension](enum.Dimension.html "enum geoarrow_schema::Dimension") onto self.

[`Geometry`](about:blank/enum.GeoArrowType.html#variant.Geometry "variant geoarrow_schema::GeoArrowType::Geometry") and WKB and WKT variants will return the same type as they do not have a parameterized dimension.

##### [§](#examples-3)Examples

```
let point_type = PointType::new(Dimension::XY, Default::default());
let geoarrow_type = GeoArrowType::Point(point_type);
let new_type = geoarrow_type.with_dimension(Dimension::XYZ);

assert_eq!(new_type.dimension(), Some(Dimension::XYZ));
```

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#270-289)

Applies the provided [Metadata](struct.Metadata.html "struct geoarrow_schema::Metadata") onto self.

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#297-342)

[Source](about:blank/src/geoarrow_schema/datatype.rs.html#351-393)

Create a new [`GeoArrowType`](enum.GeoArrowType.html "enum geoarrow_schema::GeoArrowType") from an Arrow [`Field`](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/field/struct.Field.html "struct arrow_schema::field::Field"), inferring the GeoArrow type if GeoArrow metadata is not present.

This will first try [`GeoArrowType::from_extension_field`](about:blank/enum.GeoArrowType.html#method.from_extension_field "associated function geoarrow_schema::GeoArrowType::from_extension_field"), and if that fails, will try to infer the GeoArrow type from the field’s [DataType](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType"). This only works for Point, WKB, and WKT types, as those are the only types that can be unambiguously inferred from an Arrow [DataType](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html "enum arrow_schema::datatype::DataType").

[§](#impl-Freeze-for-GeoArrowType)

[§](#impl-RefUnwindSafe-for-GeoArrowType)

[§](#impl-Send-for-GeoArrowType)

[§](#impl-Sync-for-GeoArrowType)

[§](#impl-Unpin-for-GeoArrowType)

[§](#impl-UnwindSafe-for-GeoArrowType)
