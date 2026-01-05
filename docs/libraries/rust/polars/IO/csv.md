# CSV - Polars user guide
Read & write
------------

Reading a CSV file should look familiar:

Python Rust

[`read_csv`](https://docs.pola.rs/api/python/stable/reference/api/polars.read_csv.html)

```
df = pl.read_csv("docs/assets/data/path.csv")

```


[`CsvReader`](https://docs.pola.rs/api/rust/dev/polars/prelude/struct.CsvReader.html) · [Available on feature csv](about:/user-guide/installation/#feature-flags "To use this functionality enable the feature flag csv")

```
use polars::prelude::*;

let mut df = df!(
    "foo" => &[1, 2, 3],
    "bar" => &[None, Some("bak"), Some("baz")],
)
.unwrap();

let mut file = std::fs::File::create("docs/assets/data/path.csv").unwrap();
CsvWriter::new(&mut file).finish(&mut df).unwrap();

let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("docs/assets/data/path.csv".into()))
    .unwrap()
    .finish()
    .unwrap();

```


Writing a CSV file is similar with the `write_csv` function:

Python Rust

[`write_csv`](https://docs.pola.rs/api/python/stable/reference/api/polars.DataFrame.write_csv.html)

```
df = pl.DataFrame({"foo": [1, 2, 3], "bar": [None, "bak", "baz"]})
df.write_csv("docs/assets/data/path.csv")

```


[`CsvWriter`](https://docs.pola.rs/api/rust/dev/polars/prelude/struct.CsvWriter.html) · [Available on feature csv](about:/user-guide/installation/#feature-flags "To use this functionality enable the feature flag csv")

```
let mut df = df!(
    "foo" => &[1, 2, 3],
    "bar" => &[None, Some("bak"), Some("baz")],
)
.unwrap();

let mut file = std::fs::File::create("docs/assets/data/path.csv").unwrap();
CsvWriter::new(&mut file).finish(&mut df).unwrap();

```


Scan
----

Polars allows you to _scan_ a CSV input. Scanning delays the actual parsing of the file and instead returns a lazy computation holder called a `LazyFrame`.

Python Rust

[`scan_csv`](https://docs.pola.rs/api/python/stable/reference/api/polars.scan_csv.html)

```
df = pl.scan_csv("docs/assets/data/path.csv")

```


If you want to know why this is desirable, you can read more about these Polars optimizations [here](../../concepts/lazy-api/).