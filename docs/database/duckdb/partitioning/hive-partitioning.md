# Hive Partitioning – DuckDB

## [Examples](#examples)

Read data from a Hive partitioned dataset:

```
SELECT *
FROM read_parquet('orders/*/*/*.parquet', hive_partitioning = true);

```

Write a table to a Hive partitioned dataset:

```
COPY orders
TO 'orders' (FORMAT parquet, PARTITION_BY (year, month));

```

Note that the `PARTITION_BY` options cannot use expressions. You can produce columns on the fly using the following syntax:

```
COPY (SELECT *, year(timestamp) AS year, month(timestamp) AS month FROM services)
TO 'test' (PARTITION_BY (year, month));

```

When reading, the partition columns are read from the directory structure and can be included or excluded depending on the `hive_partitioning` parameter.

```
FROM read_parquet('test/*/*/*.parquet', hive_partitioning = false); -- will not include year, month columns
FROM read_parquet('test/*/*/*.parquet', hive_partitioning = true);  -- will include year, month partition columns

```

Hive partitioning is a [partitioning strategy](<https://en.wikipedia.org/wiki/Partition_(database)>) that is used to split a table into multiple files based on **partition keys**. The files are organized into folders. Within each folder, the **partition key** has a value that is determined by the name of the folder.

Below is an example of a Hive partitioned file hierarchy. The files are partitioned on two keys (`year` and `month`).

```
orders
├── year=2021
│    ├── month=1
│    │   ├── file1.parquet
│    │   └── file2.parquet
│    └── month=2
│        └── file3.parquet
└── year=2022
     ├── month=11
     │   ├── file4.parquet
     │   └── file5.parquet
     └── month=12
         └── file6.parquet

```

Files stored in this hierarchy can be read using the `hive_partitioning` flag.

```
SELECT *
FROM read_parquet('orders/*/*/*.parquet', hive_partitioning = true);

```

When we specify the `hive_partitioning` flag, the values of the columns will be read from the directories.

### [Filter Pushdown](#filter-pushdown)

Filters on the partition keys are automatically pushed down into the files. This way the system skips reading files that are not necessary to answer a query. For example, consider the following query on the above dataset:

```
SELECT *
FROM read_parquet('orders/*/*/*.parquet', hive_partitioning = true)
WHERE year = 2022
  AND month = 11;

```

When executing this query, only the following files will be read:

```
orders
└── year=2022
     └── month=11
         ├── file4.parquet
         └── file5.parquet

```

### [Auto-detection](#auto-detection)

By default the system tries to infer if the provided files are in a hive partitioned hierarchy. And if so, the `hive_partitioning` flag is enabled automatically. The auto-detection will look at the names of the folders and search for a `'key' = 'value'` pattern. This behavior can be overridden by using the `hive_partitioning` configuration option:

```
SET hive_partitioning = false;

```

### [Hive Types](#hive-types)

`hive_types` is a way to specify the logical types of the hive partitions in a struct:

```
SELECT *
FROM read_parquet(
    'dir/**/*.parquet',
    hive_partitioning = true,
    hive_types = {'release': DATE, 'orders': BIGINT}
);

```

`hive_types` will be auto-detected for the following types: `DATE`, `TIMESTAMP` and `BIGINT`. To switch off the auto-detection, the flag `hive_types_autocast = 0` can be set.

### [Writing Partitioned Files](#writing-partitioned-files)

See the [Partitioned Writes](https://duckdb.org/docs/stable/data/partitioning/partitioned_writes.html) section.

© 2025 DuckDB Foundation, Amsterdam NL
