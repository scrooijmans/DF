# Struct GroupsTypeParIter Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/position.rs.html#565" class="src">Source</a>

``` rust
pub struct GroupsTypeParIter<'a> { /* private fields */ }
```

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#impl-ParallelIterator-for-GroupsTypeParIter%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html" class="struct" title="struct polars::prelude::GroupsTypeParIter">GroupsTypeParIter</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html" class="enum" title="enum polars::prelude::GroupsIndicator">GroupsIndicator</a>\<'a\>

The type of item that this parallel iterator produces. For example, if you use the [`for_each`](https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.for_each) method, this is the type of item that your closure will be invoked with.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.drive_unindexed" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#tymethod.drive_unindexed" class="fn">drive_unindexed</a>\<C\>( self, consumer: C, ) -\> \<C as <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.Consumer.html" class="trait" title="trait rayon::iter::plumbing::Consumer">Consumer</a>\<\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html" class="struct" title="struct polars::prelude::GroupsTypeParIter">GroupsTypeParIter</a>\<'a\> as <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>\>::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.Consumer.html#associatedtype.Result" class="associatedtype" title="type rayon::iter::plumbing::Consumer::Result">Result</a>

where C: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/plumbing/trait.UnindexedConsumer.html" class="trait" title="trait rayon::iter::plumbing::UnindexedConsumer">UnindexedConsumer</a>\<\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html" class="struct" title="struct polars::prelude::GroupsTypeParIter">GroupsTypeParIter</a>\<'a\> as <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>,

Internal method used to define the behavior of this parallel iterator. You should not need to call this directly. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#tymethod.drive_unindexed)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.for_each" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each" class="fn">for_each</a>\<OP\>(self, op: OP)

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes `OP` on each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.for_each_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each_with" class="fn">for_each_with</a>\<OP, T\>(self, init: T, op: OP)

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Executes `OP` on the given `init` value with each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each_with)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.for_each_init" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each_init" class="fn">for_each_init</a>\<OP, INIT, T\>(self, init: INIT, op: OP)

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, INIT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes `OP` on a value returned by `init` with each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.for_each_init)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.try_for_each" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each" class="fn">try_for_each</a>\<OP, R\>(self, op: OP) -\> R

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: Try\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes a fallible `OP` on each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.try_for_each_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each_with" class="fn">try_for_each_with</a>\<OP, T, R\>(self, init: T, op: OP) -\> R

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, R: Try\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes a fallible `OP` on the given `init` value with each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each_with)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.try_for_each_init" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each_init" class="fn">try_for_each_init</a>\<OP, INIT, T, R\>(self, init: INIT, op: OP) -\> R

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, INIT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: Try\<Output = <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Executes a fallible `OP` on a value returned by `init` with each item produced by the iterator, in parallel. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_for_each_init)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.count" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.count" class="fn">count</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Counts the number of items in this parallel iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.count)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.map" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map" class="fn">map</a>\<F, R\>(self, map_op: F) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/map/struct.Map.html" class="struct" title="struct rayon::iter::map::Map">Map</a>\<Self, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `map_op` to each item of this iterator, producing a new iterator with the results. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.map_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map_with" class="fn">map_with</a>\<F, T, R\>(self, init: T, map_op: F) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/map_with/struct.MapWith.html" class="struct" title="struct rayon::iter::map_with::MapWith">MapWith</a>\<Self, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `map_op` to the given `init` value with each item of this iterator, producing a new iterator with the results. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map_with)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.map_init" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map_init" class="fn">map_init</a>\<F, INIT, T, R\>( self, init: INIT, map_op: F, ) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/map_with/struct.MapInit.html" class="struct" title="struct rayon::iter::map_with::MapInit">MapInit</a>\<Self, INIT, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, INIT: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `map_op` to a value returned by `init` with each item of this iterator, producing a new iterator with the results. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.map_init)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.cloned" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.cloned" class="fn">cloned</a>\<'a, T\>(self) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/cloned/struct.Cloned.html" class="struct" title="struct rayon::iter::cloned::Cloned">Cloned</a>\<Self\>

where T: 'a + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, Self: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

Creates an iterator which clones all of its elements. This may be useful when you have an iterator over `&T`, but you need `T`, and that type implements `Clone`. See also [`copied()`](https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.copied). [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.cloned)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.copied" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.copied" class="fn">copied</a>\<'a, T\>(self) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/copied/struct.Copied.html" class="struct" title="struct rayon::iter::copied::Copied">Copied</a>\<Self\>

where T: 'a + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, Self: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>\>,

Creates an iterator which copies all of its elements. This may be useful when you have an iterator over `&T`, but you need `T`, and that type implements `Copy`. See also [`cloned()`](https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.cloned). [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.copied)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.inspect" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.inspect" class="fn">inspect</a>\<OP\>(self, inspect_op: OP) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/inspect/struct.Inspect.html" class="struct" title="struct rayon::iter::inspect::Inspect">Inspect</a>\<Self, OP\>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `inspect_op` to a reference to each item of this iterator, producing a new iterator passing through the original items. This is often useful for debugging to see what’s happening in iterator stages. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.inspect)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.update" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.update" class="fn">update</a>\<F\>(self, update_op: F) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/update/struct.Update.html" class="struct" title="struct rayon::iter::update::Update">Update</a>\<Self, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&mut Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Mutates each item of this iterator before yielding it. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.update)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.filter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.filter" class="fn">filter</a>\<P\>(self, filter_op: P) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/filter/struct.Filter.html" class="struct" title="struct rayon::iter::filter::Filter">Filter</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `filter_op` to each item of this iterator, producing a new iterator with only the items that gave `true` results. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.filter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.filter_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.filter_map" class="fn">filter_map</a>\<P, R\>(self, filter_op: P) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/filter_map/struct.FilterMap.html" class="struct" title="struct rayon::iter::filter_map::FilterMap">FilterMap</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `filter_op` to each item of this iterator to get an `Option`, producing a new iterator with only the items from `Some` results. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.filter_map)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.flat_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flat_map" class="fn">flat_map</a>\<F, PI\>(self, map_op: F) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/flat_map/struct.FlatMap.html" class="struct" title="struct rayon::iter::flat_map::FlatMap">FlatMap</a>\<Self, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> PI + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, PI: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>,

Applies `map_op` to each item of this iterator to get nested parallel iterators, producing a new parallel iterator that flattens these back into one. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flat_map)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.flat_map_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flat_map_iter" class="fn">flat_map_iter</a>\<F, SI\>(self, map_op: F) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/flat_map_iter/struct.FlatMapIter.html" class="struct" title="struct rayon::iter::flat_map_iter::FlatMapIter">FlatMapIter</a>\<Self, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> SI + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, SI: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, \<SI as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies `map_op` to each item of this iterator to get nested serial iterators, producing a new parallel iterator that flattens these back into one. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flat_map_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.flatten" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flatten" class="fn">flatten</a>(self) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/flatten/struct.Flatten.html" class="struct" title="struct rayon::iter::flatten::Flatten">Flatten</a>\<Self\>

where Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>,

An adaptor that flattens parallel-iterable `Item`s into one large iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flatten)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.flatten_iter" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flatten_iter" class="fn">flatten_iter</a>(self) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/flatten_iter/struct.FlattenIter.html" class="struct" title="struct rayon::iter::flatten_iter::FlattenIter">FlattenIter</a>\<Self\>

where Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>, \<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> as <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\>::<a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype" title="type core::iter::traits::collect::IntoIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

An adaptor that flattens serial-iterable `Item`s into one large iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.flatten_iter)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.reduce" class="fn">reduce</a>\<OP, ID\>(self, identity: ID, op: OP) -\> Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Reduces the items in the iterator into one item using `op`. The argument `identity` should be a closure that can produce “identity” value which may be inserted into the sequence as needed to create opportunities for parallel execution. So, for example, if you are doing a summation, then `identity()` ought to produce something that represents the zero for your type (but consider just calling `sum()` in that case). [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.reduce)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.reduce_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.reduce_with" class="fn">reduce_with</a>\<OP\>(self, op: OP) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Reduces the items in the iterator into one item using `op`. If the iterator is empty, `None` is returned; otherwise, `Some` is returned. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.reduce_with)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.try_reduce" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_reduce" class="fn">try_reduce</a>\<T, OP, ID\>(self, identity: ID, op: OP) -\> Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, T) -\> Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: Try\<Output = T\>,

Reduces the items in the iterator into one item using a fallible `op`. The `identity` argument is used the same way as in [`reduce()`](https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.reduce). [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_reduce)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.try_reduce_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_reduce_with" class="fn">try_reduce_with</a>\<T, OP\>(self, op: OP) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where OP: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, T) -\> Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: Try\<Output = T\>,

Reduces the items in the iterator into one item using a fallible `op`. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_reduce_with)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.fold" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.fold" class="fn">fold</a>\<T, ID, F\>(self, identity: ID, fold_op: F) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/fold/struct.Fold.html" class="struct" title="struct rayon::iter::fold::Fold">Fold</a>\<Self, ID, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Parallel fold is similar to sequential fold except that the sequence of items may be subdivided before it is folded. Consider a list of numbers like `22 3 77 89 46`. If you used sequential fold to add them (`fold(0, |a,b| a+b)`, you would wind up first adding 0 + 22, then 22 + 3, then 25 + 77, and so forth. The **parallel fold** works similarly except that it first breaks up your list into sublists, and hence instead of yielding up a single sum at the end, it yields up multiple sums. The number of results is nondeterministic, as is the point where the breaks occur. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.fold)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.fold_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.fold_with" class="fn">fold_with</a>\<F, T\>(self, init: T, fold_op: F) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/fold/struct.FoldWith.html" class="struct" title="struct rayon::iter::fold::FoldWith">FoldWith</a>\<Self, T, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Applies `fold_op` to the given `init` value with each item of this iterator, finally producing the value for further use. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.fold_with)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.try_fold" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_fold" class="fn">try_fold</a>\<T, R, ID, F\>( self, identity: ID, fold_op: F, ) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/try_fold/struct.TryFold.html" class="struct" title="struct rayon::iter::try_fold::TryFold">TryFold</a>\<Self, R, ID, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, ID: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>() -\> T + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: Try\<Output = T\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Performs a fallible parallel fold. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_fold)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.try_fold_with" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_fold_with" class="fn">try_fold_with</a>\<F, T, R\>(self, init: T, fold_op: F) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/try_fold/struct.TryFoldWith.html" class="struct" title="struct rayon::iter::try_fold::TryFoldWith">TryFoldWith</a>\<Self, R, F\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(T, Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> R + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: Try\<Output = T\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Performs a fallible parallel fold with a cloneable `init` value. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.try_fold_with)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.sum" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.sum" class="fn">sum</a>\<S\>(self) -\> S

where S: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html" class="trait" title="trait core::iter::traits::accum::Sum">Sum</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html" class="trait" title="trait core::iter::traits::accum::Sum">Sum</a>,

Sums up the items in the iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.sum)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.product" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.product" class="fn">product</a>\<P\>(self) -\> P

where P: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html" class="trait" title="trait core::iter::traits::accum::Product">Product</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\> + <a href="https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html" class="trait" title="trait core::iter::traits::accum::Product">Product</a>,

Multiplies all the items in the iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.product)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.min" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min" class="fn">min</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Computes the minimum of all the items in the iterator. If the iterator is empty, `None` is returned; otherwise, `Some(min)` is returned. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.min_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min_by" class="fn">min_by</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>, &Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Computes the minimum of all the items in the iterator with respect to the given comparison function. If the iterator is empty, `None` is returned; otherwise, `Some(min)` is returned. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min_by)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.min_by_key" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min_by_key" class="fn">min_by_key</a>\<K, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> K,

Computes the item that yields the minimum value for the given function. If the iterator is empty, `None` is returned; otherwise, `Some(item)` is returned. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.min_by_key)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.max" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max" class="fn">max</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a>,

Computes the maximum of all the items in the iterator. If the iterator is empty, `None` is returned; otherwise, `Some(max)` is returned. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.max_by" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max_by" class="fn">max_by</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>, &Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>,

Computes the maximum of all the items in the iterator with respect to the given comparison function. If the iterator is empty, `None` is returned; otherwise, `Some(max)` is returned. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max_by)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.max_by_key" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max_by_key" class="fn">max_by_key</a>\<K, F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, F: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> K,

Computes the item that yields the maximum value for the given function. If the iterator is empty, `None` is returned; otherwise, `Some(item)` is returned. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.max_by_key)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.chain" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.chain" class="fn">chain</a>\<C\>(self, chain: C) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/chain/struct.Chain.html" class="struct" title="struct rayon::iter::chain::Chain">Chain</a>\<Self, \<C as <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\>::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html#associatedtype.Iter" class="associatedtype" title="type rayon::iter::IntoParallelIterator::Iter">Iter</a>\>

where C: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.IntoParallelIterator.html" class="trait" title="trait rayon::iter::IntoParallelIterator">IntoParallelIterator</a>\<Item = Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>,

Takes two iterators and creates a new iterator over both. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.chain)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.find_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_any" class="fn">find_any</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for **some** item in the parallel iterator that matches the given predicate and returns it. This operation is similar to [`find` on sequential iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find) but the item returned may not be the **first** one in the parallel sequence which matches, since we search the entire sequence in parallel. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_any)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.find_first" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_first" class="fn">find_first</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for the sequentially **first** item in the parallel iterator that matches the given predicate and returns it. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_first)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.find_last" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_last" class="fn">find_last</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for the sequentially **last** item in the parallel iterator that matches the given predicate and returns it. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_last)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.find_map_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_any" class="fn">find_map_any</a>\<P, R\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies the given predicate to the items in the parallel iterator and returns **any** non-None result of the map operation. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_any)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.find_map_first" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_first" class="fn">find_map_first</a>\<P, R\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies the given predicate to the items in the parallel iterator and returns the sequentially **first** non-None result of the map operation. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_first)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.find_map_last" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_last" class="fn">find_map_last</a>\<P, R\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Applies the given predicate to the items in the parallel iterator and returns the sequentially **last** non-None result of the map operation. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.find_map_last)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.any" class="fn">any</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Searches for **some** item in the parallel iterator that matches the given predicate, and if so returns true. Once a match is found, we’ll attempt to stop process the rest of the items. Proving that there’s no match, returning false, does require visiting every item. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.any)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.all" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.all" class="fn">all</a>\<P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Tests that every item in the parallel iterator matches the given predicate, and if so returns true. If a counter-example is found, we’ll attempt to stop processing more items, then return false. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.all)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.while_some" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.while_some" class="fn">while_some</a>\<T\>(self) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/while_some/struct.WhileSome.html" class="struct" title="struct rayon::iter::while_some::WhileSome">WhileSome</a>\<Self\>

where Self: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>\>, T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Creates an iterator over the `Some` items of this iterator, halting as soon as any `None` is found. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.while_some)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.panic_fuse" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.panic_fuse" class="fn">panic_fuse</a>(self) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/panic_fuse/struct.PanicFuse.html" class="struct" title="struct rayon::iter::panic_fuse::PanicFuse">PanicFuse</a>\<Self\>

Wraps an iterator with a fuse in case of panics, to halt all threads as soon as possible. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.panic_fuse)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.collect" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.collect" class="fn">collect</a>\<C\>(self) -\> C

where C: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.FromParallelIterator.html" class="trait" title="trait rayon::iter::FromParallelIterator">FromParallelIterator</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>,

Creates a fresh collection containing all the elements produced by this parallel iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.collect)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.unzip" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.unzip" class="fn">unzip</a>\<A, B, FromA, FromB\>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(FromA, FromB)</a>

where Self: <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html" class="trait" title="trait rayon::iter::ParallelIterator">ParallelIterator</a>\<Item = <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>\>, FromA: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<A\>, FromB: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<B\>, A: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, B: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Unzips the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.unzip)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.partition" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.partition" class="fn">partition</a>\<A, B, P\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>

where A: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>, B: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Partitions the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. Items for which the `predicate` returns true go into the first container, and the rest go into the second. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.partition)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.partition_map" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.partition_map" class="fn">partition_map</a>\<A, B, P, L, R\>(self, predicate: P) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.tuple.html" class="primitive">(A, B)</a>

where A: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<L\>, B: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelExtend.html" class="trait" title="trait rayon::iter::ParallelExtend">ParallelExtend</a>\<R\>, P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://docs.rs/either/1.15.0/x86_64-unknown-linux-gnu/either/enum.Either.html" class="enum" title="enum either::Either">Either</a>\<L, R\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, L: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>, R: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Partitions and maps the items of a parallel iterator into a pair of arbitrary `ParallelExtend` containers. `Either::Left` items go into the first container, and `Either::Right` items go into the second. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.partition_map)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.intersperse" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.intersperse" class="fn">intersperse</a>(self, element: Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/intersperse/struct.Intersperse.html" class="struct" title="struct rayon::iter::intersperse::Intersperse">Intersperse</a>\<Self\>

where Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

Intersperses clones of an element between items of this iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.intersperse)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.take_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.take_any" class="fn">take_any</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/take_any/struct.TakeAny.html" class="struct" title="struct rayon::iter::take_any::TakeAny">TakeAny</a>\<Self\>

Creates an iterator that yields `n` elements from *anywhere* in the original iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.take_any)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.skip_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.skip_any" class="fn">skip_any</a>(self, n: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/skip_any/struct.SkipAny.html" class="struct" title="struct rayon::iter::skip_any::SkipAny">SkipAny</a>\<Self\>

Creates an iterator that skips `n` elements from *anywhere* in the original iterator. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.skip_any)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.take_any_while" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.take_any_while" class="fn">take_any_while</a>\<P\>(self, predicate: P) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/take_any_while/struct.TakeAnyWhile.html" class="struct" title="struct rayon::iter::take_any_while::TakeAnyWhile">TakeAnyWhile</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Creates an iterator that takes elements from *anywhere* in the original iterator until the given `predicate` returns `false`. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.take_any_while)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.skip_any_while" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.skip_any_while" class="fn">skip_any_while</a>\<P\>(self, predicate: P) -\> <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/skip_any_while/struct.SkipAnyWhile.html" class="struct" title="struct rayon::iter::skip_any_while::SkipAnyWhile">SkipAnyWhile</a>\<Self, P\>

where P: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(&Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>,

Creates an iterator that skips elements from *anywhere* in the original iterator until the given `predicate` returns `false`. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.skip_any_while)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.collect_vec_list" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.collect_vec_list" class="fn">collect_vec_list</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html" class="struct" title="struct alloc::collections::linked_list::LinkedList">LinkedList</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<Self::<a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#associatedtype.Item" class="associatedtype" title="type rayon::iter::ParallelIterator::Item">Item</a>\>\>

Collects this iterator into a linked list of vectors. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.collect_vec_list)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#method.opt_len" class="anchor">§</a>

#### fn <a href="https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.opt_len" class="fn">opt_len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Internal method used to define the behavior of this parallel iterator. You should not need to call this directly. [Read more](https://docs.rs/rayon/1.10.0/x86_64-unknown-linux-gnu/rayon/iter/trait.ParallelIterator.html#method.opt_len)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.GroupsTypeParIter.html#blanket-implementations" class="anchor">§</a>
