# Optimizations - Polars user guide
If you use Polars' lazy API, Polars will run several optimizations on your query. Some of them are executed up front, others are determined just in time as the materialized data comes in.

Here is a non-complete overview of optimizations done by polars, what they do and how often they run.



* Optimization: Predicate pushdown
  * Explanation: Applies filters as early as possible/ at scan level.
  * runs: 1 time
* Optimization: Projection pushdown
  * Explanation: Select only the columns that are needed at the scan level.
  * runs: 1 time
* Optimization: Slice pushdown
  * Explanation: Only load the required slice from the scan level. Don't materialize sliced outputs (e.g. join.head(10)).
  * runs: 1 time
* Optimization: Common subplan elimination
  * Explanation: Cache subtrees/file scans that are used by multiple subtrees in the query plan.
  * runs: 1 time
* Optimization: Simplify expressions
  * Explanation: Various optimizations, such as constant folding and replacing expensive operations with faster alternatives.
  * runs: until fixed point
* Optimization: Join ordering
  * Explanation: Estimates the branches of joins that should be executed first in order to reduce memory pressure.
  * runs: 1 time
* Optimization: Type coercion
  * Explanation: Coerce types such that operations succeed and run on minimal required memory.
  * runs: until fixed point
* Optimization: Cardinality estimation
  * Explanation: Estimates cardinality in order to determine optimal group by strategy.
  * runs: 0/n times; dependent on query
