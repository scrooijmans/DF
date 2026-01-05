# Function display_schema Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/logical_plan/display.rs.html#113" class="src">Source</a>

``` rust
pub fn display_schema(schema: &Schema) -> impl Display
```

Expand description

Print the schema in a compact representation to `buf`

For example: `foo:Utf8` if `foo` can not be null, and `foo:Utf8;N` if `foo` is nullable.

``` rust
use arrow::datatypes::{Field, Schema, DataType};
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("first_name", DataType::Utf8, true),
 ]);

 assert_eq!(
     "[id:Int32, first_name:Utf8;N]",
     format!("{}", display_schema(&schema))
 );
```
