# Struct GroupBy Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/mod.rs.html#189" class="src">Source</a>

``` rust
pub struct GroupBy<'a> {
    pub df: &'a DataFrame,
    /* private fields */
}
```

Expand description

Returned by a group_by operation on a DataFrame. This struct supports several aggregations.

Until described otherwise, the examples in this struct are performed on the following DataFrame:

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
use polars_core::prelude::*;

let dates = &[
"2020-08-21",
"2020-08-21",
"2020-08-22",
"2020-08-23",
"2020-08-22",
];
// date format
let fmt = "%Y-%m-%d";
// create date series
let s0 = DateChunked::parse_from_str_slice("date", dates, fmt)
        .into_series();
// create temperature series
let s1 = Series::new("temp".into(), [20, 10, 7, 9, 1]);
// create rain series
let s2 = Series::new("rain".into(), [0.2, 0.1, 0.3, 0.1, 0.01]);
// create a new DataFrame
let df = DataFrame::new(vec![s0, s1, s2]).unwrap();
println!("{:?}", df);
```

Outputs:

``` text
+------------+------+------+
| date       | temp | rain |
| ---        | ---  | ---  |
| Date       | i32  | f64  |
+============+======+======+
| 2020-08-21 | 20   | 0.2  |
+------------+------+------+
| 2020-08-21 | 10   | 0.1  |
+------------+------+------+
| 2020-08-22 | 7    | 0.3  |
+------------+------+------+
| 2020-08-23 | 9    | 0.1  |
+------------+------+------+
| 2020-08-22 | 1    | 0.01 |
+------------+------+------+
```

## Fields<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#structfield.df" class="anchor field">§</a>`df: &'a `<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame"><code>DataFrame</code></a>

## Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#impl-GroupBy%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.new" class="fn">new</a>( df: &'a <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, groups: <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>, selected_agg: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.select" class="fn">select</a>\<I, S\>(self, selection: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'a\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Select the column(s) that should be aggregated. You can select a single column or a slice of columns.

Note that making a selection with this method is not required. If you skip it all columns (except for the keys) will be selected for aggregation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.get_groups" class="fn">get_groups</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

Get the internal representation of the GroupBy operation. The Vec returned contains: (first_idx, [`Vec<indexes>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")) Where second value in the tuple is a vector with all matching indexes.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.get_groups_mut" class="fn">get_groups_mut</a>(&mut self) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

Get the internal representation of the GroupBy operation. The Vec returned contains: (first_idx, [`Vec<indexes>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")) Where second value in the tuple is a vector with all matching indexes.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#safety" class="doc-anchor">§</a>Safety

Groups should always be in bounds of the `DataFrame` hold by this [`GroupBy`](https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html "struct polars::prelude::GroupBy"). If you mutate it, you must hold that invariant.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.take_groups" class="fn">take_groups</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.take_groups_mut" class="fn">take_groups_mut</a>(&mut self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.keys_sliced" class="fn">keys_sliced</a>(&self, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.keys" class="fn">keys</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.mean" class="fn">mean</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped series and compute the mean per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp", "rain"]).mean()
}
```

Returns:

``` text
+------------+-----------+-----------+
| date       | temp_mean | rain_mean |
| ---        | ---       | ---       |
| Date       | f64       | f64       |
+============+===========+===========+
| 2020-08-23 | 9         | 0.1       |
+------------+-----------+-----------+
| 2020-08-22 | 4         | 0.155     |
+------------+-----------+-----------+
| 2020-08-21 | 15        | 0.15      |
+------------+-----------+-----------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.sum" class="fn">sum</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped series and compute the sum per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-1" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).sum()
}
```

Returns:

``` text
+------------+----------+
| date       | temp_sum |
| ---        | ---      |
| Date       | i32      |
+============+==========+
| 2020-08-23 | 9        |
+------------+----------+
| 2020-08-22 | 8        |
+------------+----------+
| 2020-08-21 | 30       |
+------------+----------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.min" class="fn">min</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped series and compute the minimal value per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-2" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).min()
}
```

Returns:

``` text
+------------+----------+
| date       | temp_min |
| ---        | ---      |
| Date       | i32      |
+============+==========+
| 2020-08-23 | 9        |
+------------+----------+
| 2020-08-22 | 1        |
+------------+----------+
| 2020-08-21 | 10       |
+------------+----------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.max" class="fn">max</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped series and compute the maximum value per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-3" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).max()
}
```

Returns:

``` text
+------------+----------+
| date       | temp_max |
| ---        | ---      |
| Date       | i32      |
+============+==========+
| 2020-08-23 | 9        |
+------------+----------+
| 2020-08-22 | 7        |
+------------+----------+
| 2020-08-21 | 20       |
+------------+----------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.first" class="fn">first</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped `Series` and find the first value per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-4" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).first()
}
```

Returns:

``` text
+------------+------------+
| date       | temp_first |
| ---        | ---        |
| Date       | i32        |
+============+============+
| 2020-08-23 | 9          |
+------------+------------+
| 2020-08-22 | 7          |
+------------+------------+
| 2020-08-21 | 20         |
+------------+------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.last" class="fn">last</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped `Series` and return the last value per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-5" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).last()
}
```

Returns:

``` text
+------------+------------+
| date       | temp_last |
| ---        | ---        |
| Date       | i32        |
+============+============+
| 2020-08-23 | 9          |
+------------+------------+
| 2020-08-22 | 1          |
+------------+------------+
| 2020-08-21 | 10         |
+------------+------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.n_unique" class="fn">n_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped `Series` by counting the number of unique values.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-6" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).n_unique()
}
```

Returns:

``` text
+------------+---------------+
| date       | temp_n_unique |
| ---        | ---           |
| Date       | u32           |
+============+===============+
| 2020-08-23 | 1             |
+------------+---------------+
| 2020-08-22 | 2             |
+------------+---------------+
| 2020-08-21 | 2             |
+------------+---------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.quantile" class="fn">quantile</a>( &self, quantile: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, method: <a href="https://docs.rs/polars/latest/polars/prelude/enum.QuantileMethod.html" class="enum" title="enum polars::prelude::QuantileMethod">QuantileMethod</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and determine the quantile per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-7" class="doc-anchor">§</a>Example

``` rust

fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).quantile(0.2, QuantileMethod::default())
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.median" class="fn">median</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and determine the median per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-8" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).median()
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.var" class="fn">var</a>(&self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and determine the variance per group.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.std" class="fn">std</a>(&self, ddof: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate grouped [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and determine the standard deviation per group.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.count" class="fn">count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Aggregate grouped series and compute the number of values per group.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-9" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.select(["temp"]).count()
}
```

Returns:

``` text
+------------+------------+
| date       | temp_count |
| ---        | ---        |
| Date       | u32        |
+============+============+
| 2020-08-23 | 1          |
+------------+------------+
| 2020-08-22 | 2          |
+------------+------------+
| 2020-08-21 | 2          |
+------------+------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.groups" class="fn">groups</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the group_by group indexes.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-10" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["date"])?.groups()
}
```

Returns:

``` text
+--------------+------------+
| date         | groups     |
| ---          | ---        |
| Date(days)   | list [u32] |
+==============+============+
| 2020-08-23   | "[3]"      |
+--------------+------------+
| 2020-08-22   | "[2, 4]"   |
+--------------+------------+
| 2020-08-21   | "[0, 1]"   |
+--------------+------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.agg_list" class="fn">agg_list</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

👎Deprecated since 0.24.1: use polars.lazy aggregations

Aggregate the groups of the group_by operation into lists.

##### <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#example-11" class="doc-anchor">§</a>Example

``` rust
fn example(df: DataFrame) -> PolarsResult<DataFrame> {
    // GroupBy and aggregate to Lists
    df.group_by(["date"])?.select(["temp"]).agg_list()
}
```

Returns:

``` text
+------------+------------------------+
| date       | temp_agg_list          |
| ---        | ---                    |
| Date       | list [i32]             |
+============+========================+
| 2020-08-23 | "[Some(9)]"            |
+------------+------------------------+
| 2020-08-22 | "[Some(7), Some(1)]"   |
+------------+------------------------+
| 2020-08-21 | "[Some(20), Some(10)]" |
+------------+------------------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.par_apply" class="fn">par_apply</a>\<F\>(&self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

👎Deprecated since 0.24.1: use polars.lazy aggregations

Apply a closure over the groups as a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") in parallel.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.apply" class="fn">apply</a>\<F\>(&self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>,

Apply a closure over the groups as a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.sliced" class="fn">sliced</a>(self, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'a\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#impl-Clone-for-GroupBy%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#impl-Debug-for-GroupBy%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/frame/group_by/struct.GroupBy.html#blanket-implementations" class="anchor">§</a>
