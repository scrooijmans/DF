# Module display Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#69" class="src">Source</a>

Expand description

Implementation of physical plan display. See [`crate::displayable`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/fn.displayable.html "fn datafusion::physical_plan::displayable") for examples of how to format

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DefaultDisplay.html" class="struct" title="struct datafusion::physical_plan::display::DefaultDisplay">DefaultDisplay</a>  
A new type wrapper to display `T` implementing`DisplayAs` using the `Default` mode

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.DisplayableExecutionPlan.html" class="struct" title="struct datafusion::physical_plan::display::DisplayableExecutionPlan">DisplayableExecutionPlan</a>  
Wraps an `ExecutionPlan` with various methods for formatting

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.ProjectSchemaDisplay.html" class="struct" title="struct datafusion::physical_plan::display::ProjectSchemaDisplay">ProjectSchemaDisplay</a>  
A wrapper to customize partitioned file display

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/struct.VerboseDisplay.html" class="struct" title="struct datafusion::physical_plan::display::VerboseDisplay">VerboseDisplay</a>  
A new type wrapper to display `T` implementing `DisplayAs` using the `Verbose` mode

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::display::DisplayFormatType">DisplayFormatType</a>  
Options for controlling how each [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") should format itself

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::display::DisplayAs">DisplayAs</a>  
Trait for types which could have additional details when formatted in `Verbose` mode

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/fn.display_orderings.html" class="fn" title="fn datafusion::physical_plan::display::display_orderings">display_orderings</a>
