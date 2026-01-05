# Enum TypeSignature Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/signature.rs.html#147" class="src">Source</a>

``` rust
pub enum TypeSignature {
Show 13 variants    Variadic(Vec<DataType>),
    UserDefined,
    VariadicAny,
    Uniform(usize, Vec<DataType>),
    Exact(Vec<DataType>),
    Coercible(Vec<Coercion>),
    Comparable(usize),
    Any(usize),
    OneOf(Vec<TypeSignature>),
    ArraySignature(ArrayFunctionSignature),
    Numeric(usize),
    String(usize),
    Nullary,
}
```

Expand description

The types of arguments for which a function has implementations.

[`TypeSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature") **DOES NOT** define the types that a user query could call the function with. DataFusion will automatically coerce (cast) argument types to one of the supported function signatures, if possible.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#overview" class="doc-anchor">§</a>Overview

Functions typically provide implementations for a small number of different argument [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType")s, rather than all possible combinations. If a user calls a function with arguments that do not match any of the declared types, DataFusion will attempt to automatically coerce (add casts to) function arguments so they match the [`TypeSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature"). See the [`type_coercion`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/type_coercion/index.html "mod datafusion::logical_expr_common::type_coercion") module for more details

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#example-numeric-functions" class="doc-anchor">§</a>Example: Numeric Functions

For example, a function like `cos` may only provide an implementation for [`DataType::Float64`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Float64 "variant datafusion::common::arrow::datatypes::DataType::Float64"). When users call `cos` with a different argument type, such as `cos(int_column)`, and type coercion automatically adds a cast such as `cos(CAST int_column AS DOUBLE)` during planning.

### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#example-strings" class="doc-anchor">§</a>Example: Strings

There are several different string types in Arrow, such as [`DataType::Utf8`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Utf8 "variant datafusion::common::arrow::datatypes::DataType::Utf8"), [`DataType::LargeUtf8`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.LargeUtf8 "variant datafusion::common::arrow::datatypes::DataType::LargeUtf8"), and [`DataType::Utf8View`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Utf8View "variant datafusion::common::arrow::datatypes::DataType::Utf8View").

Some functions may have specialized implementations for these types, while others may be able to handle only one of them. For example, a function that only works with [`DataType::Utf8View`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Utf8View "variant datafusion::common::arrow::datatypes::DataType::Utf8View") would have the following signature:

``` rust
 // Declares the function must be invoked with a single argument of type `Utf8View`.
 // if a user calls the function with `Utf8` or `LargeUtf8`, DataFusion will
 // automatically add a cast to `Utf8View` during planning.
 let type_signature = TypeSignature::Exact(vec![DataType::Utf8View]);
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#example-timestamps" class="doc-anchor">§</a>Example: Timestamps

Types to match are represented using Arrow’s [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType"). [`DataType::Timestamp`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Timestamp "variant datafusion::common::arrow::datatypes::DataType::Timestamp") has an optional variable timezone specification. To specify a function can handle a timestamp with *ANY* timezone, use the [`TIMEZONE_WILDCARD`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/constant.TIMEZONE_WILDCARD.html "constant datafusion::logical_expr::TIMEZONE_WILDCARD"). For example:

``` rust
let type_signature = TypeSignature::Exact(vec![
  // A nanosecond precision timestamp with ANY timezone
  // matches  Timestamp(Nanosecond, Some("+0:00"))
  // matches  Timestamp(Nanosecond, Some("+5:00"))
  // does not match  Timestamp(Nanosecond, None)
  DataType::Timestamp(TimeUnit::Nanosecond, Some(TIMEZONE_WILDCARD.into())),
]);
```

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Variadic" class="anchor">§</a>

### Variadic(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>)

One or more arguments of a common type out of a list of valid types.

For functions that take no arguments (e.g. `random()` see [`TypeSignature::Nullary`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Nullary "variant datafusion::logical_expr::TypeSignature::Nullary")).

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#examples" class="doc-anchor">§</a>Examples

A function such as `concat` is `Variadic(vec![DataType::Utf8, DataType::LargeUtf8])`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.UserDefined" class="anchor">§</a>

### UserDefined

The acceptable signature and coercions rules are special for this function.

If this signature is specified, DataFusion will call [`ScalarUDFImpl::coerce_types`](https://docs.rs/datafusion/latest/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.coerce_types) to prepare argument types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.VariadicAny" class="anchor">§</a>

### VariadicAny

One or more arguments with arbitrary types

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Uniform" class="anchor">§</a>

### Uniform(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>)

One or more arguments of an arbitrary but equal type out of a list of valid types.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#examples-1" class="doc-anchor">§</a>Examples

1.  A function of one argument of f64 is `Uniform(1, vec![DataType::Float64])`
2.  A function of one argument of f64 or f32 is `Uniform(1, vec![DataType::Float32, DataType::Float64])`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Exact" class="anchor">§</a>

### Exact(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>)

One or more arguments with exactly the specified types in order.

For functions that take no arguments (e.g. `random()`) use [`TypeSignature::Nullary`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Nullary "variant datafusion::logical_expr::TypeSignature::Nullary").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Coercible" class="anchor">§</a>

### Coercible(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>\>)

One or more arguments belonging to the [`TypeSignatureClass`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html "enum datafusion::logical_expr::TypeSignatureClass"), in order.

[`Coercion`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html "enum datafusion::logical_expr::Coercion") contains not only the desired type but also the allowed casts. For example, if you expect a function has string type, but you also allow it to be casted from binary type.

For functions that take no arguments (e.g. `random()`) see [`TypeSignature::Nullary`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Nullary "variant datafusion::logical_expr::TypeSignature::Nullary").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Comparable" class="anchor">§</a>

### Comparable(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

One or more arguments coercible to a single, comparable type.

Each argument will be coerced to a single type using the coercion rules described in [`comparison_coercion_numeric`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/type_coercion/binary/fn.comparison_coercion_numeric.html "fn datafusion::logical_expr::type_coercion::binary::comparison_coercion_numeric").

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#examples-2" class="doc-anchor">§</a>Examples

If the `nullif(1, 2)` function is called with `i32` and `i64` arguments the types will both be coerced to `i64` before the function is invoked.

If the `nullif('1', 2)` function is called with `Utf8` and `i64` arguments the types will both be coerced to `Utf8` before the function is invoked.

Note:

- For functions that take no arguments (e.g. `random()` see [`TypeSignature::Nullary`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Nullary "variant datafusion::logical_expr::TypeSignature::Nullary")).
- If all arguments have type [`DataType::Null`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#variant.Null "variant datafusion::common::arrow::datatypes::DataType::Null"), they are coerced to `Utf8`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Any" class="anchor">§</a>

### Any(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

One or more arguments of arbitrary types.

For functions that take no arguments (e.g. `random()`) use [`TypeSignature::Nullary`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Nullary "variant datafusion::logical_expr::TypeSignature::Nullary").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.OneOf" class="anchor">§</a>

### OneOf(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>\>)

Matches exactly one of a list of [`TypeSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature")s.

Coercion is attempted to match the signatures in order, and stops after the first success, if any.

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#examples-3" class="doc-anchor">§</a>Examples

Since `make_array` takes 0 or more arguments with arbitrary types, its `TypeSignature` is `OneOf(vec![Any(0), VariadicAny])`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.ArraySignature" class="anchor">§</a>

### ArraySignature(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ArrayFunctionSignature.html" class="enum" title="enum datafusion::logical_expr::ArrayFunctionSignature">ArrayFunctionSignature</a>)

A function that has an [`ArrayFunctionSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.ArrayFunctionSignature.html "enum datafusion::logical_expr::ArrayFunctionSignature")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Numeric" class="anchor">§</a>

### Numeric(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

One or more arguments of numeric types.

See [`NativeType::is_numeric`](https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html#method.is_numeric "method datafusion::common::types::NativeType::is_numeric") to know which type is considered numeric

For functions that take no arguments (e.g. `random()`) use [`TypeSignature::Nullary`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Nullary "variant datafusion::logical_expr::TypeSignature::Nullary").

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.String" class="anchor">§</a>

### String(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

One or arguments of all the same string types.

The precedence of type from high to low is Utf8View, LargeUtf8 and Utf8. Null is considered as `Utf8` by default Dictionary with string value type is also handled.

For example, if a function is called with (utf8, large_utf8), all arguments will be coerced to `LargeUtf8`

For functions that take no arguments (e.g. `random()` use [`TypeSignature::Nullary`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Nullary "variant datafusion::logical_expr::TypeSignature::Nullary")).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#variant.Nullary" class="anchor">§</a>

### Nullary

No arguments

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-TypeSignature" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.is_one_of" class="fn">is_one_of</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-TypeSignature-1" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.to_string_repr" class="fn">to_string_repr</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.join_types" class="fn">join_types</a>\<T\>(types: &<a href="https://doc.rust-lang.org/nightly/std/primitive.slice.html" class="primitive">[T]</a>, delimiter: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>,

Helper function to join types with specified delimiter.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.supports_zero_argument" class="fn">supports_zero_argument</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check whether 0 input argument is valid for given `TypeSignature`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.used_to_support_zero_arguments" class="fn">used_to_support_zero_arguments</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the signature currently supports or used to supported 0 input arguments in a previous version of DataFusion.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.get_possible_types" class="fn">get_possible_types</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>\>

👎Deprecated since 46.0.0: See get_example_types instead

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.get_example_types" class="fn">get_example_types</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>\>

Return example acceptable types for this `TypeSignature`’

Returns a `Vec<DataType>` for each argument to the function

This is used for `information_schema` and can be used to generate documentation or error messages.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-Clone-for-TypeSignature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-Debug-for-TypeSignature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-Hash-for-TypeSignature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-PartialEq-for-TypeSignature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-PartialOrd-for-TypeSignature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-Eq-for-TypeSignature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#impl-StructuralPartialEq-for-TypeSignature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html#blanket-implementations" class="anchor">§</a>
