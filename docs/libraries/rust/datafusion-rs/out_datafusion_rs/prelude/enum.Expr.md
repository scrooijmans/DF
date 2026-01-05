# Enum Expr Copy item path

<a href="https://docs.rs/datafusion-expr/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr/expr.rs.html#280" class="src">Source</a>

``` rust
pub enum Expr {
Show 33 variants    Alias(Alias),
    Column(Column),
    ScalarVariable(DataType, Vec<String>),
    Literal(ScalarValue, Option<FieldMetadata>),
    BinaryExpr(BinaryExpr),
    Like(Like),
    SimilarTo(Like),
    Not(Box<Expr>),
    IsNotNull(Box<Expr>),
    IsNull(Box<Expr>),
    IsTrue(Box<Expr>),
    IsFalse(Box<Expr>),
    IsUnknown(Box<Expr>),
    IsNotTrue(Box<Expr>),
    IsNotFalse(Box<Expr>),
    IsNotUnknown(Box<Expr>),
    Negative(Box<Expr>),
    Between(Between),
    Case(Case),
    Cast(Cast),
    TryCast(TryCast),
    ScalarFunction(ScalarFunction),
    AggregateFunction(AggregateFunction),
    WindowFunction(Box<WindowFunction>),
    InList(InList),
    Exists(Exists),
    InSubquery(InSubquery),
    ScalarSubquery(Subquery),
    Wildcard {
        qualifier: Option<TableReference>,
        options: Box<WildcardOptions>,
    },
    GroupingSet(GroupingSet),
    Placeholder(Placeholder),
    OuterReferenceColumn(DataType, Column),
    Unnest(Unnest),
}
```

Expand description

Represents logical expressions such as `A + 1`, or `CAST(c1 AS int)`.

For example the expression `A + 1` will be represented as

``` text
  BinaryExpr {
    left: Expr::Column("A"),
    op: Operator::Plus,
    right: Expr::Literal(ScalarValue::Int32(Some(1)), None)
 }
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#creating-expressions" class="doc-anchor">§</a>Creating Expressions

`Expr`s can be created directly, but it is often easier and less verbose to use the fluent APIs in [`crate::expr_fn`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr_fn/index.html "mod datafusion::logical_expr::expr_fn") such as [`col`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.col.html "fn datafusion::prelude::col") and [`lit`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.lit.html "fn datafusion::prelude::lit"), or methods such as [`Expr::alias`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.alias "method datafusion::prelude::Expr::alias"), [`Expr::cast_to`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.cast_to "method datafusion::prelude::Expr::cast_to"), and [`Expr::Like`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Like "variant datafusion::prelude::Expr::Like")).

See also [`ExprFunctionExt`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html "trait datafusion::prelude::ExprFunctionExt") for creating aggregate and window functions.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#printing-expressions" class="doc-anchor">§</a>Printing Expressions

You can print `Expr`s using the the `Debug` trait, `Display` trait, or [`Self::human_display`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.human_display "method datafusion::prelude::Expr::human_display"). See the [examples](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#examples-displaying-exprs) below.

If you need SQL to pass to other systems, consider using [`Unparser`](https://docs.rs/datafusion/latest/datafusion/sql/unparser/struct.Unparser.html).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#schema-access" class="doc-anchor">§</a>Schema Access

See [`ExprSchemable::get_type`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.get_type "method datafusion::logical_expr::ExprSchemable::get_type") to access the [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") and nullability of an `Expr`.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#visiting-and-rewriting-exprs" class="doc-anchor">§</a>Visiting and Rewriting `Expr`s

The `Expr` struct implements the [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") trait for walking and rewriting expressions. For example [`TreeNode::apply`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply "method datafusion::common::tree_node::TreeNode::apply") recursively visits an `Expr` and [`TreeNode::transform`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform "method datafusion::common::tree_node::TreeNode::transform") can be used to rewrite an expression. See the examples below and [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") for more information.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#examples-creating-and-using-exprs" class="doc-anchor">§</a>Examples: Creating and Using `Expr`s

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#column-references-and-literals" class="doc-anchor">§</a>Column References and Literals

[`Expr::Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Column "variant datafusion::prelude::Expr::Column") refer to the values of columns and are often created with the [`col`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.col.html "fn datafusion::prelude::col") function. For example to create an expression `c1` referring to column named “c1”:

``` rust
let expr = col("c1");
assert_eq!(expr, Expr::Column(Column::from_name("c1")));
```

[`Expr::Literal`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Literal "variant datafusion::prelude::Expr::Literal") refer to literal, or constant, values. These are created with the [`lit`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.lit.html "fn datafusion::prelude::lit") function. For example to create an expression `42`:

``` rust
// All literals are strongly typed in DataFusion. To make an `i64` 42:
let expr = lit(42i64);
assert_eq!(expr, Expr::Literal(ScalarValue::Int64(Some(42)), None));
assert_eq!(expr, Expr::Literal(ScalarValue::Int64(Some(42)), None));
// To make a (typed) NULL:
let expr = Expr::Literal(ScalarValue::Int64(None), None);
// to make an (untyped) NULL (the optimizer will coerce this to the correct type):
let expr = lit(ScalarValue::Null);
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#binary-expressions" class="doc-anchor">§</a>Binary Expressions

Exprs implement traits that allow easy to understand construction of more complex expressions. For example, to create `c1 + c2` to add columns “c1” and “c2” together

``` rust
// Use the `+` operator to add two columns together
let expr = col("c1") + col("c2");
assert!(matches!(expr, Expr::BinaryExpr { ..} ));
if let Expr::BinaryExpr(binary_expr) = expr {
  assert_eq!(*binary_expr.left, col("c1"));
  assert_eq!(*binary_expr.right, col("c2"));
  assert_eq!(binary_expr.op, Operator::Plus);
}
```

The expression `c1 = 42` to compares the value in column “c1” to the literal value `42`:

``` rust
let expr = col("c1").eq(lit(42_i32));
assert!(matches!(expr, Expr::BinaryExpr { .. } ));
if let Expr::BinaryExpr(binary_expr) = expr {
  assert_eq!(*binary_expr.left, col("c1"));
  let scalar = ScalarValue::Int32(Some(42));
  assert_eq!(*binary_expr.right, Expr::Literal(scalar, None));
  assert_eq!(binary_expr.op, Operator::Eq);
}
```

Here is how to implement the equivalent of `SELECT *` to select all [`Expr::Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Column "variant datafusion::prelude::Expr::Column") from a [`DFSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema")’s columns:

``` rust
// Create a schema c1(int, c2 float)
let arrow_schema = Schema::new(vec![
   Field::new("c1", DataType::Int32, false),
   Field::new("c2", DataType::Float64, false),
]);
// DFSchema is a an Arrow schema with optional relation name
let df_schema = DFSchema::try_from_qualified_schema("t1", &arrow_schema)
  .unwrap();

// Form Vec<Expr> with an expression for each column in the schema
let exprs: Vec<_> = df_schema.iter()
  .map(Expr::from)
  .collect();

assert_eq!(exprs, vec![
  Expr::from(Column::from_qualified_name("t1.c1")),
  Expr::from(Column::from_qualified_name("t1.c2")),
]);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#examples-displaying-exprs" class="doc-anchor">§</a>Examples: Displaying `Exprs`

There are three ways to print an `Expr` depending on the usecase.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#use-debug-trait" class="doc-anchor">§</a>Use `Debug` trait

Following Rust conventions, the `Debug` implementation prints out the internal structure of the expression, which is useful for debugging.

``` rust
let expr = col("c1") + lit(42);
assert_eq!(format!("{expr:?}"), "BinaryExpr(BinaryExpr { left: Column(Column { relation: None, name: \"c1\" }), op: Plus, right: Literal(Int32(42), None) })");
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#use-the-display-trait--detailed-expression" class="doc-anchor">§</a>Use the `Display` trait (detailed expression)

The `Display` implementation prints out the expression in a SQL-like form, but has additional details such as the data type of literals. This is useful for understanding the expression in more detail and is used for the low level [`ExplainFormat::Indent`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#variant.Indent "variant datafusion::logical_expr::ExplainFormat::Indent") explain plan format.

``` rust
let expr = col("c1") + lit(42);
assert_eq!(format!("{expr}"), "c1 + Int32(42)");
```

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#use-selfhuman_display-human-readable" class="doc-anchor">§</a>Use [`Self::human_display`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.human_display "method datafusion::prelude::Expr::human_display") (human readable)

[`Self::human_display`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.human_display "method datafusion::prelude::Expr::human_display") prints out the expression in a SQL-like form, optimized for human consumption by end users. It is used for the [`ExplainFormat::Tree`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ExplainFormat.html#variant.Tree "variant datafusion::logical_expr::ExplainFormat::Tree") explain plan format.

``` rust
 let expr = col("c1") + lit(42);
 assert_eq!(format!("{}", expr.human_display()), "c1 + 42");
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#examples-visiting-and-rewriting-exprs" class="doc-anchor">§</a>Examples: Visiting and Rewriting `Expr`s

Here is an example that finds all literals in an `Expr` tree:

``` rust
use datafusion_common::ScalarValue;
use datafusion_common::tree_node::{TreeNode, TreeNodeRecursion};
// Expression a = 5 AND b = 6
let expr = col("a").eq(lit(5)) & col("b").eq(lit(6));
// find all literals in a HashMap
let mut scalars = HashSet::new();
// apply recursively visits all nodes in the expression tree
expr.apply(|e| {
   if let Expr::Literal(scalar, _) = e {
      scalars.insert(scalar);
   }
   // The return value controls whether to continue visiting the tree
   Ok(TreeNodeRecursion::Continue)
}).unwrap();
// All subtrees have been visited and literals found
assert_eq!(scalars.len(), 2);
assert!(scalars.contains(&ScalarValue::Int32(Some(5))));
assert!(scalars.contains(&ScalarValue::Int32(Some(6))));
```

Rewrite an expression, replacing references to column “a” in an to the literal `42`:

``` rust
// expression a = 5 AND b = 6
let expr = col("a").eq(lit(5)).and(col("b").eq(lit(6)));
// rewrite all references to column "a" to the literal 42
let rewritten = expr.transform(|e| {
 if let Expr::Column(c) = &e {
   if &c.name == "a" {
     // return Transformed::yes to indicate the node was changed
     return Ok(Transformed::yes(lit(42)))
   }
 }
 // return Transformed::no to indicate the node was not changed
 Ok(Transformed::no(e))
}).unwrap();
// The expression has been rewritten
assert!(rewritten.transformed);
// to 42 = 5 AND b = 6
assert_eq!(rewritten.data, lit(42).eq(lit(5)).and(col("b").eq(lit(6))));
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Alias" class="anchor">§</a>

### Alias(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Alias.html" class="struct" title="struct datafusion::logical_expr::expr::Alias">Alias</a>)

An expression with a specific name.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Column" class="anchor">§</a>

### Column(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>)

A named reference to a qualified field in a schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.ScalarVariable" class="anchor">§</a>

### ScalarVariable(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>)

A named reference to a variable in a registry.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Literal" class="anchor">§</a>

### Literal(<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>\>)

A constant value along with associated [`FieldMetadata`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html "struct datafusion::logical_expr::expr::FieldMetadata").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.BinaryExpr" class="anchor">§</a>

### BinaryExpr(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.BinaryExpr.html" class="struct" title="struct datafusion::logical_expr::BinaryExpr">BinaryExpr</a>)

A binary expression such as “age \> 21”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Like" class="anchor">§</a>

### Like(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Like.html" class="struct" title="struct datafusion::logical_expr::Like">Like</a>)

LIKE expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.SimilarTo" class="anchor">§</a>

### SimilarTo(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Like.html" class="struct" title="struct datafusion::logical_expr::Like">Like</a>)

LIKE expression that uses regular expressions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Not" class="anchor">§</a>

### Not(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

Negation of an expression. The expression’s type must be a boolean to make sense.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.IsNotNull" class="anchor">§</a>

### IsNotNull(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

True if argument is not NULL, false otherwise. This expression itself is never NULL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.IsNull" class="anchor">§</a>

### IsNull(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

True if argument is NULL, false otherwise. This expression itself is never NULL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.IsTrue" class="anchor">§</a>

### IsTrue(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

True if argument is true, false otherwise. This expression itself is never NULL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.IsFalse" class="anchor">§</a>

### IsFalse(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

True if argument is false, false otherwise. This expression itself is never NULL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.IsUnknown" class="anchor">§</a>

### IsUnknown(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

True if argument is NULL, false otherwise. This expression itself is never NULL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.IsNotTrue" class="anchor">§</a>

### IsNotTrue(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

True if argument is FALSE or NULL, false otherwise. This expression itself is never NULL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.IsNotFalse" class="anchor">§</a>

### IsNotFalse(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

True if argument is TRUE OR NULL, false otherwise. This expression itself is never NULL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.IsNotUnknown" class="anchor">§</a>

### IsNotUnknown(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

True if argument is TRUE or FALSE, false otherwise. This expression itself is never NULL.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Negative" class="anchor">§</a>

### Negative(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>)

arithmetic negation of an expression, the operand must be of a signed numeric data type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Between" class="anchor">§</a>

### Between(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Between.html" class="struct" title="struct datafusion::logical_expr::Between">Between</a>)

Whether an expression is between a given range.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Case" class="anchor">§</a>

### Case(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Case.html" class="struct" title="struct datafusion::logical_expr::Case">Case</a>)

A CASE expression (see docs on [`Case`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Case.html "struct datafusion::logical_expr::Case"))

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Cast" class="anchor">§</a>

### Cast(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Cast.html" class="struct" title="struct datafusion::logical_expr::Cast">Cast</a>)

Casts the expression to a given type and will return a runtime error if the expression cannot be cast. This expression is guaranteed to have a fixed type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.TryCast" class="anchor">§</a>

### TryCast(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.TryCast.html" class="struct" title="struct datafusion::logical_expr::TryCast">TryCast</a>)

Casts the expression to a given type and will return a null value if the expression cannot be cast. This expression is guaranteed to have a fixed type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.ScalarFunction" class="anchor">§</a>

### ScalarFunction(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.ScalarFunction.html" class="struct" title="struct datafusion::logical_expr::expr::ScalarFunction">ScalarFunction</a>)

Call a scalar function with a set of arguments.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.AggregateFunction" class="anchor">§</a>

### AggregateFunction(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.AggregateFunction.html" class="struct" title="struct datafusion::logical_expr::expr::AggregateFunction">AggregateFunction</a>)

Calls an aggregate function with arguments, and optional `ORDER BY`, `FILTER`, `DISTINCT` and `NULL TREATMENT`.

See also [`ExprFunctionExt`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html "trait datafusion::prelude::ExprFunctionExt") to set these fields.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction" class="anchor">§</a>

### WindowFunction(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunction.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunction">WindowFunction</a>\>)

Call a window function with a set of arguments.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.InList" class="anchor">§</a>

### InList(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.InList.html" class="struct" title="struct datafusion::logical_expr::expr::InList">InList</a>)

Returns whether the list contains the expr value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Exists" class="anchor">§</a>

### Exists(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Exists.html" class="struct" title="struct datafusion::logical_expr::expr::Exists">Exists</a>)

EXISTS subquery

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.InSubquery" class="anchor">§</a>

### InSubquery(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.InSubquery.html" class="struct" title="struct datafusion::logical_expr::expr::InSubquery">InSubquery</a>)

IN subquery

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.ScalarSubquery" class="anchor">§</a>

### ScalarSubquery(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Subquery.html" class="struct" title="struct datafusion::logical_expr::Subquery">Subquery</a>)

Scalar subquery

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard" class="anchor">§</a>

### Wildcard

👎Deprecated since 46.0.0: A wildcard needs to be resolved to concrete expressions when constructing the logical plan. See https://github.com/apache/datafusion/issues/7765

Represents a reference to all available fields in a specific schema, with an optional (schema) qualifier.

This expr has to be resolved to a list of columns before translating logical plan into physical plan.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard.field.qualifier" class="anchor field">§</a>`qualifier: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference"><code>TableReference</code></a>`>`

👎Deprecated since 46.0.0: A wildcard needs to be resolved to concrete expressions when constructing the logical plan. See https://github.com/apache/datafusion/issues/7765

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard.field.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WildcardOptions.html" class="struct" title="struct datafusion::logical_expr::expr::WildcardOptions"><code>WildcardOptions</code></a>`>`

👎Deprecated since 46.0.0: A wildcard needs to be resolved to concrete expressions when constructing the logical plan. See https://github.com/apache/datafusion/issues/7765

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.GroupingSet" class="anchor">§</a>

### GroupingSet(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.GroupingSet.html" class="enum" title="enum datafusion::logical_expr::GroupingSet">GroupingSet</a>)

List of grouping set expressions. Only valid in the context of an aggregate GROUP BY expression list

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Placeholder" class="anchor">§</a>

### Placeholder(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Placeholder.html" class="struct" title="struct datafusion::logical_expr::expr::Placeholder">Placeholder</a>)

A place holder for parameters in a prepared statement (e.g. `$foo` or `$1`)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.OuterReferenceColumn" class="anchor">§</a>

### OuterReferenceColumn(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>)

A placeholder which holds a reference to a qualified field in the outer query, used for correlated sub queries.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Unnest" class="anchor">§</a>

### Unnest(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.Unnest.html" class="struct" title="struct datafusion::logical_expr::expr::Unnest">Unnest</a>)

Unnest expression

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.schema_name" class="fn">schema_name</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>

The name of the column (field) that this `Expr` will produce.

For example, for a projection (e.g. `SELECT <expr>`) the resulting arrow [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema") will have a field with this name.

Note that the resulting string is subtlety different from the `Display` representation for certain `Expr`. Some differences:

1.  [`Expr::Alias`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Alias "variant datafusion::prelude::Expr::Alias"), which shows only the alias itself
2.  [`Expr::Cast`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Cast "variant datafusion::prelude::Expr::Cast") / [`Expr::TryCast`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.TryCast "variant datafusion::prelude::Expr::TryCast"), which only displays the expression

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#example" class="doc-anchor">§</a>Example

``` rust
let expr = col("foo").eq(lit(42));
assert_eq!("foo = Int32(42)", expr.schema_name().to_string());

let expr = col("foo").alias("bar").eq(lit(11));
assert_eq!("bar = Int32(11)", expr.schema_name().to_string());
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.human_display" class="fn">human_display</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>

Human readable display formatting for this expression.

This function is primarily used in printing the explain tree output, (e.g. `EXPLAIN FORMAT TREE <query>`), providing a readable format to show how expressions are used in physical and logical plans. See the [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") for other ways to format expressions

Note this format is intended for human consumption rather than SQL for other systems. If you need SQL to pass to other systems, consider using [`Unparser`](https://docs.rs/datafusion/latest/datafusion/sql/unparser/struct.Unparser.html).

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#example-1" class="doc-anchor">§</a>Example

``` rust
let expr = col("foo") + lit(42);
// For EXPLAIN output:
// "foo + 42"
println!("{}", expr.human_display());
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.qualified_name" class="fn">qualified_name</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Returns the qualifier and the schema name of this expression.

Used when the expression forms the output field of a certain plan. The result is the field’s qualifier and field name in the plan’s output schema. We can use this qualified name to reference the field.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.variant_name" class="fn">variant_name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Return String representation of the variant represented by `self` Useful for non-rust based bindings

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.eq" class="fn">eq</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self == other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.not_eq" class="fn">not_eq</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self != other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.gt" class="fn">gt</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self > other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.gt_eq" class="fn">gt_eq</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self >= other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.lt" class="fn">lt</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self < other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.lt_eq" class="fn">lt_eq</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self <= other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.and" class="fn">and</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self && other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.or" class="fn">or</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self || other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.like" class="fn">like</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self LIKE other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.not_like" class="fn">not_like</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self NOT LIKE other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.ilike" class="fn">ilike</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self ILIKE other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.not_ilike" class="fn">not_ilike</a>(self, other: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self NOT ILIKE other`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.name_for_alias" class="fn">name_for_alias</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return the name to use for the specific Expr

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.alias_if_changed" class="fn">alias_if_changed</a>( self, original_name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Ensure `expr` has the name as `original_name` by adding an alias if necessary.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.alias" class="fn">alias</a>(self, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self AS name` alias expression

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.alias_with_metadata" class="fn">alias_with_metadata</a>( self, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self AS name` alias expression with metadata

The metadata will be attached to the Arrow Schema field when the expression is converted to a field via `Expr.to_field()`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#example-2" class="doc-anchor">§</a>Example

``` rust
let metadata = HashMap::from([("key".to_string(), "value".to_string())]);
let metadata = FieldMetadata::from(metadata);
let expr = col("foo").alias_with_metadata("bar", Some(metadata));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.alias_qualified" class="fn">alias_qualified</a>( self, relation: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>\>, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self AS name` alias expression with a specific qualifier

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.alias_qualified_with_metadata" class="fn">alias_qualified_with_metadata</a>( self, relation: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>\>, name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>, metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self AS name` alias expression with a specific qualifier and metadata

The metadata will be attached to the Arrow Schema field when the expression is converted to a field via `Expr.to_field()`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#example-3" class="doc-anchor">§</a>Example

``` rust
let metadata = HashMap::from([("key".to_string(), "value".to_string())]);
let metadata = FieldMetadata::from(metadata);
let expr = col("foo").alias_qualified_with_metadata(Some("tbl"), "bar", Some(metadata));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.unalias" class="fn">unalias</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Remove an alias from an expression if one exists.

If the expression is not an alias, the expression is returned unchanged. This method does not remove aliases from nested expressions.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#example-4" class="doc-anchor">§</a>Example

``` rust
// `foo as "bar"` is unaliased to `foo`
let expr = col("foo").alias("bar");
assert_eq!(expr.unalias(), col("foo"));

// `foo as "bar" + baz` is not unaliased
let expr = col("foo").alias("bar") + col("baz");
assert_eq!(expr.clone().unalias(), expr);

// `foo as "bar" as "baz" is unaliased to foo as "bar"
let expr = col("foo").alias("bar").alias("baz");
assert_eq!(expr.unalias(), col("foo").alias("bar"));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.unalias_nested" class="fn">unalias_nested</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>

Recursively removed potentially multiple aliases from an expression.

This method removes nested aliases and returns [`Transformed`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html "struct datafusion::common::tree_node::Transformed") to signal if the expression was changed.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#example-5" class="doc-anchor">§</a>Example

``` rust
// `foo as "bar"` is unaliased to `foo`
let expr = col("foo").alias("bar");
assert_eq!(expr.unalias_nested().data, col("foo"));

// `foo as "bar" + baz` is  unaliased
let expr = col("foo").alias("bar") + col("baz");
assert_eq!(expr.clone().unalias_nested().data, col("foo") + col("baz"));

// `foo as "bar" as "baz" is unalaised to foo
let expr = col("foo").alias("bar").alias("baz");
assert_eq!(expr.unalias_nested().data, col("foo"));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.in_list" class="fn">in_list</a>(self, list: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, negated: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self IN <list>` if `negated` is false, otherwise return `self NOT IN <list>`.a

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_null" class="fn">is_null</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return \`IsNull(Box(self))

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_not_null" class="fn">is_not_null</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return \`IsNotNull(Box(self))

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.sort" class="fn">sort</a>(self, asc: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, nulls_first: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>

Create a sort configuration from an existing expression.

``` rust
let sort_expr = col("foo").sort(true, true); // SORT ASC NULLS_FIRST
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_true" class="fn">is_true</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `IsTrue(Box(self))`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_not_true" class="fn">is_not_true</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `IsNotTrue(Box(self))`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_false" class="fn">is_false</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `IsFalse(Box(self))`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_not_false" class="fn">is_not_false</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `IsNotFalse(Box(self))`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_unknown" class="fn">is_unknown</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `IsUnknown(Box(self))`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_not_unknown" class="fn">is_not_unknown</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `IsNotUnknown(Box(self))`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.between" class="fn">between</a>(self, low: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, high: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

return `self BETWEEN low AND high`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.not_between" class="fn">not_between</a>(self, low: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, high: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Return `self NOT BETWEEN low AND high`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.try_as_col" class="fn">try_as_col</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>

Return a reference to the inner `Column` if any

returns `None` if the expression is not a `Column`

Note: None may be returned for expressions that are not `Column` but are convertible to `Column` such as `Cast` expressions.

Example

``` rust
use datafusion_expr::{col, Expr};
let expr = col("foo");
assert_eq!(expr.try_as_col(), Some(&Column::from("foo")));

let expr = col("foo").alias("bar");
assert_eq!(expr.try_as_col(), None);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.get_as_join_column" class="fn">get_as_join_column</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>

Returns the inner `Column` if any. This is a specialized version of [`Self::try_as_col`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.try_as_col "method datafusion::prelude::Expr::try_as_col") that take Cast expressions into account when the expression is as on condition for joins.

Called this method when you are sure that the expression is a `Column` or a `Cast` expression that wraps a `Column`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.column_refs" class="fn">column_refs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>

Return all references to columns in this expression.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#example-6" class="doc-anchor">§</a>Example

``` rust
// For an expression `a + (b * a)`
let expr = col("a") + (col("b") * col("a"));
let refs = expr.column_refs();
// refs contains "a" and "b"
assert_eq!(refs.len(), 2);
assert!(refs.contains(&Column::new_unqualified("a")));
assert!(refs.contains(&Column::new_unqualified("b")));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.add_column_refs" class="fn">add_column_refs</a>\<'a\>(&'a self, set: &mut <a href="https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html" class="struct" title="struct std::collections::hash::set::HashSet">HashSet</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\>)

Adds references to all columns in this expression to the set

See [`Self::column_refs`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.column_refs "method datafusion::prelude::Expr::column_refs") for details

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.column_refs_counts" class="fn">column_refs_counts</a>(&self) -\> <a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Return all references to columns and their occurrence counts in the expression.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#example-7" class="doc-anchor">§</a>Example

``` rust
// For an expression `a + (b * a)`
let expr = col("a") + (col("b") * col("a"));
let mut refs = expr.column_refs_counts();
// refs contains "a" and "b"
assert_eq!(refs.len(), 2);
assert_eq!(*refs.get(&Column::new_unqualified("a")).unwrap(), 2);
assert_eq!(*refs.get(&Column::new_unqualified("b")).unwrap(), 1);
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.add_column_ref_counts" class="fn">add_column_ref_counts</a>\<'a\>(&'a self, map: &mut <a href="https://docs.rs/hashbrown/0.14.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Adds references to all columns and their occurrence counts in the expression to the map.

See [`Self::column_refs_counts`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.column_refs_counts "method datafusion::prelude::Expr::column_refs_counts") for details

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.any_column_refs" class="fn">any_column_refs</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if there are any column references in this Expr

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.contains_outer" class="fn">contains_outer</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if the expression contains out reference(correlated) expressions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_volatile_node" class="fn">is_volatile_node</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the expression node is volatile, i.e. whether it can return different results when evaluated multiple times with the same input. Note: unlike [`Self::is_volatile`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_volatile "method datafusion::prelude::Expr::is_volatile"), this function does not consider inputs:

- `rand()` returns `true`,
- `a + rand()` returns `false`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.is_volatile" class="fn">is_volatile</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the expression is volatile, i.e. whether it can return different results when evaluated multiple times with the same input.

For example the function call `RANDOM()` is volatile as each call will return a different value.

See [`Volatility`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html "enum datafusion::logical_expr::Volatility") for more information.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.infer_placeholder_types" class="fn">infer_placeholder_types</a>( self, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html" class="struct" title="struct datafusion::common::DFSchema">DFSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Recursively find all [`Expr::Placeholder`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Placeholder "variant datafusion::prelude::Expr::Placeholder") expressions, and to infer their [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") from the context of their use.

For example, given an expression like `<int32> = $0` will infer `$0` to have type `int32`.

Returns transformed expression and flag that is true if expression contains at least one placeholder.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.short_circuits" class="fn">short_circuits</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if some of this `exprs` subexpressions may not be evaluated and thus any side effects (like divide by zero) may not be encountered

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.spans" class="fn">spans</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Spans.html" class="struct" title="struct datafusion::common::Spans">Spans</a>\>

Returns a reference to the set of locations in the SQL query where this expression appears, if known. [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") is returned if the expression type doesn’t support tracking locations yet.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.as_literal" class="fn">as_literal</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/scalar/enum.ScalarValue.html" class="enum" title="enum datafusion::scalar::ScalarValue">ScalarValue</a>\>

Check if the Expr is literal and get the literal value if it is.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Add-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> + <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `+` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.add" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add" class="fn">add</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Performs the `+` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html#tymethod.add)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-BitAnd-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitAnd.html" class="trait" title="trait datafusion::prelude::BitAnd">BitAnd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> & <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-5" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitAnd.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `&` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.bitand" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitAnd.html#tymethod.bitand" class="fn">bitand</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Performs the `&` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitAnd.html#tymethod.bitand)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-BitOr-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> | <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-6" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `|` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.bitor" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#tymethod.bitor" class="fn">bitor</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Performs the `|` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html#tymethod.bitor)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-BitXor-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitXor.html" class="trait" title="trait datafusion::prelude::BitXor">BitXor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> ^ <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-7" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitXor.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `^` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.bitxor" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitXor.html#tymethod.bitxor" class="fn">bitxor</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Performs the `^` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitXor.html#tymethod.bitxor)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Clone-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Debug-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Default-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Display-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Format expressions for display as part of a logical plan. In many cases, this will produce similar output to `Expr.name()` except that column names will be prefixed with ‘#’.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Div-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> / <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-3" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `/` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.div" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div" class="fn">div</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Performs the `/` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html#tymethod.div)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-ExprFunctionExt-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html" class="trait" title="trait datafusion::prelude::ExprFunctionExt">ExprFunctionExt</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.order_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.order_by" class="fn">order_by</a>(self, order_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `ORDER BY <order_by>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.filter" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.filter" class="fn">filter</a>(self, filter: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `FILTER <filter>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.distinct" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.distinct" class="fn">distinct</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `DISTINCT`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.null_treatment" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.null_treatment" class="fn">null_treatment</a>( self, null_treatment: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.NullTreatment.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::NullTreatment">NullTreatment</a>\>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `RESPECT NULLS` or `IGNORE NULLS`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.partition_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.partition_by" class="fn">partition_by</a>(self, partition_by: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add `PARTITION BY`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.window_frame" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html#tymethod.window_frame" class="fn">window_frame</a>(self, window_frame: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.WindowFrame.html" class="struct" title="struct datafusion::logical_expr::WindowFrame">WindowFrame</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>

Add appropriate window frame conditions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-ExprSchemable-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html" class="trait" title="trait datafusion::logical_expr::ExprSchemable">ExprSchemable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.get_type" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.get_type" class="fn">get_type</a>(&self, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the [arrow::datatypes::DataType](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") of the expression based on [ExprSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html "trait datafusion::common::ExprSchema")

Note: [`DFSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema") implements [ExprSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html "trait datafusion::common::ExprSchema").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#examples" class="doc-anchor">§</a>Examples

Get the type of an expression that adds 2 columns. Adding an Int32 and Float32 results in Float32 type

``` rust

fn main() {
  let expr = col("c1") + col("c2");
  let schema = DFSchema::from_unqualified_fields(
    vec![
      Field::new("c1", DataType::Int32, true),
      Field::new("c2", DataType::Float32, true),
      ].into(),
      HashMap::new(),
  ).unwrap();
  assert_eq!("Float32", format!("{}", expr.get_type(&schema).unwrap()));
}
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#errors" class="doc-anchor">§</a>Errors

This function errors when it is not possible to compute its [arrow::datatypes::DataType](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType"). This happens when e.g. the expression refers to a column that does not exist in the schema, or when the expression is incorrectly typed (e.g. `[utf8] + [bool]`).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.nullable" class="fn">nullable</a>( &self, input_schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the nullability of the expression based on [ExprSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html "trait datafusion::common::ExprSchema").

Note: [`DFSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema") implements [ExprSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html "trait datafusion::common::ExprSchema").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#errors-1" class="doc-anchor">§</a>Errors

This function errors when it is not possible to compute its nullability. This happens when the expression refers to a column that does not exist in the schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.data_type_and_nullable" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.data_type_and_nullable" class="fn">data_type_and_nullable</a>( &self, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns the datatype and nullability of the expression based on [ExprSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html "trait datafusion::common::ExprSchema").

Note: [`DFSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema") implements [ExprSchema](https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html "trait datafusion::common::ExprSchema").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#errors-2" class="doc-anchor">§</a>Errors

This function errors when it is not possible to compute its datatype or nullability.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.to_field" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.to_field" class="fn">to_field</a>( &self, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Returns a [arrow::datatypes::Field](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html "struct datafusion::common::arrow::datatypes::Field") compatible with this expression.

So for example, a projected expression `col(c1) + col(c2)` is placed in an output field **named** col(“c1 + c2”)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.cast_to" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.cast_to" class="fn">cast_to</a>( self, cast_to_type: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Wraps this expression in a cast to a target [arrow::datatypes::DataType](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#errors-3" class="doc-anchor">§</a>Errors

This function errors when it is impossible to cast the expression to the target [arrow::datatypes::DataType](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.metadata" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ExprSchemable.html#tymethod.metadata" class="fn">metadata</a>( &self, schema: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/trait.ExprSchema.html" class="trait" title="trait datafusion::common::ExprSchema">ExprSchema</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.FieldMetadata.html" class="struct" title="struct datafusion::logical_expr::expr::FieldMetadata">FieldMetadata</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Given a schema, return the expr’s optional metadata

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-FieldAccessor-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html" class="trait" title="trait datafusion::functions::core::expr_ext::FieldAccessor">FieldAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.field" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/core/expr_ext/trait.FieldAccessor.html#tymethod.field" class="fn">field</a>(self, name: impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.Literal.html" class="trait" title="trait datafusion::logical_expr::Literal">Literal</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-From%3C(Option%3C%26TableReference%3E,+%26Arc%3CField%3E)%3E-for-Expr" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &'a <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Create an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") from an optional qualifier and a [`FieldRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.FieldRef.html "type datafusion::common::arrow::datatypes::FieldRef"). This is useful for creating [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") from a [`DFSchema`](https://docs.rs/datafusion/50.2.0/datafusion/common/struct.DFSchema.html "struct datafusion::common::DFSchema").

See example on [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.from-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: (<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\>, &'a <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Field.html" class="struct" title="struct datafusion::common::arrow::datatypes::Field">Field</a>\>)) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-From%3CColumn%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Create an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") from a [`Column`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html "struct datafusion::prelude::Column")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-From%3CExpr%3E-for-SelectExpr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.from-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(expr: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/select_expr/enum.SelectExpr.html" class="enum" title="enum datafusion::logical_expr::select_expr::SelectExpr">SelectExpr</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-From%3CWindowFunction%3E-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunction.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunction">WindowFunction</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Create an [`Expr`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html "enum datafusion::prelude::Expr") from a [`WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunction.html "struct datafusion::logical_expr::expr::WindowFunction")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/expr/struct.WindowFunction.html" class="struct" title="struct datafusion::logical_expr::expr::WindowFunction">WindowFunction</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Hash-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-HashNode-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html" class="trait" title="trait datafusion::common::cse::HashNode">HashNode</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.hash_node" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.HashNode.html#tymethod.hash_node" class="fn">hash_node</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

As it is pretty easy to forget changing this method when `Expr` changes the implementation doesn’t use wildcard patterns (`..`, `_`) to catch changes compile time.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-IndexAccessor-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.IndexAccessor.html" class="trait" title="trait datafusion::functions_nested::expr_ext::IndexAccessor">IndexAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.index" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.IndexAccessor.html#tymethod.index" class="fn">index</a>(self, key: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Mul-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> * <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-2" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `*` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.mul" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul" class="fn">mul</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Performs the `*` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html#tymethod.mul)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Neg-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `- <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-10" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.neg" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#tymethod.neg" class="fn">neg</a>(self) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Neg::Output">Output</a>

Performs the unary `-` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html#tymethod.neg)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-NormalizeEq-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html" class="trait" title="trait datafusion::common::cse::NormalizeEq">NormalizeEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.normalize_eq" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.NormalizeEq.html#tymethod.normalize_eq" class="fn">normalize_eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Normalizeable-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html" class="trait" title="trait datafusion::common::cse::Normalizeable">Normalizeable</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.can_normalize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/cse/trait.Normalizeable.html#tymethod.can_normalize" class="fn">can_normalize</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Not-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Not.html" class="trait" title="trait datafusion::prelude::Not">Not</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `NOT <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-11" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Not.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `!` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.not" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Not.html#tymethod.not" class="fn">not</a>(self) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Not.html" class="trait" title="trait datafusion::prelude::Not">Not</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Not.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Not::Output">Output</a>

Performs the unary `!` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Not.html#tymethod.not)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-PartialEq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.eq-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-PartialOrd-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.lt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.gt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Rem-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> % <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-4" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `%` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.rem" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem" class="fn">rem</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Performs the `%` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html#tymethod.rem)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Shl-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shl.html" class="trait" title="trait datafusion::prelude::Shl">Shl</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> << <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-8" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shl.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `<<` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.shl" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shl.html#tymethod.shl" class="fn">shl</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shl.html" class="trait" title="trait datafusion::prelude::Shl">Shl</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shl.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Shl::Output">Output</a>

Performs the `<<` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shl.html#tymethod.shl)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Shr-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shr.html" class="trait" title="trait datafusion::prelude::Shr">Shr</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> >> <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-9" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shr.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `>>` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.shr" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shr.html#tymethod.shr" class="fn">shr</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shr.html" class="trait" title="trait datafusion::prelude::Shr">Shr</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shr.html#associatedtype.Output" class="associatedtype" title="type datafusion::prelude::Shr::Output">Output</a>

Performs the `>>` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shr.html#tymethod.shr)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-SliceAccessor-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.SliceAccessor.html" class="trait" title="trait datafusion::functions_nested::expr_ext::SliceAccessor">SliceAccessor</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.range" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions_nested/expr_ext/trait.SliceAccessor.html#tymethod.range" class="fn">range</a>(self, start: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>, stop: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Sub-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Support `<expr> - <expr>` fluent style

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#associatedtype.Output-1" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#associatedtype.Output" class="associatedtype">Output</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

The resulting type after applying the `-` operator.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.sub" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub" class="fn">sub</a>(self, rhs: <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Performs the `-` operation. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html#tymethod.sub)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-TreeNode-for-Expr" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html" class="trait" title="trait datafusion::common::tree_node::TreeNode">TreeNode</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

Implementation of the [`TreeNode`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html "trait datafusion::common::tree_node::TreeNode") trait

This allows logical expressions (`Expr`) to be traversed and transformed Facilitates tasks such as optimization and rewriting during query planning.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.apply_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children" class="fn">apply_children</a>\<'n, F\>( &'n self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'n <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies a function `f` to each child expression of `self`.

The function `f` determines whether to continue traversing the tree or to stop. This method collects all child expressions and applies `f` to each.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.map_children" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children" class="fn">map_children</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Maps each child of `self` using the provided closure `f`.

The closure `f` takes ownership of an expression and returns a `Transformed` result, indicating whether the expression was transformed or left unchanged.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit" class="fn">visit</a>\<'n, V\>( &'n self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeVisitor">TreeNodeVisitor</a>\<'n, Node = Self\>,

Visit the tree node with a [`TreeNodeVisitor`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeVisitor.html "trait datafusion::common::tree_node::TreeNodeVisitor"), performing a depth-first walk of the node and its children. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.visit)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.rewrite" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite" class="fn">rewrite</a>\<R\>( self, rewriter: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut R</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where R: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeRewriter">TreeNodeRewriter</a>\<Node = Self\>,

Rewrite the tree node with a [`TreeNodeRewriter`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeRewriter.html "trait datafusion::common::tree_node::TreeNodeRewriter"), performing a depth-first walk of the node and its children. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.rewrite)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.apply" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply" class="fn">apply</a>\<'n, F\>(&'n self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'n Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies `f` to the node then each of its children, recursively (a top-down, pre-order traversal). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.apply)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.transform" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform" class="fn">transform</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Recursively rewrite the node’s children and then the node using `f` (a bottom-up post-order traversal). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.transform_down" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down" class="fn">transform_down</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Recursively rewrite the tree using `f` in a top-down (pre-order) fashion. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.transform_up" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up" class="fn">transform_up</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Recursively rewrite the node using `f` in a bottom-up (post-order) fashion. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_up)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.transform_down_up" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up" class="fn">transform_down_up</a>\<FD, FU\>( self, f_down: FD, f_up: FU, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where FD: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>, FU: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<Self\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Transforms the node using `f_down` while traversing the tree top-down (pre-order), and using `f_up` while traversing the tree bottom-up (post-order). [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.transform_down_up)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.exists" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.exists" class="fn">exists</a>\<F\>(&self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&Self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Returns true if `f` returns true for any node in the tree. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#method.exists)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-TreeNodeContainer%3C&#39;a,+Expr%3E-for-CreateFunctionBody" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeContainer">TreeNodeContainer</a>\<'a, <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateFunctionBody.html" class="struct" title="struct datafusion::logical_expr::CreateFunctionBody">CreateFunctionBody</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.apply_elements-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html#tymethod.apply_elements" class="fn">apply_elements</a>\<F\>( &'a self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies `f` to all elements of the container. This method is usually called from [`TreeNode::apply_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children "method datafusion::common::tree_node::TreeNode::apply_children") implementations as a node is actually a container of the node’s children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.map_elements-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html#tymethod.map_elements" class="fn">map_elements</a>\<F\>( self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.CreateFunctionBody.html" class="struct" title="struct datafusion::logical_expr::CreateFunctionBody">CreateFunctionBody</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Maps all elements of the container with `f`. This method is usually called from [`TreeNode::map_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children "method datafusion::common::tree_node::TreeNode::map_children") implementations as a node is actually a container of the node’s children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-TreeNodeContainer%3C&#39;a,+Expr%3E-for-Expr" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeContainer">TreeNodeContainer</a>\<'a, <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.apply_elements" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html#tymethod.apply_elements" class="fn">apply_elements</a>\<F\>( &'a self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies `f` to all elements of the container. This method is usually called from [`TreeNode::apply_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children "method datafusion::common::tree_node::TreeNode::apply_children") implementations as a node is actually a container of the node’s children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.map_elements" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html#tymethod.map_elements" class="fn">map_elements</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Maps all elements of the container with `f`. This method is usually called from [`TreeNode::map_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children "method datafusion::common::tree_node::TreeNode::map_children") implementations as a node is actually a container of the node’s children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-TreeNodeContainer%3C&#39;a,+Expr%3E-for-OperateFunctionArg" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeContainer">TreeNodeContainer</a>\<'a, <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.OperateFunctionArg.html" class="struct" title="struct datafusion::logical_expr::OperateFunctionArg">OperateFunctionArg</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.apply_elements-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html#tymethod.apply_elements" class="fn">apply_elements</a>\<F\>( &'a self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies `f` to all elements of the container. This method is usually called from [`TreeNode::apply_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children "method datafusion::common::tree_node::TreeNode::apply_children") implementations as a node is actually a container of the node’s children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.map_elements-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html#tymethod.map_elements" class="fn">map_elements</a>\<F\>( self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.OperateFunctionArg.html" class="struct" title="struct datafusion::logical_expr::OperateFunctionArg">OperateFunctionArg</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Maps all elements of the container with `f`. This method is usually called from [`TreeNode::map_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children "method datafusion::common::tree_node::TreeNode::map_children") implementations as a node is actually a container of the node’s children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-TreeNodeContainer%3C&#39;a,+Expr%3E-for-SortExpr" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html" class="trait" title="trait datafusion::common::tree_node::TreeNodeContainer">TreeNodeContainer</a>\<'a, <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.apply_elements-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html#tymethod.apply_elements" class="fn">apply_elements</a>\<F\>( &'a self, f: F, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/enum.TreeNodeRecursion.html" class="enum" title="enum datafusion::common::tree_node::TreeNodeRecursion">TreeNodeRecursion</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Applies `f` to all elements of the container. This method is usually called from [`TreeNode::apply_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.apply_children "method datafusion::common::tree_node::TreeNode::apply_children") implementations as a node is actually a container of the node’s children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#method.map_elements-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNodeContainer.html#tymethod.map_elements" class="fn">map_elements</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">Sort</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html" class="trait" title="trait core::ops::function::FnMut">FnMut</a>(<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/struct.Transformed.html" class="struct" title="struct datafusion::common::tree_node::Transformed">Transformed</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>,

Maps all elements of the container with `f`. This method is usually called from [`TreeNode::map_children`](https://docs.rs/datafusion/50.2.0/datafusion/common/tree_node/trait.TreeNode.html#tymethod.map_children "method datafusion::common::tree_node::TreeNode::map_children") implementations as a node is actually a container of the node’s children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-Eq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#impl-StructuralPartialEq-for-Expr" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#blanket-implementations" class="anchor">§</a>
