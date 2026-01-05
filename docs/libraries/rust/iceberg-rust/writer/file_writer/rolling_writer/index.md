# Module rolling_writer Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/rolling_writer.rs.html#18-330" class="src">Source</a>

Expand description

Module providing writers that can automatically roll over to new files based on size thresholds.

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriter.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriter">RollingFileWriter</a>  
A writer that automatically rolls over to a new file when the data size exceeds a target threshold.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/rolling_writer/struct.RollingFileWriterBuilder.html" class="struct" title="struct iceberg::writer::file_writer::rolling_writer::RollingFileWriterBuilder">RollingFileWriterBuilder</a>  
Builder for creating a `RollingFileWriter` that rolls over to a new file when the data size exceeds a target threshold.
