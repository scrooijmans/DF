# Struct TailCutLayerBuilder Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/layers/tail_cut.rs.html#55-62" class="src">Source</a>

``` rust
pub struct TailCutLayerBuilder { /* private fields */ }
```

Expand description

Builder for TailCutLayer.

Use this to configure the layer, then call `build()` to create a layer that can be cloned and shared across multiple operators.

## <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#examples" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::layers::TailCutLayer;
use std::time::Duration;

let layer = TailCutLayer::builder()
    .percentile(95)
    .window(Duration::from_secs(60))
    .build();

let op = Operator::new(services::Memory::default())?
    .layer(layer)
    .finish();
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#impl-TailCutLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html" class="struct" title="struct opendal::layers::TailCutLayerBuilder">TailCutLayerBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.new" class="fn">new</a>() -\> Self

Create a new builder with default settings.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.percentile" class="fn">percentile</a>(self, percentile: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Set the percentile threshold (e.g., 95 for P95, 99 for P99).

Requests slower than this percentile Ã— safety_factor will be cancelled.

Default: 95

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#panics" class="doc-anchor">Â§</a>Panics

Panics if percentile is not between 50 and 99.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.safety_factor" class="fn">safety_factor</a>(self, factor: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>) -\> Self

Set the safety factor multiplier.

The actual deadline is calculated as: P{percentile} Ã— safety_factor. A higher value reduces false positives but may miss some long tails.

Default: 1.3 (30% buffer)

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#panics-1" class="doc-anchor">Â§</a>Panics

Panics if factor is not between 1.0 and 5.0.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.window" class="fn">window</a>(self, window: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Set the sliding window duration for statistics collection.

Longer windows provide more stable statistics but react slower to changes. Shorter windows adapt faster but may be more noisy.

Default: 60 seconds

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#panics-2" class="doc-anchor">Â§</a>Panics

Panics if window is greater than 120 seconds.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.min_samples" class="fn">min_samples</a>(self, min_samples: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Set the minimum number of samples required before enabling adaptive cancellation.

During cold start (when sample count \< min_samples), the layer will not cancel any requests to avoid false positives.

Default: 200

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.min_deadline" class="fn">min_deadline</a>(self, deadline: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Set the minimum deadline (floor).

Even if calculated deadline is shorter, it will be clamped to this value. This prevents overly aggressive cancellation on very fast backends.

Default: 500ms

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.max_deadline" class="fn">max_deadline</a>(self, deadline: <a href="https://doc.rust-lang.org/nightly/core/time/struct.Duration.html" class="struct" title="struct core::time::Duration">Duration</a>) -\> Self

Set the maximum deadline (ceiling).

Even if calculated deadline is longer, it will be clamped to this value. This acts as a safety fallback timeout.

Default: 30s

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayer.html" class="struct" title="struct opendal::layers::TailCutLayer">TailCutLayer</a>

Build the layer.

The returned layer can be cloned to share statistics across operators.

##### <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#examples-1" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::layers::TailCutLayer;
use std::time::Duration;

let layer = TailCutLayer::builder()
    .percentile(95)
    .window(Duration::from_secs(60))
    .build();

// Share the layer across operators
let op1 = Operator::new(services::Memory::default())?
    .layer(layer.clone())
    .finish();

let op2 = Operator::new(services::Memory::default())?
    .layer(layer.clone())
    .finish();
// op1 and op2 share the same statistics
```

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#impl-Clone-for-TailCutLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html" class="struct" title="struct opendal::layers::TailCutLayerBuilder">TailCutLayerBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html" class="struct" title="struct opendal::layers::TailCutLayerBuilder">TailCutLayerBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#impl-Default-for-TailCutLayerBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html" class="struct" title="struct opendal::layers::TailCutLayerBuilder">TailCutLayerBuilder</a>

<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/layers/struct.TailCutLayerBuilder.html#blanket-implementations" class="anchor">Â§</a>
