# CrsTransform in geoarrow_schema::crs - Rust

```
pub trait CrsTransform: Debug {
    // Required methods
    fn _convert_to_projjson(&self, crs: &Crs) -> GeoArrowResult<Option<Value>>;
    fn _convert_to_wkt(&self, crs: &Crs) -> GeoArrowResult<Option<String>>;

    // Provided methods
    fn extract_projjson(&self, crs: &Crs) -> GeoArrowResult<Option<Value>> { ... }
    fn extract_wkt(&self, crs: &Crs) -> GeoArrowResult<Option<String>> { ... }
}
```

Expand description

CRS transforms used for writing GeoArrow data to file formats that require different CRS representations.

[Source](about:blank/src/geoarrow_schema/crs.rs.html#172)

Convert the CRS contained in this Metadata to a PROJJSON object.

Users should prefer calling `extract_projjson`, which will first unwrap the underlying array metadata if it’s already PROJJSON.

[Source](about:blank/src/geoarrow_schema/crs.rs.html#178)

Convert the CRS contained in this Metadata to a WKT string.

Users should prefer calling `extract_wkt`, which will first unwrap the underlying array metadata if it’s already PROJJSON.

Extract PROJJSON from the provided metadata.

If the CRS is already stored as PROJJSON, this will return that. Otherwise it will call [`Self::_convert_to_projjson`](about:blank/trait.CrsTransform.html#tymethod._convert_to_projjson).

Extract WKT from the provided metadata.

If the CRS is already stored as WKT, this will return that. Otherwise it will call [`Self::_convert_to_wkt`](about:blank/trait.CrsTransform.html#tymethod._convert_to_wkt).

[Source](about:blank/src/geoarrow_schema/crs.rs.html#214-228)
[§](#impl-CrsTransform-for-DefaultCrsTransform)
