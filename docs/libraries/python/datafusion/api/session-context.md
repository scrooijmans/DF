# datafusion.SessionContext — Apache Arrow DataFusion documentation

_class_ datafusion.SessionContext(_config\=None_, _runtime\=None_)

Bases: `object`

PySessionContext is able to plan and execute DataFusion plans. It has a powerful optimizer, a physical planner for local execution, and a multi-threaded execution engine to perform the execution.

\_\_init\_\_()

Methods

- **init**(): catalog([name])
- **init**(): create_dataframe(partitions[, name])
- **init**(): create_dataframe_from_logical_plan(plan)
  - Create a DataFrame from an existing logical plan
- **init**(): deregister_table(name)
- **init**(): empty_table()
- **init**(): execute(plan, part)
  - Execute a partition of an execution plan and return a stream of record batches
- **init**(): from_arrow_table(data[, name])
  - Construct datafusion dataframe from Arrow Table
- **init**(): from_pandas(data[, name])
  - Construct datafusion dataframe from pandas
- **init**(): from_polars(data[, name])
  - Construct datafusion dataframe from polars
- **init**(): from_pydict(data[, name])
  - Construct datafusion dataframe from Python dictionary
- **init**(): from_pylist(data[, name])
  - Construct datafusion dataframe from Python list
- **init**(): read_avro(path[, schema, ...])
- **init**(): read_csv(path[, schema, has_header, ...])
- **init**(): read_json(path[, schema, ...])
- **init**(): read_parquet(path[, table_partition_cols, ...])
- **init**(): read_table(table)
- **init**(): register_avro(name, path[, schema, ...])
- **init**(): register_csv(name, path[, schema, ...])
- **init**(): register_dataset(name, dataset)
- **init**(): register_json(name, path[, schema, ...])
- **init**(): register_object_store(scheme, store[, host])
  - Register a an object store with the given name
- **init**(): register_parquet(name, path[, ...])
- **init**(): register_record_batches(name, partitions)
- **init**(): register_table(name, table)
- **init**(): register_udaf(udaf)
- **init**(): register_udf(udf)
- **init**(): session_id()
- **init**(): sql(query)
  - Returns a PyDataFrame whose plan corresponds to the SQL statement.
- **init**(): table(name)
- **init**(): table_exist(name)
- **init**(): tables()

catalog(_name\='datafusion'_)

create_dataframe(_partitions_, _name\=None_)

create_dataframe_from_logical_plan(_plan_)

Create a DataFrame from an existing logical plan

deregister_table(_name_)

empty_table()

execute(_plan_, _part_)

Execute a partition of an execution plan and return a stream of record batches

from_arrow_table(_data_, _name\=None_)

Construct datafusion dataframe from Arrow Table

from_pandas(_data_, _name\=None_)

Construct datafusion dataframe from pandas

from_polars(_data_, _name\=None_)

Construct datafusion dataframe from polars

from_pydict(_data_, _name\=None_)

Construct datafusion dataframe from Python dictionary

from_pylist(_data_, _name\=None_)

Construct datafusion dataframe from Python list

read_avro(_path_, _schema\=None_, _table_partition_cols\=Ellipsis_, _file_extension\='.avro'_)

read_csv(_path_, _schema\=None_, _has_header\=True_, _delimiter\=','_, _schema_infer_max_records\=1000_, _file_extension\='.csv'_, _table_partition_cols\=Ellipsis_, _file_compression_type\=None_)

read_json(_path_, _schema\=None_, _schema_infer_max_records\=1000_, _file_extension\='.json'_, _table_partition_cols\=Ellipsis_, _file_compression_type\=None_)

read_parquet(_path_, _table_partition_cols\=Ellipsis_, _parquet_pruning\=True_, _file_extension\='.parquet'_, _skip_metadata\=True_, _schema\=None_, _file_sort_order\=None_)

read_table(_table_)

register_avro(_name_, _path_, _schema\=None_, _file_extension\='.avro'_, _table_partition_cols\=Ellipsis_)

register_csv(_name_, _path_, _schema\=None_, _has_header\=True_, _delimiter\=','_, _schema_infer_max_records\=1000_, _file_extension\='.csv'_, _file_compression_type\=None_)

register_dataset(_name_, _dataset_)

register_json(_name_, _path_, _schema\=None_, _schema_infer_max_records\=1000_, _file_extension\='.json'_, _table_partition_cols\=Ellipsis_, _file_compression_type\=None_)

register_object_store(_scheme_, _store_, _host\=None_)

Register a an object store with the given name

register_parquet(_name_, _path_, _table_partition_cols\=Ellipsis_, _parquet_pruning\=True_, _file_extension\='.parquet'_, _skip_metadata\=True_, _schema\=None_, _file_sort_order\=None_)

register_record_batches(_name_, _partitions_)

register_table(_name_, _table_)

register_udaf(_udaf_)

register_udf(_udf_)

session_id()

sql(_query_)

Returns a PyDataFrame whose plan corresponds to the SQL statement.

table(_name_)

table_exist(_name_)

tables()
