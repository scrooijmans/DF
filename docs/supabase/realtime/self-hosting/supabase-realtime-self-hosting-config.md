# Realtime Self-hosting Config | Supabase Docs

---

You can use Environment Variables to configure your Realtime Server.

## General Settings

General server settings.

##### Parameters

PORT

REQUIRED

no type

Port which you can connect your client/listeners

REPLICATION_MODE

REQUIRED

no type

Connect to database via either IPv4 or IPv6. Disregarded if database host is an IP address (e.g. '127.0.0.1') and recommended if database host is a name (e.g. 'db.abcd.supabase.co') to prevent potential non-existent domain (NXDOMAIN) errors.

SLOT_NAME

REQUIRED

no type

A unique name for Postgres to track the Write-Ahead Logging (WAL). If the Realtime server dies then this slot can keep the changes since the last committed position.

TEMPORARY_SLOT

REQUIRED

no type

Start logical replication slot as either temporary or permanent

REALTIME_IP_VERSION

REQUIRED

no type

Bind realtime via either IPv4 or IPv6

PUBLICATIONS

REQUIRED

no type

JSON encoded array of publication names. Realtime RLS currently accepts one publication.

SECURE_CHANNELS

REQUIRED

no type

Enable/Disable channels authorization via JWT verification

JWT_SECRET

REQUIRED

no type

HS algorithm octet key (e.g. "95x0oR8jq9unl9pOIx")

JWT_CLAIM_VALIDATORS

REQUIRED

no type

Expected claim key/value pairs compared to JWT claims via equality checks in order to validate JWT. e.g. '{"iss": "Issuer", "nbf": 1610078130}'.

EXPOSE_METRICS

REQUIRED

no type

Expose Prometheus metrics at '/metrics' endpoint.

DB_RECONNECT_BACKOFF_MIN

REQUIRED

no type

Specify the minimum amount of time to wait before reconnecting to database

DB_RECONNECT_BACKOFF_MAX

REQUIRED

no type

Specify the maximum amount of time to wait before reconnecting to database

REPLICATION_POLL_INTERVAL

REQUIRED

no type

Specify how often Realtime RLS should poll the replication slot for changes

SUBSCRIPTION_SYNC_INTERVAL

REQUIRED

no type

Specify how often Realtime RLS should confirm connected subscribers and the tables they're listening to

MAX_CHANGES

REQUIRED

no type

Soft limit for the number of database changes to fetch per replication poll

MAX_RECORD_BYTES

REQUIRED

no type

Controls the maximum size of a WAL record

## Database Settings

Connecting to your database.

##### Parameters

DB_HOST

REQUIRED

no type

Database host URL

DB_NAME

REQUIRED

no type

Database name

DB_USER

REQUIRED

no type

Database user

DB_PASSWORD

REQUIRED

no type

Database password

DB_PORT

REQUIRED

no type

Database port

DB_SSL

REQUIRED

no type

Database SSL connection

DB_IP_VERSION

REQUIRED

no type

Connect to database via either IPv4 or IPv6. Disregarded if database host is an IP address (e.g. '127.0.0.1') and recommended if database host is a name (e.g. 'db.abcd.supabase.co') to prevent potential non-existent domain (NXDOMAIN) errors.
