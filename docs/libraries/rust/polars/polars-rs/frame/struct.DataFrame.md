# Struct DataFrame Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/mod.rs.html#173" class="src">Source</a>

``` rust
pub struct DataFrame { /* private fields */ }
```

Expand description

A contiguous growable collection of `Series` that have the same length.

### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#use-declarations" class="doc-anchor">§</a>Use declarations

All the common tools can be found in [`crate::prelude`](https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/prelude/index.html "mod polars_core::prelude") (or in `polars::prelude`).

``` rust
use polars_core::prelude::*; // if the crate polars-core is used directly
// use polars::prelude::*;      if the crate polars is used
```

## <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#initialization" class="doc-anchor">§</a>Initialization

### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#default" class="doc-anchor">§</a>Default

A `DataFrame` can be initialized empty:

``` rust
let df = DataFrame::default();
assert!(df.is_empty());
```

### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#wrapping-a-vecseries" class="doc-anchor">§</a>Wrapping a `Vec<Series>`

A `DataFrame` is built upon a `Vec<Series>` where the `Series` have the same length.

``` rust
let s1 = Column::new("Fruit".into(), ["Apple", "Apple", "Pear"]);
let s2 = Column::new("Color".into(), ["Red", "Yellow", "Green"]);

let df: PolarsResult<DataFrame> = DataFrame::new(vec![s1, s2]);
```

### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#using-a-macro" class="doc-anchor">§</a>Using a macro

The [`df!`](https://docs.rs/polars/latest/polars/macro.df.html "macro polars::df") macro is a convenient method:

``` rust
let df: PolarsResult<DataFrame> = df!("Fruit" => ["Apple", "Apple", "Pear"],
                                      "Color" => ["Red", "Yellow", "Green"]);
```

### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#using-a-csv-file" class="doc-anchor">§</a>Using a CSV file

See the `polars_io::csv::CsvReader`.

## <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#indexing" class="doc-anchor">§</a>Indexing

### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#by-a-number" class="doc-anchor">§</a>By a number

The `Index<usize>` is implemented for the `DataFrame`.

``` rust
let df = df!("Fruit" => ["Apple", "Apple", "Pear"],
             "Color" => ["Red", "Yellow", "Green"])?;

assert_eq!(df[0], Column::new("Fruit".into(), &["Apple", "Apple", "Pear"]));
assert_eq!(df[1], Column::new("Color".into(), &["Red", "Yellow", "Green"]));
```

### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#by-a-series-name" class="doc-anchor">§</a>By a `Series` name

``` rust
let df = df!("Fruit" => ["Apple", "Apple", "Pear"],
             "Color" => ["Red", "Yellow", "Green"])?;

assert_eq!(df["Fruit"], Column::new("Fruit".into(), &["Apple", "Apple", "Pear"]));
assert_eq!(df["Color"], Column::new("Color".into(), &["Red", "Yellow", "Green"]));
```

## Implementations<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.to_ndarray" class="fn">to_ndarray</a>\<N\>( &self, ordering: <a href="https://docs.rs/polars/latest/polars/prelude/enum.IndexOrder.html" class="enum" title="enum polars::prelude::IndexOrder">IndexOrder</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/ndarray/0.16.1/x86_64-unknown-linux-gnu/ndarray/struct.ArrayBase.html" class="struct" title="struct ndarray::ArrayBase">ArrayBase</a>\<<a href="https://docs.rs/ndarray/0.16.1/x86_64-unknown-linux-gnu/ndarray/data_repr/struct.OwnedRepr.html" class="struct" title="struct ndarray::data_repr::OwnedRepr">OwnedRepr</a>\<\<N as <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html#associatedtype.Native" class="associatedtype" title="type polars::prelude::PolarsNumericType::Native">Native</a>\>, <a href="https://docs.rs/ndarray/0.16.1/x86_64-unknown-linux-gnu/ndarray/dimension/dim/struct.Dim.html" class="struct" title="struct ndarray::dimension::dim::Dim">Dim</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\]\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where N: <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsNumericType.html" class="trait" title="trait polars::prelude::PolarsNumericType">PolarsNumericType</a>,

Create a 2D [`ndarray::Array`](https://docs.rs/ndarray/0.16.1/x86_64-unknown-linux-gnu/ndarray/type.Array.html "type ndarray::Array") from this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame"). This requires all columns in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") to be non-null and numeric. They will be cast to the same data type (if they aren’t already).

For floating point data we implicitly convert `None` to `NaN` without failure.

``` rust
use polars_core::prelude::*;
let a = UInt32Chunked::new("a".into(), &[1, 2, 3]).into_column();
let b = Float64Chunked::new("b".into(), &[10., 8., 6.]).into_column();

let df = DataFrame::new(vec![a, b]).unwrap();
let ndarray = df.to_ndarray::<Float64Type>(IndexOrder::Fortran).unwrap();
println!("{:?}", ndarray);
```

Outputs:

``` text
[[1.0, 10.0],
 [2.0, 8.0],
 [3.0, 6.0]], shape=[3, 2], strides=[1, 3], layout=Ff (0xa), const ndim=2
```

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.sample_n" class="fn">sample_n</a>( &self, n: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, with_replacement: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, shuffle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Sample n datapoints from this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.sample_n_literal" class="fn">sample_n_literal</a>( &self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, with_replacement: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, shuffle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.sample_frac" class="fn">sample_frac</a>( &self, frac: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, with_replacement: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, shuffle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, seed: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Sample a fraction between 0.0-1.0 of this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-2" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.into_struct" class="fn">into_struct</a>(self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructType.html" class="struct" title="struct polars::prelude::StructType">StructType</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-3" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.split_chunks" class="fn">split_chunks</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.split_chunks_by_n" class="fn">split_chunks_by_n</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, parallel: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-4" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.explode_impl" class="fn">explode_impl</a>( &self, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.explode" class="fn">explode</a>\<I, S\>(&self, columns: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Explode `DataFrame` to long format by exploding a column with Lists.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example" class="doc-anchor">§</a>Example

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
let s0 = Series::new("a".into(), &[1i64, 2, 3]);
let s1 = Series::new("b".into(), &[1i64, 1, 1]);
let s2 = Series::new("c".into(), &[2i64, 2, 2]);
let list = Series::new("foo", &[s0, s1, s2]);

let s0 = Series::new("B".into(), [1, 2, 3]);
let s1 = Series::new("C".into(), [1, 1, 1]);
let df = DataFrame::new(vec![list, s0, s1])?;
let exploded = df.explode(["foo"])?;

println!("{:?}", df);
println!("{:?}", exploded);
```

Outputs:

``` text
 +-------------+-----+-----+
 | foo         | B   | C   |
 | ---         | --- | --- |
 | list [i64]  | i32 | i32 |
 +=============+=====+=====+
 | "[1, 2, 3]" | 1   | 1   |
 +-------------+-----+-----+
 | "[1, 1, 1]" | 2   | 1   |
 +-------------+-----+-----+
 | "[2, 2, 2]" | 3   | 1   |
 +-------------+-----+-----+

 +-----+-----+-----+
 | foo | B   | C   |
 | --- | --- | --- |
 | i64 | i32 | i32 |
 +=====+=====+=====+
 | 1   | 1   | 1   |
 +-----+-----+-----+
 | 2   | 1   | 1   |
 +-----+-----+-----+
 | 3   | 1   | 1   |
 +-----+-----+-----+
 | 1   | 2   | 1   |
 +-----+-----+-----+
 | 1   | 2   | 1   |
 +-----+-----+-----+
 | 1   | 2   | 1   |
 +-----+-----+-----+
 | 2   | 3   | 1   |
 +-----+-----+-----+
 | 2   | 3   | 1   |
 +-----+-----+-----+
 | 2   | 3   | 1   |
 +-----+-----+-----+
```

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-5" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.group_by_with_series" class="fn">group_by_with_series</a>( &self, by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, multithreaded: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, sorted: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.group_by" class="fn">group_by</a>\<I, S\>(&self, by: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Group DataFrame using a Series column.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-1" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
fn group_by_sum(df: &DataFrame) -> PolarsResult<DataFrame> {
    df.group_by(["column_name"])?
    .select(["agg_column_name"])
    .sum()
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.group_by_stable" class="fn">group_by_stable</a>\<I, S\>(&self, by: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupBy.html" class="struct" title="struct polars::prelude::GroupBy">GroupBy</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Group DataFrame using a Series column. The groups are ordered by their smallest row index.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-6" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.hstack_mut_unchecked" class="fn">hstack_mut_unchecked</a>( &mut self, columns: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\], ) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Add columns horizontally.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety" class="doc-anchor">§</a>Safety

The caller must ensure:

- the length of all [`Column`](https://docs.rs/polars/latest/polars/prelude/enum.Column.html "enum polars::prelude::Column") is equal to the height of this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")
- the columns names are unique

Note: If `self` is empty, `self.height` will always be overridden by the height of the first column in `columns`.

Note that on a debug build this will panic on duplicates / height mismatch.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.hstack_mut" class="fn">hstack_mut</a>( &mut self, columns: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Add multiple [`Column`](https://docs.rs/polars/latest/polars/prelude/enum.Column.html "enum polars::prelude::Column") to a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame"). Errors if the resulting DataFrame columns have duplicate names or unequal heights.

Note: If `self` is empty, `self.height` will always be overridden by the height of the first column in `columns`.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-2" class="doc-anchor">§</a>Example

``` rust
fn stack(df: &mut DataFrame, columns: &[Column]) {
    df.hstack_mut(columns);
}
```

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-7" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_row" class="fn">get_row</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a row from a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame"). Use of this is discouraged as it will likely be slow.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_row_amortized" class="fn">get_row_amortized</a>\<'a\>( &'a self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, row: &mut <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Amortize allocations by reusing a row. The caller is responsible to make sure that the row has at least the capacity for the number of columns in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_row_amortized_unchecked" class="fn">get_row_amortized_unchecked</a>\<'a\>( &'a self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, row: &mut <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>, )

Amortize allocations by reusing a row. The caller is responsible to make sure that the row has at least the capacity for the number of columns in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-1" class="doc-anchor">§</a>Safety

Does not do any bounds checking.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.from_rows_and_schema" class="fn">from_rows_and_schema</a>( rows: &\[<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'\_\>\], schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") from rows.

This should only be used when you have row wise data, as this is a lot slower than creating the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") in a columnar fashion

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.from_rows_iter_and_schema" class="fn">from_rows_iter_and_schema</a>\<'a, I\>( rows: I, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &'a <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>\>,

Create a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") from an iterator over rows.

This should only be used when you have row wise data, as this is a lot slower than creating the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") in a columnar fashion.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.try_from_rows_iter_and_schema" class="fn">try_from_rows_iter_and_schema</a>\<'a, I\>( rows: I, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&'a <a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'a\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>\>,

Create a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") from an iterator over rows. This should only be used when you have row wise data, as this is a lot slower than creating the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") in a columnar fashion

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.from_rows" class="fn">from_rows</a>(rows: &\[<a href="https://docs.rs/polars/latest/polars/frame/row/struct.Row.html" class="struct" title="struct polars::frame::row::Row">Row</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") from rows. This should only be used when you have row wise data, as this is a lot slower than creating the [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") in a columnar fashion

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-8" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.transpose" class="fn">transpose</a>( &mut self, keep_names_as: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, new_col_names: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html" class="enum" title="enum either::Either">Either</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.transpose_impl" class="fn">transpose_impl</a>( &mut self, keep_names_as: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, new_col_names: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html" class="enum" title="enum either::Either">Either</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Transpose a DataFrame. This is a very expensive operation.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-9" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.validate_columns_slice" class="fn">validate_columns_slice</a>(columns: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Ensure all equal height and names are unique.

An Ok() result indicates `columns` is a valid state for a DataFrame.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-10" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.clear_schema" class="fn">clear_schema</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.column_iter" class="fn">column_iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.materialized_column_iter" class="fn">materialized_column_iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.par_materialized_column_iter" class="fn">par_materialized_column_iter</a>( &self, ) -\> impl <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.estimated_size" class="fn">estimated_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns an estimation of the total (heap) allocated size of the `DataFrame` in bytes.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#implementation" class="doc-anchor">§</a>Implementation

This estimation is the sum of the size of its buffers, validity, including nested arrays. Multiple arrays may share buffers and bitmaps. Therefore, the size of 2 arrays is not the sum of the sizes computed from this function. In particular, [`StructArray`](https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html "struct polars::prelude::StructArray")’s size is an upper bound.

When an array is sliced, its allocated size remains constant because the buffer unchanged. However, this function will yield a smaller number. This is because this function returns the visible size of the buffer, not its total capacity.

FFI buffers are included in this estimation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._apply_columns" class="fn">_apply_columns</a>(&self, func: &dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._apply_columns_par" class="fn">_apply_columns_par</a>( &self, func: &(dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a>), ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new" class="fn">new</a>(columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Create a DataFrame from a Vector of Series.

Errors if a column names are not unique, or if heights are not all equal.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-3" class="doc-anchor">§</a>Example

``` rust
let s0 = Column::new("days".into(), [0, 1, 2].as_ref());
let s1 = Column::new("temp".into(), [22.1, 19.9, 7.].as_ref());

let df = DataFrame::new(vec![s0, s1])?;
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new_with_height" class="fn">new_with_height</a>( height: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new_with_broadcast" class="fn">new_with_broadcast</a>( columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Converts a sequence of columns into a DataFrame, broadcasting length-1 columns to match the other columns.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new_with_broadcast_len" class="fn">new_with_broadcast_len</a>( columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, broadcast_len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Converts a sequence of columns into a DataFrame, broadcasting length-1 columns to broadcast_len.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new_with_broadcast_no_namecheck" class="fn">new_with_broadcast_no_namecheck</a>( columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, broadcast_len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Converts a sequence of columns into a DataFrame, broadcasting length-1 columns to match the other columns.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-2" class="doc-anchor">§</a>Safety

Does not check that the column names are unique (which they must be).

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new_from_index" class="fn">new_from_index</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, height: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub const fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.empty" class="fn">empty</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Creates an empty `DataFrame` usable in a compile time context (such as static initializers).

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-4" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::DataFrame;
static EMPTY: DataFrame = DataFrame::empty();
```

#### pub const fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.empty_with_height" class="fn">empty_with_height</a>(height: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Creates an empty `DataFrame` with a specific `height`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.empty_with_schema" class="fn">empty_with_schema</a>(schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Create an empty `DataFrame` with empty columns as per the `schema`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.empty_with_arrow_schema" class="fn">empty_with_arrow_schema</a>(schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Create an empty `DataFrame` with empty columns as per the `schema`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.full_null" class="fn">full_null</a>(schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, height: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Create a new `DataFrame` with the given schema, only containing nulls.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.pop" class="fn">pop</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

Removes the last `Series` from the `DataFrame` and returns it, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is empty.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-5" class="doc-anchor">§</a>Example

``` rust
let s1 = Column::new("Ocean".into(), ["Atlantic", "Indian"]);
let s2 = Column::new("Area (km²)".into(), [106_460_000, 70_560_000]);
let mut df = DataFrame::new(vec![s1.clone(), s2.clone()])?;

assert_eq!(df.pop(), Some(s2));
assert_eq!(df.pop(), Some(s1));
assert_eq!(df.pop(), None);
assert!(df.is_empty());
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.with_row_index" class="fn">with_row_index</a>( &self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, offset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Add a new column at index 0 that counts the rows.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-6" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Name" => ["James", "Mary", "John", "Patricia"])?;
assert_eq!(df1.shape(), (4, 1));

let df2: DataFrame = df1.with_row_index("Id".into(), None)?;
assert_eq!(df2.shape(), (4, 2));
println!("{}", df2);
```

Output:

``` text
 shape: (4, 2)
 +-----+----------+
 | Id  | Name     |
 | --- | ---      |
 | u32 | str      |
 +=====+==========+
 | 0   | James    |
 +-----+----------+
 | 1   | Mary     |
 +-----+----------+
 | 2   | John     |
 +-----+----------+
 | 3   | Patricia |
 +-----+----------+
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.with_row_index_mut" class="fn">with_row_index_mut</a>( &mut self, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, offset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>, ) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Add a row index column in place.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-3" class="doc-anchor">§</a>Safety

The caller should ensure the DataFrame does not already contain a column with the given name.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#panics" class="doc-anchor">§</a>Panics

Panics if the resulting column would reach or overflow IdxSize::MAX.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new_no_checks_height_from_first" class="fn">new_no_checks_height_from_first</a>(columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Create a new `DataFrame` but does not check the length or duplicate occurrence of the `Series`.

Calculates the height from the first column or `0` if no columns are given.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-4" class="doc-anchor">§</a>Safety

It is the callers responsibility to uphold the contract of all `Series` having an equal length and a unique name, if not this may panic down the line.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new_no_checks" class="fn">new_no_checks</a>(height: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Create a new `DataFrame` but does not check the length or duplicate occurrence of the `Series`.

It is advised to use [DataFrame::new](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.new "associated function polars::prelude::DataFrame::new") in favor of this method.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-5" class="doc-anchor">§</a>Safety

It is the callers responsibility to uphold the contract of all `Series` having an equal length and a unique name, if not this may panic down the line.

#### pub const unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._new_no_checks_impl" class="fn">_new_no_checks_impl</a>( height: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

This will not panic even in debug mode - there are some (rare) use cases where a DataFrame is temporarily constructed containing duplicates for dispatching to functions. A DataFrame constructed with this method is generally highly unsafe and should not be long-lived.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.shrink_to_fit" class="fn">shrink_to_fit</a>(&mut self)

Shrink the capacity of this DataFrame to fit its length.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.as_single_chunk" class="fn">as_single_chunk</a>(&mut self) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Aggregate all the chunks in the DataFrame to a single chunk.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.as_single_chunk_par" class="fn">as_single_chunk_par</a>(&mut self) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Aggregate all the chunks in the DataFrame to a single chunk in parallel. This may lead to more peak memory consumption.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.rechunk_mut" class="fn">rechunk_mut</a>(&mut self)

Rechunks all columns to only have a single chunk.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._deshare_views_mut" class="fn">_deshare_views_mut</a>(&mut self)

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.rechunk_to_record_batch" class="fn">rechunk_to_record_batch</a>( self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>, ) -\> <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html" class="struct" title="struct polars_arrow::record_batch::RecordBatchT">RecordBatchT</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>

Rechunks all columns to only have a single chunk and turns it into a [`RecordBatchT`](https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html "struct polars_arrow::record_batch::RecordBatchT").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.should_rechunk" class="fn">should_rechunk</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the chunks of the columns do not align and re-chunking should be done

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.align_chunks_par" class="fn">align_chunks_par</a>(&mut self) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Ensure all the chunks in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") are aligned.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.align_chunks" class="fn">align_chunks</a>(&mut self) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>

Get the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") schema.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-7" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Thing" => ["Observable universe", "Human stupidity"],
                        "Diameter (m)" => [8.8e26, f64::INFINITY])?;

let f1: Field = Field::new("Thing".into(), DataType::String);
let f2: Field = Field::new("Diameter (m)".into(), DataType::Float64);
let sc: Schema = Schema::from_iter(vec![f1, f2]);

assert_eq!(&**df.schema(), &sc);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_columns" class="fn">get_columns</a>(&self) -\> &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]

Get a reference to the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") columns.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-8" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Name" => ["Adenine", "Cytosine", "Guanine", "Thymine"],
                        "Symbol" => ["A", "C", "G", "T"])?;
let columns: &[Column] = df.get_columns();

assert_eq!(columns[0].name(), "Name");
assert_eq!(columns[1].name(), "Symbol");
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_columns_mut" class="fn">get_columns_mut</a>(&mut self) -\> &mut <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

Get mutable access to the underlying columns.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-6" class="doc-anchor">§</a>Safety

The caller must ensure the length of all [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") remains equal to `height` or [`DataFrame::set_height`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.set_height "method polars::prelude::DataFrame::set_height") is called afterwards with the appropriate `height`. The caller must ensure that the cached schema is cleared if it modifies the schema by calling [`DataFrame::clear_schema`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.clear_schema "method polars::prelude::DataFrame::clear_schema").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.clear_columns" class="fn">clear_columns</a>(&mut self)

Remove all the columns in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") but keep the `height`.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.column_extend_unchecked" class="fn">column_extend_unchecked</a>( &mut self, iter: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, )

Extend the columns without checking for name collisions or height.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-7" class="doc-anchor">§</a>Safety

The caller needs to ensure that:

- Column names are unique within the resulting [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").
- The length of each appended column matches the height of the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame"). For `DataFrame`\]s with no columns (ZCDFs), it is important that the height is set afterwards with [`DataFrame::set_height`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.set_height "method polars::prelude::DataFrame::set_height").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.take_columns" class="fn">take_columns</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

Take ownership of the underlying columns vec.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.iter" class="fn">iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>

Iterator over the columns as [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-9" class="doc-anchor">§</a>Example

``` rust
let s1 = Column::new("Name".into(), ["Pythagoras' theorem", "Shannon entropy"]);
let s2 = Column::new("Formula".into(), ["a²+b²=c²", "H=-Σ[P(x)log|P(x)|]"]);
let df: DataFrame = DataFrame::new(vec![s1.clone(), s2.clone()])?;

let mut iterator = df.iter();

assert_eq!(iterator.next(), Some(s1.as_materialized_series()));
assert_eq!(iterator.next(), Some(s2.as_materialized_series()));
assert_eq!(iterator.next(), None);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_column_names" class="fn">get_column_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-10" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Language" => ["Rust", "Python"],
                        "Designer" => ["Graydon Hoare", "Guido van Rossum"])?;

assert_eq!(df.get_column_names(), &["Language", "Designer"]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_column_names_owned" class="fn">get_column_names_owned</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>

Get the [`Vec<PlSmallStr>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") representing the column names.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_column_names_str" class="fn">get_column_names_str</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.set_column_names" class="fn">set_column_names</a>\<I, S\>(&mut self, names: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Set the column names.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-11" class="doc-anchor">§</a>Example

``` rust
let mut df: DataFrame = df!("Mathematical set" => ["ℕ", "ℤ", "𝔻", "ℚ", "ℝ", "ℂ"])?;
df.set_column_names(["Set"])?;

assert_eq!(df.get_column_names(), &["Set"]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.dtypes" class="fn">dtypes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>

Get the data types of the columns in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-12" class="doc-anchor">§</a>Example

``` rust
let venus_air: DataFrame = df!("Element" => ["Carbon dioxide", "Nitrogen"],
                               "Fraction" => [0.965, 0.035])?;

assert_eq!(venus_air.dtypes(), &[DataType::String, DataType::Float64]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.first_col_n_chunks" class="fn">first_col_n_chunks</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The number of chunks for the first column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.max_n_chunks" class="fn">max_n_chunks</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

The highest number of chunks for any column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.fields" class="fn">fields</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field">Field</a>\>

Get a reference to the schema fields of the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-13" class="doc-anchor">§</a>Example

``` rust
let earth: DataFrame = df!("Surface type" => ["Water", "Land"],
                           "Fraction" => [0.708, 0.292])?;

let f1: Field = Field::new("Surface type".into(), DataType::String);
let f2: Field = Field::new("Fraction".into(), DataType::Float64);

assert_eq!(earth.fields(), &[f1, f2]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.shape" class="fn">shape</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Get (height, width) of the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-14" class="doc-anchor">§</a>Example

``` rust
let df0: DataFrame = DataFrame::default();
let df1: DataFrame = df!("1" => [1, 2, 3, 4, 5])?;
let df2: DataFrame = df!("1" => [1, 2, 3, 4, 5],
                         "2" => [1, 2, 3, 4, 5])?;

assert_eq!(df0.shape(), (0 ,0));
assert_eq!(df1.shape(), (5, 1));
assert_eq!(df2.shape(), (5, 2));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.width" class="fn">width</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the width of the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") which is the number of columns.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-15" class="doc-anchor">§</a>Example

``` rust
let df0: DataFrame = DataFrame::default();
let df1: DataFrame = df!("Series 1" => [0; 0])?;
let df2: DataFrame = df!("Series 1" => [0; 0],
                         "Series 2" => [0; 0])?;

assert_eq!(df0.width(), 0);
assert_eq!(df1.width(), 1);
assert_eq!(df2.width(), 2);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.height" class="fn">height</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the height of the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") which is the number of rows.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-16" class="doc-anchor">§</a>Example

``` rust
let df0: DataFrame = DataFrame::default();
let df1: DataFrame = df!("Currency" => ["€", "$"])?;
let df2: DataFrame = df!("Currency" => ["€", "$", "¥", "£", "₿"])?;

assert_eq!(df0.height(), 0);
assert_eq!(df1.height(), 2);
assert_eq!(df2.height(), 5);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the size as number of rows \* number of columns

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") contains no rows.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-17" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = DataFrame::default();
assert!(df1.is_empty());

let df2: DataFrame = df!("First name" => ["Forever"],
                         "Last name" => ["Alone"])?;
assert!(!df2.is_empty());
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.set_height" class="fn">set_height</a>(&mut self, height: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Set the height (i.e. number of rows) of this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-8" class="doc-anchor">§</a>Safety

This needs to be equal to the length of all the columns.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.hstack" class="fn">hstack</a>(&self, columns: &\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Add multiple [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") to a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame"). The added `Series` are required to have the same length.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-18" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Element" => ["Copper", "Silver", "Gold"])?;
let s1 = Column::new("Proton".into(), [29, 47, 79]);
let s2 = Column::new("Electron".into(), [29, 47, 79]);

let df2: DataFrame = df1.hstack(&[s1, s2])?;
assert_eq!(df2.shape(), (3, 3));
println!("{}", df2);
```

Output:

``` text
shape: (3, 3)
+---------+--------+----------+
| Element | Proton | Electron |
| ---     | ---    | ---      |
| str     | i32    | i32      |
+=========+========+==========+
| Copper  | 29     | 29       |
+---------+--------+----------+
| Silver  | 47     | 47       |
+---------+--------+----------+
| Gold    | 79     | 79       |
+---------+--------+----------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.vstack" class="fn">vstack</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Concatenate a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") to this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") and return as newly allocated [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

If many `vstack` operations are done, it is recommended to call [`DataFrame::align_chunks_par`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.align_chunks_par "method polars::prelude::DataFrame::align_chunks_par").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-19" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Element" => ["Copper", "Silver", "Gold"],
                         "Melting Point (K)" => [1357.77, 1234.93, 1337.33])?;
let df2: DataFrame = df!("Element" => ["Platinum", "Palladium"],
                         "Melting Point (K)" => [2041.4, 1828.05])?;

let df3: DataFrame = df1.vstack(&df2)?;

assert_eq!(df3.shape(), (5, 2));
println!("{}", df3);
```

Output:

``` text
shape: (5, 2)
+-----------+-------------------+
| Element   | Melting Point (K) |
| ---       | ---               |
| str       | f64               |
+===========+===================+
| Copper    | 1357.77           |
+-----------+-------------------+
| Silver    | 1234.93           |
+-----------+-------------------+
| Gold      | 1337.33           |
+-----------+-------------------+
| Platinum  | 2041.4            |
+-----------+-------------------+
| Palladium | 1828.05           |
+-----------+-------------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.vstack_mut" class="fn">vstack_mut</a>( &mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Concatenate a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") to this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")

If many `vstack` operations are done, it is recommended to call [`DataFrame::align_chunks_par`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.align_chunks_par "method polars::prelude::DataFrame::align_chunks_par").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-20" class="doc-anchor">§</a>Example

``` rust
let mut df1: DataFrame = df!("Element" => ["Copper", "Silver", "Gold"],
                         "Melting Point (K)" => [1357.77, 1234.93, 1337.33])?;
let df2: DataFrame = df!("Element" => ["Platinum", "Palladium"],
                         "Melting Point (K)" => [2041.4, 1828.05])?;

df1.vstack_mut(&df2)?;

assert_eq!(df1.shape(), (5, 2));
println!("{}", df1);
```

Output:

``` text
shape: (5, 2)
+-----------+-------------------+
| Element   | Melting Point (K) |
| ---       | ---               |
| str       | f64               |
+===========+===================+
| Copper    | 1357.77           |
+-----------+-------------------+
| Silver    | 1234.93           |
+-----------+-------------------+
| Gold      | 1337.33           |
+-----------+-------------------+
| Platinum  | 2041.4            |
+-----------+-------------------+
| Palladium | 1828.05           |
+-----------+-------------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.vstack_mut_owned" class="fn">vstack_mut_owned</a>( &mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.vstack_mut_unchecked" class="fn">vstack_mut_unchecked</a>(&mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>)

Concatenate a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") to this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")

If many `vstack` operations are done, it is recommended to call [`DataFrame::align_chunks_par`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.align_chunks_par "method polars::prelude::DataFrame::align_chunks_par").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#panics-1" class="doc-anchor">§</a>Panics

Panics if the schema’s don’t match.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.vstack_mut_owned_unchecked" class="fn">vstack_mut_owned_unchecked</a>(&mut self, other: <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>)

Concatenate a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") to this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")

If many `vstack` operations are done, it is recommended to call [`DataFrame::align_chunks_par`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.align_chunks_par "method polars::prelude::DataFrame::align_chunks_par").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#panics-2" class="doc-anchor">§</a>Panics

Panics if the schema’s don’t match.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.extend" class="fn">extend</a>(&mut self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Extend the memory backed by this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") with the values from `other`.

Different from [`vstack`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.vstack "method polars::prelude::DataFrame::vstack") which adds the chunks from `other` to the chunks of this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") `extend` appends the data from `other` to the underlying memory locations and thus may cause a reallocation.

If this does not cause a reallocation, the resulting data structure will not have any extra chunks and thus will yield faster queries.

Prefer `extend` over `vstack` when you want to do a query after a single append. For instance during online operations where you add `n` rows and rerun a query.

Prefer `vstack` over `extend` when you want to append many times before doing a query. For instance when you read in multiple files and when to store them in a single `DataFrame`. In the latter case, finish the sequence of `append` operations with a [`rechunk`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.align_chunks_par "method polars::prelude::DataFrame::align_chunks_par").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.drop_in_place" class="fn">drop_in_place</a>(&mut self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Remove a column by name and return the column removed.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-21" class="doc-anchor">§</a>Example

``` rust
let mut df: DataFrame = df!("Animal" => ["Tiger", "Lion", "Great auk"],
                            "IUCN" => ["Endangered", "Vulnerable", "Extinct"])?;

let s1: PolarsResult<Column> = df.drop_in_place("Average weight");
assert!(s1.is_err());

let s2: Column = df.drop_in_place("Animal")?;
assert_eq!(s2, Column::new("Animal".into(), &["Tiger", "Lion", "Great auk"]));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.drop_nulls" class="fn">drop_nulls</a>\<S\>( &self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[S]</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a S</a>: for\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Return a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") where all null values are dropped.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-22" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Country" => ["Malta", "Liechtenstein", "North Korea"],
                        "Tax revenue (% GDP)" => [Some(32.7), None, None])?;
assert_eq!(df1.shape(), (3, 2));

let df2: DataFrame = df1.drop_nulls::<String>(None)?;
assert_eq!(df2.shape(), (1, 2));
println!("{}", df2);
```

Output:

``` text
shape: (1, 2)
+---------+---------------------+
| Country | Tax revenue (% GDP) |
| ---     | ---                 |
| str     | f64                 |
+=========+=====================+
| Malta   | 32.7                |
+---------+---------------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.drop" class="fn">drop</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Drop a column by name. This is a pure method and will return a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") instead of modifying the current one in place.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-23" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Ray type" => ["α", "β", "X", "γ"])?;
let df2: DataFrame = df1.drop("Ray type")?;

assert!(df2.is_empty());
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.drop_many" class="fn">drop_many</a>\<I, S\>(&self, names: I) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Drop columns that are in `names`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.drop_many_amortized" class="fn">drop_many_amortized</a>( &self, names: &<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Drop columns that are in `names` without allocating a [`HashSet`](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.insert_column" class="fn">insert_column</a>\<S\>( &mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, column: S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where S: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a>,

Insert a new column at a given index.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.with_column" class="fn">with_column</a>\<C\>( &mut self, column: C, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where C: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a>,

Add a new column to this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") or replace an existing one.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.with_column_unchecked" class="fn">with_column_unchecked</a>(&mut self, column: <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Adds a column to the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") without doing any checks on length or duplicates.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-9" class="doc-anchor">§</a>Safety

The caller must ensure `self.width() == 0 || column.len() == self.height()` .

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._add_series" class="fn">_add_series</a>( &mut self, series: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._add_columns" class="fn">_add_columns</a>( &mut self, columns: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.with_column_and_schema" class="fn">with_column_and_schema</a>\<C\>( &mut self, column: C, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where C: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a>,

Add a new column to this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") or replace an existing one. Uses an existing schema to amortize lookups. If the schema is incorrect, we will fallback to linear search.

Note: Schema can be both input or output_schema

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get" class="fn">get</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>\>\>

Get a row in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame"). Beware this is slow.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-24" class="doc-anchor">§</a>Example

``` rust
fn example(df: &mut DataFrame, idx: usize) -> Option<Vec<AnyValue>> {
    df.get(idx)
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.select_at_idx" class="fn">select_at_idx</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

Select a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") by index.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-25" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Star" => ["Sun", "Betelgeuse", "Sirius A", "Sirius B"],
                        "Absolute magnitude" => [4.83, -5.85, 1.42, 11.18])?;

let s1: Option<&Column> = df.select_at_idx(0);
let s2 = Column::new("Star".into(), ["Sun", "Betelgeuse", "Sirius A", "Sirius B"]);

assert_eq!(s1, Some(&s2));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.select_by_range" class="fn">select_by_range</a>\<R\>(&self, range: R) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where R: <a href="https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html" class="trait" title="trait core::ops::range::RangeBounds">RangeBounds</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>,

Select column(s) from this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") by range and return a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#examples" class="doc-anchor">§</a>Examples

``` rust
let df = df! {
    "0" => [0, 0, 0],
    "1" => [1, 1, 1],
    "2" => [2, 2, 2]
}?;

assert!(df.select(["0", "1"])?.equals(&df.select_by_range(0..=1)?));
assert!(df.equals(&df.select_by_range(..)?));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_column_index" class="fn">get_column_index</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get column index of a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") by name.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-26" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Name" => ["Player 1", "Player 2", "Player 3"],
                        "Health" => [100, 200, 500],
                        "Mana" => [250, 100, 0],
                        "Strength" => [30, 150, 300])?;

assert_eq!(df.get_column_index("Name"), Some(0));
assert_eq!(df.get_column_index("Health"), Some(1));
assert_eq!(df.get_column_index("Mana"), Some(2));
assert_eq!(df.get_column_index("Strength"), Some(3));
assert_eq!(df.get_column_index("Haste"), None);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.try_get_column_index" class="fn">try_get_column_index</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get column index of a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series") by name.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.column" class="fn">column</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Select a single column by name.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-27" class="doc-anchor">§</a>Example

``` rust
let s1 = Column::new("Password".into(), ["123456", "[]B$u$g$s$B#u#n#n#y[]{}"]);
let s2 = Column::new("Robustness".into(), ["Weak", "Strong"]);
let df: DataFrame = DataFrame::new(vec![s1.clone(), s2])?;

assert_eq!(df.column("Password")?, &s1);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.columns" class="fn">columns</a>\<I, S\>(&self, names: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

Selected multiple columns by name.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-28" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Latin name" => ["Oncorhynchus kisutch", "Salmo salar"],
                        "Max weight (kg)" => [16.0, 35.89])?;
let sv: Vec<&Column> = df.columns(["Latin name", "Max weight (kg)"])?;

assert_eq!(&df[0], sv[0]);
assert_eq!(&df[1], sv[1]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.select" class="fn">select</a>\<I, S\>(&self, selection: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Select column(s) from this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") and return a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
fn example(df: &DataFrame) -> PolarsResult<DataFrame> {
    df.select(["foo", "bar"])
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._select_impl" class="fn">_select_impl</a>( &self, cols: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._select_impl_unchecked" class="fn">_select_impl_unchecked</a>( &self, cols: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.select_with_schema" class="fn">select_with_schema</a>\<I, S\>( &self, selection: I, schema: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Select with a known schema. The schema names must match the column names of this DataFrame.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.select_with_schema_unchecked" class="fn">select_with_schema_unchecked</a>\<I, S\>( &self, selection: I, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Select with a known schema without checking for duplicates in `selection`. The schema names must match the column names of this DataFrame.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._select_with_schema_impl" class="fn">_select_with_schema_impl</a>( &self, cols: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\], schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, check_duplicates: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

- The schema names must match the column names of this DataFrame.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.select_physical" class="fn">select_physical</a>\<I, S\>( &self, selection: I, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.project" class="fn">project</a>( &self, to: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.select_columns" class="fn">select_columns</a>( &self, selection: impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Select column(s) from this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") and return them into a [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-29" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Name" => ["Methane", "Ethane", "Propane"],
                        "Carbon" => [1, 2, 3],
                        "Hydrogen" => [4, 6, 8])?;
let sv: Vec<Column> = df.select_columns(["Carbon", "Hydrogen"])?;

assert_eq!(df["Carbon"], sv[0]);
assert_eq!(df["Hydrogen"], sv[1]);
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.filter" class="fn">filter</a>( &self, mask: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") rows by a boolean mask.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-30" class="doc-anchor">§</a>Example

``` rust
fn example(df: &DataFrame) -> PolarsResult<DataFrame> {
    let mask = df.column("sepal_width")?.is_not_null();
    df.filter(&mask)
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._filter_seq" class="fn">_filter_seq</a>( &self, mask: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Same as `filter` but does not parallelize.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.take" class="fn">take</a>( &self, indices: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") rows by index values.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-31" class="doc-anchor">§</a>Example

``` rust
fn example(df: &DataFrame) -> PolarsResult<DataFrame> {
    let idx = IdxCa::new("idx".into(), [0, 1, 9]);
    df.take(&idx)
}
```

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.take_unchecked" class="fn">take_unchecked</a>(&self, idx: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-10" class="doc-anchor">§</a>Safety

The indices must be in-bounds.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.take_unchecked_impl" class="fn">take_unchecked_impl</a>( &self, idx: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>, allow_threads: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-11" class="doc-anchor">§</a>Safety

The indices must be in-bounds.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.take_slice_unchecked" class="fn">take_slice_unchecked</a>(&self, idx: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\]) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-12" class="doc-anchor">§</a>Safety

The indices must be in-bounds.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.take_slice_unchecked_impl" class="fn">take_slice_unchecked_impl</a>( &self, idx: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], allow_threads: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-13" class="doc-anchor">§</a>Safety

The indices must be in-bounds.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.rename" class="fn">rename</a>( &mut self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Rename a column in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-32" class="doc-anchor">§</a>Example

``` rust
fn example(df: &mut DataFrame) -> PolarsResult<&mut DataFrame> {
    let original_name = "foo";
    let new_name = "bar";
    df.rename(original_name, new_name.into())
}
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.sort_in_place" class="fn">sort_in_place</a>( &mut self, by: impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, sort_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Sort [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") in place.

See [`DataFrame::sort`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.sort "method polars::prelude::DataFrame::sort") for more instruction.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._to_metadata" class="fn">_to_metadata</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Create a `DataFrame` that has fields for all the known runtime metadata for each column.

This dataframe does not necessarily have a specified schema and may be changed at any point. It is primarily used for debugging.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.sort" class="fn">sort</a>( &self, by: impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, sort_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html" class="struct" title="struct polars::prelude::SortMultipleOptions">SortMultipleOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Return a sorted clone of this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

In many cases the output chunks will be continuous in memory but this is not guaranteed

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-33" class="doc-anchor">§</a>Example

Sort by a single column with default options:

``` rust
fn sort_by_sepal_width(df: &DataFrame) -> PolarsResult<DataFrame> {
    df.sort(["sepal_width"], Default::default())
}
```

Sort by a single column with specific order:

``` rust
fn sort_with_specific_order(df: &DataFrame, descending: bool) -> PolarsResult<DataFrame> {
    df.sort(
        ["sepal_width"],
        SortMultipleOptions::new()
            .with_order_descending(descending)
    )
}
```

Sort by multiple columns with specifying order for each column:

``` rust
fn sort_by_multiple_columns_with_specific_order(df: &DataFrame) -> PolarsResult<DataFrame> {
    df.sort(
        ["sepal_width", "sepal_length"],
        SortMultipleOptions::new()
            .with_order_descending_multi([false, true])
    )
}
```

See [`SortMultipleOptions`](https://docs.rs/polars/latest/polars/prelude/struct.SortMultipleOptions.html "struct polars::prelude::SortMultipleOptions") for more options.

Also see [`DataFrame::sort_in_place`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.sort_in_place "method polars::prelude::DataFrame::sort_in_place").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.replace" class="fn">replace</a>\<S\>( &mut self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, new_col: S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where S: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a>,

Replace a column with a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-34" class="doc-anchor">§</a>Example

``` rust
let mut df: DataFrame = df!("Country" => ["United States", "China"],
                        "Area (km²)" => [9_833_520, 9_596_961])?;
let s: Series = Series::new("Country".into(), ["USA", "PRC"]);

assert!(df.replace("Nation", s.clone()).is_err());
assert!(df.replace("Country", s).is_ok());
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.replace_or_add" class="fn">replace_or_add</a>\<S\>( &mut self, column: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, new_col: S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where S: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoSeries.html" class="trait" title="trait polars::prelude::IntoSeries">IntoSeries</a>,

Replace or update a column. The difference between this method and [DataFrame::with_column](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.with_column "method polars::prelude::DataFrame::with_column") is that now the value of `column: &str` determines the name of the column and not the name of the `Series` passed to this method.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.replace_column" class="fn">replace_column</a>\<C\>( &mut self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, new_column: C, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where C: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a>,

Replace column at index `idx` with a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-35" class="doc-anchor">§</a>Example

``` ignored
# use polars_core::prelude::*;
let s0 = Series::new("foo".into(), ["ham", "spam", "egg"]);
let s1 = Series::new("ascii".into(), [70, 79, 79]);
let mut df = DataFrame::new(vec![s0, s1])?;

// Add 32 to get lowercase ascii values
df.replace_column(1, df.select_at_idx(1).unwrap() + 32);
# Ok::<(), PolarsError>(())
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.apply" class="fn">apply</a>\<F, C\>( &mut self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> C, C: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a>,

Apply a closure to a column. This is the recommended way to do in place modification.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-36" class="doc-anchor">§</a>Example

``` rust
let s0 = Column::new("foo".into(), ["ham", "spam", "egg"]);
let s1 = Column::new("names".into(), ["Jean", "Claude", "van"]);
let mut df = DataFrame::new(vec![s0, s1])?;

fn str_to_len(str_val: &Column) -> Column {
    str_val.str()
        .unwrap()
        .into_iter()
        .map(|opt_name: Option<&str>| {
            opt_name.map(|name: &str| name.len() as u32)
         })
        .collect::<UInt32Chunked>()
        .into_column()
}

// Replace the names column by the length of the names.
df.apply("names", str_to_len);
```

Results in:

``` text
+--------+-------+
| foo    |       |
| ---    | names |
| str    | u32   |
+========+=======+
| "ham"  | 4     |
+--------+-------+
| "spam" | 6     |
+--------+-------+
| "egg"  | 3     |
+--------+-------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.apply_at_idx" class="fn">apply_at_idx</a>\<F, C\>( &mut self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> C, C: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a>,

Apply a closure to a column at index `idx`. This is the recommended way to do in place modification.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-37" class="doc-anchor">§</a>Example

``` rust
let s0 = Column::new("foo".into(), ["ham", "spam", "egg"]);
let s1 = Column::new("ascii".into(), [70, 79, 79]);
let mut df = DataFrame::new(vec![s0, s1])?;

// Add 32 to get lowercase ascii values
df.apply_at_idx(1, |s| s + 32);
```

Results in:

``` text
+--------+-------+
| foo    | ascii |
| ---    | ---   |
| str    | i32   |
+========+=======+
| "ham"  | 102   |
+--------+-------+
| "spam" | 111   |
+--------+-------+
| "egg"  | 111   |
+--------+-------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.try_apply_at_idx" class="fn">try_apply_at_idx</a>\<F, C\>( &mut self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<C, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>, C: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a>,

Apply a closure that may fail to a column at index `idx`. This is the recommended way to do in place modification.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-38" class="doc-anchor">§</a>Example

This is the idiomatic way to replace some values a column of a `DataFrame` given range of indexes.

``` rust
let s0 = Column::new("foo".into(), ["ham", "spam", "egg", "bacon", "quack"]);
let s1 = Column::new("values".into(), [1, 2, 3, 4, 5]);
let mut df = DataFrame::new(vec![s0, s1])?;

let idx = vec![0, 1, 4];

df.try_apply("foo", |c| {
    c.str()?
    .scatter_with(idx, |opt_val| opt_val.map(|string| format!("{}-is-modified", string)))
});
```

Results in:

``` text
+---------------------+--------+
| foo                 | values |
| ---                 | ---    |
| str                 | i32    |
+=====================+========+
| "ham-is-modified"   | 1      |
+---------------------+--------+
| "spam-is-modified"  | 2      |
+---------------------+--------+
| "egg"               | 3      |
+---------------------+--------+
| "bacon"             | 4      |
+---------------------+--------+
| "quack-is-modified" | 5      |
+---------------------+--------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.try_apply" class="fn">try_apply</a>\<F, C\>( &mut self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<C, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>, C: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoColumn.html" class="trait" title="trait polars::prelude::IntoColumn">IntoColumn</a>,

Apply a closure that may fail to a column. This is the recommended way to do in place modification.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-39" class="doc-anchor">§</a>Example

This is the idiomatic way to replace some values a column of a `DataFrame` given a boolean mask.

``` rust
let s0 = Column::new("foo".into(), ["ham", "spam", "egg", "bacon", "quack"]);
let s1 = Column::new("values".into(), [1, 2, 3, 4, 5]);
let mut df = DataFrame::new(vec![s0, s1])?;

// create a mask
let values = df.column("values")?.as_materialized_series();
let mask = values.lt_eq(1)? | values.gt_eq(5_i32)?;

df.try_apply("foo", |c| {
    c.str()?
    .set(&mask, Some("not_within_bounds"))
});
```

Results in:

``` text
+---------------------+--------+
| foo                 | values |
| ---                 | ---    |
| str                 | i32    |
+=====================+========+
| "not_within_bounds" | 1      |
+---------------------+--------+
| "spam"              | 2      |
+---------------------+--------+
| "egg"               | 3      |
+---------------------+--------+
| "bacon"             | 4      |
+---------------------+--------+
| "not_within_bounds" | 5      |
+---------------------+--------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Slice the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") along the rows.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-40" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Fruit" => ["Apple", "Grape", "Grape", "Fig", "Fig"],
                        "Color" => ["Green", "Red", "White", "White", "Red"])?;
let sl: DataFrame = df.slice(2, 3);

assert_eq!(sl.shape(), (3, 2));
println!("{}", sl);
```

Output:

``` text
shape: (3, 2)
+-------+-------+
| Fruit | Color |
| ---   | ---   |
| str   | str   |
+=======+=======+
| Grape | White |
+-------+-------+
| Fig   | White |
+-------+-------+
| Fig   | Red   |
+-------+-------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.split_at" class="fn">split_at</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> (<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>)

Split [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") at the given `offset`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.clear" class="fn">clear</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.slice_par" class="fn">slice_par</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._slice_and_realloc" class="fn">_slice_and_realloc</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, length: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.head" class="fn">head</a>(&self, length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Get the head of the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-41" class="doc-anchor">§</a>Example

``` rust
let countries: DataFrame =
    df!("Rank by GDP (2021)" => [1, 2, 3, 4, 5],
        "Continent" => ["North America", "Asia", "Asia", "Europe", "Europe"],
        "Country" => ["United States", "China", "Japan", "Germany", "United Kingdom"],
        "Capital" => ["Washington", "Beijing", "Tokyo", "Berlin", "London"])?;
assert_eq!(countries.shape(), (5, 4));

println!("{}", countries.head(Some(3)));
```

Output:

``` text
shape: (3, 4)
+--------------------+---------------+---------------+------------+
| Rank by GDP (2021) | Continent     | Country       | Capital    |
| ---                | ---           | ---           | ---        |
| i32                | str           | str           | str        |
+====================+===============+===============+============+
| 1                  | North America | United States | Washington |
+--------------------+---------------+---------------+------------+
| 2                  | Asia          | China         | Beijing    |
+--------------------+---------------+---------------+------------+
| 3                  | Asia          | Japan         | Tokyo      |
+--------------------+---------------+---------------+------------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.tail" class="fn">tail</a>(&self, length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Get the tail of the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-42" class="doc-anchor">§</a>Example

``` rust
let countries: DataFrame =
    df!("Rank (2021)" => [105, 106, 107, 108, 109],
        "Apple Price (€/kg)" => [0.75, 0.70, 0.70, 0.65, 0.52],
        "Country" => ["Kosovo", "Moldova", "North Macedonia", "Syria", "Turkey"])?;
assert_eq!(countries.shape(), (5, 3));

println!("{}", countries.tail(Some(2)));
```

Output:

``` text
shape: (2, 3)
+-------------+--------------------+---------+
| Rank (2021) | Apple Price (€/kg) | Country |
| ---         | ---                | ---     |
| i32         | f64                | str     |
+=============+====================+=========+
| 108         | 0.63               | Syria   |
+-------------+--------------------+---------+
| 109         | 0.63               | Turkey  |
+-------------+--------------------+---------+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.iter_chunks" class="fn">iter_chunks</a>( &self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>, parallel: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/frame/struct.RecordBatchIter.html" class="struct" title="struct polars::frame::RecordBatchIter">RecordBatchIter</a>\<'\_\> <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#" class="tooltip" data-notable-ty="RecordBatchIter&lt;&#39;_&gt;">ⓘ</a>

Iterator over the rows in this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") as Arrow RecordBatches.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#panics-3" class="doc-anchor">§</a>Panics

Panics if the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") that is passed is not rechunked.

This responsibility is left to the caller as we don’t want to take mutable references here, but we also don’t want to rechunk here, as this operation is costly and would benefit the caller as well.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.iter_chunks_physical" class="fn">iter_chunks_physical</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/frame/struct.PhysRecordBatchIter.html" class="struct" title="struct polars::frame::PhysRecordBatchIter">PhysRecordBatchIter</a>\<'\_\> <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#" class="tooltip" data-notable-ty="PhysRecordBatchIter&lt;&#39;_&gt;">ⓘ</a>

Iterator over the rows in this [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") as Arrow RecordBatches as physical values.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#panics-4" class="doc-anchor">§</a>Panics

Panics if the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") that is passed is not rechunked.

This responsibility is left to the caller as we don’t want to take mutable references here, but we also don’t want to rechunk here, as this operation is costly and would benefit the caller as well.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.reverse" class="fn">reverse</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Get a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") with all the columns in reversed order.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.shift" class="fn">shift</a>(&self, periods: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Shift the values by a given period and fill the parts that will be empty due to this operation with `Nones`.

See the method on [Series](https://docs.rs/polars/latest/polars/prelude/trait.SeriesTrait.html#tymethod.shift "method polars::prelude::SeriesTrait::shift") for more info on the `shift` operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.fill_null" class="fn">fill_null</a>( &self, strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.FillNullStrategy.html" class="enum" title="enum polars::prelude::FillNullStrategy">FillNullStrategy</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Replace None values with one of the following strategies:

- Forward fill (replace None with the previous value)
- Backward fill (replace None with the next value)
- Mean fill (replace None with the mean of the whole array)
- Min fill (replace None with the minimum of the whole array)
- Max fill (replace None with the maximum of the whole array)

See the method on [Series](https://docs.rs/polars/latest/polars/prelude/struct.Series.html#method.fill_null "method polars::prelude::Series::fill_null") for more info on the `fill_null` operation.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.pipe" class="fn">pipe</a>\<F, B\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<B, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<B, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

Pipe different functions/ closure operations that work on a DataFrame together.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.pipe_mut" class="fn">pipe_mut</a>\<F, B\>(&mut self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<B, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<B, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

Pipe different functions/ closure operations that work on a DataFrame together.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.pipe_with_args" class="fn">pipe_with_args</a>\<F, B, Args\>( self, f: F, args: Args, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<B, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, Args) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<B, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

Pipe different functions/ closure operations that work on a DataFrame together.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.unique_stable" class="fn">unique_stable</a>( &self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\>, keep: <a href="https://docs.rs/polars/latest/polars/prelude/enum.UniqueKeepStrategy.html" class="enum" title="enum polars::prelude::UniqueKeepStrategy">UniqueKeepStrategy</a>, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Drop duplicate rows from a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame"). *This fails when there is a column of type List in DataFrame*

Stable means that the order is maintained. This has a higher cost than an unstable distinct.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-43" class="doc-anchor">§</a>Example

``` rust
let df = df! {
              "flt" => [1., 1., 2., 2., 3., 3.],
              "int" => [1, 1, 2, 2, 3, 3, ],
              "str" => ["a", "a", "b", "b", "c", "c"]
          }?;

println!("{}", df.unique_stable(None, UniqueKeepStrategy::First, None)?);
```

Returns

``` text
+-----+-----+-----+
| flt | int | str |
| --- | --- | --- |
| f64 | i32 | str |
+=====+=====+=====+
| 1   | 1   | "a" |
+-----+-----+-----+
| 2   | 2   | "b" |
+-----+-----+-----+
| 3   | 3   | "c" |
+-----+-----+-----+
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.unique" class="fn">unique</a>\<I, S\>( &self, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\]\>, keep: <a href="https://docs.rs/polars/latest/polars/prelude/enum.UniqueKeepStrategy.html" class="enum" title="enum polars::prelude::UniqueKeepStrategy">UniqueKeepStrategy</a>, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Unstable distinct. See [`DataFrame::unique_stable`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html#method.unique_stable "method polars::prelude::DataFrame::unique_stable").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.unique_impl" class="fn">unique_impl</a>( &self, maintain_order: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, subset: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, keep: <a href="https://docs.rs/polars/latest/polars/prelude/enum.UniqueKeepStrategy.html" class="enum" title="enum polars::prelude::UniqueKeepStrategy">UniqueKeepStrategy</a>, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.is_unique" class="fn">is_unique</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a mask of all the unique rows in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-44" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Company" => ["Apple", "Microsoft"],
                        "ISIN" => ["US0378331005", "US5949181045"])?;
let ca: ChunkedArray<BooleanType> = df.is_unique()?;

assert!(ca.all());
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.is_duplicated" class="fn">is_duplicated</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkedArray.html" class="struct" title="struct polars::prelude::ChunkedArray">ChunkedArray</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanType.html" class="struct" title="struct polars::prelude::BooleanType">BooleanType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get a mask of all the duplicated rows in the [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame").

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-45" class="doc-anchor">§</a>Example

``` rust
let df: DataFrame = df!("Company" => ["Alphabet", "Alphabet"],
                        "ISIN" => ["US02079K3059", "US02079K1079"])?;
let ca: ChunkedArray<BooleanType> = df.is_duplicated()?;

assert!(!ca.all());
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Create a new [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") that shows the null counts per column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.get_supertype" class="fn">get_supertype</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>\>

Get the supertype of the columns in this DataFrame

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.partition_by" class="fn">partition_by</a>\<I, S\>( &self, cols: I, include_key: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Split into multiple DataFrames partitioned by groups

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.partition_by_stable" class="fn">partition_by_stable</a>\<I, S\>( &self, cols: I, include_key: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Split into multiple DataFrames partitioned by groups Order of the groups are maintained.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.unnest" class="fn">unnest</a>\<I\>(&self, cols: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Unnest the given `Struct` columns. This means that the fields of the `Struct` type will be inserted as columns.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.append_record_batch" class="fn">append_record_batch</a>( &mut self, rb: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html" class="struct" title="struct polars_arrow::record_batch::RecordBatchT">RecordBatchT</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-11" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.serialize_into_writer" class="fn">serialize_into_writer</a>( &mut self, writer: &mut dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.serialize_to_bytes" class="fn">serialize_to_bytes</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.deserialize_from_reader" class="fn">deserialize_from_reader</a>( reader: &mut dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrame-12" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.schema_equal" class="fn">schema_equal</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Check if [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")’ schemas are equal.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.equals" class="fn">equals</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")s are equal. Note that `None == None` evaluates to `false`

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-46" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Atomic number" => &[1, 51, 300],
                        "Element" => &[Some("Hydrogen"), Some("Antimony"), None])?;
let df2: DataFrame = df!("Atomic number" => &[1, 51, 300],
                        "Element" => &[Some("Hydrogen"), Some("Antimony"), None])?;

assert!(!df1.equals(&df2));
```

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.equals_missing" class="fn">equals_missing</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if all values in [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame")s are equal where `None == None` evaluates to `true`.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#example-47" class="doc-anchor">§</a>Example

``` rust
let df1: DataFrame = df!("Atomic number" => &[1, 51, 300],
                        "Element" => &[Some("Hydrogen"), Some("Antimony"), None])?;
let df2: DataFrame = df!("Atomic number" => &[1, 51, 300],
                        "Element" => &[Some("Hydrogen"), Some("Antimony"), None])?;

assert!(df1.equals_missing(&df2));
```

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-AsofJoinBy-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoinBy.html" class="trait" title="trait polars::prelude::AsofJoinBy">AsofJoinBy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.join_asof_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoinBy.html#method.join_asof_by" class="fn">join_asof_by</a>\<I, S\>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, right_on: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, left_by: I, right_by: I, strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.AsofStrategy.html" class="enum" title="enum polars::prelude::AsofStrategy">AsofStrategy</a>, tolerance: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'static\>\>, allow_eq: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, check_sortedness: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = S\>, S: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>,

This is similar to a left-join except that we match on nearest key rather than equal keys. The keys must be sorted to perform an asof join. This is a special implementation of an asof join that searches for the nearest keys within a subgroup set by `by`.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Clone-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Container-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html" class="trait" title="trait polars_core::utils::Container">Container</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.slice-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.slice" class="fn">slice</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.split_at-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.split_at" class="fn">split_at</a>(&self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> (<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.iter_chunks-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.iter_chunks" class="fn">iter_chunks</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.should_rechunk-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.should_rechunk" class="fn">should_rechunk</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.n_chunks" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.n_chunks" class="fn">n_chunks</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.chunk_lengths" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/utils/trait.Container.html#tymethod.chunk_lengths" class="fn">chunk_lengths</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-CrossJoin-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.CrossJoin.html" class="trait" title="trait polars::prelude::CrossJoin">CrossJoin</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.cross_join" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.CrossJoin.html#method.cross_join" class="fn">cross_join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, suffix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Creates the Cartesian product from both frames, preserves the order of the left keys.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-DataFrameJoinOps-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html" class="trait" title="trait polars::prelude::DataFrameJoinOps">DataFrameJoinOps</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.join" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.join" class="fn">join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, right_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinArgs.html" class="struct" title="struct polars::prelude::JoinArgs">JoinArgs</a>, options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.JoinTypeOptions.html" class="enum" title="enum polars::prelude::JoinTypeOptions">JoinTypeOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Generic join method. Can be used to join on multiple columns. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.join)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.inner_join" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.inner_join" class="fn">inner_join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, right_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Perform an inner join on two DataFrames. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.inner_join)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.left_join" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.left_join" class="fn">left_join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, right_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Perform a left outer join on two DataFrames [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.left_join)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.full_join" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.full_join" class="fn">full_join</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, left_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, right_on: impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Perform a full outer join on two DataFrames [Read more](https://docs.rs/polars/latest/polars/prelude/trait.DataFrameJoinOps.html#method.full_join)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Debug-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Default-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Deserialize%3C&#39;de%3E-for-DataFrame" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>( deserializer: D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, \<D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Display-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-From%3CDataFrame%3E-for-Vec%3CColumn%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>\> for <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(df: <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-From%3CRecordBatchT%3CBox%3Cdyn+Array%3E%3E%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html" class="struct" title="struct polars_arrow::record_batch::RecordBatchT">RecordBatchT</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(rb: <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html" class="struct" title="struct polars_arrow::record_batch::RecordBatchT">RecordBatchT</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Converts to this type from the input type.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-FromIterator%3CColumn%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.from_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iter: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>,

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#panics-6" class="doc-anchor">§</a>Panics

Panics if Column have different lengths.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-FromIterator%3CSeries%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T\>(iter: T) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>,

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#panics-5" class="doc-anchor">§</a>Panics

Panics if Series have different lengths.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Index%3C%26str%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Output-7" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

The returned type after indexing.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.index-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, index: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Index%3CRange%3Cusize%3E%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]

The returned type after indexing.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.index-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range">Range</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Index%3CRangeFrom%3Cusize%3E%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html" class="struct" title="struct core::ops::range::RangeFrom">RangeFrom</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]

The returned type after indexing.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.index-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html" class="struct" title="struct core::ops::range::RangeFrom">RangeFrom</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html" class="struct" title="struct core::ops::range::RangeFrom">RangeFrom</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Index%3CRangeFull%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html" class="struct" title="struct core::ops::range::RangeFull">RangeFull</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Output-6" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]

The returned type after indexing.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.index-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html" class="struct" title="struct core::ops::range::RangeFull">RangeFull</a>) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html" class="struct" title="struct core::ops::range::RangeFull">RangeFull</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Index%3CRangeInclusive%3Cusize%3E%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html" class="struct" title="struct core::ops::range::RangeInclusive">RangeInclusive</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]

The returned type after indexing.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.index-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html" class="struct" title="struct core::ops::range::RangeInclusive">RangeInclusive</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html" class="struct" title="struct core::ops::range::RangeInclusive">RangeInclusive</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Index%3CRangeTo%3Cusize%3E%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html" class="struct" title="struct core::ops::range::RangeTo">RangeTo</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]

The returned type after indexing.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.index-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html" class="struct" title="struct core::ops::range::RangeTo">RangeTo</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html" class="struct" title="struct core::ops::range::RangeTo">RangeTo</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Index%3CRangeToInclusive%3Cusize%3E%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html" class="struct" title="struct core::ops::range::RangeToInclusive">RangeToInclusive</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = \[<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\]

The returned type after indexing.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.index-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>( &self, index: <a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html" class="struct" title="struct core::ops::range::RangeToInclusive">RangeToInclusive</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html" class="struct" title="struct core::ops::range::RangeToInclusive">RangeToInclusive</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Index%3Cusize%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>

The returned type after indexing.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.index" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype" title="type core::ops::index::Index::Output">Output</a>

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-IntoDf-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/polars_ops/frame/trait.IntoDf.html" class="trait" title="trait polars_ops::frame::IntoDf">IntoDf</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.to_df" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-ops/0.51.0/x86_64-unknown-linux-gnu/polars_ops/frame/trait.IntoDf.html#tymethod.to_df" class="fn">to_df</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-IntoLazy-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html" class="trait" title="trait polars::prelude::IntoLazy">IntoLazy</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.lazy" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoLazy.html#tymethod.lazy" class="fn">lazy</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>

Convert the `DataFrame` into a `LazyFrame`

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-JoinDispatch-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html" class="trait" title="trait polars::prelude::JoinDispatch">JoinDispatch</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.create_left_df_chunked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method.create_left_df_chunked" class="fn">create_left_df_chunked</a>( &self, chunk_ids: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\], left_join: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, was_sliced: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method.create_left_df_chunked)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._create_left_df_from_slice" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._create_left_df_from_slice" class="fn">_create_left_df_from_slice</a>( &self, join_tuples: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], left_join: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, was_sliced: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, sorted_tuple_idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._create_left_df_from_slice)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._finish_anti_semi_join" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._finish_anti_semi_join" class="fn">_finish_anti_semi_join</a>( &self, idx: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\], slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._finish_anti_semi_join)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._semi_anti_join_from_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._semi_anti_join_from_series" class="fn">_semi_anti_join_from_series</a>( &self, s_left: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, s_right: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>, anti: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, nulls_equal: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._full_join_from_series" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.JoinDispatch.html#method._full_join_from_series" class="fn">_full_join_from_series</a>( &self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, s_left: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, s_right: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, args: <a href="https://docs.rs/polars/latest/polars/prelude/struct.JoinArgs.html" class="struct" title="struct polars::prelude::JoinArgs">JoinArgs</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-MinMaxHorizontal-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html" class="trait" title="trait polars::prelude::MinMaxHorizontal">MinMaxHorizontal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.min_horizontal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html#tymethod.min_horizontal" class="fn">min_horizontal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Aggregate the column horizontally to their min values.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.max_horizontal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.MinMaxHorizontal.html#tymethod.max_horizontal" class="fn">max_horizontal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Aggregate the column horizontally to their max values.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-PartialEq-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-PolarsTemporalGroupby-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html" class="trait" title="trait polars::prelude::PolarsTemporalGroupby">PolarsTemporalGroupby</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.rolling" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html#tymethod.rolling" class="fn">rolling</a>( &self, group_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\]\>\>, options: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.RollingGroupOptions.html" class="struct" title="struct polars::prelude::RollingGroupOptions">RollingGroupOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.group_by_dynamic" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsTemporalGroupby.html#tymethod.group_by_dynamic" class="fn">group_by_dynamic</a>( &self, group_by: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\]\>\>, options: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.DynamicGroupOptions.html" class="struct" title="struct polars::prelude::DynamicGroupOptions">DynamicGroupOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupPositions.html" class="struct" title="struct polars::prelude::GroupPositions">GroupPositions</a>), <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-PolarsUpsample-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html" class="trait" title="trait polars::prelude::PolarsUpsample">PolarsUpsample</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.upsample" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#tymethod.upsample" class="fn">upsample</a>\<I\>( &self, by: I, time_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, every: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration">Duration</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Upsample a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") at a regular frequency. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#tymethod.upsample)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.upsample_stable" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#tymethod.upsample_stable" class="fn">upsample_stable</a>\<I\>( &self, by: I, time_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, every: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Duration.html" class="struct" title="struct polars::prelude::Duration">Duration</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://docs.rs/polars/latest/polars/prelude/trait.IntoVec.html" class="trait" title="trait polars::prelude::IntoVec">IntoVec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>,

Upsample a [`DataFrame`](https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html "struct polars::prelude::DataFrame") at a regular frequency. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.PolarsUpsample.html#tymethod.upsample_stable)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-Serialize-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>( &self, serializer: S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-SumMeanHorizontal-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html" class="trait" title="trait polars::prelude::SumMeanHorizontal">SumMeanHorizontal</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.sum_horizontal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html#tymethod.sum_horizontal" class="fn">sum_horizontal</a>( &self, null_strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.NullStrategy.html" class="enum" title="enum polars::prelude::NullStrategy">NullStrategy</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Sum all values horizontally across columns.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.mean_horizontal" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SumMeanHorizontal.html#tymethod.mean_horizontal" class="fn">mean_horizontal</a>( &self, null_strategy: <a href="https://docs.rs/polars/latest/polars/prelude/enum.NullStrategy.html" class="enum" title="enum polars::prelude::NullStrategy">NullStrategy</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Column.html" class="enum" title="enum polars::prelude::Column">Column</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Compute the mean of all numeric values horizontally across columns.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-TakeChunked-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html" class="trait" title="trait polars::prelude::TakeChunked">TakeChunked</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.take_chunked_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#tymethod.take_chunked_unchecked" class="fn">take_chunked_unchecked</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, idx: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], sorted: <a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>, avoid_sharing: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Take elements by a slice of [`ChunkId`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html "struct polars::prelude::ChunkId")s.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-14" class="doc-anchor">§</a>Safety

Does not do any bound checks. `sorted` indicates if the chunks are sorted.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.take_opt_chunked_unchecked" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunked.html#tymethod.take_opt_chunked_unchecked" class="fn">take_opt_chunked_unchecked</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, idx: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], avoid_sharing: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Take elements by a slice of optional [`ChunkId`](https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html "struct polars::prelude::ChunkId")s.

##### <a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#safety-15" class="doc-anchor">§</a>Safety

Does not do any bound checks.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-TakeChunkedHorPar-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html" class="trait" title="trait polars::prelude::TakeChunkedHorPar">TakeChunkedHorPar</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._take_chunked_unchecked_hor_par" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#method._take_chunked_unchecked_hor_par" class="fn">_take_chunked_unchecked_hor_par</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, idx: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], sorted: <a href="https://docs.rs/polars/latest/polars/series/enum.IsSorted.html" class="enum" title="enum polars::series::IsSorted">IsSorted</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#method._take_chunked_unchecked_hor_par)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method._take_opt_chunked_unchecked_hor_par" class="anchor">§</a>

#### unsafe fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#method._take_opt_chunked_unchecked_hor_par" class="fn">_take_opt_chunked_unchecked_hor_par</a>\<const B: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>( &self, idx: &\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.ChunkId.html" class="struct" title="struct polars::prelude::ChunkId">ChunkId</a>\<B\>\], ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

Safety [Read more](https://docs.rs/polars/latest/polars/prelude/trait.TakeChunkedHorPar.html#method._take_opt_chunked_unchecked_hor_par)

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-TryExtend%3CRecordBatchT%3CBox%3Cdyn+Array%3E%3E%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.TryExtend.html" class="trait" title="trait polars_arrow::array::TryExtend">TryExtend</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html" class="struct" title="struct polars_arrow::record_batch::RecordBatchT">RecordBatchT</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.try_extend" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.TryExtend.html#tymethod.try_extend" class="fn">try_extend</a>\<I\>(&mut self, iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html" class="struct" title="struct polars_arrow::record_batch::RecordBatchT">RecordBatchT</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>\>,

Fallible version of [`Extend::extend`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend "method core::iter::traits::collect::Extend::extend").

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-TryExtend%3CResult%3CRecordBatchT%3CBox%3Cdyn+Array%3E%3E,+PolarsError%3E%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.TryExtend.html" class="trait" title="trait polars_arrow::array::TryExtend">TryExtend</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html" class="struct" title="struct polars_arrow::record_batch::RecordBatchT">RecordBatchT</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.try_extend-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.TryExtend.html#tymethod.try_extend" class="fn">try_extend</a>\<I\>(&mut self, iter: I) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/record_batch/struct.RecordBatchT.html" class="struct" title="struct polars_arrow::record_batch::RecordBatchT">RecordBatchT</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/array/trait.Array.html" class="trait" title="trait polars_arrow::array::Array">Array</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>\>,

Fallible version of [`Extend::extend`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend "method core::iter::traits::collect::Extend::extend").

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-TryFrom%3CStructArray%3E-for-DataFrame" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>(arr: <a href="https://docs.rs/polars/latest/polars/prelude/struct.StructArray.html" class="struct" title="struct polars::prelude::StructArray">StructArray</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Performs the conversion.

<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#impl-AsofJoin-for-DataFrame" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.AsofJoin.html" class="trait" title="trait polars::prelude::AsofJoin">AsofJoin</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#blanket-implementations" class="anchor">§</a>
