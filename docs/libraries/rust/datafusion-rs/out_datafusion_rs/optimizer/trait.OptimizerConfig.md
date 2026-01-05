# Trait OptimizerConfig Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/optimizer.rs.html#102" class="src">Source</a>

``` rust
pub trait OptimizerConfig {
    // Required methods
    fn query_execution_start_time(&self) -> DateTime<Utc>;
    fn alias_generator(&self) -> &Arc<AliasGenerator>;
    fn options(&self) -> Arc<ConfigOptions>;

    // Provided method
    fn function_registry(&self) -> Option<&dyn FunctionRegistry> { ... }
}
```

Expand description

Options to control the DataFusion Optimizer.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.query_execution_start_time" class="fn">query_execution_start_time</a>(&self) -\> <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>

Return the time at which the query execution started. This time is used as the value for now()

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.alias_generator" class="fn">alias_generator</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator">AliasGenerator</a>\>

Return alias generator used to generate unique aliases for subqueries

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#tymethod.options" class="fn">options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\>

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#method.function_registry" class="fn">function_registry</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.FunctionRegistry.html" class="trait" title="trait datafusion::execution::FunctionRegistry">FunctionRegistry</a>\>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#impl-OptimizerConfig-for-SessionState" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html#impl-OptimizerConfig-for-OptimizerContext" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/trait.OptimizerConfig.html" class="trait" title="trait datafusion::optimizer::OptimizerConfig">OptimizerConfig</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/struct.OptimizerContext.html" class="struct" title="struct datafusion::optimizer::OptimizerContext">OptimizerContext</a>
