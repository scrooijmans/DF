# Crate csv Copy item path

<a href="https://docs.rs/arrow-csv/56.0.0/x86_64-unknown-linux-gnu/src/arrow_csv/lib.rs.html#18-62" class="src">Source</a>

Expand description

Transfer data between the Arrow memory format and CSV (comma-separated values).

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/reader/index.html" class="mod" title="mod datafusion::common::arrow::csv::reader">reader</a>  
CSV Reader

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/writer/index.html" class="mod" title="mod datafusion::common::arrow::csv::writer">writer</a>  
CSV Writer

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.ReaderBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::ReaderBuilder">ReaderBuilder</a>  
CSV file reader builder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::csv::Writer">Writer</a>  
A CSV writer

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::csv::WriterBuilder">WriterBuilder</a>  
A CSV writer builder

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/fn.infer_schema_from_files.html" class="fn" title="fn datafusion::common::arrow::csv::infer_schema_from_files">infer_schema_from_files</a>  
Infer schema from a list of CSV files by reading through first n records with `max_read_records` controlling the maximum number of records to read.

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/csv/type.Reader.html" class="type" title="type datafusion::common::arrow::csv::Reader">Reader</a>  
CSV file reader using [`std::io::BufReader`](https://doc.rust-lang.org/nightly/std/io/buffered/bufreader/struct.BufReader.html "struct std::io::buffered::bufreader::BufReader")
