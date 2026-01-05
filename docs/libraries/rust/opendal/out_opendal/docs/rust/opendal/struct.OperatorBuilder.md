# Struct OperatorBuilder Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/types/operator/builder.rs.html#466-468" class="src">Source</a>

``` rust
pub struct OperatorBuilder<A: Access> { /* private fields */ }
```

Expand description

OperatorBuilder is a typed builder to build an Operator.

## <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#notes" class="doc-anchor">Â§</a>Notes

OpenDAL uses static dispatch internally and only performs dynamic dispatch at the outmost type erase layer. OperatorBuilder is the only public API provided by OpenDAL come with generic parameters.

Itâ€™s required to call `finish` after the operator built.

## <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#examples" class="doc-anchor">Â§</a>Examples

For users who want to support many services, we can build a helper function like the following:

``` rust
use std::collections::HashMap;

use opendal::layers::LoggingLayer;
use opendal::layers::RetryLayer;
use opendal::services;
use opendal::Builder;
use opendal::Operator;
use opendal::Result;
use opendal::Scheme;

fn init_service<B: Builder>(cfg: HashMap<String, String>) -> Result<Operator> {
    let op = Operator::from_map::<B>(cfg)?
        .layer(LoggingLayer::default())
        .layer(RetryLayer::new())
        .finish();

    Ok(op)
}

async fn init(scheme: Scheme, cfg: HashMap<String, String>) -> Result<()> {
    let _ = match scheme {
        Scheme::Memory => init_service::<services::Memory>(cfg)?,
        _ => todo!(),
    };

    Ok(())
}
```

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#impl-OperatorBuilder%3CA%3E" class="anchor">Â§</a>

### impl\<A: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html" class="struct" title="struct opendal::OperatorBuilder">OperatorBuilder</a>\<A\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#method.new" class="fn">new</a>(accessor: A) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html" class="struct" title="struct opendal::OperatorBuilder">OperatorBuilder</a>\<impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Access.html" class="trait" title="trait opendal::raw::Access">Access</a>\>

Create a new operator builder.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#method.layer" class="fn">layer</a>\<L: <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html" class="trait" title="trait opendal::raw::Layer">Layer</a>\<A\>\>(self, layer: L) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html" class="struct" title="struct opendal::OperatorBuilder">OperatorBuilder</a>\<L::<a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.Layer.html#associatedtype.LayeredAccess" class="associatedtype" title="type opendal::raw::Layer::LayeredAccess">LayeredAccess</a>\>

Create a new layer with static dispatch.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#notes-1" class="doc-anchor">Â§</a>Notes

`OperatorBuilder::layer()` is using static dispatch which is zero cost. `Operator::layer()` is using dynamic dispatch which has a bit runtime overhead with an extra vtable lookup and unable to inline.

Itâ€™s always recommended to use `OperatorBuilder::layer()` instead.

##### <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#examples-1" class="doc-anchor">Â§</a>Examples

``` rust
use opendal::layers::LoggingLayer;
use opendal::services::Memory;
use opendal::Operator;

let op = Operator::new(Memory::default())?
    .layer(LoggingLayer::default())
    .finish();
// All operations will go through the new_layer
let _ = op.read("test_file").await?;
```

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/struct.Operator.html" class="struct" title="struct opendal::Operator">Operator</a>

Finish the building to construct an Operator.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/struct.OperatorBuilder.html#blanket-implementations" class="anchor">Â§</a>
