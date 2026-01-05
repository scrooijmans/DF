# Struct MetricLabels Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/observe/metrics.rs.html#151-172" class="src">Source</a>

``` rust
pub struct MetricLabels {
    pub scheme: &'static str,
    pub namespace: Arc<str>,
    pub root: Arc<str>,
    pub operation: &'static str,
    pub error: Option<ErrorKind>,
    pub status_code: Option<StatusCode>,
}
```

Expand description

MetricLabels are the labels for the metrics.

## Fields<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#fields" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#structfield.scheme" class="anchor field">Â§</a>`scheme: &'static `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

The storage scheme identifier (e.g., â€œs3â€?, â€œgcsâ€?, â€œazblobâ€?, â€œfsâ€?). Used to differentiate between different storage backends.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#structfield.namespace" class="anchor field">Â§</a>`namespace: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>`

The storage namespace (e.g., bucket name, container name). Identifies the specific storage container being accessed.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#structfield.root" class="anchor field">Â§</a>`root: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>`>`

The root path within the namespace that was configured. Used to track operations within a specific path prefix.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#structfield.operation" class="anchor field">Â§</a>`operation: &'static `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

The operation being performed (e.g., â€œreadâ€?, â€œwriteâ€?, â€œlistâ€?). Identifies which API operation generated this metric.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#structfield.error" class="anchor field">Â§</a>`error: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://opendal.apache.org/docs/rust/opendal/enum.ErrorKind.html" class="enum" title="enum opendal::ErrorKind"><code>ErrorKind</code></a>`>`

The specific error kind that occurred during an operation. Only populated for `OperationErrorsTotal` metric. Used to track frequency of specific error types.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#structfield.status_code" class="anchor field">Â§</a>`status_code: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<StatusCode>`

The HTTP status code received in an error response. Only populated for `HttpStatusErrorsTotal` metric. Used to track frequency of specific HTTP error status codes.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#impl-Clone-for-MetricLabels" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#impl-Debug-for-MetricLabels" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#impl-Default-for-MetricLabels" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#impl-Hash-for-MetricLabels" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#method.hash" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#method.hash_slice" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#impl-PartialEq-for-MetricLabels" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#impl-Eq-for-MetricLabels" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#impl-StructuralPartialEq-for-MetricLabels" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html" class="struct" title="struct opendal::layers::observe::MetricLabels">MetricLabels</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/observe/struct.MetricLabels.html#blanket-implementations" class="anchor">Â§</a>
