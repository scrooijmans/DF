# SessionConfig in datafusion::execution::context - Rust

## Struct SessionConfig 

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#93)

```
pub struct SessionConfig { /* private fields */ }
```

Expand description

Configuration options for [`SessionContext`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html).

Can be passed to [`SessionContext::new_with_config`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.new_with_config) to customize the configuration of DataFusion.

Options can be set using namespaces keys with `.` as the separator, where the namespace determines which configuration struct the value to routed to. All built-in options are under the `datafusion` namespace.

For example, the key `datafusion.execution.batch_size` will set [ExecutionOptions::batch_size](about:blank/config/struct.ExecutionOptions.html#structfield.batch_size "field datafusion::config::ExecutionOptions::batch_size"), because [ConfigOptions::execution](about:blank/config/struct.ConfigOptions.html#structfield.execution "field datafusion::config::ConfigOptions::execution") is [ExecutionOptions](../../config/struct.ExecutionOptions.html "struct datafusion::config::ExecutionOptions"). Similarly, the key `datafusion.execution.parquet.pushdown_filters` will set [ParquetOptions::pushdown_filters](about:blank/config/struct.ParquetOptions.html#structfield.pushdown_filters "field datafusion::config::ParquetOptions::pushdown_filters"), since [ExecutionOptions::parquet](about:blank/config/struct.ExecutionOptions.html#structfield.parquet "field datafusion::config::ExecutionOptions::parquet") is [ParquetOptions](../../config/struct.ParquetOptions.html "struct datafusion::config::ParquetOptions").

Some options have convenience methods. For example [SessionConfig::with_batch_size](about:blank/prelude/struct.SessionConfig.html#method.with_batch_size "method datafusion::prelude::SessionConfig::with_batch_size") is shorthand for setting `datafusion.execution.batch_size`.

```
use datafusion_execution::config::SessionConfig;
use datafusion_common::ScalarValue;

let config = SessionConfig::new()
   .set("datafusion.execution.batch_size", &ScalarValue::UInt64(Some(1234)))
   .set_bool("datafusion.execution.parquet.pushdown_filters", true);

assert_eq!(config.batch_size(), 1234);
assert_eq!(config.options().execution.batch_size, 1234);
assert_eq!(config.options().execution.parquet.pushdown_filters, true);
```

You can also directly mutate the options via [SessionConfig::options_mut](about:blank/prelude/struct.SessionConfig.html#method.options_mut "method datafusion::prelude::SessionConfig::options_mut"). So the following is equivalent to the above:

```
let mut config = SessionConfig::new();
config.options_mut().execution.batch_size = 1234;
config.options_mut().execution.parquet.pushdown_filters = true;
```

### [§](#built-in-options)Built-in options

### [§](#custom-configuration)Custom configuration

Configuration options can be extended. See [SessionConfig::with_extension](about:blank/prelude/struct.SessionConfig.html#method.with_extension "method datafusion::prelude::SessionConfig::with_extension") for details.

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#116)
[§](#impl-SessionConfig)

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#118)

Create an execution config with default setting

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#126)

Create an execution config with config options read from the environment

See [`ConfigOptions::from_env`](about:blank/config/struct.ConfigOptions.html#method.from_env "associated function datafusion::config::ConfigOptions::from_env") for details on how environment variables are mapped to config options.

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#131)

Create new ConfigOptions struct, taking values from a string hash map.

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#145)

Return a handle to the configuration options.

Can be used to read the current configuration.

```
use datafusion_execution::config::SessionConfig;

let config = SessionConfig::new();
assert!(config.options().execution.batch_size > 0);
```

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#160)

Return a mutable handle to the configuration options.

Can be used to set configuration options.

```
use datafusion_execution::config::SessionConfig;

let mut config = SessionConfig::new();
config.options_mut().execution.batch_size = 1024;
assert_eq!(config.options().execution.batch_size, 1024);
```

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#165)

Set a configuration option

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#170)

Set a boolean configuration option

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#175)

Set a generic `u64` configuration option

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#180)

Set a generic `usize` configuration option

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#185)

Set a generic `str` configuration option

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#191)

Customize batch size

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#201)

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#211)

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#219)

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#224)

Is the information schema enabled?

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#229)

Should the context create the default catalog and schema?

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#234)

Are joins repartitioned during execution?

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#239)

Are aggregates repartitioned during execution?

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#244)

Are window functions repartitioned during execution?

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#250)

Do we execute sorts in a per-partition fashion and merge afterwards, or do we coalesce partitions first and sort globally?

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#258)

Prefer existing sort (true) or maximize parallelism (false). See [prefer_existing_sort](about:blank/config/struct.OptimizerOptions.html#structfield.prefer_existing_sort "field datafusion::config::OptimizerOptions::prefer_existing_sort") for more details

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#263)

Are statistics collected during execution?

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#268)

Compression codec for spill file

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#273-277)

Selects a name for the default catalog and schema

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#284)

Controls whether the default catalog and schema will be automatically created

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#290)

Enables or disables the inclusion of `information_schema` virtual tables

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#296)

Enables or disables the use of repartitioning for joins to improve parallelism

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#302)

Enables or disables the use of repartitioning for aggregations to improve parallelism

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#308)

Sets minimum file range size for repartitioning scans

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#314)

Enables or disables the allowing unordered symmetric hash join

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#322)

Enables or disables the use of repartitioning for file scans

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#328)

Enables or disables the use of repartitioning for window functions to improve parallelism

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#334)

Enables or disables the use of per-partition sorting to improve parallelism

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#343)

Prefer existing sort (true) or maximize parallelism (false). See [prefer_existing_sort](about:blank/config/struct.OptimizerOptions.html#structfield.prefer_existing_sort "field datafusion::config::OptimizerOptions::prefer_existing_sort") for more details

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#351)

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#357)

Enables or disables the use of pruning predicate for parquet readers to skip row groups

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#363)

Returns true if pruning predicate should be used to skip parquet row groups

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#368)

Returns true if bloom filter should be used to skip parquet row groups

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#373)

Enables or disables the use of bloom filter for parquet readers to skip row groups

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#379)

Returns true if page index should be used to skip parquet data pages

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#384)

Enables or disables the use of page index for parquet readers to skip parquet data pages

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#390)

Enables or disables the collection of statistics after listing files

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#396)

Get the currently configured batch size

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#401)

Enables or disables the coalescence of small batches into larger batches

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#408)

Returns true if record batches will be examined between each operator and small batches will be coalesced into larger batches.

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#413)

Enables or disables the round robin repartition for increasing parallelism

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#420)

Returns true if the physical plan optimizer will try to add round robin repartition to increase parallelism to leverage more CPU cores.

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#428-431)

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#440)

Set the compression codec [`spill_compression`](about:blank/config/struct.ExecutionOptions.html#structfield.spill_compression "field datafusion::config::ExecutionOptions::spill_compression") used when spilling data to disk.

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#449-452)

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#459-462)

Enables or disables the enforcement of batch size in joins

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#469)

Returns true if the joins will be enforced to output batches of the configured size

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#480)

Convert configuration options to name-value pairs with values converted to strings.

Note that this method will eventually be deprecated and replaced by [`options`](about:blank/prelude/struct.SessionConfig.html#method.options "method datafusion::prelude::SessionConfig::options").

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#534-536)

Add extensions.

Extensions can be used to attach extra data to the session config – e.g. tracing information or caches. Extensions are opaque and the types are unknown to DataFusion itself, which makes them extremely flexible. [1](#fn1)

Extensions are stored within an [`Arc`](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc") so they do NOT require [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"). The are immutable. If you need to modify their state over their lifetime – e.g. for caches – you need to establish some for of interior mutability.

Extensions are indexed by their type `T`. If multiple values of the same type are provided, only the last one will be kept.

You may use [`get_extension`](about:blank/prelude/struct.SessionConfig.html#method.get_extension "method datafusion::prelude::SessionConfig::get_extension") to retrieve extensions.

##### [§](#example)Example

```
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

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#576-578)

Set extension. Pretty much the same as [`with_extension`](about:blank/prelude/struct.SessionConfig.html#method.with_extension "method datafusion::prelude::SessionConfig::with_extension"), but take mutable reference instead of owning it. Useful if you want to add another extension after the [`SessionConfig`](../../prelude/struct.SessionConfig.html "struct datafusion::prelude::SessionConfig") is created.

##### [§](#example-1)Example

```
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

[Source](https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/config.rs.html#588-590)

Get extension, if any for the specified type `T` exists.

See [`with_extension`](about:blank/prelude/struct.SessionConfig.html#method.with_extension "method datafusion::prelude::SessionConfig::with_extension") on how to add attach extensions.

[§](#impl-Freeze-for-SessionConfig)

[§](#impl-RefUnwindSafe-for-SessionConfig)

[§](#impl-Send-for-SessionConfig)

[§](#impl-Sync-for-SessionConfig)

[§](#impl-Unpin-for-SessionConfig)

[§](#impl-UnwindSafe-for-SessionConfig)
