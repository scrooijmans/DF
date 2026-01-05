# Struct ExecutionProps Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#35" class="src">Source</a>

``` rust
pub struct ExecutionProps {
    pub query_execution_start_time: DateTime<Utc>,
    pub alias_generator: Arc<AliasGenerator>,
    pub config_options: Option<Arc<ConfigOptions>>,
    pub var_providers: Option<HashMap<VarType, Arc<dyn VarProvider + Sync + Send>>>,
}
```

Expand description

Holds per-query execution properties and data (such as statement starting timestamps).

An [`ExecutionProps`](https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html "struct datafusion::execution::context::ExecutionProps") is created each time a `LogicalPlan` is prepared for execution (optimized). If the same plan is optimized multiple times, a new `ExecutionProps` is created each time.

It is important that this structure be cheap to create as it is done so during predicate pruning and expression simplification

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#structfield.query_execution_start_time" class="anchor field">§</a>`query_execution_start_time: `<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime"><code>DateTime</code></a>`<`<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc"><code>Utc</code></a>`>`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#structfield.alias_generator" class="anchor field">§</a>`alias_generator: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/alias/struct.AliasGenerator.html" class="struct" title="struct datafusion::common::alias::AliasGenerator"><code>AliasGenerator</code></a>`>`

Alias generator used by subquery optimizer rules

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#structfield.config_options" class="anchor field">§</a>`config_options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions"><code>ConfigOptions</code></a>`>>`

Snapshot of config options when the query started

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#structfield.var_providers" class="anchor field">§</a>`var_providers: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap"><code>HashMap</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/enum.VarType.html" class="enum" title="enum datafusion::variable::VarType"><code>VarType</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html" class="trait" title="trait datafusion::variable::VarProvider"><code>VarProvider</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync"><code>Sync</code></a>` + `<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send"><code>Send</code></a>`>>>`

Providers for scalar variables

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#impl-ExecutionProps" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Creates a new execution props

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.with_query_execution_start_time" class="fn">with_query_execution_start_time</a>( self, query_execution_start_time: <a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/datetime/struct.DateTime.html" class="struct" title="struct chrono::datetime::DateTime">DateTime</a>\<<a href="https://docs.rs/chrono/0.4.41/x86_64-unknown-linux-gnu/chrono/offset/utc/struct.Utc.html" class="struct" title="struct chrono::offset::utc::Utc">Utc</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Set the query execution start time to use

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.start_execution" class="fn">start_execution</a>(&mut self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

👎Deprecated since 50.0.0: Use mark_start_execution instead

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.mark_start_execution" class="fn">mark_start_execution</a>( &mut self, config_options: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\>, ) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Marks the execution of query started timestamp. This also instantiates a new alias generator.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.add_var_provider" class="fn">add_var_provider</a>( &mut self, var_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/enum.VarType.html" class="enum" title="enum datafusion::variable::VarType">VarType</a>, provider: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html" class="trait" title="trait datafusion::variable::VarProvider">VarProvider</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html" class="trait" title="trait datafusion::variable::VarProvider">VarProvider</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>

Registers a variable provider, returning the existing provider, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.get_var_provider" class="fn">get_var_provider</a>( &self, var_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/enum.VarType.html" class="enum" title="enum datafusion::variable::VarType">VarType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/variable/trait.VarProvider.html" class="trait" title="trait datafusion::variable::VarProvider">VarProvider</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>

Returns the provider for the `var_type`, if any

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.config_options" class="fn">config_options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>\>\>

Returns the configuration properties for this execution if the execution has started

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#impl-Clone-for-ExecutionProps" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#impl-Debug-for-ExecutionProps" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#impl-Default-for-ExecutionProps" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html" class="struct" title="struct datafusion::execution::context::ExecutionProps">ExecutionProps</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.ExecutionProps.html#blanket-implementations" class="anchor">§</a>
