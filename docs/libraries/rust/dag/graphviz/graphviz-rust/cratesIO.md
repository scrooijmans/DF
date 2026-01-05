Description: The library provides functionality for interacting with the `graphviz` DOT language.

Title: graphviz_rust - Rust

Docs.rs

- graphviz-rust-0.9.6

- graphviz-rust 0.9.6
- Docs.rs crate page

- Links
- Homepage
- Repository
- crates.io
- Source

- Owners
- besok

- Dependencies
- - dot-generator ^0.2.0 _normal_
- dot-structures ^0.1.2 _normal_
- into-attr ^0.1.1 _normal_
- into-attr-derive ^0.2.1 _normal_
- pest ^2.0 _normal_
- pest_derive ^2.0 _normal_
- rand ^0.9 _normal_
- tempfile ^3.13.0 _normal_

- Versions

- **2.26%** of the crate is documented

- Platform
- i686-pc-windows-msvc
- i686-unknown-linux-gnu
- x86_64-apple-darwin
- x86_64-pc-windows-msvc
- x86_64-unknown-linux-gnu
- Feature flags

- docs.rs
- About docs.rs
- Badges
- Builds
- Metadata
- Shorthand URLs
- Download
- Rustdoc JSON
- Build queue
- Privacy policy

- Rust
- Rust website
- The Book
- Standard Library API Reference
- Rust by Example
- The Cargo Guide
- Clippy Documentation

## Crate graphviz_rust

# Crate graphviz_rust Copy item path

Source

Expand description

The library provides functionality for interacting with the `graphviz` DOT language.

## §Description:

This library contains 4 primary functions:

- parse: parses a string in the dot `notation` into a Graph.
- print: serializes a Graph into a string given a DotPrinter.
- exec: executes the `dot` command line executable given a Graph.
- exec_dot: executes the `dot` command line executable given a string in the dot `notation`.

## §Examples:

```
use dot_generator::*;
use dot_structures::*;
use graphviz_rust::{
attributes::*,
cmd::{CommandArg, Format},
exec, exec_dot, parse,
printer::{DotPrinter, PrinterContext},
};

let g: Graph = parse(
r#"
strict digraph t {
aa[color=green]
subgraph v {
aa[shape=square]
subgraph vv{a2 -> b2}
aaa[color=red]
aaa -> bbb
}
aa -> be -> subgraph v { d -> aaa}
aa -> aaa -> v
}
"#,
)
.unwrap();

assert_eq!(
g,
graph!(strict di id!("t");
node!("aa";attr!("color","green")),
subgraph!("v";
node!("aa"; attr!("shape","square")),
subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
node!("aaa";attr!("color","red")),
edge!(node_id!("aaa") => node_id!("bbb"))
),
edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
)
);

let mut g = graph!(strict di id!("id"));
assert_eq!(
"strict digraph id {\n\n}".to_string(),
g.print(&mut PrinterContext::default())
);

fn output_test() {
let mut g = graph!(id!("id");
node!("nod"),
subgraph!("sb";
edge!(node_id!("a") => subgraph!(;
node!("n";
NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
))
),
edge!(node_id!("a1") => node_id!(esc "a2"))
);
let graph_svg = exec(
g,
&mut PrinterContext::default(),
vec![Format::Svg.into()],
)
.unwrap();
}
fn output_exec_from_test() {
let mut g = graph!(id!("id");
node!("nod"),
subgraph!("sb";
edge!(node_id!("a") => subgraph!(;
node!("n";
NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
))
),
edge!(node_id!("a1") => node_id!(esc "a2"))
);
let dot = g.print(&mut PrinterContext::default());
println!("{}", dot);
let format = Format::Svg;

let graph_svg = exec_dot(dot.clone(), vec![format.into()]).unwrap();

let graph_svg = exec_dot(dot, vec![format.clone().into()]).unwrap();
}
```

## Re-exports§

`pub extern crate dot_generator;`

`pub extern crate dot_structures;`

`pub extern crate into_attr;`

`pub extern crate into_attr_derive;`

## Modules§

attributes

graphviz `attributes`

cmd

Utilities for executing the `dot` command line executable.

printer

Serialize a Graph into a string according to the `graphviz` DOT language.

## Macros§

as_item

generate_attr

## Functions§

exec

Executes the `dot` command line executable using the given Graph, PrinterContext and command line arguments.

exec_dot

Executes the `dot` command line executable using the given string dot notation, PrinterContext and command line arguments.

parse

Parses a string into a Graph.

print

Serializes a Graph into a string given a DotPrinter.
