# Struct Diagnostic Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/diagnostic.rs.html#40" class="src">Source</a>

``` rust
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<DiagnosticNote>,
    pub helps: Vec<DiagnosticHelp>,
}
```

Expand description

Additional contextual information intended for end users, to help them understand what went wrong by providing human-readable messages, and locations in the source query that relate to the error in some way.

You can think of a single [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") as a single “block” of output from rustc. i.e. either an error or a warning, optionally with some notes and help messages.

Example:

``` rust
let span = Some(Span {
    start: Location{ line: 2, column: 1 },
    end: Location{ line: 4, column: 15 }
});
let diagnostic = Diagnostic::new_error("Something went wrong", span)
    .with_help("Have you tried turning it on and off again?", None);
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#structfield.kind" class="anchor field">§</a>`kind: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/enum.DiagnosticKind.html" class="enum" title="enum datafusion::common::diagnostic::DiagnosticKind"><code>DiagnosticKind</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#structfield.message" class="anchor field">§</a>`message: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#structfield.span" class="anchor field">§</a>`span: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span"><code>Span</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#structfield.notes" class="anchor field">§</a>`notes: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticNote.html" class="struct" title="struct datafusion::common::diagnostic::DiagnosticNote"><code>DiagnosticNote</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#structfield.helps" class="anchor field">§</a>`helps: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html" class="struct" title="struct datafusion::common::diagnostic::DiagnosticHelp"><code>DiagnosticHelp</code></a>`>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#impl-Diagnostic" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.new_error" class="fn">new_error</a>(message: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, span: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>

Creates a new [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") for a fatal error that prevents the SQL query from being planned and executed. Optionally takes in a [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html "struct datafusion::common::Span") to describe the location in the source code that caused the error, should be provided when available.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.new_warning" class="fn">new_warning</a>(message: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, span: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>

Creates a new [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") for a NON-fatal warning, such as a performance problem, or possible cause for undesired results. Optionally takes in a [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html "struct datafusion::common::Span") to describe the location in the source code that caused the error, should be provided when available.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.add_note" class="fn">add_note</a>(&mut self, message: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, span: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>)

Adds a “note” to the [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic"), which can have zero or many. A “note” helps contextualize the error and helps the end user understand why it occurred. It can refer to an arbitrary location in the SQL query, or to no location.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.add_help" class="fn">add_help</a>(&mut self, message: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, span: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>)

Adds a “help” to the [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic"), which can have zero or many. A “help” helps the user understand how they might fix the error or warning. It can refer to an arbitrary location in the SQL query, or to no location.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.with_note" class="fn">with_note</a>( self, message: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, span: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>

Like [`Diagnostic::add_note`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.add_note "method datafusion::common::Diagnostic::add_note"), but returns `self` to allow chaining.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.with_help" class="fn">with_help</a>( self, message: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, span: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html" class="struct" title="struct datafusion::common::Span">Span</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>

Like [`Diagnostic::add_help`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.add_help "method datafusion::common::Diagnostic::add_help"), but returns `self` to allow chaining.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#impl-Clone-for-Diagnostic" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#impl-Debug-for-Diagnostic" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html" class="struct" title="struct datafusion::common::Diagnostic">Diagnostic</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html#blanket-implementations" class="anchor">§</a>
