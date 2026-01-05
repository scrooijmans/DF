# Distributing DataFusion with Ballista — Apache DataFusion Ballista  documentation
To connect to a Ballista cluster from Rust, first start by creating a `SessionContext` connected to remote scheduler server.

```
use ballista::prelude::*;
use datafusion::{
    execution::SessionStateBuilder,
    prelude::{SessionConfig, SessionContext},
};

let config = SessionConfig::new_with_ballista()
    .with_target_partitions(4)
    .with_ballista_job_name("Remote SQL Example");

let state = SessionStateBuilder::new()
    .with_config(config)
    .with_default_features()
    .build();

let ctx = SessionContext::remote_with_state("df://localhost:50050", state).await?;

```


For testing purposes, standalone, in process cluster could be started with:

```
use ballista::prelude::*;
use datafusion::{
    execution::SessionStateBuilder,
    prelude::{SessionConfig, SessionContext},
};
let config = SessionConfig::new_with_ballista()
    .with_target_partitions(1)
    .with_ballista_standalone_parallelism(2);

let state = SessionStateBuilder::new()
    .with_config(config)
    .with_default_features()
    .build();

let ctx = SessionContext::standalone_with_state(state).await?;

```


Following examples require running remove scheduler and executor nodes.

Full example using the DataFrame API.

```
use ballista::prelude::*;
use ballista_examples::test_util;
use datafusion::{
    prelude::{col, lit, ParquetReadOptions, SessionContext},
};

/// This example demonstrates executing a simple query against an Arrow data source (Parquet) and
/// fetching results, using the DataFrame trait
#[tokio::main]
async fn main() -> Result<()> {
    // creating SessionContext with default settings
    let ctx = SessionContext::remote("df://localhost:50050").await?;

    let test_data = test_util::examples_test_data();
    let filename = format!("{test_data}/alltypes_plain.parquet");

    let df = ctx
        .read_parquet(filename, ParquetReadOptions::default())
        .await?
        .select_columns(&["id", "bool_col", "timestamp_col"])?
        .filter(col("id").gt(lit(1)))?;

    df.show().await?;

    Ok(())
}

```


Here is a full example demonstrating SQL usage, with user specific `SessionConfig`:

```
use ballista::prelude::*;
use ballista_examples::test_util;
use datafusion::{
    execution::SessionStateBuilder,
    prelude::{CsvReadOptions, SessionConfig, SessionContext},
};

/// This example demonstrates executing a simple query against an Arrow data source (CSV) and
/// fetching results, using SQL
#[tokio::main]
async fn main() -> Result<()> {
    let config = SessionConfig::new_with_ballista()
        .with_target_partitions(4)
        .with_ballista_job_name("Remote SQL Example");

    let state = SessionStateBuilder::new()
        .with_config(config)
        .with_default_features()
        .build();

    let ctx = SessionContext::remote_with_state("df://localhost:50050", state).await?;

    let test_data = test_util::examples_test_data();

    ctx.register_csv(
        "test",
        &format!("{test_data}/aggregate_test_100.csv"),
        CsvReadOptions::new(),
    )
    .await?;

    let df = ctx
        .sql(
            "SELECT c1, MIN(c12), MAX(c12) \
        FROM test \
        WHERE c11 > 0.1 AND c11 < 0.9 \
        GROUP BY c1",
        )
        .await?;

    df.show().await?;

    Ok(())
}

```
