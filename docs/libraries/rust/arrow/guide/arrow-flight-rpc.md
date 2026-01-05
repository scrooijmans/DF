# Arrow Flight RPC — Apache Arrow v21.0.0
Arrow Flight is an RPC framework for high-performance data services based on Arrow data, and is built on top of [gRPC](https://grpc.io/) and the [IPC format](IPC.html).

Flight is organized around streams of Arrow record batches, being either downloaded from or uploaded to another service. A set of metadata methods offers discovery and introspection of streams, as well as the ability to implement application-specific methods.

Methods and message wire formats are defined by [Protobuf](https://developers.google.com/protocol-buffers/), enabling interoperability with clients that may support gRPC and Arrow separately, but not Flight. However, Flight implementations include further optimizations to avoid overhead in usage of Protobuf (mostly around avoiding excessive memory copies).

RPC Methods and Request Patterns[#](#rpc-methods-and-request-patterns "Permalink to this heading")
--------------------------------------------------------------------------------------------------

Flight defines a set of RPC methods for uploading/downloading data, retrieving metadata about a data stream, listing available data streams, and for implementing application-specific RPC methods. A Flight service implements some subset of these methods, while a Flight client can call any of these methods.

Data streams are identified by descriptors (the `FlightDescriptor` message), which are either a path or an arbitrary binary command. For instance, the descriptor may encode a SQL query, a path to a file on a distributed file system, or even a pickled Python object; the application can use this message as it sees fit.

Thus, one Flight client can connect to any service and perform basic operations. To facilitate this, Flight services are _expected_ to support some common request patterns, described next. Of course, applications may ignore compatibility and simply treat the Flight RPC methods as low-level building blocks for their own purposes.

See [Protocol Buffer Definitions](#protocol-buffer-definitions) for full details on the methods and messages involved.

### Downloading Data[#](#downloading-data "Permalink to this heading")

A client that wishes to download the data would:

Retrieving data via `DoGet`.[#](#id1 "Permalink to this image")

1.  Construct or acquire a `FlightDescriptor` for the data set they are interested in.
    
    A client may know what descriptor they want already, or they may use methods like `ListFlights` to discover them.
    
2.  Call `GetFlightInfo(FlightDescriptor)` to get a `FlightInfo` message.
    
    Flight does not require that data live on the same server as metadata. Hence, `FlightInfo` contains details on where the data is located, so the client can go fetch the data from an appropriate server. This is encoded as a series of `FlightEndpoint` messages inside `FlightInfo`. Each endpoint represents some location that contains a subset of the response data.
    
    An endpoint contains a list of locations (server addresses) where this data can be retrieved from, and a `Ticket`, an opaque binary token that the server will use to identify the data being requested.
    
    If `FlightInfo.ordered` is true, this signals there is some order between data from different endpoints. Clients should produce the same results as if the data returned from each of the endpoints was concatenated, in order, from front to back.
    
    If `FlightInfo.ordered` is false, the client may return data from any of the endpoints in arbitrary order. Data from any specific endpoint must be returned in order, but the data from different endpoints may be interleaved to allow parallel fetches.
    
    Note that since some clients may ignore `FlightInfo.ordered`, if ordering is important and client support cannot be ensured, servers should return a single endpoint.
    
    The response also contains other metadata, like the schema, and optionally an estimate of the dataset size.
    
3.  Consume each endpoint returned by the server.
    
    To consume an endpoint, the client should connect to one of the locations in the endpoint, then call `DoGet(Ticket)` with the ticket in the endpoint. This will give the client a stream of Arrow record batches.
    
    If the server wishes to indicate that the data is on the local server and not a different location, then it can return an empty list of locations. The client can then reuse the existing connection to the original server to fetch data. Otherwise, the client must connect to one of the indicated locations.
    
    The server may list “itself” as a location alongside other server locations. Normally this requires the server to know its public address, but it may also use the special URI string `arrow-flight-reuse-connection://?` to tell clients that they may reuse an existing connection to the same server, without having to be able to name itself. See [Connection Reuse](#connection-reuse) below.
    
    In this way, the locations inside an endpoint can also be thought of as performing look-aside load balancing or service discovery functions. And the endpoints can represent data that is partitioned or otherwise distributed.
    
    The client must consume all endpoints to retrieve the complete data set. The client can consume endpoints in any order, or even in parallel, or distribute the endpoints among multiple machines for consumption; this is up to the application to implement. The client can also use `FlightInfo.ordered`. See the previous item for details of `FlightInfo.ordered`.
    
    Each endpoint may have expiration time (`FlightEndpoint.expiration_time`). If an endpoint has expiration time, the client can get data multiple times by `DoGet` until the expiration time is reached. Otherwise, it is application-defined whether `DoGet` requests may be retried. The expiration time is represented as [google.protobuf.Timestamp](https://protobuf.dev/reference/protobuf/google.protobuf/#timestamp).
    
    If the expiration time is short, the client may be able to extend the expiration time by `RenewFlightEndpoint` action. The client need to use `DoAction` with `RenewFlightEndpoint` action type to extend the expiration time. `Action.body` must be `RenewFlightEndpointRequest` that has `FlightEndpoint` to be renewed.
    
    The client may be able to cancel the returned `FlightInfo` by `CancelFlightInfo` action. The client need to use `DoAction` with `CancelFlightInfo` action type to cancel the `FlightInfo`.
    

### Downloading Data by Running a Heavy Query[#](#downloading-data-by-running-a-heavy-query "Permalink to this heading")

A client may need to request a heavy query to download data. However, `GetFlightInfo` doesn’t return until the query completes, so the client is blocked. In this situation, the client can use `PollFlightInfo` instead of `GetFlightInfo`:

Polling a long-running query by `PollFlightInfo`.[#](#id2 "Permalink to this image")

1.  Construct or acquire a `FlightDescriptor`, as before.
    
2.  Call `PollFlightInfo(FlightDescriptor)` to get a `PollInfo` message.
    
    A server should respond as quickly as possible on the first call. So the client shouldn’t wait for the first `PollInfo` response.
    
    If the query isn’t finished, `PollInfo.flight_descriptor` has a `FlightDescriptor`. The client should use the descriptor (not the original `FlightDescriptor`) to call the next `PollFlightInfo()`. A server should recognize a `PollInfo.flight_descriptor` that is not necessarily the latest in case the client misses an update in between.
    
    If the query is finished, `PollInfo.flight_descriptor` is unset.
    
    `PollInfo.info` is the currently available results so far. It’s a complete `FlightInfo` each time not just the delta between the previous and current `FlightInfo`. A server should only append to the endpoints in `PollInfo.info` each time. So the client can run `DoGet(Ticket)` with the `Ticket` in the `PollInfo.info` even when the query isn’t finished yet. `FlightInfo.ordered` is also valid.
    
    A server should not respond until the result would be different from last time. That way, the client can “long poll” for updates without constantly making requests. Clients can set a short timeout to avoid blocking calls if desired.
    
    `PollInfo.progress` may be set. It represents progress of the query. If it’s set, the value must be in `[0.0, 1.0]`. The value is not necessarily monotonic or nondecreasing. A server may respond by only updating the `PollInfo.progress` value, though it shouldn’t spam the client with updates.
    
    `PollInfo.timestamp` is the expiration time for this request. After this passes, a server might not accept the poll descriptor anymore and the query may be cancelled. This may be updated on a call to `PollFlightInfo`. The expiration time is represented as [google.protobuf.Timestamp](https://protobuf.dev/reference/protobuf/google.protobuf/#timestamp).
    
    A client may be able to cancel the query by the `CancelFlightInfo` action.
    
    A server should return an error status instead of a response if the query fails. The client should not poll the request except for `TIMED_OUT` and `UNAVAILABLE`, which may not originate from the server.
    
3.  Consume each endpoint returned by the server, as before.
    

### Uploading Data[#](#uploading-data "Permalink to this heading")

To upload data, a client would:

Uploading data via `DoPut`.[#](#id3 "Permalink to this image")

1.  Construct or acquire a `FlightDescriptor`, as before.
    
2.  Call `DoPut(FlightData)` and upload a stream of Arrow record batches.
    
    The `FlightDescriptor` is included with the first message so the server can identify the dataset.
    

`DoPut` allows the server to send response messages back to the client with custom metadata. This can be used to implement things like resumable writes (e.g. the server can periodically send a message indicating how many rows have been committed so far).

### Exchanging Data[#](#exchanging-data "Permalink to this heading")

Some use cases may require uploading and downloading data within a single call. While this can be emulated with multiple calls, this may be difficult if the application is stateful. For instance, the application may wish to implement a call where the client uploads data and the server responds with a transformation of that data; this would require being stateful if implemented using `DoGet` and `DoPut`. Instead, `DoExchange` allows this to be implemented as a single call. A client would:

Complex data flow with `DoExchange`.[#](#id4 "Permalink to this image")

1.  Construct or acquire a `FlightDescriptor`, as before.
    
2.  Call `DoExchange(FlightData)`.
    
    The `FlightDescriptor` is included with the first message, as with `DoPut`. At this point, both the client and the server may simultaneously stream data to the other side.
    

Authentication[#](#authentication "Permalink to this heading")
--------------------------------------------------------------

Flight supports a variety of authentication methods that applications can customize for their needs.

“Handshake” authentication

This is implemented in two parts. At connection time, the client calls the `Handshake` RPC method, and the application-defined authentication handler can exchange any number of messages with its counterpart on the server. The handler then provides a binary token. The Flight client will then include this token in the headers of all future calls, which is validated by the server authentication handler.

Applications may use any part of this; for instance, they may ignore the initial handshake and send an externally acquired token (e.g. a bearer token) on each call, or they may establish trust during the handshake and not validate a token for each call, treating the connection as stateful (a “login” pattern).

Warning

Unless a token is validated on every call, this pattern is not secure, especially in the presence of a layer 7 load balancer, as is common with gRPC, or if gRPC transparently reconnects the client.

Header-based/middleware-based authentication

Clients may include custom headers with calls. Custom middleware can then be implemented to validate and accept/reject calls on the server side.

[Mutual TLS (mTLS)](https://grpc.io/docs/guides/auth/#supported-auth-mechanisms)

The client provides a certificate during connection establishment which is verified by the server. The application does not need to implement any authentication code, but must provision and distribute certificates.

This may only be available in certain implementations, and is only available when TLS is also enabled.

Some Flight implementations may expose the underlying gRPC API as well, in which case any [authentication method supported by gRPC](https://grpc.io/docs/guides/auth/) is available.

Location URIs[#](#location-uris "Permalink to this heading")
------------------------------------------------------------

Flight is primarily defined in terms of its Protobuf and gRPC specification below, but Arrow implementations may also support alternative transports (see [Flight RPC](about:blank/status.html#status-flight-rpc)). Clients and servers need to know which transport to use for a given URI in a Location, so Flight implementations should use the following URI schemes for the given transports:

Notes:

*   (1) See [Extended Location URIs](#flight-extended-uris) for semantics when using
    
    http/https as the transport. It should be accessible via a GET request.
    

### Connection Reuse[#](#connection-reuse "Permalink to this heading")

“Reuse connection” above is not a particular transport. Instead, it means that the client may try to execute DoGet against the same server (and through the same connection) that it originally obtained the FlightInfo from (i.e., that it called GetFlightInfo against). This is interpreted the same way as when no specific `Location` are returned.

This allows the server to return “itself” as one possible location to fetch data without having to know its own public address, which can be useful in deployments where knowing this would be difficult or impossible. For example, a developer may forward a remote service in a cloud environment to their local machine; in this case, the remote service would have no way to know the local hostname and port that it is being accessed over.

For compatibility reasons, the URI should always be `arrow-flight-reuse-connection://?`, with the trailing empty query string. Java’s URI implementation does not accept `scheme:` or `scheme://`, and C++’s implementation does not accept an empty string, so the obvious candidates are not compatible. The chosen representation can be parsed by both implementations, as well as Go’s `net/url` and Python’s `urllib.parse`.

### Extended Location URIs[#](#extended-location-uris "Permalink to this heading")

In addition to alternative transports, a server may also return URIs that reference an external service or object storage location. This can be useful in cases where intermediate data is cached as Apache Parquet files on cloud storage or is otherwise accessible via an HTTP service. In these scenarios, it is more efficient to be able to provide a URI where the client may simply download the data directly, rather than requiring a Flight service to read it back into memory and serve it from a `DoGet` request.

To avoid the complexities of Flight clients having to implement support for multiple different cloud storage vendors (e.g. AWS S3, Google Cloud), we extend the URIs to only allow an HTTP/HTTPS URI where the client can perform a simple GET request to download the data. Authentication can be handled either by negotiating externally to the Flight protocol or by the server sending a presigned URL that the client can make a GET request to. This should be supported by all current major cloud storage vendors, meaning only the server needs to know the semantics of the underlying object store APIs.

When using an extended location URI, the client should ignore any value in the `Ticket` field of the `FlightEndpoint`. The `Ticket` is only used for identifying data in the context of a Flight service, and is not needed when the client is directly downloading data from an external service.

Clients should assume that, unless otherwise specified, the data is being returned using the [Serialization and Interprocess Communication (IPC)](about:blank/Columnar.html#format-ipc) just as it would via a `DoGet` call. If the returned `Content-Type` header is a generic media type such as `application/octet-stream`, the client should still assume it is an Arrow IPC stream. For other media types, such as Apache Parquet, the server should use the appropriate IANA Media Type that a client would recognize.

Finally, the server may also allow the client to choose what format the data is returned in by respecting the `Accept` header in the request. If multiple formats are requested and supported, the choice of which to use is server-specific. If none of the requested content-types are supported, the server may respond with either 406 (Not Acceptable), 415 (Unsupported Media Type), or successfully respond with a different format that it does support, along with the correct `Content-Type` header.

Error Handling[#](#error-handling "Permalink to this heading")
--------------------------------------------------------------

Arrow Flight defines its own set of error codes. The implementation differs between languages (e.g. in C++, Unimplemented is a general Arrow error status while it’s a Flight-specific exception in Java), but the following set is exposed:

External Resources[#](#external-resources "Permalink to this heading")
----------------------------------------------------------------------

*   [https://arrow.apache.org/blog/2019/10/13/introducing-arrow-flight/](https://arrow.apache.org/blog/2019/10/13/introducing-arrow-flight/)
    
*   [https://arrow.apache.org/blog/2018/10/09/0.11.0-release/](https://arrow.apache.org/blog/2018/10/09/0.11.0-release/)
    
*   [https://www.slideshare.net/JacquesNadeau5/apache-arrow-flight-overview](https://www.slideshare.net/JacquesNadeau5/apache-arrow-flight-overview)
    

Protocol Buffer Definitions[#](#protocol-buffer-definitions "Permalink to this heading")
----------------------------------------------------------------------------------------

```
  1/*
  2 * Licensed to the Apache Software Foundation (ASF) under one
  3 * or more contributor license agreements.  See the NOTICE file
  4 * distributed with this work for additional information
  5 * regarding copyright ownership.  The ASF licenses this file
  6 * to you under the Apache License, Version 2.0 (the
  7 * "License"); you may not use this file except in compliance
  8 * with the License.  You may obtain a copy of the License at
  9 * <p>
 10 * http://www.apache.org/licenses/LICENSE-2.0
 11 * <p>
 12 * Unless required by applicable law or agreed to in writing, software
 13 * distributed under the License is distributed on an "AS IS" BASIS,
 14 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 15 * See the License for the specific language governing permissions and
 16 * limitations under the License.
 17 */
 18
 19syntax = "proto3";
 20import "google/protobuf/timestamp.proto";
 21
 22option java_package = "org.apache.arrow.flight.impl";
 23option go_package = "github.com/apache/arrow-go/arrow/flight/gen/flight";
 24option csharp_namespace = "Apache.Arrow.Flight.Protocol";
 25
 26package arrow.flight.protocol;
 27
 28/*
 29 * A flight service is an endpoint for retrieving or storing Arrow data. A
 30 * flight service can expose one or more predefined endpoints that can be
 31 * accessed using the Arrow Flight Protocol. Additionally, a flight service
 32 * can expose a set of actions that are available.
 33 */
 34service FlightService {
 35
 36  /*
 37   * Handshake between client and server. Depending on the server, the
 38   * handshake may be required to determine the token that should be used for
 39   * future operations. Both request and response are streams to allow multiple
 40   * round-trips depending on auth mechanism.
 41   */
 42  rpc Handshake(stream HandshakeRequest) returns (stream HandshakeResponse) {}
 43
 44  /*
 45   * Get a list of available streams given a particular criteria. Most flight
 46   * services will expose one or more streams that are readily available for
 47   * retrieval. This api allows listing the streams available for
 48   * consumption. A user can also provide a criteria. The criteria can limit
 49   * the subset of streams that can be listed via this interface. Each flight
 50   * service allows its own definition of how to consume criteria.
 51   */
 52  rpc ListFlights(Criteria) returns (stream FlightInfo) {}
 53
 54  /*
 55   * For a given FlightDescriptor, get information about how the flight can be
 56   * consumed. This is a useful interface if the consumer of the interface
 57   * already can identify the specific flight to consume. This interface can
 58   * also allow a consumer to generate a flight stream through a specified
 59   * descriptor. For example, a flight descriptor might be something that
 60   * includes a SQL statement or a Pickled Python operation that will be
 61   * executed. In those cases, the descriptor will not be previously available
 62   * within the list of available streams provided by ListFlights but will be
 63   * available for consumption for the duration defined by the specific flight
 64   * service.
 65   */
 66  rpc GetFlightInfo(FlightDescriptor) returns (FlightInfo) {}
 67
 68  /*
 69   * For a given FlightDescriptor, start a query and get information
 70   * to poll its execution status. This is a useful interface if the
 71   * query may be a long-running query. The first PollFlightInfo call
 72   * should return as quickly as possible. (GetFlightInfo doesn't
 73   * return until the query is complete.)
 74   *
 75   * A client can consume any available results before
 76   * the query is completed. See PollInfo.info for details.
 77   *
 78   * A client can poll the updated query status by calling
 79   * PollFlightInfo() with PollInfo.flight_descriptor. A server
 80   * should not respond until the result would be different from last
 81   * time. That way, the client can "long poll" for updates
 82   * without constantly making requests. Clients can set a short timeout
 83   * to avoid blocking calls if desired.
 84   *
 85   * A client can't use PollInfo.flight_descriptor after
 86   * PollInfo.expiration_time passes. A server might not accept the
 87   * retry descriptor anymore and the query may be cancelled.
 88   *
 89   * A client may use the CancelFlightInfo action with
 90   * PollInfo.info to cancel the running query.
 91   */
 92  rpc PollFlightInfo(FlightDescriptor) returns (PollInfo) {}
 93
 94  /*
 95   * For a given FlightDescriptor, get the Schema as described in Schema.fbs::Schema
 96   * This is used when a consumer needs the Schema of flight stream. Similar to
 97   * GetFlightInfo this interface may generate a new flight that was not previously
 98   * available in ListFlights.
 99   */
100   rpc GetSchema(FlightDescriptor) returns (SchemaResult) {}
101
102  /*
103   * Retrieve a single stream associated with a particular descriptor
104   * associated with the referenced ticket. A Flight can be composed of one or
105   * more streams where each stream can be retrieved using a separate opaque
106   * ticket that the flight service uses for managing a collection of streams.
107   */
108  rpc DoGet(Ticket) returns (stream FlightData) {}
109
110  /*
111   * Push a stream to the flight service associated with a particular
112   * flight stream. This allows a client of a flight service to upload a stream
113   * of data. Depending on the particular flight service, a client consumer
114   * could be allowed to upload a single stream per descriptor or an unlimited
115   * number. In the latter, the service might implement a 'seal' action that
116   * can be applied to a descriptor once all streams are uploaded.
117   */
118  rpc DoPut(stream FlightData) returns (stream PutResult) {}
119
120  /*
121   * Open a bidirectional data channel for a given descriptor. This
122   * allows clients to send and receive arbitrary Arrow data and
123   * application-specific metadata in a single logical stream. In
124   * contrast to DoGet/DoPut, this is more suited for clients
125   * offloading computation (rather than storage) to a Flight service.
126   */
127  rpc DoExchange(stream FlightData) returns (stream FlightData) {}
128
129  /*
130   * Flight services can support an arbitrary number of simple actions in
131   * addition to the possible ListFlights, GetFlightInfo, DoGet, DoPut
132   * operations that are potentially available. DoAction allows a flight client
133   * to do a specific action against a flight service. An action includes
134   * opaque request and response objects that are specific to the type action
135   * being undertaken.
136   */
137  rpc DoAction(Action) returns (stream Result) {}
138
139  /*
140   * A flight service exposes all of the available action types that it has
141   * along with descriptions. This allows different flight consumers to
142   * understand the capabilities of the flight service.
143   */
144  rpc ListActions(Empty) returns (stream ActionType) {}
145}
146
147/*
148 * The request that a client provides to a server on handshake.
149 */
150message HandshakeRequest {
151
152  /*
153   * A defined protocol version
154   */
155  uint64 protocol_version = 1;
156
157  /*
158   * Arbitrary auth/handshake info.
159   */
160  bytes payload = 2;
161}
162
163message HandshakeResponse {
164
165  /*
166   * A defined protocol version
167   */
168  uint64 protocol_version = 1;
169
170  /*
171   * Arbitrary auth/handshake info.
172   */
173  bytes payload = 2;
174}
175
176/*
177 * A message for doing simple auth.
178 */
179message BasicAuth {
180  string username = 2;
181  string password = 3;
182}
183
184message Empty {}
185
186/*
187 * Describes an available action, including both the name used for execution
188 * along with a short description of the purpose of the action.
189 */
190message ActionType {
191  string type = 1;
192  string description = 2;
193}
194
195/*
196 * A service specific expression that can be used to return a limited set
197 * of available Arrow Flight streams.
198 */
199message Criteria {
200  bytes expression = 1;
201}
202
203/*
204 * An opaque action specific for the service.
205 */
206message Action {
207  string type = 1;
208  bytes body = 2;
209}
210
211/*
212 * An opaque result returned after executing an action.
213 */
214message Result {
215  bytes body = 1;
216}
217
218/*
219 * Wrap the result of a getSchema call
220 */
221message SchemaResult {
222  // The schema of the dataset in its IPC form:
223  //   4 bytes - an optional IPC_CONTINUATION_TOKEN prefix
224  //   4 bytes - the byte length of the payload
225  //   a flatbuffer Message whose header is the Schema
226  bytes schema = 1;
227}
228
229/*
230 * The name or tag for a Flight. May be used as a way to retrieve or generate
231 * a flight or be used to expose a set of previously defined flights.
232 */
233message FlightDescriptor {
234
235  /*
236   * Describes what type of descriptor is defined.
237   */
238  enum DescriptorType {
239
240    // Protobuf pattern, not used.
241    UNKNOWN = 0;
242
243    /*
244     * A named path that identifies a dataset. A path is composed of a string
245     * or list of strings describing a particular dataset. This is conceptually
246     *  similar to a path inside a filesystem.
247     */
248    PATH = 1;
249
250    /*
251     * An opaque command to generate a dataset.
252     */
253    CMD = 2;
254  }
255
256  DescriptorType type = 1;
257
258  /*
259   * Opaque value used to express a command. Should only be defined when
260   * type = CMD.
261   */
262  bytes cmd = 2;
263
264  /*
265   * List of strings identifying a particular dataset. Should only be defined
266   * when type = PATH.
267   */
268  repeated string path = 3;
269}
270
271/*
272 * The access coordinates for retrieval of a dataset. With a FlightInfo, a
273 * consumer is able to determine how to retrieve a dataset.
274 */
275message FlightInfo {
276  // The schema of the dataset in its IPC form:
277  //   4 bytes - an optional IPC_CONTINUATION_TOKEN prefix
278  //   4 bytes - the byte length of the payload
279  //   a flatbuffer Message whose header is the Schema
280  bytes schema = 1;
281
282  /*
283   * The descriptor associated with this info.
284   */
285  FlightDescriptor flight_descriptor = 2;
286
287  /*
288   * A list of endpoints associated with the flight. To consume the
289   * whole flight, all endpoints (and hence all Tickets) must be
290   * consumed. Endpoints can be consumed in any order.
291   *
292   * In other words, an application can use multiple endpoints to
293   * represent partitioned data.
294   *
295   * If the returned data has an ordering, an application can use
296   * "FlightInfo.ordered = true" or should return the all data in a
297   * single endpoint. Otherwise, there is no ordering defined on
298   * endpoints or the data within.
299   *
300   * A client can read ordered data by reading data from returned
301   * endpoints, in order, from front to back.
302   *
303   * Note that a client may ignore "FlightInfo.ordered = true". If an
304   * ordering is important for an application, an application must
305   * choose one of them:
306   *
307   * * An application requires that all clients must read data in
308   *   returned endpoints order.
309   * * An application must return the all data in a single endpoint.
310   */
311  repeated FlightEndpoint endpoint = 3;
312
313  // Set these to -1 if unknown.
314  int64 total_records = 4;
315  int64 total_bytes = 5;
316
317  /*
318   * FlightEndpoints are in the same order as the data.
319   */
320  bool ordered = 6;
321
322  /*
323   * Application-defined metadata.
324   *
325   * There is no inherent or required relationship between this
326   * and the app_metadata fields in the FlightEndpoints or resulting
327   * FlightData messages. Since this metadata is application-defined,
328   * a given application could define there to be a relationship,
329   * but there is none required by the spec.
330   */
331  bytes app_metadata = 7;
332}
333
334/*
335 * The information to process a long-running query.
336 */
337message PollInfo {
338  /*
339   * The currently available results.
340   *
341   * If "flight_descriptor" is not specified, the query is complete
342   * and "info" specifies all results. Otherwise, "info" contains
343   * partial query results.
344   *
345   * Note that each PollInfo response contains a complete
346   * FlightInfo (not just the delta between the previous and current
347   * FlightInfo).
348   *
349   * Subsequent PollInfo responses may only append new endpoints to
350   * info.
351   *
352   * Clients can begin fetching results via DoGet(Ticket) with the
353   * ticket in the info before the query is
354   * completed. FlightInfo.ordered is also valid.
355   */
356  FlightInfo info = 1;
357
358  /*
359   * The descriptor the client should use on the next try.
360   * If unset, the query is complete.
361   */
362  FlightDescriptor flight_descriptor = 2;
363
364  /*
365   * Query progress. If known, must be in [0.0, 1.0] but need not be
366   * monotonic or nondecreasing. If unknown, do not set.
367   */
368  optional double progress = 3;
369
370  /*
371   * Expiration time for this request. After this passes, the server
372   * might not accept the retry descriptor anymore (and the query may
373   * be cancelled). This may be updated on a call to PollFlightInfo.
374   */
375  google.protobuf.Timestamp expiration_time = 4;
376}
377
378/*
379 * The request of the CancelFlightInfo action.
380 *
381 * The request should be stored in Action.body.
382 */
383message CancelFlightInfoRequest {
384  FlightInfo info = 1;
385}
386
387/*
388 * The result of a cancel operation.
389 *
390 * This is used by CancelFlightInfoResult.status.
391 */
392enum CancelStatus {
393  // The cancellation status is unknown. Servers should avoid using
394  // this value (send a NOT_FOUND error if the requested query is
395  // not known). Clients can retry the request.
396  CANCEL_STATUS_UNSPECIFIED = 0;
397  // The cancellation request is complete. Subsequent requests with
398  // the same payload may return CANCELLED or a NOT_FOUND error.
399  CANCEL_STATUS_CANCELLED = 1;
400  // The cancellation request is in progress. The client may retry
401  // the cancellation request.
402  CANCEL_STATUS_CANCELLING = 2;
403  // The query is not cancellable. The client should not retry the
404  // cancellation request.
405  CANCEL_STATUS_NOT_CANCELLABLE = 3;
406}
407
408/*
409 * The result of the CancelFlightInfo action.
410 *
411 * The result should be stored in Result.body.
412 */
413message CancelFlightInfoResult {
414  CancelStatus status = 1;
415}
416
417/*
418 * An opaque identifier that the service can use to retrieve a particular
419 * portion of a stream.
420 *
421 * Tickets are meant to be single use. It is an error/application-defined
422 * behavior to reuse a ticket.
423 */
424message Ticket {
425  bytes ticket = 1;
426}
427
428/*
429 * A location to retrieve a particular stream from. This URI should be one of
430 * the following:
431 *  - An empty string or the string 'arrow-flight-reuse-connection://?':
432 *    indicating that the ticket can be redeemed on the service where the
433 *    ticket was generated via a DoGet request.
434 *  - A valid grpc URI (grpc://, grpc+tls://, grpc+unix://, etc.):
435 *    indicating that the ticket can be redeemed on the service at the given
436 *    URI via a DoGet request.
437 *  - A valid HTTP URI (http://, https://, etc.):
438 *    indicating that the client should perform a GET request against the
439 *    given URI to retrieve the stream. The ticket should be empty
440 *    in this case and should be ignored by the client. Cloud object storage
441 *    can be utilized by presigned URLs or mediating the auth separately and
442 *    returning the full URL (e.g. https://amzn-s3-demo-bucket.s3.us-west-2.amazonaws.com/...).
443 *
444 * We allow non-Flight URIs for the purpose of allowing Flight services to indicate that
445 * results can be downloaded in formats other than Arrow (such as Parquet) or to allow
446 * direct fetching of results from a URI to reduce excess copying and data movement.
447 * In these cases, the following conventions should be followed by servers and clients:
448 *
449 *  - Unless otherwise specified by the 'Content-Type' header of the response,
450 *    a client should assume the response is using the Arrow IPC Streaming format.
451 *    Usage of an IANA media type like 'application/octet-stream' should be assumed to
452 *    be using the Arrow IPC Streaming format.
453 *  - The server may allow the client to choose a specific response format by
454 *    specifying an 'Accept' header in the request, such as 'application/vnd.apache.parquet'
455 *    or 'application/vnd.apache.arrow.stream'. If multiple types are requested and
456 *    supported by the server, the choice of which to use is server-specific. If
457 *    none of the requested content-types are supported, the server may respond with
458 *    either 406 (Not Acceptable) or 415 (Unsupported Media Type), or successfully
459 *    respond with a different format that it does support along with the correct
460 *    'Content-Type' header.
461 *
462 * Note: new schemes may be proposed in the future to allow for more flexibility based
463 * on community requests.
464 */
465message Location {
466  string uri = 1;
467}
468
469/*
470 * A particular stream or split associated with a flight.
471 */
472message FlightEndpoint {
473
474  /*
475   * Token used to retrieve this stream.
476   */
477  Ticket ticket = 1;
478
479  /*
480   * A list of URIs where this ticket can be redeemed via DoGet().
481   *
482   * If the list is empty, the expectation is that the ticket can only
483   * be redeemed on the current service where the ticket was
484   * generated.
485   *
486   * If the list is not empty, the expectation is that the ticket can be
487   * redeemed at any of the locations, and that the data returned will be
488   * equivalent. In this case, the ticket may only be redeemed at one of the
489   * given locations, and not (necessarily) on the current service. If one
490   * of the given locations is "arrow-flight-reuse-connection://?", the
491   * client may redeem the ticket on the service where the ticket was
492   * generated (i.e., the same as above), in addition to the other
493   * locations. (This URI was chosen to maximize compatibility, as 'scheme:'
494   * or 'scheme://' are not accepted by Java's java.net.URI.)
495   *
496   * In other words, an application can use multiple locations to
497   * represent redundant and/or load balanced services.
498   */
499  repeated Location location = 2;
500
501  /*
502   * Expiration time of this stream. If present, clients may assume
503   * they can retry DoGet requests. Otherwise, it is
504   * application-defined whether DoGet requests may be retried.
505   */
506  google.protobuf.Timestamp expiration_time = 3;
507
508  /*
509   * Application-defined metadata.
510   *
511   * There is no inherent or required relationship between this
512   * and the app_metadata fields in the FlightInfo or resulting
513   * FlightData messages. Since this metadata is application-defined,
514   * a given application could define there to be a relationship,
515   * but there is none required by the spec.
516   */
517  bytes app_metadata = 4;
518}
519
520/*
521 * The request of the RenewFlightEndpoint action.
522 *
523 * The request should be stored in Action.body.
524 */
525message RenewFlightEndpointRequest {
526  FlightEndpoint endpoint = 1;
527}
528
529/*
530 * A batch of Arrow data as part of a stream of batches.
531 */
532message FlightData {
533
534  /*
535   * The descriptor of the data. This is only relevant when a client is
536   * starting a new DoPut stream.
537   */
538  FlightDescriptor flight_descriptor = 1;
539
540  /*
541   * Header for message data as described in Message.fbs::Message.
542   */
543  bytes data_header = 2;
544
545  /*
546   * Application-defined metadata.
547   */
548  bytes app_metadata = 3;
549
550  /*
551   * The actual batch of Arrow data. Preferably handled with minimal-copies
552   * coming last in the definition to help with sidecar patterns (it is
553   * expected that some implementations will fetch this field off the wire
554   * with specialized code to avoid extra memory copies).
555   */
556  bytes data_body = 1000;
557}
558
559/**
560 * The response message associated with the submission of a DoPut.
561 */
562message PutResult {
563  bytes app_metadata = 1;
564}
565
566/*
567 * EXPERIMENTAL: Union of possible value types for a Session Option to be set to.
568 *
569 * By convention, an attempt to set a valueless SessionOptionValue should
570 * attempt to unset or clear the named option value on the server.
571 */
572message SessionOptionValue {
573  message StringListValue {
574    repeated string values = 1;
575  }
576
577  oneof option_value {
578    string string_value = 1;
579    bool bool_value = 2;
580    sfixed64 int64_value = 3;
581    double double_value = 4;
582    StringListValue string_list_value = 5;
583  }
584}
585
586/*
587 * EXPERIMENTAL: A request to set session options for an existing or new (implicit)
588 * server session.
589 *
590 * Sessions are persisted and referenced via a transport-level state management, typically
591 * RFC 6265 HTTP cookies when using an HTTP transport.  The suggested cookie name or state
592 * context key is 'arrow_flight_session_id', although implementations may freely choose their
593 * own name.
594 *
595 * Session creation (if one does not already exist) is implied by this RPC request, however
596 * server implementations may choose to initiate a session that also contains client-provided
597 * session options at any other time, e.g. on authentication, or when any other call is made
598 * and the server wishes to use a session to persist any state (or lack thereof).
599 */
600message SetSessionOptionsRequest {
601  map<string, SessionOptionValue> session_options = 1;
602}
603
604/*
605 * EXPERIMENTAL: The results (individually) of setting a set of session options.
606 *
607 * Option names should only be present in the response if they were not successfully
608 * set on the server; that is, a response without an Error for a name provided in the
609 * SetSessionOptionsRequest implies that the named option value was set successfully.
610 */
611message SetSessionOptionsResult {
612  enum ErrorValue {
613    // Protobuf deserialization fallback value: The status is unknown or unrecognized.
614    // Servers should avoid using this value. The request may be retried by the client.
615    UNSPECIFIED = 0;
616    // The given session option name is invalid.
617    INVALID_NAME = 1;
618    // The session option value or type is invalid.
619    INVALID_VALUE = 2;
620    // The session option cannot be set.
621    ERROR = 3;
622  }
623
624  message Error {
625    ErrorValue value = 1;
626  }
627
628  map<string, Error> errors = 1;
629}
630
631/*
632 * EXPERIMENTAL: A request to access the session options for the current server session.
633 *
634 * The existing session is referenced via a cookie header or similar (see
635 * SetSessionOptionsRequest above); it is an error to make this request with a missing,
636 * invalid, or expired session cookie header or other implementation-defined session
637 * reference token.
638 */
639message GetSessionOptionsRequest {
640}
641
642/*
643 * EXPERIMENTAL: The result containing the current server session options.
644 */
645message GetSessionOptionsResult {
646    map<string, SessionOptionValue> session_options = 1;
647}
648
649/*
650 * Request message for the "Close Session" action.
651 *
652 * The exiting session is referenced via a cookie header.
653 */
654message CloseSessionRequest {
655}
656
657/*
658 * The result of closing a session.
659 */
660message CloseSessionResult {
661  enum Status {
662    // Protobuf deserialization fallback value: The session close status is unknown or
663    // not recognized. Servers should avoid using this value (send a NOT_FOUND error if
664    // the requested session is not known or expired). Clients can retry the request.
665    UNSPECIFIED = 0;
666    // The session close request is complete. Subsequent requests with
667    // the same session produce a NOT_FOUND error.
668    CLOSED = 1;
669    // The session close request is in progress. The client may retry
670    // the close request.
671    CLOSING = 2;
672    // The session is not closeable. The client should not retry the
673    // close request.
674    NOT_CLOSEABLE = 3;
675  }
676
677  Status status = 1;
678}

```
