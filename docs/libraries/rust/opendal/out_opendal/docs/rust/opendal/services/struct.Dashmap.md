# Struct Dashmap Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/dashmap/backend.rs.html#38-40" class="src">Source</a>

``` rust
pub struct Dashmap { /* private fields */ }
```

Expand description

[dashmap](https://github.com/xacrimon/dashmap) backend support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☒ list
- ☐ presign

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the root path for this dashmap instance.

You can refer to [`DashmapBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html "struct opendal::services::Dashmap")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Dashmap;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Dashmap::default()
        .root("/");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#impl-DashmapBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html" class="struct" title="struct opendal::services::Dashmap">DashmapBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#method.root" class="fn">root</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-dashmap`** only.

Set the root for dashmap.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#impl-Builder-for-DashmapBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html" class="struct" title="struct opendal::services::Dashmap">DashmapBuilder</a>

Available on **crate feature `services-dashmap`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.DashmapConfig.html" class="struct" title="struct opendal::services::DashmapConfig">DashmapConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#impl-Debug-for-DashmapBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html" class="struct" title="struct opendal::services::Dashmap">DashmapBuilder</a>

Available on **crate feature `services-dashmap`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#impl-Default-for-DashmapBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html" class="struct" title="struct opendal::services::Dashmap">DashmapBuilder</a>

Available on **crate feature `services-dashmap`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html" class="struct" title="struct opendal::services::Dashmap">DashmapBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Dashmap.html#blanket-implementations" class="anchor">Â§</a>
