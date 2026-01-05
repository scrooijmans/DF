# Expression API — Apache DataFusion  documentation
DataFrame methods such as `select` and `filter` accept one or more logical expressions and there are many functions available for creating logical expressions. These are documented below.

Tip

Most functions and methods may receive and return an `Expr`, which can be chained together using a fluent-style API:

```
use datafusion::prelude::*;
// create the expression `(a > 6) AND (b < 7)`
col("a").gt(lit(6)).and(col("b").lt(lit(7)));

```


Identifiers
---------------------------------------------------


|Syntax    |Description                               |
|----------|------------------------------------------|
|col(ident)|Reference a column in a dataframe col("a")|


Note

ident

A type which implement `Into<Column>` trait

Literal Values
---------------------------------------------------------


|Syntax    |Description                                   |
|----------|----------------------------------------------|
|lit(value)|Literal value such as lit(123) or lit("hello")|


Note

value

A type which implement `Literal`

Boolean Expressions
-------------------------------------------------------------------


|Syntax             |Description|
|-------------------|-----------|
|and(x, y), x.and(y)|Logical AND|
|or(x, y), x.or(y)  |Logical OR |
|!x, not(x), x.not()|Logical NOT|


Note

`!` is a bitwise or logical complement operator in Rust, but it only works as a logical NOT in expression API.

Note

Since `&&` and `||` are logical operators in Rust and cannot be overloaded these are not available in the expression API.

Bitwise Expressions
-------------------------------------------------------------------


|Syntax                                     |Description|
|-------------------------------------------|-----------|
|x & y, bitwise_and(x, y), x.bitand(y)      |AND        |
|x | y, bitwise_or(x, y), x.bitor(y)        |OR         |
|x ^ y, bitwise_xor(x, y), x.bitxor(y)      |XOR        |
|x << y, bitwise_shift_left(x, y), x.shl(y) |Left shift |
|x >> y, bitwise_shift_right(x, y), x.shr(y)|Right shift|


Comparison Expressions
-------------------------------------------------------------------------


|Syntax     |Description          |
|-----------|---------------------|
|x.eq(y)    |Equal                |
|x.not_eq(y)|Not Equal            |
|x.gt(y)    |Greater Than         |
|x.gt_eq(y) |Greater Than or Equal|
|x.lt(y)    |Less Than            |
|x.lt_eq(y) |Less Than or Equal   |


Note

Comparison operators (`<`, `<=`, `==`, `>=`, `>`) could be overloaded by the `PartialOrd` and `PartialEq` trait in Rust, but these operators always return a `bool` which makes them not work with the expression API.

Arithmetic Expressions
-------------------------------------------------------------------------


|Syntax         |Description   |
|---------------|--------------|
|x + y, x.add(y)|Addition      |
|x - y, x.sub(y)|Subtraction   |
|x * y, x.mul(y)|Multiplication|
|x / y, x.div(y)|Division      |
|x % y, x.rem(y)|Remainder     |
|-x, x.neg()    |Negation      |


Math Functions
---------------------------------------------------------


|Syntax               |Description                                      |
|---------------------|-------------------------------------------------|
|abs(x)               |absolute value                                   |
|acos(x)              |inverse cosine                                   |
|acosh(x)             |inverse hyperbolic cosine                        |
|asin(x)              |inverse sine                                     |
|asinh(x)             |inverse hyperbolic sine                          |
|atan(x)              |inverse tangent                                  |
|atanh(x)             |inverse hyperbolic tangent                       |
|atan2(y, x)          |inverse tangent of y / x                         |
|cbrt(x)              |cube root                                        |
|ceil(x)              |nearest integer greater than or equal to argument|
|cos(x)               |cosine                                           |
|cosh(x)              |hyperbolic cosine                                |
|degrees(x)           |converts radians to degrees                      |
|exp(x)               |exponential                                      |
|factorial(x)         |factorial                                        |
|floor(x)             |nearest integer less than or equal to argument   |
|gcd(x, y)            |greatest common divisor                          |
|isnan(x)             |predicate determining whether NaN/-NaN or not    |
|iszero(x)            |predicate determining whether 0.0/-0.0 or not    |
|lcm(x, y)            |least common multiple                            |
|ln(x)                |natural logarithm                                |
|log(base, x)         |logarithm of x for a particular base             |
|log10(x)             |base 10 logarithm                                |
|log2(x)              |base 2 logarithm                                 |
|nanvl(x, y)          |returns x if x is not NaN otherwise returns y    |
|pi()                 |approximate value of π                           |
|power(base, exponent)|base raised to the power of exponent             |
|radians(x)           |converts degrees to radians                      |
|round(x)             |round to nearest integer                         |
|signum(x)            |sign of the argument (-1, 0, +1)                 |
|sin(x)               |sine                                             |
|sinh(x)              |hyperbolic sine                                  |
|sqrt(x)              |square root                                      |
|tan(x)               |tangent                                          |
|tanh(x)              |hyperbolic tangent                               |
|trunc(x)             |truncate toward zero                             |


Note

Unlike to some databases the math functions in Datafusion works the same way as Rust math functions, avoiding failing on corner cases e.g.

```
select log(-1), log(0), sqrt(-1);
+----------------+---------------+-----------------+
| log(Int64(-1)) | log(Int64(0)) | sqrt(Int64(-1)) |
+----------------+---------------+-----------------+
| NaN            | -inf          | NaN             |
+----------------+---------------+-----------------+

```


Conditional Expressions
---------------------------------------------------------------------------



* Syntax: coalesce([value, …])
  * Description: Returns the first of its arguments that is not null. Null is returned only if all arguments are null. It is often used to substitute a default value for null values when data is retrieved for display.
* Syntax: case(expr)    .when(expr)    .end(),case(expr)    .when(expr)    .otherwise(expr)
  * Description: CASE expression. The expression may chain multiple when expressions and end with an end or otherwise expression. Example: urltomarkdowncodeblockplaceholder20.9481414072109278or, end with otherwise to match any other conditions: urltomarkdowncodeblockplaceholder30.33038266021837015
* Syntax: nullif(value1, value2)
  * Description: Returns a null value if value1 equals value2; otherwise it returns value1. This can be used to perform the inverse operation of the coalesce expression.


String Expressions
-----------------------------------------------------------------



* Syntax: ascii(character)
  * Description: Returns a numeric representation of the character (character). Example: ascii('a') -> 97
* Syntax: bit_length(text)
  * Description: Returns the length of the string (text) in bits. Example: bit_length('spider') -> 48
* Syntax: btrim(text, characters)
  * Description: Removes all specified characters (characters) from both the beginning and the end of the string (text). Example: btrim('aabchelloccb', 'abc') -> hello
* Syntax: char_length(text)
  * Description: Returns number of characters in the string (text). The same as character_length and length. Example: character_length('lion') -> 4
* Syntax: character_length(text)
  * Description: Returns number of characters in the string (text). The same as char_length and length. Example: char_length('lion') -> 4
* Syntax: concat(value1, [value2 [, …]])
  * Description: Concatenates the text representations (value1, [value2 [, ...]]) of all the arguments. NULL arguments are ignored. Example: concat('aaa', 'bbc', NULL, 321) -> aaabbc321
* Syntax: concat_ws(separator, value1, [value2 [, …]])
  * Description: Concatenates the text representations (value1, [value2 [, ...]]) of all the arguments with the separator (separator). NULL arguments are ignored. concat_ws('/', 'path', 'to', NULL, 'my', 'folder', 123) -> path/to/my/folder/123
* Syntax: chr(integer)
  * Description: Returns a character by its numeric representation (integer). Example: chr(90) -> 8
* Syntax: initcap
  * Description: Converts the first letter of each word to upper case and the rest to lower case. Example: initcap('hi TOM') -> Hi Tom
* Syntax: left(text, number)
  * Description: Returns a certain number (number) of first characters (text). Example: left('like', 2) -> li
* Syntax: length(text)
  * Description: Returns number of characters in the string (text). The same as character_length and char_length. Example: length('lion') -> 4
* Syntax: lower(text)
  * Description: Converts all characters in the string (text) into lower case. Example: lower('HELLO') -> hello
* Syntax: lpad(text, length, [, fill])
  * Description: Extends the string to length (length) by prepending the characters (fill) (a space by default). Example: lpad('bb', 5, 'a') → aaabb
* Syntax: ltrim(text, text)
  * Description: Removes all specified characters (characters) from the beginning of the string (text). Example: ltrim('aabchelloccb', 'abc') -> helloccb
* Syntax: md5(text)
  * Description: Computes the MD5 hash of the argument (text).
* Syntax: octet_length(text)
  * Description: Returns number of bytes in the string (text).
* Syntax: repeat(text, number)
  * Description: Repeats the string the specified number of times. Example: repeat('1', 4) -> 1111
* Syntax: replace(string, from, to)
  * Description: Replaces a specified string (from) with another specified string (to) in the string (string). Example: replace('Hello', 'replace', 'el') -> Hola
* Syntax: reverse(text)
  * Description: Reverses the order of the characters in the string (text). Example: reverse('hello') -> olleh
* Syntax: right(text, number)
  * Description: Returns a certain number (number) of last characters (text). Example: right('like', 2) -> ke
* Syntax: rpad(text, length, [, fill])
  * Description: Extends the string to length (length) by prepending the characters (fill) (a space by default). Example: rpad('bb', 5, 'a') → bbaaa
* Syntax: rtrim
  * Description: Removes all specified characters (characters) from the end of the string (text). Example: rtrim('aabchelloccb', 'abc') -> aabchello
* Syntax: digest(input, algorithm)
  * Description: Computes the binary hash of input, using the algorithm.
* Syntax: split_part(string, delimiter, index)
  * Description: Splits the string (string) based on a delimiter (delimiter) and picks out the desired field based on the index (index).
* Syntax: starts_with(string, prefix)
  * Description: Returns true if the string (string) starts with the specified prefix (prefix). If not, it returns false. Example: starts_with('Hi Tom', 'Hi') -> true
* Syntax: strpos
  * Description: Finds the position from where the substring matches the string
* Syntax: substr(string, position, [, length])
  * Description: Returns substring from the position (position) with length (length) characters in the string (string).
* Syntax: translate(string, from, to)
  * Description: Replaces the characters in from with the counterpart in to. Example: translate('abcde', 'acd', '15') -> 1b5e
* Syntax: trim(string)
  * Description: Removes all characters, space by default from the string (string)
* Syntax: upper
  * Description: Converts all characters in the string into upper case. Example: upper('hello') -> HELLO


Array Expressions
---------------------------------------------------------------



* Syntax: array_any_value(array)
  * Description: Returns the first non-null element in the array. array_any_value([NULL, 1, 2, 3]) -> 1
* Syntax: array_append(array, element)
  * Description: Appends an element to the end of an array. array_append([1, 2, 3], 4) -> [1, 2, 3, 4]
* Syntax: array_concat(array[, …, array_n])
  * Description: Concatenates arrays. array_concat([1, 2, 3], [4, 5, 6]) -> [1, 2, 3, 4, 5, 6]
* Syntax: array_has(array, element)
  * Description: Returns true if the array contains the element array_has([1,2,3], 1) -> true
* Syntax: array_has_all(array, sub-array)
  * Description: Returns true if all elements of sub-array exist in array array_has_all([1,2,3], [1,3]) -> true
* Syntax: array_has_any(array, sub-array)
  * Description: Returns true if any elements exist in both arrays array_has_any([1,2,3], [1,4]) -> true
* Syntax: array_dims(array)
  * Description: Returns an array of the array’s dimensions. array_dims([[1, 2, 3], [4, 5, 6]]) -> [2, 3]
* Syntax: array_distinct(array)
  * Description: Returns distinct values from the array after removing duplicates. array_distinct([1, 3, 2, 3, 1, 2, 4]) -> [1, 2, 3, 4]
* Syntax: array_element(array, index)
  * Description: Extracts the element with the index n from the array array_element([1, 2, 3, 4], 3) -> 3
* Syntax: empty(array)
  * Description: Returns true for an empty array or false for a non-empty array. empty([1]) -> false
* Syntax: flatten(array)
  * Description: Converts an array of arrays to a flat array flatten([[1], [2, 3], [4, 5, 6]]) -> [1, 2, 3, 4, 5, 6]
* Syntax: array_length(array, dimension)
  * Description: Returns the length of the array dimension. array_length([1, 2, 3, 4, 5]) -> 5
* Syntax: array_ndims(array)
  * Description: Returns the number of dimensions of the array. array_ndims([[1, 2, 3], [4, 5, 6]]) -> 2
* Syntax: array_pop_front(array)
  * Description: Returns the array without the first element. array_pop_front([1, 2, 3]) -> [2, 3]
* Syntax: array_pop_back(array)
  * Description: Returns the array without the last element. array_pop_back([1, 2, 3]) -> [1, 2]
* Syntax: array_position(array, element)
  * Description: Searches for an element in the array, returns first occurrence. array_position([1, 2, 2, 3, 4], 2) -> 2
* Syntax: array_positions(array, element)
  * Description: Searches for an element in the array, returns all occurrences. array_positions([1, 2, 2, 3, 4], 2) -> [2, 3]
* Syntax: array_prepend(element, array)
  * Description: Prepends an element to the beginning of an array. array_prepend(1, [2, 3, 4]) -> [1, 2, 3, 4]
* Syntax: array_repeat(element, count)
  * Description: Returns an array containing element count times. array_repeat(1, 3) -> [1, 1, 1]
* Syntax: array_remove(array, element)
  * Description: Removes the first element from the array equal to the given value. array_remove([1, 2, 2, 3, 2, 1, 4], 2) -> [1, 2, 3, 2, 1, 4]
* Syntax: array_remove_n(array, element, max)
  * Description: Removes the first max elements from the array equal to the given value. array_remove_n([1, 2, 2, 3, 2, 1, 4], 2, 2) -> [1, 3, 2, 1, 4]
* Syntax: array_remove_all(array, element)
  * Description: Removes all elements from the array equal to the given value. array_remove_all([1, 2, 2, 3, 2, 1, 4], 2) -> [1, 3, 1, 4]
* Syntax: array_replace(array, from, to)
  * Description: Replaces the first occurrence of the specified element with another specified element. array_replace([1, 2, 2, 3, 2, 1, 4], 2, 5) -> [1, 5, 2, 3, 2, 1, 4]
* Syntax: array_replace_n(array, from, to, max)
  * Description: Replaces the first max occurrences of the specified element with another specified element. array_replace_n([1, 2, 2, 3, 2, 1, 4], 2, 5, 2) -> [1, 5, 5, 3, 2, 1, 4]
* Syntax: array_replace_all(array, from, to)
  * Description: Replaces all occurrences of the specified element with another specified element. array_replace_all([1, 2, 2, 3, 2, 1, 4], 2, 5) -> [1, 5, 5, 3, 5, 1, 4]
* Syntax: array_slice(array, begin,end)
  * Description: Returns a slice of the array. array_slice([1, 2, 3, 4, 5, 6, 7, 8], 3, 6) -> [3, 4, 5, 6]
* Syntax: array_slice(array, begin, end, stride)
  * Description: Returns a slice of the array with added stride feature. array_slice([1, 2, 3, 4, 5, 6, 7, 8], 3, 6, 2) -> [3, 5, 6]
* Syntax: array_to_string(array, delimiter)
  * Description: Converts each element to its text representation. array_to_string([1, 2, 3, 4], ',') -> 1,2,3,4
* Syntax: array_intersect(array1, array2)
  * Description: Returns an array of the elements in the intersection of array1 and array2. array_intersect([1, 2, 3, 4], [5, 6, 3, 4]) -> [3, 4]
* Syntax: array_union(array1, array2)
  * Description: Returns an array of the elements in the union of array1 and array2 without duplicates. array_union([1, 2, 3, 4], [5, 6, 3, 4]) -> [1, 2, 3, 4, 5, 6]
* Syntax: array_except(array1, array2)
  * Description: Returns an array of the elements that appear in the first array but not in the second. array_except([1, 2, 3, 4], [5, 6, 3, 4]) -> [1, 2]
* Syntax: array_resize(array, size, value)
  * Description: Resizes the list to contain size elements. Initializes new elements with value or empty if value is not set. array_resize([1, 2, 3], 5, 0) -> [1, 2, 3, 0, 0]
* Syntax: array_sort(array, desc, null_first)
  * Description: Returns sorted array. array_sort([3, 1, 2, 5, 4]) -> [1, 2, 3, 4, 5]
* Syntax: cardinality(array/map)
  * Description: Returns the total number of elements in the array or map. cardinality([[1, 2, 3], [4, 5, 6]]) -> 6
* Syntax: make_array(value1, [value2 [, …]])
  * Description: Returns an Arrow array using the specified input expressions. make_array(1, 2, 3) -> [1, 2, 3]
* Syntax: range(start [, stop, step])
  * Description: Returns an Arrow array between start and stop with step. SELECT range(2, 10, 3) -> [2, 5, 8]
* Syntax: string_to_array(array, delimiter, null_string)
  * Description: Splits a string based on a delimiter and returns an array of parts. Any parts matching the optional null_string will be replaced with NULL. string_to_array('abc#def#ghi', '#', ' ') -> ['abc', 'def', 'ghi']
* Syntax: trim_array(array, n)
  * Description: Deprecated


Regular Expressions
-------------------------------------------------------------------


|Syntax        |Description                                                                  |
|--------------|-----------------------------------------------------------------------------|
|regexp_match  |Matches a regular expression against a string and returns matched substrings.|
|regexp_replace|Replaces strings that match a regular expression                             |


Temporal Expressions
---------------------------------------------------------------------


|Syntax              |Description                                          |
|--------------------|-----------------------------------------------------|
|date_part           |Extracts a subfield from the date.                   |
|date_trunc          |Truncates the date to a specified level of precision.|
|from_unixtime       |Returns the unix time in format.                     |
|to_timestamp        |Converts a string to a Timestamp(_, _)               |
|to_timestamp_millis |Converts a string to a Timestamp(Milliseconds, None) |
|to_timestamp_micros |Converts a string to a Timestamp(Microseconds, None) |
|to_timestamp_seconds|Converts a string to a Timestamp(Seconds, None)      |
|now()               |Returns current time.                                |


Other Expressions
---------------------------------------------------------------



* Syntax: array([value1, …])
  * Description: Returns an array of fixed size with each argument ([value1, ...]) on it.
* Syntax: in_list(expr, list, negated)
  * Description: Returns true if (expr) belongs or not belongs (negated) to a list (list), otherwise returns false.
* Syntax: random()
  * Description: Returns a random value from 0 (inclusive) to 1 (exclusive).
* Syntax: sha224(text)
  * Description: Computes the SHA224 hash of the argument (text).
* Syntax: sha256(text)
  * Description: Computes the SHA256 hash of the argument (text).
* Syntax: sha384(text)
  * Description: Computes the SHA384 hash of the argument (text).
* Syntax: sha512(text)
  * Description: Computes the SHA512 hash of the argument (text).
* Syntax: to_hex(integer)
  * Description: Converts the integer (integer) to the corresponding hexadecimal string.


Aggregate Functions
-------------------------------------------------------------------



* Syntax: avg(expr)
  * Description: Сalculates the average value for expr.
* Syntax: avg_distinct(expr)
  * Description: Creates an expression to represent the avg(distinct) aggregate function
* Syntax: approx_distinct(expr)
  * Description: Calculates an approximate count of the number of distinct values for expr.
* Syntax: approx_median(expr)
  * Description: Calculates an approximation of the median for expr.
* Syntax: approx_percentile_cont(expr, percentile [, centroids])
  * Description: Calculates an approximation of the specified percentile for expr. Optional centroids parameter controls accuracy (default: 100).
* Syntax: approx_percentile_cont_with_weight(expr, weight_expr, percentile [, centroids])
  * Description: Calculates an approximation of the specified percentile for expr and weight_expr. Optional centroids parameter controls accuracy (default: 100).
* Syntax: bit_and(expr)
  * Description: Computes the bitwise AND of all non-null input values for expr.
* Syntax: bit_or(expr)
  * Description: Computes the bitwise OR of all non-null input values for expr.
* Syntax: bit_xor(expr)
  * Description: Computes the bitwise exclusive OR of all non-null input values for expr.
* Syntax: bool_and(expr)
  * Description: Returns true if all non-null input values (expr) are true, otherwise false.
* Syntax: bool_or(expr)
  * Description: Returns true if any non-null input value (expr) is true, otherwise false.
* Syntax: count(expr)
  * Description: Returns the number of rows for expr.
* Syntax: count_distinct(expr)
  * Description: Creates an expression to represent the count(distinct) aggregate function
* Syntax: cube(exprs)
  * Description: Creates a grouping set for all combination of exprs
* Syntax: grouping_set(exprs)
  * Description: Create a grouping set.
* Syntax: max(expr)
  * Description: Finds the maximum value of expr.
* Syntax: median(expr)
  * Description: Сalculates the median of expr.
* Syntax: min(expr)
  * Description: Finds the minimum value of expr.
* Syntax: rollup(exprs)
  * Description: Creates a grouping set for rollup sets.
* Syntax: sum(expr)
  * Description: Сalculates the sum of expr.
* Syntax: sum_distinct(expr)
  * Description: Creates an expression to represent the sum(distinct) aggregate function


Aggregate Function Builder
---------------------------------------------------------------------------------

You can also use the `ExprFunctionExt` trait to more easily build Aggregate arguments `Expr`.

See `datafusion-examples/examples/expr_api.rs` for example usage.



* Syntax: first_value_udaf.call(vec![expr]).order_by(vec![expr]).build().unwrap()
  * Equivalent to: first_value(expr, Some(vec![expr]))


Subquery Expressions
---------------------------------------------------------------------



* Syntax: exists
  * Description: Creates an EXISTS subquery expression
* Syntax: in_subquery
  * Description: df1.filter(in_subquery(col("foo"), df2))? is the equivalent of the SQL WHERE foo IN <df2>
* Syntax: not_exists
  * Description: Creates a NOT EXISTS subquery expression
* Syntax: not_in_subquery
  * Description: Creates a NOT IN subquery expression
* Syntax: scalar_subquery
  * Description: Creates a scalar subquery expression


User-Defined Function Expressions
-----------------------------------------------------------------------------------------------


|Syntax     |Description                                                              |
|-----------|-------------------------------------------------------------------------|
|create_udf |Creates a new UDF with a specific signature and specific return type.    |
|create_udaf|Creates a new UDAF with a specific signature, state type and return type.|
