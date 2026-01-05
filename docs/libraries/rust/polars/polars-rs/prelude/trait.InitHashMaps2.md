# Trait InitHashMaps2 Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/aliases.rs.html#26" class="src">Source</a>

``` rust
pub trait InitHashMaps2 {
    type HashMap;

    // Required methods
    fn new() -> Self::HashMap;
    fn with_capacity(capacity: usize) -> Self::HashMap;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#associatedtype.HashMap" class="associatedtype">HashMap</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#tymethod.new" class="fn">new</a>() -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps2::HashMap">HashMap</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#tymethod.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps2::HashMap">HashMap</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#impl-InitHashMaps2-for-HashMap%3CK,+V,+BuildHasherDefault%3CIdHasher%3E%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html" class="trait" title="trait polars::prelude::InitHashMaps2">InitHashMaps2</a> for <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<K, V, <a href="https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html" class="struct" title="struct core::hash::BuildHasherDefault">BuildHasherDefault</a>\<<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/hashing/identity/struct.IdHasher.html" class="struct" title="struct polars_core::hashing::identity::IdHasher">IdHasher</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#associatedtype.HashMap-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#associatedtype.HashMap" class="associatedtype">HashMap</a> = <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<K, V, <a href="https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html" class="struct" title="struct core::hash::BuildHasherDefault">BuildHasherDefault</a>\<<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/hashing/identity/struct.IdHasher.html" class="struct" title="struct polars_core::hashing::identity::IdHasher">IdHasher</a>\>\>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#tymethod.new" class="fn">new</a>() -\> \<<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<K, V, <a href="https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html" class="struct" title="struct core::hash::BuildHasherDefault">BuildHasherDefault</a>\<<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/hashing/identity/struct.IdHasher.html" class="struct" title="struct polars_core::hashing::identity::IdHasher">IdHasher</a>\>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html" class="trait" title="trait polars::prelude::InitHashMaps2">InitHashMaps2</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps2::HashMap">HashMap</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#method.with_capacity" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#tymethod.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<K, V, <a href="https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html" class="struct" title="struct core::hash::BuildHasherDefault">BuildHasherDefault</a>\<<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/polars_core/hashing/identity/struct.IdHasher.html" class="struct" title="struct polars_core::hashing::identity::IdHasher">IdHasher</a>\>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps2.html#implementors" class="anchor">§</a>
