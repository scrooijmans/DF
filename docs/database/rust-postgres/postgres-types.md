Description: Conversions to and from Postgres types.

Title: postgres\_types - Rust

Docs.rs

*   postgres-types-0.2.9

*   postgres-types 0.2.9
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
*   *   array-init ^2 _normal_ _optional_
*   bit-vec ^0.6 _normal_ _optional_
*   bytes ^1.0 _normal_
*   chrono ^0.4.16 _normal_ _optional_
*   cidr ^0.2 _normal_ _optional_
*   cidr ^0.3 _normal_ _optional_
*   eui48 ^0.4 _normal_ _optional_
*   eui48 ^1.0 _normal_ _optional_
*   fallible-iterator ^0.2 _normal_
*   geo-types ^0.6 _normal_ _optional_
*   geo-types ^0.7 _normal_ _optional_
*   jiff ^0.1 _normal_ _optional_
*   postgres-derive ^0.4.6 _normal_ _optional_
*   postgres-protocol ^0.6.8 _normal_
*   serde ^1.0 _normal_ _optional_
*   serde\_json ^1.0 _normal_ _optional_
*   smol\_str ^0.1.23 _normal_ _optional_
*   time ^0.2 _normal_ _optional_
*   time ^0.3 _normal_ _optional_
*   uuid ^0.8 _normal_ _optional_
*   uuid ^1.0 _normal_ _optional_

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

Crate postgres\_typesCopy item path
===================================

Source

Expand description

Conversions to and from Postgres types.

This crate is used by the `tokio-postgres` and `postgres` crates. You normally don’t need to depend directly on it unless you want to define your own `ToSql` or `FromSql` definitions.

§Derive
-------

If the `derive` cargo feature is enabled, you can derive `ToSql` and `FromSql` implementations for custom Postgres types. Explicitly, modify your `Cargo.toml` file to include the following:

```
[dependencies]
postgres-types = { version = "0.X.X", features = ["derive"] }
```

### §Enums

Postgres enums correspond to C-like enums in Rust:

```
CREATE TYPE "Mood" AS ENUM (
'Sad',
'Ok',
'Happy'
);
```

```
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
enum Mood {
Sad,
Ok,
Happy,
}
```

### §Domains

Postgres domains correspond to tuple structs with one member in Rust:

```
CREATE DOMAIN "SessionId" AS BYTEA CHECK(octet_length(VALUE) = 16);
```

```
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
struct SessionId(Vec<u8>);
```

### §Newtypes

The `#[postgres(transparent)]` attribute can be used on a single-field tuple struct to create a Rust-only wrapper type that will use the `ToSql` & `FromSql` implementation of the inner value :

```
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(transparent)]
struct UserId(i32);
```

### §Composites

Postgres composite types correspond to structs in Rust:

```
CREATE TYPE "InventoryItem" AS (
name TEXT,
supplier_id INT,
price DOUBLE PRECISION
);
```

```
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
struct InventoryItem {
name: String,
supplier_id: i32,
price: Option<f64>,
}
```

### §Naming

The derived implementations will enforce exact matches of type, field, and variant names between the Rust and Postgres types. The `#[postgres(name = "...")]` attribute can be used to adjust the name on a type, variant, or field:

```
CREATE TYPE mood AS ENUM (
'sad',
'ok',
'happy'
);
```

```
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "mood")]
enum Mood {
#[postgres(name = "sad")]
Sad,
#[postgres(name = "ok")]
Ok,
#[postgres(name = "happy")]
Happy,
}
```

Alternatively, the `#[postgres(rename_all = "...")]` attribute can be used to rename all fields or variants with the chosen casing convention. This will not affect the struct or enum’s type name. Note that `#[postgres(name = "...")]` takes precendence when used in conjunction with `#[postgres(rename_all = "...")]`:

```
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "mood", rename_all = "snake_case")]
enum Mood {
#[postgres(name = "ok")]
Ok,             // ok
VeryHappy,      // very_happy
}
```

The following case conventions are supported:

*   `"lowercase"`
*   `"UPPERCASE"`
*   `"PascalCase"`
*   `"camelCase"`
*   `"snake_case"`
*   `"SCREAMING_SNAKE_CASE"`
*   `"kebab-case"`
*   `"SCREAMING-KEBAB-CASE"`
*   `"Train-Case"`

### §Allowing Enum Mismatches

By default the generated implementation of `ToSql` & `FromSql` for enums will require an exact match of the enum variants between the Rust and Postgres types. To allow mismatches, the `#[postgres(allow_mismatch)]` attribute can be used on the enum definition:

```
CREATE TYPE mood AS ENUM (
'Sad',
'Ok',
'Happy'
);
```

```
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(allow_mismatch)]
enum Mood {
Happy,
Meh,
}
```

Macros§
-------

accepts

Generates a simple implementation of `ToSql::accepts` which accepts the types passed to it.

to\_sql\_checked

Generates an implementation of `ToSql::to_sql_checked`.

Structs§
--------

Field

Information about a field of a composite type.

PgLsn

Postgres `PG_LSN` type.

Type

A Postgres type.

WasNull

An error indicating that a `NULL` Postgres value was passed to a `FromSql` implementation that does not support `NULL` values.

WrongType

An error indicating that a conversion was attempted between incompatible Rust and Postgres types.

Enums§
------

Date

A wrapper that can be used to represent infinity with `Type::Date` types.

Format

Supported Postgres message format types

IsNull

An enum representing the nullability of a Postgres value.

Kind

Represents the kind of a Postgres type.

Timestamp

A wrapper that can be used to represent infinity with `Type::Timestamp` and `Type::Timestamptz` types.

Traits§
-------

BorrowToSql

A trait used by clients to abstract over `&dyn ToSql` and `T: ToSql`.

FromSql

A trait for types that can be created from a Postgres value.

FromSqlOwned

A trait for types which can be created from a Postgres value without borrowing any data.

ToSql

A trait for types that can be converted into Postgres values.

Type Aliases§
-------------

Oid

A Postgres OID.