# Struct Huggingface Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/huggingface/backend.rs.html#39-41" class="src">Source</a>

``` rust
pub struct Huggingface { /* private fields */ }
```

Expand description

[Huggingface](https://huggingface.co/docs/huggingface_hub/package_reference/hf_api)â€™s API support. This service will visit the [Huggingface API](https://huggingface.co/docs/huggingface_hub/package_reference/hf_api) to access the Huggingface File System. Currently, we only support the `model` and `dataset` types of repositories, and operations are limited to reading and listing/stating.

Huggingface doesnâ€™t host official HTTP API docs. Detailed HTTP request API information can be found on the [`huggingface_hub` Source Code](https://github.com/huggingface/huggingface_hub).

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#capabilities" class="doc-anchor">Â§</a>Capabilities

This service can be used to:

- ☒ stat
- ☒ read
- ☐ write
- ☐ create_dir
- ☐ delete
- ☐ copy
- ☐ rename
- ☒ list
- ☐ ~~presign~~
- ☐ blocking

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#configurations" class="doc-anchor">Â§</a>Configurations

- `repo_type`: The type of the repository.
- `repo_id`: The id of the repository.
- `revision`: The revision of the repository.
- `root`: Set the work directory for backend.
- `token`: The token for accessing the repository.

Refer to [`HuggingfaceBuilder`](https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html "struct opendal::services::Huggingface")â€™s public API docs for more information.

### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#examples" class="doc-anchor">Â§</a>Examples

#### <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#via-builder" class="doc-anchor">Â§</a>Via Builder

``` rust
use std::sync::Arc;

use anyhow::Result;
use opendal::services::Huggingface;
use opendal::Operator;

#[tokio::main]
async fn main() -> Result<()> {
    // Create Huggingface backend builder
    let mut builder = Huggingface::default()
        // set the type of Huggingface repository
        .repo_type("dataset")
        // set the id of Huggingface repository
        .repo_id("databricks/databricks-dolly-15k")
        // set the revision of Huggingface repository
        .revision("main")
        // set the root for Huggingface, all operations will happen under this root
        .root("/path/to/dir")
        // set the token for accessing the repository
        .token("access_token");

    let op: Operator = Operator::new(builder)?.finish();

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#impl-HuggingfaceBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.repo_type" class="fn">repo_type</a>(self, repo_type: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-huggingface`** only.

Set repo type of this backend. Default is model.

Available values:

- model
- dataset

Currently, only models and datasets are supported. [Reference](https://huggingface.co/docs/hub/repositories)

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.repo_id" class="fn">repo_id</a>(self, repo_id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-huggingface`** only.

Set repo id of this backend. This is required.

Repo id consists of the account name and the repository name.

For example, modelâ€™s repo id looks like:

- meta-llama/Llama-2-7b

Datasetâ€™s repo id looks like:

- databricks/databricks-dolly-15k

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.revision" class="fn">revision</a>(self, revision: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-huggingface`** only.

Set revision of this backend. Default is main.

Revision can be a branch name or a commit hash.

For example, revision can be:

- main
- 1d0c4eb

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-huggingface`** only.

Set root of this backend.

All operations will happen under this root.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.token" class="fn">token</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-huggingface`** only.

Set the token of this backend.

This is optional.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#impl-Builder-for-HuggingfaceBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

Available on **crate feature `services-huggingface`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Build a HuggingfaceBackend.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.HuggingfaceConfig.html" class="struct" title="struct opendal::services::HuggingfaceConfig">HuggingfaceConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#impl-Clone-for-HuggingfaceBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

Available on **crate feature `services-huggingface`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#impl-Debug-for-HuggingfaceBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

Available on **crate feature `services-huggingface`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#impl-Default-for-HuggingfaceBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

Available on **crate feature `services-huggingface`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html" class="struct" title="struct opendal::services::Huggingface">HuggingfaceBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Huggingface.html#blanket-implementations" class="anchor">Â§</a>
