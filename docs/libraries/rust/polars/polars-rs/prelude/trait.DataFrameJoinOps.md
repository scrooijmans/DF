# Trait DataFrameJoinOps Copy item path

<a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/src/polars_ops/frame/join/mod.rs.html#50" class="src">Source</a>

``` rust
pub trait DataFrameJoinOps: IntoDf {
    // Provided methods
    fn join(
        &self,
        other: &DataFrame,
        left_on: impl IntoIterator<Item = impl Into<PlSmallStr>>,
        right_on: impl IntoIterator<Item = impl Into<PlSmallStr>>,
        args: JoinArgs,
        options: Option<JoinTypeOptions>,
    ) -> Result<DataFrame, PolarsError> { ... }
    fn inner_join(
        &self,
        other: &DataFrame,
        left_on: impl IntoIterator<Item = impl Into<PlSmallStr>>,
        right_on: impl IntoIterator<Item = impl Into<PlSmallStr>>,
    ) -> Result<DataFrame, PolarsError> { ... }
    fn left_join(
        &self,
        other: &DataFrame,
        left_on: impl IntoIterator<Item = impl Into<PlSmallStr>>,
        right_on: impl IntoIterator<Item = impl Into<PlSmallStr>>,
    ) -> Result<DataFrame, PolarsError> { ... }
    fn full_join(
        &self,
        other: &DataFrame,
        left_on: impl IntoIterator<Item = impl Into<PlSmallStr>>,
        right_on: impl IntoIterator<Item = impl Into<PlSmallStr>>,
    ) -> Result<DataFrame, PolarsError> { ... }
}
```

Available on **crate feature `polars-ops`** only.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.join" class="fn">join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, right_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinArgs.html" class="struct" title="struct polars::prelude::JoinArgs">JoinArgs</a>, options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptions.html" class="enum" title="enum polars::prelude::JoinTypeOptions">JoinTypeOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Generic join method. Can be used to join on multiple columns.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#example" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Fruit" => &["Apple", "Banana", "Pear"],
                         "Phosphorus (mg/100g)" => &[11, 22, 12])?;
let df2: DataFrame = df!("Name" => &["Apple", "Banana", "Pear"],
                         "Potassium (mg/100g)" => &[107, 358, 115])?;

let df3: DataFrame = df1.join(&df2, ["Fruit"], ["Name"], JoinArgs::new(JoinType::Inner),
None)?;
assert_eq!(df3.shape(), (3, 3));
println!("{}", df3);
```

Output:

``` text
shape: (3, 3)
+--------+----------------------+---------------------+
| Fruit  | Phosphorus (mg/100g) | Potassium (mg/100g) |
| ---    | ---                  | ---                 |
| str    | i32                  | i32                 |
+========+======================+=====================+
| Apple  | 11                   | 107                 |
+--------+----------------------+---------------------+
| Banana | 22                   | 358                 |
+--------+----------------------+---------------------+
| Pear   | 12                   | 115                 |
+--------+----------------------+---------------------+
```

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.inner_join" class="fn">inner_join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, right_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Perform an inner join on two DataFrames.

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#example-1" class="doc-anchor">§</a>Example

``` rust
fn join_dfs(left: &DataFrame, right: &DataFrame) -> PolarsResult<DataFrame> {
    left.inner_join(right, ["join_column_left"], ["join_column_right"])
}
```

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.left_join" class="fn">left_join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, right_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Perform a left outer join on two DataFrames

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#example-2" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Wavelength (nm)" => &[480.0, 650.0, 577.0, 1201.0, 100.0])?;
let df2: DataFrame = df!("Color" => &["Blue", "Yellow", "Red"],
                         "Wavelength nm" => &[480.0, 577.0, 650.0])?;

let df3: DataFrame = df1.left_join(&df2, ["Wavelength (nm)"], ["Wavelength nm"])?;
println!("{:?}", df3);
```

Output:

``` text
shape: (5, 2)
+-----------------+--------+
| Wavelength (nm) | Color  |
| ---             | ---    |
| f64             | str    |
+=================+========+
| 480             | Blue   |
+-----------------+--------+
| 650             | Red    |
+-----------------+--------+
| 577             | Yellow |
+-----------------+--------+
| 1201            | null   |
+-----------------+--------+
| 100             | null   |
+-----------------+--------+
```

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.full_join" class="fn">full_join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, right_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Perform a full outer join on two DataFrames

##### <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#example-3" class="doc-anchor">§</a>Example

``` rust
fn join_dfs(left: &DataFrame, right: &DataFrame) -> PolarsResult<DataFrame> {
    left.full_join(right, ["join_column_left"], ["join_column_right"])
}
```

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#impl-DataFrameJoinOps-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html" class="trait" title="trait polars::prelude::DataFrameJoinOps">DataFrameJoinOps</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>
