# Module location_generator Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/writer/file_writer/location_generator.rs.html#18-310" class="src">Source</a>

Expand description

This module contains the location generator and file name generator for generating path of data file.

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultFileNameGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultFileNameGenerator">DefaultFileNameGenerator</a>  
`DefaultFileNameGenerator` used to generate file name for data file. The file name can be passed to `LocationGenerator` to generate the location of the file. The file name format is “{prefix}-{file_count}\[-{suffix}\].{file_format}”.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/struct.DefaultLocationGenerator.html" class="struct" title="struct iceberg::writer::file_writer::location_generator::DefaultLocationGenerator">DefaultLocationGenerator</a>  
`DefaultLocationGenerator` used to generate the data dir location of data file. The location is generated based on the table location and the data location in table properties.

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.FileNameGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::FileNameGenerator">FileNameGenerator</a>  
`FileNameGeneratorTrait` used to generate file name for data file. The file name can be passed to `LocationGenerator` to generate the location of the file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/writer/file_writer/location_generator/trait.LocationGenerator.html" class="trait" title="trait iceberg::writer::file_writer::location_generator::LocationGenerator">LocationGenerator</a>  
`LocationGenerator` used to generate the location of data file.
