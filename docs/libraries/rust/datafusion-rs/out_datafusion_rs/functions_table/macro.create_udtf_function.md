# Macro create_udtf_function Copy item path

<a href="https://docs.rs/datafusion-functions-table/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions_table/lib.rs.html#43" class="src">Source</a>

``` rust
macro_rules! create_udtf_function {
    ($module:path, $name:expr) => { ... };
}
```

Expand description

Creates a singleton instance of a table function

- `$module`: A struct implementing `TableFunctionImpl` to create the function from
- `$name`: The name to give to the created function

This is used to ensure creating the list of `TableFunction` only happens once.
