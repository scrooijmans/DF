# Type Alias FileScanTaskStream Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/scan/task.rs.html#26" class="src">Source</a>

``` rust
pub type FileScanTaskStream = BoxStream<'static, Result<FileScanTask>>;
```

Expand description

A stream of [`FileScanTask`](https://docs.rs/iceberg/0.7.0/iceberg/scan/struct.FileScanTask.html "struct iceberg::scan::FileScanTask").

## Aliased Type<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.FileScanTaskStream.html#aliased-type" class="anchor">§</a>

``` rust
pub struct FileScanTaskStream { /* private fields */ }
```
