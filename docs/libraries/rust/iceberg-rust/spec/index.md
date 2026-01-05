# Module spec Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/mod.rs.html#18-54" class="src">Source</a>

Expand description

Spec for Iceberg.

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.BlobMetadata.html" class="struct" title="struct iceberg::spec::BlobMetadata">BlobMetadata</a>  
Represents a blob of metadata, which is a part of a statistics file

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ByteBuf.html" class="struct" title="struct iceberg::spec::ByteBuf">ByteBuf</a>  
Wrapper around `Vec<u8>` to serialize and deserialize efficiently.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>  
Data file carries data file path, partition tuple, metrics, …

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html" class="struct" title="struct iceberg::spec::DataFileBuilder">DataFileBuilder</a>  
Builder for [`DataFile`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html).

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>  
Literal associated with its type. The value and type pair is checked when construction, so the type and value is guaranteed to be correct when used.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.EncryptedKey.html" class="struct" title="struct iceberg::spec::EncryptedKey">EncryptedKey</a>  
Keys used for table encryption

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.FieldSummary.html" class="struct" title="struct iceberg::spec::FieldSummary">FieldSummary</a>  
Field summary for partition field in the spec.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ListType.html" class="struct" title="struct iceberg::spec::ListType">ListType</a>  
A list is a collection of values with some element type. The element field has an integer id that is unique in the table schema. Elements can be either optional or required. Element types may be any type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>  
A manifest contains metadata and a list of entries.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html" class="struct" title="struct iceberg::spec::ManifestEntry">ManifestEntry</a>  
A manifest is an immutable Avro file that lists data files or delete files, along with each file’s partition data tuple, metrics, and tracking information.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestFile.html" class="struct" title="struct iceberg::spec::ManifestFile">ManifestFile</a>  
Entry in a manifest list.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestList.html" class="struct" title="struct iceberg::spec::ManifestList">ManifestList</a>  
Snapshots are embedded in table metadata, but the list of manifests for a snapshot are stored in a separate manifest list file.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestListWriter.html" class="struct" title="struct iceberg::spec::ManifestListWriter">ManifestListWriter</a>  
A manifest list writer.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestMetadata.html" class="struct" title="struct iceberg::spec::ManifestMetadata">ManifestMetadata</a>  
Meta data of a manifest that is stored in the key-value metadata of the Avro file

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html" class="struct" title="struct iceberg::spec::ManifestWriter">ManifestWriter</a>  
A manifest writer.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriterBuilder.html" class="struct" title="struct iceberg::spec::ManifestWriterBuilder">ManifestWriterBuilder</a>  
The builder used to create a [`ManifestWriter`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestWriter.html "struct iceberg::spec::ManifestWriter").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>  
Map is a collection of key-value pairs with a key type and a value type. It used in Literal::Map, to make it hashable, the order of key-value pairs is stored in a separate vector so that we can hash the map in a deterministic way. But it also means that the order of key-value pairs is matter for the hash value.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MapType.html" class="struct" title="struct iceberg::spec::MapType">MapType</a>  
A map is a collection of key-value pairs with a key type and a value type. Both the key field and value field each have an integer id that is unique in the table schema. Map keys are required and map values can be either optional or required. Both map keys and map values may be any type, including nested types.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MappedField.html" class="struct" title="struct iceberg::spec::MappedField">MappedField</a>  
Maps field names to IDs.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.MetadataLog.html" class="struct" title="struct iceberg::spec::MetadataLog">MetadataLog</a>  
Encodes changes to the previous metadata files for the table

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NameMapping.html" class="struct" title="struct iceberg::spec::NameMapping">NameMapping</a>  
Iceberg fallback field name to ID mapping.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.NestedField.html" class="struct" title="struct iceberg::spec::NestedField">NestedField</a>  
A struct is a tuple of typed values. Each field in the tuple is named and has an integer id that is unique in the table schema. Each field can be either optional or required, meaning that values can (or cannot) be null. Fields may be any type. Fields may have an optional comment or doc string. Fields can have default values.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionField.html" class="struct" title="struct iceberg::spec::PartitionField">PartitionField</a>  
Partition fields capture the transform from table data to partition values.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionKey.html" class="struct" title="struct iceberg::spec::PartitionKey">PartitionKey</a>  
A partition key represents a specific partition in a table, containing the partition spec, schema, and the actual partition values.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html" class="struct" title="struct iceberg::spec::PartitionSpec">PartitionSpec</a>  
Partition spec that defines how to produce a tuple of partition values from a record.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::PartitionSpecBuilder">PartitionSpecBuilder</a>  
Create valid partition specs for a given schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionStatisticsFile.html" class="struct" title="struct iceberg::spec::PartitionStatisticsFile">PartitionStatisticsFile</a>  
Statistics file for a partition

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.RawLiteral.html" class="struct" title="struct iceberg::spec::RawLiteral">RawLiteral</a>  
Raw literal representation used for serde. The serialize way is used for Avro serializer.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html" class="struct" title="struct iceberg::spec::Schema">Schema</a>  
Defines schema in iceberg.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SchemaBuilder.html" class="struct" title="struct iceberg::spec::SchemaBuilder">SchemaBuilder</a>  
Schema builder.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html" class="struct" title="struct iceberg::spec::Snapshot">Snapshot</a>  
A snapshot represents the state of a table at some time and is used to access the complete set of data files in the table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotLog.html" class="struct" title="struct iceberg::spec::SnapshotLog">SnapshotLog</a>  
A log of when each snapshot was made.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotReference.html" class="struct" title="struct iceberg::spec::SnapshotReference">SnapshotReference</a>  
Iceberg tables keep track of branches and tags using snapshot references.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SnapshotSummaryCollector.html" class="struct" title="struct iceberg::spec::SnapshotSummaryCollector">SnapshotSummaryCollector</a>  
`SnapshotSummaryCollector` collects and aggregates snapshot update metrics. It gathers metrics about added or removed data files and manifests, and tracks partition-specific updates.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortField.html" class="struct" title="struct iceberg::spec::SortField">SortField</a>  
Entry for every column that is to be sorted

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>  
A sort order is defined by a sort order id and a list of sort fields. The order of the sort fields within the list defines the order in which the sort is applied to the data.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html" class="struct" title="struct iceberg::spec::SortOrderBuilder">SortOrderBuilder</a>  
Builder for [`SortOrder`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html).

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SqlViewRepresentation.html" class="struct" title="struct iceberg::spec::SqlViewRepresentation">SqlViewRepresentation</a>  
The SQL representation stores the view definition as a SQL SELECT, with metadata such as the SQL dialect.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StatisticsFile.html" class="struct" title="struct iceberg::spec::StatisticsFile">StatisticsFile</a>  
Represents a statistics file

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Struct.html" class="struct" title="struct iceberg::spec::Struct">Struct</a>  
The partition struct stores the tuple of partition values for each file. Its type is derived from the partition fields of the partition spec used to write the manifest file. In v2, the partition struct’s field ids must match the ids from the partition spec.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.StructType.html" class="struct" title="struct iceberg::spec::StructType">StructType</a>  
DataType for a specific struct

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Summary.html" class="struct" title="struct iceberg::spec::Summary">Summary</a>  
Summarises the changes in the snapshot.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html" class="struct" title="struct iceberg::spec::TableMetadata">TableMetadata</a>  
Fields for the version 2 of the table metadata.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuildResult.html" class="struct" title="struct iceberg::spec::TableMetadataBuildResult">TableMetadataBuildResult</a>  
Result of modifying or creating a `TableMetadata`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadataBuilder.html" class="struct" title="struct iceberg::spec::TableMetadataBuilder">TableMetadataBuilder</a>  
Manipulating table metadata.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionField.html" class="struct" title="struct iceberg::spec::UnboundPartitionField">UnboundPartitionField</a>  
Unbound partition field can be built without a schema and later bound to a schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpec">UnboundPartitionSpec</a>  
Unbound partition spec can be built without a schema and later bound to a schema. They are used to transport schema information as part of the REST specification. The main difference to [`PartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html "struct iceberg::spec::PartitionSpec") is that the field ids are optional.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpecBuilder.html" class="struct" title="struct iceberg::spec::UnboundPartitionSpecBuilder">UnboundPartitionSpecBuilder</a>  
Create a new UnboundPartitionSpec

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html" class="struct" title="struct iceberg::spec::ViewMetadata">ViewMetadata</a>  
Fields for the version 1 of the view metadata.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadataBuilder.html" class="struct" title="struct iceberg::spec::ViewMetadataBuilder">ViewMetadataBuilder</a>  
Manipulating view metadata.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>  
A list of view representations.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html" class="struct" title="struct iceberg::spec::ViewVersion">ViewVersion</a>  
A view versions represents the definition of a view at a specific point in time.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersionLog.html" class="struct" title="struct iceberg::spec::ViewVersionLog">ViewVersionLog</a>  
A log of when each snapshot was made.

## Enums<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataContentType.html" class="enum" title="enum iceberg::spec::DataContentType">DataContentType</a>  
Type of content stored by the data file: data, equality deletes, or position deletes (all v1 files are data files)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataFileBuilderError.html" class="enum" title="enum iceberg::spec::DataFileBuilderError">DataFileBuilderError</a>  
Error type for DataFileBuilder

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataFileFormat.html" class="enum" title="enum iceberg::spec::DataFileFormat">DataFileFormat</a>  
Format of this data.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.FormatVersion.html" class="enum" title="enum iceberg::spec::FormatVersion">FormatVersion</a>  
Iceberg format version

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>  
Values present in iceberg type

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ManifestContentType.html" class="enum" title="enum iceberg::spec::ManifestContentType">ManifestContentType</a>  
The type of files tracked by the manifest, either data or delete files; Data(0) for all v1 manifests

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ManifestStatus.html" class="enum" title="enum iceberg::spec::ManifestStatus">ManifestStatus</a>  
Used to track additions and deletions in ManifestEntry.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.NullOrder.html" class="enum" title="enum iceberg::spec::NullOrder">NullOrder</a>  
Describes the order of null values when sorted.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Operation.html" class="enum" title="enum iceberg::spec::Operation">Operation</a>  
The operation field is used by some operations, like snapshot expiration, to skip processing certain snapshots.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveLiteral.html" class="enum" title="enum iceberg::spec::PrimitiveLiteral">PrimitiveLiteral</a>  
Values present in iceberg type

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.PrimitiveType.html" class="enum" title="enum iceberg::spec::PrimitiveType">PrimitiveType</a>  
Primitive data types

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SnapshotRetention.html" class="enum" title="enum iceberg::spec::SnapshotRetention">SnapshotRetention</a>  
The snapshot expiration procedure removes snapshots from table metadata and applies the table’s retention policy.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortDirection.html" class="enum" title="enum iceberg::spec::SortDirection">SortDirection</a>  
Sort direction in a partition, either ascending or descending

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.SortOrderBuilderError.html" class="enum" title="enum iceberg::spec::SortOrderBuilderError">SortOrderBuilderError</a>  
Error type for SortOrderBuilder

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>  
Transform is used to transform predicates to partition predicates, in addition to transforming data values.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>  
All data types are either primitives or nested types, which are maps, lists, or structs.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewFormatVersion.html" class="enum" title="enum iceberg::spec::ViewFormatVersion">ViewFormatVersion</a>  
Iceberg format version

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewRepresentation.html" class="enum" title="enum iceberg::spec::ViewRepresentation">ViewRepresentation</a>  
View definitions can be represented in multiple ways. Representations are documented ways to express a view definition.

## Constants<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.DEFAULT_SCHEMA_ID.html" class="constant" title="constant iceberg::spec::DEFAULT_SCHEMA_ID">DEFAULT_SCHEMA_ID</a>  
Default schema id.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.DEFAULT_SCHEMA_NAME_MAPPING.html" class="constant" title="constant iceberg::spec::DEFAULT_SCHEMA_NAME_MAPPING">DEFAULT_SCHEMA_NAME_MAPPING</a>  
Property name for name mapping.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.LIST_FIELD_NAME.html" class="constant" title="constant iceberg::spec::LIST_FIELD_NAME">LIST_FIELD_NAME</a>  
Field name for list type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.MAIN_BRANCH.html" class="constant" title="constant iceberg::spec::MAIN_BRANCH">MAIN_BRANCH</a>  
The ref name of the main branch of the table.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.MAP_KEY_FIELD_NAME.html" class="constant" title="constant iceberg::spec::MAP_KEY_FIELD_NAME">MAP_KEY_FIELD_NAME</a>  
Field name for map type’s key.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.MAP_VALUE_FIELD_NAME.html" class="constant" title="constant iceberg::spec::MAP_VALUE_FIELD_NAME">MAP_VALUE_FIELD_NAME</a>  
Field name for map type’s value.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_COMMIT_MAX_RETRY_WAIT_MS.html" class="constant" title="constant iceberg::spec::PROPERTY_COMMIT_MAX_RETRY_WAIT_MS">PROPERTY_COMMIT_MAX_RETRY_WAIT_MS</a>  
Property key for maximum wait time (ms) between retries.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_COMMIT_MAX_RETRY_WAIT_MS_DEFAULT.html" class="constant" title="constant iceberg::spec::PROPERTY_COMMIT_MAX_RETRY_WAIT_MS_DEFAULT">PROPERTY_COMMIT_MAX_RETRY_WAIT_MS_DEFAULT</a>  
Default value for maximum wait time (ms) between retries.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_COMMIT_MIN_RETRY_WAIT_MS.html" class="constant" title="constant iceberg::spec::PROPERTY_COMMIT_MIN_RETRY_WAIT_MS">PROPERTY_COMMIT_MIN_RETRY_WAIT_MS</a>  
Property key for minimum wait time (ms) between retries.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_COMMIT_MIN_RETRY_WAIT_MS_DEFAULT.html" class="constant" title="constant iceberg::spec::PROPERTY_COMMIT_MIN_RETRY_WAIT_MS_DEFAULT">PROPERTY_COMMIT_MIN_RETRY_WAIT_MS_DEFAULT</a>  
Default value for minimum wait time (ms) between retries.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_COMMIT_NUM_RETRIES.html" class="constant" title="constant iceberg::spec::PROPERTY_COMMIT_NUM_RETRIES">PROPERTY_COMMIT_NUM_RETRIES</a>  
Property key for number of commit retries.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_COMMIT_NUM_RETRIES_DEFAULT.html" class="constant" title="constant iceberg::spec::PROPERTY_COMMIT_NUM_RETRIES_DEFAULT">PROPERTY_COMMIT_NUM_RETRIES_DEFAULT</a>  
Default value for number of commit retries.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_COMMIT_TOTAL_RETRY_TIME_MS.html" class="constant" title="constant iceberg::spec::PROPERTY_COMMIT_TOTAL_RETRY_TIME_MS">PROPERTY_COMMIT_TOTAL_RETRY_TIME_MS</a>  
Property key for total maximum retry time (ms).

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_COMMIT_TOTAL_RETRY_TIME_MS_DEFAULT.html" class="constant" title="constant iceberg::spec::PROPERTY_COMMIT_TOTAL_RETRY_TIME_MS_DEFAULT">PROPERTY_COMMIT_TOTAL_RETRY_TIME_MS_DEFAULT</a>  
Default value for total maximum retry time (ms).

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_CURRENT_SCHEMA.html" class="constant" title="constant iceberg::spec::PROPERTY_CURRENT_SCHEMA">PROPERTY_CURRENT_SCHEMA</a>  
Reserved table property for the JSON representation of current schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_CURRENT_SNAPSHOT_ID.html" class="constant" title="constant iceberg::spec::PROPERTY_CURRENT_SNAPSHOT_ID">PROPERTY_CURRENT_SNAPSHOT_ID</a>  
Reserved table property for current snapshot id.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_CURRENT_SNAPSHOT_SUMMARY.html" class="constant" title="constant iceberg::spec::PROPERTY_CURRENT_SNAPSHOT_SUMMARY">PROPERTY_CURRENT_SNAPSHOT_SUMMARY</a>  
Reserved table property for current snapshot summary.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_CURRENT_SNAPSHOT_TIMESTAMP.html" class="constant" title="constant iceberg::spec::PROPERTY_CURRENT_SNAPSHOT_TIMESTAMP">PROPERTY_CURRENT_SNAPSHOT_TIMESTAMP</a>  
Reserved table property for current snapshot timestamp.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_DEFAULT_FILE_FORMAT.html" class="constant" title="constant iceberg::spec::PROPERTY_DEFAULT_FILE_FORMAT">PROPERTY_DEFAULT_FILE_FORMAT</a>  
Default file format for data files

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_DEFAULT_FILE_FORMAT_DEFAULT.html" class="constant" title="constant iceberg::spec::PROPERTY_DEFAULT_FILE_FORMAT_DEFAULT">PROPERTY_DEFAULT_FILE_FORMAT_DEFAULT</a>  
Default value for data file format

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_DEFAULT_PARTITION_SPEC.html" class="constant" title="constant iceberg::spec::PROPERTY_DEFAULT_PARTITION_SPEC">PROPERTY_DEFAULT_PARTITION_SPEC</a>  
Reserved table property for the JSON representation of current(default) partition spec.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_DEFAULT_SORT_ORDER.html" class="constant" title="constant iceberg::spec::PROPERTY_DEFAULT_SORT_ORDER">PROPERTY_DEFAULT_SORT_ORDER</a>  
Reserved table property for the JSON representation of current(default) sort order.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_DELETE_DEFAULT_FILE_FORMAT.html" class="constant" title="constant iceberg::spec::PROPERTY_DELETE_DEFAULT_FILE_FORMAT">PROPERTY_DELETE_DEFAULT_FILE_FORMAT</a>  
Default file format for delete files

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_FORMAT_VERSION.html" class="constant" title="constant iceberg::spec::PROPERTY_FORMAT_VERSION">PROPERTY_FORMAT_VERSION</a>  
Reserved table property for table format version.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_METADATA_PREVIOUS_VERSIONS_MAX.html" class="constant" title="constant iceberg::spec::PROPERTY_METADATA_PREVIOUS_VERSIONS_MAX">PROPERTY_METADATA_PREVIOUS_VERSIONS_MAX</a>  
Property key for max number of previous versions to keep.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_METADATA_PREVIOUS_VERSIONS_MAX_DEFAULT.html" class="constant" title="constant iceberg::spec::PROPERTY_METADATA_PREVIOUS_VERSIONS_MAX_DEFAULT">PROPERTY_METADATA_PREVIOUS_VERSIONS_MAX_DEFAULT</a>  
Default value for max number of previous versions to keep.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_SNAPSHOT_COUNT.html" class="constant" title="constant iceberg::spec::PROPERTY_SNAPSHOT_COUNT">PROPERTY_SNAPSHOT_COUNT</a>  
Reserved table property for the total number of snapshots.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_UUID.html" class="constant" title="constant iceberg::spec::PROPERTY_UUID">PROPERTY_UUID</a>  
Reserved table property for table UUID.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_WRITE_PARTITION_SUMMARY_LIMIT.html" class="constant" title="constant iceberg::spec::PROPERTY_WRITE_PARTITION_SUMMARY_LIMIT">PROPERTY_WRITE_PARTITION_SUMMARY_LIMIT</a>  
Property key for max number of partitions to keep summary stats for.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_WRITE_PARTITION_SUMMARY_LIMIT_DEFAULT.html" class="constant" title="constant iceberg::spec::PROPERTY_WRITE_PARTITION_SUMMARY_LIMIT_DEFAULT">PROPERTY_WRITE_PARTITION_SUMMARY_LIMIT_DEFAULT</a>  
Default value for the max number of partitions to keep summary stats for.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_WRITE_TARGET_FILE_SIZE_BYTES.html" class="constant" title="constant iceberg::spec::PROPERTY_WRITE_TARGET_FILE_SIZE_BYTES">PROPERTY_WRITE_TARGET_FILE_SIZE_BYTES</a>  
Target file size for newly written files.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.PROPERTY_WRITE_TARGET_FILE_SIZE_BYTES_DEFAULT.html" class="constant" title="constant iceberg::spec::PROPERTY_WRITE_TARGET_FILE_SIZE_BYTES_DEFAULT">PROPERTY_WRITE_TARGET_FILE_SIZE_BYTES_DEFAULT</a>  
Default target file size

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.RESERVED_PROPERTIES.html" class="constant" title="constant iceberg::spec::RESERVED_PROPERTIES">RESERVED_PROPERTIES</a>  
Reserved Iceberg table properties list.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.UNASSIGNED_SEQUENCE_NUMBER.html" class="constant" title="constant iceberg::spec::UNASSIGNED_SEQUENCE_NUMBER">UNASSIGNED_SEQUENCE_NUMBER</a>  
Placeholder for sequence number. The field with this value must be replaced with the actual sequence number before it write.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.UNASSIGNED_SNAPSHOT_ID.html" class="constant" title="constant iceberg::spec::UNASSIGNED_SNAPSHOT_ID">UNASSIGNED_SNAPSHOT_ID</a>  
Placeholder for snapshot ID. The field with this value must be replaced with the actual snapshot ID before it is committed.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.VIEW_PROPERTY_REPLACE_DROP_DIALECT_ALLOWED.html" class="constant" title="constant iceberg::spec::VIEW_PROPERTY_REPLACE_DROP_DIALECT_ALLOWED">VIEW_PROPERTY_REPLACE_DROP_DIALECT_ALLOWED</a>  
Property key for allowing to drop dialects when replacing a view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.VIEW_PROPERTY_REPLACE_DROP_DIALECT_ALLOWED_DEFAULT.html" class="constant" title="constant iceberg::spec::VIEW_PROPERTY_REPLACE_DROP_DIALECT_ALLOWED_DEFAULT">VIEW_PROPERTY_REPLACE_DROP_DIALECT_ALLOWED_DEFAULT</a>  
Default value for the property key for allowing to drop dialects when replacing a view.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.VIEW_PROPERTY_VERSION_HISTORY_SIZE.html" class="constant" title="constant iceberg::spec::VIEW_PROPERTY_VERSION_HISTORY_SIZE">VIEW_PROPERTY_VERSION_HISTORY_SIZE</a>  
Property key for the number of history entries to keep.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/constant.VIEW_PROPERTY_VERSION_HISTORY_SIZE_DEFAULT.html" class="constant" title="constant iceberg::spec::VIEW_PROPERTY_VERSION_HISTORY_SIZE_DEFAULT">VIEW_PROPERTY_VERSION_HISTORY_SIZE_DEFAULT</a>  
Default value for the property key for the number of history entries to keep.

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.PartnerAccessor.html" class="trait" title="trait iceberg::spec::PartnerAccessor">PartnerAccessor</a>  
Accessor used to get child partner from parent partner.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaVisitor.html" class="trait" title="trait iceberg::spec::SchemaVisitor">SchemaVisitor</a>  
A post order schema visitor.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/trait.SchemaWithPartnerVisitor.html" class="trait" title="trait iceberg::spec::SchemaWithPartnerVisitor">SchemaWithPartnerVisitor</a>  
A post order schema visitor with partner.

## Functions<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.deserialize_data_file_from_json.html" class="fn" title="fn iceberg::spec::deserialize_data_file_from_json">deserialize_data_file_from_json</a>  
Deserialize a DataFile from a JSON string.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.prune_columns.html" class="fn" title="fn iceberg::spec::prune_columns">prune_columns</a>  
Visit a schema and returns only the fields selected by id set

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.read_data_files_from_avro.html" class="fn" title="fn iceberg::spec::read_data_files_from_avro">read_data_files_from_avro</a>  
Parse data files from avro bytes.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.serialize_data_file_to_json.html" class="fn" title="fn iceberg::spec::serialize_data_file_to_json">serialize_data_file_to_json</a>  
Serialize a DataFile to a JSON string.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.visit_schema.html" class="fn" title="fn iceberg::spec::visit_schema">visit_schema</a>  
Visit schema in post order.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.visit_schema_with_partner.html" class="fn" title="fn iceberg::spec::visit_schema_with_partner">visit_schema_with_partner</a>  
Visit schema in post order.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.visit_struct.html" class="fn" title="fn iceberg::spec::visit_struct">visit_struct</a>  
Visit struct type in post order.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.visit_struct_with_partner.html" class="fn" title="fn iceberg::spec::visit_struct_with_partner">visit_struct_with_partner</a>  
Visit struct type in post order.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/fn.write_data_files_to_avro.html" class="fn" title="fn iceberg::spec::write_data_files_to_avro">write_data_files_to_avro</a>  
Convert data files to avro bytes and write to writer. Return the bytes written.

## Type Aliases<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ManifestEntryRef.html" class="type" title="type iceberg::spec::ManifestEntryRef">ManifestEntryRef</a>  
Reference to [`ManifestEntry`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestEntry.html "struct iceberg::spec::ManifestEntry").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.NestedFieldRef.html" class="type" title="type iceberg::spec::NestedFieldRef">NestedFieldRef</a>  
Reference to nested field.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.PartitionSpecRef.html" class="type" title="type iceberg::spec::PartitionSpecRef">PartitionSpecRef</a>  
Reference to [`PartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.PartitionSpec.html "struct iceberg::spec::PartitionSpec").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaId.html" class="type" title="type iceberg::spec::SchemaId">SchemaId</a>  
Type alias for schema id.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SchemaRef.html" class="type" title="type iceberg::spec::SchemaRef">SchemaRef</a>  
Reference to [`Schema`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Schema.html "struct iceberg::spec::Schema").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SnapshotRef.html" class="type" title="type iceberg::spec::SnapshotRef">SnapshotRef</a>  
Reference to [`Snapshot`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Snapshot.html "struct iceberg::spec::Snapshot").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.SortOrderRef.html" class="type" title="type iceberg::spec::SortOrderRef">SortOrderRef</a>  
Reference to [`SortOrder`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html "struct iceberg::spec::SortOrder").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.TableMetadataRef.html" class="type" title="type iceberg::spec::TableMetadataRef">TableMetadataRef</a>  
Reference to [`TableMetadata`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.TableMetadata.html "struct iceberg::spec::TableMetadata").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.UnboundPartitionSpecRef.html" class="type" title="type iceberg::spec::UnboundPartitionSpecRef">UnboundPartitionSpecRef</a>  
Reference to [`UnboundPartitionSpec`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.UnboundPartitionSpec.html "struct iceberg::spec::UnboundPartitionSpec").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewMetadataRef.html" class="type" title="type iceberg::spec::ViewMetadataRef">ViewMetadataRef</a>  
Reference to [`ViewMetadata`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewMetadata.html "struct iceberg::spec::ViewMetadata").

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewVersionId.html" class="type" title="type iceberg::spec::ViewVersionId">ViewVersionId</a>  
Alias for the integer type used for view version ids.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/type.ViewVersionRef.html" class="type" title="type iceberg::spec::ViewVersionRef">ViewVersionRef</a>  
Reference to [`ViewVersion`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewVersion.html "struct iceberg::spec::ViewVersion").
