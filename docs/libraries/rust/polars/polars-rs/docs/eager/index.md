# Module eager Copy item path

<a href="https://docs.rs/polars/latest/src/polars/docs/eager.rs.html#1-730" class="src">Source</a>

Expand description

## <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#polars-eager-cookbook" class="doc-anchor">§</a>Polars Eager cookbook

This page should serve as a cookbook to quickly get you started with most fundamental operations executed on a [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray"), [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") or [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#tree-of-contents" class="doc-anchor">§</a>Tree Of Contents

- [Creation of data structures](https://docs.rs/polars/latest/polars/docs/eager/index.html#creation-of-data-structures)
  - [ChunkedArray](https://docs.rs/polars/latest/polars/docs/eager/index.html#chunkedarray)
  - [Series](https://docs.rs/polars/latest/polars/docs/eager/index.html#series)
  - [DataFrame](https://docs.rs/polars/latest/polars/docs/eager/index.html#dataframe)
- [Arithmetic](https://docs.rs/polars/latest/polars/docs/eager/index.html#arithmetic)
- [Comparisons](https://docs.rs/polars/latest/polars/docs/eager/index.html#comparisons)
- [Apply functions/ closures](https://docs.rs/polars/latest/polars/docs/eager/index.html#apply-functions-closures)
  - [Series / ChunkedArrays](https://docs.rs/polars/latest/polars/docs/eager/index.html#dataframe-1)
  - [DataFrame](https://docs.rs/polars/latest/polars/docs/eager/index.html#dataframe-1)
- [Filter](https://docs.rs/polars/latest/polars/docs/eager/index.html#filter)
- [Sort](https://docs.rs/polars/latest/polars/docs/eager/index.html#sort)
- [Joins](https://docs.rs/polars/latest/polars/docs/eager/index.html#joins)
- [GroupBy](https://docs.rs/polars/latest/polars/docs/eager/index.html#group_by)
- [pivot](https://docs.rs/polars/latest/polars/docs/eager/index.html#pivot)
- [Unpivot](https://docs.rs/polars/latest/polars/docs/eager/index.html#unpivot)
- [Explode](https://docs.rs/polars/latest/polars/docs/eager/index.html#explode)
- [IO](https://docs.rs/polars/latest/polars/docs/eager/index.html#io)
  - [Read CSV](https://docs.rs/polars/latest/polars/docs/eager/index.html#read-csv)
  - [Write CSV](https://docs.rs/polars/latest/polars/docs/eager/index.html#write-csv)
  - [Read IPC](https://docs.rs/polars/latest/polars/docs/eager/index.html#read-ipc)
  - [Write IPC](https://docs.rs/polars/latest/polars/docs/eager/index.html#write-ipc)
  - [Read Parquet](https://docs.rs/polars/latest/polars/docs/eager/index.html#read-parquet)
  - [Write Parquet](https://docs.rs/polars/latest/polars/docs/eager/index.html#write-parquet)
- [Various](https://docs.rs/polars/latest/polars/docs/eager/index.html#various)
  - [Replace NaN with Missing](https://docs.rs/polars/latest/polars/docs/eager/index.html#replace-nan-with-missing)
  - [Extracting data](https://docs.rs/polars/latest/polars/docs/eager/index.html#extracting-data)

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#creation-of-data-structures" class="doc-anchor">§</a>Creation of data structures

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#chunkedarray" class="doc-anchor">§</a>ChunkedArray

``` rust
use polars::prelude::*;

// use iterators
let ca: UInt32Chunked = (0..10).map(Some).collect();

// from slices
let ca = UInt32Chunked::new("foo".into(), &[1, 2, 3]);

// use builders
let mut builder = PrimitiveChunkedBuilder::<UInt32Type>::new("foo".into(), 10);
for value in 0..10 {
    builder.append_value(value);
}
let ca = builder.finish();
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#series" class="doc-anchor">§</a>Series

``` rust
use polars::prelude::*;

// use iterators
let s: Series = (0..10).map(Some).collect();

// from slices
let s = Series::new("foo".into(), &[1, 2, 3]);

// from a chunked-array
let ca = UInt32Chunked::new("foo".into(), &[Some(1), None, Some(3)]);
let s = ca.into_series();

// into a Column
let s = s.into_column();
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#dataframe" class="doc-anchor">§</a>DataFrame

``` rust
use polars::prelude::*;
use polars::df;

// use macro
let df = df! [
    "names" => ["a", "b", "c"],
    "values" => [1, 2, 3],
    "values_nulls" => [Some(1), None, Some(3)]
]?;

// from a Vec<Column>
let c1 = Column::new("names".into(), &["a", "b", "c"]);
let c2 = Column::new("values".into(), &[Some(1), None, Some(3)]);
let df = DataFrame::new(vec![c1, c2])?;
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#arithmetic" class="doc-anchor">§</a>Arithmetic

Arithmetic can be done on both [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray"). The most notable difference is that a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") coerces the data to match the underlying data types.

``` rust
use polars::prelude::*;
let s_int = Series::new("a".into(), &[1, 2, 3]);
let s_flt = Series::new("b".into(), &[1.0, 2.0, 3.0]);

let added = &s_int + &s_flt;
let subtracted = &s_int - &s_flt;
let multiplied = &s_int * &s_flt;
let divided = &s_int / &s_flt;
let moduloed = &s_int % &s_flt;


// on chunked-arrays we first need to cast to same types
let ca_int = s_int.i32()?;
let ca_flt = s_flt.f32()?;

ca_int.cast(&DataType::Float32)?.f32()? * ca_flt;
ca_flt.cast(&DataType::Int32)?.i32()? * ca_int;

// we can also do arithmetic with numeric values
let multiplied = ca_int * 2.0;
let multiplied = s_flt * 2.0;

// or broadcast Series to match the operands type
let added = &s_int * &Series::new("broadcast_me".into(), &[10]);
```

Because Rust’s Orphan Rule doesn’t allow us to implement left side operations, we need to call such operations directly.

``` rust
let series = Series::new("foo".into(), [1, 2, 3]);

// 1 / s
let divide_one_by_s = 1.div(&series);

// 1 - s
let subtract_one_by_s = 1.sub(&series);
```

For [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") left hand side operations can be done with the [`apply_values`](https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html#tymethod.apply_values "method polars::prelude::ChunkApply::apply_values") method.

``` rust
let ca = UInt32Chunked::new("foo".into(), &[1, 2, 3]);

// 1 / ca
let divide_one_by_ca = ca.apply_values(|rhs| 1 / rhs);
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#comparisons" class="doc-anchor">§</a>Comparisons

[`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") and [`ChunkedArray`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray") can be used in comparison operations to create *boolean* masks/predicates.

``` rust
use polars::prelude::*;

let s = Series::new("a".into(), &[1, 2, 3]);
let ca = UInt32Chunked::new("b".into(), &[Some(3), None, Some(1)]);

// compare Series with numeric values
// ==
s.equal(2);
// !=
s.not_equal(2);
// >
s.gt(2);
// >=
s.gt_eq(2);
// <
s.lt(2);
// <=
s.lt_eq(2);


// compare Series with Series
// ==
s.equal(&s);
// !=
s.not_equal(&s);
// >
s.gt(&s);
// >=
s.gt_eq(&s);
// <
s.lt(&s);
// <=
s.lt_eq(&s);


// compare chunked-array with numeric values
// ==
ca.equal(2);
// !=
ca.not_equal(2);
// >
ca.gt(2);
// >=
ca.gt_eq(2);
// <
ca.lt(2);
// <=
ca.lt_eq(2);

// compare chunked-array with chunked-array
// ==
ca.equal(&ca);
// !=
ca.not_equal(&ca);
// >
ca.gt(&ca);
// >=
ca.gt_eq(&ca);
// <
ca.lt(&ca);
// <=
ca.lt_eq(&ca);

// use iterators
let a: BooleanChunked = ca.iter()
    .map(|opt_value| {
         match opt_value {
         Some(value) => value < 10,
         None => false
}}).collect();
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#apply-functions-closures" class="doc-anchor">§</a>Apply functions/ closures

See all possible [apply methods here](https://docs.rs/polars/latest/polars/prelude/trait.ChunkApply.html "trait polars::prelude::ChunkApply").

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#series--chunkedarrays" class="doc-anchor">§</a>Series / ChunkedArrays

``` rust
use polars::prelude::*;
use polars::prelude::arity::unary_elementwise_values;

// apply a closure over all values
let s = Series::new("foo".into(), &[Some(1), Some(2), None]);
s.i32()?.apply_values(|value| value * 20);

// count string lengths
let s = Series::new("foo".into(), &["foo", "bar", "foobar"]);
unary_elementwise_values::<StringType, UInt64Type, _>(s.str()?, |str_val| str_val.len() as u64);
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#multiple-columns" class="doc-anchor">§</a>Multiple columns

``` rust
use polars::prelude::*;
fn my_black_box_function(a: f32, b: f32) -> f32 {
    // do something
    a
}

fn apply_multiples(col_a: &Series, col_b: &Series) -> Float32Chunked {
    match (col_a.dtype(), col_b.dtype()) {
        (DataType::Float32, DataType::Float32) => {
            // downcast to `ChunkedArray`
            let a = col_a.f32().unwrap();
            let b = col_b.f32().unwrap();

            a.into_iter()
                .zip(b.into_iter())
                .map(|(opt_a, opt_b)| match (opt_a, opt_b) {
                    (Some(a), Some(b)) => Some(my_black_box_function(a, b)),
                    // if either value is `None` we propagate that null
                    _ => None,
                })
                .collect()
        }
        _ => panic!("unexpected dtypes"),
    }
}
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#dataframe-1" class="doc-anchor">§</a>DataFrame

``` rust
use polars::prelude::*;
use polars::df;

let mut df = df![
    "letters" => ["a", "b", "c", "d"],
    "numbers" => [1, 2, 3, 4]
]?;


// coerce numbers to floats
df.try_apply("number", |s: &Series| s.cast(&DataType::Float64))?;

// transform letters to uppercase letters
df.try_apply("letters", |s: &Series| {
    Ok(s.str()?.to_uppercase())
});
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#filter" class="doc-anchor">§</a>Filter

``` rust
use polars::prelude::*;

// create a mask to filter out null values
let mask = df.column("sepal_width")?.is_not_null();

// select column
let s = df.column("sepal_length")?;

// apply filter on a Series
let filtered_series = s.filter(&mask);

// apply the filter on a DataFrame
let filtered_df = df.filter(&mask)?;
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#sort" class="doc-anchor">§</a>Sort

``` rust
use polars::prelude::*;
use polars::df;

let df = df![
    "a" => [1, 2, 3],
    "b" => ["a", "a", "b"]
]?;
// sort this DataFrame by multiple columns

// ordering of the columns
let descending = vec![true, false];
// columns to sort by
let by = [PlSmallStr::from_static("b"), PlSmallStr::from_static("a")];
// do the sort operation
let sorted = df.sort(
    by,
    SortMultipleOptions::default()
        .with_order_descending_multi(descending)
        .with_maintain_order(true)
)?;

// sorted:

// ╭─────┬─────╮
// │ a   ┆ b   │
// │ --- ┆ --- │
// │ i64 ┆ str │
// ╞═════╪═════╡
// │ 1   ┆ "a" │
// │ 2   ┆ "a" │
// │ 3   ┆ "b" │
// ╰─────┴─────╯
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#joins" class="doc-anchor">§</a>Joins

``` rust
use polars::prelude::*;
use polars::df;

// Create first df.
let temp = df!("days" => &[0, 1, 2, 3, 4],
               "temp" => &[22.1, 19.9, 7., 2., 3.],
               "other" => &[1, 2, 3, 4, 5]
)?;

// Create second df.
let rain = df!("days" => &[1, 2],
               "rain" => &[0.1, 0.2],
               "other" => &[1, 2, 3, 4, 5]
)?;

// join on a single column
temp.left_join(&rain, ["days"], ["days"]);
temp.inner_join(&rain, ["days"], ["days"]);
temp.full_join(&rain, ["days"], ["days"]);

// join on multiple columns
temp.join(&rain, vec!["days", "other"], vec!["days", "other"], JoinArgs::new(JoinType::Left), None);
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#groupby" class="doc-anchor">§</a>Groupby

Note that Polars lazy is a lot more powerful in and more performant in group_by operations. In lazy a myriad of aggregations can be combined from expressions.

See more in:

- [Groupby](https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html "struct polars::prelude::GroupBy")

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#groupby-1" class="doc-anchor">§</a>GroupBy

``` rust
use polars::prelude::*;

 // group_by "groups" | sum "foo"
 let out = df.group_by(["groups"])?
    .select(["foo"])
    .sum();
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#pivot" class="doc-anchor">§</a>Pivot

``` rust
use polars::prelude::*;
use polars::df;

 let df = df!("foo" => ["A", "A", "B", "B", "C"],
     "N" => [1, 2, 2, 4, 2],
     "bar" => ["k", "l", "m", "n", "0"]
     )?;

// group_by "foo" | pivot "bar" column | aggregate "N"
 let pivoted = pivot::pivot(
    &df,
    [PlSmallStr::from_static("foo")],
    Some([PlSmallStr::from_static("bar")]),
    Some([PlSmallStr::from_static("N")]),
    false, Some(first()),
    None
);

// pivoted:
// +-----+------+------+------+------+------+
// | foo | o    | n    | m    | l    | k    |
// | --- | ---  | ---  | ---  | ---  | ---  |
// | str | i32  | i32  | i32  | i32  | i32  |
// +=====+======+======+======+======+======+
// | "A" | null | null | null | 2    | 1    |
// +-----+------+------+------+------+------+
// | "B" | null | 4    | 2    | null | null |
// +-----+------+------+------+------+------+
// | "C" | 2    | null | null | null | null |
// +-----+------+------+------+------+------+!
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#unpivot" class="doc-anchor">§</a>Unpivot

``` rust
use polars::prelude::*;
use polars::df;

let df = df!["A" => &["a", "b", "a"],
             "B" => &[1, 3, 5],
             "C" => &[10, 11, 12],
             "D" => &[2, 4, 6]
    ]?;

let unpivoted = df.unpivot(
    [PlSmallStr::from_static("A"), PlSmallStr::from_static("B")],
    [PlSmallStr::from_static("C"), PlSmallStr::from_static("D")],
).unwrap();
// unpivoted:

// +-----+-----+----------+-------+
// | A   | B   | variable | value |
// | --- | --- | ---      | ---   |
// | str | i32 | str      | i32   |
// +=====+=====+==========+=======+
// | "a" | 1   | "C"      | 10    |
// +-----+-----+----------+-------+
// | "b" | 3   | "C"      | 11    |
// +-----+-----+----------+-------+
// | "a" | 5   | "C"      | 12    |
// +-----+-----+----------+-------+
// | "a" | 1   | "D"      | 2     |
// +-----+-----+----------+-------+
// | "b" | 3   | "D"      | 4     |
// +-----+-----+----------+-------+
// | "a" | 5   | "D"      | 6     |
// +-----+-----+----------+-------+
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#explode" class="doc-anchor">§</a>Explode

``` rust
use polars::prelude::*;
use polars::df;

let s0 = Series::new("a".into(), &[1i64, 2, 3]);
let s1 = Series::new("b".into(), &[1i64, 1, 1]);
let s2 = Series::new("c".into(), &[2i64, 2, 2]);
// construct a new ListChunked for a slice of Series.
let list = Column::new("foo".into(), &[s0, s1, s2]);

// construct a few more Series.
let s0 = Column::new("B".into(), [1, 2, 3]);
let s1 = Column::new("C".into(), [1, 1, 1]);
let df = DataFrame::new(vec![list, s0, s1])?;

let exploded = df.explode([PlSmallStr::from("foo")])?;
// exploded:

// +-----+-----+-----+
// | foo | B   | C   |
// | --- | --- | --- |
// | i64 | i32 | i32 |
// +=====+=====+=====+
// | 1   | 1   | 1   |
// +-----+-----+-----+
// | 2   | 1   | 1   |
// +-----+-----+-----+
// | 3   | 1   | 1   |
// +-----+-----+-----+
// | 1   | 2   | 1   |
// +-----+-----+-----+
// | 1   | 2   | 1   |
// +-----+-----+-----+
// | 1   | 2   | 1   |
// +-----+-----+-----+
// | 2   | 3   | 1   |
// +-----+-----+-----+
// | 2   | 3   | 1   |
// +-----+-----+-----+
// | 2   | 3   | 1   |
// +-----+-----+-----+
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#io" class="doc-anchor">§</a>IO

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#read-csv" class="doc-anchor">§</a>Read CSV

``` rust
use polars::prelude::*;

// read from path
let mut file = std::fs::File::open("iris.csv")?;
let df = CsvReader::new(file).finish()?;
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#write-csv" class="doc-anchor">§</a>Write CSV

``` rust
use polars::prelude::*;
use std::fs::File;

// create a file
let mut file = File::create("example.csv").expect("could not create file");

// write DataFrame to file
CsvWriter::new(&mut file)
    .include_header(true)
    .with_separator(b',')
    .finish(df);
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#read-ipc" class="doc-anchor">§</a>Read IPC

``` rust
use polars::prelude::*;
use std::fs::File;

// open file
let file = File::open("file.ipc").expect("file not found");

// read to DataFrame
let df = IpcReader::new(file)
   .finish()?;
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#write-ipc" class="doc-anchor">§</a>Write IPC

``` rust
use polars::prelude::*;
use std::fs::File;

// create a file
let mut file = File::create("file.ipc").expect("could not create file");

// write DataFrame to file
IpcWriter::new(&mut file)
    .finish(df)
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#read-parquet" class="doc-anchor">§</a>Read Parquet

``` rust
use polars::prelude::*;
use std::fs::File;

// open file
let file = File::open("some_file.parquet").unwrap();

// read to DataFrame
let df = ParquetReader::new(file).finish()?;
```

#### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#write-parquet" class="doc-anchor">§</a>Write Parquet

``` rust
use polars::prelude::*;
use std::fs::File;

// create a file
let file = File::create("example.parquet").expect("could not create file");

ParquetWriter::new(file)
    .finish(df)
```

## <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#various" class="doc-anchor">§</a>Various

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#replace-nan-with-missing" class="doc-anchor">§</a>Replace NaN with Missing.

The floating point [Not a Number: NaN](https://en.wikipedia.org/wiki/NaN) is conceptually different than missing data in Polars. In the snippet below we show how we can replace [`NaN`](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.NAN) values with missing values, by setting them to [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None").

``` rust
use polars::prelude::*;
use polars::df;

/// Replaces NaN with missing values.
fn fill_nan_with_nulls() -> PolarsResult<DataFrame> {
    let nan = f64::NAN;

    let mut df = df! {
       "a" => [nan, 1.0, 2.0],
       "b" => [nan, 1.0, 2.0]
    }
    .unwrap();

    for idx in 0..df.width() {
        df.try_apply_at_idx(idx, |series| {
            let mask = series.is_nan()?;
            let ca = series.f64()?;
            ca.set(&mask, None)
        })?;
    }
    Ok(df)
}
```

### <a href="https://docs.rs/polars/latest/polars/docs/eager/index.html#extracting-data" class="doc-anchor">§</a>Extracting data

To iterate over the values of a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series"), or to convert the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") into another structure such as a [`Vec<T>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"), we must first downcast to a data type aware [`ChunkedArray<T>`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html "struct polars::prelude::ChunkedArray").

``` rust
use polars::prelude::*;
use polars::df;

fn extract_data() -> PolarsResult<()> {
    let df = df! [
       "a" => [None, Some(1.0f32), Some(2.0)],
       "str" => ["foo", "bar", "ham"]
    ]?;

    // first extract ChunkedArray to get the inner type.
    let ca = df.column("a")?.f32()?;

    // Then convert to vec
    let to_vec: Vec<Option<f32>> = Vec::from(ca);

    // We can also do this with iterators
    let ca = df.column("str")?.str()?;
    let to_vec: Vec<Option<&str>> = ca.into_iter().collect();
    let to_vec_no_options: Vec<&str> = ca.into_no_null_iter().collect();

    Ok(())
}
```
