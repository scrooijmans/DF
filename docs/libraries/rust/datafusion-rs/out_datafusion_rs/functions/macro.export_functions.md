# Macro export_functions Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/macros.rs.html#44" class="src">Source</a>

``` rust
macro_rules! export_functions {
    ($(($FUNC:ident, $DOC:expr, $($arg:tt)*)),*) => { ... };
    (single $FUNC:ident, $DOC:expr, $arg:ident,) => { ... };
    (single $FUNC:ident, $DOC:expr, $($arg:ident)*) => { ... };
}
```

Expand description

macro that exports a list of function names as:

1.  individual functions in an `expr_fn` module
2.  a single function that returns a list of all functions

Equivalent to

``` text
pub mod expr_fn {
    use super::*;
    /// Return encode(arg)
    pub fn encode(args: Vec<Expr>) -> Expr {
        super::encode().call(args)
    }
 ...
/// Return a list of all functions in this package
pub(crate) fn functions() -> Vec<Arc<ScalarUDF>> {
    vec![
      encode(),
      decode()
   ]
}
```

Exported functions accept:

- `Vec<Expr>` argument (single argument followed by a comma)
- Variable number of `Expr` arguments (zero or more arguments, must be without commas)
