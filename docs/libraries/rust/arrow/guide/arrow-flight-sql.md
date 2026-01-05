# Arrow Flight SQL — Apache Arrow v21.0.0
Arrow Flight SQL is a protocol for interacting with SQL databases using the Arrow in-memory format and the [Flight RPC](Flight.html) framework.

Generally, a database will implement the RPC methods according to the specification, but does not need to implement a client-side driver. A database client can use the provided Flight SQL client to interact with any database that supports the necessary endpoints. Flight SQL clients wrap the underlying Flight client to provide methods for the new RPC methods described here.

RPC Methods[#](#rpc-methods "Permalink to this heading")
--------------------------------------------------------

Flight SQL reuses the predefined RPC methods in Arrow Flight, and provides various commands that pair those methods with request/response messages defined via Protobuf (see below).

### SQL Metadata[#](#sql-metadata "Permalink to this heading")

Flight SQL provides a variety of commands to fetch catalog metadata about the database server.

All of these commands can be used with the GetFlightInfo and GetSchema RPC methods. The Protobuf request message should be packed into a google.protobuf.Any message, then serialized and packed as the `cmd` field in a CMD-type FlightDescriptor.

If the command is used with GetFlightInfo, the server will return a FlightInfo response. The client should then use the Ticket(s) in the FlightInfo with the DoGet RPC method to fetch a Arrow data containing the results of the command. In other words, SQL metadata is returned as Arrow data, just like query results themselves.

The Arrow schema returned by GetSchema or DoGet for a particular command is fixed according to the specification.

`CommandGetCatalogs`

List the catalogs available in the database. The definition varies by vendor.

`CommandGetCrossReference`

List the foreign key columns in a given table that reference columns in a given parent table.

`CommandGetDbSchemas`

List the schemas (note: a grouping of tables, _not_ an Arrow schema) available in the database. The definition varies by vendor.

`CommandGetExportedKeys`

List foreign key columns that reference the primary key columns of a given table.

`CommandGetImportedKeys`

List foreign keys of a given table.

`CommandGetPrimaryKeys`

List the primary keys of a given table.

`CommandGetSqlInfo`

Fetch metadata about the database server and its supported SQL features.

`CommandGetTables`

List tables in the database.

`CommandGetTableTypes`

List table types in the database. The list of types varies by vendor.

### Query Execution[#](#query-execution "Permalink to this heading")

Flight SQL also provides commands to execute SQL queries and manage prepared statements.

Many of these commands are also used with GetFlightInfo/GetSchema and work identically to the metadata methods above. Some of these commands can be used with the DoPut RPC method, but the command should still be encoded in the request FlightDescriptor in the same way.

Commands beginning with “Action” are instead used with the DoAction RPC method, in which case the command should be packed into a google.protobuf.Any message, then serialized and packed into the `body` of a Flight Action. Also, the action `type` should be set to the command name (i.e. for `ActionClosePreparedStatementRequest`, the `type` should be `ClosePreparedStatement`).

Commands that execute updates such as `CommandStatementUpdate` and `CommandStatementIngest` return a Flight SQL `DoPutUpdateResult` after consuming the entire FlightData stream. This message is encoded in the `app_metadata` field of the Flight RPC `PutResult` returned.

`ActionClosePreparedStatementRequest`

Close a previously created prepared statement.

`ActionCreatePreparedStatementRequest`

Create a new prepared statement for a SQL query.

The response will contain an opaque handle used to identify the prepared statement. It may also contain two optional schemas: the Arrow schema of the result set, and the Arrow schema of the bind parameters (if any). Because the schema of the result set may depend on the bind parameters, the schemas may not necessarily be provided here as a result, or if provided, they may not be accurate. Clients should not assume the schema provided here will be the schema of any data actually returned by executing the prepared statement.

Some statements may have bind parameters without any specific type. (As a trivial example for SQL, consider `SELECT ?`.) It is not currently specified how this should be handled in the bind parameter schema above. We suggest either using a union type to enumerate the possible types, or using the NA (null) type as a wildcard/placeholder.

`CommandPreparedStatementQuery`

Execute a previously created prepared statement and get the results.

When used with DoPut: binds parameter values to the prepared statement. The server may optionally provide an updated handle in the response. Updating the handle allows the client to supply all state required to execute the query in an ActionPreparedStatementExecute message. For example, stateless servers can encode the bound parameter values into the new handle, and the client will send that new handle with parameters back to the server.

Note that a handle returned from a DoPut call with CommandPreparedStatementQuery can itself be passed to a subsequent DoPut call with CommandPreparedStatementQuery to bind a new set of parameters. The subsequent call itself may return an updated handle which again should be used for subsequent requests.

The server is responsible for detecting the case where the client does not use the updated handle and should return an error.

When used with GetFlightInfo: execute the prepared statement. The prepared statement can be reused after fetching results.

When used with GetSchema: get the expected Arrow schema of the result set. If the client has bound parameter values with DoPut previously, the server should take those values into account.

`CommandPreparedStatementUpdate`

Execute a previously created prepared statement that does not return results.

When used with DoPut: execute the query and return the number of affected rows. The prepared statement can be reused afterwards.

`CommandStatementQuery`

Execute an ad-hoc SQL query.

When used with GetFlightInfo: execute the query (call DoGet to fetch results).

When used with GetSchema: return the schema of the query results.

`CommandStatementUpdate`

Execute an ad-hoc SQL query that does not return results.

When used with DoPut: execute the query and return the number of affected rows.

`CommandStatementIngest`

Execute a bulk ingestion.

When used with DoPut: load the stream of Arrow record batches into the specified target table and return the number of rows ingested via a `DoPutUpdateResult` message.

### Flight Server Session Management[#](#flight-server-session-management "Permalink to this heading")

Flight SQL provides commands to set and update server session variables which affect the server behaviour in various ways. Common options may include (depending on the server implementation) `catalog` and `schema`, indicating the currently-selected catalog and schema for queries to be run against.

Clients should prefer, where possible, setting options prior to issuing queries and other commands, as some server implementations may require these options be set exactly once and prior to any other activity which may trigger their implicit setting.

For compatibility with Database Connectivity drivers (JDBC, ODBC, and others), it is strongly recommended that server implementations accept string representations of all option values which may be provided to the driver as part of a server connection string and passed through to the server without further conversion. For ease of use it is also recommended to accept and convert other numeric types to the preferred type for an option value, however this is not required.

Sessions are persisted between the client and server using an implementation-defined mechanism, which is typically RFC 6265 cookies. Servers may also combine other connection state opaquely with the session token: Consider that the lifespan and semantics of a session should make sense for any additional uses, e.g. CloseSession would also invalidate any authentication context persisted via the session context. A session may be initiated upon a nonempty (or empty) SetSessionOptions call, or at any other time of the server’s choosing.

`SetSessionOptions` Set server session option(s) by name/value.

`GetSessionOptions` Get the current server session options, including those set by the client and any defaulted or implicitly set by the server.

`CloseSession` Close and invalidate the current session context.

Sequence Diagrams[#](#sequence-diagrams "Permalink to this heading")
--------------------------------------------------------------------

Listing available tables.[#](#id1 "Permalink to this image")

Executing an ad-hoc query.[#](#id2 "Permalink to this image")

Creating a prepared statement, then executing it.[#](#id3 "Permalink to this image")

Executing a bulk ingestion.[#](#id4 "Permalink to this image")

External Resources[#](#external-resources "Permalink to this heading")
----------------------------------------------------------------------

*   [Introducing Apache Arrow Flight SQL: Accelerating Database Access](https://arrow.apache.org/blog/2022/02/16/introducing-arrow-flight-sql/)
    

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
  20import "google/protobuf/descriptor.proto";
  21
  22option java_package = "org.apache.arrow.flight.sql.impl";
  23option go_package = "github.com/apache/arrow-go/arrow/flight/gen/flight";
  24package arrow.flight.protocol.sql;
  25
  26/*
  27 * Represents a metadata request. Used in the command member of FlightDescriptor
  28 * for the following RPC calls:
  29 *  - GetSchema: return the Arrow schema of the query.
  30 *  - GetFlightInfo: execute the metadata request.
  31 *
  32 * The returned Arrow schema will be:
  33 * <
  34 *  info_name: uint32 not null,
  35 *  value: dense_union<
  36 *              string_value: utf8,
  37 *              bool_value: bool,
  38 *              bigint_value: int64,
  39 *              int32_bitmask: int32,
  40 *              string_list: list<string_data: utf8>
  41 *              int32_to_int32_list_map: map<key: int32, value: list<$data$: int32>>
  42 * >
  43 * where there is one row per requested piece of metadata information.
  44 */
  45message CommandGetSqlInfo {
  46
  47  /*
  48   * Values are modelled after ODBC's SQLGetInfo() function. This information is intended to provide
  49   * Flight SQL clients with basic, SQL syntax and SQL functions related information.
  50   * More information types can be added in future releases.
  51   * E.g. more SQL syntax support types, scalar functions support, type conversion support etc.
  52   *
  53   * Note that the set of metadata may expand.
  54   *
  55   * Initially, Flight SQL will support the following information types:
  56   * - Server Information - Range [0-500)
  57   * - Syntax Information - Range [500-1000)
  58   * Range [0-10,000) is reserved for defaults (see SqlInfo enum for default options).
  59   * Custom options should start at 10,000.
  60   *
  61   * If omitted, then all metadata will be retrieved.
  62   * Flight SQL Servers may choose to include additional metadata above and beyond the specified set, however they must
  63   * at least return the specified set. IDs ranging from 0 to 10,000 (exclusive) are reserved for future use.
  64   * If additional metadata is included, the metadata IDs should start from 10,000.
  65   */
  66  repeated uint32 info = 1;
  67}
  68
  69// Options for CommandGetSqlInfo.
  70enum SqlInfo {
  71
  72  // Server Information [0-500): Provides basic information about the Flight SQL Server.
  73
  74  // Retrieves a UTF-8 string with the name of the Flight SQL Server.
  75  FLIGHT_SQL_SERVER_NAME = 0;
  76
  77  // Retrieves a UTF-8 string with the native version of the Flight SQL Server.
  78  FLIGHT_SQL_SERVER_VERSION = 1;
  79
  80  // Retrieves a UTF-8 string with the Arrow format version of the Flight SQL Server.
  81  FLIGHT_SQL_SERVER_ARROW_VERSION = 2;
  82
  83  /*
  84   * Retrieves a boolean value indicating whether the Flight SQL Server is read only.
  85   *
  86   * Returns:
  87   * - false: if read-write
  88   * - true: if read only
  89   */
  90  FLIGHT_SQL_SERVER_READ_ONLY = 3;
  91
  92  /*
  93   * Retrieves a boolean value indicating whether the Flight SQL Server supports executing
  94   * SQL queries.
  95   *
  96   * Note that the absence of this info (as opposed to a false value) does not necessarily
  97   * mean that SQL is not supported, as this property was not originally defined.
  98   */
  99  FLIGHT_SQL_SERVER_SQL = 4;
 100
 101  /*
 102   * Retrieves a boolean value indicating whether the Flight SQL Server supports executing
 103   * Substrait plans.
 104   */
 105  FLIGHT_SQL_SERVER_SUBSTRAIT = 5;
 106
 107  /*
 108   * Retrieves a string value indicating the minimum supported Substrait version, or null
 109   * if Substrait is not supported.
 110   */
 111  FLIGHT_SQL_SERVER_SUBSTRAIT_MIN_VERSION = 6;
 112
 113  /*
 114   * Retrieves a string value indicating the maximum supported Substrait version, or null
 115   * if Substrait is not supported.
 116   */
 117  FLIGHT_SQL_SERVER_SUBSTRAIT_MAX_VERSION = 7;
 118
 119  /*
 120   * Retrieves an int32 indicating whether the Flight SQL Server supports the
 121   * BeginTransaction/EndTransaction/BeginSavepoint/EndSavepoint actions.
 122   *
 123   * Even if this is not supported, the database may still support explicit "BEGIN
 124   * TRANSACTION"/"COMMIT" SQL statements (see SQL_TRANSACTIONS_SUPPORTED); this property
 125   * is only about whether the server implements the Flight SQL API endpoints.
 126   *
 127   * The possible values are listed in `SqlSupportedTransaction`.
 128   */
 129  FLIGHT_SQL_SERVER_TRANSACTION = 8;
 130
 131  /*
 132   * Retrieves a boolean value indicating whether the Flight SQL Server supports explicit
 133   * query cancellation (the CancelQuery action).
 134   */
 135  FLIGHT_SQL_SERVER_CANCEL = 9;
 136
 137  /*
 138   * Retrieves a boolean value indicating whether the Flight SQL Server supports executing
 139   * bulk ingestion.
 140   */
 141   FLIGHT_SQL_SERVER_BULK_INGESTION = 10;
 142
 143  /*
 144   * Retrieves a boolean value indicating whether transactions are supported for bulk ingestion. If not, invoking
 145   * the method commit in the context of a bulk ingestion is a noop, and the isolation level is
 146   * `arrow.flight.protocol.sql.SqlTransactionIsolationLevel.TRANSACTION_NONE`.
 147   *
 148   * Returns:
 149   * - false: if bulk ingestion transactions are unsupported;
 150   * - true: if bulk ingestion transactions are supported.
 151   */
 152   FLIGHT_SQL_SERVER_INGEST_TRANSACTIONS_SUPPORTED = 11;
 153
 154  /*
 155   * Retrieves an int32 indicating the timeout (in milliseconds) for prepared statement handles.
 156   *
 157   * If 0, there is no timeout.  Servers should reset the timeout when the handle is used in a command.
 158   */
 159  FLIGHT_SQL_SERVER_STATEMENT_TIMEOUT = 100;
 160
 161  /*
 162   * Retrieves an int32 indicating the timeout (in milliseconds) for transactions, since transactions are not tied to a connection.
 163   *
 164   * If 0, there is no timeout.  Servers should reset the timeout when the handle is used in a command.
 165   */
 166  FLIGHT_SQL_SERVER_TRANSACTION_TIMEOUT = 101;
 167
 168  // SQL Syntax Information [500-1000): provides information about SQL syntax supported by the Flight SQL Server.
 169
 170  /*
 171   * Retrieves a boolean value indicating whether the Flight SQL Server supports CREATE and DROP of catalogs.
 172   *
 173   * Returns:
 174   * - false: if it doesn't support CREATE and DROP of catalogs.
 175   * - true: if it supports CREATE and DROP of catalogs.
 176   */
 177  SQL_DDL_CATALOG = 500;
 178
 179  /*
 180   * Retrieves a boolean value indicating whether the Flight SQL Server supports CREATE and DROP of schemas.
 181   *
 182   * Returns:
 183   * - false: if it doesn't support CREATE and DROP of schemas.
 184   * - true: if it supports CREATE and DROP of schemas.
 185   */
 186  SQL_DDL_SCHEMA = 501;
 187
 188  /*
 189   * Indicates whether the Flight SQL Server supports CREATE and DROP of tables.
 190   *
 191   * Returns:
 192   * - false: if it doesn't support CREATE and DROP of tables.
 193   * - true: if it supports CREATE and DROP of tables.
 194   */
 195  SQL_DDL_TABLE = 502;
 196
 197  /*
 198   * Retrieves a int32 ordinal representing the case sensitivity of catalog, table, schema and table names.
 199   *
 200   * The possible values are listed in `arrow.flight.protocol.sql.SqlSupportedCaseSensitivity`.
 201   */
 202  SQL_IDENTIFIER_CASE = 503;
 203
 204  // Retrieves a UTF-8 string with the supported character(s) used to surround a delimited identifier.
 205  SQL_IDENTIFIER_QUOTE_CHAR = 504;
 206
 207  /*
 208   * Retrieves a int32 describing the case sensitivity of quoted identifiers.
 209   *
 210   * The possible values are listed in `arrow.flight.protocol.sql.SqlSupportedCaseSensitivity`.
 211   */
 212  SQL_QUOTED_IDENTIFIER_CASE = 505;
 213
 214  /*
 215   * Retrieves a boolean value indicating whether all tables are selectable.
 216   *
 217   * Returns:
 218   * - false: if not all tables are selectable or if none are;
 219   * - true: if all tables are selectable.
 220   */
 221  SQL_ALL_TABLES_ARE_SELECTABLE = 506;
 222
 223  /*
 224   * Retrieves the null ordering.
 225   *
 226   * Returns a int32 ordinal for the null ordering being used, as described in
 227   * `arrow.flight.protocol.sql.SqlNullOrdering`.
 228   */
 229  SQL_NULL_ORDERING = 507;
 230
 231  // Retrieves a UTF-8 string list with values of the supported keywords.
 232  SQL_KEYWORDS = 508;
 233
 234  // Retrieves a UTF-8 string list with values of the supported numeric functions.
 235  SQL_NUMERIC_FUNCTIONS = 509;
 236
 237  // Retrieves a UTF-8 string list with values of the supported string functions.
 238  SQL_STRING_FUNCTIONS = 510;
 239
 240  // Retrieves a UTF-8 string list with values of the supported system functions.
 241  SQL_SYSTEM_FUNCTIONS = 511;
 242
 243  // Retrieves a UTF-8 string list with values of the supported datetime functions.
 244  SQL_DATETIME_FUNCTIONS = 512;
 245
 246  /*
 247   * Retrieves the UTF-8 string that can be used to escape wildcard characters.
 248   * This is the string that can be used to escape '_' or '%' in the catalog search parameters that are a pattern
 249   * (and therefore use one of the wildcard characters).
 250   * The '_' character represents any single character; the '%' character represents any sequence of zero or more
 251   * characters.
 252   */
 253  SQL_SEARCH_STRING_ESCAPE = 513;
 254
 255  /*
 256   * Retrieves a UTF-8 string with all the "extra" characters that can be used in unquoted identifier names
 257   * (those beyond a-z, A-Z, 0-9 and _).
 258   */
 259  SQL_EXTRA_NAME_CHARACTERS = 514;
 260
 261  /*
 262   * Retrieves a boolean value indicating whether column aliasing is supported.
 263   * If so, the SQL AS clause can be used to provide names for computed columns or to provide alias names for columns
 264   * as required.
 265   *
 266   * Returns:
 267   * - false: if column aliasing is unsupported;
 268   * - true: if column aliasing is supported.
 269   */
 270  SQL_SUPPORTS_COLUMN_ALIASING = 515;
 271
 272  /*
 273   * Retrieves a boolean value indicating whether concatenations between null and non-null values being
 274   * null are supported.
 275   *
 276   * - Returns:
 277   * - false: if concatenations between null and non-null values being null are unsupported;
 278   * - true: if concatenations between null and non-null values being null are supported.
 279   */
 280  SQL_NULL_PLUS_NULL_IS_NULL = 516;
 281
 282  /*
 283   * Retrieves a map where the key is the type to convert from and the value is a list with the types to convert to,
 284   * indicating the supported conversions. Each key and each item on the list value is a value to a predefined type on
 285   * SqlSupportsConvert enum.
 286   * The returned map will be:  map<int32, list<int32>>
 287   */
 288  SQL_SUPPORTS_CONVERT = 517;
 289
 290  /*
 291   * Retrieves a boolean value indicating whether, when table correlation names are supported,
 292   * they are restricted to being different from the names of the tables.
 293   *
 294   * Returns:
 295   * - false: if table correlation names are unsupported;
 296   * - true: if table correlation names are supported.
 297   */
 298  SQL_SUPPORTS_TABLE_CORRELATION_NAMES = 518;
 299
 300  /*
 301   * Retrieves a boolean value indicating whether, when table correlation names are supported,
 302   * they are restricted to being different from the names of the tables.
 303   *
 304   * Returns:
 305   * - false: if different table correlation names are unsupported;
 306   * - true: if different table correlation names are supported
 307   */
 308  SQL_SUPPORTS_DIFFERENT_TABLE_CORRELATION_NAMES = 519;
 309
 310  /*
 311   * Retrieves a boolean value indicating whether expressions in ORDER BY lists are supported.
 312   *
 313   * Returns:
 314   * - false: if expressions in ORDER BY are unsupported;
 315   * - true: if expressions in ORDER BY are supported;
 316   */
 317  SQL_SUPPORTS_EXPRESSIONS_IN_ORDER_BY = 520;
 318
 319  /*
 320   * Retrieves a boolean value indicating whether using a column that is not in the SELECT statement in a GROUP BY
 321   * clause is supported.
 322   *
 323   * Returns:
 324   * - false: if using a column that is not in the SELECT statement in a GROUP BY clause is unsupported;
 325   * - true: if using a column that is not in the SELECT statement in a GROUP BY clause is supported.
 326   */
 327  SQL_SUPPORTS_ORDER_BY_UNRELATED = 521;
 328
 329  /*
 330   * Retrieves the supported GROUP BY commands;
 331   *
 332   * Returns an int32 bitmask value representing the supported commands.
 333   * The returned bitmask should be parsed in order to retrieve the supported commands.
 334   *
 335   * For instance:
 336   * - return 0 (\b0)   => [] (GROUP BY is unsupported);
 337   * - return 1 (\b1)   => [SQL_GROUP_BY_UNRELATED];
 338   * - return 2 (\b10)  => [SQL_GROUP_BY_BEYOND_SELECT];
 339   * - return 3 (\b11)  => [SQL_GROUP_BY_UNRELATED, SQL_GROUP_BY_BEYOND_SELECT].
 340   * Valid GROUP BY types are described under `arrow.flight.protocol.sql.SqlSupportedGroupBy`.
 341   */
 342  SQL_SUPPORTED_GROUP_BY = 522;
 343
 344  /*
 345   * Retrieves a boolean value indicating whether specifying a LIKE escape clause is supported.
 346   *
 347   * Returns:
 348   * - false: if specifying a LIKE escape clause is unsupported;
 349   * - true: if specifying a LIKE escape clause is supported.
 350   */
 351  SQL_SUPPORTS_LIKE_ESCAPE_CLAUSE = 523;
 352
 353  /*
 354   * Retrieves a boolean value indicating whether columns may be defined as non-nullable.
 355   *
 356   * Returns:
 357   * - false: if columns cannot be defined as non-nullable;
 358   * - true: if columns may be defined as non-nullable.
 359   */
 360  SQL_SUPPORTS_NON_NULLABLE_COLUMNS = 524;
 361
 362  /*
 363   * Retrieves the supported SQL grammar level as per the ODBC specification.
 364   *
 365   * Returns an int32 bitmask value representing the supported SQL grammar level.
 366   * The returned bitmask should be parsed in order to retrieve the supported grammar levels.
 367   *
 368   * For instance:
 369   * - return 0 (\b0)   => [] (SQL grammar is unsupported);
 370   * - return 1 (\b1)   => [SQL_MINIMUM_GRAMMAR];
 371   * - return 2 (\b10)  => [SQL_CORE_GRAMMAR];
 372   * - return 3 (\b11)  => [SQL_MINIMUM_GRAMMAR, SQL_CORE_GRAMMAR];
 373   * - return 4 (\b100) => [SQL_EXTENDED_GRAMMAR];
 374   * - return 5 (\b101) => [SQL_MINIMUM_GRAMMAR, SQL_EXTENDED_GRAMMAR];
 375   * - return 6 (\b110) => [SQL_CORE_GRAMMAR, SQL_EXTENDED_GRAMMAR];
 376   * - return 7 (\b111) => [SQL_MINIMUM_GRAMMAR, SQL_CORE_GRAMMAR, SQL_EXTENDED_GRAMMAR].
 377   * Valid SQL grammar levels are described under `arrow.flight.protocol.sql.SupportedSqlGrammar`.
 378   */
 379  SQL_SUPPORTED_GRAMMAR = 525;
 380
 381  /*
 382   * Retrieves the supported ANSI92 SQL grammar level.
 383   *
 384   * Returns an int32 bitmask value representing the supported ANSI92 SQL grammar level.
 385   * The returned bitmask should be parsed in order to retrieve the supported commands.
 386   *
 387   * For instance:
 388   * - return 0 (\b0)   => [] (ANSI92 SQL grammar is unsupported);
 389   * - return 1 (\b1)   => [ANSI92_ENTRY_SQL];
 390   * - return 2 (\b10)  => [ANSI92_INTERMEDIATE_SQL];
 391   * - return 3 (\b11)  => [ANSI92_ENTRY_SQL, ANSI92_INTERMEDIATE_SQL];
 392   * - return 4 (\b100) => [ANSI92_FULL_SQL];
 393   * - return 5 (\b101) => [ANSI92_ENTRY_SQL, ANSI92_FULL_SQL];
 394   * - return 6 (\b110) => [ANSI92_INTERMEDIATE_SQL, ANSI92_FULL_SQL];
 395   * - return 7 (\b111) => [ANSI92_ENTRY_SQL, ANSI92_INTERMEDIATE_SQL, ANSI92_FULL_SQL].
 396   * Valid ANSI92 SQL grammar levels are described under `arrow.flight.protocol.sql.SupportedAnsi92SqlGrammarLevel`.
 397   */
 398  SQL_ANSI92_SUPPORTED_LEVEL = 526;
 399
 400  /*
 401   * Retrieves a boolean value indicating whether the SQL Integrity Enhancement Facility is supported.
 402   *
 403   * Returns:
 404   * - false: if the SQL Integrity Enhancement Facility is supported;
 405   * - true: if the SQL Integrity Enhancement Facility is supported.
 406   */
 407  SQL_SUPPORTS_INTEGRITY_ENHANCEMENT_FACILITY = 527;
 408
 409  /*
 410   * Retrieves the support level for SQL OUTER JOINs.
 411   *
 412   * Returns a int32 ordinal for the SQL ordering being used, as described in
 413   * `arrow.flight.protocol.sql.SqlOuterJoinsSupportLevel`.
 414   */
 415  SQL_OUTER_JOINS_SUPPORT_LEVEL = 528;
 416
 417  // Retrieves a UTF-8 string with the preferred term for "schema".
 418  SQL_SCHEMA_TERM = 529;
 419
 420  // Retrieves a UTF-8 string with the preferred term for "procedure".
 421  SQL_PROCEDURE_TERM = 530;
 422
 423  /*
 424   * Retrieves a UTF-8 string with the preferred term for "catalog".
 425   * If a empty string is returned its assumed that the server does NOT supports catalogs.
 426   */
 427  SQL_CATALOG_TERM = 531;
 428
 429  /*
 430   * Retrieves a boolean value indicating whether a catalog appears at the start of a fully qualified table name.
 431   *
 432   * - false: if a catalog does not appear at the start of a fully qualified table name;
 433   * - true: if a catalog appears at the start of a fully qualified table name.
 434   */
 435  SQL_CATALOG_AT_START = 532;
 436
 437  /*
 438   * Retrieves the supported actions for a SQL schema.
 439   *
 440   * Returns an int32 bitmask value representing the supported actions for a SQL schema.
 441   * The returned bitmask should be parsed in order to retrieve the supported actions for a SQL schema.
 442   *
 443   * For instance:
 444   * - return 0 (\b0)   => [] (no supported actions for SQL schema);
 445   * - return 1 (\b1)   => [SQL_ELEMENT_IN_PROCEDURE_CALLS];
 446   * - return 2 (\b10)  => [SQL_ELEMENT_IN_INDEX_DEFINITIONS];
 447   * - return 3 (\b11)  => [SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_INDEX_DEFINITIONS];
 448   * - return 4 (\b100) => [SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS];
 449   * - return 5 (\b101) => [SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS];
 450   * - return 6 (\b110) => [SQL_ELEMENT_IN_INDEX_DEFINITIONS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS];
 451   * - return 7 (\b111) => [SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_INDEX_DEFINITIONS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS].
 452   * Valid actions for a SQL schema described under `arrow.flight.protocol.sql.SqlSupportedElementActions`.
 453   */
 454  SQL_SCHEMAS_SUPPORTED_ACTIONS = 533;
 455
 456  /*
 457   * Retrieves the supported actions for a SQL schema.
 458   *
 459   * Returns an int32 bitmask value representing the supported actions for a SQL catalog.
 460   * The returned bitmask should be parsed in order to retrieve the supported actions for a SQL catalog.
 461   *
 462   * For instance:
 463   * - return 0 (\b0)   => [] (no supported actions for SQL catalog);
 464   * - return 1 (\b1)   => [SQL_ELEMENT_IN_PROCEDURE_CALLS];
 465   * - return 2 (\b10)  => [SQL_ELEMENT_IN_INDEX_DEFINITIONS];
 466   * - return 3 (\b11)  => [SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_INDEX_DEFINITIONS];
 467   * - return 4 (\b100) => [SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS];
 468   * - return 5 (\b101) => [SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS];
 469   * - return 6 (\b110) => [SQL_ELEMENT_IN_INDEX_DEFINITIONS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS];
 470   * - return 7 (\b111) => [SQL_ELEMENT_IN_PROCEDURE_CALLS, SQL_ELEMENT_IN_INDEX_DEFINITIONS, SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS].
 471   * Valid actions for a SQL catalog are described under `arrow.flight.protocol.sql.SqlSupportedElementActions`.
 472   */
 473  SQL_CATALOGS_SUPPORTED_ACTIONS = 534;
 474
 475  /*
 476   * Retrieves the supported SQL positioned commands.
 477   *
 478   * Returns an int32 bitmask value representing the supported SQL positioned commands.
 479   * The returned bitmask should be parsed in order to retrieve the supported SQL positioned commands.
 480   *
 481   * For instance:
 482   * - return 0 (\b0)   => [] (no supported SQL positioned commands);
 483   * - return 1 (\b1)   => [SQL_POSITIONED_DELETE];
 484   * - return 2 (\b10)  => [SQL_POSITIONED_UPDATE];
 485   * - return 3 (\b11)  => [SQL_POSITIONED_DELETE, SQL_POSITIONED_UPDATE].
 486   * Valid SQL positioned commands are described under `arrow.flight.protocol.sql.SqlSupportedPositionedCommands`.
 487   */
 488  SQL_SUPPORTED_POSITIONED_COMMANDS = 535;
 489
 490  /*
 491   * Retrieves a boolean value indicating whether SELECT FOR UPDATE statements are supported.
 492   *
 493   * Returns:
 494   * - false: if SELECT FOR UPDATE statements are unsupported;
 495   * - true: if SELECT FOR UPDATE statements are supported.
 496   */
 497  SQL_SELECT_FOR_UPDATE_SUPPORTED = 536;
 498
 499  /*
 500   * Retrieves a boolean value indicating whether stored procedure calls that use the stored procedure escape syntax
 501   * are supported.
 502   *
 503   * Returns:
 504   * - false: if stored procedure calls that use the stored procedure escape syntax are unsupported;
 505   * - true: if stored procedure calls that use the stored procedure escape syntax are supported.
 506   */
 507  SQL_STORED_PROCEDURES_SUPPORTED = 537;
 508
 509  /*
 510   * Retrieves the supported SQL subqueries.
 511   *
 512   * Returns an int32 bitmask value representing the supported SQL subqueries.
 513   * The returned bitmask should be parsed in order to retrieve the supported SQL subqueries.
 514   *
 515   * For instance:
 516   * - return 0   (\b0)     => [] (no supported SQL subqueries);
 517   * - return 1   (\b1)     => [SQL_SUBQUERIES_IN_COMPARISONS];
 518   * - return 2   (\b10)    => [SQL_SUBQUERIES_IN_EXISTS];
 519   * - return 3   (\b11)    => [SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_EXISTS];
 520   * - return 4   (\b100)   => [SQL_SUBQUERIES_IN_INS];
 521   * - return 5   (\b101)   => [SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_INS];
 522   * - return 6   (\b110)   => [SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_EXISTS];
 523   * - return 7   (\b111)   => [SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_INS];
 524   * - return 8   (\b1000)  => [SQL_SUBQUERIES_IN_QUANTIFIEDS];
 525   * - return 9   (\b1001)  => [SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_QUANTIFIEDS];
 526   * - return 10  (\b1010)  => [SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_QUANTIFIEDS];
 527   * - return 11  (\b1011)  => [SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_QUANTIFIEDS];
 528   * - return 12  (\b1100)  => [SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_QUANTIFIEDS];
 529   * - return 13  (\b1101)  => [SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_QUANTIFIEDS];
 530   * - return 14  (\b1110)  => [SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_QUANTIFIEDS];
 531   * - return 15  (\b1111)  => [SQL_SUBQUERIES_IN_COMPARISONS, SQL_SUBQUERIES_IN_EXISTS, SQL_SUBQUERIES_IN_INS, SQL_SUBQUERIES_IN_QUANTIFIEDS];
 532   * - ...
 533   * Valid SQL subqueries are described under `arrow.flight.protocol.sql.SqlSupportedSubqueries`.
 534   */
 535  SQL_SUPPORTED_SUBQUERIES = 538;
 536
 537  /*
 538   * Retrieves a boolean value indicating whether correlated subqueries are supported.
 539   *
 540   * Returns:
 541   * - false: if correlated subqueries are unsupported;
 542   * - true: if correlated subqueries are supported.
 543   */
 544  SQL_CORRELATED_SUBQUERIES_SUPPORTED = 539;
 545
 546  /*
 547   * Retrieves the supported SQL UNIONs.
 548   *
 549   * Returns an int32 bitmask value representing the supported SQL UNIONs.
 550   * The returned bitmask should be parsed in order to retrieve the supported SQL UNIONs.
 551   *
 552   * For instance:
 553   * - return 0 (\b0)   => [] (no supported SQL positioned commands);
 554   * - return 1 (\b1)   => [SQL_UNION];
 555   * - return 2 (\b10)  => [SQL_UNION_ALL];
 556   * - return 3 (\b11)  => [SQL_UNION, SQL_UNION_ALL].
 557   * Valid SQL positioned commands are described under `arrow.flight.protocol.sql.SqlSupportedUnions`.
 558   */
 559  SQL_SUPPORTED_UNIONS = 540;
 560
 561  // Retrieves a int64 value representing the maximum number of hex characters allowed in an inline binary literal.
 562  SQL_MAX_BINARY_LITERAL_LENGTH = 541;
 563
 564  // Retrieves a int64 value representing the maximum number of characters allowed for a character literal.
 565  SQL_MAX_CHAR_LITERAL_LENGTH = 542;
 566
 567  // Retrieves a int64 value representing the maximum number of characters allowed for a column name.
 568  SQL_MAX_COLUMN_NAME_LENGTH = 543;
 569
 570  // Retrieves a int64 value representing the maximum number of columns allowed in a GROUP BY clause.
 571  SQL_MAX_COLUMNS_IN_GROUP_BY = 544;
 572
 573  // Retrieves a int64 value representing the maximum number of columns allowed in an index.
 574  SQL_MAX_COLUMNS_IN_INDEX = 545;
 575
 576  // Retrieves a int64 value representing the maximum number of columns allowed in an ORDER BY clause.
 577  SQL_MAX_COLUMNS_IN_ORDER_BY = 546;
 578
 579  // Retrieves a int64 value representing the maximum number of columns allowed in a SELECT list.
 580  SQL_MAX_COLUMNS_IN_SELECT = 547;
 581
 582  // Retrieves a int64 value representing the maximum number of columns allowed in a table.
 583  SQL_MAX_COLUMNS_IN_TABLE = 548;
 584
 585  // Retrieves a int64 value representing the maximum number of concurrent connections possible.
 586  SQL_MAX_CONNECTIONS = 549;
 587
 588  // Retrieves a int64 value the maximum number of characters allowed in a cursor name.
 589  SQL_MAX_CURSOR_NAME_LENGTH = 550;
 590
 591  /*
 592   * Retrieves a int64 value representing the maximum number of bytes allowed for an index,
 593   * including all of the parts of the index.
 594   */
 595  SQL_MAX_INDEX_LENGTH = 551;
 596
 597  // Retrieves a int64 value representing the maximum number of characters allowed in a schema name.
 598  SQL_DB_SCHEMA_NAME_LENGTH = 552;
 599
 600  // Retrieves a int64 value representing the maximum number of characters allowed in a procedure name.
 601  SQL_MAX_PROCEDURE_NAME_LENGTH = 553;
 602
 603  // Retrieves a int64 value representing the maximum number of characters allowed in a catalog name.
 604  SQL_MAX_CATALOG_NAME_LENGTH = 554;
 605
 606  // Retrieves a int64 value representing the maximum number of bytes allowed in a single row.
 607  SQL_MAX_ROW_SIZE = 555;
 608
 609  /*
 610   * Retrieves a boolean indicating whether the return value for the JDBC method getMaxRowSize includes the SQL
 611   * data types LONGVARCHAR and LONGVARBINARY.
 612   *
 613   * Returns:
 614   * - false: if return value for the JDBC method getMaxRowSize does
 615   *          not include the SQL data types LONGVARCHAR and LONGVARBINARY;
 616   * - true: if return value for the JDBC method getMaxRowSize includes
 617   *         the SQL data types LONGVARCHAR and LONGVARBINARY.
 618   */
 619  SQL_MAX_ROW_SIZE_INCLUDES_BLOBS = 556;
 620
 621  /*
 622   * Retrieves a int64 value representing the maximum number of characters allowed for an SQL statement;
 623   * a result of 0 (zero) means that there is no limit or the limit is not known.
 624   */
 625  SQL_MAX_STATEMENT_LENGTH = 557;
 626
 627  // Retrieves a int64 value representing the maximum number of active statements that can be open at the same time.
 628  SQL_MAX_STATEMENTS = 558;
 629
 630  // Retrieves a int64 value representing the maximum number of characters allowed in a table name.
 631  SQL_MAX_TABLE_NAME_LENGTH = 559;
 632
 633  // Retrieves a int64 value representing the maximum number of tables allowed in a SELECT statement.
 634  SQL_MAX_TABLES_IN_SELECT = 560;
 635
 636  // Retrieves a int64 value representing the maximum number of characters allowed in a user name.
 637  SQL_MAX_USERNAME_LENGTH = 561;
 638
 639  /*
 640   * Retrieves this database's default transaction isolation level as described in
 641   * `arrow.flight.protocol.sql.SqlTransactionIsolationLevel`.
 642   *
 643   * Returns a int32 ordinal for the SQL transaction isolation level.
 644   */
 645  SQL_DEFAULT_TRANSACTION_ISOLATION = 562;
 646
 647  /*
 648   * Retrieves a boolean value indicating whether transactions are supported. If not, invoking the method commit is a
 649   * noop, and the isolation level is `arrow.flight.protocol.sql.SqlTransactionIsolationLevel.TRANSACTION_NONE`.
 650   *
 651   * Returns:
 652   * - false: if transactions are unsupported;
 653   * - true: if transactions are supported.
 654   */
 655  SQL_TRANSACTIONS_SUPPORTED = 563;
 656
 657  /*
 658   * Retrieves the supported transactions isolation levels.
 659   *
 660   * Returns an int32 bitmask value representing the supported transactions isolation levels.
 661   * The returned bitmask should be parsed in order to retrieve the supported transactions isolation levels.
 662   *
 663   * For instance:
 664   * - return 0   (\b0)     => [] (no supported SQL transactions isolation levels);
 665   * - return 1   (\b1)     => [SQL_TRANSACTION_NONE];
 666   * - return 2   (\b10)    => [SQL_TRANSACTION_READ_UNCOMMITTED];
 667   * - return 3   (\b11)    => [SQL_TRANSACTION_NONE, SQL_TRANSACTION_READ_UNCOMMITTED];
 668   * - return 4   (\b100)   => [SQL_TRANSACTION_REPEATABLE_READ];
 669   * - return 5   (\b101)   => [SQL_TRANSACTION_NONE, SQL_TRANSACTION_REPEATABLE_READ];
 670   * - return 6   (\b110)   => [SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ];
 671   * - return 7   (\b111)   => [SQL_TRANSACTION_NONE, SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ];
 672   * - return 8   (\b1000)  => [SQL_TRANSACTION_REPEATABLE_READ];
 673   * - return 9   (\b1001)  => [SQL_TRANSACTION_NONE, SQL_TRANSACTION_REPEATABLE_READ];
 674   * - return 10  (\b1010)  => [SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ];
 675   * - return 11  (\b1011)  => [SQL_TRANSACTION_NONE, SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ];
 676   * - return 12  (\b1100)  => [SQL_TRANSACTION_REPEATABLE_READ, SQL_TRANSACTION_REPEATABLE_READ];
 677   * - return 13  (\b1101)  => [SQL_TRANSACTION_NONE, SQL_TRANSACTION_REPEATABLE_READ, SQL_TRANSACTION_REPEATABLE_READ];
 678   * - return 14  (\b1110)  => [SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ, SQL_TRANSACTION_REPEATABLE_READ];
 679   * - return 15  (\b1111)  => [SQL_TRANSACTION_NONE, SQL_TRANSACTION_READ_UNCOMMITTED, SQL_TRANSACTION_REPEATABLE_READ, SQL_TRANSACTION_REPEATABLE_READ];
 680   * - return 16  (\b10000) => [SQL_TRANSACTION_SERIALIZABLE];
 681   * - ...
 682   * Valid SQL positioned commands are described under `arrow.flight.protocol.sql.SqlTransactionIsolationLevel`.
 683   */
 684  SQL_SUPPORTED_TRANSACTIONS_ISOLATION_LEVELS = 564;
 685
 686  /*
 687   * Retrieves a boolean value indicating whether a data definition statement within a transaction forces
 688   * the transaction to commit.
 689   *
 690   * Returns:
 691   * - false: if a data definition statement within a transaction does not force the transaction to commit;
 692   * - true: if a data definition statement within a transaction forces the transaction to commit.
 693   */
 694  SQL_DATA_DEFINITION_CAUSES_TRANSACTION_COMMIT = 565;
 695
 696  /*
 697   * Retrieves a boolean value indicating whether a data definition statement within a transaction is ignored.
 698   *
 699   * Returns:
 700   * - false: if a data definition statement within a transaction is taken into account;
 701   * - true: a data definition statement within a transaction is ignored.
 702   */
 703  SQL_DATA_DEFINITIONS_IN_TRANSACTIONS_IGNORED = 566;
 704
 705  /*
 706   * Retrieves an int32 bitmask value representing the supported result set types.
 707   * The returned bitmask should be parsed in order to retrieve the supported result set types.
 708   *
 709   * For instance:
 710   * - return 0   (\b0)     => [] (no supported result set types);
 711   * - return 1   (\b1)     => [SQL_RESULT_SET_TYPE_UNSPECIFIED];
 712   * - return 2   (\b10)    => [SQL_RESULT_SET_TYPE_FORWARD_ONLY];
 713   * - return 3   (\b11)    => [SQL_RESULT_SET_TYPE_UNSPECIFIED, SQL_RESULT_SET_TYPE_FORWARD_ONLY];
 714   * - return 4   (\b100)   => [SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE];
 715   * - return 5   (\b101)   => [SQL_RESULT_SET_TYPE_UNSPECIFIED, SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE];
 716   * - return 6   (\b110)   => [SQL_RESULT_SET_TYPE_FORWARD_ONLY, SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE];
 717   * - return 7   (\b111)   => [SQL_RESULT_SET_TYPE_UNSPECIFIED, SQL_RESULT_SET_TYPE_FORWARD_ONLY, SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE];
 718   * - return 8   (\b1000)  => [SQL_RESULT_SET_TYPE_SCROLL_SENSITIVE];
 719   * - ...
 720   * Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetType`.
 721   */
 722  SQL_SUPPORTED_RESULT_SET_TYPES = 567;
 723
 724  /*
 725   * Returns an int32 bitmask value concurrency types supported for
 726   * `arrow.flight.protocol.sql.SqlSupportedResultSetType.SQL_RESULT_SET_TYPE_UNSPECIFIED`.
 727   *
 728   * For instance:
 729   * - return 0 (\b0)   => [] (no supported concurrency types for this result set type)
 730   * - return 1 (\b1)   => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED]
 731   * - return 2 (\b10)  => [SQL_RESULT_SET_CONCURRENCY_READ_ONLY]
 732   * - return 3 (\b11)  => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY]
 733   * - return 4 (\b100) => [SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 734   * - return 5 (\b101) => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 735   * - return 6 (\b110)  => [SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 736   * - return 7 (\b111)  => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 737   * Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetConcurrency`.
 738   */
 739  SQL_SUPPORTED_CONCURRENCIES_FOR_RESULT_SET_UNSPECIFIED = 568;
 740
 741  /*
 742   * Returns an int32 bitmask value concurrency types supported for
 743   * `arrow.flight.protocol.sql.SqlSupportedResultSetType.SQL_RESULT_SET_TYPE_FORWARD_ONLY`.
 744   *
 745   * For instance:
 746   * - return 0 (\b0)   => [] (no supported concurrency types for this result set type)
 747   * - return 1 (\b1)   => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED]
 748   * - return 2 (\b10)  => [SQL_RESULT_SET_CONCURRENCY_READ_ONLY]
 749   * - return 3 (\b11)  => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY]
 750   * - return 4 (\b100) => [SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 751   * - return 5 (\b101) => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 752   * - return 6 (\b110)  => [SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 753   * - return 7 (\b111)  => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 754   * Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetConcurrency`.
 755   */
 756  SQL_SUPPORTED_CONCURRENCIES_FOR_RESULT_SET_FORWARD_ONLY = 569;
 757
 758  /*
 759   * Returns an int32 bitmask value concurrency types supported for
 760   * `arrow.flight.protocol.sql.SqlSupportedResultSetType.SQL_RESULT_SET_TYPE_SCROLL_SENSITIVE`.
 761   *
 762   * For instance:
 763   * - return 0 (\b0)   => [] (no supported concurrency types for this result set type)
 764   * - return 1 (\b1)   => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED]
 765   * - return 2 (\b10)  => [SQL_RESULT_SET_CONCURRENCY_READ_ONLY]
 766   * - return 3 (\b11)  => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY]
 767   * - return 4 (\b100) => [SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 768   * - return 5 (\b101) => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 769   * - return 6 (\b110)  => [SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 770   * - return 7 (\b111)  => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 771   * Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetConcurrency`.
 772   */
 773  SQL_SUPPORTED_CONCURRENCIES_FOR_RESULT_SET_SCROLL_SENSITIVE = 570;
 774
 775  /*
 776   * Returns an int32 bitmask value concurrency types supported for
 777   * `arrow.flight.protocol.sql.SqlSupportedResultSetType.SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE`.
 778   *
 779   * For instance:
 780   * - return 0 (\b0)   => [] (no supported concurrency types for this result set type)
 781   * - return 1 (\b1)   => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED]
 782   * - return 2 (\b10)  => [SQL_RESULT_SET_CONCURRENCY_READ_ONLY]
 783   * - return 3 (\b11)  => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY]
 784   * - return 4 (\b100) => [SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 785   * - return 5 (\b101) => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 786   * - return 6 (\b110)  => [SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 787   * - return 7 (\b111)  => [SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED, SQL_RESULT_SET_CONCURRENCY_READ_ONLY, SQL_RESULT_SET_CONCURRENCY_UPDATABLE]
 788   * Valid result set types are described under `arrow.flight.protocol.sql.SqlSupportedResultSetConcurrency`.
 789   */
 790  SQL_SUPPORTED_CONCURRENCIES_FOR_RESULT_SET_SCROLL_INSENSITIVE = 571;
 791
 792  /*
 793   * Retrieves a boolean value indicating whether this database supports batch updates.
 794   *
 795   * - false: if this database does not support batch updates;
 796   * - true: if this database supports batch updates.
 797   */
 798  SQL_BATCH_UPDATES_SUPPORTED = 572;
 799
 800  /*
 801   * Retrieves a boolean value indicating whether this database supports savepoints.
 802   *
 803   * Returns:
 804   * - false: if this database does not support savepoints;
 805   * - true: if this database supports savepoints.
 806   */
 807  SQL_SAVEPOINTS_SUPPORTED = 573;
 808
 809  /*
 810   * Retrieves a boolean value indicating whether named parameters are supported in callable statements.
 811   *
 812   * Returns:
 813   * - false: if named parameters in callable statements are unsupported;
 814   * - true: if named parameters in callable statements are supported.
 815   */
 816  SQL_NAMED_PARAMETERS_SUPPORTED = 574;
 817
 818  /*
 819   * Retrieves a boolean value indicating whether updates made to a LOB are made on a copy or directly to the LOB.
 820   *
 821   * Returns:
 822   * - false: if updates made to a LOB are made directly to the LOB;
 823   * - true: if updates made to a LOB are made on a copy.
 824   */
 825  SQL_LOCATORS_UPDATE_COPY = 575;
 826
 827  /*
 828   * Retrieves a boolean value indicating whether invoking user-defined or vendor functions
 829   * using the stored procedure escape syntax is supported.
 830   *
 831   * Returns:
 832   * - false: if invoking user-defined or vendor functions using the stored procedure escape syntax is unsupported;
 833   * - true: if invoking user-defined or vendor functions using the stored procedure escape syntax is supported.
 834   */
 835  SQL_STORED_FUNCTIONS_USING_CALL_SYNTAX_SUPPORTED = 576;
 836}
 837
 838// The level of support for Flight SQL transaction RPCs.
 839enum SqlSupportedTransaction {
 840  // Unknown/not indicated/no support
 841  SQL_SUPPORTED_TRANSACTION_NONE = 0;
 842  // Transactions, but not savepoints.
 843  // A savepoint is a mark within a transaction that can be individually
 844  // rolled back to. Not all databases support savepoints.
 845  SQL_SUPPORTED_TRANSACTION_TRANSACTION = 1;
 846  // Transactions and savepoints
 847  SQL_SUPPORTED_TRANSACTION_SAVEPOINT = 2;
 848}
 849
 850enum SqlSupportedCaseSensitivity {
 851  SQL_CASE_SENSITIVITY_UNKNOWN = 0;
 852  SQL_CASE_SENSITIVITY_CASE_INSENSITIVE = 1;
 853  SQL_CASE_SENSITIVITY_UPPERCASE = 2;
 854  SQL_CASE_SENSITIVITY_LOWERCASE = 3;
 855}
 856
 857enum SqlNullOrdering {
 858  SQL_NULLS_SORTED_HIGH = 0;
 859  SQL_NULLS_SORTED_LOW = 1;
 860  SQL_NULLS_SORTED_AT_START = 2;
 861  SQL_NULLS_SORTED_AT_END = 3;
 862}
 863
 864enum SupportedSqlGrammar {
 865  SQL_MINIMUM_GRAMMAR = 0;
 866  SQL_CORE_GRAMMAR = 1;
 867  SQL_EXTENDED_GRAMMAR = 2;
 868}
 869
 870enum SupportedAnsi92SqlGrammarLevel {
 871  ANSI92_ENTRY_SQL = 0;
 872  ANSI92_INTERMEDIATE_SQL = 1;
 873  ANSI92_FULL_SQL = 2;
 874}
 875
 876enum SqlOuterJoinsSupportLevel {
 877  SQL_JOINS_UNSUPPORTED = 0;
 878  SQL_LIMITED_OUTER_JOINS = 1;
 879  SQL_FULL_OUTER_JOINS = 2;
 880}
 881
 882enum SqlSupportedGroupBy {
 883  SQL_GROUP_BY_UNRELATED = 0;
 884  SQL_GROUP_BY_BEYOND_SELECT = 1;
 885}
 886
 887enum SqlSupportedElementActions {
 888  SQL_ELEMENT_IN_PROCEDURE_CALLS = 0;
 889  SQL_ELEMENT_IN_INDEX_DEFINITIONS = 1;
 890  SQL_ELEMENT_IN_PRIVILEGE_DEFINITIONS = 2;
 891}
 892
 893enum SqlSupportedPositionedCommands {
 894  SQL_POSITIONED_DELETE = 0;
 895  SQL_POSITIONED_UPDATE = 1;
 896}
 897
 898enum SqlSupportedSubqueries {
 899  SQL_SUBQUERIES_IN_COMPARISONS = 0;
 900  SQL_SUBQUERIES_IN_EXISTS = 1;
 901  SQL_SUBQUERIES_IN_INS = 2;
 902  SQL_SUBQUERIES_IN_QUANTIFIEDS = 3;
 903}
 904
 905enum SqlSupportedUnions {
 906  SQL_UNION = 0;
 907  SQL_UNION_ALL = 1;
 908}
 909
 910enum SqlTransactionIsolationLevel {
 911  SQL_TRANSACTION_NONE = 0;
 912  SQL_TRANSACTION_READ_UNCOMMITTED = 1;
 913  SQL_TRANSACTION_READ_COMMITTED = 2;
 914  SQL_TRANSACTION_REPEATABLE_READ = 3;
 915  SQL_TRANSACTION_SERIALIZABLE = 4;
 916}
 917
 918enum SqlSupportedTransactions {
 919  SQL_TRANSACTION_UNSPECIFIED = 0;
 920  SQL_DATA_DEFINITION_TRANSACTIONS = 1;
 921  SQL_DATA_MANIPULATION_TRANSACTIONS = 2;
 922}
 923
 924enum SqlSupportedResultSetType {
 925  SQL_RESULT_SET_TYPE_UNSPECIFIED = 0;
 926  SQL_RESULT_SET_TYPE_FORWARD_ONLY = 1;
 927  SQL_RESULT_SET_TYPE_SCROLL_INSENSITIVE = 2;
 928  SQL_RESULT_SET_TYPE_SCROLL_SENSITIVE = 3;
 929}
 930
 931enum SqlSupportedResultSetConcurrency {
 932  SQL_RESULT_SET_CONCURRENCY_UNSPECIFIED = 0;
 933  SQL_RESULT_SET_CONCURRENCY_READ_ONLY = 1;
 934  SQL_RESULT_SET_CONCURRENCY_UPDATABLE = 2;
 935}
 936
 937enum SqlSupportsConvert {
 938  SQL_CONVERT_BIGINT = 0;
 939  SQL_CONVERT_BINARY = 1;
 940  SQL_CONVERT_BIT = 2;
 941  SQL_CONVERT_CHAR = 3;
 942  SQL_CONVERT_DATE = 4;
 943  SQL_CONVERT_DECIMAL = 5;
 944  SQL_CONVERT_FLOAT = 6;
 945  SQL_CONVERT_INTEGER = 7;
 946  SQL_CONVERT_INTERVAL_DAY_TIME = 8;
 947  SQL_CONVERT_INTERVAL_YEAR_MONTH = 9;
 948  SQL_CONVERT_LONGVARBINARY = 10;
 949  SQL_CONVERT_LONGVARCHAR = 11;
 950  SQL_CONVERT_NUMERIC = 12;
 951  SQL_CONVERT_REAL = 13;
 952  SQL_CONVERT_SMALLINT = 14;
 953  SQL_CONVERT_TIME = 15;
 954  SQL_CONVERT_TIMESTAMP = 16;
 955  SQL_CONVERT_TINYINT = 17;
 956  SQL_CONVERT_VARBINARY = 18;
 957  SQL_CONVERT_VARCHAR = 19;
 958}
 959
 960/**
 961 * The JDBC/ODBC-defined type of any object.
 962 * All the values here are the same as in the JDBC and ODBC specs.
 963 */
 964enum XdbcDataType {
 965  XDBC_UNKNOWN_TYPE = 0;
 966  XDBC_CHAR = 1;
 967  XDBC_NUMERIC = 2;
 968  XDBC_DECIMAL = 3;
 969  XDBC_INTEGER = 4;
 970  XDBC_SMALLINT = 5;
 971  XDBC_FLOAT = 6;
 972  XDBC_REAL = 7;
 973  XDBC_DOUBLE = 8;
 974  XDBC_DATETIME = 9;
 975  XDBC_INTERVAL = 10;
 976  XDBC_VARCHAR = 12;
 977  XDBC_DATE = 91;
 978  XDBC_TIME = 92;
 979  XDBC_TIMESTAMP = 93;
 980  XDBC_LONGVARCHAR = -1;
 981  XDBC_BINARY = -2;
 982  XDBC_VARBINARY = -3;
 983  XDBC_LONGVARBINARY = -4;
 984  XDBC_BIGINT = -5;
 985  XDBC_TINYINT = -6;
 986  XDBC_BIT = -7;
 987  XDBC_WCHAR = -8;
 988  XDBC_WVARCHAR = -9;
 989}
 990
 991/**
 992 * Detailed subtype information for XDBC_TYPE_DATETIME and XDBC_TYPE_INTERVAL.
 993 */
 994enum XdbcDatetimeSubcode {
 995  option allow_alias = true;
 996  XDBC_SUBCODE_UNKNOWN = 0;
 997  XDBC_SUBCODE_YEAR = 1;
 998  XDBC_SUBCODE_DATE = 1;
 999  XDBC_SUBCODE_TIME = 2;
1000  XDBC_SUBCODE_MONTH = 2;
1001  XDBC_SUBCODE_TIMESTAMP = 3;
1002  XDBC_SUBCODE_DAY = 3;
1003  XDBC_SUBCODE_TIME_WITH_TIMEZONE = 4;
1004  XDBC_SUBCODE_HOUR = 4;
1005  XDBC_SUBCODE_TIMESTAMP_WITH_TIMEZONE = 5;
1006  XDBC_SUBCODE_MINUTE = 5;
1007  XDBC_SUBCODE_SECOND = 6;
1008  XDBC_SUBCODE_YEAR_TO_MONTH = 7;
1009  XDBC_SUBCODE_DAY_TO_HOUR = 8;
1010  XDBC_SUBCODE_DAY_TO_MINUTE = 9;
1011  XDBC_SUBCODE_DAY_TO_SECOND = 10;
1012  XDBC_SUBCODE_HOUR_TO_MINUTE = 11;
1013  XDBC_SUBCODE_HOUR_TO_SECOND = 12;
1014  XDBC_SUBCODE_MINUTE_TO_SECOND = 13;
1015  XDBC_SUBCODE_INTERVAL_YEAR = 101;
1016  XDBC_SUBCODE_INTERVAL_MONTH = 102;
1017  XDBC_SUBCODE_INTERVAL_DAY = 103;
1018  XDBC_SUBCODE_INTERVAL_HOUR = 104;
1019  XDBC_SUBCODE_INTERVAL_MINUTE = 105;
1020  XDBC_SUBCODE_INTERVAL_SECOND = 106;
1021  XDBC_SUBCODE_INTERVAL_YEAR_TO_MONTH = 107;
1022  XDBC_SUBCODE_INTERVAL_DAY_TO_HOUR = 108;
1023  XDBC_SUBCODE_INTERVAL_DAY_TO_MINUTE = 109;
1024  XDBC_SUBCODE_INTERVAL_DAY_TO_SECOND = 110;
1025  XDBC_SUBCODE_INTERVAL_HOUR_TO_MINUTE = 111;
1026  XDBC_SUBCODE_INTERVAL_HOUR_TO_SECOND = 112;
1027  XDBC_SUBCODE_INTERVAL_MINUTE_TO_SECOND = 113;
1028}
1029
1030enum Nullable {
1031  /**
1032   * Indicates that the fields does not allow the use of null values.
1033   */
1034  NULLABILITY_NO_NULLS = 0;
1035
1036  /**
1037   * Indicates that the fields allow the use of null values.
1038   */
1039  NULLABILITY_NULLABLE = 1;
1040
1041  /**
1042   * Indicates that nullability of the fields cannot be determined.
1043   */
1044  NULLABILITY_UNKNOWN = 2;
1045}
1046
1047enum Searchable {
1048  /**
1049   * Indicates that column cannot be used in a WHERE clause.
1050   */
1051  SEARCHABLE_NONE = 0;
1052
1053  /**
1054   * Indicates that the column can be used in a WHERE clause if it is using a
1055   * LIKE operator.
1056   */
1057  SEARCHABLE_CHAR = 1;
1058
1059  /**
1060   * Indicates that the column can be used In a WHERE clause with any
1061   * operator other than LIKE.
1062   *
1063   * - Allowed operators: comparison, quantified comparison, BETWEEN,
1064   *                      DISTINCT, IN, MATCH, and UNIQUE.
1065   */
1066  SEARCHABLE_BASIC = 2;
1067
1068  /**
1069   * Indicates that the column can be used in a WHERE clause using any operator.
1070   */
1071  SEARCHABLE_FULL = 3;
1072}
1073
1074/*
1075 * Represents a request to retrieve information about data type supported on a Flight SQL enabled backend.
1076 * Used in the command member of FlightDescriptor for the following RPC calls:
1077 *  - GetSchema: return the schema of the query.
1078 *  - GetFlightInfo: execute the catalog metadata request.
1079 *
1080 * The returned schema will be:
1081 * <
1082 *   type_name: utf8 not null (The name of the data type, for example: VARCHAR, INTEGER, etc),
1083 *   data_type: int32 not null (The SQL data type),
1084 *   column_size: int32 (The maximum size supported by that column.
1085 *                       In case of exact numeric types, this represents the maximum precision.
1086 *                       In case of string types, this represents the character length.
1087 *                       In case of datetime data types, this represents the length in characters of the string representation.
1088 *                       NULL is returned for data types where column size is not applicable.),
1089 *   literal_prefix: utf8 (Character or characters used to prefix a literal, NULL is returned for
1090 *                         data types where a literal prefix is not applicable.),
1091 *   literal_suffix: utf8 (Character or characters used to terminate a literal,
1092 *                         NULL is returned for data types where a literal suffix is not applicable.),
1093 *   create_params: list<utf8 not null>
1094 *                        (A list of keywords corresponding to which parameters can be used when creating
1095 *                         a column for that specific type.
1096 *                         NULL is returned if there are no parameters for the data type definition.),
1097 *   nullable: int32 not null (Shows if the data type accepts a NULL value. The possible values can be seen in the
1098 *                             Nullable enum.),
1099 *   case_sensitive: bool not null (Shows if a character data type is case-sensitive in collations and comparisons),
1100 *   searchable: int32 not null (Shows how the data type is used in a WHERE clause. The possible values can be seen in the
1101 *                               Searchable enum.),
1102 *   unsigned_attribute: bool (Shows if the data type is unsigned. NULL is returned if the attribute is
1103 *                             not applicable to the data type or the data type is not numeric.),
1104 *   fixed_prec_scale: bool not null (Shows if the data type has predefined fixed precision and scale.),
1105 *   auto_increment: bool (Shows if the data type is auto incremental. NULL is returned if the attribute
1106 *                         is not applicable to the data type or the data type is not numeric.),
1107 *   local_type_name: utf8 (Localized version of the data source-dependent name of the data type. NULL
1108 *                          is returned if a localized name is not supported by the data source),
1109 *   minimum_scale: int32 (The minimum scale of the data type on the data source.
1110 *                         If a data type has a fixed scale, the MINIMUM_SCALE and MAXIMUM_SCALE
1111 *                         columns both contain this value. NULL is returned if scale is not applicable.),
1112 *   maximum_scale: int32 (The maximum scale of the data type on the data source.
1113 *                         NULL is returned if scale is not applicable.),
1114 *   sql_data_type: int32 not null (The value of the SQL DATA TYPE which has the same values
1115 *                                  as data_type value. Except for interval and datetime, which
1116 *                                  uses generic values. More info about those types can be
1117 *                                  obtained through datetime_subcode. The possible values can be seen
1118 *                                  in the XdbcDataType enum.),
1119 *   datetime_subcode: int32 (Only used when the SQL DATA TYPE is interval or datetime. It contains
1120 *                            its sub types. For type different from interval and datetime, this value
1121 *                            is NULL. The possible values can be seen in the XdbcDatetimeSubcode enum.),
1122 *   num_prec_radix: int32 (If the data type is an approximate numeric type, this column contains
1123 *                          the value 2 to indicate that COLUMN_SIZE specifies a number of bits. For
1124 *                          exact numeric types, this column contains the value 10 to indicate that
1125 *                          column size specifies a number of decimal digits. Otherwise, this column is NULL.),
1126 *   interval_precision: int32 (If the data type is an interval data type, then this column contains the value
1127 *                              of the interval leading precision. Otherwise, this column is NULL. This fields
1128 *                              is only relevant to be used by ODBC).
1129 * >
1130 * The returned data should be ordered by data_type and then by type_name.
1131 */
1132message CommandGetXdbcTypeInfo {
1133
1134  /*
1135   * Specifies the data type to search for the info.
1136   */
1137  optional int32 data_type = 1;
1138}
1139
1140/*
1141 * Represents a request to retrieve the list of catalogs on a Flight SQL enabled backend.
1142 * The definition of a catalog depends on vendor/implementation. It is usually the database itself
1143 * Used in the command member of FlightDescriptor for the following RPC calls:
1144 *  - GetSchema: return the Arrow schema of the query.
1145 *  - GetFlightInfo: execute the catalog metadata request.
1146 *
1147 * The returned Arrow schema will be:
1148 * <
1149 *  catalog_name: utf8 not null
1150 * >
1151 * The returned data should be ordered by catalog_name.
1152 */
1153message CommandGetCatalogs {
1154}
1155
1156/*
1157 * Represents a request to retrieve the list of database schemas on a Flight SQL enabled backend.
1158 * The definition of a database schema depends on vendor/implementation. It is usually a collection of tables.
1159 * Used in the command member of FlightDescriptor for the following RPC calls:
1160 *  - GetSchema: return the Arrow schema of the query.
1161 *  - GetFlightInfo: execute the catalog metadata request.
1162 *
1163 * The returned Arrow schema will be:
1164 * <
1165 *  catalog_name: utf8,
1166 *  db_schema_name: utf8 not null
1167 * >
1168 * The returned data should be ordered by catalog_name, then db_schema_name.
1169 */
1170message CommandGetDbSchemas {
1171
1172  /*
1173   * Specifies the Catalog to search for the tables.
1174   * An empty string retrieves those without a catalog.
1175   * If omitted the catalog name should not be used to narrow the search.
1176   */
1177  optional string catalog = 1;
1178
1179  /*
1180   * Specifies a filter pattern for schemas to search for.
1181   * When no db_schema_filter_pattern is provided, the pattern will not be used to narrow the search.
1182   * In the pattern string, two special characters can be used to denote matching rules:
1183   *    - "%" means to match any substring with 0 or more characters.
1184   *    - "_" means to match any one character.
1185   */
1186  optional string db_schema_filter_pattern = 2;
1187}
1188
1189/*
1190 * Represents a request to retrieve the list of tables, and optionally their schemas, on a Flight SQL enabled backend.
1191 * Used in the command member of FlightDescriptor for the following RPC calls:
1192 *  - GetSchema: return the Arrow schema of the query.
1193 *  - GetFlightInfo: execute the catalog metadata request.
1194 *
1195 * The returned Arrow schema will be:
1196 * <
1197 *  catalog_name: utf8,
1198 *  db_schema_name: utf8,
1199 *  table_name: utf8 not null,
1200 *  table_type: utf8 not null,
1201 *  [optional] table_schema: bytes not null (schema of the table as described in Schema.fbs::Schema,
1202 *                                           it is serialized as an IPC message.)
1203 * >
1204 * Fields on table_schema may contain the following metadata:
1205 *  - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
1206 *  - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
1207 *  - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
1208 *  - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
1209 *  - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
1210 *  - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
1211 *  - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
1212 *  - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
1213 *  - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
1214 *  - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.
1215 *  - ARROW:FLIGHT:SQL:REMARKS           - A comment describing the column. This field has been added after all others, clients should be prepared to find it missing.
1216 * The returned data should be ordered by catalog_name, db_schema_name, table_name, then table_type, followed by table_schema if requested.
1217 */
1218message CommandGetTables {
1219
1220  /*
1221   * Specifies the Catalog to search for the tables.
1222   * An empty string retrieves those without a catalog.
1223   * If omitted the catalog name should not be used to narrow the search.
1224   */
1225  optional string catalog = 1;
1226
1227  /*
1228   * Specifies a filter pattern for schemas to search for.
1229   * When no db_schema_filter_pattern is provided, all schemas matching other filters are searched.
1230   * In the pattern string, two special characters can be used to denote matching rules:
1231   *    - "%" means to match any substring with 0 or more characters.
1232   *    - "_" means to match any one character.
1233   */
1234  optional string db_schema_filter_pattern = 2;
1235
1236  /*
1237   * Specifies a filter pattern for tables to search for.
1238   * When no table_name_filter_pattern is provided, all tables matching other filters are searched.
1239   * In the pattern string, two special characters can be used to denote matching rules:
1240   *    - "%" means to match any substring with 0 or more characters.
1241   *    - "_" means to match any one character.
1242   */
1243  optional string table_name_filter_pattern = 3;
1244
1245  /*
1246   * Specifies a filter of table types which must match.
1247   * The table types depend on vendor/implementation. It is usually used to separate tables from views or system tables.
1248   * TABLE, VIEW, and SYSTEM TABLE are commonly supported.
1249   */
1250  repeated string table_types = 4;
1251
1252  // Specifies if the Arrow schema should be returned for found tables.
1253  bool include_schema = 5;
1254}
1255
1256/*
1257 * Represents a request to retrieve the list of table types on a Flight SQL enabled backend.
1258 * The table types depend on vendor/implementation. It is usually used to separate tables from views or system tables.
1259 * TABLE, VIEW, and SYSTEM TABLE are commonly supported.
1260 * Used in the command member of FlightDescriptor for the following RPC calls:
1261 *  - GetSchema: return the Arrow schema of the query.
1262 *  - GetFlightInfo: execute the catalog metadata request.
1263 *
1264 * The returned Arrow schema will be:
1265 * <
1266 *  table_type: utf8 not null
1267 * >
1268 * The returned data should be ordered by table_type.
1269 */
1270message CommandGetTableTypes {
1271}
1272
1273/*
1274 * Represents a request to retrieve the primary keys of a table on a Flight SQL enabled backend.
1275 * Used in the command member of FlightDescriptor for the following RPC calls:
1276 *  - GetSchema: return the Arrow schema of the query.
1277 *  - GetFlightInfo: execute the catalog metadata request.
1278 *
1279 * The returned Arrow schema will be:
1280 * <
1281 *  catalog_name: utf8,
1282 *  db_schema_name: utf8,
1283 *  table_name: utf8 not null,
1284 *  column_name: utf8 not null,
1285 *  key_name: utf8,
1286 *  key_sequence: int32 not null
1287 * >
1288 * The returned data should be ordered by catalog_name, db_schema_name, table_name, key_name, then key_sequence.
1289 */
1290message CommandGetPrimaryKeys {
1291
1292  /*
1293   * Specifies the catalog to search for the table.
1294   * An empty string retrieves those without a catalog.
1295   * If omitted the catalog name should not be used to narrow the search.
1296   */
1297  optional string catalog = 1;
1298
1299  /*
1300   * Specifies the schema to search for the table.
1301   * An empty string retrieves those without a schema.
1302   * If omitted the schema name should not be used to narrow the search.
1303   */
1304  optional string db_schema = 2;
1305
1306  // Specifies the table to get the primary keys for.
1307  string table = 3;
1308}
1309
1310enum UpdateDeleteRules {
1311  CASCADE = 0;
1312  RESTRICT = 1;
1313  SET_NULL = 2;
1314  NO_ACTION = 3;
1315  SET_DEFAULT = 4;
1316}
1317
1318/*
1319 * Represents a request to retrieve a description of the foreign key columns that reference the given table's
1320 * primary key columns (the foreign keys exported by a table) of a table on a Flight SQL enabled backend.
1321 * Used in the command member of FlightDescriptor for the following RPC calls:
1322 *  - GetSchema: return the Arrow schema of the query.
1323 *  - GetFlightInfo: execute the catalog metadata request.
1324 *
1325 * The returned Arrow schema will be:
1326 * <
1327 *  pk_catalog_name: utf8,
1328 *  pk_db_schema_name: utf8,
1329 *  pk_table_name: utf8 not null,
1330 *  pk_column_name: utf8 not null,
1331 *  fk_catalog_name: utf8,
1332 *  fk_db_schema_name: utf8,
1333 *  fk_table_name: utf8 not null,
1334 *  fk_column_name: utf8 not null,
1335 *  key_sequence: int32 not null,
1336 *  fk_key_name: utf8,
1337 *  pk_key_name: utf8,
1338 *  update_rule: uint8 not null,
1339 *  delete_rule: uint8 not null
1340 * >
1341 * The returned data should be ordered by fk_catalog_name, fk_db_schema_name, fk_table_name, fk_key_name, then key_sequence.
1342 * update_rule and delete_rule returns a byte that is equivalent to actions declared on UpdateDeleteRules enum.
1343 */
1344message CommandGetExportedKeys {
1345
1346  /*
1347   * Specifies the catalog to search for the foreign key table.
1348   * An empty string retrieves those without a catalog.
1349   * If omitted the catalog name should not be used to narrow the search.
1350   */
1351  optional string catalog = 1;
1352
1353  /*
1354   * Specifies the schema to search for the foreign key table.
1355   * An empty string retrieves those without a schema.
1356   * If omitted the schema name should not be used to narrow the search.
1357   */
1358  optional string db_schema = 2;
1359
1360  // Specifies the foreign key table to get the foreign keys for.
1361  string table = 3;
1362}
1363
1364/*
1365 * Represents a request to retrieve the foreign keys of a table on a Flight SQL enabled backend.
1366 * Used in the command member of FlightDescriptor for the following RPC calls:
1367 *  - GetSchema: return the Arrow schema of the query.
1368 *  - GetFlightInfo: execute the catalog metadata request.
1369 *
1370 * The returned Arrow schema will be:
1371 * <
1372 *  pk_catalog_name: utf8,
1373 *  pk_db_schema_name: utf8,
1374 *  pk_table_name: utf8 not null,
1375 *  pk_column_name: utf8 not null,
1376 *  fk_catalog_name: utf8,
1377 *  fk_db_schema_name: utf8,
1378 *  fk_table_name: utf8 not null,
1379 *  fk_column_name: utf8 not null,
1380 *  key_sequence: int32 not null,
1381 *  fk_key_name: utf8,
1382 *  pk_key_name: utf8,
1383 *  update_rule: uint8 not null,
1384 *  delete_rule: uint8 not null
1385 * >
1386 * The returned data should be ordered by pk_catalog_name, pk_db_schema_name, pk_table_name, pk_key_name, then key_sequence.
1387 * update_rule and delete_rule returns a byte that is equivalent to actions:
1388 *    - 0 = CASCADE
1389 *    - 1 = RESTRICT
1390 *    - 2 = SET NULL
1391 *    - 3 = NO ACTION
1392 *    - 4 = SET DEFAULT
1393 */
1394message CommandGetImportedKeys {
1395
1396  /*
1397   * Specifies the catalog to search for the primary key table.
1398   * An empty string retrieves those without a catalog.
1399   * If omitted the catalog name should not be used to narrow the search.
1400   */
1401  optional string catalog = 1;
1402
1403  /*
1404   * Specifies the schema to search for the primary key table.
1405   * An empty string retrieves those without a schema.
1406   * If omitted the schema name should not be used to narrow the search.
1407   */
1408  optional string db_schema = 2;
1409
1410  // Specifies the primary key table to get the foreign keys for.
1411  string table = 3;
1412}
1413
1414/*
1415 * Represents a request to retrieve a description of the foreign key columns in the given foreign key table that
1416 * reference the primary key or the columns representing a unique constraint of the parent table (could be the same
1417 * or a different table) on a Flight SQL enabled backend.
1418 * Used in the command member of FlightDescriptor for the following RPC calls:
1419 *  - GetSchema: return the Arrow schema of the query.
1420 *  - GetFlightInfo: execute the catalog metadata request.
1421 *
1422 * The returned Arrow schema will be:
1423 * <
1424 *  pk_catalog_name: utf8,
1425 *  pk_db_schema_name: utf8,
1426 *  pk_table_name: utf8 not null,
1427 *  pk_column_name: utf8 not null,
1428 *  fk_catalog_name: utf8,
1429 *  fk_db_schema_name: utf8,
1430 *  fk_table_name: utf8 not null,
1431 *  fk_column_name: utf8 not null,
1432 *  key_sequence: int32 not null,
1433 *  fk_key_name: utf8,
1434 *  pk_key_name: utf8,
1435 *  update_rule: uint8 not null,
1436 *  delete_rule: uint8 not null
1437 * >
1438 * The returned data should be ordered by pk_catalog_name, pk_db_schema_name, pk_table_name, pk_key_name, then key_sequence.
1439 * update_rule and delete_rule returns a byte that is equivalent to actions:
1440 *    - 0 = CASCADE
1441 *    - 1 = RESTRICT
1442 *    - 2 = SET NULL
1443 *    - 3 = NO ACTION
1444 *    - 4 = SET DEFAULT
1445 */
1446message CommandGetCrossReference {
1447
1448  /**
1449   * The catalog name where the parent table is.
1450   * An empty string retrieves those without a catalog.
1451   * If omitted the catalog name should not be used to narrow the search.
1452   */
1453  optional string pk_catalog = 1;
1454
1455  /**
1456   * The Schema name where the parent table is.
1457   * An empty string retrieves those without a schema.
1458   * If omitted the schema name should not be used to narrow the search.
1459   */
1460  optional string pk_db_schema = 2;
1461
1462  /**
1463   * The parent table name. It cannot be null.
1464   */
1465  string pk_table = 3;
1466
1467  /**
1468   * The catalog name where the foreign table is.
1469   * An empty string retrieves those without a catalog.
1470   * If omitted the catalog name should not be used to narrow the search.
1471   */
1472  optional string fk_catalog = 4;
1473
1474  /**
1475   * The schema name where the foreign table is.
1476   * An empty string retrieves those without a schema.
1477   * If omitted the schema name should not be used to narrow the search.
1478   */
1479  optional string fk_db_schema = 5;
1480
1481  /**
1482   * The foreign table name. It cannot be null.
1483   */
1484  string fk_table = 6;
1485}
1486
1487// Query Execution Action Messages
1488
1489/*
1490 * Request message for the "CreatePreparedStatement" action on a Flight SQL enabled backend.
1491 */
1492message ActionCreatePreparedStatementRequest {
1493
1494  // The valid SQL string to create a prepared statement for.
1495  string query = 1;
1496  // Create/execute the prepared statement as part of this transaction (if
1497  // unset, executions of the prepared statement will be auto-committed).
1498  optional bytes transaction_id = 2;
1499}
1500
1501/*
1502 * An embedded message describing a Substrait plan to execute.
1503 */
1504message SubstraitPlan {
1505
1506  // The serialized substrait.Plan to create a prepared statement for.
1507  // XXX(ARROW-16902): this is bytes instead of an embedded message
1508  // because Protobuf does not really support one DLL using Protobuf
1509  // definitions from another DLL.
1510  bytes plan = 1;
1511  // The Substrait release, e.g. "0.12.0". This information is not
1512  // tracked in the plan itself, so this is the only way for consumers
1513  // to potentially know if they can handle the plan.
1514  string version = 2;
1515}
1516
1517/*
1518 * Request message for the "CreatePreparedSubstraitPlan" action on a Flight SQL enabled backend.
1519 */
1520message ActionCreatePreparedSubstraitPlanRequest {
1521
1522  // The serialized substrait.Plan to create a prepared statement for.
1523  SubstraitPlan plan = 1;
1524  // Create/execute the prepared statement as part of this transaction (if
1525  // unset, executions of the prepared statement will be auto-committed).
1526  optional bytes transaction_id = 2;
1527}
1528
1529/*
1530 * Wrap the result of a "CreatePreparedStatement" or "CreatePreparedSubstraitPlan" action.
1531 *
1532 * The resultant PreparedStatement can be closed either:
1533 * - Manually, through the "ClosePreparedStatement" action;
1534 * - Automatically, by a server timeout.
1535 *
1536 * The result should be wrapped in a google.protobuf.Any message.
1537 */
1538message ActionCreatePreparedStatementResult {
1539
1540  // Opaque handle for the prepared statement on the server.
1541  bytes prepared_statement_handle = 1;
1542
1543  // If a result set generating query was provided, dataset_schema contains the
1544  // schema of the result set.  It should be an IPC-encapsulated Schema, as described in Schema.fbs.
1545  // For some queries, the schema of the results may depend on the schema of the parameters.  The server
1546  // should provide its best guess as to the schema at this point.  Clients must not assume that this
1547  // schema, if provided, will be accurate.
1548  bytes dataset_schema = 2;
1549
1550  // If the query provided contained parameters, parameter_schema contains the
1551  // schema of the expected parameters.  It should be an IPC-encapsulated Schema, as described in Schema.fbs.
1552  bytes parameter_schema = 3;
1553}
1554
1555/*
1556 * Request message for the "ClosePreparedStatement" action on a Flight SQL enabled backend.
1557 * Closes server resources associated with the prepared statement handle.
1558 */
1559message ActionClosePreparedStatementRequest {
1560
1561  // Opaque handle for the prepared statement on the server.
1562  bytes prepared_statement_handle = 1;
1563}
1564
1565/*
1566 * Request message for the "BeginTransaction" action.
1567 * Begins a transaction.
1568 */
1569message ActionBeginTransactionRequest {
1570}
1571
1572/*
1573 * Request message for the "BeginSavepoint" action.
1574 * Creates a savepoint within a transaction.
1575 *
1576 * Only supported if FLIGHT_SQL_TRANSACTION is
1577 * FLIGHT_SQL_TRANSACTION_SUPPORT_SAVEPOINT.
1578 */
1579message ActionBeginSavepointRequest {
1580
1581  // The transaction to which a savepoint belongs.
1582  bytes transaction_id = 1;
1583  // Name for the savepoint.
1584  string name = 2;
1585}
1586
1587/*
1588 * The result of a "BeginTransaction" action.
1589 *
1590 * The transaction can be manipulated with the "EndTransaction" action, or
1591 * automatically via server timeout. If the transaction times out, then it is
1592 * automatically rolled back.
1593 *
1594 * The result should be wrapped in a google.protobuf.Any message.
1595 */
1596message ActionBeginTransactionResult {
1597
1598  // Opaque handle for the transaction on the server.
1599  bytes transaction_id = 1;
1600}
1601
1602/*
1603 * The result of a "BeginSavepoint" action.
1604 *
1605 * The transaction can be manipulated with the "EndSavepoint" action.
1606 * If the associated transaction is committed, rolled back, or times
1607 * out, then the savepoint is also invalidated.
1608 *
1609 * The result should be wrapped in a google.protobuf.Any message.
1610 */
1611message ActionBeginSavepointResult {
1612
1613  // Opaque handle for the savepoint on the server.
1614  bytes savepoint_id = 1;
1615}
1616
1617/*
1618 * Request message for the "EndTransaction" action.
1619 *
1620 * Commit (COMMIT) or rollback (ROLLBACK) the transaction.
1621 *
1622 * If the action completes successfully, the transaction handle is
1623 * invalidated, as are all associated savepoints.
1624 */
1625message ActionEndTransactionRequest {
1626
1627  enum EndTransaction {
1628    END_TRANSACTION_UNSPECIFIED = 0;
1629    // Commit the transaction.
1630    END_TRANSACTION_COMMIT = 1;
1631    // Roll back the transaction.
1632    END_TRANSACTION_ROLLBACK = 2;
1633  }
1634  // Opaque handle for the transaction on the server.
1635  bytes transaction_id = 1;
1636  // Whether to commit/rollback the given transaction.
1637  EndTransaction action = 2;
1638}
1639
1640/*
1641 * Request message for the "EndSavepoint" action.
1642 *
1643 * Release (RELEASE) the savepoint or rollback (ROLLBACK) to the
1644 * savepoint.
1645 *
1646 * Releasing a savepoint invalidates that savepoint.  Rolling back to
1647 * a savepoint does not invalidate the savepoint, but invalidates all
1648 * savepoints created after the current savepoint.
1649 */
1650message ActionEndSavepointRequest {
1651
1652  enum EndSavepoint {
1653    END_SAVEPOINT_UNSPECIFIED = 0;
1654    // Release the savepoint.
1655    END_SAVEPOINT_RELEASE = 1;
1656    // Roll back to a savepoint.
1657    END_SAVEPOINT_ROLLBACK = 2;
1658  }
1659  // Opaque handle for the savepoint on the server.
1660  bytes savepoint_id = 1;
1661  // Whether to rollback/release the given savepoint.
1662  EndSavepoint action = 2;
1663}
1664
1665// Query Execution Messages.
1666
1667/*
1668 * Represents a SQL query. Used in the command member of FlightDescriptor
1669 * for the following RPC calls:
1670 *  - GetSchema: return the Arrow schema of the query.
1671 *    Fields on this schema may contain the following metadata:
1672 *    - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
1673 *    - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
1674 *    - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
1675 *    - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
1676 *    - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
1677 *    - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
1678 *    - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
1679 *    - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
1680 *    - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
1681 *    - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.
1682 *    - ARROW:FLIGHT:SQL:REMARKS           - A comment describing the column. This field has been added after all others, clients should be prepared to find it missing.
1683 *  - GetFlightInfo: execute the query.
1684 */
1685message CommandStatementQuery {
1686
1687  // The SQL syntax.
1688  string query = 1;
1689  // Include the query as part of this transaction (if unset, the query is auto-committed).
1690  optional bytes transaction_id = 2;
1691}
1692
1693/*
1694 * Represents a Substrait plan. Used in the command member of FlightDescriptor
1695 * for the following RPC calls:
1696 *  - GetSchema: return the Arrow schema of the query.
1697 *    Fields on this schema may contain the following metadata:
1698 *    - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
1699 *    - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
1700 *    - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
1701 *    - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
1702 *    - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
1703 *    - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
1704 *    - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
1705 *    - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
1706 *    - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
1707 *    - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.
1708 *    - ARROW:FLIGHT:SQL:REMARKS           - A comment describing the column. This field has been added after all others, clients should be prepared to find it missing.
1709 *  - GetFlightInfo: execute the query.
1710 *  - DoPut: execute the query.
1711 */
1712message CommandStatementSubstraitPlan {
1713
1714  // A serialized substrait.Plan
1715  SubstraitPlan plan = 1;
1716  // Include the query as part of this transaction (if unset, the query is auto-committed).
1717  optional bytes transaction_id = 2;
1718}
1719
1720/**
1721 * Represents a ticket resulting from GetFlightInfo with a CommandStatementQuery.
1722 * This should be used only once and treated as an opaque value, that is, clients should not attempt to parse this.
1723 */
1724message TicketStatementQuery {
1725
1726  // Unique identifier for the instance of the statement to execute.
1727  bytes statement_handle = 1;
1728}
1729
1730/*
1731 * Represents an instance of executing a prepared statement. Used in the command member of FlightDescriptor for
1732 * the following RPC calls:
1733 *  - GetSchema: return the Arrow schema of the query.
1734 *    Fields on this schema may contain the following metadata:
1735 *    - ARROW:FLIGHT:SQL:CATALOG_NAME      - Table's catalog name
1736 *    - ARROW:FLIGHT:SQL:DB_SCHEMA_NAME    - Database schema name
1737 *    - ARROW:FLIGHT:SQL:TABLE_NAME        - Table name
1738 *    - ARROW:FLIGHT:SQL:TYPE_NAME         - The data source-specific name for the data type of the column.
1739 *    - ARROW:FLIGHT:SQL:PRECISION         - Column precision/size
1740 *    - ARROW:FLIGHT:SQL:SCALE             - Column scale/decimal digits if applicable
1741 *    - ARROW:FLIGHT:SQL:IS_AUTO_INCREMENT - "1" indicates if the column is auto incremented, "0" otherwise.
1742 *    - ARROW:FLIGHT:SQL:IS_CASE_SENSITIVE - "1" indicates if the column is case-sensitive, "0" otherwise.
1743 *    - ARROW:FLIGHT:SQL:IS_READ_ONLY      - "1" indicates if the column is read only, "0" otherwise.
1744 *    - ARROW:FLIGHT:SQL:IS_SEARCHABLE     - "1" indicates if the column is searchable via WHERE clause, "0" otherwise.
1745 *    - ARROW:FLIGHT:SQL:REMARKS           - A comment describing the column. This field has been added after all others, clients should be prepared to find it missing.
1746 *
1747 *    If the schema is retrieved after parameter values have been bound with DoPut, then the server should account
1748 *    for the parameters when determining the schema.
1749 *  - DoPut: bind parameter values. All of the bound parameter sets will be executed as a single atomic execution.
1750 *  - GetFlightInfo: execute the prepared statement instance.
1751 */
1752message CommandPreparedStatementQuery {
1753
1754  // Opaque handle for the prepared statement on the server.
1755  bytes prepared_statement_handle = 1;
1756}
1757
1758/*
1759 * Represents a SQL update query. Used in the command member of FlightDescriptor
1760 * for the RPC call DoPut to cause the server to execute the included SQL update.
1761 */
1762message CommandStatementUpdate {
1763
1764  // The SQL syntax.
1765  string query = 1;
1766  // Include the query as part of this transaction (if unset, the query is auto-committed).
1767  optional bytes transaction_id = 2;
1768}
1769
1770/*
1771 * Represents a SQL update query. Used in the command member of FlightDescriptor
1772 * for the RPC call DoPut to cause the server to execute the included
1773 * prepared statement handle as an update.
1774 */
1775message CommandPreparedStatementUpdate {
1776
1777  // Opaque handle for the prepared statement on the server.
1778  bytes prepared_statement_handle = 1;
1779}
1780
1781/*
1782 * Represents a bulk ingestion request. Used in the command member of FlightDescriptor
1783 * for the the RPC call DoPut to cause the server load the contents of the stream's
1784 * FlightData into the target destination.
1785 */
1786message CommandStatementIngest {
1787
1788  // Options for table definition behavior
1789  message TableDefinitionOptions {
1790    // The action to take if the target table does not exist
1791    enum TableNotExistOption {
1792      // Do not use. Servers should error if this is specified by a client.
1793      TABLE_NOT_EXIST_OPTION_UNSPECIFIED = 0;
1794      // Create the table if it does not exist
1795      TABLE_NOT_EXIST_OPTION_CREATE = 1;
1796      // Fail if the table does not exist
1797      TABLE_NOT_EXIST_OPTION_FAIL = 2;
1798    }
1799    // The action to take if the target table already exists
1800    enum TableExistsOption {
1801      // Do not use. Servers should error if this is specified by a client.
1802      TABLE_EXISTS_OPTION_UNSPECIFIED = 0;
1803      // Fail if the table already exists
1804      TABLE_EXISTS_OPTION_FAIL = 1;
1805      // Append to the table if it already exists
1806      TABLE_EXISTS_OPTION_APPEND = 2;
1807      // Drop and recreate the table if it already exists
1808      TABLE_EXISTS_OPTION_REPLACE = 3;
1809    }
1810
1811    TableNotExistOption if_not_exist = 1;
1812    TableExistsOption if_exists = 2;
1813  }
1814
1815  // The behavior for handling the table definition.
1816  TableDefinitionOptions table_definition_options = 1;
1817  // The table to load data into.
1818  string table = 2;
1819  // The db_schema of the destination table to load data into. If unset, a backend-specific default may be used.
1820  optional string schema = 3;
1821  // The catalog of the destination table to load data into. If unset, a backend-specific default may be used.
1822  optional string catalog = 4;
1823  /*
1824   * Store ingested data in a temporary table.
1825   * The effect of setting temporary is to place the table in a backend-defined namespace, and to drop the table at the end of the session.
1826   * The namespacing may make use of a backend-specific schema and/or catalog.
1827   * The server should return an error if an explicit choice of schema or catalog is incompatible with the server's namespacing decision.
1828  */
1829  bool temporary = 5;
1830  // Perform the ingestion as part of this transaction. If specified, results should not be committed in the event of an error/cancellation.
1831  optional bytes transaction_id = 6;
1832
1833  // Future extensions to the parameters of CommandStatementIngest should be added here, at a lower index than the generic 'options' parameter.
1834
1835  // Backend-specific options.
1836  map<string, string> options = 1000;
1837}
1838
1839/*
1840 * Returned from the RPC call DoPut when a CommandStatementUpdate,
1841 * CommandPreparedStatementUpdate, or CommandStatementIngest was
1842 * in the request, containing results from the update.
1843 */
1844message DoPutUpdateResult {
1845
1846  // The number of records updated. A return value of -1 represents
1847  // an unknown updated record count.
1848  int64 record_count = 1;
1849}
1850
1851/* An *optional* response returned when `DoPut` is called with `CommandPreparedStatementQuery`.
1852 *
1853 * *Note on legacy behavior*: previous versions of the protocol did not return any result for
1854 * this command, and that behavior should still be supported by clients. In that case, the client
1855 * can continue as though the fields in this message were not provided or set to sensible default values.
1856 */
1857message DoPutPreparedStatementResult {
1858
1859  // Represents a (potentially updated) opaque handle for the prepared statement on the server.
1860  // Because the handle could potentially be updated, any previous handles for this prepared
1861  // statement should be considered invalid, and all subsequent requests for this prepared
1862  // statement must use this new handle.
1863  // The updated handle allows implementing query parameters with stateless services.
1864  //
1865  // When an updated handle is not provided by the server, clients should continue
1866  // using the previous handle provided by `ActionCreatePreparedStatementResonse`.
1867  optional bytes prepared_statement_handle = 1;
1868}
1869
1870/*
1871 * Request message for the "CancelQuery" action.
1872 *
1873 * Explicitly cancel a running query.
1874 *
1875 * This lets a single client explicitly cancel work, no matter how many clients
1876 * are involved/whether the query is distributed or not, given server support.
1877 * The transaction/statement is not rolled back; it is the application's job to
1878 * commit or rollback as appropriate. This only indicates the client no longer
1879 * wishes to read the remainder of the query results or continue submitting
1880 * data.
1881 *
1882 * This command is idempotent.
1883 *
1884 * This command is deprecated since 13.0.0. Use the "CancelFlightInfo"
1885 * action with DoAction instead.
1886 */
1887message ActionCancelQueryRequest {
1888  option deprecated = true;
1889
1890  // The result of the GetFlightInfo RPC that initiated the query.
1891  // XXX(ARROW-16902): this must be a serialized FlightInfo, but is
1892  // rendered as bytes because Protobuf does not really support one
1893  // DLL using Protobuf definitions from another DLL.
1894  bytes info = 1;
1895}
1896
1897/*
1898 * The result of cancelling a query.
1899 *
1900 * The result should be wrapped in a google.protobuf.Any message.
1901 *
1902 * This command is deprecated since 13.0.0. Use the "CancelFlightInfo"
1903 * action with DoAction instead.
1904 */
1905message ActionCancelQueryResult {
1906  option deprecated = true;
1907
1908  enum CancelResult {
1909    // The cancellation status is unknown. Servers should avoid using
1910    // this value (send a NOT_FOUND error if the requested query is
1911    // not known). Clients can retry the request.
1912    CANCEL_RESULT_UNSPECIFIED = 0;
1913    // The cancellation request is complete. Subsequent requests with
1914    // the same payload may return CANCELLED or a NOT_FOUND error.
1915    CANCEL_RESULT_CANCELLED = 1;
1916    // The cancellation request is in progress. The client may retry
1917    // the cancellation request.
1918    CANCEL_RESULT_CANCELLING = 2;
1919    // The query is not cancellable. The client should not retry the
1920    // cancellation request.
1921    CANCEL_RESULT_NOT_CANCELLABLE = 3;
1922  }
1923
1924  CancelResult result = 1;
1925}
1926
1927extend google.protobuf.MessageOptions {
1928  bool experimental = 1000;
1929}

```
