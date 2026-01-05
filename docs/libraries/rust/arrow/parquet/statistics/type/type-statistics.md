# TypedStatistics in parquet::file::statistics - Rust

[parquet](../../index.html)::[file](../index.html)::[statistics](index.html)

## Type Alias TypedStatistics 

[Source](about:blank/src/parquet/file/statistics.rs.html#484)

```
pub type TypedStatistics<T> = ValueStatistics<<T as DataType>::T>;
```

Expand description

Typed implementation for [`Statistics`](enum.Statistics.html "enum parquet::file::statistics::Statistics").

## Aliased Type[§](#aliased-type)

```
pub struct TypedStatistics<T> {
    min: Option<<T as DataType>::T>,
    max: Option<<T as DataType>::T>,
    distinct_count: Option<u64>,
    null_count: Option<u64>,
    is_max_value_exact: bool,
    is_min_value_exact: bool,
    is_min_max_deprecated: bool,
    is_min_max_backwards_compatible: bool,
}
```

## Fields[§](#fields)

[§](#structfield.min)`min: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [DataType](../../data_type/trait.DataType.html "trait parquet::data_type::DataType")>::[T](about:blank/data_type/trait.DataType.html#associatedtype.T "type parquet::data_type::DataType::T")>`[§](#structfield.max)`max: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [DataType](../../data_type/trait.DataType.html "trait parquet::data_type::DataType")>::[T](about:blank/data_type/trait.DataType.html#associatedtype.T "type parquet::data_type::DataType::T")>`[§](#structfield.distinct_count)`distinct_count: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)>`[§](#structfield.null_count)`null_count: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)>`[§](#structfield.is_max_value_exact)`is_max_value_exact: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)`[§](#structfield.is_min_value_exact)`is_min_value_exact: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)`[§](#structfield.is_min_max_deprecated)`is_min_max_deprecated: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)`

If `true` populate the deprecated `min` and `max` fields instead of `min_value` and `max_value`

[§](#structfield.is_min_max_backwards_compatible)`is_min_max_backwards_compatible: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)`

If `true` the statistics are compatible with the deprecated `min` and `max` fields. See [`ValueStatistics::is_min_max_backwards_compatible`](about:blank/struct.ValueStatistics.html#method.is_min_max_backwards_compatible "method parquet::file::statistics::ValueStatistics::is_min_max_backwards_compatible")
