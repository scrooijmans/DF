# datafusion.expr — Apache Arrow DataFusion  documentation
This module supports expressions, one of the core concepts in DataFusion.

See [Expressions](about:blank/user-guide/common-operations/expressions.html#expressions) in the online documentation for more details.

Attributes
-------------------------------------------------


|Aggregate                |   |
|-------------------------|---|
|AggregateFunction        |   |
|Alias                    |   |
|Analyze                  |   |
|Between                  |   |
|BinaryExpr               |   |
|Case                     |   |
|Cast                     |   |
|Column                   |   |
|CopyTo                   |   |
|CreateCatalog            |   |
|CreateCatalogSchema      |   |
|CreateExternalTable      |   |
|CreateFunction           |   |
|CreateFunctionBody       |   |
|CreateIndex              |   |
|CreateMemoryTable        |   |
|CreateView               |   |
|Deallocate               |   |
|DescribeTable            |   |
|Distinct                 |   |
|DmlStatement             |   |
|DropCatalogSchema        |   |
|DropFunction             |   |
|DropTable                |   |
|DropView                 |   |
|EXPR_TYPE_ERROR          |   |
|EmptyRelation            |   |
|Execute                  |   |
|Exists                   |   |
|Explain                  |   |
|Extension                |   |
|FileType                 |   |
|Filter                   |   |
|GroupingSet              |   |
|ILike                    |   |
|InList                   |   |
|InSubquery               |   |
|IsFalse                  |   |
|IsNotFalse               |   |
|IsNotNull                |   |
|IsNotTrue                |   |
|IsNotUnknown             |   |
|IsNull                   |   |
|IsTrue                   |   |
|IsUnknown                |   |
|Join                     |   |
|JoinConstraint           |   |
|JoinType                 |   |
|Like                     |   |
|Limit                    |   |
|Literal                  |   |
|Negative                 |   |
|Not                      |   |
|OperateFunctionArg       |   |
|Partitioning             |   |
|Placeholder              |   |
|Prepare                  |   |
|Projection               |   |
|RecursiveQuery           |   |
|Repartition              |   |
|ScalarSubquery           |   |
|ScalarVariable           |   |
|SetVariable              |   |
|SimilarTo                |   |
|Sort                     |   |
|SortKey                  |   |
|Subquery                 |   |
|SubqueryAlias            |   |
|TableScan                |   |
|TransactionAccessMode    |   |
|TransactionConclusion    |   |
|TransactionEnd           |   |
|TransactionIsolationLevel|   |
|TransactionStart         |   |
|TryCast                  |   |
|Union                    |   |
|Unnest                   |   |
|UnnestExpr               |   |
|Values                   |   |
|WindowExpr               |   |


Classes
-------------------------------------------


|CaseBuilder     |Builder class for constructing case statements.           |
|----------------|----------------------------------------------------------|
|Expr            |Expression object.                                        |
|SortExpr        |Used to specify sorting on either a DataFrame or function.|
|Window          |Define reusable window parameters.                        |
|WindowFrame     |Defines a window frame for performing window operations.  |
|WindowFrameBound|Defines a single window frame bound.                      |


Functions
-----------------------------------------------



* ensure_expr(→ datafusion._internal.expr.Expr): ensure_expr_list(→ list[datafusion._internal.expr.Expr])
  * Return the internal expression from Expr or raise TypeError.: Flatten an iterable of expressions, validating each via ensure_expr.


Module Contents
-----------------------------------------------------------

_class_ datafusion.expr.CaseBuilder(_case\_builder: datafusion.\_internal.expr.CaseBuilder_)

Builder class for constructing case statements.

An example usage would be as follows:

```
import datafusion.functions as f
from datafusion import lit, col
df.select(
    f.case(col("column_a"))
    .when(lit(1), lit("One"))
    .when(lit(2), lit("Two"))
    .otherwise(lit("Unknown"))
)

```


Constructs a case builder.

This is not typically called by the end user directly. See [`datafusion.functions.case()`](about:blank/functions/index.html#datafusion.functions.case "datafusion.functions.case") instead.

end() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Finish building a case statement.

Any non-matching cases will end in a null value.

otherwise(_else\_expr: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Set a default value for the case statement.

when(_when\_expr: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_, _then\_expr: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_) → [CaseBuilder](#datafusion.expr.CaseBuilder "datafusion.expr.CaseBuilder")

Add a case to match against.

case\_builder

_class_ datafusion.expr.Expr(_expr: datafusion.\_internal.expr.RawExpr_)

Expression object.

Expressions are one of the core concepts in DataFusion. See [Expressions](about:blank/user-guide/common-operations/expressions.html#expressions) in the online documentation for more information.

This constructor should not be called by the end user.

\_\_add\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Addition operator.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_and\_\_(_rhs: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Logical AND.

\_\_eq\_\_(_rhs: object_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Equal to.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_ge\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Greater than or equal to.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_getitem\_\_(_key: str | int_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Retrieve sub-object.

If `key` is a string, returns the subfield of the struct. If `key` is an integer, retrieves the element in the array. Note that the element index begins at `0`, unlike [`array_element()`](about:blank/functions/index.html#datafusion.functions.array_element "datafusion.functions.array_element") which begins at `1`. If `key` is a slice, returns an array that contains a slice of the original array. Similar to integer indexing, this follows Python convention where the index begins at `0` unlike [`array_slice()`](about:blank/functions/index.html#datafusion.functions.array_slice "datafusion.functions.array_slice") which begins at `1`.

\_\_gt\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Greater than.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_invert\_\_() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Binary not (~).

\_\_le\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Less than or equal to.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_lt\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Less than.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_mod\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Modulo operator (%).

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_mul\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Multiplication operator.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_ne\_\_(_rhs: object_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Not equal to.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_or\_\_(_rhs: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Logical OR.

\_\_repr\_\_() → str

Generate a string representation of this expression.

\_\_richcmp\_\_(_other: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_, _op: int_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Comparison operator.

\_\_sub\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Subtraction operator.

Accepts either an expression or any valid PyArrow scalar literal value.

\_\_truediv\_\_(_rhs: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Division operator.

Accepts either an expression or any valid PyArrow scalar literal value.

abs() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Return the absolute value of a given number.

### Returns:

Expr

A new expression representing the absolute value of the input expression.

acos() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the arc cosine or inverse cosine of a number.

### Returns:

Expr

A new expression representing the arc cosine of the input expression.

acosh() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns inverse hyperbolic cosine.

alias(_name: str_, _metadata: dict\[str, str\] | None \= None_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Assign a name to the expression.

Parameters:

*   **name** – The name to assign to the expression.
    
*   **metadata** – Optional metadata to attach to the expression.
    

Returns:

A new expression with the assigned name.

array\_dims() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns an array of the array’s dimensions.

array\_distinct() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns distinct values from the array after removing duplicates.

array\_empty() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns a boolean indicating whether the array is empty.

array\_length() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the length of the array.

array\_ndims() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the number of dimensions of the array.

array\_pop\_back() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the array without the last element.

array\_pop\_front() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the array without the first element.

arrow\_typeof() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the Arrow type of the expression.

ascii() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the numeric code of the first character of the argument.

asin() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the arc sine or inverse sine of a number.

asinh() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns inverse hyperbolic sine.

atan() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns inverse tangent of a number.

atanh() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns inverse hyperbolic tangent.

between(_low: Any_, _high: Any_, _negated: bool \= False_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns `True` if this expression is between a given range.

Parameters:

*   **low** – lower bound of the range (inclusive).
    
*   **high** – higher bound of the range (inclusive).
    
*   **negated** – negates whether the expression is between a given range
    

bit\_length() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the number of bits in the string argument.

btrim() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Removes all characters, spaces by default, from both sides of a string.

canonical\_name() → str

Returns a complete string representation of this expression.

cardinality() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the total number of elements in the array.

cast(_to: pyarrow.DataType\[Any\] | type_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Cast to a new data type.

cbrt() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the cube root of a number.

ceil() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the nearest integer greater than or equal to argument.

char\_length() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

The number of characters in the `string`.

character\_length() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the number of characters in the argument.

chr() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Converts the Unicode code point to a UTF8 character.

_static_ column(_value: str_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Creates a new expression representing a column.

column\_name(_plan: [datafusion.plan.LogicalPlan](about:blank/plan/index.html#datafusion.plan.LogicalPlan "datafusion.plan.LogicalPlan")_) → str

Compute the output column name based on the provided logical plan.

cos() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the cosine of the argument.

cosh() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the hyperbolic cosine of the argument.

cot() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the cotangent of the argument.

degrees() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Converts the argument from radians to degrees.

display\_name() → str

Returns the name of this expression as it should appear in a schema.

This name will not include any CAST expressions.

distinct() → ExprFuncBuilder

Only evaluate distinct values for an aggregate function.

This function will create an `ExprFuncBuilder` that can be used to set parameters for either window or aggregate functions. If used on any other type of expression, an error will be generated when `build()` is called.

empty() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

This is an alias for [`array_empty()`](#datafusion.expr.Expr.array_empty "datafusion.expr.Expr.array_empty").

exp() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the exponential of the argument.

factorial() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the factorial of the argument.

fill\_nan(_value: Any | [Expr](#datafusion.expr.Expr "datafusion.expr.Expr") | None \= None_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Fill NaN values with a provided value.

fill\_null(_value: Any | [Expr](#datafusion.expr.Expr "datafusion.expr.Expr") | None \= None_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Fill NULL values with a provided value.

filter(_filter: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_) → ExprFuncBuilder

Filter an aggregate function.

This function will create an `ExprFuncBuilder` that can be used to set parameters for either window or aggregate functions. If used on any other type of expression, an error will be generated when `build()` is called.

flatten() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Flattens an array of arrays into a single array.

floor() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the nearest integer less than or equal to the argument.

from\_unixtime() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Converts an integer to RFC3339 timestamp format string.

initcap() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Set the initial letter of each word to capital.

Converts the first letter of each word in `string` to uppercase and the remaining characters to lowercase.

is\_not\_null() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns `True` if this expression is not null.

is\_null() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns `True` if this expression is null.

isnan() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns true if a given number is +NaN or -NaN otherwise returns false.

iszero() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns true if a given number is +0.0 or -0.0 otherwise returns false.

length() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

The number of characters in the `string`.

list\_dims() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns an array of the array’s dimensions.

This is an alias for [`array_dims()`](#datafusion.expr.Expr.array_dims "datafusion.expr.Expr.array_dims").

list\_distinct() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns distinct values from the array after removing duplicates.

This is an alias for [`array_distinct()`](#datafusion.expr.Expr.array_distinct "datafusion.expr.Expr.array_distinct").

list\_length() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the length of the array.

This is an alias for [`array_length()`](#datafusion.expr.Expr.array_length "datafusion.expr.Expr.array_length").

list\_ndims() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the number of dimensions of the array.

This is an alias for [`array_ndims()`](#datafusion.expr.Expr.array_ndims "datafusion.expr.Expr.array_ndims").

_static_ literal(_value: Any_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Creates a new expression representing a scalar value.

`value` must be a valid PyArrow scalar value or easily castable to one.

_static_ literal\_with\_metadata(_value: Any_, _metadata: dict\[str, str\]_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Creates a new expression representing a scalar value with metadata.

Parameters:

*   **value** – A valid PyArrow scalar value or easily castable to one.
    
*   **metadata** – Metadata to attach to the expression.
    

ln() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the natural logarithm (base e) of the argument.

log10() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Base 10 logarithm of the argument.

log2() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Base 2 logarithm of the argument.

lower() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Converts a string to lowercase.

ltrim() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Removes all characters, spaces by default, from the beginning of a string.

md5() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Computes an MD5 128-bit checksum for a string expression.

null\_treatment(_null\_treatment: datafusion.common.NullTreatment_) → ExprFuncBuilder

Set the treatment for `null` values for a window or aggregate function.

This function will create an `ExprFuncBuilder` that can be used to set parameters for either window or aggregate functions. If used on any other type of expression, an error will be generated when `build()` is called.

octet\_length() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the number of bytes of a string.

order\_by(_\*exprs: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr") | [SortExpr](#datafusion.expr.SortExpr "datafusion.expr.SortExpr")_) → ExprFuncBuilder

Set the ordering for a window or aggregate function.

This function will create an `ExprFuncBuilder` that can be used to set parameters for either window or aggregate functions. If used on any other type of expression, an error will be generated when `build()` is called.

over(_window: [Window](#datafusion.expr.Window "datafusion.expr.Window")_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Turn an aggregate function into a window function.

This function turns any aggregate function into a window function. With the exception of `partition_by`, how each of the parameters is used is determined by the underlying aggregate function.

Parameters:

**window** – Window definition

partition\_by(_\*partition\_by: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_) → ExprFuncBuilder

Set the partitioning for a window function.

This function will create an `ExprFuncBuilder` that can be used to set parameters for either window or aggregate functions. If used on any other type of expression, an error will be generated when `build()` is called.

python\_value() → Any

Extracts the Expr value into a PyObject.

This is only valid for literal expressions.

Returns:

Python object representing literal value of the expression.

radians() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Converts the argument from degrees to radians.

reverse() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Reverse the string argument.

rex\_call\_operands() → list\[[Expr](#datafusion.expr.Expr "datafusion.expr.Expr")\]

Return the operands of the expression based on it’s variant type.

Row expressions, Rex(s), operate on the concept of operands. Different variants of Expressions, Expr(s), store those operands in different datastructures. This function examines the Expr variant and returns the operands to the calling logic.

rex\_call\_operator() → str

Extracts the operator associated with a row expression type call.

rex\_type() → datafusion.common.RexType

Return the Rex Type of this expression.

A Rex (Row Expression) specifies a single row of data.That specification could include user defined functions or types. RexType identifies the row as one of the possible valid `RexType`.

rtrim() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Removes all characters, spaces by default, from the end of a string.

schema\_name() → str

Returns the name of this expression as it should appear in a schema.

This name will not include any CAST expressions.

sha224() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Computes the SHA-224 hash of a binary string.

sha256() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Computes the SHA-256 hash of a binary string.

sha384() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Computes the SHA-384 hash of a binary string.

sha512() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Computes the SHA-512 hash of a binary string.

signum() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the sign of the argument (-1, 0, +1).

sin() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the sine of the argument.

sinh() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the hyperbolic sine of the argument.

sort(_ascending: bool \= True_, _nulls\_first: bool \= True_) → [SortExpr](#datafusion.expr.SortExpr "datafusion.expr.SortExpr")

Creates a sort [`Expr`](#datafusion.expr.Expr "datafusion.expr.Expr") from an existing [`Expr`](#datafusion.expr.Expr "datafusion.expr.Expr").

Parameters:

*   **ascending** – If true, sort in ascending order.
    
*   **nulls\_first** – Return null values first.
    

sqrt() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the square root of the argument.

_static_ string\_literal(_value: str_) → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Creates a new expression representing a UTF8 literal value.

It is different from literal because it is pa.string() instead of pa.string\_view()

This is needed for cases where DataFusion is expecting a UTF8 instead of UTF8View literal, like in: [https://github.com/apache/datafusion/blob/86740bfd3d9831d6b7c1d0e1bf4a21d91598a0ac/datafusion/functions/src/core/arrow\_cast.rs#L179](https://github.com/apache/datafusion/blob/86740bfd3d9831d6b7c1d0e1bf4a21d91598a0ac/datafusion/functions/src/core/arrow_cast.rs#L179)

tan() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the tangent of the argument.

tanh() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Returns the hyperbolic tangent of the argument.

to\_hex() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Converts an integer to a hexadecimal string.

to\_variant() → Any

Convert this expression into a python object if possible.

trim() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Removes all characters, spaces by default, from both sides of a string.

types() → datafusion.common.DataTypeMap

Return the `DataTypeMap`.

Returns:

DataTypeMap which represents the PythonType, Arrow DataType, and SqlType Enum which this expression represents.

upper() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Converts a string to uppercase.

variant\_name() → str

Returns the name of the Expr variant.

Ex: `IsNotNull`, `Literal`, `BinaryExpr`, etc

window\_frame(_window\_frame: [WindowFrame](#datafusion.expr.WindowFrame "datafusion.expr.WindowFrame")_) → ExprFuncBuilder

Set the frame fora window function.

This function will create an `ExprFuncBuilder` that can be used to set parameters for either window or aggregate functions. If used on any other type of expression, an error will be generated when `build()` is called.

\_\_radd\_\_

\_\_rand\_\_

\_\_rmod\_\_

\_\_rmul\_\_

\_\_ror\_\_

\_\_rsub\_\_

\_\_rtruediv\_\_

\_to\_pyarrow\_types_: ClassVar\[dict\[type, pyarrow.DataType\]\]_

expr

_class_ datafusion.expr.SortExpr(_expr: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")_, _ascending: bool_, _nulls\_first: bool_)

Used to specify sorting on either a DataFrame or function.

This constructor should not be called by the end user.

\_\_repr\_\_() → str

Generate a string representation of this expression.

ascending() → bool

Return ascending property.

expr() → [Expr](#datafusion.expr.Expr "datafusion.expr.Expr")

Return the raw expr backing the SortExpr.

nulls\_first() → bool

Return nulls\_first property.

raw\_sort

_class_ datafusion.expr.Window(_partition\_by: list\[[Expr](#datafusion.expr.Expr "datafusion.expr.Expr")\] | [Expr](#datafusion.expr.Expr "datafusion.expr.Expr") | None \= None_, _window\_frame: [WindowFrame](#datafusion.expr.WindowFrame "datafusion.expr.WindowFrame") | None \= None_, _order\_by: list\[[SortExpr](#datafusion.expr.SortExpr "datafusion.expr.SortExpr") | [Expr](#datafusion.expr.Expr "datafusion.expr.Expr") | str\] | [Expr](#datafusion.expr.Expr "datafusion.expr.Expr") | [SortExpr](#datafusion.expr.SortExpr "datafusion.expr.SortExpr") | str | None \= None_, _null\_treatment: datafusion.common.NullTreatment | None \= None_)

Define reusable window parameters.

Construct a window definition.

Parameters:

*   **partition\_by** – Partitions for window operation
    
*   **window\_frame** – Define the start and end bounds of the window frame
    
*   **order\_by** – Set ordering
    
*   **null\_treatment** – Indicate how nulls are to be treated
    

\_null\_treatment _\= None_

\_order\_by _\= None_

\_partition\_by _\= None_

\_window\_frame _\= None_

_class_ datafusion.expr.WindowFrame(_units: str_, _start\_bound: Any | None_, _end\_bound: Any | None_)

Defines a window frame for performing window operations.

Construct a window frame using the given parameters.

Parameters:

*   **units** – Should be one of `rows`, `range`, or `groups`.
    
*   **start\_bound** – Sets the preceding bound. Must be >= 0. If none, this will be set to unbounded. If unit type is `groups`, this parameter must be set.
    
*   **end\_bound** – Sets the following bound. Must be >= 0. If none, this will be set to unbounded. If unit type is `groups`, this parameter must be set.
    

\_\_repr\_\_() → str

Print a string representation of the window frame.

get\_frame\_units() → str

Returns the window frame units for the bounds.

get\_lower\_bound() → [WindowFrameBound](#datafusion.expr.WindowFrameBound "datafusion.expr.WindowFrameBound")

Returns starting bound.

get\_upper\_bound() → [WindowFrameBound](#datafusion.expr.WindowFrameBound "datafusion.expr.WindowFrameBound")

Returns end bound.

window\_frame

_class_ datafusion.expr.WindowFrameBound(_frame\_bound: datafusion.\_internal.expr.WindowFrameBound_)

Defines a single window frame bound.

[`WindowFrame`](#datafusion.expr.WindowFrame "datafusion.expr.WindowFrame") typically requires a start and end bound.

Constructs a window frame bound.

get\_offset() → int | None

Returns the offset of the window frame.

is\_current\_row() → bool

Returns if the frame bound is current row.

is\_following() → bool

Returns if the frame bound is following.

is\_preceding() → bool

Returns if the frame bound is preceding.

is\_unbounded() → bool

Returns if the frame bound is unbounded.

frame\_bound

datafusion.expr.ensure\_expr(_value: [Expr](#datafusion.expr.Expr "datafusion.expr.Expr") | Any_) → datafusion.\_internal.expr.Expr

Return the internal expression from `Expr` or raise `TypeError`.

This helper rejects plain strings and other non-[`Expr`](#datafusion.expr.Expr "datafusion.expr.Expr") values so higher level APIs consistently require explicit [`col()`](about:blank/index.html#datafusion.col "datafusion.col") or [`lit()`](about:blank/index.html#datafusion.lit "datafusion.lit") expressions.

Parameters:

**value** – Candidate expression or other object.

Returns:

The internal expression representation.

Raises:

**TypeError** – If `value` is not an instance of [`Expr`](#datafusion.expr.Expr "datafusion.expr.Expr").

datafusion.expr.ensure\_expr\_list(_exprs: Iterable\[[Expr](#datafusion.expr.Expr "datafusion.expr.Expr") | Iterable\[[Expr](#datafusion.expr.Expr "datafusion.expr.Expr")\]\]_) → list\[datafusion.\_internal.expr.Expr\]

Flatten an iterable of expressions, validating each via `ensure_expr`.

Parameters:

**exprs** – Possibly nested iterable containing expressions.

Returns:

A flat list of raw expressions.

Raises:

**TypeError** – If any item is not an instance of [`Expr`](#datafusion.expr.Expr "datafusion.expr.Expr").

datafusion.expr.Aggregate

datafusion.expr.AggregateFunction

datafusion.expr.Alias

datafusion.expr.Analyze

datafusion.expr.Between

datafusion.expr.BinaryExpr

datafusion.expr.Case

datafusion.expr.Cast

datafusion.expr.Column

datafusion.expr.CopyTo

datafusion.expr.CreateCatalog

datafusion.expr.CreateCatalogSchema

datafusion.expr.CreateExternalTable

datafusion.expr.CreateFunction

datafusion.expr.CreateFunctionBody

datafusion.expr.CreateIndex

datafusion.expr.CreateMemoryTable

datafusion.expr.CreateView

datafusion.expr.Deallocate

datafusion.expr.DescribeTable

datafusion.expr.Distinct

datafusion.expr.DmlStatement

datafusion.expr.DropCatalogSchema

datafusion.expr.DropFunction

datafusion.expr.DropTable

datafusion.expr.DropView

datafusion.expr.EXPR\_TYPE\_ERROR _\= 'Use col()/column() or lit()/literal() to construct expressions'_

datafusion.expr.EmptyRelation

datafusion.expr.Execute

datafusion.expr.Exists

datafusion.expr.Explain

datafusion.expr.Extension

datafusion.expr.FileType

datafusion.expr.Filter

datafusion.expr.GroupingSet

datafusion.expr.ILike

datafusion.expr.InList

datafusion.expr.InSubquery

datafusion.expr.IsFalse

datafusion.expr.IsNotFalse

datafusion.expr.IsNotNull

datafusion.expr.IsNotTrue

datafusion.expr.IsNotUnknown

datafusion.expr.IsNull

datafusion.expr.IsTrue

datafusion.expr.IsUnknown

datafusion.expr.Join

datafusion.expr.JoinConstraint

datafusion.expr.JoinType

datafusion.expr.Like

datafusion.expr.Limit

datafusion.expr.Literal

datafusion.expr.Negative

datafusion.expr.Not

datafusion.expr.OperateFunctionArg

datafusion.expr.Partitioning

datafusion.expr.Placeholder

datafusion.expr.Prepare

datafusion.expr.Projection

datafusion.expr.RecursiveQuery

datafusion.expr.Repartition

datafusion.expr.ScalarSubquery

datafusion.expr.ScalarVariable

datafusion.expr.SetVariable

datafusion.expr.SimilarTo

datafusion.expr.Sort

datafusion.expr.SortKey

datafusion.expr.Subquery

datafusion.expr.SubqueryAlias

datafusion.expr.TableScan

datafusion.expr.TransactionAccessMode

datafusion.expr.TransactionConclusion

datafusion.expr.TransactionEnd

datafusion.expr.TransactionIsolationLevel

datafusion.expr.TransactionStart

datafusion.expr.TryCast

datafusion.expr.Union

datafusion.expr.Unnest

datafusion.expr.UnnestExpr

datafusion.expr.Values

datafusion.expr.WindowExpr