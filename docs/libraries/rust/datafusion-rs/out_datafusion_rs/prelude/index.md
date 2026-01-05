# Module prelude Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/prelude.rs.html#18-49" class="src">Source</a>

Expand description

DataFusion “prelude” to simplify importing common types.

Like the standard library’s prelude, this module simplifies importing of common items. Unlike the standard prelude, the contents of this module must be imported manually:

``` rust
use datafusion::prelude::*;
```

## Re-exports<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/index.html#reexports" class="anchor">§</a>

`pub use crate::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/index.html" class="mod" title="mod datafusion::dataframe"><code>dataframe</code></a>`;`

`pub use crate::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/macro.dataframe.html" class="macro" title="macro datafusion::dataframe"><code>dataframe</code></a>`;`

`pub use crate::dataframe::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/dataframe/struct.DataFrame.html" class="struct" title="struct datafusion::dataframe::DataFrame"><code>DataFrame</code></a>`;`

`pub use crate::execution::context::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SQLOptions.html" class="struct" title="struct datafusion::execution::context::SQLOptions"><code>SQLOptions</code></a>`;`

`pub use crate::execution::context::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/context/struct.SessionContext.html" class="struct" title="struct datafusion::execution::context::SessionContext"><code>SessionContext</code></a>`;`

`pub use crate::execution::options::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.AvroReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::AvroReadOptions"><code>AvroReadOptions</code></a>`;`

`pub use crate::execution::options::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::CsvReadOptions"><code>CsvReadOptions</code></a>`;`

`pub use crate::execution::options::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::NdJsonReadOptions"><code>NdJsonReadOptions</code></a>`;`

`pub use crate::execution::options::`<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ParquetReadOptions"><code>ParquetReadOptions</code></a>`;`

## Macros<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/index.html#macros" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/macro.dataframe.html" class="macro" title="macro datafusion::prelude::dataframe">dataframe</a>  
Macro for creating DataFrame.

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.Column.html" class="struct" title="struct datafusion::prelude::Column">Column</a>  
A named reference to a qualified field in a schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.ExprFuncBuilder.html" class="struct" title="struct datafusion::prelude::ExprFuncBuilder">ExprFuncBuilder</a>  
Implementation of [`ExprFunctionExt`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html "trait datafusion::prelude::ExprFunctionExt").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>  
Configuration options for [`SessionContext`](https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SimpleAggregateUDF.html" class="struct" title="struct datafusion::prelude::SimpleAggregateUDF">SimpleAggregateUDF</a>  
Implements [`AggregateUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.AggregateUDFImpl.html "trait datafusion::logical_expr::AggregateUDFImpl") for functions that have a single signature and return type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SimpleScalarUDF.html" class="struct" title="struct datafusion::prelude::SimpleScalarUDF">SimpleScalarUDF</a>  
Implements [`ScalarUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.ScalarUDFImpl.html "trait datafusion::logical_expr::ScalarUDFImpl") for functions that have a single signature and return type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SimpleWindowUDF.html" class="struct" title="struct datafusion::prelude::SimpleWindowUDF">SimpleWindowUDF</a>  
Implements [`WindowUDFImpl`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/trait.WindowUDFImpl.html "trait datafusion::logical_expr::WindowUDFImpl") for functions that have a single signature and return type.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html" class="enum" title="enum datafusion::prelude::Expr">Expr</a>  
Represents logical expressions such as `A + 1`, or `CAST(c1 AS int)`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.ExprFuncKind.html" class="enum" title="enum datafusion::prelude::ExprFuncKind">ExprFuncKind</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.JoinType.html" class="enum" title="enum datafusion::prelude::JoinType">JoinType</a>  
Join type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Partitioning.html" class="enum" title="enum datafusion::prelude::Partitioning">Partitioning</a>  
Logical partitioning schemes supported by [`LogicalPlan::Repartition`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.LogicalPlan.html#variant.Repartition "variant datafusion::logical_expr::LogicalPlan::Repartition")

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Add.html" class="trait" title="trait datafusion::prelude::Add">Add</a>  
The addition operator `+`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitAnd.html" class="trait" title="trait datafusion::prelude::BitAnd">BitAnd</a>  
The bitwise AND operator `&`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitOr.html" class="trait" title="trait datafusion::prelude::BitOr">BitOr</a>  
The bitwise OR operator `|`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.BitXor.html" class="trait" title="trait datafusion::prelude::BitXor">BitXor</a>  
The bitwise XOR operator `^`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Div.html" class="trait" title="trait datafusion::prelude::Div">Div</a>  
The division operator `/`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.ExprFunctionExt.html" class="trait" title="trait datafusion::prelude::ExprFunctionExt">ExprFunctionExt</a>  
Extensions for configuring [`Expr::AggregateFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.AggregateFunction "variant datafusion::prelude::Expr::AggregateFunction") or [`Expr::WindowFunction`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.WindowFunction "variant datafusion::prelude::Expr::WindowFunction")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Mul.html" class="trait" title="trait datafusion::prelude::Mul">Mul</a>  
The multiplication operator `*`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Neg.html" class="trait" title="trait datafusion::prelude::Neg">Neg</a>  
The unary negation operator `-`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Not.html" class="trait" title="trait datafusion::prelude::Not">Not</a>  
The unary logical negation operator `!`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Rem.html" class="trait" title="trait datafusion::prelude::Rem">Rem</a>  
The remainder operator `%`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shl.html" class="trait" title="trait datafusion::prelude::Shl">Shl</a>  
The left shift operator `<<`. Note that because this trait is implemented for all integer types with multiple right-hand-side types, Rust’s type checker has special handling for `_ << _`, setting the result type for integer operations to the type of the left-hand-side operand. This means that though `a << b` and `a.shl(b)` are one and the same from an evaluation standpoint, they are different when it comes to type inference.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Shr.html" class="trait" title="trait datafusion::prelude::Shr">Shr</a>  
The right shift operator `>>`. Note that because this trait is implemented for all integer types with multiple right-hand-side types, Rust’s type checker has special handling for `_ >> _`, setting the result type for integer operations to the type of the left-hand-side operand. This means that though `a >> b` and `a.shr(b)` are one and the same from an evaluation standpoint, they are different when it comes to type inference.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/trait.Sub.html" class="trait" title="trait datafusion::prelude::Sub">Sub</a>  
The subtraction operator `-`.

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.abs.html" class="fn" title="fn datafusion::prelude::abs">abs</a>  
returns the absolute value of a given number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.acos.html" class="fn" title="fn datafusion::prelude::acos">acos</a>  
returns the arc cosine or inverse cosine of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.acosh.html" class="fn" title="fn datafusion::prelude::acosh">acosh</a>  
returns inverse hyperbolic cosine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.and.html" class="fn" title="fn datafusion::prelude::and">and</a>  
Return a new expression with a logical AND

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_any_value.html" class="fn" title="fn datafusion::prelude::array_any_value">array_any_value</a>`nested_expressions`  
returns the first non-null element in the array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_append.html" class="fn" title="fn datafusion::prelude::array_append">array_append</a>`nested_expressions`  
appends an element to the end of an array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_concat.html" class="fn" title="fn datafusion::prelude::array_concat">array_concat</a>`nested_expressions`  
Concatenates arrays.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_dims.html" class="fn" title="fn datafusion::prelude::array_dims">array_dims</a>`nested_expressions`  
returns an array of the array’s dimensions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_distance.html" class="fn" title="fn datafusion::prelude::array_distance">array_distance</a>`nested_expressions`  
returns the Euclidean distance between two numeric arrays.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_distinct.html" class="fn" title="fn datafusion::prelude::array_distinct">array_distinct</a>`nested_expressions`  
returns distinct values from the array after removing duplicates.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_element.html" class="fn" title="fn datafusion::prelude::array_element">array_element</a>`nested_expressions`  
extracts the element with the index n from the array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_empty.html" class="fn" title="fn datafusion::prelude::array_empty">array_empty</a>`nested_expressions`  
returns true for an empty array or false for a non-empty array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_except.html" class="fn" title="fn datafusion::prelude::array_except">array_except</a>`nested_expressions`  
returns an array of the elements that appear in the first array but not in the second.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_has.html" class="fn" title="fn datafusion::prelude::array_has">array_has</a>`nested_expressions`  
returns true, if the element appears in the first array, otherwise false.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_has_all.html" class="fn" title="fn datafusion::prelude::array_has_all">array_has_all</a>`nested_expressions`  
returns true if each element of the second array appears in the first array; otherwise, it returns false.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_has_any.html" class="fn" title="fn datafusion::prelude::array_has_any">array_has_any</a>`nested_expressions`  
returns true if at least one element of the second array appears in the first array; otherwise, it returns false.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_intersect.html" class="fn" title="fn datafusion::prelude::array_intersect">array_intersect</a>`nested_expressions`  
returns an array of the elements in the intersection of array1 and array2.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_length.html" class="fn" title="fn datafusion::prelude::array_length">array_length</a>`nested_expressions`  
returns the length of the array dimension.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_max.html" class="fn" title="fn datafusion::prelude::array_max">array_max</a>`nested_expressions`  
returns the maximum value in the array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_min.html" class="fn" title="fn datafusion::prelude::array_min">array_min</a>`nested_expressions`  
returns the minimum value in the array

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_ndims.html" class="fn" title="fn datafusion::prelude::array_ndims">array_ndims</a>`nested_expressions`  
returns the number of dimensions of the array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_pop_back.html" class="fn" title="fn datafusion::prelude::array_pop_back">array_pop_back</a>`nested_expressions`  
returns the array without the last element.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_pop_front.html" class="fn" title="fn datafusion::prelude::array_pop_front">array_pop_front</a>`nested_expressions`  
returns the array without the first element.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_position.html" class="fn" title="fn datafusion::prelude::array_position">array_position</a>`nested_expressions`  
searches for an element in the array, returns first occurrence.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_positions.html" class="fn" title="fn datafusion::prelude::array_positions">array_positions</a>`nested_expressions`  
searches for an element in the array, returns all occurrences.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_prepend.html" class="fn" title="fn datafusion::prelude::array_prepend">array_prepend</a>`nested_expressions`  
Prepends an element to the beginning of an array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_remove.html" class="fn" title="fn datafusion::prelude::array_remove">array_remove</a>`nested_expressions`  
removes the first element from the array equal to the given value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_remove_all.html" class="fn" title="fn datafusion::prelude::array_remove_all">array_remove_all</a>`nested_expressions`  
removes all elements from the array equal to the given value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_remove_n.html" class="fn" title="fn datafusion::prelude::array_remove_n">array_remove_n</a>`nested_expressions`  
removes the first `max` elements from the array equal to the given value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_repeat.html" class="fn" title="fn datafusion::prelude::array_repeat">array_repeat</a>`nested_expressions`  
returns an array containing element `count` times.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_replace.html" class="fn" title="fn datafusion::prelude::array_replace">array_replace</a>`nested_expressions`  
replaces the first occurrence of the specified element with another specified element.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_replace_all.html" class="fn" title="fn datafusion::prelude::array_replace_all">array_replace_all</a>`nested_expressions`  
replaces all occurrences of the specified element with another specified element.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_replace_n.html" class="fn" title="fn datafusion::prelude::array_replace_n">array_replace_n</a>`nested_expressions`  
replaces the first `max` occurrences of the specified element with another specified element.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_resize.html" class="fn" title="fn datafusion::prelude::array_resize">array_resize</a>`nested_expressions`  
returns an array with the specified size filled with the given value.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_reverse.html" class="fn" title="fn datafusion::prelude::array_reverse">array_reverse</a>`nested_expressions`  
reverses the order of elements in the array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_slice.html" class="fn" title="fn datafusion::prelude::array_slice">array_slice</a>`nested_expressions`  
returns a slice of the array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_sort.html" class="fn" title="fn datafusion::prelude::array_sort">array_sort</a>`nested_expressions`  
returns sorted array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_to_string.html" class="fn" title="fn datafusion::prelude::array_to_string">array_to_string</a>`nested_expressions`  
converts each element to its text representation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.array_union.html" class="fn" title="fn datafusion::prelude::array_union">array_union</a>`nested_expressions`  
returns an array of the elements in the union of array1 and array2 without duplicates.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.arrow_cast.html" class="fn" title="fn datafusion::prelude::arrow_cast">arrow_cast</a>  
Returns value2 if value1 is NULL; otherwise it returns value1

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.arrow_typeof.html" class="fn" title="fn datafusion::prelude::arrow_typeof">arrow_typeof</a>  
Returns the Arrow type of the input expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.ascii.html" class="fn" title="fn datafusion::prelude::ascii">ascii</a>  
Returns the numeric code of the first character of the argument.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.asin.html" class="fn" title="fn datafusion::prelude::asin">asin</a>  
returns the arc sine or inverse sine of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.asinh.html" class="fn" title="fn datafusion::prelude::asinh">asinh</a>  
returns inverse hyperbolic sine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.atan.html" class="fn" title="fn datafusion::prelude::atan">atan</a>  
returns inverse tangent

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.atan2.html" class="fn" title="fn datafusion::prelude::atan2">atan2</a>  
returns inverse tangent of a division given in the argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.atanh.html" class="fn" title="fn datafusion::prelude::atanh">atanh</a>  
returns inverse hyperbolic tangent

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.binary_expr.html" class="fn" title="fn datafusion::prelude::binary_expr">binary_expr</a>  
Return a new expression `left <op> right`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.bit_length.html" class="fn" title="fn datafusion::prelude::bit_length">bit_length</a>  
Returns the number of bits in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.bitwise_and.html" class="fn" title="fn datafusion::prelude::bitwise_and">bitwise_and</a>  
Return a new expression with bitwise AND

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.bitwise_or.html" class="fn" title="fn datafusion::prelude::bitwise_or">bitwise_or</a>  
Return a new expression with bitwise OR

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.bitwise_shift_left.html" class="fn" title="fn datafusion::prelude::bitwise_shift_left">bitwise_shift_left</a>  
Return a new expression with bitwise SHIFT LEFT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.bitwise_shift_right.html" class="fn" title="fn datafusion::prelude::bitwise_shift_right">bitwise_shift_right</a>  
Return a new expression with bitwise SHIFT RIGHT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.bitwise_xor.html" class="fn" title="fn datafusion::prelude::bitwise_xor">bitwise_xor</a>  
Return a new expression with bitwise XOR

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.btrim.html" class="fn" title="fn datafusion::prelude::btrim">btrim</a>  
Removes all characters, spaces by default, from both sides of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.cardinality.html" class="fn" title="fn datafusion::prelude::cardinality">cardinality</a>`nested_expressions`  
returns the total number of elements in the array or map.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.case.html" class="fn" title="fn datafusion::prelude::case">case</a>  
Create a CASE WHEN statement with literal WHEN expressions for comparison to the base expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.cast.html" class="fn" title="fn datafusion::prelude::cast">cast</a>  
Create a cast expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.cbrt.html" class="fn" title="fn datafusion::prelude::cbrt">cbrt</a>  
cube root of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.ceil.html" class="fn" title="fn datafusion::prelude::ceil">ceil</a>  
nearest integer greater than or equal to argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.char_length.html" class="fn" title="fn datafusion::prelude::char_length">char_length</a>  
the number of characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.character_length.html" class="fn" title="fn datafusion::prelude::character_length">character_length</a>  
the number of characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.chr.html" class="fn" title="fn datafusion::prelude::chr">chr</a>  
Converts the Unicode code point to a UTF8 character

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.coalesce.html" class="fn" title="fn datafusion::prelude::coalesce">coalesce</a>  
Returns `coalesce(args...)`, which evaluates to the value of the first expr which is not NULL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.col.html" class="fn" title="fn datafusion::prelude::col">col</a>  
Create a column expression based on a qualified or unqualified column name. Will normalize unquoted identifiers according to SQL rules (identifiers will become lowercase).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.concat.html" class="fn" title="fn datafusion::prelude::concat">concat</a>  
Concatenates the text representations of all the arguments. NULL arguments are ignored

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.concat_ws.html" class="fn" title="fn datafusion::prelude::concat_ws">concat_ws</a>  
Concatenates all but the first argument, with separators. The first argument is used as the separator string, and should not be NULL. Other NULL arguments are ignored.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.contains.html" class="fn" title="fn datafusion::prelude::contains">contains</a>  
Return true if `search_string` is found within `string`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.cos.html" class="fn" title="fn datafusion::prelude::cos">cos</a>  
cosine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.cosh.html" class="fn" title="fn datafusion::prelude::cosh">cosh</a>  
hyperbolic cosine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.cot.html" class="fn" title="fn datafusion::prelude::cot">cot</a>  
cotangent of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.create_udaf.html" class="fn" title="fn datafusion::prelude::create_udaf">create_udaf</a>  
Creates a new UDAF with a specific signature, state type and return type. The signature and state type must match the `Accumulator's implementation`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.create_udf.html" class="fn" title="fn datafusion::prelude::create_udf">create_udf</a>  
Convenience method to create a new user defined scalar function (UDF) with a specific signature and specific return type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.create_udwf.html" class="fn" title="fn datafusion::prelude::create_udwf">create_udwf</a>  
Creates a new UDWF with a specific signature, state type and return type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.cube.html" class="fn" title="fn datafusion::prelude::cube">cube</a>  
Create a grouping set for all combination of `exprs`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.current_date.html" class="fn" title="fn datafusion::prelude::current_date">current_date</a>  
returns current UTC date as a Date32 value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.current_time.html" class="fn" title="fn datafusion::prelude::current_time">current_time</a>  
returns current UTC time as a Time64 value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.date_bin.html" class="fn" title="fn datafusion::prelude::date_bin">date_bin</a>  
coerces an arbitrary timestamp to the start of the nearest specified interval

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.date_part.html" class="fn" title="fn datafusion::prelude::date_part">date_part</a>  
extracts a subfield from the date

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.date_trunc.html" class="fn" title="fn datafusion::prelude::date_trunc">date_trunc</a>  
truncates the date to a specified level of precision

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.decode.html" class="fn" title="fn datafusion::prelude::decode">decode</a>  
decode the `input`, using the `encoding`. encoding can be base64 or hex

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.degrees.html" class="fn" title="fn datafusion::prelude::degrees">degrees</a>  
converts radians to degrees

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.digest.html" class="fn" title="fn datafusion::prelude::digest">digest</a>  
Computes the binary hash of an expression using the specified algorithm.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.encode.html" class="fn" title="fn datafusion::prelude::encode">encode</a>  
encode the `input`, using the `encoding`. encoding can be base64 or hex

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.ends_with.html" class="fn" title="fn datafusion::prelude::ends_with">ends_with</a>  
Returns true if the `string` ends with the `suffix`, false otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.exists.html" class="fn" title="fn datafusion::prelude::exists">exists</a>  
Create an EXISTS subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.exp.html" class="fn" title="fn datafusion::prelude::exp">exp</a>  
exponential

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.factorial.html" class="fn" title="fn datafusion::prelude::factorial">factorial</a>  
factorial

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.find_in_set.html" class="fn" title="fn datafusion::prelude::find_in_set">find_in_set</a>  
Returns a value in the range of 1 to N if the string `str` is in the string list `strlist` consisting of N substrings

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.flatten.html" class="fn" title="fn datafusion::prelude::flatten">flatten</a>`nested_expressions`  
flattens an array of arrays into a single array.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.floor.html" class="fn" title="fn datafusion::prelude::floor">floor</a>  
nearest integer less than or equal to argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.from_unixtime.html" class="fn" title="fn datafusion::prelude::from_unixtime">from_unixtime</a>  
converts an integer to RFC3339 timestamp format string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.gcd.html" class="fn" title="fn datafusion::prelude::gcd">gcd</a>  
greatest common divisor

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.gen_series.html" class="fn" title="fn datafusion::prelude::gen_series">gen_series</a>`nested_expressions`  
create a list of values in the range between start and stop, include upper bound

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.get_field.html" class="fn" title="fn datafusion::prelude::get_field">get_field</a>  
Returns the value of the field with the given name from the struct

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.greatest.html" class="fn" title="fn datafusion::prelude::greatest">greatest</a>  
Returns `greatest(args...)`, which evaluates to the greatest value in the list of expressions or NULL if all the expressions are NULL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.grouping_set.html" class="fn" title="fn datafusion::prelude::grouping_set">grouping_set</a>  
Create a grouping set

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.ident.html" class="fn" title="fn datafusion::prelude::ident">ident</a>  
Create an unqualified column expression from the provided name, without normalizing the column.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.in_list.html" class="fn" title="fn datafusion::prelude::in_list">in_list</a>  
Create an in_list expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.in_subquery.html" class="fn" title="fn datafusion::prelude::in_subquery">in_subquery</a>  
Create an IN subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.initcap.html" class="fn" title="fn datafusion::prelude::initcap">initcap</a>  
converts the first letter of each word in `string` in uppercase and the remaining characters in lowercase

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.instr.html" class="fn" title="fn datafusion::prelude::instr">instr</a>  
finds the position from where the `substring` matches the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.interval_datetime_lit.html" class="fn" title="fn datafusion::prelude::interval_datetime_lit">interval_datetime_lit</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.interval_month_day_nano_lit.html" class="fn" title="fn datafusion::prelude::interval_month_day_nano_lit">interval_month_day_nano_lit</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.interval_year_month_lit.html" class="fn" title="fn datafusion::prelude::interval_year_month_lit">interval_year_month_lit</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.is_false.html" class="fn" title="fn datafusion::prelude::is_false">is_false</a>  
Create is false expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.is_not_false.html" class="fn" title="fn datafusion::prelude::is_not_false">is_not_false</a>  
Create is not false expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.is_not_true.html" class="fn" title="fn datafusion::prelude::is_not_true">is_not_true</a>  
Create is not true expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.is_not_unknown.html" class="fn" title="fn datafusion::prelude::is_not_unknown">is_not_unknown</a>  
Create is not unknown expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.is_null.html" class="fn" title="fn datafusion::prelude::is_null">is_null</a>  
Create is null expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.is_true.html" class="fn" title="fn datafusion::prelude::is_true">is_true</a>  
Create is true expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.is_unknown.html" class="fn" title="fn datafusion::prelude::is_unknown">is_unknown</a>  
Create is unknown expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.isnan.html" class="fn" title="fn datafusion::prelude::isnan">isnan</a>  
returns true if a given number is +NaN or -NaN otherwise returns false

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.iszero.html" class="fn" title="fn datafusion::prelude::iszero">iszero</a>  
returns true if a given number is +0.0 or -0.0 otherwise returns false

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.lcm.html" class="fn" title="fn datafusion::prelude::lcm">lcm</a>  
least common multiple

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.least.html" class="fn" title="fn datafusion::prelude::least">least</a>  
Returns `least(args...)`, which evaluates to the smallest value in the list of expressions or NULL if all the expressions are NULL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.left.html" class="fn" title="fn datafusion::prelude::left">left</a>  
returns the first `n` characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.length.html" class="fn" title="fn datafusion::prelude::length">length</a>  
the number of characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.levenshtein.html" class="fn" title="fn datafusion::prelude::levenshtein">levenshtein</a>  
Returns the Levenshtein distance between the two given strings

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.lit.html" class="fn" title="fn datafusion::prelude::lit">lit</a>  
Create a literal expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.lit_timestamp_nano.html" class="fn" title="fn datafusion::prelude::lit_timestamp_nano">lit_timestamp_nano</a>  
Create a literal timestamp expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.ln.html" class="fn" title="fn datafusion::prelude::ln">ln</a>  
natural logarithm (base e) of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.log.html" class="fn" title="fn datafusion::prelude::log">log</a>  
logarithm of a number for a particular `base`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.log2.html" class="fn" title="fn datafusion::prelude::log2">log2</a>  
base 2 logarithm of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.log10.html" class="fn" title="fn datafusion::prelude::log10">log10</a>  
base 10 logarithm of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.lower.html" class="fn" title="fn datafusion::prelude::lower">lower</a>  
Converts a string to lowercase.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.lpad.html" class="fn" title="fn datafusion::prelude::lpad">lpad</a>  
fill up a string to the length by prepending the characters

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.ltrim.html" class="fn" title="fn datafusion::prelude::ltrim">ltrim</a>  
Removes all characters, spaces by default, from the beginning of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.make_array.html" class="fn" title="fn datafusion::prelude::make_array">make_array</a>`nested_expressions`  
Returns an Arrow array using the specified input expressions.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.make_date.html" class="fn" title="fn datafusion::prelude::make_date">make_date</a>  
make a date from year, month and day component parts

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.map_entries.html" class="fn" title="fn datafusion::prelude::map_entries">map_entries</a>`nested_expressions`  
Return a list of all entries in the map.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.map_extract.html" class="fn" title="fn datafusion::prelude::map_extract">map_extract</a>`nested_expressions`  
Return a list containing the value for a given key or an empty list if the key is not contained in the map.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.map_keys.html" class="fn" title="fn datafusion::prelude::map_keys">map_keys</a>`nested_expressions`  
Return a list of all keys in the map.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.map_values.html" class="fn" title="fn datafusion::prelude::map_values">map_values</a>`nested_expressions`  
Return a list of all values in the map.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.md5.html" class="fn" title="fn datafusion::prelude::md5">md5</a>  
Computes an MD5 128-bit checksum for a string expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.named_struct.html" class="fn" title="fn datafusion::prelude::named_struct">named_struct</a>  
Returns a struct with the given names and arguments pairs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.nanvl.html" class="fn" title="fn datafusion::prelude::nanvl">nanvl</a>  
returns x if x is not NaN otherwise returns y

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.not.html" class="fn" title="fn datafusion::prelude::not">not</a>  
Return a new expression with a logical NOT

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.not_exists.html" class="fn" title="fn datafusion::prelude::not_exists">not_exists</a>  
Create a NOT EXISTS subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.not_in_subquery.html" class="fn" title="fn datafusion::prelude::not_in_subquery">not_in_subquery</a>  
Create a NOT IN subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.now.html" class="fn" title="fn datafusion::prelude::now">now</a>  
returns the current timestamp in nanoseconds, using the same value for all instances of now() in same statement

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.nullif.html" class="fn" title="fn datafusion::prelude::nullif">nullif</a>  
Returns NULL if value1 equals value2; otherwise it returns value1. This can be used to perform the inverse operation of the COALESCE expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.nvl.html" class="fn" title="fn datafusion::prelude::nvl">nvl</a>  
Returns value2 if value1 is NULL; otherwise it returns value1

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.nvl2.html" class="fn" title="fn datafusion::prelude::nvl2">nvl2</a>  
Returns value2 if value1 is not NULL; otherwise, it returns value3.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.octet_length.html" class="fn" title="fn datafusion::prelude::octet_length">octet_length</a>  
returns the number of bytes of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.or.html" class="fn" title="fn datafusion::prelude::or">or</a>  
Return a new expression with a logical OR

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.out_ref_col.html" class="fn" title="fn datafusion::prelude::out_ref_col">out_ref_col</a>  
Create an out reference column which hold a reference that has been resolved to a field outside of the current plan.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.overlay.html" class="fn" title="fn datafusion::prelude::overlay">overlay</a>  
replace the substring of string that starts at the start’th character and extends for count characters with new substring

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.pi.html" class="fn" title="fn datafusion::prelude::pi">pi</a>  
Returns an approximate value of π

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.placeholder.html" class="fn" title="fn datafusion::prelude::placeholder">placeholder</a>  
Create placeholder value that will be filled in (such as `$1`)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.position.html" class="fn" title="fn datafusion::prelude::position">position</a>  
finds the position from where the `substring` matches the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.power.html" class="fn" title="fn datafusion::prelude::power">power</a>  
`base` raised to the power of `exponent`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.qualified_wildcard.html" class="fn" title="fn datafusion::prelude::qualified_wildcard">qualified_wildcard</a>  
Create an ‘t.\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression that matches all columns from a specific table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.qualified_wildcard_with_options.html" class="fn" title="fn datafusion::prelude::qualified_wildcard_with_options">qualified_wildcard_with_options</a>  
Create an ‘t.\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression with the wildcard options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.radians.html" class="fn" title="fn datafusion::prelude::radians">radians</a>  
converts degrees to radians

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.random.html" class="fn" title="fn datafusion::prelude::random">random</a>  
Returns a random value in the range 0.0 \<= x \< 1.0

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.range.html" class="fn" title="fn datafusion::prelude::range">range</a>`nested_expressions`  
create a list of values in the range between start and stop

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.regexp_count.html" class="fn" title="fn datafusion::prelude::regexp_count">regexp_count</a>  
Returns the number of consecutive occurrences of a regular expression in a string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.regexp_instr.html" class="fn" title="fn datafusion::prelude::regexp_instr">regexp_instr</a>  
Returns index of regular expression matches in a string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.regexp_like.html" class="fn" title="fn datafusion::prelude::regexp_like">regexp_like</a>  
Returns true if a regex has at least one match in a string, false otherwise.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.regexp_match.html" class="fn" title="fn datafusion::prelude::regexp_match">regexp_match</a>  
Returns a list of regular expression matches in a string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.regexp_replace.html" class="fn" title="fn datafusion::prelude::regexp_replace">regexp_replace</a>  
Replaces substrings in a string that match.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.repeat.html" class="fn" title="fn datafusion::prelude::repeat">repeat</a>  
Repeats the `string` to `n` times

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.replace.html" class="fn" title="fn datafusion::prelude::replace">replace</a>  
Replaces all occurrences of `from` with `to` in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.reverse.html" class="fn" title="fn datafusion::prelude::reverse">reverse</a>  
reverses the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.right.html" class="fn" title="fn datafusion::prelude::right">right</a>  
returns the last `n` characters in the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.rollup.html" class="fn" title="fn datafusion::prelude::rollup">rollup</a>  
Create a grouping set for rollup

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.round.html" class="fn" title="fn datafusion::prelude::round">round</a>  
round to nearest integer

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.rpad.html" class="fn" title="fn datafusion::prelude::rpad">rpad</a>  
fill up a string to the length by appending the characters

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.rtrim.html" class="fn" title="fn datafusion::prelude::rtrim">rtrim</a>  
Removes all characters, spaces by default, from the end of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.scalar_subquery.html" class="fn" title="fn datafusion::prelude::scalar_subquery">scalar_subquery</a>  
Create a scalar subquery expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.sha224.html" class="fn" title="fn datafusion::prelude::sha224">sha224</a>  
Computes the SHA-224 hash of a binary string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.sha256.html" class="fn" title="fn datafusion::prelude::sha256">sha256</a>  
Computes the SHA-256 hash of a binary string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.sha384.html" class="fn" title="fn datafusion::prelude::sha384">sha384</a>  
Computes the SHA-384 hash of a binary string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.sha512.html" class="fn" title="fn datafusion::prelude::sha512">sha512</a>  
Computes the SHA-512 hash of a binary string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.signum.html" class="fn" title="fn datafusion::prelude::signum">signum</a>  
sign of the argument (-1, 0, +1)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.sin.html" class="fn" title="fn datafusion::prelude::sin">sin</a>  
sine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.sinh.html" class="fn" title="fn datafusion::prelude::sinh">sinh</a>  
hyperbolic sine

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.split_part.html" class="fn" title="fn datafusion::prelude::split_part">split_part</a>  
Splits a string based on a delimiter and picks out the desired field based on the index.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.sqrt.html" class="fn" title="fn datafusion::prelude::sqrt">sqrt</a>  
square root of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.starts_with.html" class="fn" title="fn datafusion::prelude::starts_with">starts_with</a>  
Returns true if string starts with prefix.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.string_to_array.html" class="fn" title="fn datafusion::prelude::string_to_array">string_to_array</a>`nested_expressions`  
splits a `string` based on a `delimiter` and returns an array of parts. Any parts matching the optional `null_string` will be replaced with `NULL`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.strpos.html" class="fn" title="fn datafusion::prelude::strpos">strpos</a>  
finds the position from where the `substring` matches the `string`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.struct.html" class="fn" title="fn datafusion::prelude::struct">struct</a>  
Returns a struct with the given arguments

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.substr.html" class="fn" title="fn datafusion::prelude::substr">substr</a>  
substring from the `position` to the end

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.substr_index.html" class="fn" title="fn datafusion::prelude::substr_index">substr_index</a>  
Returns the substring from str before count occurrences of the delimiter

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.substring.html" class="fn" title="fn datafusion::prelude::substring">substring</a>  
substring from the `position` with `length` characters

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.tan.html" class="fn" title="fn datafusion::prelude::tan">tan</a>  
returns the tangent of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.tanh.html" class="fn" title="fn datafusion::prelude::tanh">tanh</a>  
returns the hyperbolic tangent of a number

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_char.html" class="fn" title="fn datafusion::prelude::to_char">to_char</a>  
Returns a string representation of a date, time, timestamp or duration based on a Chrono pattern.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_date.html" class="fn" title="fn datafusion::prelude::to_date">to_date</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_hex.html" class="fn" title="fn datafusion::prelude::to_hex">to_hex</a>  
Converts an integer to a hexadecimal string.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_local_time.html" class="fn" title="fn datafusion::prelude::to_local_time">to_local_time</a>  
converts a timezone-aware timestamp to local time (with no offset or timezone information), i.e. strips off the timezone from the timestamp

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_timestamp.html" class="fn" title="fn datafusion::prelude::to_timestamp">to_timestamp</a>  
converts a string and optional formats to a `Timestamp(Nanoseconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_timestamp_micros.html" class="fn" title="fn datafusion::prelude::to_timestamp_micros">to_timestamp_micros</a>  
converts a string and optional formats to a `Timestamp(Microseconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_timestamp_millis.html" class="fn" title="fn datafusion::prelude::to_timestamp_millis">to_timestamp_millis</a>  
converts a string and optional formats to a `Timestamp(Milliseconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_timestamp_nanos.html" class="fn" title="fn datafusion::prelude::to_timestamp_nanos">to_timestamp_nanos</a>  
converts a string and optional formats to a `Timestamp(Nanoseconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_timestamp_seconds.html" class="fn" title="fn datafusion::prelude::to_timestamp_seconds">to_timestamp_seconds</a>  
converts a string and optional formats to a `Timestamp(Seconds, None)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.to_unixtime.html" class="fn" title="fn datafusion::prelude::to_unixtime">to_unixtime</a>  
converts a string and optional formats to a Unixtime

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.translate.html" class="fn" title="fn datafusion::prelude::translate">translate</a>  
replaces the characters in `from` with the counterpart in `to`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.trim.html" class="fn" title="fn datafusion::prelude::trim">trim</a>  
Removes all characters, spaces by default, from both sides of a string

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.trunc.html" class="fn" title="fn datafusion::prelude::trunc">trunc</a>  
truncate toward zero, with optional precision

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.try_cast.html" class="fn" title="fn datafusion::prelude::try_cast">try_cast</a>  
Create a try cast expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.union_extract.html" class="fn" title="fn datafusion::prelude::union_extract">union_extract</a>  
Returns the value of the field with the given name from the union when it’s selected, or NULL otherwise

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.union_tag.html" class="fn" title="fn datafusion::prelude::union_tag">union_tag</a>  
Returns the name of the currently selected field in the union

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.unnest.html" class="fn" title="fn datafusion::prelude::unnest">unnest</a>  
Create a Unnest expression

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.upper.html" class="fn" title="fn datafusion::prelude::upper">upper</a>  
Converts a string to uppercase.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.uuid.html" class="fn" title="fn datafusion::prelude::uuid">uuid</a>  
returns uuid v4 as a string value

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.when.html" class="fn" title="fn datafusion::prelude::when">when</a>  
Create a CASE WHEN statement with boolean WHEN expressions and no base expression.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.wildcard.html" class="fn" title="fn datafusion::prelude::wildcard">wildcard</a>  
Create an ‘\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression that matches all columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/fn.wildcard_with_options.html" class="fn" title="fn datafusion::prelude::wildcard_with_options">wildcard_with_options</a>  
Create an ‘\*’ [`Expr::Wildcard`](https://docs.rs/datafusion/50.2.0/datafusion/prelude/enum.Expr.html#variant.Wildcard "variant datafusion::prelude::Expr::Wildcard") expression with the wildcard options
