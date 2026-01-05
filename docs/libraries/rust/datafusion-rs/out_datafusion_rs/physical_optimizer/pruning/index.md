# Crate pruning Copy item path

<a href="https://docs.rs/datafusion-pruning/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_pruning/lib.rs.html#18-25" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.FilePruner.html" class="struct" title="struct datafusion::physical_optimizer::pruning::FilePruner">FilePruner</a>  
Prune based on partition values and file-level statistics.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PredicateRewriter.html" class="struct" title="struct datafusion::physical_optimizer::pruning::PredicateRewriter">PredicateRewriter</a>  
Rewrite a predicate expression in terms of statistics (min/max/null_counts) for use as a [`PruningPredicate`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html "struct datafusion::physical_optimizer::pruning::PruningPredicate").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html" class="struct" title="struct datafusion::physical_optimizer::pruning::PruningPredicate">PruningPredicate</a>  
Used to prove that arbitrary predicates (boolean expression) can not possibly evaluate to `true` given information about a column provided by [`PruningStatistics`](https://docs.rs/datafusion/50.2.0/datafusion/common/pruning/trait.PruningStatistics.html "trait datafusion::common::pruning::PruningStatistics").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.RequiredColumns.html" class="struct" title="struct datafusion::physical_optimizer::pruning::RequiredColumns">RequiredColumns</a>  
Describes which columns statistics are necessary to evaluate a [`PruningPredicate`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html "struct datafusion::physical_optimizer::pruning::PruningPredicate").

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.PruningStatistics.html" class="trait" title="trait datafusion::physical_optimizer::pruning::PruningStatistics">PruningStatistics</a>  
A source of runtime statistical information to [`PruningPredicate`](https://docs.rs/datafusion/latest/datafusion/physical_optimizer/pruning/struct.PruningPredicate.html)s.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/trait.UnhandledPredicateHook.html" class="trait" title="trait datafusion::physical_optimizer::pruning::UnhandledPredicateHook">UnhandledPredicateHook</a>  
Rewrites predicates that [`PredicateRewriter`](https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/struct.PredicateRewriter.html "struct datafusion::physical_optimizer::pruning::PredicateRewriter") can not handle, e.g. certain complex expressions or predicates that reference columns that are not in the schema.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/pruning/fn.build_pruning_predicate.html" class="fn" title="fn datafusion::physical_optimizer::pruning::build_pruning_predicate">build_pruning_predicate</a>  
Build a pruning predicate from an optional predicate expression. If the predicate is None or the predicate cannot be converted to a pruning predicate, return None. If there is an error creating the pruning predicate it is recorded by incrementing the `predicate_creation_errors` counter.
