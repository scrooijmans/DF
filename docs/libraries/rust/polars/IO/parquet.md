# Parquet - Polars user guide
Loading or writing [`Parquet` files](https://parquet.apache.org/) is lightning fast as the layout of data in a Polars `DataFrame` in memory mirrors the layout of a Parquet file on disk in many respects.

Unlike CSV, Parquet is a columnar format. This means that the data is stored in columns rather than rows. This is a more efficient way of storing data as it allows for better compression and faster access to data.

Read
----

We can read a `Parquet` file into a `DataFrame` using the `read_parquet` function:

Python Rust

[`read_parquet`](https://docs.pola.rs/api/python/stable/reference/api/polars.read_parquet.html)

```
df = pl.read_parquet("docs/assets/data/path.parquet")

```


Write
-----

Python Rust

[`write_parquet`](https://docs.pola.rs/api/python/stable/reference/api/polars.DataFrame.write_parquet.html)

```
df = pl.DataFrame({"foo": [1, 2, 3], "bar": [None, "bak", "baz"]})
df.write_parquet("docs/assets/data/path.parquet")

```


[`ParquetWriter`](https://docs.pola.rs/api/rust/dev/polars/prelude/struct.ParquetWriter.html) · [Available on feature parquet](about:/user-guide/installation/#feature-flags "To use this functionality enable the feature flag parquet")

```
let mut df = df!(
    "foo" => &[1, 2, 3],
    "bar" => &[None, Some("bak"), Some("baz")],
)
.unwrap();

let mut file = std::fs::File::create("docs/assets/data/path.parquet").unwrap();
ParquetWriter::new(&mut file).finish(&mut df).unwrap();

```


Scan
----

Polars allows you to _scan_ a `Parquet` input. Scanning delays the actual parsing of the file and instead returns a lazy computation holder called a `LazyFrame`.

Python Rust

[`scan_parquet`](https://docs.pola.rs/api/python/stable/reference/api/polars.scan_parquet.html)

```
df = pl.scan_parquet("docs/assets/data/path.parquet")

```


If you want to know why this is desirable, you can read more about those Polars optimizations [here](../../concepts/lazy-api/).

When we scan a `Parquet` file stored in the cloud, we can also apply predicate and projection pushdowns. This can significantly reduce the amount of data that needs to be downloaded. For scanning a Parquet file in the cloud, see [Cloud storage](about:blank/cloud-storage/#scanning-from-cloud-storage-with-query-optimisation).