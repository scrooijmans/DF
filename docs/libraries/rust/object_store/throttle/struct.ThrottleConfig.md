# Struct ThrottleConfig Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/throttle.rs.html#36-98" class="src">Source</a>

``` rust
pub struct ThrottleConfig {
    pub wait_delete_per_call: Duration,
    pub wait_get_per_byte: Duration,
    pub wait_get_per_call: Duration,
    pub wait_list_per_call: Duration,
    pub wait_list_per_entry: Duration,
    pub wait_list_with_delimiter_per_call: Duration,
    pub wait_list_with_delimiter_per_entry: Duration,
    pub wait_put_per_call: Duration,
}
```

Expand description

Configuration settings for throttled store

## Fields<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_delete_per_call" class="anchor field">§</a>`wait_delete_per_call: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

Sleep duration for every call to [`delete`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html#method.delete "method object_store::throttle::ThrottledStore::delete").

Sleeping is done before the underlying store is called and independently of the success of the operation.

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_get_per_byte" class="anchor field">§</a>`wait_get_per_byte: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

Sleep duration for every byte received during [`get`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html#method.get "method object_store::throttle::ThrottledStore::get").

Sleeping is performed after the underlying store returned and only for successful gets. The sleep duration is additive to [`wait_get_per_call`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_get_per_call "field object_store::throttle::ThrottleConfig::wait_get_per_call").

Note that the per-byte sleep only happens as the user consumes the output bytes. Should there be an intermediate failure (i.e. after partly consuming the output bytes), the resulting sleep time will be partial as well.

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_get_per_call" class="anchor field">§</a>`wait_get_per_call: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

Sleep duration for every call to [`get`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html#method.get "method object_store::throttle::ThrottledStore::get").

Sleeping is done before the underlying store is called and independently of the success of the operation. The sleep duration is additive to [`wait_get_per_byte`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_get_per_byte "field object_store::throttle::ThrottleConfig::wait_get_per_byte").

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_list_per_call" class="anchor field">§</a>`wait_list_per_call: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

Sleep duration for every call to [`list`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html#method.list "method object_store::throttle::ThrottledStore::list").

Sleeping is done before the underlying store is called and independently of the success of the operation. The sleep duration is additive to [`wait_list_per_entry`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_list_per_entry "field object_store::throttle::ThrottleConfig::wait_list_per_entry").

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_list_per_entry" class="anchor field">§</a>`wait_list_per_entry: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

Sleep duration for every entry received during [`list`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html#method.list "method object_store::throttle::ThrottledStore::list").

Sleeping is performed after the underlying store returned and only for successful lists. The sleep duration is additive to [`wait_list_per_call`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_list_per_call "field object_store::throttle::ThrottleConfig::wait_list_per_call").

Note that the per-entry sleep only happens as the user consumes the output entries. Should there be an intermediate failure (i.e. after partly consuming the output entries), the resulting sleep time will be partial as well.

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_list_with_delimiter_per_call" class="anchor field">§</a>`wait_list_with_delimiter_per_call: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

Sleep duration for every call to [`list_with_delimiter`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html#method.list_with_delimiter "method object_store::throttle::ThrottledStore::list_with_delimiter").

Sleeping is done before the underlying store is called and independently of the success of the operation. The sleep duration is additive to [`wait_list_with_delimiter_per_entry`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_list_with_delimiter_per_entry "field object_store::throttle::ThrottleConfig::wait_list_with_delimiter_per_entry").

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_list_with_delimiter_per_entry" class="anchor field">§</a>`wait_list_with_delimiter_per_entry: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

Sleep duration for every entry received during [`list_with_delimiter`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html#method.list_with_delimiter "method object_store::throttle::ThrottledStore::list_with_delimiter").

Sleeping is performed after the underlying store returned and only for successful gets. The sleep duration is additive to [`wait_list_with_delimiter_per_call`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_list_with_delimiter_per_call "field object_store::throttle::ThrottleConfig::wait_list_with_delimiter_per_call").

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#structfield.wait_put_per_call" class="anchor field">§</a>`wait_put_per_call: `<a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration"><code>Duration</code></a>

Sleep duration for every call to [`put`](https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottledStore.html#method.put "method object_store::throttle::ThrottledStore::put").

Sleeping is done before the underlying store is called and independently of the success of the operation.

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#impl-Clone-for-ThrottleConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html" class="struct" title="struct object_store::throttle::ThrottleConfig">ThrottleConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html" class="struct" title="struct object_store::throttle::ThrottleConfig">ThrottleConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#impl-Debug-for-ThrottleConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html" class="struct" title="struct object_store::throttle::ThrottleConfig">ThrottleConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#impl-Default-for-ThrottleConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html" class="struct" title="struct object_store::throttle::ThrottleConfig">ThrottleConfig</a>

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html" class="struct" title="struct object_store::throttle::ThrottleConfig">ThrottleConfig</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#impl-Copy-for-ThrottleConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html" class="struct" title="struct object_store::throttle::ThrottleConfig">ThrottleConfig</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/throttle/struct.ThrottleConfig.html#blanket-implementations" class="anchor">§</a>
