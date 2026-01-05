# Enum SortOrder Copy item path

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/src/supabase_rust_postgrest/lib.rs.html#93-96" class="src">Source</a>

``` rust
pub enum SortOrder {
    Ascending,
    Descending,
}
```

Expand description

ソート方向

## Variants<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#variants" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#variant.Ascending" class="anchor">§</a>

### Ascending

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#variant.Descending" class="anchor">§</a>

### Descending

## Trait Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#impl-Clone-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#impl-Debug-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#impl-PartialEq-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#impl-Copy-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html" class="trait" title="trait core::marker::Copy">Copy</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#impl-Eq-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#impl-StructuralPartialEq-for-SortOrder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>

## Auto Trait Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html#blanket-implementations" class="anchor">§</a>
