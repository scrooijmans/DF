# Struct SortOrder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/sort.rs.html#105-112" class="src">Source</a>

``` rust
pub struct SortOrder {
    pub order_id: i64,
    pub fields: Vec<SortField>,
}
```

Expand description

A sort order is defined by a sort order id and a list of sort fields. The order of the sort fields within the list defines the order in which the sort is applied to the data.

## Fields<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#fields" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#structfield.order_id" class="anchor field">§</a>`order_id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive"><code>i64</code></a>

Identifier for SortOrder, order_id `0` is no sort order.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#structfield.fields" class="anchor field">§</a>`fields: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortField.html" class="struct" title="struct iceberg::spec::SortField"><code>SortField</code></a>`>`

Details of the sort

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-SortOrder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.builder" class="fn">builder</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrderBuilder.html" class="struct" title="struct iceberg::spec::SortOrderBuilder">SortOrderBuilder</a>

Create sort order builder

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.unsorted_order" class="fn">unsorted_order</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

Create an unbound unsorted order

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.is_unsorted" class="fn">is_unsorted</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the sort order is unsorted.

A [`SortOrder`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html "struct iceberg::spec::SortOrder") is unsorted if it has no sort fields.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.with_order_id" class="fn">with_order_id</a>(self, order_id: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

Set the order id for the sort order

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-Clone-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-Debug-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-Default-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-Deserialize%3C&#39;de%3E-for-SortOrder" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-PartialEq-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-Serialize-for-SortOrder" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-Eq-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#impl-StructuralPartialEq-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html" class="struct" title="struct iceberg::spec::SortOrder">SortOrder</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.SortOrder.html#blanket-implementations" class="anchor">§</a>
