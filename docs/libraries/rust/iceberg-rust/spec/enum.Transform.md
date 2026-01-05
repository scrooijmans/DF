# Enum Transform Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/transform.rs.html#51-135" class="src">Source</a>

``` rust
pub enum Transform {
    Identity,
    Bucket(u32),
    Truncate(u32),
    Year,
    Month,
    Day,
    Hour,
    Void,
    Unknown,
}
```

Expand description

Transform is used to transform predicates to partition predicates, in addition to transforming data values.

Deriving partition predicates from column predicates on the table data is used to separate the logical queries from physical storage: the partitioning can change and the correct partition filters are always derived from column predicates.

This simplifies queries because users don’t have to supply both logical predicates and partition predicates.

All transforms must return `null` for a `null` input value.

## Variants<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variants" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Identity" class="anchor">§</a>

### Identity

Source value, unmodified

- Source type could be any type.
- Return type is the same with source type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Bucket" class="anchor">§</a>

### Bucket(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>)

Hash of value, mod `N`.

Bucket partition transforms use a 32-bit hash of the source value. The 32-bit hash implementation is the 32-bit Murmur3 hash, x86 variant, seeded with 0.

Transforms are parameterized by a number of buckets, N. The hash mod N must produce a positive value by first discarding the sign bit of the hash value. In pseudo-code, the function is:

``` text
def bucket_N(x) = (murmur3_x86_32_hash(x) & Integer.MAX_VALUE) % N
```

- Source type could be `int`, `long`, `decimal`, `date`, `time`, `timestamp`, `timestamptz`, `string`, `uuid`, `fixed`, `binary`.
- Return type is `int`.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Truncate" class="anchor">§</a>

### Truncate(<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>)

Value truncated to width `W`

For `int`:

- `v - (v % W)` remainders must be positive
- example: W=10: 1 ￫ 0, -1 ￫ -10
- note: The remainder, v % W, must be positive.

For `long`:

- `v - (v % W)` remainders must be positive
- example: W=10: 1 ￫ 0, -1 ￫ -10
- note: The remainder, v % W, must be positive.

For `decimal`:

- `scaled_W = decimal(W, scale(v)) v - (v % scaled_W)`
- example: W=50, s=2: 10.65 ￫ 10.50

For `string`:

- Substring of length L: `v.substring(0, L)`

- example: L=3: iceberg ￫ ice

- note: Strings are truncated to a valid UTF-8 string with no more than L code points.

- Source type could be `int`, `long`, `decimal`, `string`

- Return type is the same with source type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Year" class="anchor">§</a>

### Year

Extract a date or timestamp year, as years from 1970

- Source type could be `date`, `timestamp`, `timestamptz`
- Return type is `int`

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Month" class="anchor">§</a>

### Month

Extract a date or timestamp month, as months from 1970-01-01

- Source type could be `date`, `timestamp`, `timestamptz`
- Return type is `int`

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Day" class="anchor">§</a>

### Day

Extract a date or timestamp day, as days from 1970-01-01

- Source type could be `date`, `timestamp`, `timestamptz`
- Return type is `int`

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Hour" class="anchor">§</a>

### Hour

Extract a timestamp hour, as hours from 1970-01-01 00:00:00

- Source type could be `timestamp`, `timestamptz`
- Return type is `int`

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Void" class="anchor">§</a>

### Void

Always produces `null`

The void transform may be used to replace the transform in an existing partition field so that the field is effectively dropped in v1 tables.

- Source type could be any type..
- Return type is Source type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#variant.Unknown" class="anchor">§</a>

### Unknown

Used to represent some customized transform that can’t be recognized or supported now.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-Transform" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.to_human_string" class="fn">to_human_string</a>( &self, field_type: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns a human-readable String representation of a transformed value.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.result_type" class="fn">result_type</a>(&self, input_type: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Type.html" class="enum" title="enum iceberg::spec::Type">Type</a>\>

Get the return type of transform given the input type. Returns `None` if it can’t be transformed.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.preserves_order" class="fn">preserves_order</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the transform preserves the order of values.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.dedup_name" class="fn">dedup_name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Return the unique transform name to check if similar transforms for the same source field are added multiple times in partition spec builder.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.satisfies_order_of" class="fn">satisfies_order_of</a>(&self, other: &Self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether ordering by this transform’s result satisfies the ordering of another transform’s result.

For example, sorting by day(ts) will produce an ordering that is also by month(ts) or year(ts). However, sorting by day(ts) will not satisfy the order of hour(ts) or identity(ts).

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.strict_project" class="fn">strict_project</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, predicate: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>\>\>

Strictly projects a given predicate according to the transformation specified by the `Transform` instance.

This method ensures that the projected predicate is strictly aligned with the transformation logic, providing a more precise filtering mechanism for transformed data.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#example" class="doc-anchor">§</a>Example

Suppose, we have row filter `a = 10`, and a partition spec `bucket(a, 37) as bs`, if one row matches `a = 10`, then its partition value should match `bucket(10, 37) as bs`, and we project `a = 10` to `bs = bucket(10, 37)`

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.project" class="fn">project</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, predicate: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.BoundPredicate.html" class="enum" title="enum iceberg::expr::BoundPredicate">BoundPredicate</a>, ) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/expr/enum.Predicate.html" class="enum" title="enum iceberg::expr::Predicate">Predicate</a>\>\>

Projects a given predicate according to the transformation specified by the `Transform` instance.

This allows predicates to be effectively applied to data that has undergone transformation, enabling efficient querying and filtering based on the original, untransformed data.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#example-1" class="doc-anchor">§</a>Example

Suppose, we have row filter `a = 10`, and a partition spec `bucket(a, 37) as bs`, if one row matches `a = 10`, then its partition value should match `bucket(10, 37) as bs`, and we project `a = 10` to `bs = bucket(10, 37)`

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-Clone-for-Transform" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-Debug-for-Transform" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-Deserialize%3C&#39;de%3E-for-Transform" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<D\>(deserializer: D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-Display-for-Transform" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-FromStr-for-Transform" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html" class="trait" title="trait core::str::traits::FromStr">FromStr</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#associatedtype.Err" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err" class="associatedtype">Err</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/struct.Error.html" class="struct" title="struct iceberg::Error">Error</a>

The associated error which can be returned from parsing.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.from_str" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str" class="fn">from_str</a>(s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<Self\>

Parses a string `s` to return a value of this type. [Read more](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#tymethod.from_str)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-PartialEq-for-Transform" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-Serialize-for-Transform" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<S\>(&self, serializer: S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-Copy-for-Transform" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-Eq-for-Transform" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#impl-StructuralPartialEq-for-Transform" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html" class="enum" title="enum iceberg::spec::Transform">Transform</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Transform.html#blanket-implementations" class="anchor">§</a>
