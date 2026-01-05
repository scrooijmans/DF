# Struct DiagnosticHelp Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/diagnostic.rs.html#69" class="src">Source</a>

``` rust
pub struct DiagnosticHelp {
    pub message: String,
    pub span: Option<Span>,
}
```

Expand description

A “help” enriches a [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") with extra information, possibly referring to different locations in the original SQL query, that helps the user understand how they might fix the error or warning.

Example: SELECT id, name FROM users GROUP BY id Help: Add ‘name’ here ^^^^

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#structfield.message" class="anchor field">§</a>`message: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#structfield.span" class="anchor field">§</a>`span: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span"><code>Span</code></a>`>`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#impl-Clone-for-DiagnosticHelp" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html" class="struct" title="struct datafusion::common::diagnostic::DiagnosticHelp">DiagnosticHelp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html" class="struct" title="struct datafusion::common::diagnostic::DiagnosticHelp">DiagnosticHelp</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#impl-Debug-for-DiagnosticHelp" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html" class="struct" title="struct datafusion::common::diagnostic::DiagnosticHelp">DiagnosticHelp</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html#blanket-implementations" class="anchor">§</a>
