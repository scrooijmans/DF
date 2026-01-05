# Module data_gen Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/data_gen.rs.html#18-815" class="src">Source</a>

Available on **crate feature `test_utils`** only.

Expand description

Utilities to generate random arrays and batches

## Traits<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/trait.RandomTemporalValue.html" class="trait" title="trait arrow::util::data_gen::RandomTemporalValue">RandomTemporalValue</a>  
Useful for testing. The range of values are not likely to be representative of the actual bounds.

## Functions<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/fn.create_random_array.html" class="fn" title="fn arrow::util::data_gen::create_random_array">create_random_array</a>  
Create a random [ArrayRef](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") from a [DataType](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") with a length, null density and true density (for [BooleanArray](https://docs.rs/arrow/latest/arrow/array/struct.BooleanArray.html "struct arrow::array::BooleanArray")).

<a href="https://docs.rs/arrow/latest/arrow/util/data_gen/fn.create_random_batch.html" class="fn" title="fn arrow::util::data_gen::create_random_batch">create_random_batch</a>  
Create a random [RecordBatch](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") from a schema
