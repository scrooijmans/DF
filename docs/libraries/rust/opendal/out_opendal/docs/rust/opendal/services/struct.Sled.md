# Struct Sled Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/sled/backend.rs.html#37-39" class="src">Source</a>

``` rust
pub struct Sled { /* private fields */ }
```

Expand description

Sled services support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☐ ~~list~~
- ☐ ~~presign~~
- ☒ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#configuration" class="doc-anchor">Â§</a>Configuration

- `datadir`: Set the path to the sled data directory

You can refer to [`SledBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html "struct opendal::services::Sled")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use anyhow::Result;
use opendal::services::Sled;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = Sled::default()
        .datadir("/tmp/opendal/sled");

    let op: Operator = Operator::new(builder)?.finish();
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#impl-SledBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html" class="struct" title="struct opendal::services::Sled">SledBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#method.datadir" class="fn">datadir</a>(self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sled`** only.

Set the path to the sled data directory. Will create if not exists.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sled`** only.

Set the root for sled.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#method.tree" class="fn">tree</a>(self, tree: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-sled`** only.

Set the tree for sled.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#impl-Builder-for-SledBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html" class="struct" title="struct opendal::services::Sled">SledBuilder</a>

Available on **crate feature `services-sled`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.SledConfig.html" class="struct" title="struct opendal::services::SledConfig">SledConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#impl-Debug-for-SledBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html" class="struct" title="struct opendal::services::Sled">SledBuilder</a>

Available on **crate feature `services-sled`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#impl-Default-for-SledBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html" class="struct" title="struct opendal::services::Sled">SledBuilder</a>

Available on **crate feature `services-sled`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html" class="struct" title="struct opendal::services::Sled">SledBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Sled.html#blanket-implementations" class="anchor">Â§</a>
