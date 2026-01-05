# CrsType in geoarrow_schema::crs - Rust

```
pub enum CrsType {
    Projjson,
    Wkt2_2019,
    AuthorityCode,
    Srid,
}
```

Expand description

An optional string disambiguating the value of the `crs` field.

[§](#variant.Projjson)

Indicates that the `"crs"` field was written as [PROJJSON](https://proj.org/specifications/projjson.html).

[§](#variant.Wkt2_2019)

Indicates that the `"crs"` field was written as [WKT2:2019](https://www.ogc.org/publications/standard/wkt-crs/).

Indicates that the `"crs"` field contains an identifier in the form `AUTHORITY:CODE`. This should only be used as a last resort (i.e., producers should prefer writing a complete description of the CRS).

[§](#variant.Srid)

Indicates that the `"crs"` field contains an opaque identifier that requires the consumer to communicate with the producer outside of this metadata. This should only be used as a last resort for database drivers or readers that have no other option.

[§](#impl-Freeze-for-CrsType)

[§](#impl-RefUnwindSafe-for-CrsType)

[§](#impl-Send-for-CrsType)

[§](#impl-Sync-for-CrsType)

[§](#impl-Unpin-for-CrsType)

[§](#impl-UnwindSafe-for-CrsType)
