# Enum Coercion Copy item path

<a href="https://docs.rs/datafusion-expr-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_expr_common/signature.rs.html#632" class="src">Source</a>

``` rust
pub enum Coercion {
    Exact {
        desired_type: TypeSignatureClass,
    },
    Implicit {
        desired_type: TypeSignatureClass,
        implicit_coercion: ImplicitCoercion,
    },
}
```

Expand description

Represents type coercion rules for function arguments, specifying both the desired type and optional implicit coercion rules for source types.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#examples" class="doc-anchor">§</a>Examples

``` rust
use datafusion_expr_common::signature::{Coercion, TypeSignatureClass};
use datafusion_common::types::{NativeType, logical_binary, logical_string};

// Exact coercion that only accepts timestamp types
let exact = Coercion::new_exact(TypeSignatureClass::Timestamp);

// Implicit coercion that accepts string types but can coerce from binary types
let implicit = Coercion::new_implicit(
    TypeSignatureClass::Native(logical_string()),
    vec![TypeSignatureClass::Native(logical_binary())],
    NativeType::String
);
```

There are two variants:

- `Exact` - Only accepts arguments that exactly match the desired type
- `Implicit` - Accepts the desired type and can coerce from specified source types

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#variant.Exact" class="anchor">§</a>

### Exact

Coercion that only accepts arguments exactly matching the desired type.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#variant.Exact.field.desired_type" class="anchor field">§</a>`desired_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass"><code>TypeSignatureClass</code></a>

The required type for the argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#variant.Implicit" class="anchor">§</a>

### Implicit

Coercion that accepts the desired type and can implicitly coerce from other types.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#variant.Implicit.field.desired_type" class="anchor field">§</a>`desired_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass"><code>TypeSignatureClass</code></a>

The primary desired type for the argument

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#variant.Implicit.field.implicit_coercion" class="anchor field">§</a>`implicit_coercion: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/struct.ImplicitCoercion.html" class="struct" title="struct datafusion::logical_expr_common::signature::ImplicitCoercion"><code>ImplicitCoercion</code></a>

Rules for implicit coercion from other types

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#impl-Coercion" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.new_exact" class="fn">new_exact</a>(desired_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.new_implicit" class="fn">new_implicit</a>( desired_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>, allowed_source_types: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>\>, default_casted_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

Create a new coercion with implicit coercion rules.

`allowed_source_types` defines the possible types that can be coerced to `desired_type`. `default_casted_type` is the default type to be used for coercion if we cast from other types via `allowed_source_types`.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.allowed_source_types" class="fn">allowed_source_types</a>(&self) -\> &\[<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>\]

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.default_casted_type" class="fn">default_casted_type</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/types/enum.NativeType.html" class="enum" title="enum datafusion::common::types::NativeType">NativeType</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.desired_type" class="fn">desired_type</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.TypeSignatureClass.html" class="enum" title="enum datafusion::logical_expr::TypeSignatureClass">TypeSignatureClass</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.implicit_coercion" class="fn">implicit_coercion</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr_common/signature/struct.ImplicitCoercion.html" class="struct" title="struct datafusion::logical_expr_common::signature::ImplicitCoercion">ImplicitCoercion</a>\>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#impl-Clone-for-Coercion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#impl-Debug-for-Coercion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#impl-Display-for-Coercion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#impl-Hash-for-Coercion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#impl-PartialEq-for-Coercion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#impl-PartialOrd-for-Coercion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#impl-Eq-for-Coercion" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html" class="enum" title="enum datafusion::logical_expr::Coercion">Coercion</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/enum.Coercion.html#blanket-implementations" class="anchor">§</a>
