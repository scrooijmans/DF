# Statistics schema — Apache Arrow v21.0.0

Warning

This specification should be considered experimental.

## Rationale[#](#rationale "Permalink to this heading")

Statistics are useful for fast query processing. Many query engines use statistics to optimize their query plan.

Apache Arrow format doesn’t have statistics but other formats that can be read as Apache Arrow data may have statistics. For example, the Apache Parquet C++ implementation can read an Apache Parquet file as Apache Arrow data and the Apache Parquet file may have statistics.

We standardize the representation of statistics as an Apache Arrow array for ease of exchange.

### Use case[#](#use-case "Permalink to this heading")

One of [The Arrow C stream interface](about:blank/CStreamInterface.html#c-stream-interface) use cases is the following:

1.  Module A reads Apache Parquet file as Apache Arrow data.
2.  Module A passes the read Apache Arrow data to module B through the Arrow C stream interface.
3.  Module B processes the passed Apache Arrow data.

If module A can pass the statistics associated with the Apache Parquet file to module B, module B can use the statistics to optimize its query plan.

For example, DuckDB uses this approach but DuckDB couldn’t use statistics because there wasn’t a standardized way to represent statistics for the Apache Arrow data.

### Goals[#](#goals "Permalink to this heading")

- Establish a standard way to represent statistics as an Apache Arrow array.

### Non-goals[#](#non-goals "Permalink to this heading")

- Establish a standard way to pass an Apache Arrow array that represents statistics.
- Establish a standard way to embed statistics into an Apache Arrow array itself.

## Schema[#](#schema "Permalink to this heading")

This specification provides only the schema for statistics. This is the canonical schema to represent statistics about an Apache Arrow dataset as Apache Arrow data.

Here is the outline of the schema for statistics:

```
struct<
  column: int32,
  statistics: map<
    key: dictionary<values: utf8, indices: int32>,
    items: dense_union<...all needed types...>
  >
>

```

Here is the details of top-level `struct`:

Here is the details of the `map` of the `statistics`:

### Standard statistics[#](#standard-statistics "Permalink to this heading")

Each statistic kind has a name that appears as a key in the statistics map for each column or entire table. `dictionary<values: utf8, indices: int32>` is used to encode the name for space-efficiency.

We assign different names for variations of the same statistic instead of using flags. For example, we assign different statistic names for exact and approximate values of the “distinct count” statistic.

The colon symbol `:` is to be used as a namespace separator like [Custom Application Metadata](about:blank/Columnar.html#format-metadata). It can be used multiple times in a name.

The `ARROW` prefix is a reserved namespace for pre-defined statistic names in current and future versions of this specification. User-defined statistics must not use it. For example, you can use your product name as namespace such as `MY_PRODUCT:my_statistics:exact`.

Here are pre-defined statistics names:

If you find a statistic that might be useful to multiple systems, please propose it on the [Apache Arrow development mailing-list](https://arrow.apache.org/community/).

Interoperability improves when producers and consumers of statistics follow a previously agreed upon statistic specification.

## Examples[#](#examples "Permalink to this heading")

Here are some examples to help you understand.

### Simple record batch[#](#simple-record-batch "Permalink to this heading")

Schema:

```
vendor_id: int32
passenger_count: int64

```

Data:

```
vendor_id:       [5, 1, 5, 1, 5]
passenger_count: [1, 1, 2, 0, null]

```

Statistics:

Column indexes:

Statistics schema:

```
struct<
  column: int32,
  statistics: map<
    key: dictionary<values: utf8, indices: int32>,
    items: dense_union<0: int64>
  >
>

```

Statistics array:

```
column: [
  null, # record batch
  0,    # vendor_id
  1,    # passenger_count
]
statistics:
  offsets: [
    0,
    1, # record batch: 1 value: [0]
    5, # vendor_id: 4 values: [1, 2, 3, 4]
    9, # passenger_count: 4 values: [5, 6, 7, 8]
  ]
  key:
    values: [
      "ARROW:row_count:exact",
      "ARROW:null_count:exact",
      "ARROW:distinct_count:exact",
      "ARROW:max_value:exact",
      "ARROW:min_value:exact",
    ]
    indices: [
      0, # "ARROW:row_count:exact"
      1, # "ARROW:null_count:exact"
      2, # "ARROW:distinct_count:exact"
      3, # "ARROW:max_value:exact"
      4, # "ARROW:min_value:exact"
      1, # "ARROW:null_count:exact"
      2, # "ARROW:distinct_count:exact"
      3, # "ARROW:max_value:exact"
      4, # "ARROW:min_value:exact"
    ]
  items:
    children:
      0: [ # int64
        5, # record batch: "ARROW:row_count:exact"
        0, # vendor_id: "ARROW:null_count:exact"
        2, # vendor_id: "ARROW:distinct_count:exact"
        5, # vendor_id: "ARROW:max_value:exact"
        1, # vendor_id: "ARROW:min_value:exact"
        1, # passenger_count: "ARROW:null_count:exact"
        3, # passenger_count: "ARROW:distinct_count:exact"
        2, # passenger_count: "ARROW:max_value:exact"
        0, # passenger_count: "ARROW:min_value:exact"
      ]
    types: [ # all values are int64
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
    ]
    offsets: [
      0,
      1,
      2,
      3,
      4,
      5,
      6,
      7,
      8,
    ]

```

### Complex record batch[#](#complex-record-batch "Permalink to this heading")

This uses nested types.

Schema:

```
col1: struct<a: int32, b: list<item: int64>, c: float64>
col2: utf8

```

Data:

```
col1: [
        {a: 1, b: [20, 30, 40], c: 2.9},
        {a: 2, b: null,         c: -2.9},
        {a: 3, b: [99],         c: null},
      ]
col2: ["x", null, "z"]

```

Statistics:

Column indexes:

See also [RecordBatch message](about:blank/Columnar.html#ipc-recordbatch-message) how to compute column indexes.

Statistics schema:

```
struct<
  column: int32,
  statistics: map<
    key: dictionary<values: utf8, indices: int32>,
    items: dense_union<
      # For the number of rows, the number of nulls and so on.
      0: int64,
      # For the max/min values of col1.c.
      1: float64
    >
  >
>

```

Statistics array:

```
column: [
  null, # record batch
  0,    # col1
  1,    # col1.a
  2,    # col1.b
  3,    # col1.b.item
  4,    # col1.c
  5,    # col2
]
statistics:
  offsets: [
    0,
    1,  # record batch: 1 value: [0]
    2,  # col1: 1 value: [1]
    6,  # col1.a: 4 values: [2, 3, 4, 5]
    7,  # col1.b: 1 value: [6]
    9,  # col1.b.item: 2 values: [7, 8]
    12, # col1.c: 3 values: [9, 10, 11]
    14, # col2: 2 values: [12, 13]
  ]
  key:
    values: [
      "ARROW:row_count:exact",
      "ARROW:null_count:exact",
      "ARROW:distinct_count:exact",
      "ARROW:max_value:approximate",
      "ARROW:min_value:approximate",
      "ARROW:max_value:exact",
      "ARROW:min_value:exact",
    ]
    indices: [
      0, # "ARROW:row_count:exact"
      1, # "ARROW:null_count:exact"
      1, # "ARROW:null_count:exact"
      2, # "ARROW:distinct_count:exact"
      3, # "ARROW:max_value:approximate"
      4, # "ARROW:min_value:approximate"
      1, # "ARROW:null_count:exact"
      5, # "ARROW:max_value:exact"
      6, # "ARROW:min_value:exact"
      1, # "ARROW:null_count:exact"
      3, # "ARROW:max_value:approximate"
      4, # "ARROW:min_value:approximate"
      1, # "ARROW:null_count:exact"
      2, # "ARROW:distinct_count:exact"
    ]
  items:
    children:
      0: [ # int64
        3,  # record batch: "ARROW:row_count:exact"
        0,  # col1: "ARROW:null_count:exact"
        0,  # col1.a: "ARROW:null_count:exact"
        3,  # col1.a: "ARROW:distinct_count:exact"
        5,  # col1.a: "ARROW:max_value:approximate"
        0,  # col1.a: "ARROW:min_value:approximate"
        1,  # col1.b: "ARROW:null_count:exact"
        99, # col1.b.item: "ARROW:max_value:exact"
        20, # col1.b.item: "ARROW:min_value:exact"
        1,  # col1.c: "ARROW:null_count:exact"
        1,  # col2: "ARROW:null_count:exact"
        2,  # col2: "ARROW:distinct_count:exact"
      ]
      1: [ # float64
        3.0,  # col1.c: "ARROW:max_value:approximate"
        -3.0, # col1.c: "ARROW:min_value:approximate"
      ]
    types: [
      0, # int64: record batch: "ARROW:row_count:exact"
      0, # int64: col1: "ARROW:null_count:exact"
      0, # int64: col1.a: "ARROW:null_count:exact"
      0, # int64: col1.a: "ARROW:distinct_count:exact"
      0, # int64: col1.a: "ARROW:max_value:approximate"
      0, # int64: col1.a: "ARROW:min_value:approximate"
      0, # int64: col1.b: "ARROW:null_count:exact"
      0, # int64: col1.b.item: "ARROW:max_value:exact"
      0, # int64: col1.b.item: "ARROW:min_value:exact"
      0, # int64: col1.c: "ARROW:null_count:exact"
      1, # float64: col1.c: "ARROW:max_value:approximate"
      1, # float64: col1.c: "ARROW:min_value:approximate"
      0, # int64: col2: "ARROW:null_count:exact"
      0, # int64: col2: "ARROW:distinct_count:exact"
    ]
    offsets: [
      0,  # int64: record batch: "ARROW:row_count:exact"
      1,  # int64: col1: "ARROW:null_count:exact"
      2,  # int64: col1.a: "ARROW:null_count:exact"
      3,  # int64: col1.a: "ARROW:distinct_count:exact"
      4,  # int64: col1.a: "ARROW:max_value:approximate"
      5,  # int64: col1.a: "ARROW:min_value:approximate"
      6,  # int64: col1.b: "ARROW:null_count:exact"
      7,  # int64: col1.b.item: "ARROW:max_value:exact"
      8,  # int64: col1.b.item: "ARROW:min_value:exact"
      9,  # int64: col1.c: "ARROW:null_count:exact"
      0,  # float64: col1.c: "ARROW:max_value:approximate"
      1,  # float64: col1.c: "ARROW:min_value:approximate"
      10, # int64: col2: "ARROW:null_count:exact"
      11, # int64: col2: "ARROW:distinct_count:exact"
    ]

```

### Simple array[#](#simple-array "Permalink to this heading")

Schema:

Data:

Statistics:

Column indexes:

Statistics schema:

```
struct<
  column: int32,
  statistics: map<
    key: dictionary<values: utf8, indices: int32>,
    items: dense_union<0: int64>
  >
>

```

Statistics array:

```
column: [
  0, # array
]
statistics:
  offsets: [
    0,
    5, # array: 5 values: [0, 1, 2, 3, 4]
  ]
  key:
    values: [
      "ARROW:row_count:exact",
      "ARROW:null_count:exact",
      "ARROW:distinct_count:exact",
      "ARROW:max_value:exact",
      "ARROW:min_value:exact",
    ]
    indices: [
      0, # "ARROW:row_count:exact"
      1, # "ARROW:null_count:exact"
      2, # "ARROW:distinct_count:exact"
      3, # "ARROW:max_value:exact"
      4, # "ARROW:min_value:exact"
    ]
  items:
    children:
      0: [ # int64
        5, # array: "ARROW:row_count:exact"
        1, # array: "ARROW:null_count:exact"
        3, # array: "ARROW:distinct_count:exact"
        2, # array: "ARROW:max_value:exact"
        0, # array: "ARROW:min_value:exact"
      ]
    types: [ # all values are int64
      0,
      0,
      0,
      0,
      0,
    ]
    offsets: [
      0,
      1,
      2,
      3,
      4,
    ]

```

### Complex array[#](#complex-array "Permalink to this heading")

This uses nested types.

Schema:

```
struct<a: int32, b: list<item: int64>, c: float64>

```

Data:

```
[
  {a: 1, b: [20, 30, 40], c: 2.9},
  {a: 2, b: null,         c: -2.9},
  {a: 3, b: [99],         c: null},
]

```

Statistics:

Column indexes:

See also [RecordBatch message](about:blank/Columnar.html#ipc-recordbatch-message) how to compute column indexes.

Statistics schema:

```
struct<
  column: int32,
  statistics: map<
    key: dictionary<values: utf8, indices: int32>,
    items: dense_union<
      # For the number of rows, the number of nulls and so on.
      0: int64,
      # For the max/min values of c.
      1: float64
    >
  >
>

```

Statistics array:

```
column: [
  0, # array
  1, # a
  2, # b
  3, # b.item
  4, # c
]
statistics:
  offsets: [
    0,
    2,  # array: 2 values: [0, 1]
    6,  # a: 4 values: [2, 3, 4, 5]
    7,  # b: 1 value: [6]
    9,  # b.item: 2 values: [7, 8]
    12, # c: 3 values: [9, 10, 11]
  ]
  key:
    values: [
      "ARROW:row_count:exact",
      "ARROW:null_count:exact",
      "ARROW:distinct_count:exact",
      "ARROW:max_value:approximate",
      "ARROW:min_value:approximate",
      "ARROW:max_value:exact",
      "ARROW:min_value:exact",
    ]
    indices: [
      0, # "ARROW:row_count:exact"
      1, # "ARROW:null_count:exact"
      1, # "ARROW:null_count:exact"
      2, # "ARROW:distinct_count:exact"
      3, # "ARROW:max_value:approximate"
      4, # "ARROW:min_value:approximate"
      1, # "ARROW:null_count:exact"
      5, # "ARROW:max_value:exact"
      6, # "ARROW:min_value:exact"
      1, # "ARROW:null_count:exact"
      3, # "ARROW:max_value:approximate"
      4, # "ARROW:min_value:approximate"
    ]
  items:
    children:
      0: [ # int64
        3,  # array: "ARROW:row_count:exact"
        0,  # array: "ARROW:null_count:exact"
        0,  # a: "ARROW:null_count:exact"
        3,  # a: "ARROW:distinct_count:exact"
        5,  # a: "ARROW:max_value:approximate"
        0,  # a: "ARROW:min_value:approximate"
        1,  # b: "ARROW:null_count:exact"
        99, # b.item: "ARROW:max_value:exact"
        20, # b.item: "ARROW:min_value:exact"
        1,  # c: "ARROW:null_count:exact"
      ]
      1: [ # float64
        3.0,  # c: "ARROW:max_value:approximate"
        -3.0, # c: "ARROW:min_value:approximate"
      ]
    types: [
      0, # int64: array: "ARROW:row_count:exact"
      0, # int64: array: "ARROW:null_count:exact"
      0, # int64: a: "ARROW:null_count:exact"
      0, # int64: a: "ARROW:distinct_count:exact"
      0, # int64: a: "ARROW:max_value:approximate"
      0, # int64: a: "ARROW:min_value:approximate"
      0, # int64: b: "ARROW:null_count:exact"
      0, # int64: b.item: "ARROW:max_value:exact"
      0, # int64: b.item: "ARROW:min_value:exact"
      0, # int64: c: "ARROW:null_count:exact"
      1, # float64: c: "ARROW:max_value:approximate"
      1, # float64: c: "ARROW:min_value:approximate"
    ]
    offsets: [
      0, # int64: array: "ARROW:row_count:exact"
      1, # int64: array: "ARROW:null_count:exact"
      2, # int64: a: "ARROW:null_count:exact"
      3, # int64: a: "ARROW:distinct_count:exact"
      4, # int64: a: "ARROW:max_value:approximate"
      5, # int64: a: "ARROW:min_value:approximate"
      6, # int64: b: "ARROW:null_count:exact"
      7, # int64: b.item: "ARROW:max_value:exact"
      8, # int64: b.item: "ARROW:min_value:exact"
      9, # int64: c: "ARROW:null_count:exact"
      0, # float64: c: "ARROW:max_value:approximate"
      1, # float64: c: "ARROW:min_value:approximate"
    ]

```
