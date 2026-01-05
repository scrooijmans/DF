# Struct UnnestRelationBuilder Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/unparser/ast.rs.html#605" class="src">Source</a>

``` rust
pub struct UnnestRelationBuilder {
    pub alias: Option<TableAlias>,
    pub array_exprs: Vec<Expr>,
    /* private fields */
}
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#structfield.alias" class="anchor field">§</a>`alias: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.TableAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::TableAlias"><code>TableAlias</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#structfield.array_exprs" class="anchor field">§</a>`array_exprs: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#impl-UnnestRelationBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.alias" class="fn">alias</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.TableAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::TableAlias">TableAlias</a>\>) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.array_exprs" class="fn">array_exprs</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr">Expr</a>\>) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.with_offset" class="fn">with_offset</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.with_offset_alias" class="fn">with_offset_alias</a>( &mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident">Ident</a>\>, ) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.with_ordinality" class="fn">with_ordinality</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.build" class="fn">build</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TableFactor.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TableFactor">TableFactor</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/enum.BuilderError.html" class="enum" title="enum datafusion::sql::unparser::ast::BuilderError">BuilderError</a>\>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#impl-Clone-for-UnnestRelationBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#impl-Default-for-UnnestRelationBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html" class="struct" title="struct datafusion::sql::unparser::ast::UnnestRelationBuilder">UnnestRelationBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/unparser/ast/struct.UnnestRelationBuilder.html#blanket-implementations" class="anchor">§</a>
