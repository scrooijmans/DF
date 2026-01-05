# Trait RetryInterceptor Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/retry.rs.html#239-256" class="src">Source</a>

``` rust
pub trait RetryInterceptor:
    Send
    + Sync
    + 'static {
    // Required method
    fn intercept(&self, err: &Error, dur: Duration);
}
```

Expand description

RetryInterceptor is used to intercept while retry happened.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html#tymethod.intercept" class="fn">intercept</a>(&self, err: &<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>, dur: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>)

Everytime RetryLayer is retrying, this function will be called.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html#timing" class="doc-anchor">Â§</a>Timing

just before the retry sleep.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html#inputs" class="doc-anchor">Â§</a>Inputs

- err: The error that caused the current retry.
- dur: The duration that will sleep before next retry.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html#notes" class="doc-anchor">Â§</a>Notes

The intercept must be quick and non-blocking. No heavy IO is allowed. Otherwise, the retry will be blocked.

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html#impl-RetryInterceptor-for-F" class="anchor">Â§</a>

### impl\<F\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/trait.RetryInterceptor.html" class="trait" title="trait opendal::layers::RetryInterceptor">RetryInterceptor</a> for F

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&<a href="https://opendal.apache.org/docs/rust/opendal/struct.Error.html" class="struct" title="struct opendal::Error">Error</a>, <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,
