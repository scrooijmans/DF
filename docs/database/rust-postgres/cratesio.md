Description: A synchronous client for the PostgreSQL database.

Title: postgres - Rust

Docs.rs

*   postgres-0.19.10

*   postgres 0.19.10
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
*   *   bytes ^1.0 _normal_
*   fallible-iterator ^0.2 _normal_
*   futures-util ^0.3.14 _normal_
*   log ^0.4 _normal_
*   tokio ^1.0 _normal_
*   tokio-postgres ^0.7.13 _normal_
*   criterion ^0.5 _dev_

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

Crate postgresCopy item path
============================

Source

Expand description

A synchronous client for the PostgreSQL database.

§Example
--------

```
use postgres::{Client, NoTls};

let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

client.batch_execute("
CREATE TABLE person (
id      SERIAL PRIMARY KEY,
name    TEXT NOT NULL,
data    BYTEA
)
")?;

let name = "Ferris";
let data = None::<&[u8]>;
client.execute(
"INSERT INTO person (name, data) VALUES ($1, $2)",
&[&name, &data],
)?;

for row in client.query("SELECT id, name, data FROM person", &[])? {
let id: i32 = row.get(0);
let name: &str = row.get(1);
let data: Option<&[u8]> = row.get(2);

println!("found person: {} {} {:?}", id, name, data);
}
```

§Implementation
---------------

This crate is a lightweight wrapper over tokio-postgres. The `postgres::Client` is simply a wrapper around a `tokio_postgres::Client` along side a tokio `Runtime`. The client simply blocks on the futures provided by the async client.

§SSL/TLS support
----------------

TLS support is implemented via external libraries. `Client::connect` and `Config::connect` take a TLS implementation as an argument. The `NoTls` type in this crate can be used when TLS is not required. Otherwise, the `postgres-openssl` and `postgres-native-tls` crates provide implementations backed by the `openssl` and `native-tls` crates, respectively.

§Features
---------

The following features can be enabled from `Cargo.toml`:

| Feature | Description | Extra dependencies | Default |
| --- | --- | --- | --- |
| `with-bit-vec-0_6` | Enable support for the `bit-vec` crate. | bit-vec 0.6 | no |
| `with-chrono-0_4` | Enable support for the `chrono` crate. | chrono 0.4 | no |
| `with-eui48-0_4` | Enable support for the 0.4 version of the `eui48` crate. This is deprecated and will be removed. | eui48 0.4 | no |
| `with-eui48-1` | Enable support for the 1.0 version of the `eui48` crate. | eui48 1.0 | no |
| `with-geo-types-0_6` | Enable support for the 0.6 version of the `geo-types` crate. | geo-types 0.6 | no |
| `with-geo-types-0_7` | Enable support for the 0.7 version of the `geo-types` crate. | geo-types 0.7 | no |
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

`pub use fallible_iterator;`

Modules§
--------

binary\_copy

Utilities for working with the PostgreSQL binary copy format.

config

Connection configuration.

error

Errors.

notifications

Asynchronous notifications.

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

A synchronous PostgreSQL client.

Column

Information about a column of a query.

CopyInWriter

The writer returned by the `copy_in` method.

CopyOutReader

The reader returned by the `copy_out` method.

Notification

An asynchronous notification.

Notifications

Notifications from a PostgreSQL backend.

Portal

A portal.

RowIter

The iterator returned by `query_raw`.

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

Title: Postgres - Rust Cookbook

Description: Collection of useful Rust code examples

*   Light
*   Rust
*   Coal
*   Navy
*   Ayu

Rust Cookbook
=============

Working with Postgres
=====================

Create tables in a Postgres database
------------------------------------

Use the `postgres` crate to create tables in a Postgres database.

`Client::connect` helps in connecting to an existing database. The recipe uses a URL string format with `Client::connect`. It assumes an existing database named `library`, the username is `postgres` and the password is `postgres`.

```rust
use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

client.batch_execute("
CREATE TABLE IF NOT EXISTS author (
id              SERIAL PRIMARY KEY,
name            VARCHAR NOT NULL,
country         VARCHAR NOT NULL
)
")?;

client.batch_execute("
CREATE TABLE IF NOT EXISTS book  (
id              SERIAL PRIMARY KEY,
title           VARCHAR NOT NULL,
author_id       INTEGER NOT NULL REFERENCES author
)
")?;

Ok(())

}
```

Insert and Query data
---------------------

The recipe inserts data into the `author` table using `execute` method of `Client`. Then, displays the data from the `author` table using `query` method of `Client`.

```rust
use postgres::{Client, NoTls, Error};
use std::collections::HashMap;

struct Author {
_id: i32,
name: String,
country: String
}

fn main() -> Result<(), Error> {
let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", 
NoTls)?;

let mut authors = HashMap::new();
authors.insert(String::from("Chinua Achebe"), "Nigeria");
authors.insert(String::from("Rabindranath Tagore"), "India");
authors.insert(String::from("Anita Nair"), "India");

for (key, value) in &authors {
let author = Author {
_id: 0,
name: key.to_string(),
country: value.to_string()
};

client.execute(
"INSERT INTO author (name, country) VALUES ($1, $2)",
&[&author.name, &author.country],
)?;
}

for row in client.query("SELECT id, name, country FROM author", &[])? {
let author = Author {
_id: row.get(0),
name: row.get(1),
country: row.get(2),
};
println!("Author {} is from {}", author.name, author.country);
}

Ok(())

}
```

Aggregate data
--------------

This recipe lists the nationalities of the first 7999 artists in the database of the Museum of Modern Art in descending order.

```rust
use postgres::{Client, Error, NoTls};

struct Nation {
nationality: String,
count: i64,
}

fn main() -> Result<(), Error> {
let mut client = Client::connect(
"postgresql://postgres:[email protected]/moma",
NoTls,
)?;

for row in client.query 
("SELECT nationality, COUNT(nationality) AS count 
FROM artists GROUP BY nationality ORDER BY count DESC", &[])? {

let (nationality, count) : (Option<String>, Option<i64>) 
= (row.get (0), row.get (1));

if nationality.is_some () && count.is_some () {

let nation = Nation{
nationality: nationality.unwrap(),
count: count.unwrap(),
};
println!("{} {}", nation.nationality, nation.count);

}
}

Ok(())
}
```