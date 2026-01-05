# Module resolve Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/lib.rs.html#50" class="src">Source</a>

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/resolve/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/resolve/fn.resolve_table_references.html" class="fn" title="fn datafusion::sql::resolve::resolve_table_references">resolve_table_references</a>  
Collects all tables and views referenced in the SQL statement. CTEs are collected separately. This can be used to determine which tables need to be in the catalog for a query to be planned.
