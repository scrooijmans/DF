# Macro dataframe Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/dataframe/mod.rs.html#2390-2415" class="src">Source</a>

``` rust
macro_rules! dataframe {
    () => { ... };
    ($($name:expr => $data:expr),+ $(,)?) => { ... };
}
```

Expand description

Macro for creating DataFrame.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/macro.dataframe.html#example" class="doc-anchor">§</a>Example

``` rust
use datafusion::prelude::dataframe;
let df = dataframe!(
   "id" => [1, 2, 3],
   "name" => ["foo", "bar", "baz"]
 )?;
df.show().await?;
// +----+------+,
// | id | name |,
// +----+------+,
// | 1  | foo  |,
// | 2  | bar  |,
// | 3  | baz  |,
// +----+------+,
let df_empty = dataframe!()?; // empty DataFrame
assert_eq!(df_empty.schema().fields().len(), 0);
assert_eq!(df_empty.count().await?, 0);
```
