Apache Arrow Flight
Crates.io

See the API documentation for examples and the full API.

The API documentation for most recent, unreleased code is available here.

Usage
Add this to your Cargo.toml:

[dependencies]
arrow-flight = "54.0.0"
Apache Arrow Flight is a gRPC based protocol for exchanging Arrow data between processes. See the blog post Introducing Apache Arrow Flight: A Framework for Fast Data Transport for more information.

This crate provides a Rust implementation of the Flight.proto gRPC protocol and examples that demonstrate how to build a Flight server implemented with tonic.

Feature Flags
flight-sql-experimental: Enables experimental support for Apache Arrow FlightSQL, a protocol for interacting with SQL databases.

tls: Enables tls on tonic

CLI
This crates offers a basic Apache Arrow FlightSQL command line interface.

The client can be installed from the repository:

$ cargo install --features=cli,flight-sql-experimental,tls --bin=flight_sql_client --path=. --locked
The client comes with extensive help text:

$ flight_sql_client help
A query can be executed using:

$ flight_sql_client --host example.com statement-query "SELECT 1;"
+----------+
| Int64(1) |
+----------+
| 1        |
+----------+