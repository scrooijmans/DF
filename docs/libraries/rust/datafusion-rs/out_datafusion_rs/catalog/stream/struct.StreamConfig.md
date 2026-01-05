# Struct StreamConfig Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/stream.rs.html#247" class="src">Source</a>

``` rust
pub struct StreamConfig { /* private fields */ }
```

Expand description

The configuration for a [`StreamTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamTable.html "struct datafusion::datasource::stream::StreamTable")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#impl-StreamConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamConfig.html" class="struct" title="struct datafusion::datasource::stream::StreamConfig">StreamConfig</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#method.new" class="fn">new</a>(source: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/trait.StreamProvider.html" class="trait" title="trait datafusion::datasource::stream::StreamProvider">StreamProvider</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamConfig.html" class="struct" title="struct datafusion::datasource::stream::StreamConfig">StreamConfig</a>

Create a new `StreamConfig` from a `StreamProvider`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#method.with_order" class="fn">with_order</a>(self, order: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamConfig.html" class="struct" title="struct datafusion::datasource::stream::StreamConfig">StreamConfig</a>

Specify a sort order for the stream

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#method.with_constraints" class="fn">with_constraints</a>(self, constraints: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Constraints.html" class="struct" title="struct datafusion::common::Constraints">Constraints</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamConfig.html" class="struct" title="struct datafusion::datasource::stream::StreamConfig">StreamConfig</a>

Assign constraints

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#impl-Debug-for-StreamConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/stream/struct.StreamConfig.html" class="struct" title="struct datafusion::datasource::stream::StreamConfig">StreamConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/stream/struct.StreamConfig.html#blanket-implementations" class="anchor">§</a>
