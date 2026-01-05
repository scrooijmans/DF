# Struct Monoiofs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/monoiofs/backend.rs.html#36-38" class="src">Source</a>

``` rust
pub struct Monoiofs { /* private fields */ }
```

Expand description

File system support via \[`monoio`\].

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ append
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☐ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.

You can refer to [`MonoiofsBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html "struct opendal::services::Monoiofs")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Monoiofs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create monoiofs backend builder.
    let mut builder = Monoiofs::default()
        // Set the root for monoiofs, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/tmp");

    // `Accessor` provides the low level APIs, we will use `Operator` normally.
    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#impl-MonoiofsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html" class="struct" title="struct opendal::services::Monoiofs">MonoiofsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-monoiofs`** only.

Set root of this backend.

All operations will happen under this root.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#impl-Builder-for-MonoiofsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html" class="struct" title="struct opendal::services::Monoiofs">MonoiofsBuilder</a>

Available on **crate feature `services-monoiofs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.MonoiofsConfig.html" class="struct" title="struct opendal::services::MonoiofsConfig">MonoiofsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#impl-Debug-for-MonoiofsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html" class="struct" title="struct opendal::services::Monoiofs">MonoiofsBuilder</a>

Available on **crate feature `services-monoiofs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#impl-Default-for-MonoiofsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html" class="struct" title="struct opendal::services::Monoiofs">MonoiofsBuilder</a>

Available on **crate feature `services-monoiofs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html" class="struct" title="struct opendal::services::Monoiofs">MonoiofsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Monoiofs.html#blanket-implementations" class="anchor">Â§</a>
