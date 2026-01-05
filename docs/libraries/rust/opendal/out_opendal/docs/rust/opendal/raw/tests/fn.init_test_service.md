# Function init_test_service Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/tests/utils.rs.html#38-81" class="src">Source</a>

``` rust
pub fn init_test_service() -> Result<Option<Operator>>
```

Available on **crate feature `tests`** only.

Expand description

Init a service with given scheme.

- Load scheme from `OPENDAL_TEST`
- Construct a new Operator with given root.
- Else, returns a `None` to represent no valid config for operator.
