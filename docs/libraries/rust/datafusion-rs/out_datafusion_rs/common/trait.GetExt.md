# Trait GetExt Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/file_options/file_type.rs.html#35" class="src">Source</a>

``` rust
pub trait GetExt {
    // Required method
    fn get_ext(&self) -> String;
}
```

Expand description

Define each `FileType`/`FileCompressionType`’s extension

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#tymethod.get_ext" class="fn">get_ext</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

File extension getter

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#impl-GetExt-for-ArrowFormatFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/arrow/struct.ArrowFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::arrow::ArrowFormatFactory">ArrowFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#impl-GetExt-for-AvroFormatFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/avro/struct.AvroFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::avro::AvroFormatFactory">AvroFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#impl-GetExt-for-CsvFormatFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvFormatFactory">CsvFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#impl-GetExt-for-FileCompressionType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/file_compression_type/struct.FileCompressionType.html" class="struct" title="struct datafusion::datasource::file_format::file_compression_type::FileCompressionType">FileCompressionType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#impl-GetExt-for-JsonFormatFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonFormatFactory">JsonFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#impl-GetExt-for-ParquetFormatFactory" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/parquet/struct.ParquetFormatFactory.html" class="struct" title="struct datafusion::datasource::file_format::parquet::ParquetFormatFactory">ParquetFormatFactory</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html#impl-GetExt-for-DefaultFileType" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.GetExt.html" class="trait" title="trait datafusion::common::GetExt">GetExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/struct.DefaultFileType.html" class="struct" title="struct datafusion::datasource::file_format::DefaultFileType">DefaultFileType</a>
