# ValueStatistics in parquet::file::statistics - Rust

## Struct ValueStatistics 

[Source](about:blank/src/parquet/file/statistics.rs.html#490-508)

```
pub struct ValueStatistics<T> {
    min: Option<T>,
    max: Option<T>,
    distinct_count: Option<u64>,
    null_count: Option<u64>,
    is_max_value_exact: bool,
    is_min_value_exact: bool,
    is_min_max_deprecated: bool,
    is_min_max_backwards_compatible: bool,
}
```

Expand description

Typed statistics for one column chunk

See [`Statistics`](enum.Statistics.html "enum parquet::file::statistics::Statistics") for more details

If `true` populate the deprecated `min` and `max` fields instead of `min_value` and `max_value`

[Source](about:blank/src/parquet/file/statistics.rs.html#510-629)
[§](#impl-ValueStatistics%3CT%3E)

[Source](about:blank/src/parquet/file/statistics.rs.html#512-529)

Creates new typed statistics.

[Source](about:blank/src/parquet/file/statistics.rs.html#535-540)

Set whether the stored `min` field represents the exact minimum, or just a bound on the minimum value.

see [`Self::min_is_exact`](about:blank/struct.ValueStatistics.html#method.min_is_exact "method parquet::file::statistics::ValueStatistics::min_is_exact")

[Source](about:blank/src/parquet/file/statistics.rs.html#546-551)

Set whether the stored `max` field represents the exact maximum, or just a bound on the maximum value.

see [`Self::max_is_exact`](about:blank/struct.ValueStatistics.html#method.max_is_exact "method parquet::file::statistics::ValueStatistics::max_is_exact")

[Source](about:blank/src/parquet/file/statistics.rs.html#558-563)

Set whether to write the deprecated `min` and `max` fields for compatibility with older parquet writers

This should only be enabled if the field is signed, see [`Self::is_min_max_backwards_compatible`](about:blank/struct.ValueStatistics.html#method.is_min_max_backwards_compatible "method parquet::file::statistics::ValueStatistics::is_min_max_backwards_compatible")

[Source](about:blank/src/parquet/file/statistics.rs.html#566-568)

Returns min value of the statistics, if known.

[Source](about:blank/src/parquet/file/statistics.rs.html#571-573)

Returns max value of the statistics, if known.

[Source](about:blank/src/parquet/file/statistics.rs.html#576-578)

Returns min value as bytes of the statistics, if min value is known.

[Source](about:blank/src/parquet/file/statistics.rs.html#581-583)

Returns max value as bytes of the statistics, if max value is known.

[Source](about:blank/src/parquet/file/statistics.rs.html#587-589)

Whether or not min and max values are set. Normally both min/max values will be set to `Some(value)` or `None`.

[Source](about:blank/src/parquet/file/statistics.rs.html#592-594)

Whether or not max value is set, and is an exact value.

[Source](about:blank/src/parquet/file/statistics.rs.html#597-599)

Whether or not min value is set, and is an exact value.

[Source](about:blank/src/parquet/file/statistics.rs.html#602-604)

Returns optional value of number of distinct values occurring.

[Source](about:blank/src/parquet/file/statistics.rs.html#607-609)

Returns null count.

[Source](about:blank/src/parquet/file/statistics.rs.html#612-614)

Returns `true` if statistics were created using old min/max fields.

[Source](about:blank/src/parquet/file/statistics.rs.html#626-628)

Old versions of parquet stored statistics in `min` and `max` fields, ordered using signed comparison. This resulted in an undefined ordering for unsigned quantities, such as booleans and unsigned integers.

These fields were therefore deprecated in favour of `min_value` and `max_value`, which have a type-defined sort order.

However, not all readers have been updated. For backwards compatibility, this method returns `true` if the statistics within this have a signed sort order, that is compatible with being stored in the deprecated `min` and `max` fields

[Source](about:blank/src/parquet/file/statistics.rs.html#489)
[§](#impl-Clone-for-ValueStatistics%3CT%3E)

[Source](about:blank/src/parquet/file/statistics.rs.html#661-677)
[§](#impl-Debug-for-ValueStatistics%3CT%3E)

[Source](about:blank/src/parquet/file/statistics.rs.html#631-659)
[§](#impl-Display-for-ValueStatistics%3CT%3E)

[Source](about:blank/src/parquet/file/statistics.rs.html#349-353)
[§](#impl-From%3CValueStatistics%3CT%3E%3E-for-Statistics)

[Source](about:blank/src/parquet/file/statistics.rs.html#350-352)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/parquet/file/metadata/memory.rs.html#222-227)
[§](#impl-HeapSize-for-ValueStatistics%3CT%3E)

[Source](about:blank/src/parquet/file/metadata/memory.rs.html#223-226)
[§](#method.heap_size)

Return the size of any bytes allocated on the heap by this object, including heap memory in those structures [Read more](about:blank/metadata/memory/trait.HeapSize.html#tymethod.heap_size)

[Source](about:blank/src/parquet/file/statistics.rs.html#489)
[§](#impl-PartialEq-for-ValueStatistics%3CT%3E)

[Source](about:blank/src/parquet/file/statistics.rs.html#489)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](about:blank/src/parquet/file/statistics.rs.html#489)
[§](#impl-Eq-for-ValueStatistics%3CT%3E)

[Source](about:blank/src/parquet/file/statistics.rs.html#489)
[§](#impl-StructuralPartialEq-for-ValueStatistics%3CT%3E)
