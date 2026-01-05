# Enum GroupsIndicator Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/group_by/position.rs.html#499" class="src">Source</a>

``` rust
pub enum GroupsIndicator<'a> {
    Idx((u32, &'a UnitVec<u32>)),
    Slice([u32; 2]),
}
```

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#variant.Idx" class="anchor">§</a>

### Idx((<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>, &'a <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/idx_vec/struct.UnitVec.html" class="struct" title="struct polars_utils::idx_vec::UnitVec">UnitVec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>\>))

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#variant.Slice" class="anchor">§</a>

### Slice(\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>; <a href="https://doc.rust-lang.org/nightly/std/primitive.array.html" class="primitive">2</a>\])

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#impl-GroupsIndicator%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html" class="enum" title="enum polars::prelude::GroupsIndicator">GroupsIndicator</a>\<'\_\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#method.first" class="fn">first</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u32.html" class="primitive">u32</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.GroupsIndicator.html#blanket-implementations" class="anchor">§</a>
