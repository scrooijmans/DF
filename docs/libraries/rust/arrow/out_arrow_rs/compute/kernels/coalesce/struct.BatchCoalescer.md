# Struct BatchCoalescer Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/coalesce.rs.html#132" class="src">Source</a>

``` rust
pub struct BatchCoalescer { /* private fields */ }
```

Expand description

Concatenate multiple [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es

Implements the common pattern of incrementally creating output [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es of a specific size from an input stream of [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es.

This is useful after operations such as [`filter`](https://docs.rs/arrow/latest/arrow/compute/fn.filter.html "fn arrow::compute::filter") and [`take`](https://docs.rs/arrow/latest/arrow/compute/fn.take.html "fn arrow::compute::take") that produce smaller batches, and we want to coalesce them into larger batches for further processing.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#motivation" class="doc-anchor">§</a>Motivation

If we use [`concat_batches`](https://docs.rs/arrow/latest/arrow/compute/fn.concat_batches.html "fn arrow::compute::concat_batches") to implement the same functionality, there are 2 potential issues:

1.  At least 2x peak memory (holding the input and output of concat)
2.  2 copies of the data (to create the output of filter and then create the output of concat)

See: <https://github.com/apache/arrow-rs/issues/6692> for more discussions about the motivation.

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#example" class="doc-anchor">§</a>Example

``` rust
use arrow_array::record_batch;
use arrow_select::coalesce::{BatchCoalescer};
let batch1 = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
let batch2 = record_batch!(("a", Int32, [4, 5])).unwrap();

// Create a `BatchCoalescer` that will produce batches with at least 4 rows
let target_batch_size = 4;
let mut coalescer = BatchCoalescer::new(batch1.schema(), 4);

// push the batches
coalescer.push_batch(batch1).unwrap();
// only pushed 3 rows (not yet 4, enough to produce a batch)
assert!(coalescer.next_completed_batch().is_none());
coalescer.push_batch(batch2).unwrap();
// now we have 5 rows, so we can produce a batch
let finished = coalescer.next_completed_batch().unwrap();
// 4 rows came out (target batch size is 4)
let expected = record_batch!(("a", Int32, [1, 2, 3, 4])).unwrap();
assert_eq!(finished, expected);

// Have no more input, but still have an in-progress batch
assert!(coalescer.next_completed_batch().is_none());
// We can finish the batch, which will produce the remaining rows
coalescer.finish_buffered_batch().unwrap();
let expected = record_batch!(("a", Int32, [5])).unwrap();
assert_eq!(coalescer.next_completed_batch().unwrap(), expected);

// The coalescer is now empty
assert!(coalescer.next_completed_batch().is_none());
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#background" class="doc-anchor">§</a>Background

Generally speaking, larger [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es are more efficient to process than smaller [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es (until the CPU cache is exceeded) because there is fixed processing overhead per batch. This coalescer builds up these larger batches incrementally.

``` text
┌────────────────────┐
│    RecordBatch     │
│   num_rows = 100   │
└────────────────────┘                 ┌────────────────────┐
                                       │                    │
┌────────────────────┐     Coalesce    │                    │
│                    │      Batches    │                    │
│    RecordBatch     │                 │                    │
│   num_rows = 200   │  ─ ─ ─ ─ ─ ─ ▶  │                    │
│                    │                 │    RecordBatch     │
│                    │                 │   num_rows = 400   │
└────────────────────┘                 │                    │
                                       │                    │
┌────────────────────┐                 │                    │
│                    │                 │                    │
│    RecordBatch     │                 │                    │
│   num_rows = 100   │                 └────────────────────┘
│                    │
└────────────────────┘
```

## <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#notes" class="doc-anchor">§</a>Notes:

1.  Output rows are produced in the same order as the input rows

2.  The output is a sequence of batches, with all but the last being at exactly `target_batch_size` rows.

## Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#impl-BatchCoalescer" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html" class="struct" title="struct arrow::compute::BatchCoalescer">BatchCoalescer</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.new" class="fn">new</a>(schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>\>, target_batch_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html" class="struct" title="struct arrow::compute::BatchCoalescer">BatchCoalescer</a>

Create a new `BatchCoalescer`

##### <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#arguments" class="doc-anchor">§</a>Arguments

- `schema` - the schema of the output batches
- `target_batch_size` - the number of rows in each output batch. Typical values are `4096` or `8192` rows.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.with_biggest_coalesce_batch_size" class="fn">with_biggest_coalesce_batch_size</a>( self, limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html" class="struct" title="struct arrow::compute::BatchCoalescer">BatchCoalescer</a>

Set the coalesce batch size limit (default `None`)

This limit determine when batches should bypass coalescing. Intuitively, batches that are already large are costly to coalesce and are efficient enough to process directly without coalescing.

If `Some(limit)`, batches larger than this limit will bypass coalescing when there is no buffered data, or when the previously buffered data already exceeds this limit.

If `None`, all batches will be coalesced according to the target_batch_size.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.biggest_coalesce_batch_size" class="fn">biggest_coalesce_batch_size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Get the current biggest coalesce batch size limit

See [`Self::with_biggest_coalesce_batch_size`](https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html#method.with_biggest_coalesce_batch_size "method arrow::compute::BatchCoalescer::with_biggest_coalesce_batch_size") for details

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.set_biggest_coalesce_batch_size" class="fn">set_biggest_coalesce_batch_size</a>(&mut self, limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Set the biggest coalesce batch size limit

See [`Self::with_biggest_coalesce_batch_size`](https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html#method.with_biggest_coalesce_batch_size "method arrow::compute::BatchCoalescer::with_biggest_coalesce_batch_size") for details

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/arrow/latest/arrow/datatypes/struct.Schema.html" class="struct" title="struct arrow::datatypes::Schema">Schema</a>\>

Return the schema of the output batches

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.push_batch_with_filter" class="fn">push_batch_with_filter</a>( &mut self, batch: <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>, filter: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html" class="struct" title="struct arrow::array::BooleanArray">BooleanArray</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Push a batch into the Coalescer after applying a filter

This is semantically equivalent of calling [`Self::push_batch`](https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html#method.push_batch "method arrow::compute::BatchCoalescer::push_batch") with the results from [`filter_record_batch`](https://docs.rs/arrow/latest/arrow/compute/fn.filter_record_batch.html "fn arrow::compute::filter_record_batch")

##### <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#example-1" class="doc-anchor">§</a>Example

``` rust
let batch1 = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
let batch2 = record_batch!(("a", Int32, [4, 5, 6])).unwrap();
// Apply a filter to each batch to pick the first and last row
let filter = BooleanArray::from(vec![true, false, true]);
// create a new Coalescer that targets creating 1000 row batches
let mut coalescer = BatchCoalescer::new(batch1.schema(), 1000);
coalescer.push_batch_with_filter(batch1, &filter);
coalescer.push_batch_with_filter(batch2, &filter);
// finsh and retrieve the created batch
coalescer.finish_buffered_batch().unwrap();
let completed_batch = coalescer.next_completed_batch().unwrap();
// filtered out 2 and 5:
let expected_batch = record_batch!(("a", Int32, [1, 3, 4, 6])).unwrap();
assert_eq!(completed_batch, expected_batch);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.push_batch" class="fn">push_batch</a>(&mut self, batch: <a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Push all the rows from `batch` into the Coalescer

When buffered data plus incoming rows reach `target_batch_size` , completed batches are generated eagerly and can be retrieved via [`Self::next_completed_batch()`](https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html#method.next_completed_batch "method arrow::compute::BatchCoalescer::next_completed_batch"). Output batches contain exactly `target_batch_size` rows, so the tail of the input batch may remain buffered. Remaining partial data either waits for future input batches or can be materialized immediately by calling [`Self::finish_buffered_batch()`](https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html#method.finish_buffered_batch "method arrow::compute::BatchCoalescer::finish_buffered_batch").

##### <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#example-2" class="doc-anchor">§</a>Example

``` rust
let batch1 = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
let batch2 = record_batch!(("a", Int32, [4, 5, 6])).unwrap();
// create a new Coalescer that targets creating 1000 row batches
let mut coalescer = BatchCoalescer::new(batch1.schema(), 1000);
coalescer.push_batch(batch1);
coalescer.push_batch(batch2);
// finsh and retrieve the created batch
coalescer.finish_buffered_batch().unwrap();
let completed_batch = coalescer.next_completed_batch().unwrap();
let expected_batch = record_batch!(("a", Int32, [1, 2, 3, 4, 5, 6])).unwrap();
assert_eq!(completed_batch, expected_batch);
```

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.get_buffered_rows" class="fn">get_buffered_rows</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of buffered rows

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.finish_buffered_batch" class="fn">finish_buffered_batch</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Concatenates any buffered batches into a single `RecordBatch` and clears any output buffers

Normally this is called when the input stream is exhausted, and we want to finalize the last batch of rows.

See [`Self::next_completed_batch()`](https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html#method.next_completed_batch "method arrow::compute::BatchCoalescer::next_completed_batch") for the completed batches.

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if there is any buffered data

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.has_completed_batch" class="fn">has_completed_batch</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if there are any completed batches

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.next_completed_batch" class="fn">next_completed_batch</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>\>

Removes and returns the next completed batch, if any.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#impl-Debug-for-BatchCoalescer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/compute/struct.BatchCoalescer.html" class="struct" title="struct arrow::compute::BatchCoalescer">BatchCoalescer</a>

<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/compute/kernels/coalesce/struct.BatchCoalescer.html#blanket-implementations" class="anchor">§</a>
