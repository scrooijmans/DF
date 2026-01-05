# JSON files - Polars user guide
Polars can read and write both standard JSON and newline-delimited JSON (NDJSON).

Read
----

### JSON

Reading a JSON file should look familiar:

Python Rust

[`read_json`](https://docs.pola.rs/api/python/stable/reference/api/polars.read_json.html)

```
df = pl.read_json("docs/assets/data/path.json")

```


[`JsonReader`](https://docs.pola.rs/api/rust/dev/polars_io/json/struct.JsonReader.html) · [Available on feature json](about:/user-guide/installation/#feature-flags "To use this functionality enable the feature flag json")

```
use polars::prelude::*;

let mut file = std::fs::File::open("docs/assets/data/path.json").unwrap();
let df = JsonReader::new(&mut file).finish()?;

```


### Newline Delimited JSON

JSON objects that are delimited by newlines can be read into Polars in a much more performant way than standard json.

Polars can read an NDJSON file into a `DataFrame` using the `read_ndjson` function:

Python Rust

[`read_ndjson`](https://docs.pola.rs/api/python/stable/reference/api/polars.read_ndjson.html)

```
df = pl.read_ndjson("docs/assets/data/path.json")

```


Write
-----

Python Rust

[`write_json`](https://docs.pola.rs/api/python/stable/reference/api/polars.DataFrame.write_json.html) · [`write_ndjson`](https://docs.pola.rs/api/python/stable/reference/api/polars.DataFrame.write_ndjson.html)

```
df = pl.DataFrame({"foo": [1, 2, 3], "bar": [None, "bak", "baz"]})
df.write_json("docs/assets/data/path.json")

```


[`JsonWriter`](https://docs.pola.rs/api/rust/dev/polars_io/json/struct.JsonWriter.html) · [`JsonWriter`](https://docs.pola.rs/api/rust/dev/polars_io/json/struct.JsonWriter.html) · [Available on feature json](about:/user-guide/installation/#feature-flags "To use this functionality enable the feature flag json")

```
let mut df = df!(
    "foo" => &[1, 2, 3],
    "bar" => &[None, Some("bak"), Some("baz")],
)
.unwrap();

let mut file = std::fs::File::create("docs/assets/data/path.json").unwrap();

// json
JsonWriter::new(&mut file)
    .with_json_format(JsonFormat::Json)
    .finish(&mut df)
    .unwrap();

// ndjson
JsonWriter::new(&mut file)
    .with_json_format(JsonFormat::JsonLines)
    .finish(&mut df)
    .unwrap();

```


Scan
----

Polars allows you to _scan_ a JSON input **only for newline delimited json**. Scanning delays the actual parsing of the file and instead returns a lazy computation holder called a `LazyFrame`.

Python Rust

[`scan_ndjson`](https://docs.pola.rs/api/python/stable/reference/api/polars.scan_ndjson.html)

```
df = pl.scan_ndjson("docs/assets/data/path.json")

```
