# Struct Extensions Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/config.rs.html#1260" class="src">Source</a>

``` rust
pub struct Extensions(/* private fields */);
```

Expand description

A type-safe container for [`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#impl-Extensions" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>

Create a new, empty [`Extensions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html "struct datafusion::config::Extensions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.insert" class="fn">insert</a>\<T\>(&mut self, extension: T)

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html" class="trait" title="trait datafusion::config::ConfigExtension">ConfigExtension</a>,

Registers a [`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension") with this [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.get" class="fn">get</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html" class="trait" title="trait datafusion::config::ConfigExtension">ConfigExtension</a>,

Retrieves the extension of the given type if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.get_mut" class="fn">get_mut</a>\<T\>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html" class="trait" title="trait datafusion::config::ConfigExtension">ConfigExtension</a>,

Retrieves the extension of the given type if any

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#impl-Clone-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#impl-Debug-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#impl-Default-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html#blanket-implementations" class="anchor">§</a>
