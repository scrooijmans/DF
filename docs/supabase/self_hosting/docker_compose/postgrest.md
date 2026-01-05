Description: Configuration parameters can be provided via: Config File., Environment Variables, overriding values from the config file., In-Database Configuration, overriding values from both the config file and environment variables.. Using Configuration Reloading you can modify the parameters without restar...

Title: Configuration — PostgREST 13.0 documentation             

*   Configuration
*   View page source

Configuration
==============

Configuration parameters can be provided via:

*   Config File.

*   Environment Variables, overriding values from the config file.

*   In-Database Configuration, overriding values from both the config file and environment variables.

Using Configuration Reloading you can modify the parameters without restarting the server.

Minimum parameters
-------------------

The server is able to start without any config parameters, but it won’t be able to serve requests unless it has a role to serve anonymous requests with - or a secret to use for JWT authentication.

Config File
------------

There is no predefined location for the config file, you must specify the file path as the one and only argument to the server:

./postgrest /path/to/postgrest.conf

The configuration file must contain a set of key value pairs:

\# postgrest.conf

\# The standard connection URI format, documented at
\# https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING
db\-uri       \= "postgres://user:pass@host:5432/dbname"

\# The database role to use when no client authentication is provided.
\# Should differ from authenticator
db\-anon\-role \= "anon"

\# The secret to verify the JWT for authenticated requests with.
\# Needs to be 32 characters minimum.
jwt\-secret           \= "reallyreallyreallyreallyverysafe"
jwt\-secret\-is\-base64 \= false

\# Port the postgrest process is listening on for http requests
server\-port \= 3000

You can run `postgrest --example` to display all possible configuration parameters and how to use them in a configuration file.

Environment Variables
----------------------

Environment variables are capitalized, have a `PGRST_` prefix, and use underscores. For example: `PGRST_DB_URI` corresponds to `db-uri` and `PGRST_APP_SETTINGS_*` to `app.settings.*`.

libpq environment variables are also supported for constructing the connection string, see db-uri.

See the full list of environment variable names on List of parameters.

In-Database Configuration
--------------------------

You can also configure the server with database settings by using a pre-config function. For example, you can configure db-schemas and jwt-secret like this:

\# postgrest.conf

db\-pre\-config  \= "postgrest.pre\_config"

\# or env vars

PGRST\_DB\_PRE\_CONFIG \= "postgrest.pre\_config"

\-- create a dedicated schema, hidden from the API
create schema postgrest;
\-- grant usage on this schema to the authenticator
grant usage on schema postgrest to authenticator;

\-- the function can configure postgREST by using set\_config
create or replace function postgrest.pre\_config()
returns void as $$
select
set\_config('pgrst.db\_schemas', 'schema1, schema2', true)
, set\_config('pgrst.jwt\_secret', 'REALLYREALLYREALLYREALLYVERYSAFE', true);
$$ language sql;

Note that underscores(`_`) need to be used instead of dashes(`-`) for the in-database config parameters. See the full list of in-database names on List of parameters.

You can disable the in-database configuration by setting db-config to `false`.

Note

For backwards compatibility, you can do in-db config by modifying the authenticator role. This is no longer recommended as it requires SUPERUSER.

ALTER ROLE authenticator SET pgrst.db\_schemas \= "tenant1, tenant2, tenant3"
ALTER ROLE authenticator IN DATABASE <your\_database\_name\> SET pgrst.db\_schemas \= "tenant4, tenant5" \-- database-specific setting, overrides the previous setting

Configuration Reloading
------------------------

It’s possible to reload PostgREST’s configuration without restarting the server. You can do this via signal or via notification.

*   Any modification to the Config File will be applied during reload.

*   Any modification to the In-Database Configuration will be applied during reload.

*   Not all settings are reloadable, see the reloadable list on List of parameters.

*   It’s not possible to change Environment Variables for a running process, hence reloading a Docker container configuration will not work. In these cases, you can restart the process or use In-Database Configuration.

### Configuration Reload with signal

To reload the configuration via signal, send a SIGUSR2 signal to the server process.

killall \-SIGUSR2 postgrest

### Configuration Reload with NOTIFY

To reload the configuration from within the database, you can use the `NOTIFY` command. See Listener.

NOTIFY pgrst, 'reload config'

List of parameters
-------------------

### admin-server-host

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>server-host</cite> value</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_ADMIN_SERVER_HOST</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Specifies the host for the Admin Server. Defaults to server-host value.

### admin-server-port

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Int</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_ADMIN_SERVER_PORT</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Specifies the port for the Admin Server. Cannot be equal to server-port.

### app.settings.\*

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>&amp;</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_APP_SETTINGS_*</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Arbitrary settings that can be used to pass in secret keys directly as strings, or via OS environment variables. For instance: `app.settings.jwt_secret = "$(MYAPP_JWT_SECRET)"` will take `MYAPP_JWT_SECRET` from the environment and make it available to PostgreSQL functions as `current_setting('app.settings.jwt_secret')`.
> 
> When using the environment variable PGRST\_APP\_SETTINGS\_\* form, the remainder of the variable is used as the new name. Case is not important : `PGRST_APP_SETTINGS_MY_ENV_VARIABLE=some_value` can be accessed in postgres as `current_setting('app.settings.my_env_variable')`.
> 
> The `current_setting` function has an optional boolean second argument to avoid it from raising an error if the value was not defined. Default values to `app.settings` can then be given by combining this argument with `coalesce` and `nullif` : `coalesce(nullif(current_setting('app.settings.my_custom_variable', true), ''), 'default value')`. The use of `nullif` is necessary because if set in a transaction, the setting is sometimes not “rolled back” to `null`. See also this section for more information on this behaviour.

### db-aggregates-enabled

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>False</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_AGGREGATES_ENABLED</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_aggregates_enabled</p></td></tr></tbody></table>
> 
> When this is set to `true`, the use of Aggregate Functions is allowed.
> 
> It is recommended that this be set to `false` unless proper safeguards are in place to prevent potential performance problems from arising. For example, it is possible that a user may request the `max()` of an unindexed column in a table with millions of rows. At best, this would result in a slow query, and at worst, it could be abused to prevent other users from accessing your API (i.e. a form of denial-of-service attack.)
> 
> Proper safeguards could include:
> 
> *   Use of a statement timeout. See Impersonated Role Settings.
>     
> *   Use of the pg\_plan\_filter extension to block excessively expensive queries.
>     

### db-anon-role

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_ANON_ROLE</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_anon_role</p></td></tr></tbody></table>
> 
> The database role to use when executing commands on behalf of unauthenticated clients. For more information, see Overview of role system.
> 
> When unset anonymous access will be blocked.

### db-channel

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>pgrst</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_CHANNEL</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> The name of the notification channel that PostgREST uses for Schema Cache Reloading with NOTIFY and Configuration Reload with NOTIFY.

### db-channel-enabled

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>True</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_CHANNEL_ENABLED</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> When this is set to `true`, the notification channel specified in db-channel is enabled.
> 
> You should set this to `false` when using PostgresSQL behind an external connection pooler such as PgBouncer working in transaction pooling mode. See this section for more information.

### db-config

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>True</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_CONFIG</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> > Enables the in-database configuration.

### db-pre-config

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_PRE_CONFIG</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_pre_config</p></td></tr></tbody></table>
> 
> > Name of the function that does In-Database Configuration.

### db-extra-search-path

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>public</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_EXTRA_SEARCH_PATH</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_extra_search_path</p></td></tr></tbody></table>
> 
> Extra schemas to add to the search\_path of every request. These schemas tables, views and functions **don’t get API endpoints**, they can only be referred from the database objects inside your db-schemas.
> 
> This parameter was meant to make it easier to use **PostgreSQL extensions** (like PostGIS) that are outside of the db-schemas.
> 
> Multiple schemas can be added in a comma-separated string, e.g. `public, extensions`.

Important

We default this config to `public` because it is the most common schema used to install PostgreSQL extensions such as PostGIS. You can disable this by setting this config to `""`.

### db-hoisted-tx-settings

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>statement_timeout, plan_filter.statement_cost_limit, default_transaction_isolation</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_HOISTED_TX_SETTINGS</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_hoisted_tx_settings</p></td></tr></tbody></table>
> 
> Hoisted settings are allowed to be applied as transaction-scoped function settings. Multiple settings can be added in a comma-separated string, e.g. `work_mem, statement_timeout`.

### db-max-rows

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Int</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>∞</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_MAX_ROWS</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_max_rows</p></td></tr></tbody></table>
> 
> _For backwards compatibility, this config parameter is also available without prefix as “max-rows”._
> 
> A hard limit to the number of rows PostgREST will fetch from a view, table, or function. Limits payload size for accidental or malicious requests.

### db-plan-enabled

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>False</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_PLAN_ENABLED</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_plan_enabled</p></td></tr></tbody></table>
> 
> When this is set to `true`, the execution plan of a request can be retrieved by using the `Accept: application/vnd.pgrst.plan` header. See Execution plan.

### db-pool

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Int</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>10</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_POOL</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>n/a</p></td></tr></tbody></table>
> 
> Number of maximum connections to keep open in PostgREST’s database pool.

### db-pool-acquisition-timeout

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Int</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>10</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_POOL_ACQUISITION_TIMEOUT</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Specifies the maximum time in seconds that the request will wait for the pool to free up a connection slot to the database.

### db-pool-max-idletime

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Int</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>30</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_POOL_MAX_IDLETIME</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> > _For backwards compatibility, this config parameter is also available as “db-pool-timeout”._
> > 
> > Time in seconds to close idle pool connections.

### db-pool-max-lifetime

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Int</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>1800</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_POOL_MAX_LIFETIME</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Specifies the maximum time in seconds of an existing connection in the pool.

### db-pool-automatic-recovery

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>True</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_POOL_AUTOMATIC_RECOVERY</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Enables or disables connection retrying.
> 
> When disabled, PostgREST would terminate immediately after connection loss instead of retrying indefinitely. See this section for more information.

### db-pre-request

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_PRE_REQUEST</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_pre_request</p></td></tr></tbody></table>
> 
> _For backwards compatibility, this config parameter is also available without prefix as “pre-request”._
> 
> A schema-qualified function name to call right after the Transaction-Scoped Settings are set. See Pre-Request.

### db-prepared-statements

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>True</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_PREPARED_STATEMENTS</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_prepared_statements</p></td></tr></tbody></table>
> 
> Enables or disables prepared statements.
> 
> When disabled, the generated queries will be parameterized (invulnerable to SQL injection) but they will not be prepared (cached in the database session). Not using prepared statements will noticeably decrease performance, so it’s recommended to always have this setting enabled.
> 
> You should only set this to `false` when using PostgresSQL behind an external connection pooler such as PgBouncer working in transaction pooling mode. See this section for more information.

### db-root-spec

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_ROOT_SPEC</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_root_spec</p></td></tr></tbody></table>
> 
> Function to override the OpenAPI response. See Overriding Full OpenAPI Response.

### db-schemas

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>public</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_SCHEMAS</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_schemas</p></td></tr></tbody></table>
> 
> _For backwards compatibility, this config parameter is also available in singular as “db-schema”._
> 
> The list of database schemas to expose to clients. See Schemas.

### db-tx-end

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>commit</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_TX_END</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.db_tx_end</p></td></tr></tbody></table>
> 
> Specifies how to terminate the database transactions.
> 
> \# The transaction is always committed
> db-tx-end \= "commit"
> 
> \# The transaction is committed unless a "Prefer: tx=rollback" header is sent
> db-tx-end \= "commit-allow-override"
> 
> \# The transaction is always rolled back
> db-tx-end \= "rollback"
> 
> \# The transaction is rolled back unless a "Prefer: tx=commit" header is sent
> db-tx-end \= "rollback-allow-override"

### db-uri

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>postgresql://</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_DB_URI</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> The standard PostgreSQL connection string, there are different ways to specify it:

#### URI Format

> "postgres://authenticator:mysecretpassword@localhost:5433/postgres?parameters=val"
> 
> *   Under this format symbols and unusual characters in the password or other fields should be percent encoded to avoid a parse error.
>     
> *   If enforcing an SSL connection to the database is required you can use sslmode in the URI, for example `postgres://user:pass@host:5432/dbname?sslmode=require`.
>     
> *   The user with whom PostgREST connects to the database is also known as the `authenticator` role. For more information see Overview of role system.
>     
> *   When running PostgREST on the same machine as PostgreSQL, it is also possible to connect to the database using a Unix socket and the Peer Authentication method as an alternative to TCP/IP communication and authentication with a password, this also grants higher performance. To do this you can omit the host and the password, e.g. `postgres://user@/dbname`, see the libpq connection string documentation for more details.
>     

#### Keyword/Value Format

> "host=localhost port=5433 user=authenticator password=mysecretpassword dbname=postgres"

#### LIBPQ Environment Variables

> PGHOST\=localhost PGPORT\=5433 PGUSER\=authenticator PGDATABASE\=postgres
> 
> Any parameter that is not set in the above formats is read from libpq environment variables. The default connection string is `postgresql://`, which reads **all** parameters from the environment.

#### External config file

> Choosing a value for this parameter beginning with the at sign such as `@filename` (e.g. `@./configs/my-config`) loads the connection string out of an external file.

### jwt-aud

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_JWT_AUD</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.jwt_aud</p></td></tr></tbody></table>
> 
> > Specifies an audience for the JWT `aud` claim. See JWT aud Claim Validation.

### jwt-role-claim-key

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>.role</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_JWT_ROLE_CLAIM_KEY</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.jwt_role_claim_key</p></td></tr></tbody></table>
> 
> _For backwards compatibility, this config parameter is also available without prefix as “role-claim-key”._
> 
> See JWT Role Extraction on how to specify key paths and usage examples.

### jwt-secret

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_JWT_SECRET</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.jwt_secret</p></td></tr></tbody></table>
> 
> The secret or JSON Web Key (JWK) (or set) used to decode JWT tokens clients provide for authentication. For security the key must be **at least 32 characters long**. If this parameter is not specified then PostgREST refuses authentication requests. Choosing a value for this parameter beginning with the at sign such as `@filename` loads the secret out of an external file. This is useful for automating deployments. Note that any binary secrets must be base64 encoded. Both symmetric and asymmetric cryptography are supported. For more info see Asymmetric Keys.
> 
> Choosing a value for this parameter beginning with the at sign such as `@filename` (e.g. `@./configs/my-config`) loads the secret out of an external file.
> 
> Warning
> 
> Only when using the Config File, if the `jwt-secret` contains a `$` character by itself it will give errors. In this case, use `$$` and PostgREST will interpret it as a single `$` character.

### jwt-secret-is-base64

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>False</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_JWT_SECRET_IS_BASE64</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.jwt_secret_is_base64</p></td></tr></tbody></table>
> 
> When this is set to `true`, the value derived from `jwt-secret` will be treated as a base64 encoded secret.

### jwt-cache-max-lifetime

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Int</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>0</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_JWT_CACHE_MAX_LIFETIME</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.jwt_cache_max_lifetime</p></td></tr></tbody></table>
> 
> Maximum number of seconds of lifetime for cached entries. The default `0` disables caching. See JWT Caching.

### log-level

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>error</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_LOG_LEVEL</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Specifies the level of information to be logged while running PostgREST.
> 
> \# Only startup and db connection recovery messages are logged
> log-level \= "crit"
> 
> \# All the "crit" level events plus server errors (status 5xx) are logged
> log-level \= "error"
> 
> \# All the "error" level events plus request errors (status 4xx) are logged
> log-level \= "warn"
> 
> \# All the "warn" level events plus all requests (every status code) are logged
> log-level \= "info"
> 
> \# All the above plus events for development purposes are logged
> \# Logs connection pool events and the schema cache parsing time
> log-level \= "debug"
> 
> Because currently there’s no buffering for logging, the levels with minimal logging(`crit/error`) will increase throughput.

### log-query

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>“disabled”</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_LOG_QUERY</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Logs the SQL query for the corresponding request at the current log-level. See SQL Query Logs.
> 
> > \# Logs the main SQL query
> > log-query \= "main-query"
> > 
> > \# Disables logging the SQL query
> > log-query \= "disabled"

### openapi-mode

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>follow-privileges</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_OPENAPI_MODE</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.openapi_mode</p></td></tr></tbody></table>
> 
> Specifies how the OpenAPI output should be displayed.
> 
> \# Follows the privileges of the JWT role claim (or from db-anon-role if the JWT is not sent)
> \# Shows information depending on the permissions that the role making the request has
> openapi-mode \= "follow-privileges"
> 
> \# Ignores the privileges of the JWT role claim (or from db-anon-role if the JWT is not sent)
> \# Shows all the exposed information, regardless of the permissions that the role making the request has
> openapi-mode \= "ignore-privileges"
> 
> \# Disables the OpenApi output altogether.
> \# Throws a \`404 Not Found\` error when accessing the API root path
> openapi-mode \= "disabled"

### openapi-security-active

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>False</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_OPENAPI_SECURITY_ACTIVE</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.openapi_security_active</p></td></tr></tbody></table>

When this is set to `true`, security options are included in the OpenAPI output.

### openapi-server-proxy-uri

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_OPENAPI_SERVER_PROXY_URI</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.openapi_server_proxy_uri</p></td></tr></tbody></table>
> 
> Overrides the base URL used within the OpenAPI self-documentation hosted at the API root path. Use a complete URI syntax `scheme:[//[user:password@]host[:port]][/]path[?query][#fragment]`. Ex. `https://postgrest.com`
> 
> {
>   "swagger": "2.0",
>   "info": {
>     "version": "0.4.3.0",
>     "title": "PostgREST API",
>     "description": "This is a dynamic API generated by PostgREST"
>   },
>   "host": "postgrest.com:443",
>   "basePath": "/",
>   "schemes": \[
>     "https"
>   \]
> }

### server-cors-allowed-origins

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_SERVER_CORS_ALLOWED_ORIGINS</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>pgrst.server_cors_allowed_origins</cite></p></td></tr></tbody></table>
> 
> Specifies allowed CORS origins in this config. See CORS.
> 
> When this is not set or set to `""`, PostgREST **accepts** CORS requests from any domain.

### server-host

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>!4</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_SERVER_HOST</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Where to bind the PostgREST web server. In addition to the usual address options, PostgREST interprets these reserved addresses with special meanings:
> 
> *   `*` - any IPv4 or IPv6 hostname
>     
> *   `*4` - any IPv4 or IPv6 hostname, IPv4 preferred
>     
> *   `!4` - any IPv4 hostname
>     
> *   `*6` - any IPv4 or IPv6 hostname, IPv6 preferred
>     
> *   `!6` - any IPv6 hostname
>     
> 
> Examples:
> 
> server-host \= "127.0.0.1"

### server-port

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Int</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>3000</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_SERVER_PORT</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> The TCP port to bind the web server. Use `0` to automatically assign a port.

### server-trace-header

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_SERVER_TRACE_HEADER</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.server_trace_header</p></td></tr></tbody></table>
> 
> The header name used to trace HTTP requests. See Trace Header.

### server-timing-enabled

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>Boolean</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>False</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>Y</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_SERVER_TIMING_ENABLED</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p>pgrst.server_timing_enabled</p></td></tr></tbody></table>
> 
> Enables the Server-Timing header. See Server-Timing Header.

### server-unix-socket

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p><cite>n/a</cite></p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_SERVER_UNIX_SOCKET</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Unix domain socket where to bind the PostgREST web server. If specified, this takes precedence over server-port. Example:
> 
> server-unix-socket \= "/tmp/pgrst.sock"

### server-unix-socket-mode

> <table><tbody><tr><td><p><strong>Type</strong></p></td><td><p>String</p></td></tr><tr><td><p><strong>Default</strong></p></td><td><p>660</p></td></tr><tr><td><p><strong>Reloadable</strong></p></td><td><p>N</p></td></tr><tr><td><p><strong>Environment</strong></p></td><td><p>PGRST_SERVER_UNIX_SOCKET_MODE</p></td></tr><tr><td><p><strong>In-Database</strong></p></td><td><p><cite>n/a</cite></p></td></tr></tbody></table>
> 
> Unix file mode to be set for the socket specified in server-unix-socket Needs to be a valid octal between 600 and 777.
> 
> server-unix-socket-mode \= "660"