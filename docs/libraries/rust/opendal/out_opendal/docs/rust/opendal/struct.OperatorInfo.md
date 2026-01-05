# Struct OperatorInfo Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/operator/info.rs.html#26" class="src">Source</a>

``` rust
pub struct OperatorInfo(/* private fields */);
```

Expand description

Metadata for operator, users can use this metadata to get information of operator.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#impl-OperatorInfo" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.scheme" class="fn">scheme</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html" class="enum" title="enum opendal::Scheme">Scheme</a>

[`Scheme`](https://opendal.apache.org/docs/rust/opendal/enum.Scheme.html "enum opendal::Scheme") of operator.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.root" class="fn">root</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Root of operator, will be in format like `/path/to/dir/`

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.name" class="fn">name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Name of backend, could be empty if underlying backend doesnâ€™t have namespace concept.

For example:

- name for `s3` =\> bucket name
- name for `azblob` =\> container name

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.full_capability" class="fn">full_capability</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

Get \[`Full Capability`\] of operator.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.native_capability" class="fn">native_capability</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Capability.html" class="struct" title="struct opendal::Capability">Capability</a>

Get \[`Native Capability`\] of operator.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#impl-Clone-for-OperatorInfo" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#impl-Debug-for-OperatorInfo" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#impl-Default-for-OperatorInfo" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html" class="struct" title="struct opendal::OperatorInfo">OperatorInfo</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorInfo.html#blanket-implementations" class="anchor">Â§</a>
