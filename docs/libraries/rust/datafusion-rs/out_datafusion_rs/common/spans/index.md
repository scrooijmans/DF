# Module spans Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#56" class="src">Source</a>

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/spans/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/spans/struct.Location.html" class="struct" title="struct datafusion::common::spans::Location">Location</a>  
Represents a location, determined by a line and a column number, in the original SQL query.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/spans/struct.Span.html" class="struct" title="struct datafusion::common::spans::Span">Span</a>  
Represents an interval of characters in the original SQL query.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/spans/struct.Spans.html" class="struct" title="struct datafusion::common::spans::Spans">Spans</a>  
A collection of [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Span.html "struct datafusion::common::Span"), meant to be used as a field of entities whose location in the original SQL query is desired to be tracked. Sometimes an entity can have multiple spans. e.g. if you want to track the position of the column a that comes from SELECT 1 AS a UNION ALL SELECT 2 AS a you’ll need two spans.
