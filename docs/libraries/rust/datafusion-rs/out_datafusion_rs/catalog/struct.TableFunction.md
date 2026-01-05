# Struct TableFunction Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#324" class="src">Source</a>

``` rust
pub struct TableFunction { /* private fields */ }
```

Expand description

A table that uses a function to generate data

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#impl-TableFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#method.new" class="fn">new</a>(name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, fun: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html" class="trait" title="trait datafusion::catalog::TableFunctionImpl">TableFunctionImpl</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>

Create a new table function

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get the name of the table function

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#method.function" class="fn">function</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.TableFunctionImpl.html" class="trait" title="trait datafusion::catalog::TableFunctionImpl">TableFunctionImpl</a>\>

Get the implementation of the table function

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#method.create_table_provider" class="fn">create_table_provider</a>( &self, args: &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\], ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Get the function implementation and generate a table

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#impl-Debug-for-TableFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html" class="struct" title="struct datafusion::catalog::TableFunction">TableFunction</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.TableFunction.html#blanket-implementations" class="anchor">§</a>
