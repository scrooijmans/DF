# Struct UserDefinedFunction Copy item path

<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/src/polars_plan/dsl/udf.rs.html#11" class="src">Source</a>

``` rust
pub struct UserDefinedFunction {
    pub name: PlSmallStr,
    pub fun: LazySerde<SpecialEq<Arc<dyn AnonymousColumnsUdf>>>,
    pub options: FunctionOptions,
}
```

Available on **crate feature `lazy`** only.

Expand description

Represents a user-defined function

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#structfield.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>

name

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#structfield.fun" class="anchor field">§</a>`fun: `<a href="https://docs.rs/polars/latest/polars/prelude/enum.LazySerde.html" class="enum" title="enum polars::prelude::LazySerde"><code>LazySerde</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.SpecialEq.html" class="struct" title="struct polars::prelude::SpecialEq"><code>SpecialEq</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html" class="trait" title="trait polars::prelude::AnonymousColumnsUdf"><code>AnonymousColumnsUdf</code></a>`>>>`

The function implementation.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#structfield.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/polars-plan/0.51.0/x86_64-unknown-linux-gnu/polars_plan/plans/options/struct.FunctionOptions.html" class="struct" title="struct polars_plan::plans::options::FunctionOptions"><code>FunctionOptions</code></a>

Options for the function.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#impl-UserDefinedFunction" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html" class="struct" title="struct polars::prelude::UserDefinedFunction">UserDefinedFunction</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#method.new" class="fn">new</a>( name: <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, fun: impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.AnonymousColumnsUdf.html" class="trait" title="trait polars::prelude::AnonymousColumnsUdf">AnonymousColumnsUdf</a> + 'static, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html" class="struct" title="struct polars::prelude::UserDefinedFunction">UserDefinedFunction</a>

Create a new UserDefinedFunction

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#method.call" class="fn">call</a>(self, args: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/enum.Expr.html" class="enum" title="enum polars::prelude::Expr">Expr</a>

creates a logical expression with a call of the UDF

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#impl-Clone-for-UserDefinedFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html" class="struct" title="struct polars::prelude::UserDefinedFunction">UserDefinedFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html" class="struct" title="struct polars::prelude::UserDefinedFunction">UserDefinedFunction</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#impl-Debug-for-UserDefinedFunction" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html" class="struct" title="struct polars::prelude::UserDefinedFunction">UserDefinedFunction</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.UserDefinedFunction.html#blanket-implementations" class="anchor">§</a>
