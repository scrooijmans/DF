# Struct OptimizerContext Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/optimizer.rs.html#120" class="src">Source</a>

``` rust
pub struct OptimizerContext { /* private fields */ }
```

Expand description

A standalone [`OptimizerConfig`](https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html "trait datafusion::optimizer::OptimizerConfig") that can be used independently of DataFusion’s config management

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#impl-OptimizerContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

Create optimizer config

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.filter_null_keys" class="fn">filter_null_keys</a>(self, filter_null_keys: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

Specify whether to enable the filter_null_keys rule

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.with_query_execution_start_time" class="fn">with_query_execution_start_time</a>( self, query_execution_tart_time: <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

Specify whether the optimizer should skip rules that produce errors, or fail the query

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.with_skip_failing_rules" class="fn">with_skip_failing_rules</a>(self, b: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

Specify whether the optimizer should skip rules that produce errors, or fail the query

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.with_max_passes" class="fn">with_max_passes</a>(self, v: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

Specify how many times to attempt to optimize the plan

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#impl-Debug-for-OptimizerContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#impl-Default-for-OptimizerContext" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

Create optimizer config

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#impl-OptimizerConfig-for-OptimizerContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.query_execution_start_time" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.query_execution_start_time" class="fn">query_execution_start_time</a>(&self) -\> <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>

Return the time at which the query execution started. This time is used as the value for now()

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.alias_generator" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.alias_generator" class="fn">alias_generator</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator">AliasGenerator</a>\>

Return alias generator used to generate unique aliases for subqueries

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.options" class="fn">options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#method.function_registry" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#method.function_registry" class="fn">function_registry</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html#blanket-implementations" class="anchor">§</a>
