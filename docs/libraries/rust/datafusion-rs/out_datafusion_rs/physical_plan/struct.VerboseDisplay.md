# Struct VerboseDisplay Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/display.rs.html#1041" class="src">Source</a>

``` rust
pub struct VerboseDisplay<T>(pub T);
```

Expand description

A new type wrapper to display `T` implementing `DisplayAs` using the `Verbose` mode

## Tuple Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html#structfield.0" class="anchor field">§</a>`0: T`

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html#impl-Display-for-VerboseDisplay%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html" class="struct" title="struct datafusion::physical_plan::VerboseDisplay">VerboseDisplay</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.VerboseDisplay.html#blanket-implementations" class="anchor">§</a>
