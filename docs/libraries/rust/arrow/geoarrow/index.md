# geoarrow - Rust

Expand description

A complete, safe, native Rust implementation of [GeoArrow](https://geoarrow.org/), which adds geospatial support to the [Apache Arrow](https://arrow.apache.org/) tabular in-memory data format.

As of version 0.4, the `geoarrow` crate was refactored to a monorepo of smaller crates, each with a more well-defined scope. Users may want to depend on the subcrates manually:

- [`geoarrow_array`](https://docs.rs/geoarrow-array/0.5.0/x86_64-unknown-linux-gnu/geoarrow_array/index.html "mod geoarrow_array"): GeoArrow array definitions.
- [`geoarrow_cast`](https://docs.rs/geoarrow-cast/latest/geoarrow_cast/): Functions for converting from one GeoArrow geometry type to another.
- [`geoarrow_schema`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/index.html "mod geoarrow_schema"): GeoArrow geometry type and metadata definitions.

This crate is an “amalgam” crate, which just re-exports types from the underlying sub-crates.

[array](array/index.html "mod geoarrow::array")

Statically typed implementations of GeoArrow Arrays

[datatypes](datatypes/index.html "mod geoarrow::datatypes")

Defines the logical data types of GeoArrow arrays.

[error](error/index.html "mod geoarrow::error")

Defines `GeoArrowError` for representing failures in various GeoArrow operations.
