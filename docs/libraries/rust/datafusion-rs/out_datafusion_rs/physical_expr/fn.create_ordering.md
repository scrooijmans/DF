# Function create_ordering Copy item path

<a href="https://docs.rs/datafusion-physical-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_expr/physical_expr.rs.html#131-134" class="src">Source</a>

``` rust
pub fn create_ordering(
    schema: &Schema,
    sort_order: &[Vec<Sort>],
) -> Result<Vec<LexOrdering>, DataFusionError>
```

Expand description

Converts logical sort expressions to physical sort expressions.

This function transforms a collection of logical sort expressions into their physical representation that can be used during query execution.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_ordering.html#arguments" class="doc-anchor">§</a>Arguments

- `schema` - The schema containing column definitions.
- `sort_order` - A collection of logical sort expressions grouped into lexicographic orderings.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_ordering.html#returns" class="doc-anchor">§</a>Returns

A vector of lexicographic orderings for physical execution, or an error if the transformation fails.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr/fn.create_ordering.html#examples" class="doc-anchor">§</a>Examples

``` rust
// Create orderings from columns "id" and "name"
// Create a schema with two fields
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("name", DataType::Utf8, false),
]);

let sort_exprs = vec![
    vec![
        SortExpr { expr: Expr::Column(Column::new(Some("t"), "id")), asc: true, nulls_first: false }
    ],
    vec![
        SortExpr { expr: Expr::Column(Column::new(Some("t"), "name")), asc: false, nulls_first: true }
    ]
];
let result = create_ordering(&schema, &sort_exprs).unwrap();
```
