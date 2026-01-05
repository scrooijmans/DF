# Struct Map Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/values.rs.html#1303-1306" class="src">Source</a>

``` rust
pub struct Map { /* private fields */ }
```

Expand description

Map is a collection of key-value pairs with a key type and a value type. It used in Literal::Map, to make it hashable, the order of key-value pairs is stored in a separate vector so that we can hash the map in a deterministic way. But it also means that the order of key-value pairs is matter for the hash value.

When converting to Arrow (e.g., for Iceberg), the map should be represented using the default field name “key_value”.

Example:

``` text
let key_value_field = Field::new(
    DEFAULT_MAP_FIELD_NAME,
    arrow_schema::DataType::Struct(vec![
        Arc::new(key_field.clone()),
        Arc::new(value_field.clone()),
    ].into()),
    false
);
'''
```

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-Map" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.new" class="fn">new</a>() -\> Self

Creates a new empty map.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the number of key-value pairs in the map.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the map contains no elements.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.insert" class="fn">insert</a>( &mut self, key: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>\>

Inserts a key-value pair into the map. If the map did not have this key present, None is returned. If the map did have this key present, the value is updated, and the old value is returned.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.get" class="fn">get</a>(&self, key: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>\>

Returns a reference to the value corresponding to the key. If the key is not present in the map, None is returned.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.has_same_content" class="fn">has_same_content</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

The order of map is matter, so this method used to compare two maps has same key-value pairs without considering the order.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-Clone-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-Debug-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-Default-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-From%3C%5B(Literal,+Option%3CLiteral%3E);+N%5D%3E-for-Map" class="anchor">§</a>

### impl\<const N: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<\[(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>); <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: \[(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>); <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">N</a>\]) -\> Self

Converts to this type from the input type.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-FromIterator%3C(Literal,+Option%3CLiteral%3E)%3E-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<(<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>)\> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<T: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>)\>\>( iter: T, ) -\> Self

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-Hash-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-IntoIterator-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.Literal.html" class="enum" title="enum iceberg::spec::Literal">Literal</a>\>)

The type of the elements being iterated over.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://doc.rust-lang.org/nightly/alloc/vec/into_iter/struct.IntoIter.html" class="struct" title="struct alloc::vec::into_iter::IntoIter">IntoIter</a>\<\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> Self::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::IntoIter">IntoIter</a>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-PartialEq-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-Eq-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#impl-StructuralPartialEq-for-Map" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html" class="struct" title="struct iceberg::spec::Map">Map</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Map.html#blanket-implementations" class="anchor">§</a>
