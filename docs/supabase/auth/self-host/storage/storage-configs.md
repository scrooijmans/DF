Title: Storage Self-hosting Config | Supabase Docs

Description: How to configure and deploy Supabase Storage.

Self-Hosting

Storage Self-hosting Config

=============================

A sample `.env` file is located in the storage repository.

Use this file to configure your environment variables for your Storage server.

## General

General Settings

##### Parameters

ANON_KEY

REQUIRED

no type

A long-lived JWT with anonymous Postgres privileges.

SERVICE_KEY

REQUIRED

no type

A long-lived JWT with Postgres privileges to bypass Row Level Security.

TENANT_ID

REQUIRED

no type

The ID of a Storage tenant.

REGION

REQUIRED

no type

Region of your S3 bucket.

GLOBAL_S3_BUCKET

REQUIRED

no type

Name of your S3 bucket.

POSTGREST_URL

REQUIRED

no type

The URL of your PostgREST server.

PGRST_JWT_SECRET

REQUIRED

no type

A JWT Secret for the PostgREST database.

DATABASE_URL

REQUIRED

no type

The URL of your Postgres database.

PGOPTIONS

REQUIRED

no type

Additional configuration parameters for Postgres startup.

FILE_SIZE_LIMIT

REQUIRED

no type

The maximum file size allowed.

STORAGE_BACKEND

REQUIRED

no type

The storage provider.

FILE_STORAGE_BACKEND_PATH

REQUIRED

no type

The location storage when the "STORAGE_BACKEND" is set to "file".

## Multi-tenant

Configuration items for multi-tenant servers.

##### Parameters

IS_MULTITENANT

REQUIRED

no type

Operate across multiple tenants.

MULTITENANT_DATABASE_URL

REQUIRED

no type

The URL of the multitenant Postgres database.

X_FORWARDED_HOST_REGEXP

REQUIRED

no type

TBD.

POSTGREST_URL_SUFFIX

REQUIRED

no type

The suffix for the PostgREST instance.

ADMIN_API_KEYS

REQUIRED

no type

Secure API key for administrative endpoints.

ENCRYPTION_KEY

REQUIRED

no type

An key for encryting/decrypting secrets.

### Is this helpful?

No Yes
