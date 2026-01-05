# Function try_get_writeable Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/file.rs.html#197-200" class="src">Source</a>

``` rust
pub fn try_get_writeable(
    addr: PlPathRef<'_>,
    cloud_options: Option<&CloudOptions>,
) -> Result<Box<dyn WriteClose + Send>, PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Note: Prefer using [`Writeable`](https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html "enum polars::prelude::file::Writeable") / [`Writeable::try_new`](https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#method.try_new "associated function polars::prelude::file::Writeable::try_new") where possible.

Open a path for writing. Supports cloud paths.
