# Module arrow Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/arrow/mod.rs.html#18-37" class="src">Source</a>

Expand description

Conversion between Iceberg and Arrow schema

## Modules<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/delete_file_loader/index.html" class="mod" title="mod iceberg::arrow::delete_file_loader">delete_file_loader</a>  
Delete File loader

## Structs<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowArrayAccessor.html" class="struct" title="struct iceberg::arrow::ArrowArrayAccessor">ArrowArrayAccessor</a>  
Partner type representing accessing and walking arrow arrays alongside iceberg schema

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowFileReader.html" class="struct" title="struct iceberg::arrow::ArrowFileReader">ArrowFileReader</a>  
ArrowFileReader is a wrapper around a FileRead that impls parquets AsyncFileReader.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReader.html" class="struct" title="struct iceberg::arrow::ArrowReader">ArrowReader</a>  
Reads data from Parquet files

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/struct.ArrowReaderBuilder.html" class="struct" title="struct iceberg::arrow::ArrowReaderBuilder">ArrowReaderBuilder</a>  
Builder to create ArrowReader

## Enums<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/enum.FieldMatchMode.html" class="enum" title="enum iceberg::arrow::FieldMatchMode">FieldMatchMode</a>  
Defines how Arrow fields are matched with Iceberg fields when converting data.

## Constants<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/constant.DEFAULT_MAP_FIELD_NAME.html" class="constant" title="constant iceberg::arrow::DEFAULT_MAP_FIELD_NAME">DEFAULT_MAP_FIELD_NAME</a>  
When iceberg map type convert to Arrow map type, the default map field name is “key_value”.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/constant.UTC_TIME_ZONE.html" class="constant" title="constant iceberg::arrow::UTC_TIME_ZONE">UTC_TIME_ZONE</a>  
UTC time zone for Arrow timestamp type.

## Traits<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/trait.ArrowSchemaVisitor.html" class="trait" title="trait iceberg::arrow::ArrowSchemaVisitor">ArrowSchemaVisitor</a>  
A post order arrow schema visitor.

## Functions<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/fn.arrow_primitive_to_literal.html" class="fn" title="fn iceberg::arrow::arrow_primitive_to_literal">arrow_primitive_to_literal</a>  
Convert arrow primitive array to iceberg primitive value array. This function will assume the schema of arrow struct array is the same as iceberg struct type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/fn.arrow_schema_to_schema.html" class="fn" title="fn iceberg::arrow::arrow_schema_to_schema">arrow_schema_to_schema</a>  
Convert Arrow schema to Iceberg schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/fn.arrow_struct_to_literal.html" class="fn" title="fn iceberg::arrow::arrow_struct_to_literal">arrow_struct_to_literal</a>  
Convert arrow struct array to iceberg struct value array. This function will assume the schema of arrow struct array is the same as iceberg struct type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/fn.arrow_type_to_type.html" class="fn" title="fn iceberg::arrow::arrow_type_to_type">arrow_type_to_type</a>  
Convert Arrow type to iceberg type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/fn.schema_to_arrow_schema.html" class="fn" title="fn iceberg::arrow::schema_to_arrow_schema">schema_to_arrow_schema</a>  
Convert iceberg schema to an arrow schema.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/arrow/fn.type_to_arrow_type.html" class="fn" title="fn iceberg::arrow::type_to_arrow_type">type_to_arrow_type</a>  
Convert iceberg type to an arrow type.
