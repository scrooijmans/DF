Description: API types

Title: qdrant\_client::qdrant - Rust

Docs.rs

*   qdrant-client-1.15.0

*   qdrant-client 1.15.0
*   Permalink
*   Docs.rs crate page
*   Apache-2.0

*   Links
*   Homepage
*   Repository
*   crates.io
*   Source

*   Owners
*   generall

*   Dependencies
*   *   anyhow ^1.0.89 _normal_
*   derive\_builder ^0.20.2 _normal_
*   futures ^0.3.31 _normal_
*   futures-util ^0.3.31 _normal_ _optional_
*   prost ^0.13.3 _normal_
*   prost-types ^0.13.3 _normal_
*   reqwest ^0.12.8 _normal_ _optional_
*   semver ^1.0.24 _normal_
*   serde ^1.0.210 _normal_ _optional_
*   serde\_json ^1.0.128 _normal_ _optional_
*   thiserror ^1.0.64 _normal_
*   tokio ^1.40.0 _normal_
*   tonic ^0.12.3 _normal_
*   uuid ^1.8.2 _normal_ _optional_
*   tonic-build ^0.12.3 _dev_

*   Versions

*   **64.51%** of the crate is documented

*   Platform
*   i686-unknown-linux-gnu
*   x86\_64-unknown-linux-gnu
*   Feature flags

*   docs.rs
*   About docs.rs
*   Badges
*   Builds
*   Metadata
*   Shorthand URLs
*   Download
*   Rustdoc JSON
*   Build queue
*   Privacy policy

*   Rust
*   Rust website
*   The Book
*   Standard Library API Reference
*   Rust by Example
*   The Cargo Guide
*   Clippy Documentation

qdrant\_client

Module qdrantCopy item path
===========================

Source

Expand description

API types

Modules§
--------

alias\_operations

Nested message and enum types in `AliasOperations`.

binary\_quantization\_query\_encoding

Nested message and enum types in `BinaryQuantizationQueryEncoding`.

collections\_client

Generated client implementations.

collections\_server

Generated server implementations.

condition

Nested message and enum types in `Condition`.

count\_points\_builder

create\_collection\_builder

dense\_vector\_builder

document\_builder

expression

Nested message and enum types in `Expression`.

facet\_value

Nested message and enum types in `FacetValue`.

group\_id

Nested message and enum types in `GroupId`.

image\_builder

match

Nested message and enum types in `Match`.

max\_optimization\_threads

Nested message and enum types in `MaxOptimizationThreads`.

multi\_dense\_vector\_builder

order\_value

Nested message and enum types in `OrderValue`.

payload\_index\_params

Nested message and enum types in `PayloadIndexParams`.

point\_id

Nested message and enum types in `PointId`.

points\_client

Generated client implementations.

points\_selector

Nested message and enum types in `PointsSelector`.

points\_server

Generated server implementations.

points\_update\_operation

Nested message and enum types in `PointsUpdateOperation`.

qdrant\_client

Generated client implementations.

qdrant\_server

Generated server implementations.

quantization\_config

Nested message and enum types in `QuantizationConfig`.

quantization\_config\_diff

Nested message and enum types in `QuantizationConfigDiff`.

query

Nested message and enum types in `Query`.

read\_consistency

Nested message and enum types in `ReadConsistency`.

shard\_key

Nested message and enum types in `ShardKey`.

snapshots\_client

Generated client implementations.

snapshots\_server

Generated server implementations.

sparse\_vector\_builder

sparse\_vectors\_config

start\_from

Nested message and enum types in `StartFrom`.

stemming\_algorithm

Nested message and enum types in `StemmingAlgorithm`.

target\_vector

Nested message and enum types in `TargetVector`.

update\_collection\_cluster\_setup\_request

Nested message and enum types in `UpdateCollectionClusterSetupRequest`.

value

Nested message and enum types in `Value`.

vector

Nested message and enum types in `Vector`.

vector\_example

Nested message and enum types in `VectorExample`.

vector\_input

Nested message and enum types in `VectorInput`.

vector\_output

Nested message and enum types in `VectorOutput`.

vectors

Nested message and enum types in `Vectors`.

vectors\_config

Nested message and enum types in `VectorsConfig`.

vectors\_config\_diff

Nested message and enum types in `VectorsConfigDiff`.

vectors\_output

Nested message and enum types in `VectorsOutput`.

with\_payload\_selector

Nested message and enum types in `WithPayloadSelector`.

with\_vectors\_selector

Nested message and enum types in `WithVectorsSelector`.

Structs§
--------

AbortShardTransfer

AbortShardTransferBuilder

AliasDescription

AliasOperations

BatchResult

BinaryQuantization

BinaryQuantizationBuilder

BinaryQuantizationQueryEncoding

BoolIndexParams

BoolIndexParamsBuilder

ChangeAliases

ClearPayloadPoints

ClearPayloadPointsBuilder

CollectionClusterInfoRequest

CollectionClusterInfoResponse

CollectionConfig

CollectionDescription

CollectionExists

CollectionExistsRequest

CollectionExistsResponse

CollectionInfo

CollectionOperationResponse

CollectionParams

CollectionParamsDiff

CollectionParamsDiffBuilder

Condition

ContextExamplePair

ContextExamplePairBuilder

ContextInput

ContextInputBuilder

ContextInputPair

ContextInputPairBuilder

CountPoints

CountPointsBuilder

CountResponse

CountResult

CreateAlias

CreateAliasBuilder

CreateCollection

CreateCollectionBuilder

CreateFieldIndexCollection

CreateFieldIndexCollectionBuilder

CreateFullSnapshotRequest

CreateShardKey

CreateShardKeyBuilder

CreateShardKeyRequest

CreateShardKeyRequestBuilder

CreateShardKeyResponse

CreateSnapshotRequest

CreateSnapshotResponse

DatetimeIndexParams

DatetimeIndexParamsBuilder

DatetimeRange

DecayParamsExpression

DecayParamsExpressionBuilder

Builder for the DecayParamsExpression struct, which represents parameters for decay functions.

DeleteAlias

DeleteCollection

DeleteCollectionBuilder

DeleteFieldIndexCollection

DeleteFieldIndexCollectionBuilder

DeleteFullSnapshotRequest

DeletePayloadPoints

DeletePayloadPointsBuilder

DeletePointVectors

DeletePointVectorsBuilder

DeletePoints

DeletePointsBuilder

DeleteShardKey

DeleteShardKeyRequest

DeleteShardKeyRequestBuilder

DeleteShardKeyResponse

DeleteSnapshotRequest

DeleteSnapshotRequestBuilder

DeleteSnapshotResponse

DenseVector

DenseVectorBuilder

Disabled

DiscoverBatchPoints

DiscoverBatchPointsBuilder

DiscoverBatchResponse

DiscoverInput

DiscoverInputBuilder

DiscoverPoints

DiscoverPointsBuilder

DiscoverResponse

DivExpression

Document

DocumentBuilder

Expression

FacetCounts

FacetCountsBuilder

FacetHit

FacetResponse

FacetValue

FieldCondition

Filter

FloatIndexParams

FloatIndexParamsBuilder

Formula

FormulaBuilder

Builder for the Formula struct, which represents a scoring formula for points.

GeoBoundingBox

GeoDistance

GeoIndexParams

GeoIndexParamsBuilder

GeoLineString

GeoPoint

GeoPolygon

For a valid GeoPolygon, both the exterior and interior GeoLineStrings must consist of a minimum of 4 points. Additionally, the first and last points of each GeoLineString must be the same.

GeoRadius

GetCollectionInfoRequest

GetCollectionInfoResponse

GetPoints

GetPointsBuilder

GetResponse

GroupId

GroupsResult

HardwareUsage

HasIdCondition

HasVectorCondition

HealthCheckReply

HealthCheckRequest

HnswConfigDiff

HnswConfigDiffBuilder

Image

ImageBuilder

InferenceObject

InferenceUsage

IntegerIndexParams

IntegerIndexParamsBuilder

IsEmptyCondition

IsNullCondition

KeywordIndexParams

KeywordIndexParamsBuilder

ListAliasesRequest

ListAliasesResponse

ListCollectionAliasesRequest

ListCollectionsRequest

ListCollectionsResponse

ListFullSnapshotsRequest

ListSnapshotsRequest

ListSnapshotsResponse

ListValue

`ListValue` is a wrapper around a repeated field of values.

LocalShardInfo

LookupLocation

LookupLocationBuilder

Match

MaxOptimizationThreads

MaxOptimizationThreadsBuilder

Max number of threads (jobs) for running optimizations per shard. Each optimization job will also use `max_indexing_threads` threads by itself for index building.

MinShould

Mmr

Maximal Marginal Relevance (MMR) algorithm for re-ranking the points.

MmrBuilder

ModelUsage

MoveShard

MoveShardBuilder

MultExpression

MultiDenseVector

MultiDenseVectorBuilder

MultiVectorConfig

MultiVectorConfigBuilder

NamedVectors

NamedVectorsOutput

NearestInputWithMmr

NestedCondition

OptimizerStatus

OptimizersConfigDiff

OptimizersConfigDiffBuilder

OrderBy

OrderByBuilder

OrderValue

PayloadExcludeSelector

PayloadIncludeSelector

PayloadIndexParams

PayloadSchemaInfo

PointGroup

PointId

PointStruct

PointVectors

PointsIdsList

PointsOperationResponse

PointsSelector

PointsUpdateOperation

PowExpression

PrefetchQuery

PrefetchQueryBuilder

ProductQuantization

ProductQuantizationBuilder

QuantizationConfig

QuantizationConfigDiff

QuantizationSearchParams

QuantizationSearchParamsBuilder

Query

QueryBatchPoints

QueryBatchPointsBuilder

QueryBatchResponse

QueryGroupsResponse

QueryPointGroups

QueryPointGroupsBuilder

QueryPoints

QueryPointsBuilder

QueryResponse

Range

ReadConsistency

RecommendBatchPoints

RecommendBatchPointsBuilder

RecommendBatchResponse

RecommendGroupsResponse

RecommendInput

RecommendInputBuilder

RecommendPointGroups

RecommendPointGroupsBuilder

RecommendPoints

RecommendPointsBuilder

RecommendResponse

RemoteShardInfo

RenameAlias

RenameAliasBuilder

RepeatedIntegers

RepeatedStrings

Replica

ReplicaBuilder

ReplicateShard

ReplicateShardBuilder

ReshardingInfo

RestartTransfer

RetrievedPoint

ScalarQuantization

ScalarQuantizationBuilder

ScoredPoint

ScrollPoints

ScrollPointsBuilder

ScrollResponse

SearchBatchPoints

SearchBatchPointsBuilder

SearchBatchResponse

SearchGroupsResponse

SearchMatrixOffsets

SearchMatrixOffsetsResponse

SearchMatrixPair

SearchMatrixPairs

SearchMatrixPairsResponse

SearchMatrixPoints

SearchMatrixPointsBuilder

SearchParams

SearchParamsBuilder

SearchPointGroups

SearchPointGroupsBuilder

SearchPoints

SearchPointsBuilder

SearchResponse

SetPayloadPoints

SetPayloadPointsBuilder

ShardKey

ShardKeySelector

ShardTransferInfo

SnapshotDescription

SnapshotDownload

SnapshotDownloadBuilder

Builder for `SnapshotDownload`.

SnowballParams

SparseIndexConfig

SparseIndexConfigBuilder

SparseIndices

SparseVector

SparseVectorBuilder

SparseVectorConfig

SparseVectorParams

SparseVectorParamsBuilder

SparseVectorsConfigBuilder

StartFrom

StemmingAlgorithm

StopwordsSet

StrictModeConfig

StrictModeConfigBuilder

StrictModeMultivector

StrictModeMultivectorConfig

StrictModeMultivectorConfigBuilder

Builder for StrictModeMultivectorConfig, which defines multivector configuration for strict mode.

StrictModeSparse

StrictModeSparseConfig

StrictModeSparseConfigBuilder

Builder for StrictModeSparseConfig, which defines sparse vector configuration for strict mode.

Struct

`Struct` represents a structured data value, consisting of fields which map to dynamically typed values. In some languages, `Struct` might be supported by a native representation. For example, in scripting languages like JS a struct is represented as an object. The details of that representation are described together with the proto support for the language.

SumExpression

TargetVector

TextIndexParams

TextIndexParamsBuilder

Timestamp

A Timestamp represents a point in time independent of any time zone or local calendar, encoded as a count of seconds and fractions of seconds at nanosecond resolution. The count is relative to an epoch at UTC midnight on January 1, 1970, in the proleptic Gregorian calendar which extends the Gregorian calendar backwards to year one.

UpdateBatchPoints

UpdateBatchPointsBuilder

UpdateBatchResponse

UpdateCollection

UpdateCollectionBuilder

UpdateCollectionClusterSetupRequest

UpdateCollectionClusterSetupRequestBuilder

UpdateCollectionClusterSetupResponse

UpdatePointVectors

UpdatePointVectorsBuilder

UpdateResult

UpsertPoints

UpsertPointsBuilder

Usage

UuidIndexParams

UuidIndexParamsBuilder

Value

`Value` represents a dynamically typed value which can be either null, a number, a string, a boolean, a recursive struct value, or a list of values. A producer of value is expected to set one of those variants, absence of any variant indicates an error.

ValuesCount

Vector

Legacy vector format, which determines the vector type by the configuration of its fields.

VectorExample

VectorInput

Vector type to be used in queries. Ids will be substituted with their corresponding vectors from the collection.

VectorOutput

VectorParams

VectorParamsBuilder

VectorParamsDiff

VectorParamsDiffBuilder

VectorParamsDiffMap

VectorParamsMap

Vectors

VectorsConfig

VectorsConfigBuilder

VectorsConfigDiff

VectorsOutput

VectorsSelector

WalConfigDiff

WalConfigDiffBuilder

WithLookup

WithLookupBuilder

WithPayloadSelector

WithVectorsSelector

WriteOrdering

Enums§
------

BinaryQuantizationEncoding

CollectionStatus

CompressionRatio

CountPointsBuilderError

Error type for CountPointsBuilder

Datatype

Direction

Distance

FieldType

Fusion

Modifier

MultiVectorComparator

NullValue

`NullValue` is a singleton enumeration to represent the null value for the `Value` type union.

PayloadSchemaType

QuantizationType

ReadConsistencyType

RecommendExample

RecommendStrategy

How to use positive and negative vectors to find the results, default is `AverageVector`.

ReplicaState

ReshardingDirection

Resharding direction, scale up or down in number of shards

Sample

Sample points from the collection

ShardTransferMethod

ShardingMethod

SnapshotDownloadBuilderError

Error type for SnapshotDownloadBuilder

TokenizerType

UpdateStatus

WriteOrderingType