# Crate sqlparser Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/lib.rs.html#18-179" class="src">Source</a>

Expand description

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#sql-parser-for-rust" class="doc-anchor">§</a>SQL Parser for Rust

This crate provides an ANSI:SQL 2011 lexer and parser that can parse SQL into an Abstract Syntax Tree ([`AST`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/index.html "mod datafusion::logical_expr::sqlparser::ast")). See the [sqlparser crates.io page](https://crates.io/crates/sqlparser) for more information.

For more information:

1.  [`Parser::parse_sql`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html#method.parse_sql "associated function datafusion::logical_expr::sqlparser::parser::Parser::parse_sql") and [`Parser::new`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/struct.Parser.html#method.new "associated function datafusion::logical_expr::sqlparser::parser::Parser::new") for the Parsing API
2.  [`ast`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/index.html "mod datafusion::logical_expr::sqlparser::ast") for the AST structure
3.  [`Dialect`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/dialect/trait.Dialect.html "trait datafusion::logical_expr::sqlparser::dialect::Dialect") for supported SQL dialects
4.  [`Spanned`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html "trait datafusion::logical_expr::sqlparser::ast::Spanned") for source text locations (see “Source Spans” below for details)

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#example-parsing-sql-text" class="doc-anchor">§</a>Example parsing SQL text

``` rust
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

let dialect = GenericDialect {}; // or AnsiDialect

let sql = "SELECT a, b, 123, myfunc(b) \
           FROM table_1 \
           WHERE a > b AND b < 100 \
           ORDER BY a DESC, b";

let ast = Parser::parse_sql(&dialect, sql).unwrap();

println!("AST: {:?}", ast);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#creating-sql-text-from-ast" class="doc-anchor">§</a>Creating SQL text from AST

This crate allows users to recover the original SQL text (with comments removed, normalized whitespace and identifier capitalization), which is useful for tools that analyze and manipulate SQL.

``` rust
let sql = "SELECT a FROM table_1";

// parse to a Vec<Statement>
let ast = Parser::parse_sql(&GenericDialect, sql).unwrap();

// The original SQL text can be generated from the AST
assert_eq!(ast[0].to_string(), sql);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#pretty-printing" class="doc-anchor">§</a>Pretty Printing

SQL statements can be pretty-printed with proper indentation and line breaks using the alternate flag (`{:#}`):

``` rust
let sql = "SELECT a, b FROM table_1";
let ast = Parser::parse_sql(&GenericDialect, sql).unwrap();

// Pretty print with indentation and line breaks
let pretty_sql = format!("{:#}", ast[0]);
assert_eq!(pretty_sql, r#"
SELECT
  a,
  b
FROM
  table_1
"#.trim());
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#source-spans" class="doc-anchor">§</a>Source Spans

Starting with version `0.53.0` sqlparser introduced source spans to the AST. This feature provides source information for syntax errors, enabling better error messages. See [issue \#1548](https://github.com/apache/datafusion-sqlparser-rs/issues/1548) for more information and the [`Spanned`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html "trait datafusion::logical_expr::sqlparser::ast::Spanned") trait to access the spans.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#migration-guide" class="doc-anchor">§</a>Migration Guide

For the next few releases, we will be incrementally adding source spans to the AST nodes, trying to minimize the impact on existing users. Some breaking changes are inevitable, and the following is a summary of the changes:

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#new-fields-for-spans-must-be-added-to-any-existing-pattern-matches" class="doc-anchor">§</a>New fields for spans (must be added to any existing pattern matches)

The primary change is that new fields will be added to AST nodes to store the source `Span` or `TokenWithLocation`.

This will require

1.  Adding new fields to existing pattern matches.
2.  Filling in the proper span information when constructing AST nodes.

For example, since `Ident` now stores a `Span`, to construct an `Ident` you must provide now provide one:

Previously:

``` text
# use sqlparser::ast::Ident;
Ident {
    value: "name".into(),
    quote_style: None,
}
```

Now

``` rust
Ident {
    value: "name".into(),
    quote_style: None,
    span: Span::empty(),
};
```

Similarly, when pattern matching on `Ident`, you must now account for the `span` field.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#misc" class="doc-anchor">§</a>Misc.

- [`TokenWithLocation`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/type.TokenWithLocation.html "type datafusion::logical_expr::sqlparser::tokenizer::TokenWithLocation") stores a full `Span`, rather than just a source location. Users relying on `token.location` should use `token.location.start` instead.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/ast/index.html" class="mod" title="mod datafusion::sql::sqlparser::ast">ast</a>  
SQL Abstract Syntax Tree (AST) types

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/dialect/index.html" class="mod" title="mod datafusion::sql::sqlparser::dialect">dialect</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/parser/index.html" class="mod" title="mod datafusion::sql::sqlparser::parser">parser</a>  
SQL Parser

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/sqlparser/tokenizer/index.html" class="mod" title="mod datafusion::sql::sqlparser::tokenizer">tokenizer</a>  
SQL Tokenizer
