Title: Source Database Setup - PowerSync

Description: Configure your backend database for PowerSync, including permissions and replication settings.

PowerSync home page

Search...

⌘K

Search...

Navigation

Source Database Setup

Source Database Setup

Documentation

Client SDKs

Self-Hosting

Tutorials

Resources

*   GitHub
*   Community Discord
*   PowerSync Dashboard
*   PowerSync Website

##### Introduction

*   PowerSync Overview

*   PowerSync Philosophy

##### Installation

*   Quickstart / Overview

*   Source Database Setup

*   Overview

*   Security & IP Filtering

*   Private Endpoints

*   Database Connection

*   Client-Side Setup

*   App Backend Setup

*   Authentication Setup

##### Usage

*   Sync Rules

*   Lifecycle / Maintenance

*   Use Case Examples

*   Tools

##### Integration Guides

*   Integrations Overview

*   Supabase + PowerSync

*   FlutterFlow + PowerSync

*   Railway + PowerSync

*   Coolify + PowerSync

##### Migration Guides

*   MongoDB Atlas Device Sync Migration Guide

*   WatermelonDB Migration Guide

##### Architecture

*   Architecture Overview

*   PowerSync Service

*   Client Architecture

*   PowerSync Protocol

*   Consistency

On this page

*   Postgres
*   1\. Ensure logical replication is enabled
*   2\. Create a PowerSync database user
*   3\. Create “powersync” publication
*   Prerequisites
*   1\. Ensure logical replication is enabled
*   2\. Create a PowerSync database user
*   3\. Create “powersync” publication
*   Prerequisites
*   1\. Ensure logical replication is enabled
*   2\. Create a PowerSync database user
*   3\. Create “powersync” publication
*   1\. Ensure logical replication is enabled
*   2\. Create a PowerSync database user
*   3\. Create “powersync” publication
*   1\. Ensure logical replication is enabled
*   2\. Create a PowerSync database user
*   3\. Create “powersync” publication
*   1\. Ensure logical replication is enabled
*   2\. Create a PowerSync database user
*   3\. Create “powersync” publication
*   1\. Ensure logical replication is enabled
*   2\. Create a PowerSync database user
*   3\. Create “powersync” publication
*   1\. Ensure logical replication is enabled
*   2\. Create a PowerSync database user
*   3\. Create “powersync” publication
*   Unsupported Hosted Postgres Providers
*   MongoDB
*   Permissions required - MongoDB Atlas
*   Privileges required - Self-hosted / Custom roles
*   Post-Images
*   MongoDB Atlas private endpoints using AWS PrivateLink
*   Migrating from MongoDB Atlas Device Sync
*   MySQL (Beta)
*   Binlog Configuration
*   Database User Configuration
*   Additional Configuration (optional)
*   Binlog
*   Next Step

Jump to: Postgres | MongoDB | MySQL

​

Postgres
-------------

**Version compatibility**: PowerSync requires Postgres version 11 or greater.

Configuring your Postgres database for PowerSync generally involves three tasks:

1.  Ensure logical replication is enabled
2.  Create a PowerSync database user
3.  Create `powersync` logical replication publication

We have documented steps for some hosting providers:

Supabase

### 

​

1\. Ensure logical replication is enabled

No action required: Supabase has logical replication enabled by default.

### 

​

2\. Create a PowerSync database user

Copy

```
-- Create a role/user with replication privileges for PowerSync
CREATE ROLE powersync_role WITH REPLICATION BYPASSRLS LOGIN PASSWORD 'myhighlyrandompassword';
-- Set up permissions for the newly created role
-- Read-only (SELECT) access is required
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync_role;  
```

To restrict read access to specific tables, explicitly list allowed tables for both the `SELECT` privilege, and for the publication mentioned in the next step (as well as for any other publications that may exist).

### 

​

3\. Create “powersync” publication

Copy

```
-- Create a publication to replicate tables. 
-- Specify a subset of tables to replicate if required.
-- The publication must be named "powersync"
CREATE PUBLICATION powersync FOR ALL TABLES;
```
AWS RDS

### 

​

Prerequisites

The instance must be publicly accessible using an IPv4 address.

Access may be restricted to specific IPs if required — see IP Filtering.

### 

​

1\. Ensure logical replication is enabled

Set the `rds.logical_replication` parameter to `1` in the parameter group for the instance:

### 

​

2\. Create a PowerSync database user

Create a PowerSync user on Postgres:

Copy

```
-- SQL to create powersync user
CREATE ROLE powersync_role WITH BYPASSRLS LOGIN PASSWORD 'myhighlyrandompassword';

-- Allow the role to perform replication tasks
GRANT rds_replication TO powersync_role;

-- Set up permissions for the newly created role
-- Read-only (SELECT) access is required
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync_role;
```

To restrict read access to specific tables, explicitly list allowed tables for both the `SELECT` privilege, and for the publication (as well as for any other publications that may exist).

### 

​

3\. Create “powersync” publication

Copy

```
-- Create a publication to replicate tables. 
-- Specify a subset of tables to replicate if required.
-- The publication must be named "powersync"
CREATE PUBLICATION powersync FOR ALL TABLES;
```
Azure Postgres

PowerSync supports both “Azure Database for PostgreSQL” and “Azure Database for PostgreSQL Flexible Server”.

### 

​

Prerequisites

The database must be accessible on the public internet. Once you have created your database, navigate to **Settings** → **Networking** and enable **Public access.**

### 

​

1\. Ensure logical replication is enabled

Follow the steps as noted in this Microsoft article to allow logical replication.

### 

​

2\. Create a PowerSync database user

Copy

```
-- Create a role/user with replication privileges for PowerSync
CREATE ROLE powersync_role WITH REPLICATION BYPASSRLS LOGIN PASSWORD 'myhighlyrandompassword';
-- Set up permissions for the newly created role
-- Read-only (SELECT) access is required
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync_role;  
```

To restrict read access to specific tables, explicitly list allowed tables for both the `SELECT` privilege, and for the publication mentioned in the next step (as well as for any other publications that may exist).

### 

​

3\. Create “powersync” publication

Copy

```
-- Create a publication to replicate tables. 
-- Specify a subset of tables to replicate if required.
-- The publication must be named "powersync"
CREATE PUBLICATION powersync FOR ALL TABLES;
```
Google Cloud SQL

### 

​

1\. Ensure logical replication is enabled

In Google Cloud SQL Postgres, enabling the logical replication is done using flags:

### 

​

2\. Create a PowerSync database user

Copy

```
-- Create a role/user with replication privileges for PowerSync
CREATE ROLE powersync_role WITH REPLICATION BYPASSRLS LOGIN PASSWORD 'myhighlyrandompassword';
-- Set up permissions for the newly created role
-- Read-only (SELECT) access is required
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync_role;  
```

To restrict read access to specific tables, explicitly list allowed tables for both the `SELECT` privilege, and for the publication mentioned in the next step (as well as for any other publications that may exist).

### 

​

3\. Create “powersync” publication

Copy

```
-- Create a publication to replicate tables. 
-- Specify a subset of tables to replicate if required.
-- The publication must be named "powersync"
CREATE PUBLICATION powersync FOR ALL TABLES;
```
Neon

Neon is a serverless Postgres environment with an innovative pricing model that separates storage and compute.

### 

​

1\. Ensure logical replication is enabled

To Ensure logical replication is enabled:

1.  Select your project in the Neon Console.
2.  On the Neon Dashboard, select **Settings**.
3.  Select **Logical Replication**.
4.  Click **Enable** to Ensure logical replication is enabled.

### 

​

2\. Create a PowerSync database user

Copy

```
-- Create a role/user with replication privileges for PowerSync
CREATE ROLE powersync_role WITH REPLICATION BYPASSRLS LOGIN PASSWORD 'myhighlyrandompassword';
-- Set up permissions for the newly created role
-- Read-only (SELECT) access is required
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync_role;  
```

To restrict read access to specific tables, explicitly list allowed tables for both the `SELECT` privilege, and for the publication mentioned in the next step (as well as for any other publications that may exist).

### 

​

3\. Create “powersync” publication

Copy

```
-- Create a publication to replicate tables. 
-- Specify a subset of tables to replicate if required.
-- The publication must be named "powersync"
CREATE PUBLICATION powersync FOR ALL TABLES;
```
Fly Postgres

Fly Postgres is a Fly app with flyctl sugar on top to help you bootstrap and manage a database cluster for your apps.

### 

​

1\. Ensure logical replication is enabled

Once you’ve deployed your Fly Postgres cluster, you can use the following command to Ensure logical replication is enabled:

Copy

```
fly pg config update --wal-level=logical
```

### 

​

2\. Create a PowerSync database user

Copy

```
-- Create a role/user with replication privileges for PowerSync
CREATE ROLE powersync_role WITH REPLICATION BYPASSRLS LOGIN PASSWORD 'myhighlyrandompassword';
-- Set up permissions for the newly created role
-- Read-only (SELECT) access is required
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync_role;  
```

To restrict read access to specific tables, explicitly list allowed tables for both the `SELECT` privilege, and for the publication mentioned in the next step (as well as for any other publications that may exist).

### 

​

3\. Create “powersync” publication

Copy

```
-- Create a publication to replicate tables. 
-- Specify a subset of tables to replicate if required.
-- The publication must be named "powersync"
CREATE PUBLICATION powersync FOR ALL TABLES;
```
PlanetScale

### 

​

1\. Ensure logical replication is enabled

No action required: PlanetScale has logical replication (`wal_level = logical`) enabled by default.

### 

​

2\. Create a PowerSync database user

Copy

```
-- Create a role/user with replication privileges for PowerSync
CREATE ROLE powersync_role WITH REPLICATION BYPASSRLS LOGIN PASSWORD 'myhighlyrandompassword';
-- Set up permissions for the newly created role
-- Read-only (SELECT) access is required
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync_role;  
```

To restrict read access to specific tables, explicitly list allowed tables for both the `SELECT` privilege, and for the publication mentioned in the next step (as well as for any other publications that may exist).

### 

​

3\. Create “powersync” publication

Copy

```
-- Create a publication to replicate tables.
-- PlanetScale does not support ON ALL TABLES so
-- Specify each table you want to sync 
-- The publication must be named "powersync"
CREATE PUBLICATION powersync
FOR TABLE public.lists, public.todos; 
``` 

For other providers and self-hosted databases:Other / Self-hosted

Need help? Simply contact us on Discord and we’ll help you get set up.

### 

​

1\. Ensure logical replication is enabled

PowerSync reads the Postgres WAL using logical replication in order to create sync buckets in accordance with the specified PowerSync Sync Rules.If you are managing Postgres yourself, set `wal_level = logical` in your config file:

Alternatively, you can use the below SQL commands to check and Ensure logical replication is enabled:

Copy

```
-- Check the replication type

SHOW wal_level;

-- Ensure logical replication is enabled

ALTER SYSTEM SET wal_level = logical;
```

Note that Postgres must be restarted after changing this config.If you’re using a managed Postgres service, there may be a setting for this in the relevant section of the service’s admin console.

### 

​

2\. Create a PowerSync database user

Copy

```
-- Create a role/user with replication privileges for PowerSync
CREATE ROLE powersync_role WITH REPLICATION BYPASSRLS LOGIN PASSWORD 'myhighlyrandompassword';
-- Set up permissions for the newly created role
-- Read-only (SELECT) access is required
GRANT SELECT ON ALL TABLES IN SCHEMA public TO powersync_role;  
```

To restrict read access to specific tables, explicitly list allowed tables for both the `SELECT` privilege, and for the publication mentioned in the next step (as well as for any other publications that may exist).

### 

​

3\. Create “powersync” publication

Copy

```
-- Create a publication to replicate tables. 
-- Specify a subset of tables to replicate if required.
-- The publication must be named "powersync"
CREATE PUBLICATION powersync FOR ALL TABLES;
```

### 

​

Unsupported Hosted Postgres Providers

Due to the logical replication requirement, not all Postgres hosting providers are supported. Notably, some “serverless Postgres” providers do not support logical replication, and are therefore not supported by PowerSync yet.

​

MongoDB
------------

**Version compatibility**: PowerSync requires MongoDB version 6.0 or greater.

### 

​

Permissions required - MongoDB Atlas

For MongoDB Atlas databases, the minimum permissions when using built-in roles are:

Copy

```
readWrite@<your_database>._powersync_checkpoints
read@<your_database>
```

To allow PowerSync to automatically enable `changeStreamPreAndPostImages` on replicated collections (the default for new PowerSync instances), additionally add the `dbAdmin` permission:

Copy

```
readWrite@<your_database>._powersync_checkpoints
read@<your_database>
dbAdmin@<your_database>
```

If you are replicating from multiple databases in the cluster, you need read permissions on the entire cluster, in addition to the above:

Copy

```
readAnyDatabase@admin
```

### 

​

Privileges required - Self-hosted / Custom roles

For self-hosted MongoDB, or for creating custom roles on MongoDB Atlas, PowerSync requires the following privileges/granted actions:

*   On the database being replicated: `listCollections`
*   On all collections in the database: `changeStream`
*   This must apply to the entire database, not individual collections. Specify `collection: ""`
*   If replicating from multiple databases, this must apply to the entire cluster. Specify `db: ""`
*   On each collection being replicated: `find`
*   On the `_powersync_checkpoints` collection: `createCollection`, `dropCollection`, `find`, `changeStream`, `insert`, `update`, and `remove`
*   To allow PowerSync to automatically enable `changeStreamPreAndPostImages` on replicated collections, additionally add the `collMod` permission on all replicated collections.

### 

​

Post-Images

To replicate data from MongoDB to PowerSync in a consistent manner, PowerSync uses Change Streams with post-images to get the complete document after each change. This requires the `changeStreamPreAndPostImages` option to be enabled on replicated collections. PowerSync supports three configuration options for post-images:

1.  **Off**: (`post_images: off`): Uses `fullDocument: 'updateLookup'` for backwards compatibility. This was the default for older instances. However, this may lead to consistency issues, so we strongly recommend enabling post-images instead.
2.  **Automatic**: (`post_images: auto_configure`) The **default** for new instances: Automatically enables the `changeStreamPreAndPostImages` option on collections as needed. Requires the permissions/privileges mentioned above. If a collection is removed from Sync Rules, developers can manually disable `changeStreamPreAndPostImages`.
3.  **Read-only**: (`post_images: read_only`): Uses `fullDocument: 'required'` and requires `changeStreamPreAndPostImages: { enabled: true }` to be set on every collection referenced in the Sync Rules. Replication will error if this is not configured. This option is ideal when permissions are restricted.

To manually configure collections for `read_only` mode, run this on each collection:

Copy

```
db.runCommand( {
collMod: <collection>,
changeStreamPreAndPostImages: { enabled: <boolean> }
} )
```

You can view which collections have the option enabled using:

Copy

```
db.getCollectionInfos().filter(c => c.options?.changeStreamPreAndPostImages?.enabled)
```

Post-images can be configured for PowerSync instances as follows:

PowerSync Cloud:
----------------

Configure the **Post Images** setting in the connection configuration in the Dashboard (right-click on your instance to edit it).

Self-Hosted PowerSync:
----------------------

Configure `post_images` in the `config.yaml` file.

See an example

### 

​

MongoDB Atlas private endpoints using AWS PrivateLink

If you need to use private endpoints with MongoDB Atlas, see Private Endpoints (AWS only).

### 

​

Migrating from MongoDB Atlas Device Sync

For more information on migrating from Atlas Device Sync to PowerSync, see our migration guide.

​

MySQL (Beta)
-----------------

**Version compatibility**: PowerSync requires MySQL version 5.7 or greater.

PowerSync reads from the MySQL binary log to replicate changes. We use a modified version of the Zongji MySQL binlog listener to achieve this.

### 

​

Binlog Configuration

To ensure that PowerSync can read the binary log, you need to configure your MySQL server to enable binary logging and configure it with the following server command options:

*   server\_id: Uniquely identifies the MySQL server instance in the replication topology. Default value is **1**.
*   log\_bin: **ON**. Enables binary logging. Default is **ON** for MySQL 8.0 and later, but **OFF** for MySQL 5.7.
*   enforce\_gtid\_consistency: **ON**. Enforces GTID consistency. Default is **OFF**.
*   gtid\_mode: **ON**. Enables GTID based logging. Default is **OFF**.
*   binlog\_format: **ROW**. Sets the binary log format to row-based replication. This is required for PowerSync to correctly replicate changes. Default is **ROW**.

These can be specified in a MySQL option file:

Copy

```
server_id=<Unique Integer Value>
log_bin=ON
enforce_gtid_consistency=ON
gtid_mode=ON
binlog_format=ROW
```

### 

​

Database User Configuration

PowerSync also requires a MySQL user with **REPLICATION** and **SELECT** permission on the source databases. These can be added by running the following SQL commands:

Copy

```
-- Create a user with necessary privileges
CREATE USER 'repl_user'@'%' IDENTIFIED BY '<password>';

-- Grant replication client privilege
GRANT REPLICATION SLAVE, REPLICATION CLIENT ON *.* TO 'repl_user'@'%';

-- Grant select access to the specific database
GRANT SELECT ON <source_database>.* TO 'repl_user'@'%';

-- Apply changes
FLUSH PRIVILEGES;
```

It is possible to constrain the MySQL user further and limit access to specific tables. Care should be taken to ensure that all the tables in the sync rules are included in the grants.

Copy

```
-- Grant select to the users and the invoices tables in the source database
GRANT SELECT ON <source_database>.users TO 'repl_user'@'%';
GRANT SELECT ON <source_database>.invoices TO 'repl_user'@'%';

-- Apply changes
FLUSH PRIVILEGES;
```

### 

​

Additional Configuration (optional)

#### 

​

Binlog

The binlog can be configured to limit logging to specific databases. By default, events for tables in all the databases on the MySQL server will be logged.

*   binlog-do-db: Only updates for tables in the specified database will be logged.
*   binlog-ignore-db: No updates for tables in the specified database will be logged.

Examples:

Copy

```
# Only row events for tables in the user_db and invoices_db databases will appear in the binlog.
binlog-do-db=user_db
binlog-do-db=invoices_db
```

Copy

```
# Row events for tables in the user_db will be ignored. Events for any other database will be logged.
binlog-ignore-db=user_db
```

​

Next Step
--------------

Next, connect PowerSync to your database:

For PowerSync Cloud:
--------------------

Refer to **Database Connection**.

For Self-Hosted PowerSync:
--------------------------

Refer to **PowerSync Service Setup** in the Self-Hosting section.

Suggest editsRaise issue

Quickstart / OverviewSecurity & IP Filtering

Assistant

Responses are generated using AI and may contain mistakes.