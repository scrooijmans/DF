# Struct Attributes Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/attributes.rs.html#110" class="src">Source</a>

``` rust
pub struct Attributes(/* private fields */);
```

Expand description

Additional attributes of an object

Attributes can be specified in [PutOptions](https://docs.rs/object_store/latest/object_store/struct.PutOptions.html "struct object_store::PutOptions") and retrieved from APIs returning [GetResult](https://docs.rs/object_store/latest/object_store/struct.GetResult.html "struct object_store::GetResult").

Unlike [`ObjectMeta`](https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html "struct object_store::ObjectMeta"), [`Attributes`](https://docs.rs/object_store/latest/object_store/struct.Attributes.html "struct object_store::Attributes") are not returned by listing APIs

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-Attributes" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.new" class="fn">new</a>() -\> Self

Create a new empty [`Attributes`](https://docs.rs/object_store/latest/object_store/struct.Attributes.html "struct object_store::Attributes")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new [`Attributes`](https://docs.rs/object_store/latest/object_store/struct.Attributes.html "struct object_store::Attributes") with space for `capacity` [`Attribute`](https://docs.rs/object_store/latest/object_store/enum.Attribute.html "enum object_store::Attribute")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.insert" class="fn">insert</a>( &mut self, key: <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>, value: <a href="https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html" class="struct" title="struct object_store::AttributeValue">AttributeValue</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html" class="struct" title="struct object_store::AttributeValue">AttributeValue</a>\>

Insert a new [`Attribute`](https://docs.rs/object_store/latest/object_store/enum.Attribute.html "enum object_store::Attribute"), [`AttributeValue`](https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html "struct object_store::AttributeValue") pair

Returns the previous value for `key` if any

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.get" class="fn">get</a>(&self, key: &<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html" class="struct" title="struct object_store::AttributeValue">AttributeValue</a>\>

Returns the [`AttributeValue`](https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html "struct object_store::AttributeValue") for `key` if any

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.remove" class="fn">remove</a>(&mut self, key: &<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html" class="struct" title="struct object_store::AttributeValue">AttributeValue</a>\>

Removes the [`AttributeValue`](https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html "struct object_store::AttributeValue") for `key` if any

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.AttributesIter.html" class="struct" title="struct object_store::AttributesIter">AttributesIter</a>\<'\_\> <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#" class="tooltip" data-notable-ty="AttributesIter&lt;&#39;_&gt;">ⓘ</a>

Returns an [`AttributesIter`](https://docs.rs/object_store/latest/object_store/struct.AttributesIter.html "struct object_store::AttributesIter") over this

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of [`Attribute`](https://docs.rs/object_store/latest/object_store/enum.Attribute.html "enum object_store::Attribute") in this collection

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this contains no [`Attribute`](https://docs.rs/object_store/latest/object_store/enum.Attribute.html "enum object_store::Attribute")

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-Clone-for-Attributes" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-Debug-for-Attributes" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-Default-for-Attributes" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-From%3CAttributes%3E-for-PutMultipartOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutMultipartOptions.html" class="struct" title="struct object_store::PutMultipartOptions">PutMultipartOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.from-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(attributes: <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-From%3CAttributes%3E-for-PutOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutOptions.html" class="struct" title="struct object_store::PutOptions">PutOptions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(attributes: <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-FromIterator%3C(K,+V)%3E-for-Attributes" class="anchor">§</a>

### impl\<K, V\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

where K: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>\>, V: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html" class="struct" title="struct object_store::AttributeValue">AttributeValue</a>\>,

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(K, V)</a>\>\>(iter: T) -\> Self

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-IntoIterator-for-%26Attributes" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (&'a <a href="https://docs.rs/object_store/latest/object_store/enum.Attribute.html" class="enum" title="enum object_store::Attribute">Attribute</a>, &'a <a href="https://docs.rs/object_store/latest/object_store/struct.AttributeValue.html" class="struct" title="struct object_store::AttributeValue">AttributeValue</a>)

The type of the elements being iterated over.

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/object_store/latest/object_store/struct.AttributesIter.html" class="struct" title="struct object_store::AttributesIter">AttributesIter</a>\<'a\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-PartialEq-for-Attributes" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-Eq-for-Attributes" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#impl-StructuralPartialEq-for-Attributes" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes">Attributes</a>

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html#blanket-implementations" class="anchor">§</a>
