# Function default_config_transform Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1320-1323" class="src">Source</a>

``` rust
pub fn default_config_transform<T>(input: &str) -> Result<T, DataFusionError>where
    T: FromStr,
    <T as FromStr>::Err: Sync + Send + Error + 'static,
```

Expand description

Default transformation to parse a [`ConfigField`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html "trait datafusion::config::ConfigField") for a string.

This uses [`FromStr`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") to parse the data.
