# Struct Documentation Copy item path

<a href="https://docs.rs/datafusion-doc/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_doc/lib.rs.html#43" class="src">Source</a>

``` rust
pub struct Documentation {
    pub doc_section: DocSection,
    pub description: String,
    pub syntax_example: String,
    pub sql_example: Option<String>,
    pub arguments: Option<Vec<(String, String)>>,
    pub alternative_syntax: Option<Vec<String>>,
    pub related_udfs: Option<Vec<String>>,
}
```

Expand description

Documentation for use by [`ScalarUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/ScalarUDFImpl), [`AggregateUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/AggregateUDFImpl) and [`WindowUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/WindowUDFImpl) functions.

See the [`DocumentationBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html "struct datafusion::logical_expr::DocumentationBuilder") to create a new [`Documentation`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html "struct datafusion::logical_expr::Documentation") struct.

The DataFusion [SQL function documentation](https://datafusion.apache.org/user-guide/sql/index.html) is automatically generated from these structs. The name of the udf will be pulled from the [`ScalarUDFImpl::name`](ScalarUDFImpl::name), [`AggregateUDFImpl::name`](AggregateUDFImpl::name) or [`WindowUDFImpl::name`](WindowUDFImpl::name) function as appropriate.

All strings in the documentation are required to be in [markdown format](https://www.markdownguide.org/basic-syntax/).

Currently, documentation only supports a single language thus all text should be in English.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#structfield.doc_section" class="anchor field">§</a>`doc_section: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocSection.html" class="struct" title="struct datafusion::logical_expr::DocSection"><code>DocSection</code></a>

The section in the documentation where the UDF will be documented

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#structfield.description" class="anchor field">§</a>`description: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

The description for the UDF

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#structfield.syntax_example" class="anchor field">§</a>`syntax_example: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

A brief example of the syntax. For example “ascii(str)”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#structfield.sql_example" class="anchor field">§</a>`sql_example: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

A sql example for the UDF, usually in the form of a sql prompt query and output. It is strongly recommended to provide an example for anything but the most basic UDF’s

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#structfield.arguments" class="anchor field">§</a>`arguments: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`)>>`

Arguments for the UDF which will be displayed in array order. Left member of a pair is the argument name, right is a description for the argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#structfield.alternative_syntax" class="anchor field">§</a>`alternative_syntax: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>>`

A list of alternative syntax examples for a function

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#structfield.related_udfs" class="anchor field">§</a>`related_udfs: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>>`

Related functions if any. Values should match the related udf’s name exactly. Related udf’s must be of the same UDF type (scalar, aggregate or window) for proper linking to occur

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#impl-Documentation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.builder" class="fn">builder</a>( doc_section: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocSection.html" class="struct" title="struct datafusion::logical_expr::DocSection">DocSection</a>, description: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, syntax_example: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

Returns a new [`DocumentationBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html "struct datafusion::logical_expr::DocumentationBuilder") with no options set.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.to_doc_attribute" class="fn">to_doc_attribute</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Output the `Documentation` struct in form of custom Rust documentation attributes It is useful to semi automate during tmigration of UDF documentation generation from code based to attribute based and can be safely removed after

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#impl-Clone-for-Documentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#impl-Debug-for-Documentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#impl-Hash-for-Documentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#impl-PartialEq-for-Documentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#impl-Eq-for-Documentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#impl-StructuralPartialEq-for-Documentation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html#blanket-implementations" class="anchor">§</a>
