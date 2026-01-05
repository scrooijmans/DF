# Struct InferredDataType Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#177-190" class="src">Source</a>

``` rust
struct InferredDataType {
    packed: u16,
}
```

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#structfield.packed" class="anchor field">Â§</a>`packed: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u16.html" class="primitive"><code>u16</code></a>

Packed booleans indicating type

0 - Boolean 1 - Integer 2 - Float64 3 - Date32 4 - Timestamp(Second) 5 - Timestamp(Millisecond) 6 - Timestamp(Microsecond) 7 - Timestamp(Nanosecond) 8 - Utf8

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#impl-InferredDataType" class="anchor">Â§</a>

### impl <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html" class="struct" title="struct arrow_csv::reader::InferredDataType">InferredDataType</a>

#### fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#method.get" class="fn">get</a>(&self) -\> DataType

Returns the inferred data type

#### fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#method.update" class="fn">update</a>(&mut self, string: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>)

Updates the [`InferredDataType`](https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html "struct arrow_csv::reader::InferredDataType") with the given string

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#impl-Clone-for-InferredDataType" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html" class="struct" title="struct arrow_csv::reader::InferredDataType">InferredDataType</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html" class="struct" title="struct arrow_csv::reader::InferredDataType">InferredDataType</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#impl-Default-for-InferredDataType" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html" class="struct" title="struct arrow_csv::reader::InferredDataType">InferredDataType</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html" class="struct" title="struct arrow_csv::reader::InferredDataType">InferredDataType</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#impl-Copy-for-InferredDataType" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html" class="struct" title="struct arrow_csv::reader::InferredDataType">InferredDataType</a>

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.InferredDataType.html#blanket-implementations" class="anchor">Â§</a>
