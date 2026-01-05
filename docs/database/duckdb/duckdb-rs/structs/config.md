# Config in duckdb - Rust

```
pub struct Config { /* private fields */ }
```

Expand description

[Source](about:blank/src/duckdb/config.rs.html#50-149)
[§](#impl-Config)

[Source](about:blank/src/duckdb/config.rs.html#56-61)

enable autoload extensions

[Source](about:blank/src/duckdb/config.rs.html#64-67)

Access mode of the database (\[AUTOMATIC\], READ_ONLY or READ_WRITE)

[Source](about:blank/src/duckdb/config.rs.html#70-73)

Metadata from DuckDB callers

[Source](about:blank/src/duckdb/config.rs.html#76-79)

The order type used when none is specified (\[ASC\] or DESC)

[Source](about:blank/src/duckdb/config.rs.html#82-85)

Null ordering used when none is specified (\[NULLS_FIRST\] or NULLS_LAST)

[Source](about:blank/src/duckdb/config.rs.html#88-91)

Allow the database to access external state (through e.g. COPY TO/FROM, CSV readers, pandas replacement scans, etc)

[Source](about:blank/src/duckdb/config.rs.html#94-97)

Whether or not object cache is used to cache e.g. Parquet metadata

[Source](about:blank/src/duckdb/config.rs.html#100-103)

Allow to load third-party duckdb extensions.

[Source](about:blank/src/duckdb/config.rs.html#106-109)

The maximum memory of the system (e.g. 1GB)

[Source](about:blank/src/duckdb/config.rs.html#112-115)

The number of total threads used by the system

[Source](about:blank/src/duckdb/config.rs.html#118-121)

Add any setting to the config. DuckDB will return an error if the setting is unknown or otherwise invalid.

[§](#impl-Freeze-for-Config)

[§](#impl-RefUnwindSafe-for-Config)

[§](#impl-Send-for-Config)

[§](#impl-Sync-for-Config)

[§](#impl-Unpin-for-Config)

[§](#impl-UnwindSafe-for-Config)
