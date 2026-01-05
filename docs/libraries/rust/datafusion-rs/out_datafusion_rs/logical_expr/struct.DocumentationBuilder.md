# Struct DocumentationBuilder Copy item path

<a href="https://docs.rs/datafusion-doc/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_doc/lib.rs.html#204" class="src">Source</a>

``` rust
pub struct DocumentationBuilder {
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

A builder for [`Documentation`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html "struct datafusion::logical_expr::Documentation")’s.

Example:

``` rust

    use datafusion_doc::{DocSection, Documentation};
    let doc_section = DocSection {
        include: true,
        label: "Display Label",
        description: None,
    };

    let documentation = Documentation::builder(doc_section, "Add one to an int32".to_owned(), "add_one(2)".to_owned())
          .with_argument("arg_1", "The int32 number to add one to")
          .build();
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#structfield.doc_section" class="anchor field">§</a>`doc_section: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocSection.html" class="struct" title="struct datafusion::logical_expr::DocSection"><code>DocSection</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#structfield.description" class="anchor field">§</a>`description: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#structfield.syntax_example" class="anchor field">§</a>`syntax_example: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#structfield.sql_example" class="anchor field">§</a>`sql_example: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#structfield.arguments" class="anchor field">§</a>`arguments: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`)>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#structfield.alternative_syntax" class="anchor field">§</a>`alternative_syntax: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#structfield.related_udfs" class="anchor field">§</a>`related_udfs: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#impl-DocumentationBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.new_with_details" class="fn">new_with_details</a>( doc_section: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocSection.html" class="struct" title="struct datafusion::logical_expr::DocSection">DocSection</a>, description: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, syntax_example: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

Creates a new [`DocumentationBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html "struct datafusion::logical_expr::DocumentationBuilder") with all required fields

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.with_doc_section" class="fn">with_doc_section</a>(self, doc_section: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocSection.html" class="struct" title="struct datafusion::logical_expr::DocSection">DocSection</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.with_description" class="fn">with_description</a>( self, description: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.with_syntax_example" class="fn">with_syntax_example</a>( self, syntax_example: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.with_sql_example" class="fn">with_sql_example</a>( self, sql_example: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.with_argument" class="fn">with_argument</a>( self, arg_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, arg_description: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

Adds documentation for a specific argument to the documentation.

Arguments are displayed in the order they are added.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.with_standard_argument" class="fn">with_standard_argument</a>( self, arg_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, expression_type: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

Add a standard “expression” argument to the documentation

The argument is rendered like below if Some() is passed through:

``` text
<arg_name>:
  <expression_type> expression to operate on. Can be a constant, column, or function, and any combination of operators.
```

The argument is rendered like below if None is passed through:

``` text
<arg_name>:
 The expression to operate on. Can be a constant, column, or function, and any combination of operators.
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.with_alternative_syntax" class="fn">with_alternative_syntax</a>( self, syntax_name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.with_related_udf" class="fn">with_related_udf</a>( self, related_udf: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html" class="struct" title="struct datafusion::logical_expr::DocumentationBuilder">DocumentationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Documentation.html" class="struct" title="struct datafusion::logical_expr::Documentation">Documentation</a>

Build the documentation from provided components

Panics if `doc_section`, `description` or `syntax_example` is not set

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.DocumentationBuilder.html#blanket-implementations" class="anchor">§</a>
