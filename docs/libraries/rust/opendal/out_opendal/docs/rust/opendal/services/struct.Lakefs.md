# Struct Lakefs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/lakefs/backend.rs.html#41-43" class="src">Source</a>

``` rust
pub struct Lakefs { /* private fields */ }
```

Expand description

[Lakefs](https://docs.lakefs.io/reference/api.html#/)â€™s API support. This service will visit the [Lakefs API](https://Lakefs.co/docs/Lakefs_hub/package_reference/hf_api) to access the Lakefs File System. Currently, we only support the `model` and `dataset` types of repositories, and operations are limited to reading and listing/stating.

Lakefs doesnâ€™t host official HTTP API docs. Detailed HTTP request API information can be found on the [`Lakefs_hub` Source Code](https://github.com/Lakefs/Lakefs_hub).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☒ write
- ☐ create_dir
- ☒ delete
- ☒ copy
- ☐ rename
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#configurations" class="doc-anchor">Â§</a>Configurations

- `endpoint`: The endpoint of the Lakefs repository.
- `repository`: The id of the repository.
- `branch`: The branch of the repository.
- `root`: Set the work directory for backend.
- `username`: The username for accessing the repository.
- `password`: The password for accessing the repository.

Refer to [`LakefsBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html "struct opendal::services::Lakefs")â€™s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#examples" class="doc-anchor">Â§</a>Examples

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use opendal::Operator;
use opendal::services::Lakefs;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create Lakefs backend builder
    let mut builder = Lakefs::default()
        // set the type of Lakefs endpoint
        .endpoint("https://whole-llama-mh6mux.us-east-1.lakefscloud.io")
        // set the id of Lakefs repository
        .repository("sample-repo")
        // set the branch of Lakefs repository
        .branch("main")
        // set the username for accessing the repository
        .username("xxx")
        // set the password for accessing the repository
        .password("xxx");

    let op: Operator = Operator::new(builder)?.finish();

    let stat = op.stat("README.md").await?;
    println!("{:?}", stat);
    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#impl-LakefsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.endpoint" class="fn">endpoint</a>(self, endpoint: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-lakefs`** only.

Set the endpoint of this backend.

endpoint must be full uri.

This is required.

- `http://127.0.0.1:8000` (lakefs daemon in local)
- `https://my-lakefs.example.com` (lakefs server)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.username" class="fn">username</a>(self, username: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-lakefs`** only.

Set username of this backend. This is required.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.password" class="fn">password</a>(self, password: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-lakefs`** only.

Set password of this backend. This is required.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.branch" class="fn">branch</a>(self, branch: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-lakefs`** only.

Set branch of this backend or a commit ID. Default is main.

Branch can be a branch name.

For example, branch can be:

- main
- 1d0c4eb

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-lakefs`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.repository" class="fn">repository</a>(self, repository: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-lakefs`** only.

Set the repository of this backend.

This is required.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#impl-Builder-for-LakefsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

Available on **crate feature `services-lakefs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Build a LakefsBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.LakefsConfig.html" class="struct" title="struct opendal::services::LakefsConfig">LakefsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#impl-Clone-for-LakefsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

Available on **crate feature `services-lakefs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#impl-Debug-for-LakefsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

Available on **crate feature `services-lakefs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#impl-Default-for-LakefsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

Available on **crate feature `services-lakefs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html" class="struct" title="struct opendal::services::Lakefs">LakefsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Lakefs.html#blanket-implementations" class="anchor">Â§</a>
