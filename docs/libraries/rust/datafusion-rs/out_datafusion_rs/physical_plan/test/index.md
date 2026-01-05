# Module test Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#97" class="src">Source</a>

Expand description

Utilities for testing datafusion-physical-plan

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/index.html" class="mod" title="mod datafusion::physical_plan::test::exec">exec</a>  
Simple iterator over batches for use in testing

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/struct.TestMemoryExec.html" class="struct" title="struct datafusion::physical_plan::test::TestMemoryExec">TestMemoryExec</a>

`TestMemoryExec` is a mock equivalent to [`MemorySourceConfig`](https://github.com/apache/datafusion/tree/main/datafusion/datasource/src/memory.rs) with [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") implemented for testing. i.e. It has some but not all the functionality of [`MemorySourceConfig`](https://github.com/apache/datafusion/tree/main/datafusion/datasource/src/memory.rs). This implements an in-memory DataSource rather than explicitly implementing a trait. It is implemented in this manner to keep relevant unit tests in place while avoiding circular dependencies between `datafusion-physical-plan` and `datafusion-datasource`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/struct.TestPartitionStream.html" class="struct" title="struct datafusion::physical_plan::test::TestPartitionStream">TestPartitionStream</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.aggr_test_schema.html" class="fn" title="fn datafusion::physical_plan::test::aggr_test_schema">aggr_test_schema</a>

Get the schema for the aggregate_test\_\* csv files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.assert_is_pending.html" class="fn" title="fn datafusion::physical_plan::test::assert_is_pending">assert_is_pending</a>

Asserts that given future is pending.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.build_table_i32.html" class="fn" title="fn datafusion::physical_plan::test::build_table_i32">build_table_i32</a>

Returns record batch with 3 columns of i32 in memory

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.build_table_i32_two_cols.html" class="fn" title="fn datafusion::physical_plan::test::build_table_i32_two_cols">build_table_i32_two_cols</a>

Returns record batch with 2 columns of i32 in memory

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.build_table_scan_i32.html" class="fn" title="fn datafusion::physical_plan::test::build_table_scan_i32">build_table_scan_i32</a>

Returns memory table scan wrapped around record batch with 3 columns of i32

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.make_partition.html" class="fn" title="fn datafusion::physical_plan::test::make_partition">make_partition</a>

Return a RecordBatch with a single Int32 array with values (0..sz) in a field named “i”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.make_partition_utf8.html" class="fn" title="fn datafusion::physical_plan::test::make_partition_utf8">make_partition_utf8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.mem_exec.html" class="fn" title="fn datafusion::physical_plan::test::mem_exec">mem_exec</a>

Returns a `DataSourceExec` that scans `partitions` of 100 batches each

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.mem_exec_utf8.html" class="fn" title="fn datafusion::physical_plan::test::mem_exec_utf8">mem_exec_utf8</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.scan_partitioned.html" class="fn" title="fn datafusion::physical_plan::test::scan_partitioned">scan_partitioned</a>

Returns a `DataSourceExec` that scans `partitions` of 100 batches each

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/fn.scan_partitioned_utf8.html" class="fn" title="fn datafusion::physical_plan::test::scan_partitioned_utf8">scan_partitioned_utf8</a>
