# Struct Compfs Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/services/compfs/backend.rs.html#37-39" class="src">Source</a>

``` rust
pub struct Compfs { /* private fields */ }
```

Expand description

\[`compio`\]-based file system support.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#impl-CompfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#method.root" class="fn">root</a>(self, root: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Available on **crate feature `services-compfs`** only.

Set root for Compfs

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#impl-Builder-for-CompfsBuilder" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html" class="trait" title="trait opendal::Builder">Builder</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

Available on **crate feature `services-compfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#associatedtype.Config" class="anchor">Â§</a>

#### type <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#associatedtype.Config" class="associatedtype">Config</a> = <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.CompfsConfig.html" class="struct" title="struct opendal::services::CompfsConfig">CompfsConfig</a>

Associated configuration for this builder.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#method.build" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/trait.Builder.html#tymethod.build" class="fn">build</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Consume the accessor builder to build a service.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#impl-Clone-for-CompfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

Available on **crate feature `services-compfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#impl-Debug-for-CompfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

Available on **crate feature `services-compfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#impl-Default-for-CompfsBuilder" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

Available on **crate feature `services-compfs`** only.

<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html" class="struct" title="struct opendal::services::Compfs">CompfsBuilder</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/services/struct.Compfs.html#blanket-implementations" class="anchor">Â§</a>
