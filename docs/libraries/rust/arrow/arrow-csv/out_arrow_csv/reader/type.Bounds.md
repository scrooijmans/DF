# Type Alias Bounds Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#441" class="src">Source</a>

``` rust
type Bounds = Option<(usize, usize)>;
```

## Aliased Type<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Bounds.html#aliased-type" class="anchor">Â§</a>

``` rust
enum Bounds {
    None,
    Some((usize, usize)),
}
```

## Variants<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Bounds.html#variants" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Bounds.html#variant.None" class="anchor">Â§</a>1.0.0

### None

No value.

<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Bounds.html#variant.Some" class="anchor">Â§</a>1.0.0

### Some((<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>))

Some value of type `T`.
