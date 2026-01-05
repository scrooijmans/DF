# Struct DFSchema Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/dfschema.rs.html#108" class="src">Source</a>

``` rust
pub struct DFSchema { /* private fields */ }
```

Expand description

DFSchema wraps an Arrow schema and adds relation names.

The schema may hold the fields across multiple tables. Some fields may be qualified and some unqualified. A qualified field is a field that has a relation name associated with it.

Unqualified fields must be unique not only amongst themselves, but also must have a distinct name from any qualified field names. This allows finding a qualified field by name to be possible, so long as there aren’t multiple qualified fields with the same name.

There is an alias to `Arc<DFSchema>` named [DFSchemaRef](https://docs.rs/datafusion/50.2.0/datafusion/common/type.DFSchemaRef.html "type datafusion::common::DFSchemaRef").

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#creating-qualified-schemas" class="doc-anchor">§</a>Creating qualified schemas

Use [DFSchema::try_from_qualified_schema](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.try_from_qualified_schema "associated function datafusion::common::DFSchema::try_from_qualified_schema") to create a qualified schema from an Arrow schema.

``` rust
use datafusion_common::{DFSchema, Column};
use arrow::datatypes::{DataType, Field, Schema};

let arrow_schema = Schema::new(vec![
   Field::new("c1", DataType::Int32, false),
]);

let df_schema = DFSchema::try_from_qualified_schema("t1", &arrow_schema).unwrap();
let column = Column::from_qualified_name("t1.c1");
assert!(df_schema.has_column(&column));

// Can also access qualified fields with unqualified name, if it's unambiguous
let column = Column::from_qualified_name("c1");
assert!(df_schema.has_column(&column));
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#creating-unqualified-schemas" class="doc-anchor">§</a>Creating unqualified schemas

Create an unqualified schema using TryFrom:

``` rust
use datafusion_common::{DFSchema, Column};
use arrow::datatypes::{DataType, Field, Schema};

let arrow_schema = Schema::new(vec![
   Field::new("c1", DataType::Int32, false),
]);

let df_schema = DFSchema::try_from(arrow_schema).unwrap();
let column = Column::new_unqualified("c1");
assert!(df_schema.has_column(&column));
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#converting-back-to-arrow-schema" class="doc-anchor">§</a>Converting back to Arrow schema

Use the `Into` trait to convert `DFSchema` into an Arrow schema:

``` rust
use datafusion_common::DFSchema;
use arrow::datatypes::{Schema, Field};
use std::collections::HashMap;

let df_schema = DFSchema::from_unqualified_fields(vec![
   Field::new("c1", arrow::datatypes::DataType::Int32, false),
].into(),HashMap::new()).unwrap();
let schema = Schema::from(df_schema);
assert_eq!(schema.fields().len(), 1);
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-DFSchema" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.empty" class="fn">empty</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

Creates an empty `DFSchema`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.as_arrow" class="fn">as_arrow</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>

Return a reference to the inner Arrow [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema")

Note this does not have the qualifier information

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.inner" class="fn">inner</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Return a reference to the inner Arrow [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef")

Note this does not have the qualifier information

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.new_with_metadata" class="fn">new_with_metadata</a>( qualified_fields: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)\>, metadata: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a `DFSchema` from an Arrow schema where all the fields have a given qualifier

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.from_unqualified_fields" class="fn">from_unqualified_fields</a>( fields: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>, metadata: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new `DFSchema` from a list of Arrow [Field](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field")s

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.try_from_qualified_schema" class="fn">try_from_qualified_schema</a>( qualifier: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a `DFSchema` from an Arrow schema and a given qualifier

To create a schema from an Arrow schema without a qualifier, use `DFSchema::try_from`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.from_field_specific_qualified_schema" class="fn">from_field_specific_qualified_schema</a>( qualifiers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>\>, schema: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a `DFSchema` from an Arrow schema where all the fields have a given qualifier

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.with_field_specific_qualified_schema" class="fn">with_field_specific_qualified_schema</a>( &self, qualifiers: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the same schema, where all fields have a given qualifier.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.check_names" class="fn">check_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Check if the schema have some fields with the same name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.with_functional_dependencies" class="fn">with_functional_dependencies</a>( self, functional_dependencies: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.FunctionalDependencies.html" class="struct" title="struct datafusion::common::FunctionalDependencies">FunctionalDependencies</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Assigns functional dependencies.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.join" class="fn">join</a>(&self, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a new schema that contains the fields from this schema followed by the fields from the supplied schema. An error will be returned if there are duplicate field names.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.merge" class="fn">merge</a>(&mut self, other_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>)

Modify this schema by appending the fields from the supplied schema, ignoring any duplicate fields.

###### <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#merge-precedence" class="doc-anchor">§</a>Merge Precedence

**Schema-level metadata**: Metadata from both schemas is merged. If both schemas have the same metadata key, the value from the `other_schema` parameter takes precedence.

**Field-level merging**: Only non-duplicate fields are added. This means that the `self` fields will always take precedence over the `other_schema` fields. Duplicate field detection is based on:

- For qualified fields: both qualifier and field name must match
- For unqualified fields: only field name needs to match

Take note how the precedence for fields & metadata merging differs; merging prefers fields from `self` but prefers metadata from `other_schema`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.fields" class="fn">fields</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Fields.html" class="struct" title="struct datafusion::common::arrow::datatypes::Fields">Fields</a>

Get a list of fields

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.field" class="fn">field</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>

Returns an immutable reference of a specific `Field` instance selected using an offset within the internal `fields` vector

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.qualified_field" class="fn">qualified_field</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> (<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>)

Returns an immutable reference of a specific `Field` instance selected using an offset within the internal `fields` vector and its qualifier

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.index_of_column_by_name" class="fn">index_of_column_by_name</a>( &self, qualifier: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.maybe_index_of_column" class="fn">maybe_index_of_column</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Find the index of the column with the given qualifier and name, returning `None` if not found

See [Self::index_of_column](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.index_of_column "method datafusion::common::DFSchema::index_of_column") for a version that returns an error if the column is not found

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.index_of_column" class="fn">index_of_column</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Find the index of the column with the given qualifier and name, returning `Err` if not found

See [Self::maybe_index_of_column](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.maybe_index_of_column "method datafusion::common::DFSchema::maybe_index_of_column") for a version that returns `None` if the column is not found

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.is_column_from_schema" class="fn">is_column_from_schema</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if the column is in the current schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.field_with_name" class="fn">field_with_name</a>( &self, qualifier: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Find the field with the given name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.qualified_field_with_name" class="fn">qualified_field_with_name</a>( &self, qualifier: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Find the qualified field with the given name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.fields_with_qualified" class="fn">fields_with_qualified</a>(&self, qualifier: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>

Find all fields having the given qualifier

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.fields_indices_with_qualified" class="fn">fields_indices_with_qualified</a>( &self, qualifier: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Find all fields indices having the given qualifier

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.fields_with_unqualified_name" class="fn">fields_with_unqualified_name</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>

Find all fields that match the given name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.qualified_fields_with_unqualified_name" class="fn">qualified_fields_with_unqualified_name</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>)\>

Find all fields that match the given name and return them with their qualifier

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.columns_with_unqualified_name" class="fn">columns_with_unqualified_name</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>

Find all fields that match the given name and convert to column

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.columns" class="fn">columns</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>

Return all `Column`s for the schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.qualified_field_with_unqualified_name" class="fn">qualified_field_with_unqualified_name</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Find the qualified field with the given unqualified name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.field_with_unqualified_name" class="fn">field_with_unqualified_name</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Find the field with the given name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.field_with_qualified_name" class="fn">field_with_qualified_name</a>( &self, qualifier: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Find the field with the given qualified name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.qualified_field_from_column" class="fn">qualified_field_from_column</a>( &self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Find the field with the given qualified column

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.has_column_with_unqualified_name" class="fn">has_column_with_unqualified_name</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Find if the field exists with the given name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.has_column_with_qualified_name" class="fn">has_column_with_qualified_name</a>( &self, qualifier: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Find if the field exists with the given qualified name

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.has_column" class="fn">has_column</a>(&self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Find if the field exists with the given qualified column

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.matches_arrow_schema" class="fn">matches_arrow_schema</a>(&self, arrow_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check to see if unqualified field names matches field names in Arrow schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.check_arrow_schema_type_compatible" class="fn">check_arrow_schema_type_compatible</a>( &self, arrow_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 47.0.0: This method is no longer used

Check to see if fields in 2 Arrow schemas are compatible

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.logically_equivalent_names_and_types" class="fn">logically_equivalent_names_and_types</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the two schemas have the same qualified named fields with logically equivalent data types. Returns false otherwise.

Use [DFSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema")::equivalent_names_and_types for stricter semantic type equivalence checking.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.equivalent_names_and_types" class="fn">equivalent_names_and_types</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

👎Deprecated since 47.0.0: Use has_equivalent_names_and_types\` instead

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.has_equivalent_names_and_types" class="fn">has_equivalent_names_and_types</a>( &self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns Ok if the two schemas have the same qualified named fields with the compatible data types.

Returns an `Err` with a message otherwise.

This is a specialized version of Eq that ignores differences in nullability and metadata.

Use [DFSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema")::logically_equivalent_names_and_types for a weaker logical type checking, which for example would consider a dictionary encoded UTF8 array to be equivalent to a plain UTF8 array.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.datatype_is_logically_equal" class="fn">datatype_is_logically_equal</a>(dt1: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, dt2: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Checks if two [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType")s are logically equal. This is a notably weaker constraint than datatype_is_semantically_equal in that different representations of same data can be logically but not semantically equivalent. Semantically equivalent types are always also logically equivalent. For example:

- a Dictionary\<K,V\> type is logically equal to a plain V type
- a Dictionary\<K1, V1\> is also logically equal to Dictionary\<K2, V1\>
- Utf8 and Utf8View are logically equal

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.datatype_is_semantically_equal" class="fn">datatype_is_semantically_equal</a>(dt1: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, dt2: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true of two [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType")s are semantically equal (same name and type), ignoring both metadata and nullability, and decimal precision/scale.

request to upstream: <https://github.com/apache/arrow-rs/issues/3199>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.strip_qualifiers" class="fn">strip_qualifiers</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

Strip all field qualifier in schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.replace_qualifier" class="fn">replace_qualifier</a>(self, qualifier: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

Replace all field qualifier with new value in schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.field_names" class="fn">field_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get list of fully-qualified field names in this schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.metadata" class="fn">metadata</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get metadata of this schema

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.functional_dependencies" class="fn">functional_dependencies</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.FunctionalDependencies.html" class="struct" title="struct datafusion::common::FunctionalDependencies">FunctionalDependencies</a>

Get functional dependencies

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.iter" class="fn">iter</a>( &self, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html" class="trait" title="trait core::iter::traits::iterator::Iterator">Iterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)\>

Iterate over the qualifiers and fields in the DFSchema

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-AsRef%3CArc%3CSchema%3E%3E-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

Allow DFSchema to be converted into an Arrow `&SchemaRef` (to clone, for example)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.as_ref-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-AsRef%3CSchema%3E-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

Allow DFSchema to be converted into an Arrow `&Schema`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.as_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref" class="fn">as_ref</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>

Converts this type into a shared reference of the (usually inferred) input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-Clone-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-Debug-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-Display-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-ExprSchema-for-DFSchema" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.field_from_column" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#tymethod.field_from_column" class="fn">field_from_column</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#method.nullable" class="fn">nullable</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Is this column reference nullable?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.data_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#method.data_type" class="fn">data_type</a>(&self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

What is the datatype of this column?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.metadata-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#method.metadata" class="fn">metadata</a>( &self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the column’s optional metadata.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.data_type_and_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html#method.data_type_and_nullable" class="fn">data_type_and_nullable</a>( &self, col: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the column’s datatype and nullability

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-From%3C%26DFSchema%3E-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(df_schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>

Convert DFSchema reference into a Schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-From%3CDFSchema%3E-for-Arc%3CSchema%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\> for <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(df_schema: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-From%3CDFSchema%3E-for-Schema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(df_schema: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>

Convert DFSchema into a Schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-Hash-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-PartialEq-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-TryFrom%3CArc%3CSchema%3E%3E-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#associatedtype.Error-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.try_from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-TryFrom%3CSchema%3E-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

Create a `DFSchema` from an Arrow schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( schema: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-Eq-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#impl-StructuralPartialEq-for-DFSchema" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html#blanket-implementations" class="anchor">§</a>
