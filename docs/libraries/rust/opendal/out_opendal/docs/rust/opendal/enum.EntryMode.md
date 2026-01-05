# Enum EntryMode Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/mode.rs.html#23-30" class="src">Source</a>

``` rust
pub enum EntryMode {
    FILE,
    DIR,
    Unknown,
}
```

Expand description

EntryMode represents the mode.

## Variants<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#variants" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#variant.FILE" class="anchor">Â§</a>

### FILE

FILE means the path has data to read.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#variant.DIR" class="anchor">Â§</a>

### DIR

DIR means the path can be listed.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#variant.Unknown" class="anchor">Â§</a>

### Unknown

Unknown means we donâ€™t know what we can do on this path.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-EntryMode" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.is_file" class="fn">is_file</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this mode is FILE.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.is_dir" class="fn">is_dir</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if this mode is DIR.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-Clone-for-EntryMode" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-Debug-for-EntryMode" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-Default-for-EntryMode" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-Display-for-EntryMode" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.fmt-1" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-PartialEq-for-EntryMode" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.eq" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#method.ne" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-Copy-for-EntryMode" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-Eq-for-EntryMode" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#impl-StructuralPartialEq-for-EntryMode" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html" class="enum" title="enum opendal::EntryMode">EntryMode</a>

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/enum.EntryMode.html#blanket-implementations" class="anchor">Â§</a>
