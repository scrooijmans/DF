# Function partition Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/partition.rs.html#127" class="src">Source</a>

``` rust
pub fn partition(columns: &[Arc<dyn Array>]) -> Result<Partitions, ArrowError>
```

Expand description

Given a list of lexicographically sorted columns, computes the [`Partitions`](https://docs.rs/arrow/latest/arrow/compute/struct.Partitions.html "struct arrow::compute::Partitions"), where a partition consists of the set of consecutive rows with equal values

Returns an error if no columns are specified or all columns do not have the same number of rows.

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.partition.html#example" class="doc-anchor">§</a>Example:

For example, given columns `x`, `y` and `z`, calling [`partition`](https://docs.rs/arrow/latest/arrow/compute/fn.partition.html "fn arrow::compute::partition")`(values, (x, y))` will divide the rows into ranges where the values of `(x, y)` are equal:

``` text
┌ ─ ┬───┬ ─ ─┌───┐─ ─ ┬───┬ ─ ─ ┐
    │ 1 │    │ 1 │    │ A │        Range: 0..1 (x=1, y=1)
├ ─ ┼───┼ ─ ─├───┤─ ─ ┼───┼ ─ ─ ┤
    │ 1 │    │ 2 │    │ B │
│   ├───┤    ├───┤    ├───┤     │
    │ 1 │    │ 2 │    │ C │        Range: 1..4 (x=1, y=2)
│   ├───┤    ├───┤    ├───┤     │
    │ 1 │    │ 2 │    │ D │
├ ─ ┼───┼ ─ ─├───┤─ ─ ┼───┼ ─ ─ ┤
    │ 2 │    │ 1 │    │ E │        Range: 4..5 (x=2, y=1)
├ ─ ┼───┼ ─ ─├───┤─ ─ ┼───┼ ─ ─ ┤
    │ 3 │    │ 1 │    │ F │        Range: 5..6 (x=3, y=1)
└ ─ ┴───┴ ─ ─└───┘─ ─ ┴───┴ ─ ─ ┘

      x        y        z     partition(&[x, y])
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/fn.partition.html#example-code" class="doc-anchor">§</a>Example Code

``` rust
let batch = RecordBatch::try_from_iter(vec![
    ("x", Arc::new(Int64Array::from(vec![1, 1, 1, 1, 2, 3])) as ArrayRef),
    ("y", Arc::new(Int64Array::from(vec![1, 2, 2, 2, 1, 1])) as ArrayRef),
    ("z", Arc::new(StringArray::from(vec!["A", "B", "C", "D", "E", "F"])) as ArrayRef),
]).unwrap();

// Partition on first two columns
let ranges = partition(&batch.columns()[..2]).unwrap().ranges();

let expected = vec![
    (0..1),
    (1..4),
    (4..5),
    (5..6),
];

assert_eq!(ranges, expected);
```
