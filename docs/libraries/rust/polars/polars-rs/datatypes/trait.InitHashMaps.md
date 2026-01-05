# Trait InitHashMaps Copy item path

<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/src/polars_utils/aliases.rs.html#23" class="src">Source</a>

``` rust
pub trait InitHashMaps {
    type HashMap;

    // Required methods
    fn new() -> Self::HashMap;
    fn with_capacity(capacity: usize) -> Self::HashMap;
}
```

## Required Associated Types<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#required-associated-types" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype">HashMap</a>

## Required Methods<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.new" class="fn">new</a>() -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps::HashMap">HashMap</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps::HashMap">HashMap</a>

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#impl-InitHashMaps-for-HashSet%3CK,+RandomState%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a> for <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<K, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap-1" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype">HashMap</a> = <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<K, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.new" class="fn">new</a>() -\> \<<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<K, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps::HashMap">HashMap</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#method.with_capacity" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/set/struct.HashSet.html" class="struct" title="struct hashbrown::set::HashSet">HashSet</a>\<K, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#impl-InitHashMaps-for-IndexSet%3CK,+RandomState%3E" class="anchor">§</a>

### impl\<K\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a> for <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html" class="struct" title="struct indexmap::set::IndexSet">IndexSet</a>\<K, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap-2" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype">HashMap</a> = <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html" class="struct" title="struct indexmap::set::IndexSet">IndexSet</a>\<K, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#method.new-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.new" class="fn">new</a>() -\> \<<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html" class="struct" title="struct indexmap::set::IndexSet">IndexSet</a>\<K, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps::HashMap">HashMap</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#method.with_capacity-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.with_capacity" class="fn">with_capacity</a>( capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html" class="struct" title="struct indexmap::set::IndexSet">IndexSet</a>\<K, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps::HashMap">HashMap</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#impl-InitHashMaps-for-HashMap%3CK,+V,+RandomState%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a> for <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<K, V, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap-3" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype">HashMap</a> = <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<K, V, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#method.new-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.new" class="fn">new</a>() -\> \<<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<K, V, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps::HashMap">HashMap</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#method.with_capacity-2" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<K, V, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#impl-InitHashMaps-for-IndexMap%3CK,+V,+RandomState%3E" class="anchor">§</a>

### impl\<K, V\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a> for <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<K, V, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap-4" class="anchor">§</a>

#### type <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype">HashMap</a> = <a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<K, V, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#method.new-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.new" class="fn">new</a>() -\> \<<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<K, V, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps::HashMap">HashMap</a>

<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#method.with_capacity-3" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#tymethod.with_capacity" class="fn">with_capacity</a>( capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> \<<a href="https://docs.rs/indexmap/2.10.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html" class="struct" title="struct indexmap::map::IndexMap">IndexMap</a>\<K, V, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\> as <a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html" class="trait" title="trait polars::prelude::InitHashMaps">InitHashMaps</a>\>::<a href="https://docs.rs/polars/latest/polars/prelude/trait.InitHashMaps.html#associatedtype.HashMap" class="associatedtype" title="type polars::prelude::InitHashMaps::HashMap">HashMap</a>

## Implementors<a href="https://docs.rs/polars/latest/polars/datatypes/trait.InitHashMaps.html#implementors" class="anchor">§</a>
