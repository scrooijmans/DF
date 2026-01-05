# DuckDB | Supabase Docs

[DuckDB](https://duckdb.org/) is an open-source column-oriented Relational Database Management System.

The DuckDB Wrapper allows you to read data from DuckDB within your Postgres database.

Before you can query DuckDB, you need to enable the Wrappers extension and store your credentials in Postgres.

### Enable Wrappers

Make sure the `wrappers` extension is installed on your database:

```

1
create extension if not exists wrappers with schema extensions;
```

### Enable the DuckDB Wrapper

Enable the `duckdb_wrapper` FDW:

```

1
2
3
create foreign data wrapper duckdb_wrapper  handler duckdb_fdw_handler  validator duckdb_fdw_validator;
```

### Store your credentials (optional)

By default, Postgres stores FDW credentials inside `pg_catalog.pg_foreign_server` in plain text. Anyone with access to this table will be able to view these credentials. Wrappers is designed to work with [Vault](https://supabase.com/docs/guides/database/vault), which provides an additional level of security for storing credentials. We recommend using Vault to store your credentials.

DuckDB can connect to many data sources, the credential to be saved in Vault depends on which data source you're going to use. For example, to store AWS credentials for S3 connection, you can run below SQL and note down the secret IDs returned:

```

1
2
3
4
5
6
7
8
9
10
11
12
-- Save your AWS credentials in Vault and retrieve the created-- `aws_access_key_id` and `aws_secret_access_key`select vault.create_secret(  '<access key id>',  -- secret to be encrypted  'aws_access_key_id',  -- secret name  'AWS access key for Wrappers'  -- secret description);select vault.create_secret(  '<secret access key>'  'aws_secret_access_key',  'AWS secret access key for Wrappers');
```

### Connecting to DuckDB

We need to provide Postgres with the credentials to connect to DuckDB. We can do this using the `create server` command. Depends on the data source, there are different server options needs to be specified. Below is the list of supported data sources and their corresponding server options.

For any server options need to be stored in Vault, you can add a prefix `vault_` to its name and use the secret ID returned from the `select vault.create_secret()` statement as the option value.

#### AWS S3

- Server Option: type
  - Description: Server type, must be s3
  - Required: Y
  - Default:
- Server Option: key_id
  - Description: The ID of the key to use
  - Required: Y
  - Default:
- Server Option: secret
  - Description: The secret of the key to use
  - Required: Y
  - Default:
- Server Option: region
  - Description: The region for which to authenticate
  - Required:
  - Default: us-east-1
- Server Option: endpoint
  - Description: Specify a custom S3 endpoint
  - Required:
  - Default: s3.amazonaws.com
- Server Option: session_token
  - Description: A session token passed to use as temporary credentials
  - Required:
  - Default:
- Server Option: url_compatibility_mode
  - Description: Can help when URLs contain problematic characters
  - Required:
  - Default: true
- Server Option: url_style
  - Description: Either vhost or path
  - Required:
  - Default: vhost
- Server Option: use_ssl
  - Description: Whether to use HTTPS or HTTP
  - Required:
  - Default: true
- Server Option: kms_key_id
  - Description: AWS KMS (Key Management Service) key for Server Side Encryption S3
  - Required:
  - Default:

A `create server` statement example:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 's3',    -- The key id saved in Vault    vault_key_id '<key_ID>',    -- The secret saved in Vault    vault_secret '<secret_key>',    -- AWS region    region 'us-east-1'  );
```

This `s3` server type can also be used for other S3-compatible storage services such like [Supabase Storage](https://supabase.com/docs/guides/storage). For example,

```

1
2
3
4
5
6
7
8
9
10
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 's3',    key_id '<key_ID>',    secret '<secret_key>',    region 'us-east-1',    url_style 'path',	endpoint 'bctmhusapdbcvpetbnev.supabase.co/storage/v1/s3'  );
```

#### AWS S3 Tables

| Server Option | Description                                             | Required | Default   |
| ------------- | ------------------------------------------------------- | -------- | --------- |
| type          | Server type, must be s3_tables                          | Y        |           |
| key_id        | The ID of the key to use                                | Y        |           |
| secret        | The secret of the key to use                            | Y        |           |
| s3_tables_arn | S3 Tables ARN (available in the AWS Management Console) | Y        |           |
| region        | The region for which to authenticate                    |          | us-east-1 |

A `create server` statement example:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 's3_tables',    -- The key id saved in Vault    vault_key_id '<key_ID>',    -- The secret saved in Vault    vault_secret '<secret_key>',    -- AWS region    region 'us-east-1',    -- S3 Tables ARN    s3_tables_arn 'arn:aws:s3tables:us-east-1:203212701384:bucket/my-bucket'  );
```

#### Cloudflare R2

This is to access [Cloudflare R2](https://developers.cloudflare.com/r2/) using the [S3 Compatibility API](https://developers.cloudflare.com/r2/api/s3/api/).

| Server Option | Description                  | Required | Default |
| ------------- | ---------------------------- | -------- | ------- |
| type          | Server type, must be r2      | Y        |         |
| key_id        | The ID of the key to use     | Y        |         |
| secret        | The secret of the key to use | Y        |         |
| account_id    | The account ID to use        | Y        |         |

A `create server` statement example:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 'r2',    -- The key id saved in Vault    vault_key_id '<key_ID>',    -- The secret saved in Vault    vault_secret '<secret_key>',    -- Account ID    account_id '<account_ID>'  );
```

#### Cloudflare R2 Data Catalog

This is to access [Cloudflare R2 Data Catalog](https://developers.cloudflare.com/r2/data-catalog/).

| Server Option | Description                       | Required | Default |
| ------------- | --------------------------------- | -------- | ------- |
| type          | Server type, must be r2_catalog   | Y        |         |
| token         | The R2 API token to use           | Y        |         |
| warehouse     | Warehouse name in R2 Data Catalog | Y        |         |
| catalog_uri   | R2 Data Catalog URI               | Y        |         |

A `create server` statement example:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 'r2_catalog',    -- The R2 API token saved in Vault    vault_token '<token>',    -- Warehouse name    warehouse 'my_warehouse',    -- R2 Data Catalog URI    catalog_uri 'https://catalog.cloudflarestorage.com/1a4d06e707l56a1a724719292be42e3a/r2-data-catalog'  );
```

#### Apache Polaris

This is to access [Apache Polaris](https://polaris.apache.org/) Iceberg service.

| Server Option | Description                  | Required | Default |
| ------------- | ---------------------------- | -------- | ------- |
| type          | Server type, must be polaris | Y        |         |
| client_id     | The client ID to use         | Y        |         |
| client_secret | The client secret to use     | Y        |         |
| warehouse     | Warehouse name               | Y        |         |
| catalog_uri   | Polaris REST Catalog URI     | Y        |         |

A `create server` statement example:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 'polaris',    -- The client id saved in Vault    vault_client_id '<client_id>',    -- The client secret in Vault    vault_client_secret '<secret>',    -- Warehouse name    warehouse 'quickstart_catalog',    -- Polaris REST Catalog URI    catalog_uri '<polaris_rest_catalog_endpoint>'  );
```

#### Lakekeeper

This is to access [Lakekeeper](https://docs.lakekeeper.io/) Iceberg service.

| Server Option     | Description                          | Required | Default |
| ----------------- | ------------------------------------ | -------- | ------- |
| type              | Server type, must be lakekeeper      | Y        |         |
| client_id         | The client ID to use                 | Y        |         |
| client_secret     | The client secret to use             | Y        |         |
| oauth2_scope      | OAuth2 authentication scope          | Y        |         |
| oauth2_server_uri | Lakekeeper OAuth2 authentication URI | Y        |         |
| warehouse         | Warehouse name                       | Y        |         |
| catalog_uri       | Lakekeeper REST Catalog URI          | Y        |         |

A `create server` statement example:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 'lakekeeper',    -- The client id saved in Vault    vault_client_id '<client_id>',    -- The client secret in Vault    vault_client_secret '<secret>',    -- OAuth2 authentication settings    oauth2_scope 'lakekeeper',    oauth2_server_uri 'http://keycloak:8080/realms/iceberg/protocol/openid-connect/token'    -- Warehouse name    warehouse 'warehouse',    -- Lakekeeper REST Catalog URI    catalog_uri 'http://lakekeeper:8181/catalog'  );
```

#### Iceberg

This is to access generic Iceberg services. Check above for other specific Iceberg services like [S3 Tables](#aws-s3-tables), [R2 Data Catalog](#cloudflare-r2-data-catalog) and etc. All the [S3](#aws-s3) options are supported with below additional options.

| Server Option     | Description                  | Required | Default |
| ----------------- | ---------------------------- | -------- | ------- |
| type              | Server type, must be iceberg | Y        |         |
| warehouse         | Warehouse name               | Y        |         |
| catalog_uri       | REST Catalog URI             | Y        |         |
| token             | The API token to use         |          |         |
| client_id         | The client ID to use         |          |         |
| client_secret     | The client secret to use     |          |         |
| oauth2_scope      | OAuth2 authentication scope  |          |         |
| oauth2_server_uri | OAuth2 authentication URI    |          |         |

A `create server` statement example used to access local Iceberg service:

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 'iceberg',    -- The key id saved in Vault    vault_key_id '<key_ID>',    -- The secret saved in Vault    vault_secret '<secret_key>',    -- AWS region    region 'us-east-1',    -- S3 access settings    endpoint 'localhost:8000',    url_style 'path',    use_ssl 'false',    -- a dummy access token    token 'dummy',    -- Warehouse name    warehouse 'warehouse',    -- REST Catalog URI    catalog_uri 'localhost:8181'  );
```

### Create a schema

We recommend creating a schema to hold all the foreign tables:

```

1
create schema if not exists iceberg;
```

The full list of foreign table options are below:

- `table` - Fully qualified source table name in DuckDB, required.

This can also be a subquery enclosed in parentheses, for example,

```

1
table '(select * from my_table)'
```

or, an URI points to remote file or a function (with corresponding type of server),

```

1
table '''s3://my_bucket/products.parquet'''
```

```

1
table 'read_json(''s3://my_bucket/products.json'')'
```

We can use SQL [import foreign schema](https://www.postgresql.org/docs/current/sql-importforeignschema.html) to import foreign table definitions from DuckDB.

For example, using below SQL can automatically create foreign tables in the `duckdb` schema.

```

1
2
3
4
5
6
7
8
9
10
11
12
13
-- create all the foreign tables from Iceberg "docs_example" namespaceimport foreign schema "docs_example"  from server duckdb_server into duckdb;-- or, only create "readme" and "guides" foreign tablesimport foreign schema "docs_example"  limit to ("readme", "guides")  from server duckdb_server into duckdb;-- or, create all foreign tables except "readme"import foreign schema "docs_example"  except ("readme")  from server duckdb_server into duckdb;
```

Currently only Iceberg-like servers, such as S3 Tables, R2 Data Catalog and etc., support `import foreign schema` without specifying source tables. For other types of servers, source tables must be explicitly specified in options. For example,

```

1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
-- 'duckdb_server_s3_tables' server type is 's3_tables', so all tables-- under 'docs_example' namespace can be imported automaticallyimport foreign schema "docs_example"  from server duckdb_server_s3_tables into duckdb;-- 'duckdb_server_s3' server type is 's3', source tables to be imported-- must be specified explicitlyimport foreign schema s3  from server duckdb_server_s3 into duckdb  options (    tables '      s3://my_bucket/products.parquet,      s3://my_bucket/users.json	'  );
```

The imported table name format from Iceberg-like server is:

- `<server_type>_<schema_name>_<table_name>`

For example, the above statement will import a table name `s3_tables_docs_example_guides`.

For other types of server with explicitly specified sources tables, the imported foreign table names have the schema and sequence number as prefix with this format:

- `<schema_name>_<sequence_number>_<filename_stem>`

For example, by using belew statement,

```

1
2
3
4
5
6
7
8
import foreign schema s3  from server duckdb_server_s3 into duckdb  options (    tables '      s3://my_bucket/products.parquet,      s3://my_bucket/users.json	'  );
```

The imported foreign table names are:

- `s3_0_products`
- `s3_1_users`

### DuckDB Tables

This is an object representing DuckDB table.

Ref: [DuckDB Table](https://duckdb.org/docs/stable/sql/statements/create_table)

#### Operations

| Object | Select | Insert | Update | Delete | Truncate |
| ------ | ------ | ------ | ------ | ------ | -------- |
| table  | ✅     | ❌     | ❌     | ❌     | ❌       |

#### Usage

You can manually create the foreign table like below if you did not use `import foreign schema`.

```

1
2
3
4
5
6
7
8
9
10
create foreign table duckdb.products (  id bigint,  name text,  sku text,  created_at timestamp)  server duckdb_server  options (    table '''s3://my_bucket/products.parquet'''  );
```

This FDW supports `where`, `order by` and `limit` clause pushdown.

| Postgres Type    | DuckDB Type                              |
| ---------------- | ---------------------------------------- |
| boolean          | BOOLEAN, BOOL, LOGICAL                   |
| "char"           | TINYINT, INT1                            |
| smallint         | SMALLINT, INT2, SHORT                    |
| real             | FLOAT, FLOAT4, REAL                      |
| integer          | INTEGER, INT4, INT, SIGNED               |
| double precision | DOUBLE, FLOAT8                           |
| bigint           | BIGINT, INT8, LONG                       |
| numeric          | DECIMAL, NUMERIC                         |
| text             | BIT, VARCHAR, CHAR, BPCHAR, TEXT, STRING |
| date             | DATE                                     |
| time             | TIME                                     |
| timestamp        | TIMESTAMP, DATETIME                      |
| timestamptz      | TIMESTAMP WITH TIME ZONE, TIMESTAMPTZ    |
| jsonb            | JSON, ARRAY, LIST, MAP, STRUCT, UNION    |
| bytea            | BLOB, BYTEA, BINARY, VARBINARY           |
| uuid             | UUID                                     |

This section describes important limitations and considerations when using this FDW:

- Only supports certain server types, which data is stored remotely
- Only supports specific data type mappings between Postgres and DuckDB
- Only supports read operations (no INSERT, UPDATE, DELETE, or TRUNCATE)
- When using Iceberg REST Catalog, only supports AWS S3 (or compatible) as the storage
- Materialized views using these foreign tables may fail during logical backups

### Basic Example

First, create a `s3` server:

```

1
2
3
4
5
6
7
8
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 's3',    key_id '<AWS_access_key_ID>',    secret '<AWS_secret_access_key>',    region 'us-east-1'  );
```

Then import foreign table from a parquet file and query it:

```

1
2
3
4
5
6
7
8
9
import foreign schema s3  from server duckdb_server into duckdb  options (    tables '      s3://my_bucket/products.parquet    '  );select * from duckdb.s3_0_products;
```

This is the same as creating the foreign table manually like below,

```

1
2
3
4
5
6
7
8
9
10
11
12
create foreign table duckdb.products (  id bigint,  name text,  sku text,  created_at timestamp)  server duckdb_server  options (    table '''s3://my_bucket/products.parquet'''  );select * from duckdb.products;
```

### Read AWS S3 Tables

First, create a `s3_tables` server:

```

1
2
3
4
5
6
7
8
9
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 's3_tables',    key_id '<AWS_access_key_ID>',    secret '<AWS_secret_access_key>',    region 'us-east-1',    s3_tables_arn 'arn:aws:s3tables:us-east-1:203212701384:bucket/my-bucket'  );
```

Then, import all the tables in `docs_example` namespace and query it:

```

1
2
3
4
import foreign schema "docs_example"  from server duckdb_server into duckdb;select * from duckdb.s3_tables_docs_example_guides;
```

### Read Cloudflare R2 Data Catalog

First, follow the steps in [Getting Started Guide](https://developers.cloudflare.com/r2/data-catalog/get-started/) to create a R2 Catalog on Cloudflare. Once it is completed, create a `r2_catalog` server like below:

```

1
2
3
4
5
6
7
8
create server duckdb_server  foreign data wrapper duckdb_wrapper  options (    type 'r2_catalog',    token '<R2 API token>',    warehouse '2b303ef0293bc91a0217a0381af14a3e_r2-data-catalog-tutorial',    catalog_uri 'https://catalog.cloudflarestorage.com/2b303ef0293bc91a0217a0381af14a3e/r2-data-catalog-tutorial'  );
```

Then, import all the tables in `default` namespace and query it:

```

1
2
3
4
import foreign schema "default"  from server duckdb_server into duckdb;select * from duckdb.r2_catalog_default_people;
```

### Query Pushdown Examples

Follow the above [Read R2 Data Catalog example](#read-cloudflare-r2-data-catalog), below are some query pushdown examples:

```

1
2
3
4
5
6
7
8
9
-- the filter "name = 'Alice'" will be pushed down to DuckDBselect * from duckdb.r2_catalog_default_people where name = 'Alice';-- multiple filters must use logical 'AND'select * from duckdb.r2_catalog_default_peoplewhere name = 'Alice' and score = 80;-- 'order by' and 'limit' will be pushed down to DuckDBselect * from duckdb.r2_catalog_default_people order by id limit 2;
```
