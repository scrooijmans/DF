# Function parse_url_opts Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/parse.rs.html#187-238" class="src">Source</a>

``` rust
pub fn parse_url_opts<I, K, V>(
    url: &Url,
    options: I,
) -> Result<(Box<dyn ObjectStore>, Path), Error>where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: Into<String>,
```

Expand description

Create an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") based on the provided `url` and options

This method can be used to create an instance of one of the provided `ObjectStore` implementations based on the URL scheme (see [`ObjectStoreScheme`](https://docs.rs/object_store/latest/object_store/enum.ObjectStoreScheme.html "enum object_store::ObjectStoreScheme") for more details).

For example

- `file:///path/to/my/file` will return a [`LocalFileSystem`](https://docs.rs/object_store/latest/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem") instance
- `s3://bucket/path` will return an [`AmazonS3`](https://docs.rs/object_store/0.12.0/object_store/aws/struct.AmazonS3.html) instance if the `aws` feature is enabled.

Arguments:

- `url`: The URL to parse
- `options`: A list of key-value pairs to pass to the [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") builder. Note different object stores accept different configuration options, so the options that are read depends on the `url` value. One common pattern is to pass configuration information via process variables using [`std::env::vars`](https://doc.rust-lang.org/nightly/std/env/fn.vars.html "fn std::env::vars").

Returns

- An [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") of the corresponding type
- The [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") into the [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") of the addressed resource
