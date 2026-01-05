Title: Supabase + PowerSync - PowerSync

Description: Tutorial-style integration guide for creating offline-first apps with Supabase and PowerSync, using a demo to-do list app in Flutter, React Native, Web, Kotlin Multiplatform and Swift.

PowerSync home page

Search...

⌘K

Search...

Navigation

Supabase + PowerSync

Supabase + PowerSync

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

*   Overview

*   Handling Attachments

*   Real-time Streaming

*   RLS and Sync Rules

*   Local Development

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

*   Architecture
*   Integration Guide/Tutorial Overview
*   Configure Supabase
*   Create the Demo Database Schema
*   Create a PowerSync Database User
*   Create the Postgres Publication
*   Configuring PowerSync
*   Create a PowerSync Cloud Instance
*   Connect PowerSync to Your Supabase
*   Configure Sync Rules
*   Test Everything (Using Our Demo App)
*   Clone the demo app
*   Configure the demo app to use your PowerSync instance
*   Run the app
*   Bonus: Optional Extras

Video walkthrough of the integration guide.

Used in conjunction with **Supabase**, PowerSync enables developers to build local-first & offline-first apps that are robust in poor network conditions and that have highly responsive frontends while relying on Supabase for their backend. This guide provides instructions for how to configure PowerSync for use with your Supabase project.

Before you proceed, this guide assumes that you have already signed up for free accounts with both Supabase and PowerSync Cloud (our cloud-hosted offering). If you haven’t signed up for a **PowerSync** (Cloud) account yet, click here (and if you haven’t signed up for Supabase yet, click here).

For mobile/desktop apps, this guide assumes that you already have **Flutter / React Native / Kotlin Multiplatform / Xcode** set up.For web apps, this guide assumes that you have pnpm installed.

This guide takes 10-15 minutes to complete.

​

Architecture
-----------------

Upon successful integration of Supabase + PowerSync, your system architecture will look like this: (click to enlarge image)

The local SQLite database embedded in the PowerSync SDK is automatically kept in sync with the Supabase Postgres database (based on configured sync rules as you will see later in this guide). Client-side data modifications are persisted in the local SQLite database as well as stored in an upload queue that gets processed via the Supabase client library when network connectivity is available. Therefore reads and writes can happen in the app regardless of whether the user is online or offline, by using the local SQLite database.

For more details on PowerSync’s general architecture, see here.

​

Integration Guide/Tutorial Overview
----------------------------------------

We will follow these steps to get an offline-first ‘To-Do List’ demo app up and running:

1

Configure Supabase:

*   Create the demo database schema
*   Create the Postgres user and publication

2

Configure PowerSync:

*   Create connection to Supabase
*   Configure Sync Rules

3

Test the configuration

Test the configuration using our provided PowerSync-Supabase ‘To-Do List’ demo app with your framework of choice.

​

Configure Supabase
-----------------------

Create a new Supabase project (or use an existing project if you prefer) and follow the below steps.

### 

​

Create the Demo Database Schema

To set up the Postgres database for our _To-Do List_ demo app, we will create two new tables: `lists` and `todos`. The demo app will have access to these tables even while offline. Run the below SQL statements in your **Supabase SQL Editor**:

Copy

```
create table
public.lists (
id uuid not null default gen_random_uuid (),
created_at timestamp with time zone not null default now(),
name text not null,
owner_id uuid not null,
constraint lists_pkey primary key (id),
constraint lists_owner_id_fkey foreign key (owner_id) references auth.users (id) on delete cascade
) tablespace pg_default;

create table
public.todos (
id uuid not null default gen_random_uuid (),
created_at timestamp with time zone not null default now(),
completed_at timestamp with time zone null,
description text not null,
completed boolean not null default false,
created_by uuid null,
completed_by uuid null,
list_id uuid not null,
constraint todos_pkey primary key (id),
constraint todos_created_by_fkey foreign key (created_by) references auth.users (id) on delete set null,
constraint todos_completed_by_fkey foreign key (completed_by) references auth.users (id) on delete set null,
constraint todos_list_id_fkey foreign key (list_id) references lists (id) on delete cascade
) tablespace pg_default;
```

### 

​

Create a PowerSync Database User

PowerSync uses the Postgres Write Ahead Log (WAL) to replicate data changes in order to keep PowerSync SDK clients up to date. Run the below SQL statement in your **Supabase SQL Editor** to create a Postgres role/user with replication privileges:

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

Create the Postgres Publication

Run the below SQL statement in your **Supabase SQL Editor** to create a Postgres publication:

Copy

```
-- Create a publication to replicate tables. 
-- Specify a subset of tables to replicate if required.
-- The publication must be named "powersync"
CREATE PUBLICATION powersync FOR ALL TABLES;
```

​

Configuring PowerSync
--------------------------

### 

​

Create a PowerSync Cloud Instance

1.  In the **Overview** workspace of the PowerSync Dashboard, you will be prompted to create your first instance:

If you’ve previously created an instance in your project, you can create an additional instance by navigating to **Manage instances** and clicking **Create new instance**:You can also create an entirely new project with its own set of instances. Click on the PowerSync icon in the top left corner of the Dashboard or on **Admin Portal** at the top of the Dashboard, and then click on **Create Project**.

2.  Give your instance a name, such as “Testing”.
3.  \[Optional\] You can change the default cloud region from US to EU, JP (Japan), AU (Australia) or BR (Brazil) if desired.
*   Note: Additional cloud regions will be considered on request, especially for customers on our Enterprise plan. Please contact us if you need a different region.
4.  \[Optional\] You can opt in to using the `Next` version of the Service, which may contain early access or experimental features. Always use the `Stable` version in production.
5.  Click **Next**.

### 

​

Connect PowerSync to Your Supabase

1.  From your Supabase Dashboard, select **Connect** in the top navigation bar (or follow this link):

2.  In the **Direct connection** section, copy the complete connection string (including the `[YOUR-PASSWORD]` placeholder)

3.  Back in the PowerSync Dashboard, paste the connection string into the **URI** field. PowerSync will automatically parse this URI to populate the database connection details.
4.  Update the **Username** and **Password** fields to use the `powersync_role` and password you created when configuring your Supabase for PowerSync (see Source Database Setup).
5.  Note: PowerSync includes Supabase’s CA certificate by default, so you can use `verify-full` SSL mode without additional configuration.
6.  Your connection settings should look similar to this:

7.  Verify your setup by clicking **Test Connection** and resolve any errors.
8.  Click **Next**.
9.  PowerSync will detect the Supabase connection and prompt you to enable Supabase auth. To enable it, copy your JWT Secret from your project’s settings (JWT Keys section in the Supabase dashboard) and paste it here: 

PowerSync is compatible with Supabase’s new JWT signing keys. See this Discord thread for details on how to configure auth on your connection if you are using these keys.

10.  Click **Enable Supabase auth** to finalize your connection settings.

PowerSync will now create an isolated cloud environment for your instance. This typically takes a minute or two.

You can update your instance settings by navigating to the **Manage instances** workspace, opening your instance options and selecting **Edit instance**.

### 

​

Configure Sync Rules

Sync Rules allow developers to control which data gets synced to which user devices using a SQL-like syntax in a YAML file. For the demo app, we’re going to specify that each user can only see their own to-do lists and list items.

1.  The final step is to replace the Sync Rules file’s contents with the below:

Copy

```
bucket_definitions:
user_lists:
# Separate bucket per To-Do list
parameters: select id as list_id from lists where owner_id = request.user_id()
data:
- select * from lists where id = bucket.list_id
- select * from todos where list_id = bucket.list_id
```

2.  Click **“Validate sync rules”** and ensure there are no errors. This validates your sync rules against your Postgres database.
3.  Click **“Save and deploy”** to deploy your Sync Rules.

*   Your Sync Rules can be updated by navigating to the **Manage instances** workspace and selecting the `sync-rules.yaml` file. 
*   For additional information on PowerSync’s Sync Rules, refer to the Sync Rules documentation.
*   If you’re wondering how Sync Rules relate to Supabase Postgres RLS, see this subsection.

​

Test Everything (Using Our Demo App)
-----------------------------------------

In this step you’ll test your setup using a ‘To-Do List’ demo app provided by PowerSync.

#### 

​

Clone the demo app

Clone the demo app based on your framework:

Flutter

React Native

JavaScript Web

Kotlin

Swift

Copy

```
git clone https://github.com/powersync-ja/powersync.dart.git
cd powersync.dart/demos/supabase-todolist/
```

#### 

​

Configure the demo app to use your PowerSync instance

Locate the relevant config file for your framework:

Flutter

React Native

JavaScript Web

Kotlin

Swift

Copy

```
cp lib/app_config_template.dart lib/app_config.dart

# Edit `lib/app_config.dart` and insert the necessary credentials as detailed below.
```

1.  In the relevant config file, replace the values for `supabaseUrl` (from the Project URL section in the Supabase dashboard) and `supabaseAnonKey` (from the API Keys section in the Supabase dashboard)
2.  For the value of `powersyncUrl`, click the copy icon on your instance to copy its URL:

#### 

​

Run the app

Flutter

React Native

JavaScript Web

Kotlin

Swift

Copy

```
# Ensure you have [melos](https://melos.invertase.dev/~melos-latest/getting-started) installed.

melos bootstrap
flutter run
```

For ease of use of the demo app, you can disable email confirmation in your Supabase Auth settings. In your Supabase project, go to “Authentication” -> “Providers” -> “Email” and then disable “Confirm email”. If you keep email confirmation enabled, the Supabase user confirmation email will reference the default Supabase Site URL of`http://localhost:3000` — you can ignore this.

Once signed in to the demo app, you should see a blank list of to-do lists, so go ahead and create a new list. Try placing your device into airplane mode to test out the offline capabilities. Once the device is back online, you should see the data automatically appear in your Supabase dashboard (e.g. in the Table Editor). For more information, explore the PowerSync docs or join us on our community Discord where our team is always available to answer questions.

​

Bonus: Optional Extras
---------------------------

If you plan on sharing this demo app with other people, you may want to set up demo data triggers so that new user signups don’t see an empty screen. It’s useful to have some data when a user signs up to the demo app. The below trigger automatically creates some sample data when a user registers (you can run it in the Supabase SQL Editor). See Supabase: Managing User Data for more details.

Copy

```
create function public.handle_new_user_sample_data()
returns trigger as $$
declare
new_list_id uuid;
begin
insert into public.lists (name, owner_id)
values ('Shopping list', new.id)
returning id into new_list_id;

insert into public.todos(description, list_id, created_by)
values ('Bread', new_list_id, new.id);

insert into public.todos(description, list_id, created_by)
values ('Apples', new_list_id, new.id);

return new;
end;
$$ language plpgsql security definer;

create trigger new_user_sample_data after insert on auth.users for each row execute procedure public.handle_new_user_sample_data();
```

Suggest editsRaise issue

Integrations OverviewHandling Attachments

Assistant

Responses are generated using AI and may contain mistakes.