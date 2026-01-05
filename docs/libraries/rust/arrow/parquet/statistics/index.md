# parquet::file::statistics - Rust

Expand description

Contains definitions for working with Parquet statistics.

Though some common methods are available on enum, use pattern match to extract actual min and max values from statistics, see below:

## [§](#examples)Examples

```
use parquet::file::statistics::Statistics;

let stats = Statistics::int32(Some(1), Some(10), None, Some(3), true);
assert_eq!(stats.null_count_opt(), Some(3));
assert!(stats.is_min_max_deprecated());
assert!(stats.min_is_exact());
assert!(stats.max_is_exact());

match stats {
    Statistics::Int32(ref typed) => {
        assert_eq!(typed.min_opt(), Some(&1));
        assert_eq!(typed.max_opt(), Some(&10));
    }
    _ => {}
}
```

[private](private/index.html "mod parquet::file::statistics::private") 🔒

[statistics_enum_func](macro.statistics_enum_func.html "macro parquet::file::statistics::statistics_enum_func") 🔒

[statistics_new_func](macro.statistics_new_func.html "macro parquet::file::statistics::statistics_new_func") 🔒

Macro to generate methods to create Statistics.

[ValueStatistics](struct.ValueStatistics.html "struct parquet::file::statistics::ValueStatistics")

Typed statistics for one column chunk

[Statistics](enum.Statistics.html "enum parquet::file::statistics::Statistics")

Strongly typed statistics for a column chunk within a row group.

[from_thrift_page_stats](fn.from_thrift_page_stats.html "fn parquet::file::statistics::from_thrift_page_stats") 🔒

Converts Thrift definition into `Statistics`.

[page_stats_to_thrift](fn.page_stats_to_thrift.html "fn parquet::file::statistics::page_stats_to_thrift") 🔒

Convert Statistics into Thrift definition.

[TypedStatistics](type.TypedStatistics.html "type parquet::file::statistics::TypedStatistics")

Typed implementation for [`Statistics`](enum.Statistics.html "enum parquet::file::statistics::Statistics").
