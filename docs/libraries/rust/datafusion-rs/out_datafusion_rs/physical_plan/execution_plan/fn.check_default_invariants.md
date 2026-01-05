# Function check_default_invariants Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/execution_plan.rs.html#1090-1093" class="src">Source</a>

``` rust
pub fn check_default_invariants<P>(
    plan: &P,
    _check: InvariantLevel,
) -> Result<(), DataFusionError>where
    P: ExecutionPlan + ?Sized,
```

Expand description

Checks a set of invariants that apply to all ExecutionPlan implementations. Returns an error if the given node does not conform.
