# Trait LoggingInterceptor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/logging.rs.html#146-170" class="src">Source</a>

``` rust
pub trait LoggingInterceptor:
    Debug
    + Clone
    + Send
    + Sync
    + Unpin
    + 'static {
    // Required method
    fn log(
        &self,
        info: &AccessorInfo,
        operation: Operation,
        context: &[(&str, &str)],
        message: &str,
        err: Option<&Error>,
    );
}
```

Expand description

LoggingInterceptor is used to intercept the log.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html#tymethod.log" class="fn">log</a>( &self, info: &<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>, operation: <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.Operation.html" class="enum" title="enum opendal::raw::Operation">Operation</a>, context: &\[(&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)\], message: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, err: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>\>, )

Everytime there is a log, this function will be called.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html#inputs" class="doc-anchor">Â§</a>Inputs

- info: The serviceâ€™s access info.
- operation: The operation to log.
- context: Additional context of the log like path, etc.
- message: The log message.
- err: The error to log.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html#note" class="doc-anchor">Â§</a>Note

Users should avoid calling resource-intensive operations such as I/O or network functions here, especially anything that takes longer than 10ms. Otherwise, Opendal could perform unexpectedly slow.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.LoggingInterceptor.html#implementors" class="anchor">Â§</a>
