# GeometryCollectionType in geoarrow_schema - Rust

```
use std::sync::Arc;

use arrow_schema::{DataType, Field, UnionFields, UnionMode};
use geoarrow_schema::{
    Dimension, GeometryCollectionType, LineStringType, Metadata, MultiLineStringType,
    MultiPointType, MultiPolygonType, PointType, PolygonType,
};

let dim = Dimension::XY;
let metadata = Arc::new(Metadata::default());
let geom_type = GeometryCollectionType::new(dim, metadata.clone());

let fields = vec![
    Field::new(
        "Point",
        PointType::new(dim, metadata.clone()).data_type(),
        true,
    ),
    Field::new(
        "LineString",
        LineStringType::new(dim, metadata.clone()).data_type(),
        true,
    ),
    Field::new(
        "Polygon",
        PolygonType::new(dim, metadata.clone()).data_type(),
        true,
    ),
    Field::new(
        "MultiPoint",
        MultiPointType::new(dim, metadata.clone()).data_type(),
        true,
    ),
    Field::new(
        "MultiLineString",
        MultiLineStringType::new(dim, metadata.clone()).data_type(),
        true,
    ),
    Field::new(
        "MultiPolygon",
        MultiPolygonType::new(dim, metadata.clone()).data_type(),
        true,
    ),
];
let type_ids = vec![1, 2, 3, 4, 5, 6];

let union_fields = UnionFields::new(type_ids, fields);
let union_data_type = DataType::Union(union_fields, UnionMode::Dense);

let geometries_field = Field::new("geometries", union_data_type, false).into();
let expected_type = DataType::List(geometries_field);

assert_eq!(geom_type.data_type(), expected_type);
```
