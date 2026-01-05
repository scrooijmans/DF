# Type Alias OperatorFactory Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/operator/registry.rs.html#26" class="src">Source</a>

``` rust
pub type OperatorFactory = fn(&OperatorUri) -> Result<Operator>;
```

Expand description

Factory signature used to construct [`Operator`](https://opendal.apache.org/docs/rust/opendal/struct.Operator.html "struct opendal::Operator") from a URI and extra options.
