# Statistics in parquet::file::statistics - Rust

```
pub enum Statistics {
    Boolean(ValueStatistics<bool>),
    Int32(ValueStatistics<i32>),
    Int64(ValueStatistics<i64>),
    Int96(ValueStatistics<Int96>),
    Float(ValueStatistics<f32>),
    Double(ValueStatistics<f64>),
    ByteArray(ValueStatistics<ByteArray>),
    FixedLenByteArray(ValueStatistics<FixedLenByteArray>),
}
```

Expand description

Strongly typed statistics for a column chunk within a row group.

This structure is a natively typed, in memory representation of the thrift `Statistics` structure in a Parquet file footer. The statistics stored in this structure can be used by query engines to skip decoding pages while reading parquet data.

Page level statistics are stored separately, in [ColumnIndexMetaData](../page_index/column_index/enum.ColumnIndexMetaData.html "enum parquet::file::page_index::column_index::ColumnIndexMetaData").

[§](#variant.Boolean)

Statistics for Boolean column

[§](#variant.Int32)

Statistics for Int32 column

[§](#variant.Int64)

Statistics for Int64 column

[§](#variant.Int96)

Statistics for Int96 column

[§](#variant.Float)

Statistics for Float column

[§](#variant.Double)

Statistics for Double column

[§](#variant.ByteArray)

Statistics for ByteArray column

[§](#variant.FixedLenByteArray)

Statistics for FixedLenByteArray column

[Source](about:blank/src/parquet/file/statistics.rs.html#355-466)
[§](#impl-Statistics)

[Source](about:blank/src/parquet/file/statistics.rs.html#357-371)

Creates new statistics for a column type

[Source](about:blank/src/parquet/file/statistics.rs.html#373)

Creates new statistics for `Boolean` column type.

[Source](about:blank/src/parquet/file/statistics.rs.html#375)

Creates new statistics for `Int32` column type.

[Source](about:blank/src/parquet/file/statistics.rs.html#377)

Creates new statistics for `Int64` column type.

[Source](about:blank/src/parquet/file/statistics.rs.html#379)

Creates new statistics for `Int96` column type.

[Source](about:blank/src/parquet/file/statistics.rs.html#381)

Creates new statistics for `Float` column type.

[Source](about:blank/src/parquet/file/statistics.rs.html#383)

Creates new statistics for `Double` column type.

[Source](about:blank/src/parquet/file/statistics.rs.html#385)

Creates new statistics for `ByteArray` column type.

[Source](about:blank/src/parquet/file/statistics.rs.html#387-391)

Creates new statistics for `FixedLenByteArray` column type.

[Source](about:blank/src/parquet/file/statistics.rs.html#399-401)

Returns `true` if statistics have old `min` and `max` fields set. This means that the column order is likely to be undefined, which, for old files could mean a signed sort order of values.

Refer to [`ColumnOrder`](../../basic/enum.ColumnOrder.html "enum parquet::basic::ColumnOrder") and [`SortOrder`](../../basic/enum.SortOrder.html "enum parquet::basic::SortOrder") for more information.

[Source](about:blank/src/parquet/file/statistics.rs.html#413-415)

Old versions of parquet stored statistics in `min` and `max` fields, ordered using signed comparison. This resulted in an undefined ordering for unsigned quantities, such as booleans and unsigned integers.

These fields were therefore deprecated in favour of `min_value` and `max_value`, which have a type-defined sort order.

However, not all readers have been updated. For backwards compatibility, this method returns `true` if the statistics within this have a signed sort order, that is compatible with being stored in the deprecated `min` and `max` fields

[Source](about:blank/src/parquet/file/statistics.rs.html#419-421)

Returns optional value of number of distinct values occurring. When it is `None`, the value should be ignored.

[Source](about:blank/src/parquet/file/statistics.rs.html#429-431)

Returns number of null values for the column, if known. Note that this includes all nulls when column is part of the complex type.

Note this API returns Some(0) even if the null count was not present in the statistics. See [https://github.com/apache/arrow-rs/pull/6216/files](https://github.com/apache/arrow-rs/pull/6216/files)

[Source](about:blank/src/parquet/file/statistics.rs.html#434-436)

Returns `true` if the min value is set, and is an exact min value.

[Source](about:blank/src/parquet/file/statistics.rs.html#439-441)

Returns `true` if the max value is set, and is an exact max value.

[Source](about:blank/src/parquet/file/statistics.rs.html#444-446)

Returns slice of bytes that represent min value, if min value is known.

[Source](about:blank/src/parquet/file/statistics.rs.html#449-451)

Returns slice of bytes that represent max value, if max value is known.

[Source](about:blank/src/parquet/file/statistics.rs.html#454-465)

Returns physical type associated with statistics.

[Source](about:blank/src/parquet/file/statistics.rs.html#329)
[§](#impl-Clone-for-Statistics)

[Source](about:blank/src/parquet/file/statistics.rs.html#329)
[§](#impl-Debug-for-Statistics)

[Source](about:blank/src/parquet/file/statistics.rs.html#468-481)
[§](#impl-Display-for-Statistics)

[Source](about:blank/src/parquet/file/statistics.rs.html#349-353)
[§](#impl-From%3CValueStatistics%3CT%3E%3E-for-Statistics)

[Source](about:blank/src/parquet/file/statistics.rs.html#350-352)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/parquet/file/metadata/memory.rs.html#159-172)
[§](#impl-HeapSize-for-Statistics)

[Source](about:blank/src/parquet/file/metadata/memory.rs.html#160-171)
[§](#method.heap_size)

Return the size of any bytes allocated on the heap by this object, including heap memory in those structures [Read more](about:blank/metadata/memory/trait.HeapSize.html#tymethod.heap_size)

[Source](about:blank/src/parquet/file/statistics.rs.html#329)
[§](#impl-PartialEq-for-Statistics)

[Source](about:blank/src/parquet/file/statistics.rs.html#329)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/parquet/file/statistics.rs.html#329)
[§](#impl-StructuralPartialEq-for-Statistics)

[§](#impl-Freeze-for-Statistics)

[§](#impl-RefUnwindSafe-for-Statistics)

[§](#impl-Send-for-Statistics)

[§](#impl-Sync-for-Statistics)

[§](#impl-Unpin-for-Statistics)

[§](#impl-UnwindSafe-for-Statistics)
