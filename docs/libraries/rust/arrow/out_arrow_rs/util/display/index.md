# Module display Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/lib.rs.html#28" class="src">Source</a>

Expand description

Functions for printing array values as human-readable strings.

This is often used for debugging or logging purposes.

See the [`pretty`](https://docs.rs/arrow/latest/arrow/util/pretty/index.html "mod arrow::util::pretty") crate for additional functions for record batch pretty printing.

## Structs<a href="https://docs.rs/arrow/latest/arrow/util/display/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html" class="struct" title="struct arrow::util::display::ArrayFormatter">ArrayFormatter</a>  
A string formatter for an [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html" class="struct" title="struct arrow::util::display::FormatOptions">FormatOptions</a>  
Options for formatting arrays

<a href="https://docs.rs/arrow/latest/arrow/util/display/struct.ValueFormatter.html" class="struct" title="struct arrow::util::display::ValueFormatter">ValueFormatter</a>  
Implements [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for a specific array value

## Enums<a href="https://docs.rs/arrow/latest/arrow/util/display/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/display/enum.DurationFormat.html" class="enum" title="enum arrow::util::display::DurationFormat">DurationFormat</a>  
Format for displaying durations

## Functions<a href="https://docs.rs/arrow/latest/arrow/util/display/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/display/fn.array_value_to_string.html" class="fn" title="fn arrow::util::display::array_value_to_string">array_value_to_string</a>  
Get the value at the given row in an array as a String.

<a href="https://docs.rs/arrow/latest/arrow/util/display/fn.lexical_to_string.html" class="fn" title="fn arrow::util::display::lexical_to_string">lexical_to_string</a>  
Converts numeric type to a `String`
