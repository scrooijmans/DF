# GeoArrowArrayReader in geoarrow_array - Rust

## Trait GeoArrowArrayReader

[Source](about:blank/src/geoarrow_array/trait_.rs.html#548-554)

```
pub trait GeoArrowArrayReader: Iterator<Item = GeoArrowResult<Arc<dyn GeoArrowArray>>> {
    // Required method
    fn data_type(&self) -> GeoArrowType;
}
```

Expand description

Trait for types that can read `Arc<dyn GeoArrowArray>`’s.

There is no direct parallel to this in the upstream \[arrow-array\] crate. The closest is [RecordBatchReader](https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/arrow_array/record_batch/trait.RecordBatchReader.html "trait arrow_array::record_batch::RecordBatchReader"), which has the same implementation over an iterator of `RecordBatch`es. However, it is useful to have an iterator of GeoArrow arrays with a known [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType").

To create from an iterator, see [GeoArrowArrayIterator](struct.GeoArrowArrayIterator.html "struct geoarrow_array::GeoArrowArrayIterator").

[Source](about:blank/src/geoarrow_array/trait_.rs.html#553)

Returns the field of this `GeoArrowArrayReader`.

Implementation of this trait should guarantee that all `Arc<dyn GeoArrowArray>`’s returned by this reader should have the same [`GeoArrowType`](https://docs.rs/geoarrow-schema/0.5.0/x86_64-unknown-linux-gnu/geoarrow_schema/datatype/enum.GeoArrowType.html "enum geoarrow_schema::datatype::GeoArrowType") as returned from this method.

[Source](about:blank/src/geoarrow_array/trait_.rs.html#556-560)
[§](#impl-GeoArrowArrayReader-for-Box%3CR%3E)

[Source](about:blank/src/geoarrow_array/trait_.rs.html#557-559)
[§](#method.data_type)
