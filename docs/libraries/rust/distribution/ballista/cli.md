# Ballista Command-line Interface — Apache DataFusion Ballista  documentation
The Ballista CLI allows SQL queries to be executed against a Ballista cluster, or in standalone mode in a single process.

Use Cargo to install:

```
cargo install ballista-cli

```


Usage
---------------------------------------

```
USAGE:
    ballista-cli [OPTIONS]

OPTIONS:
    -c, --batch-size <BATCH_SIZE>    The batch size of each query, or use Ballista default
    -f, --file <FILE>...             Execute commands from file(s), then exit
        --format <FORMAT>            [default: table] [possible values: csv, tsv, table, json,
                                     nd-json]
    -h, --help                       Print help information
        --host <HOST>                Ballista scheduler host
    -p, --data-path <DATA_PATH>      Path to your data, default to current directory
        --port <PORT>                Ballista scheduler port
    -q, --quiet                      Reduce printing other than the results and work quietly
    -r, --rc <RC>...                 Run the provided files on startup instead of ~/.ballistarc
    -V, --version                    Print version information

```


Example
-------------------------------------------

Create a CSV file to query.

Run Ballista CLI in Distributed Mode
-----------------------------------------------------------------------------------------------------

The CLI can connect to a Ballista scheduler for query execution.

```
ballista-cli --host localhost --port 50050

```


Run Ballista CLI in Standalone Mode
---------------------------------------------------------------------------------------------------

It is also possible to run the CLI in standalone mode, where it will create a scheduler and executor in-process.

```
$ ballista-cli

Ballista CLI v8.0.0

> CREATE EXTERNAL TABLE foo (a INT, b INT) STORED AS CSV LOCATION 'data.csv';
0 rows in set. Query took 0.001 seconds.

> SELECT * FROM foo;
+---+---+
| a | b |
+---+---+
| 1 | 2 |
+---+---+
1 row in set. Query took 0.017 seconds.

```


Cli commands
-----------------------------------------------------

Available commands inside Ballista CLI are:

*   Quit
    

*   Help
    

*   ListTables
    

*   DescribeTable
    

*   QuietMode
    

*   list function
    

*   Search and describe function