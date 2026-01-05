# Function regexp_like Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/regex/regexplike.rs.html#203" class="src">Source</a>

``` rust
pub fn regexp_like(
    args: &[Arc<dyn Array>],
) -> Result<Arc<dyn Array>, DataFusionError>
```

Expand description

Tests a string using a regular expression returning true if at least one match, false otherwise.

The full list of supported features and syntax can be found at <https://docs.rs/regex/latest/regex/#syntax>

Supported flags can be found at <https://docs.rs/regex/latest/regex/#grouping-and-flags>

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/regex/regexplike/fn.regexp_like.html#examples" class="doc-anchor">§</a>Examples

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/regex/regexplike/fn.regexp_like.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/regex.csv", CsvReadOptions::new()).await?;

// use the regexp_like function to test col 'values',
// against patterns in col 'patterns' without flags
let df = df.with_column(
    "a",
    regexp_like(vec![col("values"), col("patterns")])
)?;
// use the regexp_like function to test col 'values',
// against patterns in col 'patterns' with flags
let df = df.with_column(
    "b",
    regexp_like(vec![col("values"), col("patterns"), col("flags")])
)?;
// literals can be used as well with dataframe calls
let df = df.with_column(
    "c",
    regexp_like(vec![lit("foobarbequebaz"), lit("(bar)(beque)")])
)?;

df.show().await?;
```
