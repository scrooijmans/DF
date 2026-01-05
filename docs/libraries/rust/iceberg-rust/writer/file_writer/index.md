# Module file_writer Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/mod.rs.html#18-50" class="src">Source</a>

Expand description

This module contains the writer for data file format supported by iceberg: parquet, orc.

## Modules<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/index.html" class="mod" title="mod iceberg::writer::file_writer::location_generator">location_generator</a>  
This module contains the location generator and file name generator for generating path of data file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/index.html" class="mod" title="mod iceberg::writer::file_writer::rolling_writer">rolling_writer</a>  
Module providing writers that can automatically roll over to new files based on size thresholds.

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriter">ParquetWriter</a>  
\`ParquetWriter\`\` is used to write arrow data into parquet file on storage.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::ParquetWriterBuilder">ParquetWriterBuilder</a>  
ParquetWriterBuilder is used to builder a [`ParquetWriter`](https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/struct.ParquetWriter.html "struct iceberg::writer::file_writer::ParquetWriter")

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriter.html" class="trait" title="trait iceberg::writer::file_writer::FileWriter">FileWriter</a>  
File writer focus on writing record batch to different physical file format.(Such as parquet. orc)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/trait.FileWriterBuilder.html" class="trait" title="trait iceberg::writer::file_writer::FileWriterBuilder">FileWriterBuilder</a>  
File writer builder trait.
