# Enum DataFusionError Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/error.rs.html#52" class="src">Source</a>

``` rust
pub enum DataFusionError {
Show 20 variants    ArrowError(Box<ArrowError>, Option<String>),
    ParquetError(Box<ParquetError>),
    AvroError(Box<Error>),
    ObjectStore(Box<Error>),
    IoError(Error),
    SQL(Box<ParserError>, Option<String>),
    NotImplemented(String),
    Internal(String),
    Plan(String),
    Configuration(String),
    SchemaError(Box<SchemaError>, Box<Option<String>>),
    Execution(String),
    ExecutionJoin(Box<JoinError>),
    ResourcesExhausted(String),
    External(Box<dyn Error + Sync + Send>),
    Context(String, Box<DataFusionError>),
    Substrait(String),
    Diagnostic(Box<Diagnostic>, Box<DataFusionError>),
    Collection(Vec<DataFusionError>),
    Shared(Arc<DataFusionError>),
}
```

Expand description

DataFusion error

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.ArrowError" class="anchor">§</a>

### ArrowError(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

Error returned by arrow.

2nd argument is for optional backtrace

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.ParquetError" class="anchor">§</a>

### ParquetError(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/errors/enum.ParquetError.html" class="enum" title="enum parquet::errors::ParquetError">ParquetError</a>\>)

Error when reading / writing Parquet data.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.AvroError" class="anchor">§</a>

### AvroError(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/apache-avro/0.20.0/x86_64-unknown-linux-gnu/apache_avro/error/struct.Error.html" class="struct" title="struct apache_avro::error::Error">Error</a>\>)

Error when reading Avro data.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.ObjectStore" class="anchor">§</a>

### ObjectStore(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\>)

Error when reading / writing to / from an object_store (e.g. S3 or LocalFile)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.IoError" class="anchor">§</a>

### IoError(<a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>)

Error when an I/O operation fails

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.SQL" class="anchor">§</a>

### SQL(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

Error when SQL is syntactically incorrect.

2nd argument is for optional backtrace

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.NotImplemented" class="anchor">§</a>

### NotImplemented(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Error when a feature is not yet implemented.

These errors are sometimes returned for features that are still in development and are not entirely complete. Often, these errors are tracked in our issue tracker.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Internal" class="anchor">§</a>

### Internal(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Error due to bugs in DataFusion

This error should not happen in normal usage of DataFusion. It results from something that wasn’t expected/anticipated by the implementation and that is most likely a bug (the error message even encourages users to open a bug report). A user should not be able to trigger internal errors under normal circumstances by feeding in malformed queries, bad data, etc.

Note that I/O errors (or any error that happens due to external systems) do NOT fall under this category. See other variants such as [`Self::IoError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html#variant.IoError "variant datafusion::error::DataFusionError::IoError") and [`Self::External`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html#variant.External "variant datafusion::error::DataFusionError::External").

DataFusions has internal invariants that the compiler is not always able to check. This error is raised when one of those invariants does not hold for some reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Plan" class="anchor">§</a>

### Plan(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Error during planning of the query.

This error happens when the user provides a bad query or plan, for example the user attempts to call a function that doesn’t exist, or if the types of a function call are not supported.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Configuration" class="anchor">§</a>

### Configuration(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Error for invalid or unsupported configuration options.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.SchemaError" class="anchor">§</a>

### SchemaError(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.SchemaError.html" class="enum" title="enum datafusion::common::SchemaError">SchemaError</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>)

Error when there is a problem with the query related to schema.

This error can be returned in cases such as when schema inference is not possible and when column names are not unique.

2nd argument is for optional backtrace Boxing the optional backtrace to prevent <https://rust-lang.github.io/rust-clippy/master/index.html#/result_large_err>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Execution" class="anchor">§</a>

### Execution(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Error during execution of the query.

This error is returned when an error happens during execution due to a malformed input. For example, the user passed malformed arguments to a SQL method, opened a CSV file that is broken, or tried to divide an integer by zero.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.ExecutionJoin" class="anchor">§</a>

### ExecutionJoin(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html" class="struct" title="struct tokio::runtime::task::error::JoinError">JoinError</a>\>)

[`JoinError`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/task/error/struct.JoinError.html "struct tokio::runtime::task::error::JoinError") during execution of the query.

This error can’t occur for unjoined tasks, such as execution shutdown.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.ResourcesExhausted" class="anchor">§</a>

### ResourcesExhausted(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Error when resources (such as memory of scratch disk space) are exhausted.

This error is thrown when a consumer cannot acquire additional memory or other resources needed to execute the query from the Memory Manager.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.External" class="anchor">§</a>

### External(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>)

Errors originating from outside DataFusion’s core codebase.

For example, a custom S3Error from the crate datafusion-objectstore-s3

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Context" class="anchor">§</a>

### Context(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>)

Error with additional context

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Substrait" class="anchor">§</a>

### Substrait(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Errors from either mapping LogicalPlans to/from Substrait plans or serializing/deserializing protobytes to Substrait plans

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Diagnostic" class="anchor">§</a>

### Diagnostic(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>)

Error wrapped together with additional contextual information intended for end users, to help them understand what went wrong by providing human-readable messages, and locations in the source query that relate to the error in some way.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Collection" class="anchor">§</a>

### Collection(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>)

A collection of one or more [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError"). Useful in cases where DataFusion can recover from an erroneous state, and produce more errors before terminating. e.g. when planning a SELECT clause, DataFusion can synchronize to the next `SelectItem` if the previous one had errors. The end result is that the user can see errors about all `SelectItem`, instead of just the first one.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#variant.Shared" class="anchor">§</a>

### Shared(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>)

A [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError") which shares an underlying [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError").

This is useful when the same underlying [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError") is passed to multiple receivers. For example, when the source of a repartition errors and the error is propagated to multiple consumers.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-DataFusionError" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

#### pub const <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#associatedconstant.BACK_TRACE_SEP" class="constant">BACK_TRACE_SEP</a>: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a> = "\n\nbacktrace: "

The separator between the error message and the backtrace

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.find_root" class="fn">find_root</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Get deepest underlying [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")

[`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")s sometimes form a chain, such as `DataFusionError::ArrowError()` in order to conform to the correct error signature. Thus sometimes there is a chain several layers deep that can obscure the original error. This function finds the lowest level DataFusionError possible.

For example, `find_root` will return`DataFusionError::ResourceExhausted` given the input

``` text
DataFusionError::ArrowError
  ArrowError::External
   Box(DataFusionError::Context)
     DataFusionError::ResourceExhausted
```

This may be the same as `self`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.context" class="fn">context</a>(self, description: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

wraps self in Self::Context with a description

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.strip_backtrace" class="fn">strip_backtrace</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Strips backtrace out of the error message If backtrace enabled then error has a format “message” [`Self::BACK_TRACE_SEP`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html#associatedconstant.BACK_TRACE_SEP "associated constant datafusion::error::DataFusionError::BACK_TRACE_SEP") “backtrace” The method strips the backtrace and outputs “message”

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.get_back_trace" class="fn">get_back_trace</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

To enable optional rust backtrace in DataFusion:

- \[`Setup Env Variables`\]<https://doc.rust-lang.org/std/backtrace/index.html#environment-variables>
- Enable `backtrace` cargo feature

Example: cargo build –features ‘backtrace’ RUST_BACKTRACE=1 ./app

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.builder" class="fn">builder</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html" class="struct" title="struct datafusion::common::error::DataFusionErrorBuilder">DataFusionErrorBuilder</a>

Return a [`DataFusionErrorBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/common/error/struct.DataFusionErrorBuilder.html "struct datafusion::common::error::DataFusionErrorBuilder") to build a [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.message" class="fn">message</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html" class="enum" title="enum alloc::borrow::Cow">Cow</a>\<'\_, <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.with_diagnostic" class="fn">with_diagnostic</a>(self, diagnostic: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Wraps the error with contextual information intended for end users

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.with_diagnostic_fn" class="fn">with_diagnostic_fn</a>\<F\>(self, f: F) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(&<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>,

Wraps the error with contextual information intended for end users. Takes a function that inspects the error and returns the diagnostic to wrap it with.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.diagnostic" class="fn">diagnostic</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>\>

Gets the [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") associated with the error, if any. If there is more than one, only the outermost [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") is returned.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.iter" class="fn">iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = &<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return an iterator over this [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError") and any other [`DataFusionError`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html "enum datafusion::error::DataFusionError")s in a [`DataFusionError::Collection`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html#variant.Collection "variant datafusion::error::DataFusionError::Collection").

Sometimes DataFusion is able to collect multiple errors in a SQL query before terminating, e.g. across different expressions in a SELECT statements or different sides of a UNION. This method returns an iterator over all the errors in the collection.

For this to work, the top-level error must be a `DataFusionError::Collection`, not something that contains it.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-Debug-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-Display-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-Error-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.source" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source" class="fn">source</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&(dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + 'static)\>

Returns the lower-level source of this error, if any. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.source)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#137" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.description" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description" class="fn">description</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

👎Deprecated since 1.42.0: use the Display impl or to_string()

[Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.description)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/error.rs.html#147" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.cause" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.cause" class="fn">cause</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a>\>

👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.provide" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide" class="fn">provide</a>\<'a\>(&'a self, request: &mut <a href="https://doc.rust-lang.org/nightly/core/error/struct.Request.html" class="struct" title="struct core::error::Request">Request</a>\<'a\>)

🔬This is a nightly-only experimental API. (`error_generic_member_access`)

Provides type-based access to context intended for error reports. [Read more](https://doc.rust-lang.org/nightly/core/error/trait.Error.html#method.provide)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3C%26Arc%3CDataFusionError%3E%3E-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-4" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CArrowError%3E-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CBox%3Cdyn+Error+%2B+Send+%2B+Sync%3E%3E-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-10" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(err: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/error/trait.Error.html" class="trait" title="trait core::error::Error">Error</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CBuilderError%3E-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-11" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CDataFusionError%3E-for-ArrowError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CError%3E-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(\_e: <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CError%3E-for-DataFusionError-1" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CError%3E-for-DataFusionError-2" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/apache-avro/0.20.0/x86_64-unknown-linux-gnu/apache_avro/error/struct.Error.html" class="struct" title="struct apache_avro::error::Error">Error</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-6" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/apache-avro/0.20.0/x86_64-unknown-linux-gnu/apache_avro/error/struct.Error.html" class="struct" title="struct apache_avro::error::Error">Error</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CError%3E-for-DataFusionError-3" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-7" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/enum.Error.html" class="enum" title="enum object_store::Error">Error</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CError%3E-for-DataFusionError-4" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-8" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/enum.Error.html" class="enum" title="enum object_store::path::Error">Error</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CParquetError%3E-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/errors/enum.ParquetError.html" class="enum" title="enum parquet::errors::ParquetError">ParquetError</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-5" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/parquet/56.0.0/x86_64-unknown-linux-gnu/parquet/errors/enum.ParquetError.html" class="enum" title="enum parquet::errors::ParquetError">ParquetError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#impl-From%3CParserError%3E-for-DataFusionError" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#method.from-9" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(e: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/error/enum.DataFusionError.html#blanket-implementations" class="anchor">§</a>
