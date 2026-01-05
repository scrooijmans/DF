# Crs in geoarrow_schema::crs - Rust

```
pub struct Crs { /* private fields */ }
```

Expand description

Coordinate Reference System information.

As of GeoArrow version 0.2, GeoArrow supports various CRS representations:

- A JSON object describing the coordinate reference system (CRS) using [PROJJSON](https://proj.org/specifications/projjson.html).
- A string containing a serialized CRS representation. This option is intended as a fallback for producers (e.g., database drivers or file readers) that are provided a CRS in some form but do not have the means to convert it to PROJJSON.
- Omitted, indicating that the producer does not have any information about the CRS.

For maximum compatibility, producers should write PROJJSON.

Note that regardless of the axis order specified by the CRS, axis order will be interpreted according to the wording in the [GeoPackage WKB binary encoding](https://www.geopackage.org/spec130/index.html#gpb_format): axis order is always (longitude, latitude) and (easting, northing) regardless of the the axis order encoded in the CRS specification.

Note that [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") and [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") currently use their default, derived implementations, so only `Crs` that are structurally exactly equal will compare as equal. Two different representations of the same logical CRS will not compare as equal.

[Source](about:blank/src/geoarrow_schema/crs.rs.html#64-136)
[§](#impl-Crs)

[Source](about:blank/src/geoarrow_schema/crs.rs.html#69-74)

Construct from a PROJJSON object.

Note that `value` should be a _parsed_ JSON object; this should not contain `Value::String`.

[Source](about:blank/src/geoarrow_schema/crs.rs.html#77-82)

Construct from a WKT:2019 string.

[Source](about:blank/src/geoarrow_schema/crs.rs.html#85-90)

Construct from an opaque string.

Construct from an authority:code string.

[Source](about:blank/src/geoarrow_schema/crs.rs.html#102-107)

Construct from an opaque string identifier

[Source](about:blank/src/geoarrow_schema/crs.rs.html#110-112)

Access the underlying [CrsType](enum.CrsType.html "enum geoarrow_schema::crs::CrsType").

[Source](about:blank/src/geoarrow_schema/crs.rs.html#128-130)

Access the underlying CRS value.

The return value is one of:

- A JSON object ([`Value::Object`](https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html#variant.Object "variant serde_json::value::Value::Object")) describing the coordinate reference system (CRS) using [PROJJSON](https://proj.org/specifications/projjson.html).
- A string ([`Value::String`](https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html#variant.String "variant serde_json::value::Value::String")) containing a serialized CRS representation. This option is intended as a fallback for producers (e.g., database drivers or file readers) that are provided a CRS in some form but do not have the means to convert it to PROJJSON.
- Omitted, indicating that the producer does not have any information about the CRS.

Consult [`crs_type`](about:blank/struct.Crs.html#method.crs_type "method geoarrow_schema::crs::Crs::crs_type") to accurately determine the CRS type.

[§](#impl-Freeze-for-Crs)

[§](#impl-RefUnwindSafe-for-Crs)

[§](#impl-Send-for-Crs)

[§](#impl-Sync-for-Crs)

[§](#impl-Unpin-for-Crs)

[§](#impl-UnwindSafe-for-Crs)
