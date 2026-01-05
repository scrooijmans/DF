# Module parser Copy item path

<a href="https://docs.rs/datafusion-sql/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_sql/lib.rs.html#46" class="src">Source</a>

Expand description

[`DFParser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html "struct datafusion::sql::parser::DFParser"): DataFusion SQL Parser based on [`sqlparser`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/index.html "mod datafusion::logical_expr::sqlparser")

This parser implements DataFusion specific statements such as `CREATE EXTERNAL TABLE`

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.CopyToStatement.html" class="struct" title="struct datafusion::sql::parser::CopyToStatement">CopyToStatement</a>  
DataFusion extension DDL for `COPY`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.CreateExternalTable.html" class="struct" title="struct datafusion::sql::parser::CreateExternalTable">CreateExternalTable</a>  
DataFusion extension DDL for `CREATE EXTERNAL TABLE`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html" class="struct" title="struct datafusion::sql::parser::DFParser">DFParser</a>  
DataFusion SQL Parser based on [`sqlparser`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/index.html "mod datafusion::logical_expr::sqlparser")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParserBuilder.html" class="struct" title="struct datafusion::sql::parser::DFParserBuilder">DFParserBuilder</a>  
Builder for [`DFParser`](https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.DFParser.html "struct datafusion::sql::parser::DFParser")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/struct.ExplainStatement.html" class="struct" title="struct datafusion::sql::parser::ExplainStatement">ExplainStatement</a>  
DataFusion specific `EXPLAIN`

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.CopyToSource.html" class="enum" title="enum datafusion::sql::parser::CopyToSource">CopyToSource</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/sql/parser/enum.Statement.html" class="enum" title="enum datafusion::sql::parser::Statement">Statement</a>  
DataFusion SQL Statement.
