# datafusion_substrait - Rust
Crate datafusion\_substrait 
----------------------------

[Source](about:blank/src/datafusion_substrait/lib.rs.html#18-93)

Expand description

Serialize / Deserialize DataFusion Plans to [Substrait.io](https://substrait.io/)

This crate provides support for serializing and deserializing both DataFusion [`LogicalPlan`](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_expr/logical_plan/plan/enum.LogicalPlan.html "enum datafusion_expr::logical_plan::plan::LogicalPlan") and [`ExecutionPlan`](https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/datafusion_physical_plan/execution_plan/trait.ExecutionPlan.html "trait datafusion_physical_plan::execution_plan::ExecutionPlan") to and from the generated types in [substrait::proto](https://docs.rs/substrait/0.58.0/x86_64-unknown-linux-gnu/substrait/proto/index.html "mod substrait::proto") from the [substrait](https://docs.rs/substrait/0.58.0/x86_64-unknown-linux-gnu/substrait/index.html "mod substrait") crate.

[Substrait.io](https://substrait.io/) provides a cross-language serialization format for relational algebra (e.g. query plans and expressions), based on protocol buffers.

Potential uses of this crate:

*   Use DataFusion to run Substrait plans created by other systems (e.g. Apache Calcite)
*   Use DataFusion to create plans to run on other systems
*   Pass query plans over FFI boundaries, such as from Python to Rust
*   Pass query plans across node boundaries

[§](#see-also)See Also
----------------------

Substrait does not (yet) support the full range of plans and expressions that DataFusion offers. See the [datafusion-proto](https://docs.rs/datafusion-proto/latest/datafusion_proto) crate for a DataFusion specific format that does support of the full range.

Note that generated types such as [`substrait::proto::Plan`](https://docs.rs/substrait/0.58.0/x86_64-unknown-linux-gnu/substrait/proto/struct.Plan.html "struct substrait::proto::Plan") and [`substrait::proto::Rel`](https://docs.rs/substrait/0.58.0/x86_64-unknown-linux-gnu/substrait/proto/struct.Rel.html "struct substrait::proto::Rel") can be serialized / deserialized to bytes, JSON and other formats using [prost](https://docs.rs/prost/0.13.5/x86_64-unknown-linux-gnu/prost/index.html "mod prost") and the rest of the Rust protobuf ecosystem.

[§](#example-serializing-logicalplans)Example: Serializing [`LogicalPlan`](https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/datafusion_expr/logical_plan/plan/enum.LogicalPlan.html "enum datafusion_expr::logical_plan::plan::LogicalPlan")s
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

```
// Create a plan that scans table 't'
 let ctx = SessionContext::new();
 let batch = RecordBatch::try_from_iter(vec![("x", Arc::new(Int32Array::from(vec![42])) as _)])?;
 ctx.register_batch("t", batch)?;
 let df = ctx.sql("SELECT x from t").await?;
 let plan = df.into_optimized_plan()?;

 // Convert the plan into a substrait (protobuf) Plan
 let substrait_plan = logical_plan::producer::to_substrait_plan(&plan, &ctx.state())?;

 // Receive a substrait protobuf from somewhere, and turn it into a LogicalPlan
 let logical_round_trip = logical_plan::consumer::from_substrait_plan(&ctx.state(), &substrait_plan).await?;
 let logical_round_trip = ctx.state().optimize(&logical_round_trip)?;
 assert_eq!(format!("{:?}", plan), format!("{:?}", logical_round_trip));
```


`pub use [substrait](https://docs.rs/substrait/0.58.0/x86_64-unknown-linux-gnu/substrait/index.html "mod substrait");`

[extensions](extensions/index.html "mod datafusion_substrait::extensions")

[logical\_plan](logical_plan/index.html "mod datafusion_substrait::logical_plan")

[physical\_plan](physical_plan/index.html "mod datafusion_substrait::physical_plan")`physical`

[serializer](serializer/index.html "mod datafusion_substrait::serializer")

[variation\_const](variation_const/index.html "mod datafusion_substrait::variation_const")

Type variation constants