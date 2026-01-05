# Struct Dot Copy item path

<a href="https://docs.rs/petgraph/latest/src/petgraph/dot/mod.rs.html#51-59" class="src">Source</a>

``` rust
pub struct Dot<'a, G>where
    G: IntoEdgeReferences + IntoNodeReferences,{ /* private fields */ }
```

Expand description

`Dot` implements output to graphviz .dot format for a graph.

Formatting and options are rather simple, this is mostly intended for debugging. Exact output may change.

## <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#examples" class="doc-anchor">§</a>Examples

``` rust
use petgraph::Graph;
use petgraph::dot::{Dot, Config};

let mut graph = Graph::<_, ()>::new();
graph.add_node("A");
graph.add_node("B");
graph.add_node("C");
graph.add_node("D");
graph.extend_with_edges(&[
    (0, 1), (0, 2), (0, 3),
    (1, 2), (1, 3),
    (2, 3),
]);

println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

// In this case the output looks like this:
//
// digraph {
//     0 [label="\"A\""]
//     1 [label="\"B\""]
//     2 [label="\"C\""]
//     3 [label="\"D\""]
//     0 -> 1 [ ]
//     0 -> 2 [ ]
//     0 -> 3 [ ]
//     1 -> 2 [ ]
//     1 -> 3 [ ]
//     2 -> 3 [ ]
// }

// If you need multiple config options, just list them all in the slice.
```

## Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#impl-Dot%3C&#39;a,+G%3E" class="anchor">§</a>

### impl\<'a, G\> <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html" class="struct" title="struct petgraph::dot::Dot">Dot</a>\<'a, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a>,

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#method.new" class="fn">new</a>(graph: G) -\> Self

Create a `Dot` formatting wrapper with default configuration.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#method.with_config" class="fn">with_config</a>(graph: G, config: &'a \[<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html" class="enum" title="enum petgraph::dot::Config">Config</a>\]) -\> Self

Create a `Dot` formatting wrapper with custom configuration.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#method.with_attr_getters" class="fn">with_attr_getters</a>( graph: G, config: &'a \[<a href="https://docs.rs/petgraph/latest/petgraph/dot/enum.Config.html" class="enum" title="enum petgraph::dot::Config">Config</a>\], get_edge_attributes: &'a dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(G, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html#associatedtype.EdgeRef" class="associatedtype" title="type petgraph::visit::IntoEdgeReferences::EdgeRef">EdgeRef</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, get_node_attributes: &'a dyn <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(G, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html#associatedtype.NodeRef" class="associatedtype" title="type petgraph::visit::IntoNodeReferences::NodeRef">NodeRef</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, ) -\> Self

Create a `Dot` that uses the given functions to generate edge and node attributes.

NOTE: `Config::EdgeNoLabel` and `Config::NodeNoLabel` should be set if you intend to generate your own `label` attributes. The getter functions should return an attribute list as a String. For example, if you want to calculate a `label` for a node, then return `"label = \"your label here\""`. Each function should take as arguments the graph and that graph’s `EdgeRef` or `NodeRef`, respectively. Check the documentation for the graph type to see how it implements `IntoNodeReferences`. The [Graphviz docs](https://graphviz.org/doc/info/attrs.html) list the available attributes.

Note that some attribute values, such as labels, should be strings and must be quoted. These can be written using escapes (`"label = \"foo\""`) or [raw strings](https://doc.rust-lang.org/rust-by-example/std/str.html#literals-and-escapes) (`r#"label = "foo""#`).

For example, using a `Graph<&str, &str>` where we want the node labels to be the nodes’ weights shortened to 4 characters, and all the edges are colored blue with no labels:

``` rust
use petgraph::Graph;
use petgraph::dot::{Config, Dot};

let mut deps = Graph::<&str, &str>::new();
let pg = deps.add_node("petgraph");
let fb = deps.add_node("fixedbitset");
let qc = deps.add_node("quickcheck");
let rand = deps.add_node("rand");
let libc = deps.add_node("libc");
deps.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);

println!(
    "{:?}",
    Dot::with_attr_getters(
        &deps,
        &[Config::EdgeNoLabel, Config::NodeNoLabel],
        &|_, _| "color = blue".to_string(),
        &|_, (_, s)| format!(r#"label = "{}""#, s.chars().take(4).collect::<String>()),
    )
);
// This outputs:
// digraph {
//     0 [ label = "petg"]
//     1 [ label = "fixe"]
//     2 [ label = "quic"]
//     3 [ label = "rand"]
//     4 [ label = "libc"]
//     0 -> 1 [ color = blue]
//     0 -> 2 [ color = blue]
//     2 -> 3 [ color = blue]
//     3 -> 4 [ color = blue]
//     2 -> 4 [ color = blue]
// }
```

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#impl-Dot%3C&#39;_,+G%3E" class="anchor">§</a>

### impl\<G\> <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html" class="struct" title="struct petgraph::dot::Dot">Dot</a>\<'\_, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a>,

A low-level function allows specifying fmt functions for nodes and edges separately.

#### pub fn <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#method.graph_fmt" class="fn">graph_fmt</a>\<NF, EF\>( &self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, node_fmt: NF, edge_fmt: EF, ) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

where NF: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>, &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>, EF: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>, &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>,

## Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#impl-Debug-for-Dot%3C&#39;_,+G%3E" class="anchor">§</a>

### impl\<G\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html" class="struct" title="struct petgraph::dot::Dot">Dot</a>\<'\_, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#method.fmt-3" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#impl-Display-for-Dot%3C&#39;_,+G%3E" class="anchor">§</a>

### impl\<G\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html" class="struct" title="struct petgraph::dot::Dot">Dot</a>\<'\_, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#impl-LowerHex-for-Dot%3C&#39;_,+G%3E" class="anchor">§</a>

### impl\<G\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html" class="trait" title="trait core::fmt::LowerHex">LowerHex</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html" class="struct" title="struct petgraph::dot::Dot">Dot</a>\<'\_, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html" class="trait" title="trait core::fmt::LowerHex">LowerHex</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html" class="trait" title="trait core::fmt::LowerHex">LowerHex</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt)

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#impl-UpperHex-for-Dot%3C&#39;_,+G%3E" class="anchor">§</a>

### impl\<G\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html" class="trait" title="trait core::fmt::UpperHex">UpperHex</a> for <a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html" class="struct" title="struct petgraph::dot::Dot">Dot</a>\<'\_, G\>

where G: <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoEdgeReferences.html" class="trait" title="trait petgraph::visit::IntoEdgeReferences">IntoEdgeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.IntoNodeReferences.html" class="trait" title="trait petgraph::visit::IntoNodeReferences">IntoNodeReferences</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.NodeIndexable.html" class="trait" title="trait petgraph::visit::NodeIndexable">NodeIndexable</a> + <a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.GraphProp.html" class="trait" title="trait petgraph::visit::GraphProp">GraphProp</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.EdgeWeight" class="associatedtype" title="type petgraph::visit::Data::EdgeWeight">EdgeWeight</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html" class="trait" title="trait core::fmt::UpperHex">UpperHex</a>, G::<a href="https://docs.rs/petgraph/latest/petgraph/visit/trait.Data.html#associatedtype.NodeWeight" class="associatedtype" title="type petgraph::visit::Data::NodeWeight">NodeWeight</a>: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html" class="trait" title="trait core::fmt::UpperHex">UpperHex</a>,

<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#method.fmt-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/petgraph/latest/petgraph/dot/struct.Dot.html#blanket-implementations" class="anchor">§</a>
