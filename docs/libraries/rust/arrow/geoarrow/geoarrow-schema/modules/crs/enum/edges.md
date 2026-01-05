# Edges in geoarrow_schema - Rust

[![](https://github.com/geoarrow.png)](../geoarrow_schema/index.html)

```
pub enum Edges {
    Andoyer,
    Karney,
    Spherical,
    Thomas,
    Vincenty,
}
```

Expand description

The edge interpretation between explicitly defined vertices.

This does not affect format conversions (e.g., parsing `geoarrow.wkb` as `geoarrow.linestring`), but does affect distance, intersection, bounding, overlay, length, and area calculations. The `edges` key must be omitted to indicate planar/linear edges or be one of:

If the `edges` key is omitted, edges will be interpreted following the language of [Simple features access](https://www.opengeospatial.org/standards/sfa):

> **simple feature** feature with all geometric attributes described piecewise by straight line or planar interpolation between sets of points (Section 4.19).

If an implementation only has support for a single edge interpretation (e.g., a library with only planar edge support), an array with a different edge type may be imported without losing information if the geometries in the array do not contain edges (e.g., `geoarrow.point`, `geoarrow.multipoint`, a `geoarrow.wkb`/`geoarrow.wkt` that only contains points and multipoints, or any array that only contains empty geometries). For arrays that contain edges, the error introduced by ignoring the original edge interpretation is similar to the error introduced by applying a coordinate transformation to vertices (which is usually small but may be large or create invalid geometries, particularly if vertices are not closely spaced). Ignoring the original edge interpretation will silently introduce invalid and/or misinterpreted geometries for any edge that crosses the antimeridian (i.e., longitude 180/-180) when translating from non-planar to planar edges.

Implementations may implicitly import arrays with an unsupported edge type if the arrays do not contain edges. Implementations may otherwise import arrays with an unsupported edge type with an explicit opt-in from a user or if accompanied by a prominent warning.

[§](#variant.Andoyer)

Edges in the longitude-latitude dimensions follow a path calculated by the fomula in Thomas, Paul D. Mathematical models for navigation systems. US Naval Oceanographic Office, 1965 using the ellipsoid specified by the `"crs"`.

[§](#variant.Karney)

[§](#variant.Spherical)

Edges in the longitude-latitude dimensions follow the shortest distance between vertices approximated as the shortest distance between the vertices on a perfect sphere. This edge interpretation is used by [BigQuery Geography](https://cloud.google.com/bigquery/docs/geospatial-data#coordinate_systems_and_edges), and [Snowflake Geography](https://docs.snowflake.com/en/sql-reference/data-types-geospatial).

A common library for interpreting edges in this way is [Google’s s2geometry](https://github.com/google/s2geometry); a common formula for calculating distances along this trajectory is the [Haversine Formula](https://en.wikipedia.org/wiki/Haversine_formula).

[§](#variant.Thomas)

Edges in the longitude-latitude dimensions follow a path calculated by the fomula in Thomas, Paul D. Spheroidal geodesics, reference systems, & local geometry. US Naval Oceanographic Office, 1970 using the ellipsoid specified by the `"crs"`.

[§](#variant.Vincenty)

Edges in the longitude-latitude dimensions follow a path calculated using [Vincenty’s formula](https://en.wikipedia.org/wiki/Vincenty%27s_formulae) and the ellipsoid specified by the `"crs"`.

[§](#impl-Freeze-for-Edges)

[§](#impl-RefUnwindSafe-for-Edges)

[§](#impl-Send-for-Edges)

[§](#impl-Sync-for-Edges)

[§](#impl-Unpin-for-Edges)

[§](#impl-UnwindSafe-for-Edges)
