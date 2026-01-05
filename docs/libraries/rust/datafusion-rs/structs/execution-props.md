# ExecutionProps in datafusion::execution::context - Rust

## Struct ExecutionProps 

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#35)

```
pub struct ExecutionProps {
    pub query_execution_start_time: DateTime<Utc>,
    pub alias_generator: Arc<AliasGenerator>,
    pub config_options: Option<Arc<ConfigOptions>>,
    pub var_providers: Option<HashMap<VarType, Arc<dyn VarProvider + Sync + Send>>>,
}
```

Expand description

Holds per-query execution properties and data (such as statement starting timestamps).

An [`ExecutionProps`](struct.ExecutionProps.html "struct datafusion::execution::context::ExecutionProps") is created each time a `LogicalPlan` is prepared for execution (optimized). If the same plan is optimized multiple times, a new `ExecutionProps` is created each time.

It is important that this structure be cheap to create as it is done so during predicate pruning and expression simplification

Alias generator used by subquery optimizer rules

Snapshot of config options when the query started

Providers for scalar variables

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#51)
[§](#impl-ExecutionProps)

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#53)

Creates a new execution props

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#65-68)

Set the query execution start time to use

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#74)

👎Deprecated since 50.0.0: Use mark_start_execution instead

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#81)

Marks the execution of query started timestamp. This also instantiates a new alias generator.

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#90-94)

Registers a variable provider, returning the existing provider, if any

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#105-108)

Returns the provider for the `var_type`, if any

[Source](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/execution_props.rs.html#116)

Returns the configuration properties for this execution if the execution has started

[§](#impl-Freeze-for-ExecutionProps)

[§](#impl-RefUnwindSafe-for-ExecutionProps)

[§](#impl-Send-for-ExecutionProps)

[§](#impl-Sync-for-ExecutionProps)

[§](#impl-Unpin-for-ExecutionProps)

[§](#impl-UnwindSafe-for-ExecutionProps)
