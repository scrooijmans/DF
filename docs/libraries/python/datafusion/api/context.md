# datafusion.context — Apache Arrow DataFusion documentation

Session Context and it’s associated configuration.

## Classes

- ArrowArrayExportable: ArrowStreamExportable
  - Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.: Type hint for object exporting Arrow C Stream via Arrow PyCapsule Interface.
- ArrowArrayExportable: CatalogProviderExportable
  - Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.: Type hint for object that has **datafusion_catalog_provider** PyCapsule.
- ArrowArrayExportable: RuntimeConfig
  - Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.: See RuntimeEnvBuilder.
- ArrowArrayExportable: RuntimeEnvBuilder
  - Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.: Runtime configuration options.
- ArrowArrayExportable: SQLOptions
  - Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.: Options to be used when performing SQL queries.
- ArrowArrayExportable: SessionConfig
  - Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.: Session configuration options.
- ArrowArrayExportable: SessionContext
  - Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.: This is the main interface for executing queries and creating DataFrames.
- ArrowArrayExportable: TableProviderExportable
  - Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.: Type hint for object that has **datafusion_table_provider** PyCapsule.

## Module Contents

_class_ datafusion.context.ArrowArrayExportable

Bases: `Protocol`

Type hint for object exporting Arrow C Array via Arrow PyCapsule Interface.

[https://arrow.apache.org/docs/format/CDataInterface/PyCapsuleInterface.html](https://arrow.apache.org/docs/format/CDataInterface/PyCapsuleInterface.html)

\_\_arrow_c_array\_\_(_requested_schema: object | None \= None_) → tuple\[object, object\]

_class_ datafusion.context.ArrowStreamExportable

Bases: `Protocol`

Type hint for object exporting Arrow C Stream via Arrow PyCapsule Interface.

[https://arrow.apache.org/docs/format/CDataInterface/PyCapsuleInterface.html](https://arrow.apache.org/docs/format/CDataInterface/PyCapsuleInterface.html)

\_\_arrow_c_stream\_\_(_requested_schema: object | None \= None_) → object

_class_ datafusion.context.CatalogProviderExportable

Bases: `Protocol`

Type hint for object that has \_\_datafusion_catalog_provider\_\_ PyCapsule.

[https://docs.rs/datafusion/latest/datafusion/catalog/trait.CatalogProvider.html](https://docs.rs/datafusion/latest/datafusion/catalog/trait.CatalogProvider.html)

\_\_datafusion_catalog_provider\_\_() → object

_class_ datafusion.context.RuntimeConfig

Bases: [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder")

See RuntimeEnvBuilder.

Create a new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") with default values.

_class_ datafusion.context.RuntimeEnvBuilder

Runtime configuration options.

Create a new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") with default values.

with_disk_manager_disabled() → [RuntimeEnvBuilder](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder")

Disable the disk manager, attempts to create temporary files will error.

Returns:

A new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") object with the updated setting.

with_disk_manager_os() → [RuntimeEnvBuilder](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder")

Use the operating system’s temporary directory for disk manager.

Returns:

A new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") object with the updated setting.

with_disk_manager_specified(_\*paths: str | pathlib.Path_) → [RuntimeEnvBuilder](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder")

Use the specified paths for the disk manager’s temporary files.

Parameters:

**paths** – Paths to use for the disk manager’s temporary files.

Returns:

A new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") object with the updated setting.

with_fair_spill_pool(_size: int_) → [RuntimeEnvBuilder](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder")

Use a fair spill pool with the specified size.

This pool works best when you know beforehand the query has multiple spillable operators that will likely all need to spill. Sometimes it will cause spills even when there was sufficient memory (reserved for other operators) to avoid doing so:

```
┌───────────────────────z──────────────────────z───────────────┐
│                       z                      z               │
│                       z                      z               │
│       Spillable       z       Unspillable    z     Free      │
│        Memory         z        Memory        z    Memory     │
│                       z                      z               │
│                       z                      z               │
└───────────────────────z──────────────────────z───────────────┘

```

Parameters:

**size** – Size of the memory pool in bytes.

Returns:

A new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") object with the updated setting.

Examples usage:

```
config = RuntimeEnvBuilder().with_fair_spill_pool(1024)

```

with_greedy_memory_pool(_size: int_) → [RuntimeEnvBuilder](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder")

Use a greedy memory pool with the specified size.

This pool works well for queries that do not need to spill or have a single spillable operator. See [`with_fair_spill_pool()`](#datafusion.context.RuntimeEnvBuilder.with_fair_spill_pool "datafusion.context.RuntimeEnvBuilder.with_fair_spill_pool") if there are multiple spillable operators that all will spill.

Parameters:

**size** – Size of the memory pool in bytes.

Returns:

A new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") object with the updated setting.

Example usage:

```
config = RuntimeEnvBuilder().with_greedy_memory_pool(1024)

```

with_temp_file_path(_path: str | pathlib.Path_) → [RuntimeEnvBuilder](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder")

Use the specified path to create any needed temporary files.

Parameters:

**path** – Path to use for temporary files.

Returns:

A new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") object with the updated setting.

Example usage:

```
config = RuntimeEnvBuilder().with_temp_file_path("/tmp")

```

with_unbounded_memory_pool() → [RuntimeEnvBuilder](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder")

Use an unbounded memory pool.

Returns:

A new [`RuntimeEnvBuilder`](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") object with the updated setting.

config_internal

_class_ datafusion.context.SQLOptions

Options to be used when performing SQL queries.

Create a new [`SQLOptions`](#datafusion.context.SQLOptions "datafusion.context.SQLOptions") with default values.

The default values are: - DDL commands are allowed - DML commands are allowed - Statements are allowed

with_allow_ddl(_allow: bool \= True_) → [SQLOptions](#datafusion.context.SQLOptions "datafusion.context.SQLOptions")

Should DDL (Data Definition Language) commands be run?

Examples of DDL commands include `CREATE TABLE` and `DROP TABLE`.

Parameters:

**allow** – Allow DDL commands to be run.

Returns:

A new [`SQLOptions`](#datafusion.context.SQLOptions "datafusion.context.SQLOptions") object with the updated setting.

Example usage:

```
options = SQLOptions().with_allow_ddl(True)

```

with_allow_dml(_allow: bool \= True_) → [SQLOptions](#datafusion.context.SQLOptions "datafusion.context.SQLOptions")

Should DML (Data Manipulation Language) commands be run?

Examples of DML commands include `INSERT INTO` and `DELETE`.

Parameters:

**allow** – Allow DML commands to be run.

Returns:

A new [`SQLOptions`](#datafusion.context.SQLOptions "datafusion.context.SQLOptions") object with the updated setting.

Example usage:

```
options = SQLOptions().with_allow_dml(True)

```

with_allow_statements(_allow: bool \= True_) → [SQLOptions](#datafusion.context.SQLOptions "datafusion.context.SQLOptions")

Should statements such as `SET VARIABLE` and `BEGIN TRANSACTION` be run?

Parameters:

**allow** – Allow statements to be run.

Returns:

py:class:SQLOptions\` object with the updated setting.

Return type:

A new

Example usage:

```
options = SQLOptions().with_allow_statements(True)

```

options_internal

_class_ datafusion.context.SessionConfig(_config_options: dict\[str, str\] | None \= None_)

Session configuration options.

Create a new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") with the given configuration options.

Parameters:

**config_options** – Configuration options.

set(_key: str_, _value: str_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Set a configuration option.

Args: key: Option key. value: Option value.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_batch_size(_batch_size: int_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Customize batch size.

Parameters:

**batch_size** – Batch size.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_create_default_catalog_and_schema(_enabled: bool \= True_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Control if the default catalog and schema will be automatically created.

Parameters:

**enabled** – Whether the default catalog and schema will be automatically created.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_default_catalog_and_schema(_catalog: str_, _schema: str_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Select a name for the default catalog and schema.

Parameters:

- **catalog** – Catalog name.
- **schema** – Schema name.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_information_schema(_enabled: bool \= True_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Enable or disable the inclusion of `information_schema` virtual tables.

Parameters:

**enabled** – Whether to include `information_schema` virtual tables.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_parquet_pruning(_enabled: bool \= True_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Enable or disable the use of pruning predicate for parquet readers.

Pruning predicates will enable the reader to skip row groups.

Parameters:

**enabled** – Whether to use pruning predicate for parquet readers.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_repartition_aggregations(_enabled: bool \= True_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Enable or disable the use of repartitioning for aggregations.

Enabling this improves parallelism.

Parameters:

**enabled** – Whether to use repartitioning for aggregations.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_repartition_file_min_size(_size: int_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Set minimum file range size for repartitioning scans.

Parameters:

**size** – Minimum file range size.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_repartition_file_scans(_enabled: bool \= True_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Enable or disable the use of repartitioning for file scans.

Parameters:

**enabled** – Whether to use repartitioning for file scans.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_repartition_joins(_enabled: bool \= True_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Enable or disable the use of repartitioning for joins to improve parallelism.

Parameters:

**enabled** – Whether to use repartitioning for joins.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_repartition_sorts(_enabled: bool \= True_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Enable or disable the use of repartitioning for window functions.

This may improve parallelism.

Parameters:

**enabled** – Whether to use repartitioning for window functions.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_repartition_windows(_enabled: bool \= True_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Enable or disable the use of repartitioning for window functions.

This may improve parallelism.

Parameters:

**enabled** – Whether to use repartitioning for window functions.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

with_target_partitions(_target_partitions: int_) → [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig")

Customize the number of target partitions for query execution.

Increasing partitions can increase concurrency.

Parameters:

**target_partitions** – Number of target partitions.

Returns:

A new [`SessionConfig`](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") object with the updated setting.

config_internal

_class_ datafusion.context.SessionContext(_config: [SessionConfig](#datafusion.context.SessionConfig "datafusion.context.SessionConfig") | None \= None_, _runtime: [RuntimeEnvBuilder](#datafusion.context.RuntimeEnvBuilder "datafusion.context.RuntimeEnvBuilder") | None \= None_)

This is the main interface for executing queries and creating DataFrames.

See [Concepts](about:blank/user-guide/basics.html#user-guide-concepts) in the online documentation for more information.

Main interface for executing queries with DataFusion.

Maintains the state of the connection between a user and an instance of the connection between a user and an instance of the DataFusion engine.

Parameters:

- **config** – Session configuration options.
- **runtime** – Runtime configuration options.

Example usage:

The following example demonstrates how to use the context to execute a query against a CSV data source using the `DataFrame` API:

```
from datafusion import SessionContext

ctx = SessionContext()
df = ctx.read_csv("data.csv")

```

\_\_repr\_\_() → str

Print a string representation of the Session Context.

_static_ \_convert_file_sort_order(_file_sort_order: collections.abc.Sequence\[collections.abc.Sequence\[datafusion.expr.SortKey\]\] | None_) → list\[list\[datafusion.\_internal.expr.SortExpr\]\] | None

Convert nested `SortKey` sequences into raw sort expressions.

Each `SortKey` can be a column name string, an `Expr`, or a `SortExpr` and will be converted using `datafusion.expr.sort_list_to_raw_sort_list()`.

_static_ \_convert_table_partition_cols(_table_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\]_) → list\[tuple\[str, pyarrow.DataType\]\]

catalog(_name: str \= 'datafusion'_) → [datafusion.catalog.Catalog](about:blank/catalog/index.html#datafusion.catalog.Catalog "datafusion.catalog.Catalog")

Retrieve a catalog by name.

catalog_names() → set\[str\]

Returns the list of catalogs in this context.

create_dataframe(_partitions: list\[list\[pyarrow.RecordBatch\]\]_, _name: str | None \= None_, _schema: pyarrow.Schema | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create and return a dataframe using the provided partitions.

Parameters:

- **partitions** – `pa.RecordBatch` partitions to register.
- **name** – Resultant dataframe name.
- **schema** – Schema for the partitions.

Returns:

DataFrame representation of the SQL query.

create_dataframe_from_logical_plan(_plan: [datafusion.plan.LogicalPlan](about:blank/plan/index.html#datafusion.plan.LogicalPlan "datafusion.plan.LogicalPlan")_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from an existing plan.

Parameters:

**plan** – Logical plan.

Returns:

DataFrame representation of the logical plan.

deregister_table(_name: str_) → None

Remove a table from the session.

empty_table() → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create an empty [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame").

enable_url_table() → [SessionContext](#datafusion.context.SessionContext "datafusion.context.SessionContext")

Control if local files can be queried as tables.

Returns:

A new [`SessionContext`](#datafusion.context.SessionContext "datafusion.context.SessionContext") object with url table enabled.

execute(_plan: [datafusion.plan.ExecutionPlan](about:blank/plan/index.html#datafusion.plan.ExecutionPlan "datafusion.plan.ExecutionPlan")_, _partitions: int_) → [datafusion.record_batch.RecordBatchStream](about:blank/record_batch/index.html#datafusion.record_batch.RecordBatchStream "datafusion.record_batch.RecordBatchStream")

Execute the `plan` and return the results.

from_arrow(_data: [ArrowStreamExportable](#datafusion.context.ArrowStreamExportable "datafusion.context.ArrowStreamExportable") | [ArrowArrayExportable](#datafusion.context.ArrowArrayExportable "datafusion.context.ArrowArrayExportable")_, _name: str | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from an Arrow source.

The Arrow data source can be any object that implements either `__arrow_c_stream__` or `__arrow_c_array__`. For the latter, it must return a struct array.

Arrow data can be Polars, Pandas, Pyarrow etc.

Parameters:

- **data** – Arrow data source.
- **name** – Name of the DataFrame.

Returns:

DataFrame representation of the Arrow table.

from_arrow_table(_data: pyarrow.Table_, _name: str | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from an Arrow table.

This is an alias for [`from_arrow()`](#datafusion.context.SessionContext.from_arrow "datafusion.context.SessionContext.from_arrow").

from_pandas(_data: pandas.DataFrame_, _name: str | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from a Pandas DataFrame.

Parameters:

- **data** – Pandas DataFrame.
- **name** – Name of the DataFrame.

Returns:

DataFrame representation of the Pandas DataFrame.

from_polars(_data: polars.DataFrame_, _name: str | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from a Polars DataFrame.

Parameters:

- **data** – Polars DataFrame.
- **name** – Name of the DataFrame.

Returns:

DataFrame representation of the Polars DataFrame.

from_pydict(_data: dict\[str, list\[Any\]\]_, _name: str | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from a dictionary.

Parameters:

- **data** – Dictionary of lists.
- **name** – Name of the DataFrame.

Returns:

DataFrame representation of the dictionary of lists.

from_pylist(_data: list\[dict\[str, Any\]\]_, _name: str | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from a list.

Parameters:

- **data** – List of dictionaries.
- **name** – Name of the DataFrame.

Returns:

DataFrame representation of the list of dictionaries.

_classmethod_ global_ctx() → [SessionContext](#datafusion.context.SessionContext "datafusion.context.SessionContext")

Retrieve the global context as a SessionContext wrapper.

Returns:

A SessionContext object that wraps the global SessionContextInternal.

read_avro(_path: str | pathlib.Path_, _schema: pyarrow.Schema | None \= None_, _file_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\] | None \= None_, _file_extension: str \= '.avro'_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a `DataFrame` for reading Avro data source.

Parameters:

- **path** – Path to the Avro file.
- **schema** – The data source schema.
- **file_partition_cols** – Partition columns.
- **file_extension** – File extension to select.

Returns:

DataFrame representation of the read Avro file

read_csv(_path: str | pathlib.Path | list\[str\] | list\[pathlib.Path\]_, _schema: pyarrow.Schema | None \= None_, _has_header: bool \= True_, _delimiter: str \= ','_, _schema_infer_max_records: int \= 1000_, _file_extension: str \= '.csv'_, _table_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\] | None \= None_, _file_compression_type: str | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Read a CSV data source.

Parameters:

- **path** – Path to the CSV file
- **schema** – An optional schema representing the CSV files. If None, the CSV reader will try to infer it based on data in file.
- **has_header** – Whether the CSV file have a header. If schema inference is run on a file with no headers, default column names are created.
- **delimiter** – An optional column delimiter.
- **schema_infer_max_records** – Maximum number of rows to read from CSV files for schema inference if needed.
- **file_extension** – File extension; only files with this extension are selected for data input.
- **table_partition_cols** – Partition columns.
- **file_compression_type** – File compression type.

Returns:

DataFrame representation of the read CSV files

read_json(_path: str | pathlib.Path_, _schema: pyarrow.Schema | None \= None_, _schema_infer_max_records: int \= 1000_, _file_extension: str \= '.json'_, _table_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\] | None \= None_, _file_compression_type: str | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Read a line-delimited JSON data source.

Parameters:

- **path** – Path to the JSON file.
- **schema** – The data source schema.
- **schema_infer_max_records** – Maximum number of rows to read from JSON files for schema inference if needed.
- **file_extension** – File extension; only files with this extension are selected for data input.
- **table_partition_cols** – Partition columns.
- **file_compression_type** – File compression type.

Returns:

DataFrame representation of the read JSON files.

read_parquet(_path: str | pathlib.Path_, _table_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\] | None \= None_, _parquet_pruning: bool \= True_, _file_extension: str \= '.parquet'_, _skip_metadata: bool \= True_, _schema: pyarrow.Schema | None \= None_, _file_sort_order: collections.abc.Sequence\[collections.abc.Sequence\[datafusion.expr.SortKey\]\] | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Read a Parquet source into a `Dataframe`.

Parameters:

- **path** – Path to the Parquet file.
- **table_partition_cols** – Partition columns.
- **parquet_pruning** – Whether the parquet reader should use the predicate to prune row groups.
- **file_extension** – File extension; only files with this extension are selected for data input.
- **skip_metadata** – Whether the parquet reader should skip any metadata that may be in the file schema. This can help avoid schema conflicts due to metadata.
- **schema** – An optional schema representing the parquet files. If None, the parquet reader will try to infer it based on data in the file.
- **file_sort_order** – Sort order for the file. Each sort key can be specified as a column name (`str`), an expression (`Expr`), or a `SortExpr`.

Returns:

DataFrame representation of the read Parquet files

read_table(_table: [datafusion.catalog.Table](about:blank/catalog/index.html#datafusion.catalog.Table "datafusion.catalog.Table")_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Creates a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from a table.

For a [`Table`](about:blank/catalog/index.html#datafusion.catalog.Table "datafusion.catalog.Table") such as a `ListingTable`, create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame").

register_avro(_name: str_, _path: str | pathlib.Path_, _schema: pyarrow.Schema | None \= None_, _file_extension: str \= '.avro'_, _table_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\] | None \= None_) → None

Register an Avro file as a table.

The registered table can be referenced from SQL statement executed against this context.

Parameters:

- **name** – Name of the table to register.
- **path** – Path to the Avro file.
- **schema** – The data source schema.
- **file_extension** – File extension to select.
- **table_partition_cols** – Partition columns.

register_catalog_provider(_name: str_, _provider: [CatalogProviderExportable](#datafusion.context.CatalogProviderExportable "datafusion.context.CatalogProviderExportable") | [datafusion.catalog.CatalogProvider](about:blank/catalog/index.html#datafusion.catalog.CatalogProvider "datafusion.catalog.CatalogProvider") | [datafusion.catalog.Catalog](about:blank/catalog/index.html#datafusion.catalog.Catalog "datafusion.catalog.Catalog")_) → None

Register a catalog provider.

register_csv(_name: str_, _path: str | pathlib.Path | list\[str | pathlib.Path\]_, _schema: pyarrow.Schema | None \= None_, _has_header: bool \= True_, _delimiter: str \= ','_, _schema_infer_max_records: int \= 1000_, _file_extension: str \= '.csv'_, _file_compression_type: str | None \= None_) → None

Register a CSV file as a table.

The registered table can be referenced from SQL statement executed against.

Parameters:

- **name** – Name of the table to register.
- **path** – Path to the CSV file. It also accepts a list of Paths.
- **schema** – An optional schema representing the CSV file. If None, the CSV reader will try to infer it based on data in file.
- **has_header** – Whether the CSV file have a header. If schema inference is run on a file with no headers, default column names are created.
- **delimiter** – An optional column delimiter.
- **schema_infer_max_records** – Maximum number of rows to read from CSV files for schema inference if needed.
- **file_extension** – File extension; only files with this extension are selected for data input.
- **file_compression_type** – File compression type.

register_dataset(_name: str_, _dataset: pyarrow.dataset.Dataset_) → None

Register a `pa.dataset.Dataset` as a table.

Parameters:

- **name** – Name of the table to register.
- **dataset** – PyArrow dataset.

register_json(_name: str_, _path: str | pathlib.Path_, _schema: pyarrow.Schema | None \= None_, _schema_infer_max_records: int \= 1000_, _file_extension: str \= '.json'_, _table_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\] | None \= None_, _file_compression_type: str | None \= None_) → None

Register a JSON file as a table.

The registered table can be referenced from SQL statement executed against this context.

Parameters:

- **name** – Name of the table to register.
- **path** – Path to the JSON file.
- **schema** – The data source schema.
- **schema_infer_max_records** – Maximum number of rows to read from JSON files for schema inference if needed.
- **file_extension** – File extension; only files with this extension are selected for data input.
- **table_partition_cols** – Partition columns.
- **file_compression_type** – File compression type.

register_listing_table(_name: str_, _path: str | pathlib.Path_, _table_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\] | None \= None_, _file_extension: str \= '.parquet'_, _schema: pyarrow.Schema | None \= None_, _file_sort_order: collections.abc.Sequence\[collections.abc.Sequence\[datafusion.expr.SortKey\]\] | None \= None_) → None

Register multiple files as a single table.

Registers a [`Table`](about:blank/catalog/index.html#datafusion.catalog.Table "datafusion.catalog.Table") that can assemble multiple files from locations in an `ObjectStore` instance.

Parameters:

- **name** – Name of the resultant table.
- **path** – Path to the file to register.
- **table_partition_cols** – Partition columns.
- **file_extension** – File extension of the provided table.
- **schema** – The data source schema.
- **file_sort_order** – Sort order for the file. Each sort key can be specified as a column name (`str`), an expression (`Expr`), or a `SortExpr`.

register_object_store(_schema: str_, _store: Any_, _host: str | None \= None_) → None

Add a new object store into the session.

Parameters:

- **schema** – The data source schema.
- **store** – The `ObjectStore` to register.
- **host** – URL for the host.

register_parquet(_name: str_, _path: str | pathlib.Path_, _table_partition_cols: list\[tuple\[str, str | pyarrow.DataType\]\] | None \= None_, _parquet_pruning: bool \= True_, _file_extension: str \= '.parquet'_, _skip_metadata: bool \= True_, _schema: pyarrow.Schema | None \= None_, _file_sort_order: collections.abc.Sequence\[collections.abc.Sequence\[datafusion.expr.SortKey\]\] | None \= None_) → None

Register a Parquet file as a table.

The registered table can be referenced from SQL statement executed against this context.

Parameters:

- **name** – Name of the table to register.
- **path** – Path to the Parquet file.
- **table_partition_cols** – Partition columns.
- **parquet_pruning** – Whether the parquet reader should use the predicate to prune row groups.
- **file_extension** – File extension; only files with this extension are selected for data input.
- **skip_metadata** – Whether the parquet reader should skip any metadata that may be in the file schema. This can help avoid schema conflicts due to metadata.
- **schema** – The data source schema.
- **file_sort_order** – Sort order for the file. Each sort key can be specified as a column name (`str`), an expression (`Expr`), or a `SortExpr`.

register_record_batches(_name: str_, _partitions: list\[list\[pyarrow.RecordBatch\]\]_) → None

Register record batches as a table.

This function will convert the provided partitions into a table and register it into the session using the given name.

Parameters:

- **name** – Name of the resultant table.
- **partitions** – Record batches to register as a table.

register_table(_name: str_, _table: [datafusion.catalog.Table](about:blank/catalog/index.html#datafusion.catalog.Table "datafusion.catalog.Table")_) → None

Register a :py:class: ~datafusion.catalog.Table as a table.

The registered table can be referenced from SQL statement executed against.

Parameters:

- **name** – Name of the resultant table.
- **table** – DataFusion table to add to the session context.

register_table_provider(_name: str_, _provider: [TableProviderExportable](#datafusion.context.TableProviderExportable "datafusion.context.TableProviderExportable")_) → None

Register a table provider.

This table provider must have a method called `__datafusion_table_provider__` which returns a PyCapsule that exposes a `FFI_TableProvider`.

register_udaf(_udaf: [datafusion.user_defined.AggregateUDF](about:blank/user_defined/index.html#datafusion.user_defined.AggregateUDF "datafusion.user_defined.AggregateUDF")_) → None

Register a user-defined aggregation function (UDAF) with the context.

register_udf(_udf: [datafusion.user_defined.ScalarUDF](about:blank/user_defined/index.html#datafusion.user_defined.ScalarUDF "datafusion.user_defined.ScalarUDF")_) → None

Register a user-defined function (UDF) with the context.

register_udtf(_func: [datafusion.user_defined.TableFunction](about:blank/user_defined/index.html#datafusion.user_defined.TableFunction "datafusion.user_defined.TableFunction")_) → None

Register a user defined table function.

register_udwf(_udwf: [datafusion.user_defined.WindowUDF](about:blank/user_defined/index.html#datafusion.user_defined.WindowUDF "datafusion.user_defined.WindowUDF")_) → None

Register a user-defined window function (UDWF) with the context.

register_view(_name: str_, _df: [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")_) → None

Register a :py:class: ~datafusion.detaframe.DataFrame as a view.

Parameters:

- **name** (_str_) – The name to register the view under.
- **df** ([_DataFrame_](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")) – The DataFrame to be converted into a view and registered.

session_id() → str

Return an id that uniquely identifies this [`SessionContext`](#datafusion.context.SessionContext "datafusion.context.SessionContext").

sql(_query: str_, _options: [SQLOptions](#datafusion.context.SQLOptions "datafusion.context.SQLOptions") | None \= None_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a `DataFrame` from SQL query text.

Note: This API implements DDL statements such as `CREATE TABLE` and `CREATE VIEW` and DML statements such as `INSERT INTO` with in-memory default implementation.See [`sql_with_options()`](#datafusion.context.SessionContext.sql_with_options "datafusion.context.SessionContext.sql_with_options").

Parameters:

- **query** – SQL query text.
- **options** – If provided, the query will be validated against these options.

Returns:

DataFrame representation of the SQL query.

sql_with_options(_query: str_, _options: [SQLOptions](#datafusion.context.SQLOptions "datafusion.context.SQLOptions")_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Create a [`DataFrame`](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame") from SQL query text.

This function will first validate that the query is allowed by the provided options.

Parameters:

- **query** – SQL query text.
- **options** – SQL options.

Returns:

DataFrame representation of the SQL query.

table(_name: str_) → [datafusion.dataframe.DataFrame](about:blank/dataframe/index.html#datafusion.dataframe.DataFrame "datafusion.dataframe.DataFrame")

Retrieve a previously registered table by name.

table_exist(_name: str_) → bool

Return whether a table with the given name exists.

ctx

_class_ datafusion.context.TableProviderExportable

Bases: `Protocol`

Type hint for object that has \_\_datafusion_table_provider\_\_ PyCapsule.

[https://datafusion.apache.org/python/user-guide/io/table_provider.html](https://datafusion.apache.org/python/user-guide/io/table_provider.html)

\_\_datafusion_table_provider\_\_() → object
