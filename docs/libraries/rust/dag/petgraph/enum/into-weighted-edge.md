Description: Convert an element like `(i, j)` or `(i, j, w)` into a triple of source, target, edge weight.

Title: IntoWeightedEdge in petgraph - Rust

Docs.rs

- petgraph-0.8.3

- petgraph 0.8.3
- Permalink
- Docs.rs crate page
- MIT OR Apache-2.0

- Links
- Repository
- crates.io
- Source

- Owners
- bluss
- github:petgraph:release-team
- ABorgna

- Dependencies
- - dot-parser ^0.5.1 _normal_ _optional_
- dot-parser-macros ^0.5.1 _normal_ _optional_
- fixedbitset ^0.5.7 _normal_
- hashbrown ^0.15.0 _normal_
- indexmap ^2.5.0 _normal_
- quickcheck ^0.8 _normal_ _optional_
- rayon ^1.5.3 _normal_ _optional_
- serde ^1.0 _normal_ _optional_
- serde_derive ^1.0 _normal_ _optional_
- ahash ^0.7.2 _dev_
- bincode ^1.3.3 _dev_
- defmac ^0.2.1 _dev_
- fxhash ^0.2.1 _dev_
- itertools ^0.12.1 _dev_
- odds ^0.4.0 _dev_
- rand ^0.5.5 _dev_

- Versions

- **79.17%** of the crate is documented

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

## IntoWeightedEdge

petgraph

# Trait IntoWeightedEdge Copy item path

Source

```
pub trait IntoWeightedEdge<E> {
type NodeId;

// Required method
fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E);
}
```

Expand description

Convert an element like `(i, j)` or `(i, j, w)` into a triple of source, target, edge weight.

For `Graph::from_edges` and `GraphMap::from_edges`.

## Required Associated Types§

Source

#### type NodeId

## Required Methods§

Source

#### fn into_weighted_edge(self) -> (Self::NodeId, Self::NodeId, E)

## Implementations on Foreign Types§

Source§

### impl<Ix, E> IntoWeightedEdge<E> for &(Ix, Ix)

where Ix: Copy, E: Default,

Source§

#### type NodeId = Ix

Source§

#### fn into_weighted_edge(self) -> (Ix, Ix, E)

Source§

### impl<Ix, E> IntoWeightedEdge<E> for &(Ix, Ix, E)

where Ix: Copy, E: Clone,

Source§

#### type NodeId = Ix

Source§

#### fn into_weighted_edge(self) -> (Ix, Ix, E)

Source§

### impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix, &E)

where E: Clone,

Source§

#### type NodeId = Ix

Source§

#### fn into_weighted_edge(self) -> (Ix, Ix, E)

Source§

### impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix)

where E: Default,

Source§

#### type NodeId = Ix

Source§

#### fn into_weighted_edge(self) -> (Ix, Ix, E)

Source§

### impl<Ix, E> IntoWeightedEdge<E> for (Ix, Ix, E)

Source§

#### type NodeId = Ix

Source§

#### fn into_weighted_edge(self) -> (Ix, Ix, E)

## Implementors§
