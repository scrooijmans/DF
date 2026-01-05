# Expression Plugins - Polars user guide
Expression plugins are the preferred way to create user defined functions. They allow you to compile a Rust function and register that as an expression into the Polars library. The Polars engine will dynamically link your function at runtime and your expression will run almost as fast as native expressions. Note that this works without any interference of Python and thus no GIL contention.

They will benefit from the same benefits default expressions have:

*   Optimization
*   Parallelism
*   Rust native performance

To get started we will see what is needed to create a custom expression.

Our first custom expression: Pig Latin
--------------------------------------

For our first expression we are going to create a pig latin converter. Pig latin is a silly language where in every word the first letter is removed, added to the back and finally "ay" is added. So the word "pig" would convert to "igpay".

We could of course already do that with expressions, e.g. `col("name").str.slice(1) + col("name").str.slice(0, 1) + "ay"`, but a specialized function for this would perform better and allows us to learn about the plugins.

### Setting up

We start with a new library as the following `Cargo.toml` file

```
[package]
name = "expression_lib"
version = "0.1.0"
edition = "2021"

[lib]
name = "expression_lib"
crate-type = ["cdylib"]

[dependencies]
polars = { version = "*" }
pyo3 = { version = "*", features = ["extension-module", "abi3-py38"] }
pyo3-polars = { version = "*", features = ["derive"] }
serde = { version = "*", features = ["derive"] }

```


### Writing the expression

In this library we create a helper function that converts a `&str` to pig-latin, and we create the function that we will expose as an expression. To expose a function we must add the `#[polars_expr(output_type=DataType)]` attribute and the function must always accept `inputs: &[Series]` as its first argument.

```
// src/expressions.rs
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use std::fmt::Write;

fn pig_latin_str(value: &str, output: &mut String) {
    if let Some(first_char) = value.chars().next() {
        write!(output, "{}{}ay", &value[1..], first_char).unwrap()
    }
}

#[polars_expr(output_type=String)]
fn pig_latinnify(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(pig_latin_str);
    Ok(out.into_series())
}

```


Note that we use `apply_into_string_amortized`, as opposed to `apply_values`, to avoid allocating a new string for each row. If your plugin takes in multiple inputs, operates elementwise, and produces a `String` output, then you may want to look at the `binary_elementwise_into_string_amortized` utility function in `polars::prelude::arity`.

This is all that is needed on the Rust side. On the Python side we must setup a folder with the same name as defined in the `Cargo.toml`, in this case "expression\_lib". We will create a folder in the same directory as our Rust `src` folder named `expression_lib` and we create an `expression_lib/__init__.py`. The resulting file structure should look something like this:

```
├── 📁 expression_lib/  # name must match "lib.name" in Cargo.toml
|   └── __init__.py
|
├── 📁src/
|   ├── lib.rs
|   └── expressions.rs
|
├── Cargo.toml
└── pyproject.toml

```


Then we create new expressions. The function name of our expression can be registered. Note that it is important that this name is correct, otherwise the main Polars package cannot resolve the function name. Furthermore we can set additional keyword arguments that explain to Polars how this expression behaves. In this case we tell Polars that this function is elementwise. This allows Polars to run this expression in batches. Whereas for other operations this would not be allowed, think for instance of a sort, or a slice.

```
# expression_lib/__init__.py
from pathlib import Path
from typing import TYPE_CHECKING

import polars as pl
from polars.plugins import register_plugin_function
from polars._typing import IntoExpr

PLUGIN_PATH = Path(__file__).parent

def pig_latinnify(expr: IntoExpr) -> pl.Expr:
    """Pig-latinnify expression."""
    return register_plugin_function(
        plugin_path=PLUGIN_PATH,
        function_name="pig_latinnify",
        args=expr,
        is_elementwise=True,
    )

```


We can then compile this library in our environment by installing `maturin` and running `maturin develop --release`.

And that's it. Our expression is ready to use!

```
import polars as pl
from expression_lib import pig_latinnify

df = pl.DataFrame(
    {
        "convert": ["pig", "latin", "is", "silly"],
    }
)
out = df.with_columns(pig_latin=pig_latinnify("convert"))

```


Alternatively, you can [register a custom namespace](https://docs.pola.rs/api/python/stable/reference/api/polars.api.register_expr_namespace.html#polars.api.register_expr_namespace), which enables you to create a `Expr.language` namespace, allowing users to write:

```
out = df.with_columns(
    pig_latin=pl.col("convert").language.pig_latinnify(),
)

```


Accepting kwargs
----------------

If you want to accept `kwargs` (keyword arguments) in a polars expression, all you have to do is define a Rust `struct` and make sure that it derives `serde::Deserialize`.

```
/// Provide your own kwargs struct with the proper schema and accept that type
/// in your plugin expression.
#[derive(Deserialize)]
pub struct MyKwargs {
    float_arg: f64,
    integer_arg: i64,
    string_arg: String,
    boolean_arg: bool,
}

/// If you want to accept `kwargs`. You define a `kwargs` argument
/// on the second position in you plugin. You can provide any custom struct that is deserializable
/// with the pickle protocol (on the Rust side).
#[polars_expr(output_type=String)]
fn append_kwargs(input: &[Series], kwargs: MyKwargs) -> PolarsResult<Series> {
    let input = &input[0];
    let input = input.cast(&DataType::String)?;
    let ca = input.str().unwrap();

    Ok(ca
        .apply_into_string_amortized(|val, buf| {
            write!(
                buf,
                "{}-{}-{}-{}-{}",
                val, kwargs.float_arg, kwargs.integer_arg, kwargs.string_arg, kwargs.boolean_arg
            )
                .unwrap()
        })
        .into_series())
}

```


On the Python side the kwargs can be passed when we register the plugin.

```
def append_args(
    expr: IntoExpr,
    float_arg: float,
    integer_arg: int,
    string_arg: str,
    boolean_arg: bool,
) -> pl.Expr:
    """
    This example shows how arguments other than `Series` can be used.
    """
    return register_plugin_function(
        plugin_path=PLUGIN_PATH,
        function_name="append_kwargs",
        args=expr,
        kwargs={
            "float_arg": float_arg,
            "integer_arg": integer_arg,
            "string_arg": string_arg,
            "boolean_arg": boolean_arg,
        },
        is_elementwise=True,
    )

```


Output data types
-----------------

Output data types of course don't have to be fixed. They often depend on the input types of an expression. To accommodate this you can provide the `#[polars_expr()]` macro with an `output_type_func` argument that points to a function. This function can map input fields `&[Field]` to an output `Field` (name and data type).

In the snippet below is an example where we use the utility `FieldsMapper` to help with this mapping.

```
use polars_plan::dsl::FieldsMapper;

fn haversine_output(input_fields: &[Field]) -> PolarsResult<Field> {
    FieldsMapper::new(input_fields).map_to_float_dtype()
}

#[polars_expr(output_type_func=haversine_output)]
fn haversine(inputs: &[Series]) -> PolarsResult<Series> {
    let out = match inputs[0].dtype() {
        DataType::Float32 => {
            let start_lat = inputs[0].f32().unwrap();
            let start_long = inputs[1].f32().unwrap();
            let end_lat = inputs[2].f32().unwrap();
            let end_long = inputs[3].f32().unwrap();
            crate::distances::naive_haversine(start_lat, start_long, end_lat, end_long)?
                .into_series()
        }
        DataType::Float64 => {
            let start_lat = inputs[0].f64().unwrap();
            let start_long = inputs[1].f64().unwrap();
            let end_lat = inputs[2].f64().unwrap();
            let end_long = inputs[3].f64().unwrap();
            crate::distances::naive_haversine(start_lat, start_long, end_lat, end_long)?
                .into_series()
        }
        _ => polars_bail!(InvalidOperation: "only supported for float types"),
    };
    Ok(out)
}

```


That's all you need to know to get started. Take a look at [this repo](https://github.com/pola-rs/pyo3-polars/tree/main/example/derive_expression) to see how this all fits together, and at [this tutorial](https://marcogorelli.github.io/polars-plugins-tutorial/) to gain a more thorough understanding.

# IO Plugins - Polars user guide
Besides [expression plugins](../expr_plugins/), we also support IO plugins. These allow you to register different file formats as sources to the Polars engines. Because sources can move data zero copy via Arrow FFI and sources can produce large chunks of data before returning, we've decided to interface to IO plugins via Python for now, as we don't think the short time the GIL is needed should lead to any contention.

E.g. an IO source can read their dataframe's in rust and only at the rendez-vous move the data zero-copy having only a short time the GIL is needed.

Use case
--------

You want IO plugins if you have a source file not supported by Polars and you want to benefit from optimizations like projection pushdown, predicate pushdown, early stopping and support of our streaming engine.

Example
-------

So let's write a simple, very bad, custom CSV source and register that as an IO plugin. I want to stress that this is a very bad example and is only given for learning purposes.

First we define some imports we need:

```
# Use python for csv parsing.
import csv
import polars as pl
# Used to register a new generator on every instantiation.
from polars.io.plugins import register_io_source
from typing import Iterator
import io

```


### Parsing the schema

Every `scan` function in Polars has to be able to provide the schema of the data it reads. For this simple csv parser we will always read the data as `pl.String`. The only thing that differs are the field names and the number of fields.

```
def parse_schema(csv_str: str) -> pl.Schema:
    first_line = csv_str.split("\n")
[0]

    return pl.Schema({k: pl.String for k in first_line.split(",")})

```


If we run this with small csv file `"a,b,c\n1,2,3"` we get the schema: `Schema([('a', String), ('b', String), ('c', String)])`.

```
>>> print(parse_schema("a,b,c\n1,2,3"))
Schema([('a', String), ('b', String), ('c', String)])

```


### Writing the source

Next up is the actual source. For this we create an outer and an inner function. The outer function `my_scan_csv` is the user facing function. This function will accept the file name and other potential arguments you would need for reading the source. For csv files, these arguments could be "delimiter", "quote\_char" and such.

This outer function calls `register_io_source` which accepts a `callable` and a `schema`. The schema is the Polars schema of the complete source file (independent of projection pushdown).

The callable is a function that will return a generator that produces `pl.DataFrame` objects.

The arguments of this function are predefined and this function must accept:

*   `with_columns`

Columns that are projected. The reader must project these columns if applied

*   `predicate`

Polars expression. The reader must filter their rows accordingly.

*   `n_rows`

Materialize only n rows from the source. The reader can stop when `n_rows` are read.

*   `batch_size`

A hint of the ideal batch size the reader's generator must produce.

The inner function is the actual implementation of the IO source and can also call into Rust/C++ or wherever the IO plugin is written. If you want to see an IO source implemented in Rust, take a look at our [plugins repository](https://github.com/pola-rs/pyo3-polars/tree/main/example/io_plugin).

```
def my_scan_csv(csv_str: str) -> pl.LazyFrame:
    schema = parse_schema(csv_str)

    def source_generator(
        with_columns: list[str] | None,
        predicate: pl.Expr | None,
        n_rows: int | None,
        batch_size: int | None,
    ) -> Iterator[pl.DataFrame]:
        """
        Generator function that creates the source.
        This function will be registered as IO source.
        """
        if batch_size is None:
            batch_size = 100

        # Initialize the reader.
        reader = csv.reader(io.StringIO(csv_str), delimiter=',')
        # Skip the header.
        _ = next(reader)

        # Ensure we don't read more rows than requested from the engine
        while n_rows is None or n_rows > 0:
            if n_rows is not None:
                batch_size = min(batch_size, n_rows)

            rows = []

            for _ in range(batch_size):
                try:
                    row = next(reader)
                except StopIteration:
                    n_rows = 0
                    break
                rows.append(row)

            df = pl.from_records(rows, schema=schema, orient="row")
            n_rows -= df.height

            # If we would make a performant reader, we would not read these
            # columns at all.
            if with_columns is not None:
                df = df.select(with_columns)

            # If the source supports predicate pushdown, the expression can be parsed
            # to skip rows/groups.
            if predicate is not None:
                df = df.filter(predicate)

            yield df

    return register_io_source(io_source=source_generator, schema=schema)

```


### Taking it for a (very slow) spin

Finally we can test our source:

```
csv_str1 = """a,b,c,d
1,2,3,4
9,10,11,2
1,2,3,4
1,122,3,4"""

print(my_scan_csv(csv_str1).collect())


csv_str2 = """a,b
1,2
9,10
1,2
1,122"""

print(my_scan_csv(csv_str2).head(2).collect())

```


Running the script above would print the following output to the console:

```
shape: (4, 4)
┌─────┬─────┬─────┬─────┐
│ a   ┆ b   ┆ c   ┆ d   │
│ --- ┆ --- ┆ --- ┆ --- │
│ str ┆ str ┆ str ┆ str │
╞═════╪═════╪═════╪═════╡
│ 1   ┆ 2   ┆ 3   ┆ 4   │
│ 9   ┆ 10  ┆ 11  ┆ 2   │
│ 1   ┆ 2   ┆ 3   ┆ 4   │
│ 1   ┆ 122 ┆ 3   ┆ 4   │
└─────┴─────┴─────┴─────┘
shape: (2, 2)
┌─────┬─────┐
│ a   ┆ b   │
│ --- ┆ --- │
│ str ┆ str │
╞═════╪═════╡
│ 1   ┆ 2   │
│ 9   ┆ 10  │
└─────┴─────┘

```


Further reading
---------------

*   [Rust example (distribution source)](https://github.com/pola-rs/pyo3-polars/tree/main/example/io_plugin)