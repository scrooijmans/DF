# Function parse_url Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/parse.rs.html#160-162" class="src">Source</a>

``` rust
pub fn parse_url(url: &Url) -> Result<(Box<dyn ObjectStore>, Path), Error>
```

Expand description

Create an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") based on the provided `url`

Returns

- An [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") of the corresponding type
- The [`Path`](https://docs.rs/object_store/latest/object_store/path/struct.Path.html "struct object_store::path::Path") into the [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") of the addressed resource
