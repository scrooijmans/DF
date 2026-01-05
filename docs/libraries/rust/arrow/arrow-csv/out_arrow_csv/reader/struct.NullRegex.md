# Struct NullRegex Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#162" class="src">Source</a>

``` rust
struct NullRegex(Option<Regex>);
```

Expand description

A wrapper over `Option<Regex>` to check if the value is `NULL`.

## Tuple Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#structfield.0" class="anchor field">Â§</a>`0: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<Regex>`

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#impl-NullRegex" class="anchor">Â§</a>

### impl <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex">NullRegex</a>

#### fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#method.is_null" class="fn">is_null</a>(&self, s: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if the value should be considered as `NULL` according to the provided regular expression.

## Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#trait-implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#impl-Clone-for-NullRegex" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex">NullRegex</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#method.clone" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex">NullRegex</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 Â· <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#method.clone_from" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#impl-Debug-for-NullRegex" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex">NullRegex</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#method.fmt" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#impl-Default-for-NullRegex" class="anchor">Â§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex">NullRegex</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#method.default" class="anchor">Â§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html" class="struct" title="struct arrow_csv::reader::NullRegex">NullRegex</a>

Returns the â€œdefault valueâ€? for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.NullRegex.html#blanket-implementations" class="anchor">Â§</a>
