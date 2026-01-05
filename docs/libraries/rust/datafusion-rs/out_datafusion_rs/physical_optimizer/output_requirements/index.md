# Module output_requirements Copy item path

<a href="https://docs.rs/datafusion-physical-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_optimizer/lib.rs.html#39" class="src">Source</a>

Expand description

The GlobalOrderRequire optimizer rule either:

- Adds an auxiliary `OutputRequirementExec` operator to keep track of global ordering and distribution requirement across rules, or
- Removes the auxiliary `OutputRequirementExec` operator from the physical plan. Since the `OutputRequirementExec` operator is only a helper operator, it shouldn’t occur in the final plan (i.e. the executed plan).

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirementExec.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirementExec">OutputRequirementExec</a>  
An ancillary, non-executable operator whose sole purpose is to track global requirements during optimization. It imposes

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirements">OutputRequirements</a>  
This rule either adds or removes [`OutputRequirements`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirements.html "struct datafusion::physical_optimizer::output_requirements::OutputRequirements")s to/from the physical plan according to its `mode` attribute, which is set by the constructors `new_add_mode` and `new_remove_mode`. With this rule, we can keep track of the global requirements (ordering and distribution) across rules.
