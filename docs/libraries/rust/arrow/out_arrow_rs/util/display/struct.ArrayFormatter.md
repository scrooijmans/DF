# Struct ArrayFormatter Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/display.rs.html#269" class="src">Source</a>

``` rust
pub struct ArrayFormatter<'a> { /* private fields */ }
```

Expand description

A string formatter for an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

This can be used with [`std::write`](https://doc.rust-lang.org/nightly/core/macro.write.html "macro core::write") to write type-erased `dyn Array`

``` rust
struct MyContainer {
    values: ArrayRef,
}

impl Display for MyContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let options = FormatOptions::default();
        let formatter = ArrayFormatter::try_new(self.values.as_ref(), &options)
            .map_err(|_| std::fmt::Error)?;

        let mut iter = 0..self.values.len();
        if let Some(idx) = iter.next() {
            write!(f, "{}", formatter.value(idx))?;
        }
        for idx in iter {
            write!(f, ", {}", formatter.value(idx))?;
        }
        Ok(())
    }
}
```

[`ValueFormatter::write`](https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html#method.write "method arrow::util::display::ValueFormatter::write") can also be used to get a semantic error, instead of the opaque [`std::fmt::Error`](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")

``` rust
fn format_array(
    f: &mut dyn Write,
    array: &dyn Array,
    options: &FormatOptions,
) -> Result<(), ArrowError> {
    let formatter = ArrayFormatter::try_new(array, options)?;
    for i in 0..array.len() {
        formatter.value(i).write(f)?
    }
    Ok(())
}
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html#impl-ArrayFormatter%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html" class="struct" title="struct arrow::util::display::ArrayFormatter">ArrayFormatter</a>\<'a\>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html#method.try_new" class="fn">try_new</a>( array: &'a dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.Array.html" class="trait" title="trait arrow::array::Array">Array</a>, options: &<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>\<'a\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html" class="struct" title="struct arrow::util::display::ArrayFormatter">ArrayFormatter</a>\<'a\>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>

Returns an [`ArrayFormatter`](https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html "struct arrow::util::display::ArrayFormatter") that can be used to format `array`

This returns an error if an array of the given data type cannot be formatted

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html#method.value" class="fn">value</a>(&self, idx: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html" class="struct" title="struct arrow::util::display::ValueFormatter">ValueFormatter</a>\<'\_\>

Returns a [`ValueFormatter`](https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html "struct arrow::util::display::ValueFormatter") that implements [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for the value of the array at `idx`

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html#blanket-implementations" class="anchor">§</a>
