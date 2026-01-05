# Module diagnostic Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#42" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.Diagnostic.html" class="struct" title="struct datafusion::common::diagnostic::Diagnostic">Diagnostic</a>  
Additional contextual information intended for end users, to help them understand what went wrong by providing human-readable messages, and locations in the source query that relate to the error in some way.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticHelp.html" class="struct" title="struct datafusion::common::diagnostic::DiagnosticHelp">DiagnosticHelp</a>  
A “help” enriches a [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") with extra information, possibly referring to different locations in the original SQL query, that helps the user understand how they might fix the error or warning.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/struct.DiagnosticNote.html" class="struct" title="struct datafusion::common::diagnostic::DiagnosticNote">DiagnosticNote</a>  
A note enriches a [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") with extra information, possibly referring to different locations in the original SQL query, that helps contextualize the error and helps the end user understand why it occurred.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/diagnostic/enum.DiagnosticKind.html" class="enum" title="enum datafusion::common::diagnostic::DiagnosticKind">DiagnosticKind</a>  
A [`Diagnostic`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Diagnostic.html "struct datafusion::common::Diagnostic") can either be a hard error that prevents the query from being planned and executed, or a warning that indicates potential issues, performance problems, or causes for unexpected results, but is non-fatal. This enum expresses these two possibilities.
