# Struct BackoffConfig Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/client/backoff.rs.html#31-38" class="src">Source</a>

``` rust
pub struct BackoffConfig {
    pub init_backoff: Duration,
    pub max_backoff: Duration,
    pub base: f64,
}
```

Available on **crate feature `cloud`** only.

Expand description

Exponential backoff with decorrelated jitter algorithm

The first backoff will always be `init_backoff`.

Subsequent backoffs will pick a random value between `init_backoff` and `base * previous` where `previous` is the duration of the previous backoff

See <https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/>

## Fields<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#structfield.init_backoff" class="anchor field">§</a>`init_backoff: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

The initial backoff duration

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#structfield.max_backoff" class="anchor field">§</a>`max_backoff: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

The maximum backoff duration

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#structfield.base" class="anchor field">§</a>`base: `<a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive"><code>f64</code></a>

The multiplier to use for the next backoff duration

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#impl-Clone-for-BackoffConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html" class="struct" title="struct object_store::BackoffConfig">BackoffConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html" class="struct" title="struct object_store::BackoffConfig">BackoffConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#impl-Debug-for-BackoffConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html" class="struct" title="struct object_store::BackoffConfig">BackoffConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#impl-Default-for-BackoffConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html" class="struct" title="struct object_store::BackoffConfig">BackoffConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.BackoffConfig.html#blanket-implementations" class="anchor">§</a>
