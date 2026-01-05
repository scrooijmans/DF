Overview
Find out more in our github repository⁠.

To get started create a docker-compose.yml with the following:

version: '3'

# Before supabase/postgres 14.1.0
services:
  db:
    image: supabase/postgres
    ports:
      - "5432:5432"
    environment:
      POSTGRES_PASSWORD: postgres

# supabase/postgres 14.1.0 and beyond
services:
  db:
    image: supabase/postgres
    ports:
      - "5432:5432"
    command: postgres -c config_file=/etc/postgresql/postgresql.conf 
    environment:
      POSTGRES_PASSWORD: postgres
and then run: docker-compose up (add -d to run in detached mode). The database should now be available in port 5432.

As the image is based on the postgreSQL image, environment variables from the PostgreSQL image are applicable to this image.

Extensions
Extension	Version	Description
Postgres contrib modules⁠	-	Because everyone should enable pg_stat_statements.
PostGIS⁠	3.1.4⁠	Postgres' most popular extension - support for geographic objects.
pgRouting⁠	v3.3.0⁠	Extension of PostGIS - provides geospatial routing functionalities.
pgTAP⁠	v1.1.0⁠	Unit Testing for Postgres.
pg_cron⁠	v1.4.1⁠	Run CRON jobs inside Postgres.
pgAudit⁠	1.6.1⁠	Generate highly compliant audit logs.
pgjwt⁠	commit⁠	Generate JSON Web Tokens (JWT) in Postgres.
pgsql-http⁠	1.3.1⁠	HTTP client for Postgres.
plpgsql_check⁠	2.0.6⁠	Linter tool for PL/pgSQL.
pg-safeupdate⁠	1.4⁠	Protect your data from accidental updates or deletes.
wal2json⁠	2.4⁠	JSON output plugin for logical replication decoding.
PL/Java⁠	1.6.3⁠	Write in Java functions in Postgres.
plv8⁠	commit⁠	Write in Javascript functions in Postgres.
pg_plan_filter⁠	commit⁠	Only allow statements that fulfill set criteria to be executed.
pg_net⁠	v0.3⁠	Expose the SQL interface for async networking.
rum⁠	1.3.9⁠	An alternative to the GIN index.
pg_hashids⁠	commit⁠	Generate unique identifiers from numbers.
pg_sodium⁠	v1.3.0⁠	Modern encryption API using libsodium.
Can't find your favorite extension? Suggest for it to be added into future releases here⁠!