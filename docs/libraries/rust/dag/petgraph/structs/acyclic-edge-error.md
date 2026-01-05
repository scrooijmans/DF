Description: An error that can occur during edge addition for acyclic graphs.

Title: AcyclicEdgeError in petgraph::acyclic - Rust

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

## AcyclicEdgeError

petgraph::acyclic

# Enum AcyclicEdgeError Copy item path

Source

```
pub enum AcyclicEdgeError<N> {
Cycle(Cycle<N>),
SelfLoop,
InvalidEdge,
}
```

Expand description

An error that can occur during edge addition for acyclic graphs.

## Variants§

§

### Cycle(Cycle<N>)

The edge would create a cycle.

§

### SelfLoop

The edge would create a self-loop.

§

### InvalidEdge

Could not successfully add the edge to the underlying graph.

## Trait Implementations§

Source§

### impl<N: Clone\> Clone for AcyclicEdgeError<N>

Source§

#### fn clone(&self) -> AcyclicEdgeError<N>

Returns a duplicate of the value. Read more

1.0.0 · Source§

#### fn clone_from(&mut self, source: &Self)

Performs copy-assignment from `source`. Read more

Source§

### impl<N: Debug\> Debug for AcyclicEdgeError<N>

Source§

#### fn fmt(&self, f: &mut Formatter<'\_>) -> Result

Formats the value using the given formatter. Read more

Source§

### impl<N> From<Cycle<N>> for AcyclicEdgeError<N>

Source§

#### fn from(cycle: Cycle<N>) -> Self

Converts to this type from the input type.

Source§

### impl<N: PartialEq\> PartialEq for AcyclicEdgeError<N>

Source§

#### fn eq(&self, other: &AcyclicEdgeError<N>) -> bool

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · Source§

#### fn ne(&self, other: &Rhs) -> bool

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

Source§

### impl<N> StructuralPartialEq for AcyclicEdgeError<N>

## Auto Trait Implementations§

§

### impl<N> Freeze for AcyclicEdgeError<N>

where N: Freeze,

§

### impl<N> RefUnwindSafe for AcyclicEdgeError<N>

where N: RefUnwindSafe,

§

### impl<N> Send for AcyclicEdgeError<N>

where N: Send,

§

### impl<N> Sync for AcyclicEdgeError<N>

where N: Sync,

§

### impl<N> Unpin for AcyclicEdgeError<N>

where N: Unpin,

§

### impl<N> UnwindSafe for AcyclicEdgeError<N>

where N: UnwindSafe,

## Blanket Implementations§

Source§

### impl<T> Any for T

where T: 'static + ?Sized,

Source§

#### fn type_id(&self) -> TypeId

Gets the `TypeId` of `self`. Read more

Source§

### impl<T> Borrow<T> for T

where T: ?Sized,

Source§

#### fn borrow(&self) -> &T

Immutably borrows from an owned value. Read more

Source§

### impl<T> BorrowMut<T> for T

where T: ?Sized,

Source§

#### fn borrow_mut(&mut self) -> &mut T

Mutably borrows from an owned value. Read more

Source§

### impl<T> CloneToUninit for T

where T: Clone,

Source§

#### unsafe fn clone_to_uninit(&self, dest: \*mut u8)

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. Read more

Source§

### impl<T> From<T> for T

Source§

#### fn from(t: T) -> T

Returns the argument unchanged.

Source§

### impl<T, U> Into<U> for T

where U: From<T>,

Source§

#### fn into(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `From<T> for U` chooses to do.

Source§

### impl<T> IntoEither for T

Source§

#### fn into_either(self, into_left: bool) -> Either<Self, Self>

Converts `self` into a `Left` variant of `Either<Self, Self>` if `into_left` is `true`. Converts `self` into a `Right` variant of `Either<Self, Self>` otherwise. Read more

Source§

#### fn into_either_with<F>(self, into_left: F) -> Either<Self, Self>

where F: FnOnce(&Self) -> bool,

Converts `self` into a `Left` variant of `Either<Self, Self>` if `into_left(&self)` returns `true`. Converts `self` into a `Right` variant of `Either<Self, Self>` otherwise. Read more

Source§

### impl<T> Pointable for T

Source§

#### const ALIGN: usize

The alignment of pointer.

Source§

#### type Init = T

The type for initializers.

Source§

#### unsafe fn init(init: <T as Pointable\>::Init) -> usize

Initializes a with the given initializer. Read more

Source§

#### unsafe fn deref<'a>(ptr: usize) -> &'a T

Dereferences the given pointer. Read more

Source§

#### unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T

Mutably dereferences the given pointer. Read more

Source§

#### unsafe fn drop(ptr: usize)

Drops the object pointed to by the given pointer. Read more

Source§

### impl<T> ToOwned for T

where T: Clone,

Source§

#### type Owned = T

The resulting type after obtaining ownership.

Source§

#### fn to_owned(&self) -> T

Creates owned data from borrowed data, usually by cloning. Read more

Source§

#### fn clone_into(&self, target: &mut T)

Uses borrowed data to replace owned data, usually by cloning. Read more

Source§

### impl<T, U> TryFrom<U> for T

where U: Into<T>,

Source§

#### type Error = Infallible

The type returned in the event of a conversion error.

Source§

#### fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error\>

Performs the conversion.

Source§

### impl<T, U> TryInto<U> for T

where U: TryFrom<T>,

Source§

#### type Error = <U as TryFrom<T>>::Error

The type returned in the event of a conversion error.

Source§

#### fn try_into(self) -> Result<U, <U as TryFrom<T>>::Error\>

Performs the conversion.
