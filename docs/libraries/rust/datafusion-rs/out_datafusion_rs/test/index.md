# Module test Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/mod.rs.html#18-259" class="src">Source</a>

Available on **non-WebAssembly** only.

Expand description

Common unit test utility methods

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/object_store/index.html" class="mod" title="mod datafusion::test::object_store">object_store</a>  
Object store implementation used for testing

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/variable/index.html" class="mod" title="mod datafusion::test::variable">variable</a>  
System variable provider

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/fn.assert_fields_eq.html" class="fn" title="fn datafusion::test::assert_fields_eq">assert_fields_eq</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/fn.columns.html" class="fn" title="fn datafusion::test::columns">columns</a>  
Returns the column names on the schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/fn.create_table_dual.html" class="fn" title="fn datafusion::test::create_table_dual">create_table_dual</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/fn.make_partition.html" class="fn" title="fn datafusion::test::make_partition">make_partition</a>  
Return a RecordBatch with a single Int32 array with values (0..sz)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/fn.partitioned_file_groups.html" class="fn" title="fn datafusion::test::partitioned_file_groups">partitioned_file_groups</a>  
Returns file groups [`Vec<FileGroup>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") for scanning `partitions` of `filename`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/fn.scan_partitioned_csv.html" class="fn" title="fn datafusion::test::scan_partitioned_csv">scan_partitioned_csv</a>  
Returns a [`DataSourceExec`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html "struct datafusion::datasource::memory::DataSourceExec") that scans “aggregate_test_100.csv” with `partitions` partitions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/fn.table_with_decimal.html" class="fn" title="fn datafusion::test::table_with_decimal">table_with_decimal</a>  
Return a new table which provide this decimal column

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test/fn.table_with_sequence.html" class="fn" title="fn datafusion::test::table_with_sequence">table_with_sequence</a>  
Return a new table provider that has a single Int32 column with values between `seq_start` and `seq_end`
