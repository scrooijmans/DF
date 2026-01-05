# Struct RetryConfig Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/retry.rs.html#225-246" class="src">Source</a>

``` rust
pub struct RetryConfig {
    pub backoff: BackoffConfig,
    pub max_retries: usize,
    pub retry_timeout: Duration,
}
```

Available on **crate feature `cloud`** only.

Expand description

The configuration for how to respond to request errors

The following categories of error will be retried:

- 5xx server errors
- Connection errors
- Dropped connections
- Timeouts for [safe](https://datatracker.ietf.org/doc/html/rfc7231#section-4.2.1) / read-only requests

Requests will be retried up to some limit, using exponential backoff with jitter. See [`BackoffConfig`](https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html "struct object_store::BackoffConfig") for more information

## Fields<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#structfield.backoff" class="anchor field">§</a>`backoff: `<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html" class="struct" title="struct object_store::BackoffConfig"><code>BackoffConfig</code></a>

The backoff configuration

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#structfield.max_retries" class="anchor field">§</a>`max_retries: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

The maximum number of times to retry a request

Set to 0 to disable retries

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#structfield.retry_timeout" class="anchor field">§</a>`retry_timeout: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

The maximum length of time from the initial request after which no further retries will be attempted

This not only bounds the length of time before a server error will be surfaced to the application, but also bounds the length of time a request’s credentials must remain valid.

As requests are retried without renewing credentials or regenerating request payloads, this number should be kept below 5 minutes to avoid errors due to expired credentials and/or request payloads

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#impl-Clone-for-RetryConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#impl-Debug-for-RetryConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#impl-Default-for-RetryConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html" class="struct" title="struct object_store::RetryConfig">RetryConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.RetryConfig.html#blanket-implementations" class="anchor">§</a>
