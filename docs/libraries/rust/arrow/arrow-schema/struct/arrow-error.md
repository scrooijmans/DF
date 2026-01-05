# ArrowError in arrow_schema - Rust

```
pub enum ArrowError {
Show 19 variants    NotYetImplemented(String),
    ExternalError(Box<dyn Error + Send + Sync>),
    CastError(String),
    MemoryError(String),
    ParseError(String),
    SchemaError(String),
    ComputeError(String),
    DivideByZero,
    ArithmeticOverflow(String),
    CsvError(String),
    JsonError(String),
    IoError(String, Error),
    IpcError(String),
    InvalidArgumentError(String),
    ParquetError(String),
    CDataInterface(String),
    DictionaryKeyOverflowError,
    RunEndIndexOverflowError,
    OffsetOverflowError(usize),
}
```

Expand description

Many different operations in the `arrow` crate return this error type.

[§](#variant.NotYetImplemented)

Returned when functionality is not yet available.

[§](#variant.ExternalError)

Wraps an external error.

[§](#variant.CastError)

Error during casting from one type to another.

[§](#variant.MemoryError)

Memory or buffer error.

[§](#variant.ParseError)

Error during parsing from a string.

[§](#variant.SchemaError)

Error during schema-related operations.

[§](#variant.ComputeError)

Error during computation.

[§](#variant.DivideByZero)

Error during division by zero.

[§](#variant.ArithmeticOverflow)

Error when an arithmetic operation overflows.

[§](#variant.CsvError)

Error during CSV-related operations.

[§](#variant.JsonError)

Error during JSON-related operations.

[§](#variant.IoError)

Error during IO operations.

[§](#variant.IpcError)

Error during IPC operations in `arrow-ipc` or `arrow-flight`.

[§](#variant.InvalidArgumentError)

Error indicating that an unexpected or bad argument was passed to a function.

[§](#variant.ParquetError)

Error during Parquet operations.

[§](#variant.CDataInterface)

Error during import or export to/from the C Data Interface

[§](#variant.DictionaryKeyOverflowError)

Error when a dictionary key is bigger than the key type

[§](#variant.RunEndIndexOverflowError)

Error when the run end index in a REE array is bigger than the array length

[§](#variant.OffsetOverflowError)

Error when the offset overflows.

[Source](about:blank/src/arrow_schema/error.rs.html#25)
[§](#impl-Debug-for-ArrowError)

[Source](about:blank/src/arrow_schema/error.rs.html#98-136)
[§](#impl-Display-for-ArrowError)

[Source](about:blank/src/arrow_schema/error.rs.html#138-146)
[§](#impl-Error-for-ArrowError)

[Source](about:blank/src/arrow_schema/error.rs.html#139-145)
[§](#method.source)

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#131)
[§](#method.description)

👎Deprecated since 1.42.0: use the Display impl or to_string()

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#141)
[§](#method.cause)

���Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

[Source](https://doc.rust-lang.org/nightly/src/core/error.rs.html#204)
[§](#method.provide)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

[Source](about:blank/src/arrow_schema/error.rs.html#74-78)
[§](#impl-From%3CError%3E-for-ArrowError)

[Source](about:blank/src/arrow_schema/error.rs.html#75-77)
[§](#method.from)

Converts to this type from the input type.

[Source](about:blank/src/arrow_schema/error.rs.html#86-90)
[§](#impl-From%3CFromUtf8Error%3E-for-ArrowError)

[Source](about:blank/src/arrow_schema/error.rs.html#87-89)
[§](#method.from-2)

Converts to this type from the input type.

[Source](about:blank/src/arrow_schema/error.rs.html#92-96)
[§](#impl-From%3CIntoInnerError%3CW%3E%3E-for-ArrowError)

[Source](about:blank/src/arrow_schema/error.rs.html#93-95)
[§](#method.from-3)

Converts to this type from the input type.

[Source](about:blank/src/arrow_schema/error.rs.html#80-84)
[§](#impl-From%3CUtf8Error%3E-for-ArrowError)

[Source](about:blank/src/arrow_schema/error.rs.html#81-83)
[§](#method.from-1)

Converts to this type from the input type.

[§](#impl-Freeze-for-ArrowError)

[§](#impl-RefUnwindSafe-for-ArrowError)

[§](#impl-Send-for-ArrowError)

[§](#impl-Sync-for-ArrowError)

[§](#impl-Unpin-for-ArrowError)

[§](#impl-UnwindSafe-for-ArrowError)
