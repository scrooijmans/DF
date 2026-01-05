Title: qdrant\_client.qdrant\_client module — Qdrant Client documentation                         

*   Get Started
*   Blog
*   Community

Linkedin Twitter Discord

*   Qdrant
*   GitHub

Table of Contents

*   Docs \>
*   qdrant\_client.qdrant\_client module

Shortcuts

qdrant\_client.qdrant\_client module¶
=====================================

_class_ QdrantClient(_location: Optional\[str\] \= None_, _url: Optional\[str\] \= None_, _port: Optional\[int\] \= 6333_, _grpc\_port: int \= 6334_, _prefer\_grpc: bool \= False_, _https: Optional\[bool\] \= None_, _api\_key: Optional\[str\] \= None_, _prefix: Optional\[str\] \= None_, _timeout: Optional\[int\] \= None_, _host: Optional\[str\] \= None_, _path: Optional\[str\] \= None_, _force\_disable\_check\_same\_thread: bool \= False_, _grpc\_options: Optional\[dict\[str, Any\]\] \= None_, _auth\_token\_provider: Optional\[Union\[Callable\[\[\], str\], Callable\[\[\], Awaitable\[str\]\]\]\] \= None_, _cloud\_inference: bool \= False_, _local\_inference\_batch\_size: Optional\[int\] \= None_, _check\_compatibility: bool \= True_, _\*\*kwargs: Any_)\[source\]¶

Bases: `QdrantFastembedMixin`

Entry point to communicate with Qdrant service via REST or gRPC API.

It combines interface classes and endpoint implementation. Additionally, it provides custom implementations for frequently used methods like initial collection upload.

All methods in QdrantClient accept both gRPC and REST structures as an input. Conversion will be performed automatically.

Note

This module methods are wrappers around generated client code for gRPC and REST methods. If you need lower-level access to generated clients, use following properties:

*   `QdrantClient.grpc_points`

*   `QdrantClient.grpc_collections`

*   `QdrantClient.rest`

Note

If you need async, please consider using Async Implementations of QdrantClient.

*   `qdrant_client.async_qdrant_client`

Parameters:

*   **location** – If “:memory:” - use in-memory Qdrant instance. If str - use it as a url parameter. If None - use default values for host and port.

*   **url** – either host or str of “Optional\[scheme\], host, Optional\[port\], Optional\[prefix\]”. Default: None

*   **port** – Port of the REST API interface. Default: 6333

*   **grpc\_port** – Port of the gRPC interface. Default: 6334

*   **prefer\_grpc** – If true - use gPRC interface whenever possible in custom methods.

*   **https** – If true - use HTTPS(SSL) protocol. Default: None

*   **api\_key** – API key for authentication in Qdrant Cloud. Default: None

*   **prefix** – If not None - add prefix to the REST URL path. Example: service/v1 will result in http://localhost:6333/service/v1/{qdrant-endpoint} for REST API. Default: None

*   **timeout** – Timeout for REST and gRPC API requests. Default: 5 seconds for REST and unlimited for gRPC

*   **host** – Host name of Qdrant service. If url and host are None, set to ‘localhost’. Default: None

*   **path** – Persistence path for QdrantLocal. Default: None

*   **force\_disable\_check\_same\_thread** – For QdrantLocal, force disable check\_same\_thread. Default: False Only use this if you can guarantee that you can resolve the thread safety outside QdrantClient.

*   **auth\_token\_provider** – Callback function to get Bearer access token. If given, the function will be called before each request to get the token.

*   **check\_compatibility** – If true - check compatibility with the server version. Default: true

*   **\*\*kwargs** – Additional arguments passed directly into REST client initialization

batch\_update\_points(_collection\_name: str_, _update\_operations: Sequence\[Union\[UpsertOperation, DeleteOperation, SetPayloadOperation, OverwritePayloadOperation, DeletePayloadOperation, ClearPayloadOperation, UpdateVectorsOperation, DeleteVectorsOperation\]\]_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _\*\*kwargs: Any_) → list\[UpdateResult\]\[source\]¶

Batch update points in the collection.

Parameters:

*   **collection\_name** – Name of the collection

*   **update\_operations** – List of update operations

*   **wait** – Await for the results to be processed. - If true, result will be returned only when all changes are applied - If false, result will be returned immediately after the confirmation of receiving.

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

Returns:

Operation results

clear\_payload(_collection\_name: str_, _points\_selector: Union\[list\[Union\[int, str, points\_pb2.PointId\]\], Filter, Filter, PointIdsList, FilterSelector, PointsSelector\]_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Delete all payload for selected points

Parameters:

*   **collection\_name** – Name of the collection

*   **wait** – Await for the results to be processed. - If true, result will be returned only when all changes are applied - If false, result will be returned immediately after the confirmation of receiving.

*   **points\_selector** – List of affected points, filter or points selector. Example: - points=\[1, 2, 3, “cd3b53f0-11a7-449f-bc50-d06310e7ed90”\] - points=Filter(must=\[FieldCondition(key=’rand\_number’, range=Range(gte=0.7))\])

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method.

Returns:

Operation result

close(_grpc\_grace: Optional\[float\] \= None_, _\*\*kwargs: Any_) → None\[source\]¶

Closes the connection to Qdrant

Parameters:

**grpc\_grace** – Grace period for gRPC connection close. Default: None

collection\_exists(_collection\_name: str_, _\*\*kwargs: Any_) → bool\[source\]¶

Check whether collection already exists

Parameters:

**collection\_name** – Name of the collection

Returns:

True if collection exists, False if not

count(_collection\_name: str_, _count\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _exact: bool \= True_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → CountResult\[source\]¶

Count points in the collection.

Count points in the collection matching the given filter.

Parameters:

*   **collection\_name** – name of the collection to count points in

*   **count\_filter** – filtering conditions

*   **exact** – If True - provide the exact count of points matching the filter. If False - provide the approximate count of points matching the filter. Works faster.

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **timeout** – Overrides global timeout for this operation. Unit is seconds.

Returns:

Amount of points in the collection matching the filter.

create\_collection(_collection\_name: str_, _vectors\_config: Optional\[Union\[VectorParams, Mapping\[str, VectorParams\]\]\] \= None_, _sparse\_vectors\_config: Optional\[Mapping\[str, SparseVectorParams\]\] \= None_, _shard\_number: Optional\[int\] \= None_, _sharding\_method: Optional\[ShardingMethod\] \= None_, _replication\_factor: Optional\[int\] \= None_, _write\_consistency\_factor: Optional\[int\] \= None_, _on\_disk\_payload: Optional\[bool\] \= None_, _hnsw\_config: Optional\[Union\[HnswConfigDiff, HnswConfigDiff\]\] \= None_, _optimizers\_config: Optional\[Union\[OptimizersConfigDiff, OptimizersConfigDiff\]\] \= None_, _wal\_config: Optional\[Union\[WalConfigDiff, WalConfigDiff\]\] \= None_, _quantization\_config: Optional\[Union\[ScalarQuantization, ProductQuantization, BinaryQuantization, QuantizationConfig\]\] \= None_, _init\_from: Optional\[Union\[InitFrom, str\]\] \= None_, _timeout: Optional\[int\] \= None_, _strict\_mode\_config: Optional\[StrictModeConfig\] \= None_, _\*\*kwargs: Any_) → bool\[source\]¶

Create empty collection with given parameters

Parameters:

*   **collection\_name** – Name of the collection to recreate

*   **vectors\_config** – Configuration of the vector storage. Vector params contains size and distance for the vector storage. If dict is passed, service will create a vector storage for each key in the dict. If single VectorParams is passed, service will create a single anonymous vector storage.

*   **sparse\_vectors\_config** – Configuration of the sparse vector storage. The service will create a sparse vector storage for each key in the dict.

*   **shard\_number** – Number of shards in collection. Default is 1, minimum is 1.

*   **sharding\_method** – Defines strategy for shard creation. Option auto (default) creates defined number of shards automatically. Data will be distributed between shards automatically. After creation, shards could be additionally replicated, but new shards could not be created. Option custom allows to create shards manually, each shard should be created with assigned unique shard\_key. Data will be distributed between based on shard\_key value.

*   **replication\_factor** – Replication factor for collection. Default is 1, minimum is 1. Defines how many copies of each shard will be created. Have effect only in distributed mode.

*   **write\_consistency\_factor** – Write consistency factor for collection. Default is 1, minimum is 1. Defines how many replicas should apply the operation for us to consider it successful. Increasing this number will make the collection more resilient to inconsistencies, but will also make it fail if not enough replicas are available. Does not have any performance impact. Have effect only in distributed mode.

*   **on\_disk\_payload** – If true - point\`s payload will not be stored in memory. It will be read from the disk every time it is requested. This setting saves RAM by (slightly) increasing the response time. Note: those payload values that are involved in filtering and are indexed - remain in RAM.

*   **hnsw\_config** – Params for HNSW index

*   **optimizers\_config** – Params for optimizer

*   **wal\_config** – Params for Write-Ahead-Log

*   **quantization\_config** – Params for quantization, if None - quantization will be disabled

*   **init\_from** – Use data stored in another collection to initialize this collection

*   **timeout** – Wait for operation commit timeout in seconds. If timeout is reached - request will return with service error.

*   **strict\_mode\_config** – Configure limitations for the collection, such as max size, rate limits, etc.

Returns:

Operation result

create\_full\_snapshot(_wait: bool \= True_, _\*\*kwargs: Any_) → Optional\[SnapshotDescription\]\[source\]¶

Create snapshot for a whole storage.

Parameters:

**wait** –

Await for the snapshot to be created.

*   If true, result will be returned only when the snapshot is created

*   If false, result will be returned immediately after the confirmation of receiving.

Returns:

Snapshot description

create\_payload\_index(_collection\_name: str_, _field\_name: str_, _field\_schema: Optional\[Union\[PayloadSchemaType, KeywordIndexParams, IntegerIndexParams, FloatIndexParams, GeoIndexParams, TextIndexParams, BoolIndexParams, DatetimeIndexParams, UuidIndexParams, int, PayloadIndexParams\]\] \= None_, _field\_type: Optional\[Union\[PayloadSchemaType, KeywordIndexParams, IntegerIndexParams, FloatIndexParams, GeoIndexParams, TextIndexParams, BoolIndexParams, DatetimeIndexParams, UuidIndexParams, int, PayloadIndexParams\]\] \= None_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Creates index for a given payload field. Indexed fields allow to perform filtered search operations faster.

Parameters:

*   **collection\_name** – Name of the collection

*   **field\_name** – Name of the payload field

*   **field\_schema** – Type of data to index

*   **field\_type** – Same as field\_schema, but deprecated

*   **wait** –

Await for the results to be processed.

*   If true, result will be returned only when all changes are applied

*   If false, result will be returned immediately after the confirmation of receiving.

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

Returns:

Operation Result

create\_shard\_key(_collection\_name: str_, _shard\_key: Union\[int, str\]_, _shards\_number: Optional\[int\] \= None_, _replication\_factor: Optional\[int\] \= None_, _placement: Optional\[list\[int\]\] \= None_, _\*\*kwargs: Any_) → bool\[source\]¶

Create shard key for collection.

Only works for collections with custom sharding method.

Parameters:

*   **collection\_name** – Name of the collection

*   **shard\_key** – Shard key to create

*   **shards\_number** – How many shards to create for this key

*   **replication\_factor** – Replication factor for this key

*   **placement** – List of peers to place shards on. If None - place on all peers.

Returns:

Operation result

create\_shard\_snapshot(_collection\_name: str_, _shard\_id: int_, _wait: bool \= True_, _\*\*kwargs: Any_) → Optional\[SnapshotDescription\]\[source\]¶

Create snapshot for a given shard.

Parameters:

*   **collection\_name** – Name of the collection

*   **shard\_id** – Index of the shard

*   **wait** –

Await for the snapshot to be created.

*   If true, result will be returned only when the snapshot is created.

*   If false, result will be returned immediately after the confirmation of receiving.

Returns:

Snapshot description

create\_snapshot(_collection\_name: str_, _wait: bool \= True_, _\*\*kwargs: Any_) → Optional\[SnapshotDescription\]\[source\]¶

Create snapshot for a given collection.

Parameters:

*   **collection\_name** – Name of the collection

*   **wait** –

Await for the snapshot to be created.

*   If true, result will be returned only when a snapshot is created

*   If false, result will be returned immediately after the confirmation of receiving.

Returns:

Snapshot description

delete(_collection\_name: str_, _points\_selector: Union\[list\[Union\[int, str, points\_pb2.PointId\]\], Filter, Filter, PointIdsList, FilterSelector, PointsSelector\]_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Deletes selected points from collection

Parameters:

*   **collection\_name** – Name of the collection

*   **wait** –

Await for the results to be processed.

*   If true, result will be returned only when all changes are applied

*   If false, result will be returned immediately after the confirmation of receiving.

*   **points\_selector** –

Selects points based on list of IDs or filter. Examples:

*   points=\[1, 2, 3, “cd3b53f0-11a7-449f-bc50-d06310e7ed90”\]

*   points=Filter(must=\[FieldCondition(key=’rand\_number’, range=Range(gte=0.7))\])

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method.

Returns:

Operation result

delete\_collection(_collection\_name: str_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → bool\[source\]¶

Removes collection and all it’s data

Parameters:

*   **collection\_name** – Name of the collection to delete

*   **timeout** – Wait for operation commit timeout in seconds. If timeout is reached - request will return with service error.

Returns:

Operation result

delete\_full\_snapshot(_snapshot\_name: str_, _wait: bool \= True_, _\*\*kwargs: Any_) → Optional\[bool\]\[source\]¶

Delete snapshot for a whole storage.

Parameters:

*   **snapshot\_name** – Snapshot name

*   **wait** –

Await for the snapshot to be deleted.

*   If true, result will be returned only when the snapshot is deleted

*   If false, result will be returned immediately after the confirmation of receiving.

Returns:

True if snapshot was deleted

delete\_payload(_collection\_name: str_, _keys: Sequence\[str\]_, _points: Union\[list\[Union\[int, str, points\_pb2.PointId\]\], Filter, Filter, PointIdsList, FilterSelector, PointsSelector\]_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Remove values from point’s payload

Parameters:

*   **collection\_name** – Name of the collection

*   **wait** –

Await for the results to be processed.

*   If true, result will be returned only when all changes are applied

*   If false, result will be returned immediately after the confirmation of receiving.

*   **keys** – List of payload keys to remove

*   **points** –

List of affected points, filter or points selector. .. rubric:: Example

*   points=\[1, 2, 3, “cd3b53f0-11a7-449f-bc50-d06310e7ed90”\]

*   points=Filter(must=\[FieldCondition(key=’rand\_number’, range=Range(gte=0.7))\])

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is downn

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method.

Returns:

Operation result

delete\_payload\_index(_collection\_name: str_, _field\_name: str_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Removes index for a given payload field.

Parameters:

*   **collection\_name** – Name of the collection

*   **field\_name** – Name of the payload field

*   **wait** –

Await for the results to be processed.

*   If true, result will be returned only when all changes are applied

*   If false, result will be returned immediately after the confirmation of receiving.

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

Returns:

Operation Result

delete\_shard\_key(_collection\_name: str_, _shard\_key: Union\[int, str\]_, _\*\*kwargs: Any_) → bool\[source\]¶

Delete shard key for collection.

Only works for collections with custom sharding method.

Parameters:

*   **collection\_name** – Name of the collection

*   **shard\_key** – Shard key to delete

Returns:

Operation result

delete\_shard\_snapshot(_collection\_name: str_, _shard\_id: int_, _snapshot\_name: str_, _wait: bool \= True_, _\*\*kwargs: Any_) → Optional\[bool\]\[source\]¶

Delete snapshot for a given shard.

Parameters:

*   **collection\_name** – Name of the collection

*   **shard\_id** – Index of the shard

*   **snapshot\_name** – Snapshot id

*   **wait** –

Await for the snapshot to be deleted.

*   If true, result will be returned only when the snapshot is deleted

*   If false, result will be returned immediately after the confirmation of receiving.

Returns:

True if snapshot was deleted

delete\_snapshot(_collection\_name: str_, _snapshot\_name: str_, _wait: bool \= True_, _\*\*kwargs: Any_) → Optional\[bool\]\[source\]¶

Delete snapshot for a given collection.

Parameters:

*   **collection\_name** – Name of the collection

*   **snapshot\_name** – Snapshot id

*   **wait** –

Await for the snapshot to be deleted.

*   If true, result will be returned only when the snapshot is deleted

*   If false, result will be returned immediately after the confirmation of receiving.

Returns:

True if snapshot was deleted

delete\_vectors(_collection\_name: str_, _vectors: Sequence\[str\]_, _points: Union\[list\[Union\[int, str, points\_pb2.PointId\]\], Filter, Filter, PointIdsList, FilterSelector, PointsSelector\]_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Delete specified vector from the collection. Does not affect payload.

Parameters:

*   **collection\_name** (_str_) – Name of the collection to delete vector from

*   **vectors** – List of names of the vectors to delete. Use “” to delete the default vector. At least one vector should be specified.

*   **points** (_Point_) –

Selects points based on list of IDs or filter Examples:

*   points=\[1, 2, 3, “cd3b53f0-11a7-449f-bc50-d06310e7ed90”\]

*   points=Filter(must=\[FieldCondition(key=’rand\_number’, range=Range(gte=0.7))\])

*   **wait** (_bool_) –

Await for the results to be processed.

*   If true, result will be returned only when all changes are applied

*   If false, result will be returned immediately after the confirmation of receiving.

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method.

Returns:

Operation result

discover(_collection\_name: str_, _target: Optional\[Union\[int, str, List\[float\], SparseVector, TargetVector\]\] \= None_, _context: Optional\[Sequence\[Union\[ContextExamplePair, ContextExamplePair\]\]\] \= None_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _search\_params: Optional\[Union\[SearchParams, SearchParams\]\] \= None_, _limit: int \= 10_, _offset: int \= 0_, _with\_payload: Union\[bool, list\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, list\[str\]\] \= False_, _using: Optional\[str\] \= None_, _lookup\_from: Optional\[Union\[LookupLocation, LookupLocation\]\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → list\[ScoredPoint\]\[source\]¶

Use context and a target to find the most similar points, constrained by the context.

Parameters:

*   **collection\_name** – Collection to discover in

*   **target** –

Look for vectors closest to this.

When using the target (with or without context), the integer part of the score represents the rank with respect to the context, while the decimal part of the score relates to the distance to the target.

*   **context** –

Pairs of { positive, negative } examples to constrain the search.

When using only the context (without a target), a special search - called context search - is performed where pairs of points are used to generate a loss that guides the search towards the zone where most positive examples overlap. This means that the score minimizes the scenario of finding a point closer to a negative than to a positive part of a pair.

Since the score of a context relates to loss, the maximum score a point can get is 0.0, and it becomes normal that many points can have a score of 0.0.

For discovery search (when including a target), the context part of the score for each pair is calculated +1 if the point is closer to a positive than to a negative part of a pair, and -1 otherwise.

*   **query\_filter** – Look only for points which satisfies this conditions

*   **search\_params** – Additional search params

*   **limit** – Max number of result to return

*   **offset** – Offset of the first result to return. May be used to paginate results. Note: large offset values may cause performance issues.

*   **with\_payload** – Select which payload to return with the response. Default: None

*   **with\_vectors** – Whether to return the point vector with the result?

*   **using** – Define which vector to use for recommendation, if not specified - try to use default vector.

*   **lookup\_from** – The location used to lookup vectors. If not specified - use current collection. Note: the other collection should have the same vector size as the current collection.

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Returns:

List of discovered points with discovery or context scores, accordingly.

discover\_batch(_collection\_name: str_, _requests: Sequence\[Union\[DiscoverRequest, DiscoverPoints\]\]_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → list\[list\[ScoredPoint\]\]\[source\]¶

facet(_collection\_name: str_, _key: str_, _facet\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _limit: int \= 10_, _exact: bool \= False_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _timeout: Optional\[int\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → FacetResponse\[source\]¶

Facet counts for the collection. For a specific payload key, returns unique values along with their counts. Higher counts come first in the results.

Parameters:

*   **collection\_name** – Name of the collection

*   **key** – Payload field to facet

*   **facet\_filter** – Filter to apply

*   **limit** – Maximum number of hits to return

*   **exact** – If True - provide the exact count of points matching the filter. If False - provide the approximate count of points matching the filter. Works faster.

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

Returns:

Unique values in the facet and the amount of points that they cover.

get\_aliases(_\*\*kwargs: Any_) → CollectionsAliasesResponse\[source\]¶

Get all aliases

Returns:

All aliases of all collections

get\_collection(_collection\_name: str_, _\*\*kwargs: Any_) → CollectionInfo\[source\]¶

Get detailed information about specified existing collection

Parameters:

**collection\_name** – Name of the collection

Returns:

Detailed information about the collection

get\_collection\_aliases(_collection\_name: str_, _\*\*kwargs: Any_) → CollectionsAliasesResponse\[source\]¶

Get collection aliases

Parameters:

**collection\_name** – Name of the collection

Returns:

Collection aliases

get\_collections(_\*\*kwargs: Any_) → CollectionsResponse\[source\]¶

Get list name of all existing collections

Returns:

List of the collections

get\_locks(_\*\*kwargs: Any_) → LocksOption\[source\]¶

Get current locks state.

info() → VersionInfo\[source\]¶

Returns information about the running Qdrant instance like version and commit id

Returns:

Title, version and optionally commit info

list\_full\_snapshots(_\*\*kwargs: Any_) → list\[SnapshotDescription\]\[source\]¶

List all snapshots for a whole storage

Returns:

List of snapshots

list\_shard\_snapshots(_collection\_name: str_, _shard\_id: int_, _\*\*kwargs: Any_) → list\[SnapshotDescription\]\[source\]¶

List all snapshots of a given shard

Parameters:

*   **collection\_name** – Name of the collection

*   **shard\_id** – Index of the shard

Returns:

List of snapshots

list\_snapshots(_collection\_name: str_, _\*\*kwargs: Any_) → list\[SnapshotDescription\]\[source\]¶

List all snapshots for a given collection.

Parameters:

**collection\_name** – Name of the collection

Returns:

List of snapshots

lock\_storage(_reason: str_, _\*\*kwargs: Any_) → LocksOption\[source\]¶

Lock storage for writing.

migrate(_dest\_client: QdrantBase_, _collection\_names: Optional\[list\[str\]\] \= None_, _batch\_size: int \= 100_, _recreate\_on\_collision: bool \= False_) → None\[source\]¶

Migrate data from one Qdrant instance to another.

Parameters:

*   **dest\_client** – Destination Qdrant instance either in local or remote mode

*   **collection\_names** – List of collection names to migrate. If None - migrate all collections

*   **batch\_size** – Batch size to be in scroll and upsert operations during migration

*   **recreate\_on\_collision** – If True - recreate collection on destination if it already exists, otherwise raise ValueError exception

overwrite\_payload(_collection\_name: str_, _payload: Dict\[str, Any\]_, _points: Union\[list\[Union\[int, str, points\_pb2.PointId\]\], Filter, Filter, PointIdsList, FilterSelector, PointsSelector\]_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Overwrites payload of the specified points After this operation is applied, only the specified payload will be present in the point. The existing payload, even if the key is not specified in the payload, will be deleted.

Examples:

Set payload:

\# Overwrite payload value with key \`"key"\` to points 1, 2, 3.
\# If any other valid payload value exists - it will be deleted
qdrant\_client.overwrite\_payload(
collection\_name\="test\_collection",
wait\=True,
payload\={
"key": "value"
},
points\=\[1,2,3\]
)

Parameters:

*   **collection\_name** – Name of the collection

*   **wait** –

Await for the results to be processed.

*   If true, result will be returned only when all changes are applied

*   If false, result will be returned immediately after the confirmation of receiving.

*   **payload** – Key-value pairs of payload to assign

*   **points** –

List of affected points, filter or points selector. .. rubric:: Example

*   points=\[1, 2, 3, “cd3b53f0-11a7-449f-bc50-d06310e7ed90”\]

*   points=Filter(must=\[FieldCondition(key=’rand\_number’, range=Range(gte=0.7))\])

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method.

Returns:

Operation result

query\_batch\_points(_collection\_name: str_, _requests: Sequence\[Union\[QueryRequest, QueryPoints\]\]_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → list\[QueryResponse\]\[source\]¶

Perform any search, recommend, discovery, context search operations in batch, and mitigate network overhead

Parameters:

*   **collection\_name** – Name of the collection

*   **requests** – List of query requests

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Returns:

List of query responses

query\_points(_collection\_name: str_, _query: Optional\[Union\[int, str, PointId, list\[float\], list\[list\[float\]\], SparseVector, NearestQuery, RecommendQuery, DiscoverQuery, ContextQuery, OrderByQuery, FusionQuery, FormulaQuery, SampleQuery, ndarray\[tuple\[int, ...\], dtype\[Union\[bool, int8, int16, int32, int64, uint8, uint16, uint32, uint64, float16, float32, float64, longdouble\]\]\], Document, Image, InferenceObject\]\] \= None_, _using: Optional\[str\] \= None_, _prefetch: Optional\[Union\[Prefetch, list\[Prefetch\]\]\] \= None_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _search\_params: Optional\[Union\[SearchParams, SearchParams\]\] \= None_, _limit: int \= 10_, _offset: Optional\[int\] \= None_, _with\_payload: Union\[bool, Sequence\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, Sequence\[str\]\] \= False_, _score\_threshold: Optional\[float\] \= None_, _lookup\_from: Optional\[Union\[LookupLocation, LookupLocation\]\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → QueryResponse\[source\]¶

Universal endpoint to run any available operation, such as search, recommendation, discovery, context search.

Parameters:

*   **collection\_name** – Collection to search in

*   **query** – Query for the chosen search type operation. - If str - use string as UUID of the existing point as a search query. - If int - use integer as ID of the existing point as a search query. - If list\[float\] - use as a dense vector for nearest search. - If list\[list\[float\]\] - use as a multi-vector for nearest search. - If SparseVector - use as a sparse vector for nearest search. - If Query - use as a query for specific search type. - If NumpyArray - use as a dense vector for nearest search. - If Document - infer vector from the document text and use it for nearest search (requires fastembed package installed). - If None - return first limit points from the collection.

*   **prefetch** – prefetch queries to make a selection of the data to be used with the main query

*   **query\_filter** –

*   Exclude vectors which doesn’t fit given conditions.

*   If None - search among all vectors

*   **search\_params** – Additional search params

*   **limit** – How many results return

*   **offset** – Offset of the first result to return. May be used to paginate results. Note: large offset values may cause performance issues.

*   **with\_payload** –

*   Specify which stored payload should be attached to the result.

*   If True - attach all payload

*   If False - do not attach any payload

*   If List of string - include only specified fields

*   If PayloadSelector - use explicit rules

*   **with\_vectors** –

*   If True - Attach stored vector to the search result.

*   If False - Do not attach vector.

*   If List of string - include only specified fields

*   Default: False

*   **score\_threshold** – Define a minimal score threshold for the result. If defined, less similar results will not be returned. Score of the returned result might be higher or smaller than the threshold depending on the Distance function used. E.g. for cosine similarity only higher scores will be returned.

*   **using** – Name of the vectors to use for query. If None - use default vectors or provided in named vector structures.

*   **lookup\_from** –

Defines a location (collection and vector field name), used to lookup vectors for recommendations,

discovery and context queries.

If None - current collection will be used.

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Examples:

Search for closest points with a filter:

qdrant.query(
collection\_name\="test\_collection",
query\=\[1.0, 0.1, 0.2, 0.7\],
query\_filter\=Filter(
must\=\[
FieldCondition(
key\='color',
range\=Match(
value\="red"
)
)
\]
)
)

Returns:

QueryResponse structure containing list of found close points with similarity scores.

query\_points\_groups(_collection\_name: str_, _group\_by: str_, _query: Optional\[Union\[int, str, PointId, list\[float\], list\[list\[float\]\], SparseVector, NearestQuery, RecommendQuery, DiscoverQuery, ContextQuery, OrderByQuery, FusionQuery, FormulaQuery, SampleQuery, ndarray\[tuple\[int, ...\], dtype\[Union\[bool, int8, int16, int32, int64, uint8, uint16, uint32, uint64, float16, float32, float64, longdouble\]\]\], Document, Image, InferenceObject\]\] \= None_, _using: Optional\[str\] \= None_, _prefetch: Optional\[Union\[Prefetch, list\[Prefetch\]\]\] \= None_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _search\_params: Optional\[Union\[SearchParams, SearchParams\]\] \= None_, _limit: int \= 10_, _group\_size: int \= 3_, _with\_payload: Union\[bool, Sequence\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, Sequence\[str\]\] \= False_, _score\_threshold: Optional\[float\] \= None_, _with\_lookup: Optional\[Union\[str, WithLookup\]\] \= None_, _lookup\_from: Optional\[Union\[LookupLocation, LookupLocation\]\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → GroupsResult\[source\]¶

Universal endpoint to group on any available operation, such as search, recommendation, discovery, context search.

Parameters:

*   **collection\_name** – Collection to search in

*   **query** – Query for the chosen search type operation. - If str - use string as UUID of the existing point as a search query. - If int - use integer as ID of the existing point as a search query. - If list\[float\] - use as a dense vector for nearest search. - If list\[list\[float\]\] - use as a multi-vector for nearest search. - If SparseVector - use as a sparse vector for nearest search. - If Query - use as a query for specific search type. - If NumpyArray - use as a dense vector for nearest search. - If Document - infer vector from the document text and use it for nearest search (requires fastembed package installed). - If None - return first limit points from the collection.

*   **prefetch** – prefetch queries to make a selection of the data to be used with the main query

*   **query\_filter** –

*   Exclude vectors which doesn’t fit given conditions.

*   If None - search among all vectors

*   **search\_params** – Additional search params

*   **limit** – How many results return

*   **group\_size** – How many results return for each group

*   **group\_by** – Name of the payload field to group by. Field must be of type “keyword” or “integer”. Nested fields are specified using dot notation, e.g. “nested\_field.subfield”.

*   **with\_payload** –

*   Specify which stored payload should be attached to the result.

*   If True - attach all payload

*   If False - do not attach any payload

*   If List of string - include only specified fields

*   If PayloadSelector - use explicit rules

*   **with\_vectors** –

*   If True - Attach stored vector to the search result.

*   If False - Do not attach vector.

*   If List of string - include only specified fields

*   Default: False

*   **score\_threshold** – Define a minimal score threshold for the result. If defined, less similar results will not be returned. Score of the returned result might be higher or smaller than the threshold depending on the Distance function used. E.g. for cosine similarity only higher scores will be returned.

*   **using** – Name of the vectors to use for query. If None - use default vectors or provided in named vector structures.

*   **with\_lookup** – Look for points in another collection using the group ids. If specified, each group will contain a record from the specified collection with the same id as the group id. In addition, the parameter allows to specify which parts of the record should be returned, like in with\_payload and with\_vectors parameters.

*   **lookup\_from** – Defines a location (collection and vector field name), used to lookup vectors being referenced in the query as IDs. If None - current collection will be used.

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Examples:

Search for closest points and group results:

qdrant.query\_points\_groups(
collection\_name="test\_collection",
query=\[1.0, 0.1, 0.2, 0.7\],
group\_by="color",
group\_size=3,
)

Returns:
List of groups with not more than \`group\_size\` hits in each group.
Each group also contains an id of the group, which is the value of the payload field.

recommend(_collection\_name: str_, _positive: Optional\[Sequence\[Union\[int, str, List\[float\], SparseVector\]\]\] \= None_, _negative: Optional\[Sequence\[Union\[int, str, List\[float\], SparseVector\]\]\] \= None_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _search\_params: Optional\[Union\[SearchParams, SearchParams\]\] \= None_, _limit: int \= 10_, _offset: int \= 0_, _with\_payload: Union\[bool, list\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, list\[str\]\] \= False_, _score\_threshold: Optional\[float\] \= None_, _using: Optional\[str\] \= None_, _lookup\_from: Optional\[Union\[LookupLocation, LookupLocation\]\] \= None_, _strategy: Optional\[RecommendStrategy\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → list\[ScoredPoint\]\[source\]¶

Recommend points: search for similar points based on already stored in Qdrant examples.

Provide IDs of the stored points, and Qdrant will perform search based on already existing vectors. This functionality is especially useful for recommendation over existing collection of points.

Parameters:

*   **collection\_name** – Collection to search in

*   **positive** – List of stored point IDs or vectors, which should be used as reference for similarity search. If there is only one example - this request is equivalent to the regular search with vector of that point. If there are more than one example, Qdrant will attempt to search for similar to all of them. Recommendation for multiple vectors is experimental. Its behaviour may change depending on selected strategy.

*   **negative** – List of stored point IDs or vectors, which should be dissimilar to the search result. Negative examples is an experimental functionality. Its behaviour may change depending on selected strategy.

*   **query\_filter** –

*   Exclude vectors which doesn’t fit given conditions.

*   If None - search among all vectors

*   **search\_params** – Additional search params

*   **limit** – How many results return

*   **offset** – Offset of the first result to return. May be used to paginate results. Note: large offset values may cause performance issues.

*   **with\_payload** –

*   Specify which stored payload should be attached to the result.

*   If True - attach all payload

*   If False - do not attach any payload

*   If List of string - include only specified fields

*   If PayloadSelector - use explicit rules

*   **with\_vectors** –

*   If True - Attach stored vector to the search result.

*   If False - Do not attach vector.

*   If List of string - include only specified fields

*   Default: False

*   **score\_threshold** – Define a minimal score threshold for the result. If defined, less similar results will not be returned. Score of the returned result might be higher or smaller than the threshold depending on the Distance function used. E.g. for cosine similarity only higher scores will be returned.

*   **using** – Name of the vectors to use for recommendations. If None - use default vectors.

*   **lookup\_from** – Defines a location (collection and vector field name), used to lookup vectors for recommendations. If None - current collection will be used.

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **strategy** –

Strategy to use for recommendation. Strategy defines how to combine multiple examples into a recommendation query. Possible values:

*   ’average\_vector’ - calculates average vector of all examples and uses it for search

*   ’best\_score’ - finds the result which is closer to positive examples and further from negative

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Returns:

List of recommended points with similarity scores.

recommend\_batch(_collection\_name: str_, _requests: Sequence\[Union\[RecommendRequest, RecommendPoints\]\]_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → list\[list\[ScoredPoint\]\]\[source\]¶

Perform multiple recommend requests in batch mode

Parameters:

*   **collection\_name** – Name of the collection

*   **requests** – List of recommend requests

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Returns:

List of recommend responses

recommend\_groups(_collection\_name: str_, _group\_by: str_, _positive: Optional\[Sequence\[Union\[int, str, List\[float\], SparseVector\]\]\] \= None_, _negative: Optional\[Sequence\[Union\[int, str, List\[float\], SparseVector\]\]\] \= None_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _search\_params: Optional\[Union\[SearchParams, SearchParams\]\] \= None_, _limit: int \= 10_, _group\_size: int \= 1_, _score\_threshold: Optional\[float\] \= None_, _with\_payload: Union\[bool, Sequence\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, Sequence\[str\]\] \= False_, _using: Optional\[str\] \= None_, _lookup\_from: Optional\[Union\[LookupLocation, LookupLocation\]\] \= None_, _with\_lookup: Optional\[Union\[str, WithLookup\]\] \= None_, _strategy: Optional\[RecommendStrategy\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → GroupsResult\[source\]¶

Recommend point groups: search for similar points based on already stored in Qdrant examples and groups by payload field.

Recommend best matches for given stored examples grouped by the value of payload field. Useful to obtain most relevant results for each category, deduplicate results, finding the best representation vector for the same entity.

Parameters:

*   **collection\_name** – Collection to search in

*   **positive** – List of stored point IDs or vectors, which should be used as reference for similarity search. If there is only one example - this request is equivalent to the regular search with vector of that point. If there are more than one example, Qdrant will attempt to search for similar to all of them. Recommendation for multiple vectors is experimental. Its behaviour may change depending on selected strategy.

*   **negative** – List of stored point IDs or vectors, which should be dissimilar to the search result. Negative examples is an experimental functionality. Its behaviour may change depending on selected strategy.

*   **group\_by** – Name of the payload field to group by. Field must be of type “keyword” or “integer”. Nested fields are specified using dot notation, e.g. “nested\_field.subfield”.

*   **query\_filter** –

*   Exclude vectors which doesn’t fit given conditions.

*   If None - search among all vectors

*   **search\_params** – Additional search params

*   **limit** – How many groups return

*   **group\_size** – How many results return for each group

*   **with\_payload** –

*   Specify which stored payload should be attached to the result.

*   If True - attach all payload

*   If False - do not attach any payload

*   If List of string - include only specified fields

*   If PayloadSelector - use explicit rules

*   **with\_vectors** –

*   If True - Attach stored vector to the search result.

*   If False - Do not attach vector.

*   If List of string - include only specified fields

*   Default: False

*   **score\_threshold** – Define a minimal score threshold for the result. If defined, less similar results will not be returned. Score of the returned result might be higher or smaller than the threshold depending on the Distance function used. E.g. for cosine similarity only higher scores will be returned.

*   **using** – Name of the vectors to use for recommendations. If None - use default vectors.

*   **lookup\_from** – Defines a location (collection and vector field name), used to lookup vectors for recommendations. If None - current collection will be used.

*   **with\_lookup** – Look for points in another collection using the group ids. If specified, each group will contain a record from the specified collection with the same id as the group id. In addition, the parameter allows to specify which parts of the record should be returned, like in with\_payload and with\_vectors parameters.

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **strategy** –

Strategy to use for recommendation. Strategy defines how to combine multiple examples into a recommendation query. Possible values:

*   ’average\_vector’ - calculates average vector of all examples and uses it for search

*   ’best\_score’ - finds the result which is closer to positive examples and further from negative

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Returns:

List of groups with not more than group\_size hits in each group. Each group also contains an id of the group, which is the value of the payload field.

recover\_shard\_snapshot(_collection\_name: str_, _shard\_id: int_, _location: str_, _api\_key: Optional\[str\] \= None_, _checksum: Optional\[str\] \= None_, _priority: Optional\[SnapshotPriority\] \= None_, _wait: bool \= True_, _\*\*kwargs: Any_) → Optional\[bool\]\[source\]¶

Recover shard from snapshot.

Parameters:

*   **collection\_name** – Name of the collection

*   **shard\_id** – Index of the shard

*   **location** – URL of the snapshot Example: - URL http://localhost:8080/collections/my\_collection/snapshots/my\_snapshot

*   **api\_key** – API key to use for accessing the snapshot on another server.

*   **checksum** – Checksum of the snapshot to verify the integrity of the snapshot.

*   **priority** –

Defines source of truth for snapshot recovery

*   replica (default) means - prefer existing data over the snapshot

*   no\_sync means - do not sync shard with other shards

*   snapshot means - prefer snapshot data over the current state

*   **wait** –

Await for the recovery to be done.

*   If true, result will be returned only when the recovery is done

*   If false, result will be returned immediately after the confirmation of receiving.

Returns:

True if snapshot was recovered

recover\_snapshot(_collection\_name: str_, _location: str_, _api\_key: Optional\[str\] \= None_, _checksum: Optional\[str\] \= None_, _priority: Optional\[SnapshotPriority\] \= None_, _wait: bool \= True_, _\*\*kwargs: Any_) → Optional\[bool\]\[source\]¶

Recover collection from snapshot.

Parameters:

*   **collection\_name** – Name of the collection

*   **location** – URL of the snapshot Example: - URL http://localhost:8080/collections/my\_collection/snapshots/my\_snapshot - Local path file:///qdrant/snapshots/test\_collection/test\_collection-6194298859870377-2023-11-09-15-17-51.snapshot

*   **api\_key** – API key to use for accessing the snapshot on another server.

*   **checksum** – Checksum of the snapshot to verify the integrity of the snapshot.

*   **priority** –

Defines source of truth for snapshot recovery

*   replica (default) means - prefer existing data over the snapshot

*   no\_sync means - do not sync shard with other shards

*   snapshot means - prefer snapshot data over the current state

*   **wait** –

Await for the recovery to be done.

*   If true, result will be returned only when the recovery is done

*   If false, result will be returned immediately after the confirmation of receiving.

Returns:

True if snapshot was recovered

recreate\_collection(_collection\_name: str_, _vectors\_config: Union\[VectorParams, Mapping\[str, VectorParams\]\]_, _sparse\_vectors\_config: Optional\[Mapping\[str, SparseVectorParams\]\] \= None_, _shard\_number: Optional\[int\] \= None_, _sharding\_method: Optional\[ShardingMethod\] \= None_, _replication\_factor: Optional\[int\] \= None_, _write\_consistency\_factor: Optional\[int\] \= None_, _on\_disk\_payload: Optional\[bool\] \= None_, _hnsw\_config: Optional\[Union\[HnswConfigDiff, HnswConfigDiff\]\] \= None_, _optimizers\_config: Optional\[Union\[OptimizersConfigDiff, OptimizersConfigDiff\]\] \= None_, _wal\_config: Optional\[Union\[WalConfigDiff, WalConfigDiff\]\] \= None_, _quantization\_config: Optional\[Union\[ScalarQuantization, ProductQuantization, BinaryQuantization, QuantizationConfig\]\] \= None_, _init\_from: Optional\[Union\[InitFrom, str\]\] \= None_, _timeout: Optional\[int\] \= None_, _strict\_mode\_config: Optional\[StrictModeConfig\] \= None_, _\*\*kwargs: Any_) → bool\[source\]¶

Delete and create empty collection with given parameters

Parameters:

*   **collection\_name** – Name of the collection to recreate

*   **vectors\_config** – Configuration of the vector storage. Vector params contains size and distance for the vector storage. If dict is passed, service will create a vector storage for each key in the dict. If single VectorParams is passed, service will create a single anonymous vector storage.

*   **sparse\_vectors\_config** – Configuration of the sparse vector storage. The service will create a sparse vector storage for each key in the dict.

*   **shard\_number** – Number of shards in collection. Default is 1, minimum is 1.

*   **sharding\_method** – Defines strategy for shard creation. Option auto (default) creates defined number of shards automatically. Data will be distributed between shards automatically. After creation, shards could be additionally replicated, but new shards could not be created. Option custom allows to create shards manually, each shard should be created with assigned unique shard\_key. Data will be distributed between based on shard\_key value.

*   **replication\_factor** – Replication factor for collection. Default is 1, minimum is 1. Defines how many copies of each shard will be created. Have effect only in distributed mode.

*   **write\_consistency\_factor** – Write consistency factor for collection. Default is 1, minimum is 1. Defines how many replicas should apply the operation for us to consider it successful. Increasing this number will make the collection more resilient to inconsistencies, but will also make it fail if not enough replicas are available. Does not have any performance impact. Have effect only in distributed mode.

*   **on\_disk\_payload** – If true - point\`s payload will not be stored in memory. It will be read from the disk every time it is requested. This setting saves RAM by (slightly) increasing the response time. Note: those payload values that are involved in filtering and are indexed - remain in RAM.

*   **hnsw\_config** – Params for HNSW index

*   **optimizers\_config** – Params for optimizer

*   **wal\_config** – Params for Write-Ahead-Log

*   **quantization\_config** – Params for quantization, if None - quantization will be disabled

*   **init\_from** – Use data stored in another collection to initialize this collection

*   **timeout** – Wait for operation commit timeout in seconds. If timeout is reached - request will return with service error.

*   **strict\_mode\_config** – Configure limitations for the collection, such as max size, rate limits, etc.

Returns:

Operation result

retrieve(_collection\_name: str_, _ids: Sequence\[Union\[int, str, PointId\]\]_, _with\_payload: Union\[bool, Sequence\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, Sequence\[str\]\] \= False_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → list\[Record\]\[source\]¶

Retrieve stored points by IDs

Parameters:

*   **collection\_name** – Name of the collection to lookup in

*   **ids** – list of IDs to lookup

*   **with\_payload** –

*   Specify which stored payload should be attached to the result.

*   If True - attach all payload

*   If False - do not attach any payload

*   If List of string - include only specified fields

*   If PayloadSelector - use explicit rules

*   **with\_vectors** –

*   If True - Attach stored vector to the search result.

*   If False - Do not attach vector.

*   If List of string - Attach only specified vectors.

*   Default: False

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **timeout** – Overrides global timeout for this operation. Unit is seconds.

Returns:

List of points

scroll(_collection\_name: str_, _scroll\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _limit: int \= 10_, _order\_by: Optional\[Union\[str, OrderBy, OrderBy\]\] \= None_, _offset: Optional\[Union\[int, str, PointId\]\] \= None_, _with\_payload: Union\[bool, Sequence\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, Sequence\[str\]\] \= False_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → tuple\[list\[Record\], Union\[int, str, points\_pb2.PointId, NoneType\]\]\[source\]¶

Scroll over all (matching) points in the collection.

This method provides a way to iterate over all stored points with some optional filtering condition. Scroll does not apply any similarity estimations, it will return points sorted by id in ascending order.

Parameters:

*   **collection\_name** – Name of the collection

*   **scroll\_filter** – If provided - only returns points matching filtering conditions

*   **limit** – How many points to return

*   **order\_by** – Order the records by a payload key. If None - order by id

*   **offset** – If provided - skip points with ids less than given offset

*   **with\_payload** –

*   Specify which stored payload should be attached to the result.

*   If True - attach all payload

*   If False - do not attach any payload

*   If List of string - include only specified fields

*   If PayloadSelector - use explicit rules

*   **with\_vectors** –

*   If True - Attach stored vector to the search result.

*   If False (default) - Do not attach vector.

*   If List of string - include only specified fields

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **timeout** – Overrides global timeout for this operation. Unit is seconds.

Returns:

A pair of (List of points) and (optional offset for the next scroll request). If next page offset is None - there is no more points in the collection to scroll.

search(_collection\_name: str_, _query\_vector: Union\[Sequence\[float\], tuple\[str, list\[float\]\], NamedVector, NamedSparseVector, ndarray\[tuple\[int, ...\], dtype\[Union\[bool, int8, int16, int32, int64, uint8, uint16, uint32, uint64, float16, float32, float64, longdouble\]\]\]\]_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _search\_params: Optional\[Union\[SearchParams, SearchParams\]\] \= None_, _limit: int \= 10_, _offset: Optional\[int\] \= None_, _with\_payload: Union\[bool, Sequence\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, Sequence\[str\]\] \= False_, _score\_threshold: Optional\[float\] \= None_, _append\_payload: bool \= True_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → list\[ScoredPoint\]\[source\]¶

Search for closest vectors in collection taking into account filtering conditions

Parameters:

*   **collection\_name** – Collection to search in

*   **query\_vector** – Search for vectors closest to this. Can be either a vector itself, or a named vector, or a named sparse vector, or a tuple of vector name and vector itself

*   **query\_filter** –

*   Exclude vectors which doesn’t fit given conditions.

*   If None - search among all vectors

*   **search\_params** – Additional search params

*   **limit** – How many results return

*   **offset** – Offset of the first result to return. May be used to paginate results. Note: large offset values may cause performance issues.

*   **with\_payload** –

*   Specify which stored payload should be attached to the result.

*   If True - attach all payload

*   If False - do not attach any payload

*   If List of string - include only specified fields

*   If PayloadSelector - use explicit rules

*   **with\_vectors** –

*   If True - Attach stored vector to the search result.

*   If False - Do not attach vector.

*   If List of string - include only specified fields

*   Default: False

*   **score\_threshold** – Define a minimal score threshold for the result. If defined, less similar results will not be returned. Score of the returned result might be higher or smaller than the threshold depending on the Distance function used. E.g. for cosine similarity only higher scores will be returned.

*   **append\_payload** – Same as with\_payload. Deprecated.

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Examples:

Search with filter:

qdrant.search(
collection\_name\="test\_collection",
query\_vector\=\[1.0, 0.1, 0.2, 0.7\],
query\_filter\=Filter(
must\=\[
FieldCondition(
key\='color',
range\=Match(
value\="red"
)
)
\]
)
)

Returns:

List of found close points with similarity scores.

search\_batch(_collection\_name: str_, _requests: Sequence\[Union\[SearchRequest, SearchPoints\]\]_, _timeout: Optional\[int\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _\*\*kwargs: Any_) → list\[list\[ScoredPoint\]\]\[source\]¶

Perform multiple searches in a collection mitigating network overhead

Parameters:

*   **collection\_name** – Name of the collection

*   **requests** – List of search requests

*   **consistency** –

Read consistency of the search. Defines how many replicas should be queried before returning the result. Values:

*   int - number of replicas to query, values should present in all queried replicas

*   ’majority’ - query all replicas, but return values present in the majority of replicas

*   ’quorum’ - query the majority of replicas, return values present in all of them

*   ’all’ - query all replicas, and return values present in all replicas

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Returns:

List of search responses

search\_groups(_collection\_name: str_, _query\_vector: Union\[Sequence\[float\], tuple\[str, list\[float\]\], NamedVector, NamedSparseVector, ndarray\[tuple\[int, ...\], dtype\[Union\[bool, int8, int16, int32, int64, uint8, uint16, uint32, uint64, float16, float32, float64, longdouble\]\]\]\]_, _group\_by: str_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _search\_params: Optional\[Union\[SearchParams, SearchParams\]\] \= None_, _limit: int \= 10_, _group\_size: int \= 1_, _with\_payload: Union\[bool, Sequence\[str\], PayloadSelectorInclude, PayloadSelectorExclude, WithPayloadSelector\] \= True_, _with\_vectors: Union\[bool, Sequence\[str\]\] \= False_, _score\_threshold: Optional\[float\] \= None_, _with\_lookup: Optional\[Union\[str, WithLookup\]\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → GroupsResult\[source\]¶

Search for closest vectors grouped by payload field.

Searches best matches for query vector grouped by the value of payload field. Useful to obtain most relevant results for each category, deduplicate results, finding the best representation vector for the same entity.

Parameters:

*   **collection\_name** – Collection to search in

*   **query\_vector** – Search for vectors closest to this. Can be either a vector itself, or a named vector, or a named sparse vector, or a tuple of vector name and vector itself

*   **group\_by** – Name of the payload field to group by. Field must be of type “keyword” or “integer”. Nested fields are specified using dot notation, e.g. “nested\_field.subfield”.

*   **query\_filter** –

*   Exclude vectors which doesn’t fit given conditions.

*   If None - search among all vectors

*   **search\_params** – Additional search params

*   **limit** – How many groups return

*   **group\_size** – How many results return for each group

*   **with\_payload** –

*   Specify which stored payload should be attached to the result.

*   If True - attach all payload

*   If False - do not attach any payload

*   If List of string - include only specified fields

*   If PayloadSelector - use explicit rules

*   **with\_vectors** –

*   If True - Attach stored vector to the search result.

*   If False - Do not attach vector.

*   If List of string - include only specified fields

*   Default: False

*   **score\_threshold** – Minimal score threshold for the result. If defined, less similar results will not be returned. Score of the returned result might be higher or smaller than the threshold depending on the Distance function used. E.g. for cosine similarity only higher scores will be returned.

*   **with\_lookup** – Look for points in another collection using the group ids. If specified, each group will contain a record from the specified collection with the same id as the group id. In addition, the parameter allows to specify which parts of the record should be returned, like in with\_payload and with\_vectors parameters.

*   **consistency** – Read consistency of the search. Defines how many replicas should be queried before returning the result. Values: - int - number of replicas to query, values should present in all queried replicas - ‘majority’ - query all replicas, but return values present in the majority of replicas - ‘quorum’ - query the majority of replicas, return values present in all of them - ‘all’ - query all replicas, and return values present in all replicas

*   **shard\_key\_selector** – This parameter allows to specify which shards should be queried. If None - query all shards. Only works for collections with custom sharding method.

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

Returns:

List of groups with not more than group\_size hits in each group. Each group also contains an id of the group, which is the value of the payload field.

search\_matrix\_offsets(_collection\_name: str_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _limit: int \= 3_, _sample: int \= 10_, _using: Optional\[str\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _timeout: Optional\[int\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → SearchMatrixOffsetsResponse\[source\]¶

Compute distance matrix for sampled points with an offset-based output format.

Parameters:

*   **collection\_name** – Name of the collection.

*   **query\_filter** – Filter to apply.

*   **limit** – How many neighbors per sample to find.

*   **sample** – How many points to select and search within.

*   **using** – Name of the vectors to use for search. If None, use default vectors.

*   **consistency** – Read consistency of the search. Defines how many replicas should be queried before returning the result. Values: - int: Number of replicas to query, values should present in all queried replicas. - ‘majority’: Query all replicas, but return values present in the majority of replicas. - ‘quorum’: Query the majority of replicas, return values present in all of them. - ‘all’: Query all replicas and return values present in all replicas.

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

*   **shard\_key\_selector** – This parameter allows specifying which shards should be queried. If None, query all shards. Only works for collections with the custom sharding method.

Returns:

Distance matrix using an offset-based encoding.

search\_matrix\_pairs(_collection\_name: str_, _query\_filter: Optional\[Union\[Filter, Filter\]\] \= None_, _limit: int \= 3_, _sample: int \= 10_, _using: Optional\[str\] \= None_, _consistency: Optional\[Union\[int, ReadConsistencyType\]\] \= None_, _timeout: Optional\[int\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → SearchMatrixPairsResponse\[source\]¶

Compute distance matrix for sampled points with a pair-based output format.

Parameters:

*   **collection\_name** – Name of the collection.

*   **query\_filter** – Filter to apply.

*   **limit** – How many neighbors per sample to find.

*   **sample** – How many points to select and search within.

*   **using** – Name of the vectors to use for search. If None, use default vectors.

*   **consistency** – Read consistency of the search. Defines how many replicas should be queried before returning the result. Values: - int: Number of replicas to query, values should be present in all queried replicas. - ‘majority’: Query all replicas, but return values present in the majority of replicas. - ‘quorum’: Query the majority of replicas, return values present in all of them. - ‘all’: Query all replicas, and return values present in all replicas.

*   **timeout** – Overrides global timeout for this search. Unit is seconds.

*   **shard\_key\_selector** – This parameter allows specifying which shards should be queried. If None, query all shards. Only works for collections with the custom sharding method.

Returns:

Distance matrix using a pair-based encoding.

set\_payload(_collection\_name: str_, _payload: Dict\[str, Any\]_, _points: Union\[list\[Union\[int, str, points\_pb2.PointId\]\], Filter, Filter, PointIdsList, FilterSelector, PointsSelector\]_, _key: Optional\[str\] \= None_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Modifies payload of the specified points.

Examples

Set payload:

\# Assign payload value with key \`"key"\` to points 1, 2, 3.
\# If payload value with specified key already exists - it will be overwritten
qdrant\_client.set\_payload(
collection\_name\="test\_collection",
wait\=True,
payload\={
"key": "value"
},
points\=\[1, 2, 3\]
)

Parameters:

*   **collection\_name** – Name of the collection.

*   **wait** – Await for the results to be processed. - If true, the result will be returned only when all changes are applied. - If false, the result will be returned immediately after confirmation of receipt.

*   **payload** – Key-value pairs of payload to assign.

*   **points** –

List of affected points, filter, or points selector. .. rubric:: Example

*   points=\[1, 2, 3, “cd3b53f0-11a7-449f-bc50-d06310e7ed90”\]

*   points=Filter(must=\[FieldCondition(key=’rand\_number’, range=Range(gte=0.7))\])

*   **ordering** (_Optional__\[__WriteOrdering__\]_) – Define strategy for ordering of the points. Possible values: - weak (default): Write operations may be reordered, works faster. - medium: Write operations go through a dynamically selected leader, may be inconsistent for a short period of time in case of leader change. - strong: Write operations go through the permanent leader, consistent, but may be unavailable if the leader is down.

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard keys are provided, the update will be written to each of them. Only works for collections with the custom sharding method.

*   **key** –

Path to the nested field in the payload to modify. If not specified, modifies the root of the payload. E.g.:

PointStruct(
id\=42,
vector\=\[...\],
payload\={
"recipe": {
"fruits": {"apple": "100g"}
}
}
)

qdrant\_client.set\_payload(
...,
payload\={"cinnamon": "2g"},
key\="recipe.fruits",
points\=\[42\]
)

PointStruct(
id\=42,
vector\=\[...\],
payload\={
"recipe": {
"fruits": {
"apple": "100g",
"cinnamon": "2g"
}
}
}
)

Returns:

Operation result.

unlock\_storage(_\*\*kwargs: Any_) → LocksOption\[source\]¶

Unlock storage for writing.

update\_collection(_collection\_name: str_, _optimizers\_config: Optional\[Union\[OptimizersConfigDiff, OptimizersConfigDiff\]\] \= None_, _collection\_params: Optional\[Union\[CollectionParamsDiff, CollectionParamsDiff\]\] \= None_, _vectors\_config: Optional\[Union\[Dict\[str, VectorParamsDiff\], VectorsConfigDiff\]\] \= None_, _hnsw\_config: Optional\[Union\[HnswConfigDiff, HnswConfigDiff\]\] \= None_, _quantization\_config: Optional\[Union\[ScalarQuantization, ProductQuantization, BinaryQuantization, Disabled, QuantizationConfigDiff\]\] \= None_, _timeout: Optional\[int\] \= None_, _sparse\_vectors\_config: Optional\[Mapping\[str, SparseVectorParams\]\] \= None_, _strict\_mode\_config: Optional\[StrictModeConfig\] \= None_, _\*\*kwargs: Any_) → bool\[source\]¶

Update parameters of the collection

Parameters:

*   **collection\_name** – Name of the collection

*   **optimizers\_config** – Override for optimizer configuration

*   **collection\_params** – Override for collection parameters

*   **vectors\_config** – Override for vector-specific configuration

*   **hnsw\_config** – Override for HNSW index params

*   **quantization\_config** – Override for quantization params

*   **timeout** – Wait for operation commit timeout in seconds. If timeout is reached - request will return with service error.

*   **sparse\_vectors\_config** – Override for sparse vector-specific configuration

*   **strict\_mode\_config** – Override for strict mode configuration

Returns:

Operation result

update\_collection\_aliases(_change\_aliases\_operations: Sequence\[Union\[CreateAliasOperation, RenameAliasOperation, DeleteAliasOperation, AliasOperations\]\]_, _timeout: Optional\[int\] \= None_, _\*\*kwargs: Any_) → bool\[source\]¶

Operation for performing changes of collection aliases.

Alias changes are atomic, meaning that no collection modifications can happen between alias operations.

Parameters:

*   **change\_aliases\_operations** – List of operations to perform

*   **timeout** – Wait for operation commit timeout in seconds. If timeout is reached - request will return with service error.

Returns:

Operation result

update\_vectors(_collection\_name: str_, _points: Sequence\[PointVectors\]_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Update specified vectors in the collection. Keeps payload and unspecified vectors unchanged.

Parameters:

*   **collection\_name** (_str_) – Name of the collection to update vectors in

*   **points** (_Point_) –

List of (id, vector) pairs to update. Vector might be a list of numbers or a dict of named vectors. Examples:

*   PointVectors(id=1, vector=\[1, 2, 3\])

*   PointVectors(id=2, vector={‘vector\_1’: \[1, 2, 3\], ‘vector\_2’: \[4, 5, 6\]})

*   **wait** (_bool_) –

Await for the results to be processed.

*   If true, result will be returned only when all changes are applied

*   If false, result will be returned immediately after the confirmation of receiving.

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method.

Returns:

Operation Result(UpdateResult)

upload\_collection(_collection\_name: str_, _vectors: Union\[Iterable\[Union\[List\[float\], List\[List\[float\]\], Dict\[str, Union\[List\[float\], SparseVector, List\[List\[float\]\], Document, Image, InferenceObject\]\], Document, Image, InferenceObject\]\], dict\[str, numpy.ndarray\[tuple\[int, ...\], numpy.dtype\[Union\[numpy.bool, numpy.int8, numpy.int16, numpy.int32, numpy.int64, numpy.uint8, numpy.uint16, numpy.uint32, numpy.uint64, numpy.float16, numpy.float32, numpy.float64, numpy.longdouble\]\]\]\], ndarray\[tuple\[int, ...\], dtype\[Union\[bool, int8, int16, int32, int64, uint8, uint16, uint32, uint64, float16, float32, float64, longdouble\]\]\]\]_, _payload: Optional\[Iterable\[dict\[Any, Any\]\]\] \= None_, _ids: Optional\[Iterable\[Union\[int, str, PointId\]\]\] \= None_, _batch\_size: int \= 64_, _parallel: int \= 1_, _method: Optional\[str\] \= None_, _max\_retries: int \= 3_, _wait: bool \= False_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → None\[source\]¶

Upload vectors and payload to the collection. This method will perform automatic batching of the data. If you need to perform a single update, use upsert method. Note: use upload\_records method if you want to upload multiple vectors with single payload.

Parameters:

*   **collection\_name** – Name of the collection to upload to

*   **vectors** – np.ndarray or an iterable over vectors to upload. Might be mmaped

*   **payload** – Iterable of vectors payload, Optional, Default: None

*   **ids** – Iterable of custom vectors ids, Optional, Default: None

*   **batch\_size** – How many vectors upload per-request, Default: 64

*   **parallel** – Number of parallel processes of upload

*   **method** – Start method for parallel processes, Default: forkserver

*   **max\_retries** – maximum number of retries in case of a failure during the upload of a batch

*   **wait** – Await for the results to be applied on the server side. If true, each update request will explicitly wait for the confirmation of completion. Might be slower. If false, each update request will return immediately after the confirmation of receiving. Default: false

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method.

upload\_points(_collection\_name: str_, _points: Iterable\[PointStruct\]_, _batch\_size: int \= 64_, _parallel: int \= 1_, _method: Optional\[str\] \= None_, _max\_retries: int \= 3_, _wait: bool \= False_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → None\[source\]¶

Upload points to the collection

Similar to upload\_collection method, but operates with points, rather than vector and payload individually.

Parameters:

*   **collection\_name** – Name of the collection to upload to

*   **points** – Iterator over points to upload

*   **batch\_size** – How many vectors upload per-request, Default: 64

*   **parallel** – Number of parallel processes of upload

*   **method** – Start method for parallel processes, Default: forkserver

*   **max\_retries** – maximum number of retries in case of a failure during the upload of a batch

*   **wait** – Await for the results to be applied on the server side. If true, each update request will explicitly wait for the confirmation of completion. Might be slower. If false, each update request will return immediately after the confirmation of receiving. Default: false

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method. This parameter overwrites shard keys written in the records.

upload\_records(_collection\_name: str_, _records: Iterable\[Record\]_, _batch\_size: int \= 64_, _parallel: int \= 1_, _method: Optional\[str\] \= None_, _max\_retries: int \= 3_, _wait: bool \= False_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → None\[source\]¶

Upload records to the collection

Similar to upload\_collection method, but operates with records, rather than vector and payload individually.

Parameters:

*   **collection\_name** – Name of the collection to upload to

*   **records** – Iterator over records to upload

*   **batch\_size** – How many vectors upload per-request, Default: 64

*   **parallel** – Number of parallel processes of upload

*   **method** – Start method for parallel processes, Default: forkserver

*   **max\_retries** – maximum number of retries in case of a failure during the upload of a batch

*   **wait** – Await for the results to be applied on the server side. If true, each update request will explicitly wait for the confirmation of completion. Might be slower. If false, each update request will return immediately after the confirmation of receiving. Default: false

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method. This parameter overwrites shard keys written in the records.

upsert(_collection\_name: str_, _points: Union\[Batch, Sequence\[Union\[PointStruct, PointStruct\]\]\]_, _wait: bool \= True_, _ordering: Optional\[WriteOrdering\] \= None_, _shard\_key\_selector: Optional\[Union\[int, str, List\[Union\[int, str\]\]\]\] \= None_, _\*\*kwargs: Any_) → UpdateResult\[source\]¶

Update or insert a new point into the collection.

If point with given ID already exists - it will be overwritten.

Parameters:

*   **collection\_name** (_str_) – To which collection to insert

*   **points** (_Point_) – Batch or list of points to insert

*   **wait** (_bool_) –

Await for the results to be processed.

*   If true, result will be returned only when all changes are applied

*   If false, result will be returned immediately after the confirmation of receiving.

*   **ordering** (_Optional__\[__WriteOrdering__\]_) –

Define strategy for ordering of the points. Possible values:

*   weak (default) - write operations may be reordered, works faster

*   medium - write operations go through dynamically selected leader, may be inconsistent for a short period of time in case of leader change

*   strong - Write operations go through the permanent leader, consistent, but may be unavailable if leader is down

*   **shard\_key\_selector** – Defines the shard groups that should be used to write updates into. If multiple shard\_keys are provided, the update will be written to each of them. Only works for collections with custom sharding method.

Returns:

Operation Result(UpdateResult)

_property_ grpc\_collections_: CollectionsStub_¶

gRPC client for collections methods

Returns:

An instance of raw gRPC client, generated from Protobuf

_property_ grpc\_points_: PointsStub_¶

gRPC client for points methods

Returns:

An instance of raw gRPC client, generated from Protobuf

_property_ http_: SyncApis\[ApiClient\]_¶

REST Client

Returns:

An instance of raw REST API client, generated from OpenAPI schema

_property_ init\_options_: dict\[str, Any\]_¶

\_\_init\_\_ Options

Returns:

A dictionary of options the client class was instantiated with

_property_ rest_: SyncApis\[ApiClient\]_¶

REST Client

Returns:

An instance of raw REST API client, generated from OpenAPI schema

*   qdrant\_client.qdrant\_client module
*   `QdrantClient`
*   `QdrantClient.batch_update_points()`
*   `QdrantClient.clear_payload()`
*   `QdrantClient.close()`
*   `QdrantClient.collection_exists()`
*   `QdrantClient.count()`
*   `QdrantClient.create_collection()`
*   `QdrantClient.create_full_snapshot()`
*   `QdrantClient.create_payload_index()`
*   `QdrantClient.create_shard_key()`
*   `QdrantClient.create_shard_snapshot()`
*   `QdrantClient.create_snapshot()`
*   `QdrantClient.delete()`
*   `QdrantClient.delete_collection()`
*   `QdrantClient.delete_full_snapshot()`
*   `QdrantClient.delete_payload()`
*   `QdrantClient.delete_payload_index()`
*   `QdrantClient.delete_shard_key()`
*   `QdrantClient.delete_shard_snapshot()`
*   `QdrantClient.delete_snapshot()`
*   `QdrantClient.delete_vectors()`
*   `QdrantClient.discover()`
*   `QdrantClient.discover_batch()`
*   `QdrantClient.facet()`
*   `QdrantClient.get_aliases()`
*   `QdrantClient.get_collection()`
*   `QdrantClient.get_collection_aliases()`
*   `QdrantClient.get_collections()`
*   `QdrantClient.get_locks()`
*   `QdrantClient.info()`
*   `QdrantClient.list_full_snapshots()`
*   `QdrantClient.list_shard_snapshots()`
*   `QdrantClient.list_snapshots()`
*   `QdrantClient.lock_storage()`
*   `QdrantClient.migrate()`
*   `QdrantClient.overwrite_payload()`
*   `QdrantClient.query_batch_points()`
*   `QdrantClient.query_points()`
*   `QdrantClient.query_points_groups()`
*   `QdrantClient.recommend()`
*   `QdrantClient.recommend_batch()`
*   `QdrantClient.recommend_groups()`
*   `QdrantClient.recover_shard_snapshot()`
*   `QdrantClient.recover_snapshot()`
*   `QdrantClient.recreate_collection()`
*   `QdrantClient.retrieve()`
*   `QdrantClient.scroll()`
*   `QdrantClient.search()`
*   `QdrantClient.search_batch()`
*   `QdrantClient.search_groups()`
*   `QdrantClient.search_matrix_offsets()`
*   `QdrantClient.search_matrix_pairs()`
*   `QdrantClient.set_payload()`
*   `QdrantClient.unlock_storage()`
*   `QdrantClient.update_collection()`
*   `QdrantClient.update_collection_aliases()`
*   `QdrantClient.update_vectors()`
*   `QdrantClient.upload_collection()`
*   `QdrantClient.upload_points()`
*   `QdrantClient.upload_records()`
*   `QdrantClient.upsert()`
*   `QdrantClient.grpc_collections`
*   `QdrantClient.grpc_points`
*   `QdrantClient.http`
*   `QdrantClient.init_options`
*   `QdrantClient.rest`

Qdrant
------

Learn more about Qdrant vector search project and ecosystem

Discover Qdrant

Similarity Learning
-------------------

Explore practical problem solving with Similarity Learning

Learn Similarity Learning

Community
---------

Find people dealing with similar problems and get answers to your questions

Join Community

To analyze traffic and optimize your experience, we serve cookies on this site. By clicking or navigating, you agree to allow our usage of cookies. Read our Privacy Policy.

*   Get Started
*   Blog
*   Community
*   *   Linkedin
*   Twitter
*   Discord
*   Github
*   Qdrant