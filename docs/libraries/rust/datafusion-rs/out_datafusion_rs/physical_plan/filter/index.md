# Module filter Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#73" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/struct.FilterExec.html" class="struct" title="struct datafusion::physical_plan::filter::FilterExec">FilterExec</a>  
FilterExec evaluates a boolean predicate against all input batches to determine which rows to include in its output batches.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/fn.batch_filter.html" class="fn" title="fn datafusion::physical_plan::filter::batch_filter">batch_filter</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/fn.collect_columns_from_predicate.html" class="fn" title="fn datafusion::physical_plan::filter::collect_columns_from_predicate">collect_columns_from_predicate</a>  
Return the equals Column-Pairs and Non-equals Column-Pairs

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/type.EqualAndNonEqual.html" class="type" title="type datafusion::physical_plan::filter::EqualAndNonEqual">EqualAndNonEqual</a>  
The equals Column-Pairs and Non-equals Column-Pairs in the Predicates

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/type.PhysicalExprPairRef.html" class="type" title="type datafusion::physical_plan::filter::PhysicalExprPairRef">PhysicalExprPairRef</a>  
Pair of `Arc<dyn PhysicalExpr>`s
