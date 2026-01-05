# Struct ViewRepresentations Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/view_version.rs.html#163" class="src">Source</a>

``` rust
pub struct ViewRepresentations(/* private fields */);
```

Expand description

A list of view representations.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-ViewRepresentations" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the number of representations

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if there are no representations

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.iter" class="fn">iter</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html" class="trait" title="trait core::iter::traits::exact_size::ExactSizeIterator">ExactSizeIterator</a>\<Item = &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewRepresentation.html" class="enum" title="enum iceberg::spec::ViewRepresentation">ViewRepresentation</a>\>

Get an iterator over the representations

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-Clone-for-ViewRepresentations" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-Debug-for-ViewRepresentations" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-Deserialize%3C&#39;de%3E-for-ViewRepresentations" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-IntoIterator-for-ViewRepresentations" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.ViewRepresentation.html" class="enum" title="enum iceberg::spec::ViewRepresentation">ViewRepresentation</a>

The type of the elements being iterated over.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://doc.rust-lang.org/nightly/alloc/vec/into_iter/struct.IntoIter.html" class="struct" title="struct alloc::vec::into_iter::IntoIter">IntoIter</a>\<\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-PartialEq-for-ViewRepresentations" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-Serialize-for-ViewRepresentations" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-Eq-for-ViewRepresentations" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#impl-StructuralPartialEq-for-ViewRepresentations" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html" class="struct" title="struct iceberg::spec::ViewRepresentations">ViewRepresentations</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ViewRepresentations.html#blanket-implementations" class="anchor">§</a>
