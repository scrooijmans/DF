# Struct Extensions Copy item path

<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/src/http/extensions.rs.html#35" class="src">Source</a>

``` rust
pub struct Extensions { /* private fields */ }
```

Expand description

A type map of protocol extensions.

`Extensions` can be used by `Request` and `Response` to store extra data derived from the underlying protocol.

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#impl-Extensions" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>

Create an empty `Extensions`.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.insert" class="fn">insert</a>\<T\>(&mut self, val: T) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Insert a type into this `Extensions`.

If a extension of this type already existed, it will be returned and replaced with the new one.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
assert!(ext.insert(5i32).is_none());
assert!(ext.insert(4u8).is_none());
assert_eq!(ext.insert(9i32), Some(5i32));
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.get" class="fn">get</a>\<T\>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Get a reference to a type previously inserted on this `Extensions`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-1" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
assert!(ext.get::<i32>().is_none());
ext.insert(5i32);

assert_eq!(ext.get::<i32>(), Some(&5i32));
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.get_mut" class="fn">get_mut</a>\<T\>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Get a mutable reference to a type previously inserted on this `Extensions`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-2" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
ext.insert(String::from("Hello"));
ext.get_mut::<String>().unwrap().push_str(" World");

assert_eq!(ext.get::<String>().unwrap(), "Hello World");
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.get_or_insert" class="fn">get_or_insert</a>\<T\>(&mut self, value: T) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Get a mutable reference to a type, inserting `value` if not already present on this `Extensions`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-3" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
*ext.get_or_insert(1i32) += 2;

assert_eq!(*ext.get::<i32>().unwrap(), 3);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.get_or_insert_with" class="fn">get_or_insert_with</a>\<T, F\>(&mut self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static, F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> T,

Get a mutable reference to a type, inserting the value created by `f` if not already present on this `Extensions`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-4" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
*ext.get_or_insert_with(|| 1i32) += 2;

assert_eq!(*ext.get::<i32>().unwrap(), 3);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.get_or_insert_default" class="fn">get_or_insert_default</a>\<T\>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>

where T: <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> + <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Get a mutable reference to a type, inserting the type’s default value if not already present on this `Extensions`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-5" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
*ext.get_or_insert_default::<i32>() += 2;

assert_eq!(*ext.get::<i32>().unwrap(), 2);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.remove" class="fn">remove</a>\<T\>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'static,

Remove a type from this `Extensions`.

If a extension of this type existed, it will be returned.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-6" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
ext.insert(5i32);
assert_eq!(ext.remove::<i32>(), Some(5i32));
assert!(ext.get::<i32>().is_none());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.clear" class="fn">clear</a>(&mut self)

Clear the `Extensions` of all inserted extensions.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-7" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
ext.insert(5i32);
ext.clear();

assert!(ext.get::<i32>().is_none());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check whether the extension set is empty or not.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-8" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
assert!(ext.is_empty());
ext.insert(5i32);
assert!(!ext.is_empty());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the number of extensions available.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-9" class="doc-anchor">§</a>Example

``` rust
let mut ext = Extensions::new();
assert_eq!(ext.len(), 0);
ext.insert(5i32);
assert_eq!(ext.len(), 1);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.extend" class="fn">extend</a>(&mut self, other: <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>)

Extends `self` with another `Extensions`.

If an instance of a specific type exists in both, the one in `self` is overwritten with the one from `other`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#example-10" class="doc-anchor">§</a>Example

``` rust
let mut ext_a = Extensions::new();
ext_a.insert(8u8);
ext_a.insert(16u16);

let mut ext_b = Extensions::new();
ext_b.insert(4u8);
ext_b.insert("hello");

ext_a.extend(ext_b);
assert_eq!(ext_a.len(), 3);
assert_eq!(ext_a.get::<u8>(), Some(&4u8));
assert_eq!(ext_a.get::<u16>(), Some(&16u16));
assert_eq!(ext_a.get::<&'static str>().copied(), Some("hello"));
```

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#impl-Clone-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#impl-Debug-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#impl-Default-for-Extensions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html" class="struct" title="struct object_store::Extensions">Extensions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.Extensions.html#blanket-implementations" class="anchor">§</a>
