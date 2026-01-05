# Struct SessionConfig Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#93" class="src">Source</a>

``` rust
pub struct SessionConfig { /* private fields */ }
```

Expand description

Configuration options for [`SessionContext`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html).

Can be passed to [`SessionContext::new_with_config`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.new_with_config) to customize the configuration of DataFusion.

Options can be set using namespaces keys with `.` as the separator, where the namespace determines which configuration struct the value to routed to. All built-in options are under the `datafusion` namespace.

For example, the key `datafusion.execution.batch_size` will set [ExecutionOptions::batch_size](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.batch_size "field datafusion::config::ExecutionOptions::batch_size"), because [ConfigOptions::execution](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#structfield.execution "field datafusion::config::ConfigOptions::execution") is [ExecutionOptions](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html "struct datafusion::config::ExecutionOptions"). Similarly, the key `datafusion.execution.parquet.pushdown_filters` will set [ParquetOptions::pushdown_filters](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html#structfield.pushdown_filters "field datafusion::config::ParquetOptions::pushdown_filters"), since [ExecutionOptions::parquet](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.parquet "field datafusion::config::ExecutionOptions::parquet") is [ParquetOptions](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html "struct datafusion::config::ParquetOptions").

Some options have convenience methods. For example [SessionConfig::with_batch_size](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_batch_size "method datafusion::prelude::SessionConfig::with_batch_size") is shorthand for setting `datafusion.execution.batch_size`.

``` rust
use datafusion_execution::config::SessionConfig;
use datafusion_common::ScalarValue;

let config = SessionConfig::new()
   .set("datafusion.execution.batch_size", &ScalarValue::UInt64(Some(1234)))
   .set_bool("datafusion.execution.parquet.pushdown_filters", true);

assert_eq!(config.batch_size(), 1234);
assert_eq!(config.options().execution.batch_size, 1234);
assert_eq!(config.options().execution.parquet.pushdown_filters, true);
```

You can also directly mutate the options via [SessionConfig::options_mut](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.options_mut "method datafusion::prelude::SessionConfig::options_mut"). So the following is equivalent to the above:

``` rust
let mut config = SessionConfig::new();
config.options_mut().execution.batch_size = 1234;
config.options_mut().execution.parquet.pushdown_filters = true;
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#built-in-options" class="doc-anchor">§</a>Built-in options

| Namespace | Config struct |
|----|----|
| `datafusion.catalog` | [CatalogOptions](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html "struct datafusion::config::CatalogOptions") |
| `datafusion.execution` | [ExecutionOptions](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html "struct datafusion::config::ExecutionOptions") |
| `datafusion.execution.parquet` | [ParquetOptions](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html "struct datafusion::config::ParquetOptions") |
| `datafusion.optimizer` | [OptimizerOptions](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.OptimizerOptions.html "struct datafusion::config::OptimizerOptions") |
| `datafusion.sql_parser` | [SqlParserOptions](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html "struct datafusion::config::SqlParserOptions") |
| `datafusion.explain` | [ExplainOptions](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExplainOptions.html "struct datafusion::config::ExplainOptions") |

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#custom-configuration" class="doc-anchor">§</a>Custom configuration

Configuration options can be extended. See [SessionConfig::with_extension](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_extension "method datafusion::prelude::SessionConfig::with_extension") for details.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#impl-SessionConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Create an execution config with default setting

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.from_env" class="fn">from_env</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create an execution config with config options read from the environment

See [`ConfigOptions::from_env`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html#method.from_env "associated function datafusion::config::ConfigOptions::from_env") for details on how environment variables are mapped to config options.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.from_string_hash_map" class="fn">from_string_hash_map</a>( settings: &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create new ConfigOptions struct, taking values from a string hash map.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.options" class="fn">options</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\>

Return a handle to the configuration options.

Can be used to read the current configuration.

``` rust
use datafusion_execution::config::SessionConfig;

let config = SessionConfig::new();
assert!(config.options().execution.batch_size > 0);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.options_mut" class="fn">options_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>

Return a mutable handle to the configuration options.

Can be used to set configuration options.

``` rust
use datafusion_execution::config::SessionConfig;

let mut config = SessionConfig::new();
config.options_mut().execution.batch_size = 1024;
assert_eq!(config.options().execution.batch_size, 1024);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.set" class="fn">set</a>(self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Set a configuration option

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.set_bool" class="fn">set_bool</a>(self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Set a boolean configuration option

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.set_u64" class="fn">set_u64</a>(self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Set a generic `u64` configuration option

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.set_usize" class="fn">set_usize</a>(self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Set a generic `usize` configuration option

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.set_str" class="fn">set_str</a>(self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Set a generic `str` configuration option

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_batch_size" class="fn">with_batch_size</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Customize batch size

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_target_partitions" class="fn">with_target_partitions</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Customize [`target_partitions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.target_partitions "field datafusion::config::ExecutionOptions::target_partitions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_option_extension" class="fn">with_option_extension</a>\<T\>(self, extension: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html" class="trait" title="trait datafusion::config::ConfigExtension">ConfigExtension</a>,

Insert new [ConfigExtension](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.target_partitions" class="fn">target_partitions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get [`target_partitions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.target_partitions "field datafusion::config::ExecutionOptions::target_partitions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.information_schema" class="fn">information_schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Is the information schema enabled?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.create_default_catalog_and_schema" class="fn">create_default_catalog_and_schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Should the context create the default catalog and schema?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.repartition_joins" class="fn">repartition_joins</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Are joins repartitioned during execution?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.repartition_aggregations" class="fn">repartition_aggregations</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Are aggregates repartitioned during execution?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.repartition_window_functions" class="fn">repartition_window_functions</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Are window functions repartitioned during execution?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.repartition_sorts" class="fn">repartition_sorts</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Do we execute sorts in a per-partition fashion and merge afterwards, or do we coalesce partitions first and sort globally?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.prefer_existing_sort" class="fn">prefer_existing_sort</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Prefer existing sort (true) or maximize parallelism (false). See [prefer_existing_sort](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.OptimizerOptions.html#structfield.prefer_existing_sort "field datafusion::config::OptimizerOptions::prefer_existing_sort") for more details

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.collect_statistics" class="fn">collect_statistics</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Are statistics collected during execution?

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.spill_compression" class="fn">spill_compression</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.SpillCompression.html" class="enum" title="enum datafusion::config::SpillCompression">SpillCompression</a>

Compression codec for spill file

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_default_catalog_and_schema" class="fn">with_default_catalog_and_schema</a>( self, catalog: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, schema: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Selects a name for the default catalog and schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_create_default_catalog_and_schema" class="fn">with_create_default_catalog_and_schema</a>( self, create: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Controls whether the default catalog and schema will be automatically created

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_information_schema" class="fn">with_information_schema</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the inclusion of `information_schema` virtual tables

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_repartition_joins" class="fn">with_repartition_joins</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the use of repartitioning for joins to improve parallelism

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_repartition_aggregations" class="fn">with_repartition_aggregations</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the use of repartitioning for aggregations to improve parallelism

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_repartition_file_min_size" class="fn">with_repartition_file_min_size</a>(self, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Sets minimum file range size for repartitioning scans

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_allow_symmetric_joins_without_pruning" class="fn">with_allow_symmetric_joins_without_pruning</a>( self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the allowing unordered symmetric hash join

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_repartition_file_scans" class="fn">with_repartition_file_scans</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the use of repartitioning for file scans

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_repartition_windows" class="fn">with_repartition_windows</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the use of repartitioning for window functions to improve parallelism

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_repartition_sorts" class="fn">with_repartition_sorts</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the use of per-partition sorting to improve parallelism

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_prefer_existing_sort" class="fn">with_prefer_existing_sort</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Prefer existing sort (true) or maximize parallelism (false). See [prefer_existing_sort](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.OptimizerOptions.html#structfield.prefer_existing_sort "field datafusion::config::OptimizerOptions::prefer_existing_sort") for more details

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_prefer_existing_union" class="fn">with_prefer_existing_union</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Prefer existing union (true). See [prefer_existing_union](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.OptimizerOptions.html#structfield.prefer_existing_union "field datafusion::config::OptimizerOptions::prefer_existing_union") for more details

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_parquet_pruning" class="fn">with_parquet_pruning</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the use of pruning predicate for parquet readers to skip row groups

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.parquet_pruning" class="fn">parquet_pruning</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if pruning predicate should be used to skip parquet row groups

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.parquet_bloom_filter_pruning" class="fn">parquet_bloom_filter_pruning</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if bloom filter should be used to skip parquet row groups

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_parquet_bloom_filter_pruning" class="fn">with_parquet_bloom_filter_pruning</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the use of bloom filter for parquet readers to skip row groups

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.parquet_page_index_pruning" class="fn">parquet_page_index_pruning</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if page index should be used to skip parquet data pages

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_parquet_page_index_pruning" class="fn">with_parquet_page_index_pruning</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the use of page index for parquet readers to skip parquet data pages

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_collect_statistics" class="fn">with_collect_statistics</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the collection of statistics after listing files

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.batch_size" class="fn">batch_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the currently configured batch size

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_coalesce_batches" class="fn">with_coalesce_batches</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the coalescence of small batches into larger batches

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.coalesce_batches" class="fn">coalesce_batches</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if record batches will be examined between each operator and small batches will be coalesced into larger batches.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_round_robin_repartition" class="fn">with_round_robin_repartition</a>(self, enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the round robin repartition for increasing parallelism

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.round_robin_repartition" class="fn">round_robin_repartition</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the physical plan optimizer will try to add round robin repartition to increase parallelism to leverage more CPU cores.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_sort_spill_reservation_bytes" class="fn">with_sort_spill_reservation_bytes</a>( self, sort_spill_reservation_bytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Set the size of [`sort_spill_reservation_bytes`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.sort_spill_reservation_bytes "field datafusion::config::ExecutionOptions::sort_spill_reservation_bytes") to control memory pre-reservation

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_spill_compression" class="fn">with_spill_compression</a>( self, spill_compression: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.SpillCompression.html" class="enum" title="enum datafusion::config::SpillCompression">SpillCompression</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Set the compression codec [`spill_compression`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.spill_compression "field datafusion::config::ExecutionOptions::spill_compression") used when spilling data to disk.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_sort_in_place_threshold_bytes" class="fn">with_sort_in_place_threshold_bytes</a>( self, sort_in_place_threshold_bytes: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Set the size of [`sort_in_place_threshold_bytes`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html#structfield.sort_in_place_threshold_bytes "field datafusion::config::ExecutionOptions::sort_in_place_threshold_bytes") to control how sort does things.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_enforce_batch_size_in_joins" class="fn">with_enforce_batch_size_in_joins</a>( self, enforce_batch_size_in_joins: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Enables or disables the enforcement of batch size in joins

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.enforce_batch_size_in_joins" class="fn">enforce_batch_size_in_joins</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the joins will be enforced to output batches of the configured size

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.to_props" class="fn">to_props</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Convert configuration options to name-value pairs with values converted to strings.

Note that this method will eventually be deprecated and replaced by [`options`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.options "method datafusion::prelude::SessionConfig::options").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_extension" class="fn">with_extension</a>\<T\>(self, ext: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Add extensions.

Extensions can be used to attach extra data to the session config – e.g. tracing information or caches. Extensions are opaque and the types are unknown to DataFusion itself, which makes them extremely flexible. <sup>[1](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#fn1)</sup>

Extensions are stored within an [`Arc`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc") so they do NOT require [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"). The are immutable. If you need to modify their state over their lifetime – e.g. for caches – you need to establish some for of interior mutability.

Extensions are indexed by their type `T`. If multiple values of the same type are provided, only the last one will be kept.

You may use [`get_extension`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.get_extension "method datafusion::prelude::SessionConfig::get_extension") to retrieve extensions.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#example" class="doc-anchor">§</a>Example

``` rust
use std::sync::Arc;
use datafusion_execution::config::SessionConfig;

// application-specific extension types
struct Ext1(u8);
struct Ext2(u8);
struct Ext3(u8);

let ext1a = Arc::new(Ext1(10));
let ext1b = Arc::new(Ext1(11));
let ext2 = Arc::new(Ext2(2));

let cfg = SessionConfig::default()
    // will only remember the last Ext1
    .with_extension(Arc::clone(&ext1a))
    .with_extension(Arc::clone(&ext1b))
    .with_extension(Arc::clone(&ext2));

let ext1_received = cfg.get_extension::<Ext1>().unwrap();
assert!(!Arc::ptr_eq(&ext1_received, &ext1a));
assert!(Arc::ptr_eq(&ext1_received, &ext1b));

let ext2_received = cfg.get_extension::<Ext2>().unwrap();
assert!(Arc::ptr_eq(&ext2_received, &ext2));

assert!(cfg.get_extension::<Ext3>().is_none());
```

------------------------------------------------------------------------

1.  Compare that to [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions") which only supports [`ScalarValue`](https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html "enum datafusion::scalar::ScalarValue") payloads. [↩](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#fnref1)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.set_extension" class="fn">set_extension</a>\<T\>(&mut self, ext: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>)

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Set extension. Pretty much the same as [`with_extension`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_extension "method datafusion::prelude::SessionConfig::with_extension"), but take mutable reference instead of owning it. Useful if you want to add another extension after the [`SessionConfig`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig") is created.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#example-1" class="doc-anchor">§</a>Example

``` rust
use std::sync::Arc;
use datafusion_execution::config::SessionConfig;

// application-specific extension types
struct Ext1(u8);
struct Ext2(u8);
struct Ext3(u8);

let ext1a = Arc::new(Ext1(10));
let ext1b = Arc::new(Ext1(11));
let ext2 = Arc::new(Ext2(2));

let mut cfg = SessionConfig::default();

// will only remember the last Ext1
cfg.set_extension(Arc::clone(&ext1a));
cfg.set_extension(Arc::clone(&ext1b));
cfg.set_extension(Arc::clone(&ext2));

let ext1_received = cfg.get_extension::<Ext1>().unwrap();
assert!(!Arc::ptr_eq(&ext1_received, &ext1a));
assert!(Arc::ptr_eq(&ext1_received, &ext1b));

let ext2_received = cfg.get_extension::<Ext2>().unwrap();
assert!(Arc::ptr_eq(&ext2_received, &ext2));

assert!(cfg.get_extension::<Ext3>().is_none());
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.get_extension" class="fn">get_extension</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<T\>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Get extension, if any for the specified type `T` exists.

See [`with_extension`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.with_extension "method datafusion::prelude::SessionConfig::with_extension") on how to add attach extensions.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#impl-Clone-for-SessionConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#impl-Debug-for-SessionConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#impl-Default-for-SessionConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#impl-From%3CConfigOptions%3E-for-SessionConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html#blanket-implementations" class="anchor">§</a>
