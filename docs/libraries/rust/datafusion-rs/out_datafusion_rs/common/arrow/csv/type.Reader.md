# Type Alias Reader Copy item path

<a href="https://docs.rs/arrow-csv/56.0.0/x86_64-unknown-linux-gnu/src/arrow_csv/reader/mod.rs.html#444" class="src">Source</a>

``` rust
pub type Reader<R> = BufReader<BufReader<R>>;
```

Expand description

CSV file reader using [`std::io::BufReader`](https://doc.rust-lang.org/nightly/std/io/buffered/bufreader/struct.BufReader.html "struct std::io::buffered::bufreader::BufReader")

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/type.Reader.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Reader<R> { /* private fields */ }
```
