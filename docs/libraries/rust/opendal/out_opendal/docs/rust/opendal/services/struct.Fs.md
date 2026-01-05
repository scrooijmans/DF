# Struct Fs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/fs/backend.rs.html#37-39" class="src">Source</a>

``` rust
pub struct Fs { /* private fields */ }
```

Expand description

POSIX file system support.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☒ append
- ☒ create_dir
- ☒ delete
- ☒ copy
- ☒ rename
- ☒ list
- ☐ ~~presign~~
- ☒ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#configuration" class="doc-anchor">Â§</a>Configuration

- `root`: Set the work dir for backend.
- 

You can refer to [`FsBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html "struct opendal::services::Fs")â€™s docs for more information

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#example" class="doc-anchor">Â§</a>Example

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Fs;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create fs backend builder.
    let mut builder = Fs::default()
        // Set the root for fs, all operations will happen under this root.
        //
        // NOTE: the root must be absolute path.
        .root("/tmp");

    // `Accessor` provides the low level APIs, we will use `Operator` normally.
    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#impl-FsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html" class="struct" title="struct opendal::services::Fs">FsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-fs`** only.

Set root for backend.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#method.atomic_write_dir" class="fn">atomic_write_dir</a>(self, dir: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-fs`** only.

Set temp dir for atomic write.

##### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#notes" class="doc-anchor">Â§</a>Notes

- When append is enabled, we will not use atomic write to avoid data loss and performance issue.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#impl-Builder-for-FsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html" class="struct" title="struct opendal::services::Fs">FsBuilder</a>

Available on **crate feature `services-fs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.FsConfig.html" class="struct" title="struct opendal::services::FsConfig">FsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#impl-Debug-for-FsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html" class="struct" title="struct opendal::services::Fs">FsBuilder</a>

Available on **crate feature `services-fs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#impl-Default-for-FsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html" class="struct" title="struct opendal::services::Fs">FsBuilder</a>

Available on **crate feature `services-fs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html" class="struct" title="struct opendal::services::Fs">FsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Fs.html#blanket-implementations" class="anchor">Â§</a>
