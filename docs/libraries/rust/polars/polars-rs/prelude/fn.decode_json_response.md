# Function decode_json_response Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/other.rs.html#196-198" class="src">Source</a>

``` rust
pub fn decode_json_response<T>(bytes: &[u8]) -> Result<T, PolarsError>where
    T: for<'de> Deserialize<'de>,
```

Available on **crate feature `polars-io`** only.

Expand description

Utility for decoding JSON that adds the response value to the error message if decoding fails. This makes it much easier to debug errors from parsing network responses.
