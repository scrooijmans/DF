# Struct AliasGenerator Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/alias.rs.html#22" class="src">Source</a>

``` rust
pub struct AliasGenerator { /* private fields */ }
```

Expand description

A utility struct that can be used to generate unique aliases when optimizing queries

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#impl-AliasGenerator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator">AliasGenerator</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator">AliasGenerator</a>

Create a new [`AliasGenerator`](https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html "struct datafusion::common::alias::AliasGenerator")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#method.next" class="fn">next</a>(&self, prefix: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Return a unique alias with the provided prefix

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#impl-Debug-for-AliasGenerator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator">AliasGenerator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#impl-Default-for-AliasGenerator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator">AliasGenerator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator">AliasGenerator</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html#blanket-implementations" class="anchor">§</a>
