# datafusion.SessionConfig — Apache Arrow DataFusion documentation

_class_ datafusion.SessionConfig(_config_options\=None_)

Bases: `object`

Configuration options for a SessionContext

\_\_init\_\_()

Methods

| **init**()                                       |     |
| ------------------------------------------------ | --- |
| set(key, value)                                  |     |
| with_batch_size(batch_size)                      |     |
| with_create_default_catalog_and_schema(enabled)  |     |
| with_default_catalog_and_schema(catalog, schema) |     |
| with_information_schema(enabled)                 |     |
| with_parquet_pruning(enabled)                    |     |
| with_repartition_aggregations(enabled)           |     |
| with_repartition_file_min_size(size)             |     |
| with_repartition_file_scans(enabled)             |     |
| with_repartition_joins(enabled)                  |     |
| with_repartition_sorts(enabled)                  |     |
| with_repartition_windows(enabled)                |     |
| with_target_partitions(target_partitions)        |     |

set(_key_, _value_)

with_batch_size(_batch_size_)

with_create_default_catalog_and_schema(_enabled_)

with_default_catalog_and_schema(_catalog_, _schema_)

with_information_schema(_enabled_)

with_parquet_pruning(_enabled_)

with_repartition_aggregations(_enabled_)

with_repartition_file_min_size(_size_)

with_repartition_file_scans(_enabled_)

with_repartition_joins(_enabled_)

with_repartition_sorts(_enabled_)

with_repartition_windows(_enabled_)

with_target_partitions(_target_partitions_)
