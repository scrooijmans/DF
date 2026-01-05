# datafusion.substrait — Apache Arrow DataFusion  documentation
This module provides support for using substrait with datafusion.

For additional information about substrait, see [https://substrait.io/](https://substrait.io/) for more information about substrait.

Classes
-------------------------------------------


|Consumer|Generates a logical plan from a substrait plan.          |
|--------|---------------------------------------------------------|
|Plan    |A class representing an encodable substrait plan.        |
|Producer|Generates substrait plans from a logical plan.           |
|Serde   |Provides the Substrait serialization and deserialization.|


Module Contents
-----------------------------------------------------------

_class_ datafusion.substrait.Consumer

Generates a logical plan from a substrait plan.

_static_ from\_substrait\_plan(_ctx: [datafusion.context.SessionContext](about:blank/context/index.html#datafusion.context.SessionContext "datafusion.context.SessionContext")_, _plan: [Plan](#datafusion.substrait.Plan "datafusion.substrait.Plan")_) → [datafusion.plan.LogicalPlan](about:blank/plan/index.html#datafusion.plan.LogicalPlan "datafusion.plan.LogicalPlan")

Convert a Substrait plan to a DataFusion LogicalPlan.

Parameters:

*   **ctx** – SessionContext to use.
    
*   **plan** – Substrait plan to convert.
    

Returns:

LogicalPlan.

_class_ datafusion.substrait.Plan(_plan: datafusion.\_internal.substrait.Plan_)

A class representing an encodable substrait plan.

Create a substrait plan.

The user should not have to call this constructor directly. Rather, it should be created via [`Serde`](#datafusion.substrait.Serde "datafusion.substrait.Serde") or py:class:Producer classes in this module.

encode() → bytes

Encode the plan to bytes.

Returns:

Encoded plan.

plan\_internal

_class_ datafusion.substrait.Producer

Generates substrait plans from a logical plan.

_static_ to\_substrait\_plan(_logical\_plan: [datafusion.plan.LogicalPlan](about:blank/plan/index.html#datafusion.plan.LogicalPlan "datafusion.plan.LogicalPlan")_, _ctx: [datafusion.context.SessionContext](about:blank/context/index.html#datafusion.context.SessionContext "datafusion.context.SessionContext")_) → [Plan](#datafusion.substrait.Plan "datafusion.substrait.Plan")

Convert a DataFusion LogicalPlan to a Substrait plan.

Parameters:

*   **logical\_plan** – LogicalPlan to convert.
    
*   **ctx** – SessionContext to use.
    

Returns:

Substrait plan.

_class_ datafusion.substrait.Serde

Provides the `Substrait` serialization and deserialization.

_static_ deserialize(_path: str | pathlib.Path_) → [Plan](#datafusion.substrait.Plan "datafusion.substrait.Plan")

Deserialize a Substrait plan from a file.

Parameters:

**path** – Path to read the Substrait plan from.

Returns:

Substrait plan.

_static_ deserialize\_bytes(_proto\_bytes: bytes_) → [Plan](#datafusion.substrait.Plan "datafusion.substrait.Plan")

Deserialize a Substrait plan from bytes.

Parameters:

**proto\_bytes** – Bytes to read the Substrait plan from.

Returns:

Substrait plan.

_static_ serialize(_sql: str_, _ctx: [datafusion.context.SessionContext](about:blank/context/index.html#datafusion.context.SessionContext "datafusion.context.SessionContext")_, _path: str | pathlib.Path_) → None

Serialize a SQL query to a Substrait plan and write it to a file.

Parameters:

*   **sql** – SQL query to serialize.
    
*   **ctx** – SessionContext to use.
    
*   **path** – Path to write the Substrait plan to.
    

_static_ serialize\_bytes(_sql: str_, _ctx: [datafusion.context.SessionContext](about:blank/context/index.html#datafusion.context.SessionContext "datafusion.context.SessionContext")_) → bytes

Serialize a SQL query to a Substrait plan as bytes.

Parameters:

*   **sql** – SQL query to serialize.
    
*   **ctx** – SessionContext to use.
    

Returns:

Substrait plan as bytes.

_static_ serialize\_to\_plan(_sql: str_, _ctx: [datafusion.context.SessionContext](about:blank/context/index.html#datafusion.context.SessionContext "datafusion.context.SessionContext")_) → [Plan](#datafusion.substrait.Plan "datafusion.substrait.Plan")

Serialize a SQL query to a Substrait plan.

Args: sql: SQL query to serialize. ctx: SessionContext to use.

Returns:

Substrait plan.