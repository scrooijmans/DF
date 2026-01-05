# Function signed_to_duration Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/time.rs.html#232-244" class="src">Source</a>

``` rust
pub fn signed_to_duration(value: &str) -> Result<Duration>
```

Expand description

Convert an unsigned [`Duration`](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration") into a jiff \[`SignedDuration`\]. Parse a duration encoded either as ISO-8601 (e.g. `PT5M`) or friendly (e.g. `5m`).
