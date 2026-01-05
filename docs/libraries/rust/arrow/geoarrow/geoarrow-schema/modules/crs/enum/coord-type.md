# CoordType in geoarrow_schema - Rust

```
pub enum CoordType {
    Interleaved,
    Separated,
}
```

Expand description

The permitted GeoArrow coordinate representations.

GeoArrow permits coordinate types to either be “Interleaved”, where the X and Y coordinates are in a single buffer as `XYXYXY` or “Separated”, where the X and Y coordinates are in multiple buffers as `XXXX` and `YYYY`.

[§](#variant.Interleaved)

Interleaved coordinates.

This stores coordinates in an Arrow [fixed-size-list-typed](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html#variant.FixedSizeList "variant arrow_schema::datatype::DataType::FixedSizeList") array.

The size of the internal fixed-size list depends on the [dimension](enum.Dimension.html "enum geoarrow_schema::Dimension") of the array.

```
FixedSizeList<double>[n_dim]
```

[§](#variant.Separated)

Separated coordinates.

This stores coordinates in an Arrow [struct-typed](https://docs.rs/arrow-schema/56.0.0/x86_64-unknown-linux-gnu/arrow_schema/datatype/enum.DataType.html#variant.Struct "variant arrow_schema::datatype::DataType::Struct") array:

```
Struct<x: double, y: double, [z: double, [m: double>]]
```

[§](#impl-Freeze-for-CoordType)

[§](#impl-RefUnwindSafe-for-CoordType)

[§](#impl-Send-for-CoordType)

[§](#impl-Sync-for-CoordType)

[§](#impl-Unpin-for-CoordType)

[§](#impl-UnwindSafe-for-CoordType)
