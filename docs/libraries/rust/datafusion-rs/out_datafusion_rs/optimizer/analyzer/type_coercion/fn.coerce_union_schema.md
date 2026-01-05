# Function coerce_union_schema Copy item path

<a href="https://docs.rs/datafusion-optimizer/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_optimizer/analyzer/type_coercion.rs.html#990" class="src">Source</a>

``` rust
pub fn coerce_union_schema(
    inputs: &[Arc<LogicalPlan>],
) -> Result<DFSchema, DataFusionError>
```

Expand description

Get a common schema that is compatible with all inputs of UNION.

This method presumes that the wildcard expansion is unneeded, or has already been applied.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/optimizer/analyzer/type_coercion/fn.coerce_union_schema.html#schema-and-field-handling-in-union-coercion" class="doc-anchor">§</a>Schema and Field Handling in Union Coercion

**Processing order**: The function starts with the base schema (first input) and then processes remaining inputs sequentially, with later inputs taking precedence in merging.

**Schema-level metadata merging**: Later schemas take precedence for duplicate keys.

**Field-level metadata merging**: Later fields take precedence for duplicate metadata keys.

**Type coercion precedence**: The coerced type is determined by iteratively applying `comparison_coercion()` between the accumulated type and each new input’s type. The result depends on type coercion rules, not input order.

**Nullability merging**: Nullability is accumulated using logical OR (`||`). Once any input field is nullable, the result field becomes nullable permanently. Later inputs can make a field nullable but cannot make it non-nullable.

**Field precedence**: Field names come from the first (base) schema, but the field properties (nullability and field-level metadata) have later schemas taking precedence.

**Example**:

``` sql
SELECT a, b FROM table1  -- a: Int32, metadata {"source": "t1"}, nullable=false
UNION
SELECT a, b FROM table2  -- a: Int64, metadata {"source": "t2"}, nullable=true
UNION
SELECT a, b FROM table3  -- a: Int32, metadata {"encoding": "utf8"}, nullable=false
-- Result:
-- a: Int64 (from type coercion), nullable=true (from table2),
-- metadata: {"source": "t2", "encoding": "utf8"} (later inputs take precedence)
```

**Precedence Summary**:

- **Datatypes**: Determined by `comparison_coercion()` rules, not input order
- **Nullability**: Later inputs can add nullability but cannot remove it (logical OR)
- **Metadata**: Later inputs take precedence for same keys (HashMap::extend semantics)
