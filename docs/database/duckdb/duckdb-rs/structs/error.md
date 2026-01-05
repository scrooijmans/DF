# Error in duckdb - Rust

```

#[non_exhaustive]pub enum Error {
Show 20 variants    DuckDBFailure(Error, Option<String>),
    FromSqlConversionFailure(usize, Type, Box<dyn Error + Send + Sync + 'static>),
    IntegralValueOutOfRange(usize, i128),
    Utf8Error(Utf8Error),
    NulError(NulError),
    InvalidParameterName(String),
    InvalidPath(PathBuf),
    ExecuteReturnedResults,
    QueryReturnedNoRows,
    QueryReturnedMoreThanOneRow,
    InvalidColumnIndex(usize),
    InvalidColumnName(String),
    InvalidColumnType(usize, String, Type),
    ArrowTypeToDuckdbType(String, DataType),
    StatementChangedRows(usize),
    ToSqlConversionFailure(Box<dyn Error + Send + Sync + 'static>),
    InvalidQuery,
    MultipleStatement,
    InvalidParameterCount(usize, usize),
    AppendError,
}
```

Expand description

Enum listing possible errors from duckdb.

This enum is marked as non-exhaustive

Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.

[§](#variant.DuckDBFailure)

An error from an underlying DuckDB call.

[§](#variant.FromSqlConversionFailure)

Error when the value of a particular column is requested, but it cannot be converted to the requested Rust type.

[§](#variant.IntegralValueOutOfRange)

Error when DuckDB gives us an integral value outside the range of the requested type (e.g., trying to get the value 1000 into a `u8`). The associated `usize` is the column index, and the associated `i64` is the value returned by SQLite.

[§](#variant.Utf8Error)

Error converting a string to UTF-8.

[§](#variant.NulError)

Error converting a string to a C-compatible string because it contained an embedded nul.

[§](#variant.InvalidParameterName)

Error when using SQL named parameters and passing a parameter name not present in the SQL.

[§](#variant.InvalidPath)

Error converting a file path to a string.

[§](#variant.ExecuteReturnedResults)

Error returned when an [`execute`](about:blank/struct.Connection.html#method.execute "method duckdb::Connection::execute") call returns rows.

[§](#variant.QueryReturnedNoRows)

Error when a query that was expected to return at least one row (e.g., for [`query_row`](about:blank/struct.Connection.html#method.query_row "method duckdb::Connection::query_row")) did not return any.

[§](#variant.QueryReturnedMoreThanOneRow)

Error when a query that was expected to return only one row (e.g., for [`query_one`](crate::Connection::query_one)) did return more than one.

[§](#variant.InvalidColumnIndex)

Error when the value of a particular column is requested, but the index is out of range for the statement.

[§](#variant.InvalidColumnName)

Error when the value of a named column is requested, but no column matches the name for the statement.

[§](#variant.InvalidColumnType)

Error when the value of a particular column is requested, but the type of the result in that column cannot be converted to the requested Rust type.

[§](#variant.ArrowTypeToDuckdbType)

Error when datatype to duckdb type

[§](#variant.StatementChangedRows)

Error when a query that was expected to insert one row did not insert any or insert many.

[§](#variant.ToSqlConversionFailure)

Error available for the implementors of the [`ToSql`](types/trait.ToSql.html "trait duckdb::types::ToSql") trait.

[§](#variant.InvalidQuery)

Error when the SQL is not a `SELECT`, is not read-only.

[§](#variant.MultipleStatement)

Error when the SQL contains multiple statements.

[§](#variant.InvalidParameterCount)

Error when the number of bound parameters does not match the number of parameters in the query. The first `usize` is how many parameters were given, the 2nd is how many were expected.

[§](#variant.AppendError)

Append Error

[Source](about:blank/src/duckdb/error.rs.html#11)
[§](#impl-Debug-for-Error)

[Source](about:blank/src/duckdb/error.rs.html#151-197)
[§](#impl-Display-for-Error)

[Source](about:blank/src/duckdb/error.rs.html#199-224)
[§](#impl-Error-for-Error)

[Source](about:blank/src/duckdb/error.rs.html#200-223)
[§](#method.source)

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#137)
[§](#method.description)

👎Deprecated since 1.42.0: use the Display impl or to_string()

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#147)
[§](#method.cause)

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#210)
[§](#method.provide)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

[Source](about:blank/src/duckdb/error.rs.html#134-149)
[§](#impl-From%3CFromSqlError%3E-for-Error)

The conversion isn’t precise, but it’s convenient to have it to allow use of `get_raw(…).as_…()?` in callbacks that take `Error`.

[Source](about:blank/src/duckdb/error.rs.html#136-148)
[§](#method.from-2)

Converts to this type from the input type.

[Source](about:blank/src/duckdb/error.rs.html#123-128)
[§](#impl-From%3CNulError%3E-for-Error)

[Source](about:blank/src/duckdb/error.rs.html#125-127)
[§](#method.from-1)

Converts to this type from the input type.

[Source](about:blank/src/duckdb/error.rs.html#116-121)
[§](#impl-From%3CUtf8Error%3E-for-Error)

[Source](about:blank/src/duckdb/error.rs.html#118-120)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/duckdb/error.rs.html#92-114)
[§](#impl-PartialEq-for-Error)

[Source](about:blank/src/duckdb/error.rs.html#93-113)
[§](#method.eq)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264)
[§](#method.ne)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[§](#impl-Freeze-for-Error)

[§](#impl-RefUnwindSafe-for-Error)

[§](#impl-Send-for-Error)

[§](#impl-Sync-for-Error)

[§](#impl-Unpin-for-Error)

[§](#impl-UnwindSafe-for-Error)
