Description: An asynchronous, pipelined, PostgreSQL client.

Title: tokio\_postgres - Rust

Docs.rs

*   tokio-postgres-0.7.13

*   tokio-postgres 0.7.13
*   Permalink
*   Docs.rs crate page
*   MIT OR Apache-2.0

*   Links
*   Repository
*   crates.io
*   Source

*   Owners
*   sfackler

*   Dependencies
*   *   async-trait ^0.1 _normal_
*   byteorder ^1.0 _normal_
*   bytes ^1.0 _normal_
*   fallible-iterator ^0.2 _normal_
*   futures-channel ^0.3 _normal_
*   futures-util ^0.3 _normal_
*   log ^0.4 _normal_
*   parking\_lot ^0.12 _normal_
*   percent-encoding ^2.0 _normal_
*   phf ^0.11 _normal_
*   pin-project-lite ^0.2 _normal_
*   postgres-protocol ^0.6.8 _normal_
*   postgres-types ^0.2.9 _normal_
*   rand ^0.9.0 _normal_
*   tokio ^1.27 _normal_
*   tokio-util ^0.7 _normal_
*   whoami ^1.4.1 _normal_
*   bit-vec ^0.6 _dev_
*   chrono ^0.4 _dev_
*   criterion ^0.5 _dev_
*   env\_logger ^0.11 _dev_
*   eui48 ^1.0 _dev_
*   futures-executor ^0.3 _dev_
*   geo-types ^0.6 _dev_
*   geo-types ^0.7 _dev_
*   jiff ^0.1 _dev_
*   serde ^1.0 _dev_
*   serde\_json ^1.0 _dev_
*   smol\_str ^0.1 _dev_
*   time ^0.2 _dev_
*   time ^0.3 _dev_
*   tokio ^1.0 _dev_
*   uuid ^0.8 _dev_
*   uuid ^1.0 _dev_
*   socket2 ^0.5 _normal_

*   Versions

*   **100%** of the crate is documented

*   Platform
*   i686-pc-windows-msvc
*   i686-unknown-linux-gnu
*   x86\_64-apple-darwin
*   x86\_64-pc-windows-msvc
*   x86\_64-unknown-linux-gnu
*   Feature flags

*   docs.rs
*   About docs.rs
*   Badges
*   Builds
*   Metadata
*   Shorthand URLs
*   Download
*   Rustdoc JSON
*   Build queue
*   Privacy policy

*   Rust
*   Rust website
*   The Book
*   Standard Library API Reference
*   Rust by Example
*   The Cargo Guide
*   Clippy Documentation

Crate tokio\_postgresCopy item path
===================================

Source

Expand description

An asynchronous, pipelined, PostgreSQL client.

§Example
--------

```
use tokio_postgres::{NoTls, Error};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
// Connect to the database.
let (client, connection) =
tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

// The connection object performs the actual communication with the database,
// so spawn it off to run on its own.
tokio::spawn(async move {
if let Err(e) = connection.await {
eprintln!("connection error: {}", e);
}
});

// Now we can execute a simple statement that just returns its parameter.
let rows = client
.query("SELECT $1::TEXT", &[&"hello world"])
.await?;

// And then check that we got back the same string we sent over.
let value: &str = rows[0].get(0);
assert_eq!(value, "hello world");

Ok(())
}
```

§Behavior
---------

Calling a method like `Client::query` on its own does nothing. The associated request is not sent to the database until the future returned by the method is first polled. Requests are executed in the order that they are first polled, not in the order that their futures are created.

§Pipelining
-----------

The client supports _pipelined_ requests. Pipelining can improve performance in use cases in which multiple, independent queries need to be executed. In a traditional workflow, each query is sent to the server after the previous query completes. In contrast, pipelining allows the client to send all of the queries to the server up front, minimizing time spent by one side waiting for the other to finish sending data:

```
Sequential                              Pipelined
| Client         | Server          |    | Client         | Server          |
|----------------|-----------------|    |----------------|-----------------|
| send query 1   |                 |    | send query 1   |                 |
|                | process query 1 |    | send query 2   | process query 1 |
| receive rows 1 |                 |    | send query 3   | process query 2 |
| send query 2   |                 |    | receive rows 1 | process query 3 |
|                | process query 2 |    | receive rows 2 |                 |
| receive rows 2 |                 |    | receive rows 3 |                 |
| send query 3   |                 |
|                | process query 3 |
| receive rows 3 |                 |
```

In both cases, the PostgreSQL server is executing the queries sequentially - pipelining just allows both sides of the connection to work concurrently when possible.

Pipelining happens automatically when futures are polled concurrently (for example, by using the futures `join` combinator):

```
use futures_util::future;
use std::future::Future;
use tokio_postgres::{Client, Error, Statement};

async fn pipelined_prepare(
client: &Client,
) -> Result<(Statement, Statement), Error>
{
future::try_join(
client.prepare("SELECT * FROM foo"),
client.prepare("INSERT INTO bar (id, name) VALUES ($1, $2)")
).await
}
```

§Runtime
--------

The client works with arbitrary `AsyncRead + AsyncWrite` streams. Convenience APIs are provided to handle the connection process, but these are gated by the `runtime` Cargo feature, which is enabled by default. If disabled, all dependence on the tokio runtime is removed.

§SSL/TLS support
----------------

TLS support is implemented via external libraries. `Client::connect` and `Config::connect` take a TLS implementation as an argument. The `NoTls` type in this crate can be used when TLS is not required. Otherwise, the `postgres-openssl` and `postgres-native-tls` crates provide implementations backed by the `openssl` and `native-tls` crates, respectively.

§Features
---------

The following features can be enabled from `Cargo.toml`:

| Feature | Description | Extra dependencies | Default |
| --- | --- | --- | --- |
| `runtime` | Enable convenience API for the connection process based on the `tokio` crate. | tokio 1.0 with the features `net` and `time` | yes |
| `array-impls` | Enables `ToSql` and `FromSql` trait impls for arrays | \- | no |
| `with-bit-vec-0_6` | Enable support for the `bit-vec` crate. | bit-vec 0.6 | no |
| `with-chrono-0_4` | Enable support for the `chrono` crate. | chrono 0.4 | no |
| `with-eui48-0_4` | Enable support for the 0.4 version of the `eui48` crate. This is deprecated and will be removed. | eui48 0.4 | no |
| `with-eui48-1` | Enable support for the 1.0 version of the `eui48` crate. | eui48 1.0 | no |
| `with-geo-types-0_6` | Enable support for the 0.6 version of the `geo-types` crate. | geo-types 0.6 | no |
| `with-geo-types-0_7` | Enable support for the 0.7 version of the `geo-types` crate. | geo-types 0.7 | no |
| `with-jiff-0_1` | Enable support for the 0.1 version of the `jiff` crate. | jiff 0.1 | no |
| `with-serde_json-1` | Enable support for the `serde_json` crate. | serde\_json 1.0 | no |
| `with-uuid-0_8` | Enable support for the `uuid` crate. | uuid 0.8 | no |
| `with-uuid-1` | Enable support for the `uuid` crate. | uuid 1.0 | no |
| `with-time-0_2` | Enable support for the 0.2 version of the `time` crate. | time 0.2 | no |
| `with-time-0_3` | Enable support for the 0.3 version of the `time` crate. | time 0.3 | no |

Re-exports§
-----------

`pub use crate::config::Config;`

`pub use crate::error::Error;`

`pub use crate::row::Row;`

`pub use crate::row::SimpleQueryRow;`

`pub use crate::tls::NoTls;`

Modules§
--------

binary\_copy

Utilities for working with the PostgreSQL binary copy format.

config

Connection configuration.

error

Errors.

row

Rows.

tls

TLS support.

types

Types.

Structs§
--------

CancelToken

The capability to request cancellation of in-progress queries on a connection.

Client

An asynchronous PostgreSQL client.

Column

Information about a column of a query.

Connection

A connection to a PostgreSQL database.

CopyInSink

A sink for `COPY ... FROM STDIN` query data.

CopyOutStream

A stream of `COPY ... TO STDOUT` query data.

Notification

An asynchronous notification.

Portal

A portal.

RowStream

A stream of table rows.

SimpleColumn

Information about a column of a single query row.

SimpleQueryStream

A stream of simple query results.

Socket

The standard stream type used by the crate.

Statement

A prepared statement.

Transaction

A representation of a PostgreSQL database transaction.

TransactionBuilder

A builder for database transactions.

Enums§
------

AsyncMessage

An asynchronous message from the server.

IsolationLevel

The isolation level of a database transaction.

SimpleQueryMessage

Message returned by the `SimpleQuery` stream.

Traits§
-------

GenericClient

A trait allowing abstraction over connections and transactions.

ToStatement

A trait abstracting over prepared and unprepared statements.

Functions§
----------

connect

A convenience function which parses a connection string and connects to the database.