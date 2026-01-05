# Struct Signature Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/signature.rs.html#794" class="src">Source</a>

``` rust
pub struct Signature {
    pub type_signature: TypeSignature,
    pub volatility: Volatility,
}
```

Expand description

Provides information necessary for calling a function.

- [`TypeSignature`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature") defines the argument types that a function has implementations for.

- [`Volatility`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html "enum datafusion::logical_expr::Volatility") defines how the output of the function changes with the input.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#structfield.type_signature" class="anchor field">§</a>`type_signature: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature"><code>TypeSignature</code></a>

The data types that the function accepts. See [TypeSignature](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature") for more information.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#structfield.volatility" class="anchor field">§</a>`volatility: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility"><code>Volatility</code></a>

The volatility of the function. See [Volatility](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html "enum datafusion::logical_expr::Volatility") for more information.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#impl-Signature" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.new" class="fn">new</a>(type_signature: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Creates a new Signature from a given type signature and volatility.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.variadic" class="fn">variadic</a>( common_types: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

An arbitrary number of arguments with the same type, from those listed in `common_types`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.user_defined" class="fn">user_defined</a>(volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

User-defined coercion rules for the function.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.numeric" class="fn">numeric</a>(arg_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

A specified number of numeric arguments

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.string" class="fn">string</a>(arg_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

A specified number of numeric arguments

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.variadic_any" class="fn">variadic_any</a>(volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

An arbitrary number of arguments of any type.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.uniform" class="fn">uniform</a>( arg_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, valid_types: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

A fixed number of arguments of the same type, from those listed in `valid_types`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.exact" class="fn">exact</a>(exact_types: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>\>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Exactly matches the types in `exact_types`, in order.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.coercible" class="fn">coercible</a>( target_types: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>\>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Target coerce types in order

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.comparable" class="fn">comparable</a>(arg_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Used for function that expects comparable data types, it will try to coerced all the types into single final one.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.nullary" class="fn">nullary</a>(volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.any" class="fn">any</a>(arg_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

A specified number of arguments of any type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.one_of" class="fn">one_of</a>( type_signatures: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html" class="enum" title="enum datafusion::logical_expr::TypeSignature">TypeSignature</a>\>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Any one of a list of [TypeSignature](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignature.html "enum datafusion::logical_expr::TypeSignature")s.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.array_and_element" class="fn">array_and_element</a>(volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Specialized [Signature](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for ArrayAppend and similar functions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.element_and_array" class="fn">element_and_array</a>(volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Specialized [Signature](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for ArrayPrepend and similar functions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.arrays" class="fn">arrays</a>( n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, coercion: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/utils/enum.ListCoercion.html" class="enum" title="enum datafusion::common::utils::ListCoercion">ListCoercion</a>\>, volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Specialized [Signature](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for functions that take a fixed number of arrays.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.array_and_element_and_optional_index" class="fn">array_and_element_and_optional_index</a>(volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Specialized [Signature](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for Array functions with an optional index.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.array_and_index" class="fn">array_and_index</a>(volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Specialized [Signature](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for ArrayElement and similar functions.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.array" class="fn">array</a>(volatility: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Volatility.html" class="enum" title="enum datafusion::logical_expr::Volatility">Volatility</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Specialized [Signature](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html "struct datafusion::logical_expr::Signature") for ArrayEmpty and similar functions.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#impl-Clone-for-Signature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#impl-Debug-for-Signature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#impl-Hash-for-Signature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#impl-PartialEq-for-Signature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#impl-PartialOrd-for-Signature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#impl-Eq-for-Signature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#impl-StructuralPartialEq-for-Signature" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html" class="struct" title="struct datafusion::logical_expr::Signature">Signature</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.Signature.html#blanket-implementations" class="anchor">§</a>
