# Function project_schema Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/utils/mod.rs.html#74-77" class="src">Source</a>

``` rust
pub fn project_schema(
    schema: &Arc<Schema>,
    projection: Option<&Vec<usize>>,
) -> Result<Arc<Schema>, DataFusionError>
```

Expand description

Applies an optional projection to a [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef"), returning the projected schema

Example:

``` rust
use arrow::datatypes::{SchemaRef, Schema, Field, DataType};
use datafusion_common::project_schema;

// Schema with columns 'a', 'b', and 'c'
let schema = SchemaRef::new(Schema::new(vec![
  Field::new("a", DataType::Int32, true),
  Field::new("b", DataType::Int64, true),
  Field::new("c", DataType::Utf8, true),
]));

// Pick columns 'c' and 'b'
let projection = Some(vec![2,1]);
let projected_schema = project_schema(
   &schema,
   projection.as_ref()
 ).unwrap();

let expected_schema = SchemaRef::new(Schema::new(vec![
  Field::new("c", DataType::Utf8, true),
  Field::new("b", DataType::Int64, true),
]));

assert_eq!(projected_schema, expected_schema);
```
