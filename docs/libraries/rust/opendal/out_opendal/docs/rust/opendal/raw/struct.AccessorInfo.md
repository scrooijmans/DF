# Struct AccessorInfo Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/accessor.rs.html#615-617" class="src">Source</a>

``` rust
pub struct AccessorInfo { /* private fields */ }
```

Expand description

Info for the accessor. Users can use this struct to retrieve information about the underlying backend.

This struct is intentionally not implemented with `Clone` to ensure that all accesses within the same operator, access layers, and services use the same instance of `AccessorInfo`. This is especially important for `HttpClient` and `Executor`.

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety" class="doc-anchor">Â§</a>Panic Safety

All methods provided by `AccessorInfo` will safely handle lock poisoning scenarios. If the inner `RwLock` is poisoned (which happens when another thread panicked while holding the write lock), this method will gracefully continue execution.

- For read operations, the method will return the current state.
- For write operations, the method will do nothing.

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#maintain-notes" class="doc-anchor">Â§</a>Maintain Notes

We are using `std::sync::RwLock` to provide thread-safe access to the inner data.

I have performed [the bench across different arc-swap alike crates](https://github.com/krdln/arc-swap-benches):

``` txt
test arcswap                    ... bench:          14.85 ns/iter (+/- 0.33)
test arcswap_full               ... bench:         128.27 ns/iter (+/- 4.30)
test baseline                   ... bench:          11.33 ns/iter (+/- 0.76)
test mutex_4                    ... bench:         296.73 ns/iter (+/- 49.96)
test mutex_unconteded           ... bench:          13.26 ns/iter (+/- 0.56)
test rwlock_fast_4              ... bench:         201.60 ns/iter (+/- 7.47)
test rwlock_fast_uncontended    ... bench:          12.77 ns/iter (+/- 0.37)
test rwlock_parking_4           ... bench:         232.02 ns/iter (+/- 11.14)
test rwlock_parking_uncontended ... bench:          13.18 ns/iter (+/- 0.39)
test rwlock_std_4               ... bench:         219.56 ns/iter (+/- 5.56)
test rwlock_std_uncontended     ... bench:          13.55 ns/iter (+/- 0.33)
```

The results show that as long as there arenâ€™t too many uncontended accesses, `RwLock` is the best choice, allowing for fast access and the ability to modify partial data without cloning everything.

And itâ€™s true: we only update and modify the internal data in a few instances, such as when building an operator or applying new layers.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#impl-AccessorInfo" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.scheme" class="fn">scheme</a>(&self) -\> &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Scheme of backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-1" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply returning the current scheme.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.set_scheme" class="fn">set_scheme</a>(&self, scheme: &'static <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &Self

Set scheme for backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-2" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply skipping the update operation rather than propagating the panic.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.root" class="fn">root</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Root of backend, will be in format like `/path/to/dir/`

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-3" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply returning the current root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.set_root" class="fn">set_root</a>(&self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &Self

Set root for backend.

Note: input root must be normalized.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-4" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply skipping the update operation rather than propagating the panic.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.name" class="fn">name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>

Name of backend, could be empty if underlying backend doesnâ€™t have namespace concept.

For example:

- `s3` =\> bucket name
- `azblob` =\> container name
- `azdfs` =\> filesystem name
- `azfile` =\> share name

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-5" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply returning the current scheme.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.set_name" class="fn">set_name</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> &Self

Set name of this backend.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-6" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply skipping the update operation rather than propagating the panic.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.native_capability" class="fn">native_capability</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

Get backendâ€™s native capabilities.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-7" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply returning the current native capability.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.set_native_capability" class="fn">set_native_capability</a>(&self, capability: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>) -\> &Self

Set native capabilities for service.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#notes" class="doc-anchor">Â§</a>NOTES

Set native capability will also flush the full capability. The only way to change full_capability is via `update_full_capability`.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-8" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply skipping the update operation rather than propagating the panic.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.full_capability" class="fn">full_capability</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

Get serviceâ€™s full capabilities.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-9" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply returning the current native capability.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.update_full_capability" class="fn">update_full_capability</a>( &self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(<a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>, ) -\> &Self

Update serviceâ€™s full capabilities.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-10" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply skipping the update operation rather than propagating the panic.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.http_client" class="fn">http_client</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>

Get http client from the context.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-11" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply returning the current http client.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.update_http_client" class="fn">update_http_client</a>( &self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpClient.html" class="struct" title="struct opendal::raw::HttpClient">HttpClient</a>, ) -\> &Self

Update http client for the context.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#note" class="doc-anchor">Â§</a>Note

Requests must be forwarded to the old HTTP client after the update. Otherwise, features such as retry, tracing, and metrics may not function properly.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-12" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply skipping the update operation.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.executor" class="fn">executor</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>

Get executor from the context.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-13" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply returning the current executor.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.update_executor" class="fn">update_executor</a>(&self, f: impl <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>(<a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Executor.html" class="struct" title="struct opendal::Executor">Executor</a>) -\> &Self

Update executor for the context.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#note-1" class="doc-anchor">Â§</a>Note

Tasks must be forwarded to the old executor after the update. Otherwise, features such as retry, timeout, and metrics may not function properly.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#panic-safety-14" class="doc-anchor">Â§</a>Panic Safety

This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned, this method will gracefully continue execution by simply skipping the update operation.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#impl-Debug-for-AccessorInfo" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#impl-Default-for-AccessorInfo" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#impl-Hash-for-AccessorInfo" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.hash" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.hash_slice" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#impl-PartialEq-for-AccessorInfo" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#impl-Eq-for-AccessorInfo" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html#blanket-implementations" class="anchor">Â§</a>
