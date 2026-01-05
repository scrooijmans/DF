# Type Alias FileStatisticsCache Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/cache/cache_manager.rs.html#35" class="src">Source</a>

``` rust
pub type FileStatisticsCache = Arc<dyn CacheAccessor<Path, Arc<Statistics>, Extra = ObjectMeta>>;
```

Expand description

A cache for [`Statistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html "struct datafusion::common::Statistics").

If enabled via [`CacheManagerConfig::with_files_statistics_cache`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManagerConfig.html#method.with_files_statistics_cache "method datafusion::execution::cache::cache_manager::CacheManagerConfig::with_files_statistics_cache") this cache avoids inferring the same file statistics repeatedly during the session lifetime.

See [`crate::runtime_env::RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv") for more details

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/type.FileStatisticsCache.html#aliased-type" class="anchor">§</a>

``` rust
pub struct FileStatisticsCache { /* private fields */ }
```
