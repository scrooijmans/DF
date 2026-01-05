# Enum Directed Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/lib.rs.html#581" class="src">Source</a>

``` rust
pub enum Directed {}
```

Expand description

Marker type for a directed graph.

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#impl-Clone-for-Directed" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#impl-Debug-for-Directed" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#impl-Deserialize%3C&#39;de%3E-for-Directed" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html" class="trait" title="trait serde_core::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>(\_\_deserializer: \_\_D) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, \_\_D::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html" class="trait" title="trait serde_core::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#impl-EdgeType-for-Directed" class="anchor">§</a>

### impl <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html" class="trait" title="trait petgraph::EdgeType">EdgeType</a> for <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#method.is_directed" class="anchor">§</a>

#### fn <a href="https://docs.rs/petgraph/latest/petgraph/trait.EdgeType.html#tymethod.is_directed" class="fn">is_directed</a>() -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#impl-Serialize-for-Directed" class="anchor">§</a>

### impl <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html" class="trait" title="trait serde_core::ser::Serialize">Serialize</a> for <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\_\_S::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde_core::ser::Serializer::Ok">Ok</a>, \_\_S::<a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde_core::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html" class="trait" title="trait serde_core::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#impl-Copy-for-Directed" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html" class="enum" title="enum petgraph::Directed">Directed</a>

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/enum.Directed.html#blanket-implementations" class="anchor">§</a>
