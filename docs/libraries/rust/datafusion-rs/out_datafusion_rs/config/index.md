# Module config Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/lib.rs.html#40" class="src">Source</a>

Expand description

Runtime configuration, via [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CatalogOptions.html" class="struct" title="struct datafusion::config::CatalogOptions">CatalogOptions</a>  
Options related to catalog and directory scanning

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ColumnDecryptionProperties.html" class="struct" title="struct datafusion::config::ColumnDecryptionProperties">ColumnDecryptionProperties</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ColumnEncryptionProperties.html" class="struct" title="struct datafusion::config::ColumnEncryptionProperties">ColumnEncryptionProperties</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigEntry.html" class="struct" title="struct datafusion::config::ConfigEntry">ConfigEntry</a>  
A key value pair, with a corresponding description

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileDecryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileDecryptionProperties">ConfigFileDecryptionProperties</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileEncryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileEncryptionProperties">ConfigFileEncryptionProperties</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html" class="struct" title="struct datafusion::config::ConfigOptions">ConfigOptions</a>  
Configuration options struct, able to store both built-in configuration and custom options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.CsvOptions.html" class="struct" title="struct datafusion::config::CsvOptions">CsvOptions</a>  
Options controlling CSV format

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.EncryptionFactoryOptions.html" class="struct" title="struct datafusion::config::EncryptionFactoryOptions">EncryptionFactoryOptions</a>  
Holds implementation-specific options for an encryption factory

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExecutionOptions.html" class="struct" title="struct datafusion::config::ExecutionOptions">ExecutionOptions</a>  
Options related to query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ExplainOptions.html" class="struct" title="struct datafusion::config::ExplainOptions">ExplainOptions</a>  
Options controlling explain output

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.Extensions.html" class="struct" title="struct datafusion::config::Extensions">Extensions</a>  
A type-safe container for [`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.FormatOptions.html" class="struct" title="struct datafusion::config::FormatOptions">FormatOptions</a>  
Options controlling the format of output when printing record batches Copies [`arrow::util::display::FormatOptions`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/display/struct.FormatOptions.html "struct datafusion::common::arrow::util::display::FormatOptions")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.JsonOptions.html" class="struct" title="struct datafusion::config::JsonOptions">JsonOptions</a>  
Options controlling JSON format

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.OptimizerOptions.html" class="struct" title="struct datafusion::config::OptimizerOptions">OptimizerOptions</a>  
Options related to query optimization

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetColumnOptions.html" class="struct" title="struct datafusion::config::ParquetColumnOptions">ParquetColumnOptions</a>  
Options controlling parquet format for individual columns.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetEncryptionOptions.html" class="struct" title="struct datafusion::config::ParquetEncryptionOptions">ParquetEncryptionOptions</a>  
Options for configuring Parquet Modular Encryption

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ParquetOptions.html" class="struct" title="struct datafusion::config::ParquetOptions">ParquetOptions</a>  
Options for reading and writing parquet files

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.SqlParserOptions.html" class="struct" title="struct datafusion::config::SqlParserOptions">SqlParserOptions</a>  
Options related to SQL parser

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>  
Represents the configuration options available for handling different table formats within a data processing application. This struct encompasses options for various file formats including CSV, Parquet, and JSON, allowing for flexible configuration of parsing and writing behaviors specific to each format. Additionally, it supports extending functionality through custom extensions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableParquetOptions.html" class="struct" title="struct datafusion::config::TableParquetOptions">TableParquetOptions</a>  
Options that control how Parquet files are read, including global options that apply to all columns and optional column-specific overrides

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.ConfigFileType.html" class="enum" title="enum datafusion::config::ConfigFileType">ConfigFileType</a>

These file types have special built in behavior for configuration. Use TableOptions::Extensions for configuring other file types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.OutputFormat.html" class="enum" title="enum datafusion::config::OutputFormat">OutputFormat</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/enum.SpillCompression.html" class="enum" title="enum datafusion::config::SpillCompression">SpillCompression</a>

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html" class="trait" title="trait datafusion::config::ConfigExtension">ConfigExtension</a>  
[`ConfigExtension`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigExtension.html "trait datafusion::config::ConfigExtension") provides a mechanism to store third-party configuration within DataFusion [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html" class="trait" title="trait datafusion::config::ConfigField">ConfigField</a>  
A trait implemented by `config_namespace` and for field types that provides the ability to walk and mutate the configuration tree

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ExtensionOptions.html" class="trait" title="trait datafusion::config::ExtensionOptions">ExtensionOptions</a>  
An object-safe API for storing arbitrary configuration.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.OutputFormatExt.html" class="trait" title="trait datafusion::config::OutputFormatExt">OutputFormatExt</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/trait.Visit.html" class="trait" title="trait datafusion::config::Visit">Visit</a>  
An implementation trait used to recursively walk configuration

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/fn.default_config_transform.html" class="fn" title="fn datafusion::config::default_config_transform">default_config_transform</a>  
Default transformation to parse a [`ConfigField`](https://docs.rs/datafusion/50.2.0/datafusion/config/trait.ConfigField.html "trait datafusion::config::ConfigField") for a string.
