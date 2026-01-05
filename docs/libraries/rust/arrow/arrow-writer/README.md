Description: Encodes `RecordBatch` to parquet

Title: ArrowWriter in parquet::arrow::arrow\_writer - Rust

parquet::arrow::arrow\_writer

Struct ArrowWriterCopy item path
================================

Source

```
pub struct ArrowWriter<W: Write> {
writer: SerializedFileWriter<W>,
in_progress: Option<ArrowRowGroupWriter>,
arrow_schema: SchemaRef,
row_group_writer_factory: ArrowRowGroupWriterFactory,
max_row_group_size: usize,
}
```

Expand description

Encodes \[`RecordBatch`\] to parquet

Writes Arrow `RecordBatch`es to a Parquet writer. Multiple \[`RecordBatch`\] will be encoded to the same row group, up to `max_row_group_size` rows. Any remaining rows will be flushed on close, leading the final row group in the output file to potentially contain fewer than `max_row_group_size` rows

§Example: Writing `RecordBatch`es
---------------------------------

```
let col = Arc::new(Int64Array::from_iter_values([1, 2, 3])) as ArrayRef;
let to_write = RecordBatch::try_from_iter([("col", col)]).unwrap();

let mut buffer = Vec::new();
let mut writer = ArrowWriter::try_new(&mut buffer, to_write.schema(), None).unwrap();
writer.write(&to_write).unwrap();
writer.close().unwrap();

let mut reader = ParquetRecordBatchReader::try_new(Bytes::from(buffer), 1024).unwrap();
let read = reader.next().unwrap().unwrap();

assert_eq!(to_write, read);
```

§Memory Usage and Limiting
--------------------------

The nature of Parquet requires buffering of an entire row group before it can be flushed to the underlying writer. Data is mostly buffered in its encoded form, reducing memory usage. However, some data such as dictionary keys, large strings or very nested data may still result in non-trivial memory usage.

See Also:

*   `ArrowWriter::memory_size`: the current memory usage of the writer.
*   `ArrowWriter::in_progress_size`: Estimated size of the buffered row group,

Call `Self::flush` to trigger an early flush of a row group based on a memory threshold and/or global memory pressure. However, smaller row groups result in higher metadata overheads, and thus may worsen compression ratios and query performance.

```
writer.write(&batch).unwrap();
// Trigger an early flush if anticipated size exceeds 1_000_000
if writer.in_progress_size() > 1_000_000 {
writer.flush().unwrap();
}
```

### §Type Support

The writer supports writing all Arrow `DataType`s that have a direct mapping to Parquet types including `StructArray` and `ListArray`.

The following are not supported:

*   `IntervalMonthDayNanoArray`: Parquet does not support nanosecond intervals.

Fields§
-------

§`writer: SerializedFileWriter<W>`

Underlying Parquet writer

§`in_progress: Option<ArrowRowGroupWriter>`

The in-progress row group if any

§`arrow_schema: SchemaRef`

A copy of the Arrow schema.

The schema is used to verify that each record batch written has the correct schema

§`row_group_writer_factory: ArrowRowGroupWriterFactory`

Creates new `ArrowRowGroupWriter` instances as required

§`max_row_group_size: usize`

The length of arrays to write to each row group

Implementations§
----------------

Source§

### impl<W: Write + Send\> ArrowWriter<W>

Source

#### pub fn try\_new( writer: W, arrow\_schema: SchemaRef, props: Option<WriterProperties\>, ) -> Result<Self>

Try to create a new Arrow writer

The writer will fail if:

*   a `SerializedFileWriter` cannot be created from the ParquetWriter
*   the Arrow schema contains unsupported datatypes such as Unions

Source

#### pub fn try\_new\_with\_options( writer: W, arrow\_schema: SchemaRef, options: ArrowWriterOptions, ) -> Result<Self>

Try to create a new Arrow writer with `ArrowWriterOptions`.

The writer will fail if:

*   a `SerializedFileWriter` cannot be created from the ParquetWriter
*   the Arrow schema contains unsupported datatypes such as Unions

Source

#### pub fn flushed\_row\_groups(&self) -> &\[RowGroupMetaData\]

Returns metadata for any flushed row groups

Source

#### pub fn memory\_size(&self) -> usize

Estimated memory usage, in bytes, of this `ArrowWriter`

This estimate is formed bu summing the values of `ArrowColumnWriter::memory_size` all in progress columns.

Source

#### pub fn in\_progress\_size(&self) -> usize

Anticipated encoded size of the in progress row group.

This estimate the row group size after being completely encoded is, formed by summing the values of `ArrowColumnWriter::get_estimated_total_bytes` for all in progress columns.

Source

#### pub fn in\_progress\_rows(&self) -> usize

Returns the number of rows buffered in the in progress row group

Source

#### pub fn bytes\_written(&self) -> usize

Returns the number of bytes written by this instance

Source

#### pub fn write(&mut self, batch: &RecordBatch) -> Result<()\>

Encodes the provided \[`RecordBatch`\]

If this would cause the current row group to exceed `WriterProperties::max_row_group_size` rows, the contents of `batch` will be written to one or more row groups such that all but the final row group in the file contain `WriterProperties::max_row_group_size` rows.

This will fail if the `batch`’s schema does not match the writer’s schema.

Source

#### pub fn write\_all(&mut self, buf: &\[u8\]) -> Result<()\>

Writes the given buf bytes to the internal buffer.

It’s safe to use this method to write data to the underlying writer, because it will ensure that the buffering and byte‐counting layers are used.

Source

#### pub fn flush(&mut self) -> Result<()\>

Flushes all buffered rows into a new row group

Source

#### pub fn append\_key\_value\_metadata(&mut self, kv\_metadata: KeyValue)

Additional `KeyValue` metadata to be written in addition to those from `WriterProperties`

This method provide a way to append kv\_metadata after write RecordBatch

Source

#### pub fn inner(&self) -> &W

Returns a reference to the underlying writer.

Source

#### pub fn inner\_mut(&mut self) -> &mut W

Returns a mutable reference to the underlying writer.

**Warning**: if you write directly to this writer, you will skip the `TrackedWrite` buffering and byte‐counting layers. That’ll cause the file footer’s recorded offsets and sizes to diverge from reality, resulting in an unreadable or corrupted Parquet file.

If you want to write safely to the underlying writer, use `Self::write_all`.

Source

#### pub fn into\_inner(self) -> Result<W>

Flushes any outstanding data and returns the underlying writer.

Source

#### pub fn finish(&mut self) -> Result<FileMetaData\>

Close and finalize the underlying Parquet writer

Unlike `Self::close` this does not consume self

Attempting to write after calling finish will result in an error

Source

#### pub fn close(self) -> Result<FileMetaData\>

Close and finalize the underlying Parquet writer

Trait Implementations§
----------------------

Source§

### impl<W: Write + Send\> Debug for ArrowWriter<W>

Source§

#### fn fmt(&self, f: &mut Formatter<'\_>) -> Result

Formats the value using the given formatter. Read more

Source§

### impl<W: Write + Send\> RecordBatchWriter for ArrowWriter<W>

Source§

#### fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>

Write a single batch to the writer.

Source§

#### fn close(self) -> Result<(), ArrowError>

Write footer or termination data, then mark the writer as done.

Auto Trait Implementations§
---------------------------

§

### impl<W> Freeze for ArrowWriter<W>

where W: Freeze,

§

### impl<W> !RefUnwindSafe for ArrowWriter<W>

§

### impl<W> Send for ArrowWriter<W>

where W: Send,

§

### impl<W> !Sync for ArrowWriter<W>

§

### impl<W> Unpin for ArrowWriter<W>

where W: Unpin,

§

### impl<W> !UnwindSafe for ArrowWriter<W>

Blanket Implementations§
------------------------

Source§

### impl<T> Any for T

where T: 'static + ?Sized,

Source§

#### fn type\_id(&self) -> TypeId

Gets the `TypeId` of `self`. Read more

Source§

### impl<T> Borrow<T> for T

where T: ?Sized,

Source§

#### fn borrow(&self) -> &T

Immutably borrows from an owned value. Read more

Source§

### impl<T> BorrowMut<T> for T

where T: ?Sized,

Source§

#### fn borrow\_mut(&mut self) -> &mut T

Mutably borrows from an owned value. Read more

Source§

### impl<T> From<T> for T

Source§

#### fn from(t: T) -> T

Returns the argument unchanged.

Source§

### impl<T, U> Into<U> for T

where U: From<T>,

Source§

#### fn into(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

Source§

### impl<T> IntoEither for T

Source§

#### fn into\_either(self, into\_left: bool) -> Either<Self, Self>

Converts `self` into a `Left` variant of `Either<Self, Self>` if `into_left` is `true`. Converts `self` into a `Right` variant of `Either<Self, Self>` otherwise. Read more

Source§

#### fn into\_either\_with<F>(self, into\_left: F) -> Either<Self, Self>

where F: FnOnce(&Self) -> bool,

Converts `self` into a `Left` variant of `Either<Self, Self>` if `into_left(&self)` returns `true`. Converts `self` into a `Right` variant of `Either<Self, Self>` otherwise. Read more

Source§

### impl<T, U> TryFrom<U> for T

where U: Into<T>,

Source§

#### type Error = Infallible

The type returned in the event of a conversion error.

Source§

#### fn try\_from(value: U) -> Result<T, <T as TryFrom<U>>::Error\>

Performs the conversion.

Source§

### impl<T, U> TryInto<U> for T

where U: TryFrom<T>,

Source§

#### type Error = <U as TryFrom<T>>::Error

The type returned in the event of a conversion error.

Source§

#### fn try\_into(self) -> Result<U, <U as TryFrom<T>>::Error\>

Performs the conversion.

§

### impl<T> ErasedDestructor for T

where T: 'static,