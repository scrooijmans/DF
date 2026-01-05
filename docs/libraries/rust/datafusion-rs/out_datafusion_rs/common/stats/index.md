# Module stats Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#57" class="src">Source</a>

Expand description

This module provides data structures to represent statistics

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/struct.ColumnStatistics.html" class="struct" title="struct datafusion::common::stats::ColumnStatistics">ColumnStatistics</a>  
Statistics for a column within a relation

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/struct.Statistics.html" class="struct" title="struct datafusion::common::stats::Statistics">Statistics</a>  
Statistics for a relation Fields are optional and can be inexact because the sources sometimes provide approximate estimates for performance reasons and the transformations output are not always predictable.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/stats/enum.Precision.html" class="enum" title="enum datafusion::common::stats::Precision">Precision</a>  
Represents a value with a degree of certainty. `Precision` is used to propagate information the precision of statistical values.
