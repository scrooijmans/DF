# Type System - Substrait: Cross-Language Serialization for Relational Algebra
Substrait tries to cover the most common types used in data manipulation. Types beyond this common core may be represented using [simple extensions](about:blank/extensions/#simple-extensions).

Substrait types fundamentally consist of four components:



* Component: Class
  * Condition: Always
  * Examples: i8, string, STRUCT, extensions
  * Description: Together with the parameter pack, describes the set of non-null values supported by the type. Subdivided into simple and compound type classes.
* Component: Nullability
  * Condition: Always
  * Examples: Either NULLABLE (? suffix) or REQUIRED (no suffix)
  * Description: Describes whether values of this type can be null. Note that null is considered to be a special value of a nullable type, rather than the only value of a special null type.
* Component: Variation
  * Condition: Always
  * Examples: No suffix or explicitly [0] (system-preferred), or an extension
  * Description: Allows different variations of the same type class to exist in a system at a time, usually distinguished by in-memory format.
* Component: Parameters
  * Condition: Compound types only
  * Examples: <10, 2> (for DECIMAL), <i32, string> (for STRUCT)
  * Description: Some combination of zero or more data types or integers. The expected set of parameters and the significance of each parameter depends on the type class.


Refer to [Type Parsing](../type_parsing/) for a description of the syntax used to describe types.

Note

Substrait employs a strict type system without any coercion rules. All changes in types must be made explicit via [cast expressions](../../expressions/specialized_record_expressions/).