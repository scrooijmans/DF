# Parquet Export – DuckDB

Documentation / Guides / File Formats

Parquet Export

To export the data from a table to a Parquet file, use the `COPY` statement:

```
COPY tbl TO 'output.parquet' (FORMAT parquet);

```

The result of queries can also be directly exported to a Parquet file:

```
COPY (SELECT * FROM tbl) TO 'output.parquet' (FORMAT parquet);

```

The flags for setting compression, row group size, etc. are listed in the [Reading and Writing Parquet files](https://duckdb.org/docs/stable/data/parquet/overview.html) page.

##### About this page

- [Report content issue](<https://github.com/duckdb/duckdb-web/issues/new?title=Issue%20found%20on%20page%20%27Parquet%20Export%27&labels=issue%20found%20on%20page&body=%0A%3E%20Please%20describe%20the%20problem%20you%20encountered%20in%20the%20DuckDB%20documentation%20and%20include%20the%20%22Page%20URL%22%20link%20shown%20below.%0A%3E%20Note:%20only%20create%20an%20issue%20if%20you%20wish%20to%20report%20a%20problem%20with%20the%20DuckDB%20documentation.%20For%20questions%20about%20DuckDB%20or%20the%20use%20of%20certain%20DuckDB%20features,%20use%20[GitHub%20Discussions](https://github.com/duckdb/duckdb/discussions/),%20[Stack%20Overflow](https://stackoverflow.com/questions/tagged/duckdb),%20or%20[Discord](https://discord.duckdb.org/).%0A%0APage%20URL:%20%3Chttps://duckdb.org/docs/stable/guides/file_formats/parquet_export.html%3E%0A> "Create GitHub issue")
- [See this page as Markdown](https://duckdb.org/docs/stable/guides/file_formats/parquet_export.md "See Markdown")
- [Edit this page on GitHub](https://github.com/duckdb/duckdb-web/edit/main/docs/stable/guides/file_formats/parquet_export.md "Go to GitHub")

© 2025 DuckDB Foundation, Amsterdam NL

[Code of Conduct](https://duckdb.org/code_of_conduct.html) [Trademark Use](https://duckdb.org/trademark_guidelines.html)
