# Module rfc_6678_simulate_layer Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/docs/rfcs/mod.rs.html#282" class="src">Source</a>

Available on **`docsrs`** only.

Expand description

Simulate Layer

- Proposal Name: `simulate_layer`
- Start Date: 2025-10-16
- RFC PR: [apache/opendal#6678](https://github.com/apache/opendal/pull/6678)
- Tracking Issue: [apache/opendal#6681](https://github.com/apache/opendal/issues/6681)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#summary" class="doc-anchor">Â§</a>Summary

Introduce a public `SimulateLayer` to replace the internal `CompleteLayer`, giving users explicit control over capability simulation while maintaining backward compatibility through a phased migration strategy.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#motivation" class="doc-anchor">Â§</a>Motivation

Currently, OpenDAL automatically applies `CompleteLayer` internally to simulate missing capabilities for all backends. While this provides a seamless â€œbatteries-includedâ€? experience, it has several limitations:

1.  **Lack of user control**: Users cannot opt-out of simulations they donâ€™t need, even in performance-sensitive scenarios where native-only operations are preferred.

2.  **Hidden complexity**: The `CompleteLayer` has grown to 385 lines handling multiple concerns (list recursive, stat dir, create_dir, reader/writer wrappers), making it difficult to maintain and understand.

3.  **All-or-nothing approach**: Users either get all simulations or none. Thereâ€™s no way to selectively enable only the simulations they need.

4.  **Binding limitations**: While Python/Java/Ruby bindings automatically benefit from simulations, they also cannot disable them, potentially causing unexpected behavior when users expect native backend semantics.

5.  **Missing features**: Some useful simulations (like `start_after` for fs backend, reported in \#6676) are not implemented because adding more logic to `CompleteLayer` would make it even more complex.

This proposal aims to:

- Make capability simulation **explicit and user-controllable**
- **Reduce complexity** by separating concerns
- Provide a **clear migration path** that maintains backward compatibility
- Enable **fine-grained control** over which simulations to apply

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#guide-level-explanation" class="doc-anchor">Â§</a>Guide-level explanation

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#for-rust-users" class="doc-anchor">Â§</a>For Rust users

After this change, capability simulation becomes explicit through `SimulateLayer`:

``` rust
use opendal::layers::SimulateLayer;
use opendal::services::Fs;
use opendal::Operator;

// Default: all simulations enabled
let op = Operator::new(Fs::default())?
    .layer(SimulateLayer::default())
    .finish();

// Selective simulation with method chaining
let op = Operator::new(Fs::default())?
    .layer(
        SimulateLayer::default()
            .with_list_recursive(true)      // Enable recursive listing simulation
            .with_list_start_after(true)    // Enable start_after simulation
            .with_stat_dir(false)           // Disable stat dir simulation
            .with_create_dir(false)         // Disable create_dir simulation
    )
    .finish();

// Performance-critical: no simulation overhead
let op = Operator::new(S3::default())?
    // Don't add SimulateLayer - use native capabilities only
    .finish();
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#for-binding-users-pythonjavaruby" class="doc-anchor">Â§</a>For binding users (Python/Java/Ruby)

Binding users will see **no breaking changes** during the migration. Simulations will continue to work automatically:

``` python
# Python - works the same way
operator = opendal.Operator("fs", root="/tmp")
files = list(operator.list("", start_after="file.txt"))  # Just works
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#migration-path" class="doc-anchor">Â§</a>Migration path

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#phase-1-next-version-soft-transition---0-3-months" class="doc-anchor">Â§</a>Phase 1: Next version (Soft transition - 0-3 months)

- `SimulateLayer` is introduced as a **public layer**
- `CompleteLayer` remains internal, controlled by `auto-simulate` feature (enabled by default)
- Documentation encourages explicit use of `SimulateLayer`
- No breaking changes

``` toml
# Still works with auto-simulate enabled by default
[dependencies]
opendal = "0.x"
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#phase-2-next-2-3-versions-transition-period---3-6-months" class="doc-anchor">Â§</a>Phase 2: Next 2-3 versions (Transition period - 3-6 months)

- Deprecation warnings added for implicit simulation
- Migration guide published
- Examples updated to use explicit `SimulateLayer`
- Users can test with `default-features = false` to disable auto-simulate

``` toml
# Opt-out of auto-simulate to prepare for migration
[dependencies]
opendal = { version = "0.x", default-features = false }
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#phase-3-next-minor-version-after-transition-breaking-change---6-months" class="doc-anchor">Â§</a>Phase 3: Next minor version after transition (Breaking change - 6 months)

- `auto-simulate` feature **disabled by default**
- Users must explicitly add `SimulateLayer` or opt-in with `features = ["auto-simulate"]`

``` rust
// Required after this version
let op = Operator::new(Fs::default())?
    .layer(SimulateLayer::default())
    .finish();
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#phase-4-next-major-version-clean-slate---12-months" class="doc-anchor">Â§</a>Phase 4: Next major version (Clean slate - 12 months)

- `CompleteLayer` and `auto-simulate` feature completely removed
- Bindings automatically apply `SimulateLayer` internally
- Core library requires explicit simulation

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#reference-level-explanation" class="doc-anchor">Â§</a>Reference-level explanation

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#architecture-changes" class="doc-anchor">Â§</a>Architecture changes

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#current-architecture" class="doc-anchor">Â§</a>Current architecture

``` rust
Backend â CompleteLayer (auto-applied) â User
          â
          â¢ list recursive simulation
          â¢ stat dir simulation  
          â¢ create_dir simulation
          â¢ reader/writer wrappers
          â¢ (all forced, no user control)
```

#### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#new-architecture" class="doc-anchor">Â§</a>New architecture

``` rust
Backend â [SimulateLayer (user-controlled)] â User
          â
          â¢ list recursive simulation (configurable)
          â¢ list start_after simulation (configurable)
          â¢ stat dir simulation (configurable)
          â¢ create_dir simulation (configurable)
          â¢ reader/writer wrappers (kept in CompleteLayer or removed)
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#simulatelayer-implementation" class="doc-anchor">Â§</a>SimulateLayer implementation

``` rust
// core/src/layers/simulate.rs

#[derive(Debug, Clone)]
pub struct SimulateLayer {
    list_recursive: bool,
    list_start_after: bool,
    list_limit: bool,        // Future
    stat_dir: bool,
    create_dir: bool,
}

impl Default for SimulateLayer {
    fn default() -> Self {
        Self {
            list_recursive: true,
            list_start_after: true,
            list_limit: false,
            stat_dir: true,
            create_dir: true,
        }
    }
}

impl SimulateLayer {
    /// Enable/disable recursive listing simulation (default: true)
    pub fn with_list_recursive(mut self, enabled: bool) -> Self {
        self.list_recursive = enabled;
        self
    }
    
    /// Enable/disable start_after simulation (default: true)
    pub fn with_list_start_after(mut self, enabled: bool) -> Self {
        self.list_start_after = enabled;
        self
    }
    
    /// Enable/disable stat directory simulation (default: true)
    pub fn with_stat_dir(mut self, enabled: bool) -> Self {
        self.stat_dir = enabled;
        self
    }
    
    /// Enable/disable create_dir simulation (default: true)
    pub fn with_create_dir(mut self, enabled: bool) -> Self {
        self.create_dir = enabled;
        self
    }
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#feature-flag-for-migration" class="doc-anchor">Â§</a>Feature flag for migration

``` rust
// core/src/types/operator/builder.rs

impl<A: Builder> Operator {
    pub fn new(accessor: A) -> OperatorBuilder<impl Access> {
        let builder = OperatorBuilder { accessor }
            .layer(ErrorContextLayer);
        
        #[cfg(feature = "auto-simulate")]
        let builder = builder.layer(CompleteLayer);
        
        builder.layer(CorrectnessCheckLayer)
    }
}

// Cargo.toml
[features]
default = ["auto-simulate"]  # Next version to Phase 3
auto-simulate = []
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#capability-updates" class="doc-anchor">Â§</a>Capability updates

Services that donâ€™t natively support certain features will have their `full_capability` updated when `SimulateLayer` is applied:

``` rust
impl<A: Access> Layer<A> for SimulateLayer {
    fn layer(&self, inner: A) -> Self::LayeredAccess {
        let info = inner.info();
        
        info.update_full_capability(|mut cap| {
            if self.list_start_after && cap.list {
                cap.list_with_start_after = true;
            }
            if self.list_recursive && cap.list {
                cap.list_with_recursive = true;
            }
            if self.create_dir && cap.list && cap.write_can_empty {
                cap.create_dir = true;
            }
            cap
        });
        
        SimulateAccessor { 
            inner: Arc::new(inner), 
            info, 
            config: self.clone() 
        }
    }
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#bindings-integration" class="doc-anchor">Â§</a>Bindings integration

Bindings will automatically apply `SimulateLayer` to maintain backward compatibility:

``` rust
// bindings/python/src/operator.rs

impl Operator {
    pub fn new(scheme: &str, options: HashMap<String, String>) -> PyResult<Self> {
        let op = ocore::Operator::new(accessor)?
            .layer(ocore::layers::SimulateLayer::default())  // Auto-apply in bindings
            .finish();
        Ok(Self { core: op })
    }
}
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#list-start_after-simulation" class="doc-anchor">Â§</a>List start_after simulation

As a concrete example, hereâ€™s how `start_after` will be simulated:

``` rust
pub struct StartAfterLister<L> {
    inner: L,
    start_after: String,
    skipped: bool,
}

impl<L: oio::List> oio::List for StartAfterLister<L> {
    async fn next(&mut self) -> Result<Option<oio::Entry>> {
        loop {
            let Some(entry) = self.inner.next().await? else {
                return Ok(None);
            };
            
            // Skip entries until we find one > start_after
            if !self.skipped {
                if entry.path() <= self.start_after.as_str() {
                    continue;
                }
                self.skipped = true;
            }
            
            return Ok(Some(entry));
        }
    }
}
```

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#drawbacks" class="doc-anchor">Â§</a>Drawbacks

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#breaking-change-risk" class="doc-anchor">Â§</a>Breaking change risk

Even with a phased migration, changing from implicit to explicit simulation is fundamentally a breaking change. Users who donâ€™t follow the migration guide will face compilation errors after Phase 3.

**Mitigation**:

- 6-month transition period with clear deprecation warnings
- Comprehensive migration guide with automated tools
- `auto-simulate` feature for temporary backward compatibility

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#documentation-burden" class="doc-anchor">Â§</a>Documentation burden

This change requires updating extensive documentation, examples, and tutorials across all bindings.

**Mitigation**:

- Automated documentation generation
- Version-specific migration guides
- Community support during transition

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#potential-performance-regression-for-some-users" class="doc-anchor">Â§</a>Potential performance regression for some users

Users who upgrade without understanding the change might add `SimulateLayer::default()` everywhere, even for backends that donâ€™t need it, adding unnecessary overhead.

**Mitigation**:

- Clear documentation about which backends benefit from simulation
- Performance recommendations in the migration guide
- Compiler warnings for unnecessary simulations (future enhancement)

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#rationale-and-alternatives" class="doc-anchor">Â§</a>Rationale and alternatives

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#why-this-design" class="doc-anchor">Â§</a>Why this design?

1.  **Explicit over implicit**: Users should consciously decide what simulations to use, leading to better understanding and fewer surprises.

2.  **Gradual migration**: The phased approach minimizes disruption while giving the community time to adapt.

3.  **Binding compatibility**: Automatic application in bindings ensures that non-Rust users arenâ€™t affected.

4.  **Extensibility**: The method chaining pattern makes it easy to add new simulations (like `list_with_limit`) without breaking changes.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#alternative-1-keep-completelayer-add-disable-flags" class="doc-anchor">Â§</a>Alternative 1: Keep CompleteLayer, add disable flags

Instead of a new layer, add flags to disable specific simulations in `CompleteLayer`.

**Rejected because**:

- Doesnâ€™t address the â€œall-or-nothingâ€? problem fundamentally
- Still forces all users to pay for simulation machinery even if disabled
- Doesnâ€™t improve code organization or maintainability

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#alternative-2-service-specific-default-simulation" class="doc-anchor">Â§</a>Alternative 2: Service-specific default simulation

Each service declares which simulations it needs, applied automatically.

**Rejected because**:

- Less transparent - users donâ€™t know whatâ€™s being simulated
- Service maintainers must decide defaults, which may not match user needs
- Doesnâ€™t allow per-application customization

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#alternative-3-split-into-multiple-layers" class="doc-anchor">Â§</a>Alternative 3: Split into multiple layers

Create separate layers: `ListSimulateLayer`, `StatSimulateLayer`, `CreateDirSimulateLayer`.

**Rejected because**:

- Too granular for most users
- Verbose: `.layer(ListSimulateLayer).layer(StatSimulateLayer)...`
- Doesnâ€™t align with OpenDALâ€™s â€œbatteries includedâ€? philosophy
- `SimulateLayer` with method chaining provides the same granularity when needed

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#alternative-4-builder-pattern" class="doc-anchor">Â§</a>Alternative 4: Builder pattern

Use a separate builder type like `SimulateLayer::builder().with_xxx().build()`.

**Rejected because**:

- More verbose than necessary for simple use cases
- Method chaining on the layer itself is more ergonomic
- OpenDAL already uses method chaining in many places (e.g., `OpList`)

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#what-if-we-dont-do-this" class="doc-anchor">Â§</a>What if we donâ€™t do this?

1.  `CompleteLayer` continues to grow in complexity
2.  New simulations (like `start_after` for fs) remain unimplemented
3.  Performance-sensitive users continue to be frustrated by forced overhead
4.  OpenDAL becomes harder to maintain and understand

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#prior-art" class="doc-anchor">Â§</a>Prior art

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#rust-ecosystem" class="doc-anchor">Â§</a>Rust ecosystem

- **Tower** (HTTP middleware): Uses explicit middleware with builder pattern
- **Actix-web**: Middleware are opt-in with clear APIs
- **Reqwest**: Features are controlled via Cargo features, not runtime

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#other-storage-libraries" class="doc-anchor">Â§</a>Other storage libraries

- **AWS SDK**: Capabilities are documented but not abstracted
- **GCS Client**: No simulation layer - users must handle missing features
- **MinIO SDK**: Provides compatibility helpers as separate modules

OpenDALâ€™s approach is unique in providing transparent simulation, and this RFC aims to maintain that advantage while adding user control.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#unresolved-questions" class="doc-anchor">Â§</a>Unresolved questions

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#when-to-remove-readerwriter-wrappers" class="doc-anchor">Â§</a>When to remove reader/writer wrappers?

`CompleteLayer` currently includes `CompleteReader` and `CompleteWriter` that validate read/write sizes. Should these:

1.  Stay in a minimal `CompleteLayer` (still auto-applied)?
2.  Move to `SimulateLayer`?
3.  Move to a separate `ValidateLayer`?

**Proposed resolution**: Keep in minimal `CompleteLayer` during transition, revisit in next major version.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#should-we-provide-migration-tooling" class="doc-anchor">Â§</a>Should we provide migration tooling?

Should we build `cargo-opendal migrate` or similar tools to automatically update code?

**Proposed resolution**: Defer to Phase 2, assess based on community feedback.

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#binding-specific-configuration" class="doc-anchor">Â§</a>Binding-specific configuration?

Should Python users be able to configure `SimulateLayer` through binding APIs?

``` python
# Hypothetical API
operator = opendal.Operator(
    "fs", 
    root="/tmp",
    simulate={"list_start_after": True, "stat_dir": False}
)
```

**Proposed resolution**: Not in initial implementation. Can be added in future based on demand.

## <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#future-possibilities" class="doc-anchor">Â§</a>Future possibilities

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#automatic-simulation-detection" class="doc-anchor">Â§</a>Automatic simulation detection

Add compile-time or runtime warnings when `SimulateLayer` is used with backends that donâ€™t need it:

``` rust
// Future enhancement
let op = Operator::new(S3::default())?
    .layer(SimulateLayer::default())  // Warning: S3 supports all features natively
    .finish();
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#conditional-simulation-based-on-operations" class="doc-anchor">Â§</a>Conditional simulation based on operations

Allow enabling simulations only for specific operations:

``` rust
// Future API
SimulateLayer::default()
    .simulate_only_for(&["list", "stat"])
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#simulation-statistics" class="doc-anchor">Â§</a>Simulation statistics

Expose metrics about which simulations are being used:

``` rust
// Future API
let stats = op.simulation_stats();
println!("start_after simulated: {} times", stats.start_after_count);
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#list-limit-simulation" class="doc-anchor">Â§</a>List limit simulation

Extend simulation to `list_with_limit` for backends that donâ€™t support it natively (requires client-side buffering).

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#versioning-simulation" class="doc-anchor">Â§</a>Versioning simulation

Extend simulation to `list_with_versions` for backends that donâ€™t support versioning natively (requires metadata storage).

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#plugin-system-for-custom-simulations" class="doc-anchor">Â§</a>Plugin system for custom simulations

Allow users to provide custom simulation logic:

``` rust
// Far future API
SimulateLayer::default()
    .with_custom_simulator(MyCustomSimulator::new())
```

### <a href="https://opendal.apache.org/docs/rust/opendal/docs/rfcs/rfc_6678_simulate_layer/index.html#zero-cost-simulation-layer" class="doc-anchor">Â§</a>Zero-cost simulation layer

Use const generics or compile-time feature detection to make simulation truly zero-cost when all features are native:

``` rust
// Speculative future optimization
let op = Operator::new(S3::default())?
    .layer(SimulateLayer::default())  // Optimized away at compile time for S3
    .finish();
```
