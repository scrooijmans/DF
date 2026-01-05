# Struct ArrowReaderBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/reader.rs.html#61-67" class="src">Source</a>

``` rust
pub struct ArrowReaderBuilder { /* private fields */ }
```

Expand description

Builder to create ArrowReader

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#impl-ArrowReaderBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html" class="struct" title="struct iceberg::arrow::ArrowReaderBuilder">ArrowReaderBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#method.with_data_file_concurrency_limit" class="fn">with_data_file_concurrency_limit</a>(self, val: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Sets the max number of in flight data files that are being fetched

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#method.with_batch_size" class="fn">with_batch_size</a>(self, batch_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Sets the desired size of batches in the response to something other than the default

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#method.with_row_group_filtering_enabled" class="fn">with_row_group_filtering_enabled</a>( self, row_group_filtering_enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> Self

Determines whether to enable row group filtering.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#method.with_row_selection_enabled" class="fn">with_row_selection_enabled</a>(self, row_selection_enabled: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Determines whether to enable row selection.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html" class="struct" title="struct iceberg::arrow::ArrowReader">ArrowReader</a>

Build the ArrowReader.

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html#blanket-implementations" class="anchor">§</a>
