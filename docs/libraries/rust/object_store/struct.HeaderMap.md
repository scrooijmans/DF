# Struct HeaderMap Copy item path

<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/src/http/header/map.rs.html#45" class="src">Source</a>

``` rust
pub struct HeaderMap<T = HeaderValue> { /* private fields */ }
```

Expand description

A set of HTTP headers

`HeaderMap` is a multimap of [`HeaderName`](https://docs.rs/object_store/latest/object_store/struct.HeaderName.html) to values.

## <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples" class="doc-anchor">§</a>Examples

Basic usage

``` rust
let mut headers = HeaderMap::new();

headers.insert(HOST, "example.com".parse().unwrap());
headers.insert(CONTENT_LENGTH, "123".parse().unwrap());

assert!(headers.contains_key(HOST));
assert!(!headers.contains_key(LOCATION));

assert_eq!(headers[HOST], "example.com");

headers.remove(HOST);

assert!(!headers.contains_key(HOST));
```

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-HeaderMap" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>

Create an empty `HeaderMap`.

The map will be created without any capacity. This function will not allocate.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-1" class="doc-anchor">§</a>Examples

``` rust
let map = HeaderMap::new();

assert!(map.is_empty());
assert_eq!(0, map.capacity());
```

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.with_capacity" class="fn">with_capacity</a>(capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

Create an empty `HeaderMap` with the specified capacity.

The returned map will allocate internal storage in order to hold about `capacity` elements without reallocating. However, this is a “best effort” as there are usage patterns that could cause additional allocations before `capacity` headers are stored in the map.

More capacity than requested may be allocated.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#panics" class="doc-anchor">§</a>Panics

This method panics if capacity exceeds max `HeaderMap` capacity.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-2" class="doc-anchor">§</a>Examples

``` rust
let map: HeaderMap<u32> = HeaderMap::with_capacity(10);

assert!(map.is_empty());
assert_eq!(12, map.capacity());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.try_with_capacity" class="fn">try_with_capacity</a>( capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.MaxSizeReached.html" class="struct" title="struct http::header::map::MaxSizeReached">MaxSizeReached</a>\>

Create an empty `HeaderMap` with the specified capacity.

The returned map will allocate internal storage in order to hold about `capacity` elements without reallocating. However, this is a “best effort” as there are usage patterns that could cause additional allocations before `capacity` headers are stored in the map.

More capacity than requested may be allocated.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#errors" class="doc-anchor">§</a>Errors

This function may return an error if `HeaderMap` exceeds max capacity

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-3" class="doc-anchor">§</a>Examples

``` rust
let map: HeaderMap<u32> = HeaderMap::try_with_capacity(10).unwrap();

assert!(map.is_empty());
assert_eq!(12, map.capacity());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of headers stored in the map.

This number represents the total number of **values** stored in the map. This number can be greater than or equal to the number of **keys** stored given that a single key may have more than one associated value.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-4" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

assert_eq!(0, map.len());

map.insert(ACCEPT, "text/plain".parse().unwrap());
map.insert(HOST, "localhost".parse().unwrap());

assert_eq!(2, map.len());

map.append(ACCEPT, "text/html".parse().unwrap());

assert_eq!(3, map.len());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.keys_len" class="fn">keys_len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of keys stored in the map.

This number will be less than or equal to `len()` as each key may have more than one associated value.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-5" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

assert_eq!(0, map.keys_len());

map.insert(ACCEPT, "text/plain".parse().unwrap());
map.insert(HOST, "localhost".parse().unwrap());

assert_eq!(2, map.keys_len());

map.insert(ACCEPT, "text/html".parse().unwrap());

assert_eq!(2, map.keys_len());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the map contains no elements.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-6" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

assert!(map.is_empty());

map.insert(HOST, "hello.world".parse().unwrap());

assert!(!map.is_empty());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.clear" class="fn">clear</a>(&mut self)

Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-7" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
map.insert(HOST, "hello.world".parse().unwrap());

map.clear();
assert!(map.is_empty());
assert!(map.capacity() > 0);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.capacity" class="fn">capacity</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of headers the map can hold without reallocating.

This number is an approximation as certain usage patterns could cause additional allocations before the returned capacity is filled.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-8" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

assert_eq!(0, map.capacity());

map.insert(HOST, "hello.world".parse().unwrap());
assert_eq!(6, map.capacity());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.reserve" class="fn">reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Reserves capacity for at least `additional` more headers to be inserted into the `HeaderMap`.

The header map may reserve more space to avoid frequent reallocations. Like with `with_capacity`, this will be a “best effort” to avoid allocations until `additional` more headers are inserted. Certain usage patterns could cause additional allocations before the number is reached.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#panics-1" class="doc-anchor">§</a>Panics

Panics if the new allocation size overflows `HeaderMap` `MAX_SIZE`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-9" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
map.reserve(10);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.try_reserve" class="fn">try_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.MaxSizeReached.html" class="struct" title="struct http::header::map::MaxSizeReached">MaxSizeReached</a>\>

Reserves capacity for at least `additional` more headers to be inserted into the `HeaderMap`.

The header map may reserve more space to avoid frequent reallocations. Like with `with_capacity`, this will be a “best effort” to avoid allocations until `additional` more headers are inserted. Certain usage patterns could cause additional allocations before the number is reached.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#errors-1" class="doc-anchor">§</a>Errors

This method differs from `reserve` by returning an error instead of panicking if the value is too large.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-10" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
map.try_reserve(10).unwrap();
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.get" class="fn">get</a>\<K\>(&self, key: K) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html" class="trait" title="trait http::header::map::as_header_name::AsHeaderName">AsHeaderName</a>,

Returns a reference to the value associated with the key.

If there are multiple values associated with the key, then the first one is returned. Use `get_all` to get all values associated with a given key. Returns `None` if there are no values associated with the key.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-11" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
assert!(map.get("host").is_none());

map.insert(HOST, "hello".parse().unwrap());
assert_eq!(map.get(HOST).unwrap(), &"hello");
assert_eq!(map.get("host").unwrap(), &"hello");

map.append(HOST, "world".parse().unwrap());
assert_eq!(map.get("host").unwrap(), &"hello");
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.get_mut" class="fn">get_mut</a>\<K\>(&mut self, key: K) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut T</a>\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html" class="trait" title="trait http::header::map::as_header_name::AsHeaderName">AsHeaderName</a>,

Returns a mutable reference to the value associated with the key.

If there are multiple values associated with the key, then the first one is returned. Use `entry` to get all values associated with a given key. Returns `None` if there are no values associated with the key.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-12" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::default();
map.insert(HOST, "hello".to_string());
map.get_mut("host").unwrap().push_str("-world");

assert_eq!(map.get(HOST).unwrap(), &"hello-world");
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.get_all" class="fn">get_all</a>\<K\>(&self, key: K) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.GetAll.html" class="struct" title="struct http::header::map::GetAll">GetAll</a>\<'\_, T\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html" class="trait" title="trait http::header::map::as_header_name::AsHeaderName">AsHeaderName</a>,

Returns a view of all values associated with a key.

The returned view does not incur any allocations and allows iterating the values associated with the key. See [`GetAll`](https://docs.rs/object_store/latest/object_store/struct.GetAll.html) for more details. Returns `None` if there are no values associated with the key.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-13" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

map.insert(HOST, "hello".parse().unwrap());
map.append(HOST, "goodbye".parse().unwrap());

let view = map.get_all("host");

let mut iter = view.iter();
assert_eq!(&"hello", iter.next().unwrap());
assert_eq!(&"goodbye", iter.next().unwrap());
assert!(iter.next().is_none());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.contains_key" class="fn">contains_key</a>\<K\>(&self, key: K) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html" class="trait" title="trait http::header::map::as_header_name::AsHeaderName">AsHeaderName</a>,

Returns true if the map contains a value for the specified key.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-14" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
assert!(!map.contains_key(HOST));

map.insert(HOST, "world".parse().unwrap());
assert!(map.contains_key("host"));
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.iter" class="fn">iter</a>(&self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.Iter.html" class="struct" title="struct http::header::map::Iter">Iter</a>\<'\_, T\>

An iterator visiting all key-value pairs.

The iteration order is arbitrary, but consistent across platforms for the same crate version. Each key will be yielded once per associated value. So, if a key has 3 associated values, it will be yielded 3 times.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-15" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

map.insert(HOST, "hello".parse().unwrap());
map.append(HOST, "goodbye".parse().unwrap());
map.insert(CONTENT_LENGTH, "123".parse().unwrap());

for (key, value) in map.iter() {
    println!("{:?}: {:?}", key, value);
}
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.iter_mut" class="fn">iter_mut</a>(&mut self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.IterMut.html" class="struct" title="struct http::header::map::IterMut">IterMut</a>\<'\_, T\>

An iterator visiting all key-value pairs, with mutable value references.

The iterator order is arbitrary, but consistent across platforms for the same crate version. Each key will be yielded once per associated value, so if a key has 3 associated values, it will be yielded 3 times.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-16" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::default();

map.insert(HOST, "hello".to_string());
map.append(HOST, "goodbye".to_string());
map.insert(CONTENT_LENGTH, "123".to_string());

for (key, value) in map.iter_mut() {
    value.push_str("-boop");
}
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.keys" class="fn">keys</a>(&self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.Keys.html" class="struct" title="struct http::header::map::Keys">Keys</a>\<'\_, T\>

An iterator visiting all keys.

The iteration order is arbitrary, but consistent across platforms for the same crate version. Each key will be yielded only once even if it has multiple associated values.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-17" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

map.insert(HOST, "hello".parse().unwrap());
map.append(HOST, "goodbye".parse().unwrap());
map.insert(CONTENT_LENGTH, "123".parse().unwrap());

for key in map.keys() {
    println!("{:?}", key);
}
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.values" class="fn">values</a>(&self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.Values.html" class="struct" title="struct http::header::map::Values">Values</a>\<'\_, T\>

An iterator visiting all values.

The iteration order is arbitrary, but consistent across platforms for the same crate version.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-18" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

map.insert(HOST, "hello".parse().unwrap());
map.append(HOST, "goodbye".parse().unwrap());
map.insert(CONTENT_LENGTH, "123".parse().unwrap());

for value in map.values() {
    println!("{:?}", value);
}
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.values_mut" class="fn">values_mut</a>(&mut self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.ValuesMut.html" class="struct" title="struct http::header::map::ValuesMut">ValuesMut</a>\<'\_, T\>

An iterator visiting all values mutably.

The iteration order is arbitrary, but consistent across platforms for the same crate version.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-19" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::default();

map.insert(HOST, "hello".to_string());
map.append(HOST, "goodbye".to_string());
map.insert(CONTENT_LENGTH, "123".to_string());

for value in map.values_mut() {
    value.push_str("-boop");
}
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.drain" class="fn">drain</a>(&mut self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.Drain.html" class="struct" title="struct http::header::map::Drain">Drain</a>\<'\_, T\>

Clears the map, returning all entries as an iterator.

The internal memory is kept for reuse.

For each yielded item that has `None` provided for the `HeaderName`, then the associated header name is the same as that of the previously yielded item. The first yielded item will have `HeaderName` set.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-20" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

map.insert(HOST, "hello".parse().unwrap());
map.append(HOST, "goodbye".parse().unwrap());
map.insert(CONTENT_LENGTH, "123".parse().unwrap());

let mut drain = map.drain();


assert_eq!(drain.next(), Some((Some(HOST), "hello".parse().unwrap())));
assert_eq!(drain.next(), Some((None, "goodbye".parse().unwrap())));

assert_eq!(drain.next(), Some((Some(CONTENT_LENGTH), "123".parse().unwrap())));

assert_eq!(drain.next(), None);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.entry" class="fn">entry</a>\<K\>(&mut self, key: K) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/enum.Entry.html" class="enum" title="enum http::header::map::Entry">Entry</a>\<'\_, T\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/into_header_name/trait.IntoHeaderName.html" class="trait" title="trait http::header::map::into_header_name::IntoHeaderName">IntoHeaderName</a>,

Gets the given key’s corresponding entry in the map for in-place manipulation.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#panics-2" class="doc-anchor">§</a>Panics

This method panics if capacity exceeds max `HeaderMap` capacity

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-21" class="doc-anchor">§</a>Examples

``` rust
let mut map: HeaderMap<u32> = HeaderMap::default();

let headers = &[
    "content-length",
    "x-hello",
    "Content-Length",
    "x-world",
];

for &header in headers {
    let counter = map.entry(header).or_insert(0);
    *counter += 1;
}

assert_eq!(map["content-length"], 2);
assert_eq!(map["x-hello"], 1);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.try_entry" class="fn">try_entry</a>\<K\>( &mut self, key: K, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/enum.Entry.html" class="enum" title="enum http::header::map::Entry">Entry</a>\<'\_, T\>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.InvalidHeaderName.html" class="struct" title="struct http::header::name::InvalidHeaderName">InvalidHeaderName</a>\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html" class="trait" title="trait http::header::map::as_header_name::AsHeaderName">AsHeaderName</a>,

Gets the given key’s corresponding entry in the map for in-place manipulation.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#errors-2" class="doc-anchor">§</a>Errors

This method differs from `entry` by allowing types that may not be valid `HeaderName`s to passed as the key (such as `String`). If they do not parse as a valid `HeaderName`, this returns an `InvalidHeaderName` error.

If reserving space goes over the maximum, this will also return an error. However, to prevent breaking changes to the return type, the error will still say `InvalidHeaderName`, unlike other `try_*` methods which return a `MaxSizeReached` error.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.insert" class="fn">insert</a>\<K\>(&mut self, key: K, val: T) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/into_header_name/trait.IntoHeaderName.html" class="trait" title="trait http::header::map::into_header_name::IntoHeaderName">IntoHeaderName</a>,

Inserts a key-value pair into the map.

If the map did not previously have this key present, then `None` is returned.

If the map did have this key present, the new value is associated with the key and all previous values are removed. **Note** that only a single one of the previous values is returned. If there are multiple values that have been previously associated with the key, then the first one is returned. See `insert_mult` on `OccupiedEntry` for an API that returns all values.

The key is not updated, though; this matters for types that can be `==` without being identical.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#panics-3" class="doc-anchor">§</a>Panics

This method panics if capacity exceeds max `HeaderMap` capacity

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-22" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
assert!(map.insert(HOST, "world".parse().unwrap()).is_none());
assert!(!map.is_empty());

let mut prev = map.insert(HOST, "earth".parse().unwrap()).unwrap();
assert_eq!("world", prev);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.try_insert" class="fn">try_insert</a>\<K\>( &mut self, key: K, val: T, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.MaxSizeReached.html" class="struct" title="struct http::header::map::MaxSizeReached">MaxSizeReached</a>\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/into_header_name/trait.IntoHeaderName.html" class="trait" title="trait http::header::map::into_header_name::IntoHeaderName">IntoHeaderName</a>,

Inserts a key-value pair into the map.

If the map did not previously have this key present, then `None` is returned.

If the map did have this key present, the new value is associated with the key and all previous values are removed. **Note** that only a single one of the previous values is returned. If there are multiple values that have been previously associated with the key, then the first one is returned. See `insert_mult` on `OccupiedEntry` for an API that returns all values.

The key is not updated, though; this matters for types that can be `==` without being identical.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#errors-3" class="doc-anchor">§</a>Errors

This function may return an error if `HeaderMap` exceeds max capacity

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-23" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
assert!(map.try_insert(HOST, "world".parse().unwrap()).unwrap().is_none());
assert!(!map.is_empty());

let mut prev = map.try_insert(HOST, "earth".parse().unwrap()).unwrap().unwrap();
assert_eq!("world", prev);
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.append" class="fn">append</a>\<K\>(&mut self, key: K, value: T) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/into_header_name/trait.IntoHeaderName.html" class="trait" title="trait http::header::map::into_header_name::IntoHeaderName">IntoHeaderName</a>,

Inserts a key-value pair into the map.

If the map did not previously have this key present, then `false` is returned.

If the map did have this key present, the new value is pushed to the end of the list of values currently associated with the key. The key is not updated, though; this matters for types that can be `==` without being identical.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#panics-4" class="doc-anchor">§</a>Panics

This method panics if capacity exceeds max `HeaderMap` capacity

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-24" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
assert!(map.insert(HOST, "world".parse().unwrap()).is_none());
assert!(!map.is_empty());

map.append(HOST, "earth".parse().unwrap());

let values = map.get_all("host");
let mut i = values.iter();
assert_eq!("world", *i.next().unwrap());
assert_eq!("earth", *i.next().unwrap());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.try_append" class="fn">try_append</a>\<K\>( &mut self, key: K, value: T, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.MaxSizeReached.html" class="struct" title="struct http::header::map::MaxSizeReached">MaxSizeReached</a>\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/into_header_name/trait.IntoHeaderName.html" class="trait" title="trait http::header::map::into_header_name::IntoHeaderName">IntoHeaderName</a>,

Inserts a key-value pair into the map.

If the map did not previously have this key present, then `false` is returned.

If the map did have this key present, the new value is pushed to the end of the list of values currently associated with the key. The key is not updated, though; this matters for types that can be `==` without being identical.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#errors-4" class="doc-anchor">§</a>Errors

This function may return an error if `HeaderMap` exceeds max capacity

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-25" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
assert!(map.try_insert(HOST, "world".parse().unwrap()).unwrap().is_none());
assert!(!map.is_empty());

map.try_append(HOST, "earth".parse().unwrap()).unwrap();

let values = map.get_all("host");
let mut i = values.iter();
assert_eq!("world", *i.next().unwrap());
assert_eq!("earth", *i.next().unwrap());
```

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.remove" class="fn">remove</a>\<K\>(&mut self, key: K) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<T\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html" class="trait" title="trait http::header::map::as_header_name::AsHeaderName">AsHeaderName</a>,

Removes a key from the map, returning the value associated with the key.

Returns `None` if the map does not contain the key. If there are multiple values associated with the key, then the first one is returned. See `remove_entry_mult` on `OccupiedEntry` for an API that yields all values.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-26" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();
map.insert(HOST, "hello.world".parse().unwrap());

let prev = map.remove(HOST).unwrap();
assert_eq!("hello.world", prev);

assert!(map.remove(HOST).is_none());
```

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-Clone-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a>,

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-Debug-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-Default-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-Extend%3C(HeaderName,+T)%3E-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<(<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>, T)\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.extend-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>, T)\>,

Extends a collection with the contents of an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.extend_one-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.extend_reserve-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-Extend%3C(Option%3CHeaderName%3E,+T)%3E-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html" class="trait" title="trait core::iter::traits::collect::Extend">Extend</a>\<(<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>\>, T)\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.extend" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend" class="fn">extend</a>\<I\>(&mut self, iter: I)

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>\>, T)\>,

Extend a `HeaderMap` with the contents of another `HeaderMap`.

This function expects the yielded items to follow the same structure as `IntoIter`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#panics-5" class="doc-anchor">§</a>Panics

This panics if the first yielded item does not have a `HeaderName`.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-29" class="doc-anchor">§</a>Examples

``` rust
let mut map = HeaderMap::new();

map.insert(ACCEPT, "text/plain".parse().unwrap());
map.insert(HOST, "hello.world".parse().unwrap());

let mut extra = HeaderMap::new();

extra.insert(HOST, "foo.bar".parse().unwrap());
extra.insert(COOKIE, "hello".parse().unwrap());
extra.append(COOKIE, "world".parse().unwrap());

map.extend(extra);

assert_eq!(map["host"], "foo.bar");
assert_eq!(map["accept"], "text/plain");
assert_eq!(map["cookie"], "hello");

let v = map.get_all("host");
assert_eq!(1, v.iter().count());

let v = map.get_all("cookie");
assert_eq!(2, v.iter().count());
```

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.extend_one" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one" class="fn">extend_one</a>(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.extend_reserve" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve" class="fn">extend_reserve</a>(&mut self, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-FromIterator%3C(HeaderName,+T)%3E-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html" class="trait" title="trait core::iter::traits::collect::FromIterator">FromIterator</a>\<(<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>, T)\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.from_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter" class="fn">from_iter</a>\<I\>(iter: I) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

where I: <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a>\<Item = (<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>, T)\>,

Creates a value from an iterator. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-Index%3CK%3E-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<K, T\> <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html" class="trait" title="trait core::ops::index::Index">Index</a>\<K\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

where K: <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/as_header_name/trait.AsHeaderName.html" class="trait" title="trait http::header::map::as_header_name::AsHeaderName">AsHeaderName</a>,

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.index" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index" class="fn">index</a>(&self, index: K) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;T</a>

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#panics-6" class="doc-anchor">§</a>Panics

Using the index operator will cause a panic if the header you’re querying isn’t set.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#associatedtype.Output" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output" class="associatedtype">Output</a> = T

The returned type after indexing.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-IntoIterator-for-%26HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (&'a <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a T</a>)

The type of the elements being iterated over.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#associatedtype.IntoIter" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.Iter.html" class="struct" title="struct http::header::map::Iter">Iter</a>\<'a, T\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.into_iter" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.Iter.html" class="struct" title="struct http::header::map::Iter">Iter</a>\<'a, T\>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-IntoIterator-for-%26mut+HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<'a, T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for &'a mut <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#associatedtype.Item-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (&'a <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a mut T</a>)

The type of the elements being iterated over.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#associatedtype.IntoIter-1" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.IterMut.html" class="struct" title="struct http::header::map::IterMut">IterMut</a>\<'a, T\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.into_iter-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.IterMut.html" class="struct" title="struct http::header::map::IterMut">IterMut</a>\<'a, T\>

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-IntoIterator-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html" class="trait" title="trait core::iter::traits::collect::IntoIterator">IntoIterator</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.into_iter-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter" class="fn">into_iter</a>(self) -\> <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.IntoIter.html" class="struct" title="struct http::header::map::IntoIter">IntoIter</a>\<T\>

Creates a consuming iterator, that is, one that moves keys and values out of the map in arbitrary order. The map cannot be used after calling this.

For each yielded item that has `None` provided for the `HeaderName`, then the associated header name is the same as that of the previously yielded item. The first yielded item will have `HeaderName` set.

##### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-27" class="doc-anchor">§</a>Examples

Basic usage.

``` rust
let mut map = HeaderMap::new();
map.insert(header::CONTENT_LENGTH, "123".parse().unwrap());
map.insert(header::CONTENT_TYPE, "json".parse().unwrap());

let mut iter = map.into_iter();
assert_eq!(iter.next(), Some((Some(header::CONTENT_LENGTH), "123".parse().unwrap())));
assert_eq!(iter.next(), Some((Some(header::CONTENT_TYPE), "json".parse().unwrap())));
assert!(iter.next().is_none());
```

Multiple values per key.

``` rust
let mut map = HeaderMap::new();

map.append(header::CONTENT_LENGTH, "123".parse().unwrap());
map.append(header::CONTENT_LENGTH, "456".parse().unwrap());

map.append(header::CONTENT_TYPE, "json".parse().unwrap());
map.append(header::CONTENT_TYPE, "html".parse().unwrap());
map.append(header::CONTENT_TYPE, "xml".parse().unwrap());

let mut iter = map.into_iter();

assert_eq!(iter.next(), Some((Some(header::CONTENT_LENGTH), "123".parse().unwrap())));
assert_eq!(iter.next(), Some((None, "456".parse().unwrap())));

assert_eq!(iter.next(), Some((Some(header::CONTENT_TYPE), "json".parse().unwrap())));
assert_eq!(iter.next(), Some((None, "html".parse().unwrap())));
assert_eq!(iter.next(), Some((None, "xml".parse().unwrap())));
assert!(iter.next().is_none());
```

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#associatedtype.Item-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item" class="associatedtype">Item</a> = (<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>\>, T)

The type of the elements being iterated over.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#associatedtype.IntoIter-2" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter" class="associatedtype">IntoIter</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/map/struct.IntoIter.html" class="struct" title="struct http::header::map::IntoIter">IntoIter</a>\<T\>

Which kind of iterator are we turning this into?

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-PartialEq-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>,

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-TryFrom%3C%26HashMap%3CK,+V,+S%3E%3E-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<'a, K, V, S, T\> <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<K, V, S\>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

where K: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> + <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a>, <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a K</a>\>, \<<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/header/name/struct.HeaderName.html" class="struct" title="struct http::header::name::HeaderName">HeaderName</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a K</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/error/struct.Error.html" class="struct" title="struct http::error::Error">Error</a>\>, T: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a>\>, \<T as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;'a V</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/error/struct.Error.html" class="struct" title="struct http::error::Error">Error</a>\>,

Try to convert a `HashMap` into a `HeaderMap`.

#### <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#examples-28" class="doc-anchor">§</a>Examples

``` rust
use std::collections::HashMap;
use std::convert::TryInto;
use http::HeaderMap;

let mut map = HashMap::new();
map.insert("X-Custom-Header".to_string(), "my value".to_string());

let headers: HeaderMap = (&map).try_into().expect("valid headers");
assert_eq!(headers["X-Custom-Header"], "my value");
```

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/error/struct.Error.html" class="struct" title="struct http::error::Error">Error</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( c: &'a <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<K, V, S\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>, \<<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<&'a <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<K, V, S\>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#impl-Eq-for-HeaderMap%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html" class="struct" title="struct object_store::HeaderMap">HeaderMap</a>\<T\>

where T: <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a>,

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.HeaderMap.html#blanket-implementations" class="anchor">§</a>
