# Module test_util Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/mod.rs.html#18-283" class="src">Source</a>

Expand description

Utility functions to make testing DataFusion based crates easier

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/csv/index.html" class="mod" title="mod datafusion::test_util::csv">csv</a>  
Helpers for writing csv files and reading them back

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/parquet/index.html" class="mod" title="mod datafusion::test_util::parquet">parquet</a>`parquet`  
Helpers for writing parquet files and reading them back

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableFactory.html" class="struct" title="struct datafusion::test_util::TestTableFactory">TestTableFactory</a>  
TableFactory for tests

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/struct.TestTableProvider.html" class="struct" title="struct datafusion::test_util::TestTableProvider">TestTableProvider</a>  
TableProvider for testing purposes

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.aggr_test_schema.html" class="fn" title="fn datafusion::test_util::aggr_test_schema">aggr_test_schema</a>  
Get the schema for the aggregate_test\_\* csv files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.arrow_test_data.html" class="fn" title="fn datafusion::test_util::arrow_test_data">arrow_test_data</a>  
Returns the arrow test data directory, which is by default stored in a git submodule rooted at `testing/data`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.bounded_stream.html" class="fn" title="fn datafusion::test_util::bounded_stream">bounded_stream</a>  
Creates a bounded stream that emits the same record batch a specified number of times. This is useful for testing purposes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.get_data_dir.html" class="fn" title="fn datafusion::test_util::get_data_dir">get_data_dir</a>  
Returns a directory path for finding test data.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.parquet_test_data.html" class="fn" title="fn datafusion::test_util::parquet_test_data">parquet_test_data</a>`parquet`  
Returns the parquet test data directory, which is by default stored in a git submodule rooted at `parquet-testing/data`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.plan_and_collect.html" class="fn" title="fn datafusion::test_util::plan_and_collect">plan_and_collect</a>  
Execute SQL and return results

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.populate_csv_partitions.html" class="fn" title="fn datafusion::test_util::populate_csv_partitions">populate_csv_partitions</a>  
Generate CSV partitions within the supplied directory

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.register_aggregate_csv.html" class="fn" title="fn datafusion::test_util::register_aggregate_csv">register_aggregate_csv</a>  
Register session context for the aggregate_test_100.csv file

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.register_unbounded_file_with_ordering.html" class="fn" title="fn datafusion::test_util::register_unbounded_file_with_ordering">register_unbounded_file_with_ordering</a>  
This function creates an unbounded sorted file for testing purposes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.scan_empty.html" class="fn" title="fn datafusion::test_util::scan_empty">scan_empty</a>  
Scan an empty data source, mainly used in tests

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.scan_empty_with_partitions.html" class="fn" title="fn datafusion::test_util::scan_empty_with_partitions">scan_empty_with_partitions</a>  
Scan an empty data source with configured partition, mainly used in tests.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.test_table.html" class="fn" title="fn datafusion::test_util::test_table">test_table</a>  
Create a table from the aggregate_test_100.csv file with the name “aggregate_test_100”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/test_util/fn.test_table_with_name.html" class="fn" title="fn datafusion::test_util::test_table_with_name">test_table_with_name</a>  
Create a table from the aggregate_test_100.csv file with the specified name
