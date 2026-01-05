# Enum JsonFormat Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/json/mod.rs.html#94" class="src">Source</a>

``` rust
pub enum JsonFormat {
    Json,
    JsonLines,
}
```

Available on **crate feature `polars-io`** only.

Expand description

The format to use to write the DataFrame to JSON: `Json` (a JSON array) or `JsonLines` (each row output on a separate line).

In either case, each row is serialized as a JSON object whose keys are the column names and whose values are the row’s corresponding values.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html#variant.Json" class="anchor">§</a>

### Json

A single JSON array containing each DataFrame row as an object. The length of the array is the number of rows in the DataFrame.

Use this to create valid JSON that can be deserialized back into an array in one fell swoop.

<a href="https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html#variant.JsonLines" class="anchor">§</a>

### JsonLines

Each DataFrame row is serialized as a JSON object on a separate line. The number of lines in the output is the number of rows in the DataFrame.

The [JSON Lines](https://jsonlines.org) format makes it easy to read records in a streaming fashion, one (line) at a time. But the output in its entirety is not valid JSON; only the individual lines are.

It is recommended to use the file extension `.jsonl` when saving as JSON Lines.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/enum.JsonFormat.html#blanket-implementations" class="anchor">§</a>
