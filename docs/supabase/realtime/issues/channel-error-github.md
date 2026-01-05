Bug report
Describe the bug
I am running a local web app that connects to a local version of Supabase Realtime running in Docker. The webapp uses supabase/realtime-js to connect to the realtime server. The Realtime server is connected to a hosted Postgres database on Supabase for my project.

This setup works when I point my web app to use Supabase's hosted Realtime servers. However, when I switch my realtime-js client library to point to my local Realtime server instead, I get an error.

On the client, I get a CHANNEL_ERROR and a null error object.

On the Realtime server, I get the following error:

realtime_1 | 2022-12-09 08:12:14.634 [info] CONNECTED TO RealtimeWeb.UserSocket in 64µs
realtime_1 | Transport: :websocket
realtime_1 | Serializer: Phoenix.Socket.V1.JSONSerializer
realtime_1 | Parameters: %{"apikey" => "<Redacted Supabase Anon JWT key>", "eventsPerSecond" => "160", "vsn" => "1.0.0"}
realtime_1 | 2022-12-09 08:12:14.657 [info] JOINED realtime:test-room-2 in 31µs
realtime_1 | Parameters: %{"config" => %{"broadcast" => %{"ack" => false, "self" => false}, "postgres_changes" => [], "presence" => %{"key" => "k56pkz275fzjq3j7h7o8rlf1lrsbnm"}}}
realtime_1 | 2022-12-09 08:12:14.666 [error] GenServer #PID<0.3943.0> terminating
realtime_1 | \*\* (FunctionClauseError) no function clause matching in RealtimeWeb.RealtimeChannel.handle_in/3
realtime_1 | (realtime 0.0.0-automated) lib/realtime_web/channels/realtime_channel.ex:138: RealtimeWeb.RealtimeChannel.handle_in("presence", %{"event" => "track", "payload" => %{"online_at" => "2022-12-09T08:12:14.664Z"}, "type" => "presence"}, %Phoenix.Socket{assigns: %{}, channel: RealtimeWeb.RealtimeChannel, channel_pid: #PID<0.3943.0>, endpoint: RealtimeWeb.Endpoint, handler: RealtimeWeb.UserSocket, id: nil, join_ref: "1", joined: true, private: %{log_handle_in: :debug, log_join: :info}, pubsub_server: Realtime.PubSub, ref: "2", serializer: Phoenix.Socket.V1.JSONSerializer, topic: "realtime:test-room-2", transport: :websocket, transport_pid: #PID<0.3941.0>})
realtime_1 | (phoenix 1.5.8) lib/phoenix/channel/server.ex:315: Phoenix.Channel.Server.handle_info/2
realtime_1 | (stdlib 3.14.2.2) gen_server.erl:689: :gen_server.try_dispatch/4
realtime_1 | (stdlib 3.14.2.2) gen_server.erl:765: :gen_server.handle_msg/6
realtime_1 | (stdlib 3.14.2.2) proc_lib.erl:226: :proc_lib.init_p_do_apply/3
realtime_1 | Last message: %Phoenix.Socket.Message{event: "presence", join_ref: "1", payload: %{"event" => "track", "payload" => %{"online_at" => "2022-12-09T08:12:14.664Z"}, "type" => "presence"}, ref: "2", topic: "realtime:test-room-2"}
To Reproduce
On the client I'm using realtime-js:

const localUrl = 'ws://localhost:4000/socket';
const realtimeClient = new RealtimeClient(localUrl, {
params: {
apikey: supabaseAnonKey,
eventsPerSecond: MAX_EVENTS_PER_SECOND,
},
});
On the backend I'm running the latest version of Realtime's Dockerfile from this repo, but modified the envvars with my Supabase Postgres settings.

version: '3'
services:
realtime:
image: supabase/realtime:latest
build: .
ports: - "4000:4000"
environment:
DB_HOST: db.<my project ID from supabase dashboard>.supabase.co
DB_NAME: postgres
DB_USER: postgres
DB_PASSWORD: <my Supabase password>
DB_PORT: 5432
PORT: 4000
JWT_SECRET: '<Redacted Supabase JWT secret from dashboard>'
SECURE_CHANNELS: 'false'
EXPOSE_METRICS: 'true'
DB_SSL: 'false'
DB_IP_VERSION: 'IPv4'
REPLICATION_MODE: 'STREAM'
SLOT_NAME: 'meshi1'
REALTIME_IP_VERSION: 'IPv6' # was v6
PUBLICATIONS: "[\"supabase_realtime\"]"
JWT_CLAIM_VALIDATORS: '{"iss": "Issuer", "nbf": 1610078130}'
MAX_REPLICATION_LAG_MB: 1000
Expected behavior
I would expect to be able to send and receive messages by using a self-hosted Realtime server with a hosted Supabase database.

Screenshots
n/a

System information
OS: macOS + Docker
Browser: Chrome
Version of supabase-js: "@supabase/realtime-js": "^2.1.0", "@supabase/supabase-js": "^2.0.2",
Version of Node.js: 16.15.1
Activity

Omarzipan
added
bug
Something isn't working
on Dec 9, 2022
chasers
chasers commented on Dec 14, 2022
chasers
on Dec 14, 2022
Contributor
We have an rc branch here which, when merged, we'll be shipping the latest version of Realtime with the rest of self-hosted Supabase.

Should be very soon!

Omarzipan
Omarzipan commented on Dec 15, 2022
Omarzipan
on Dec 15, 2022
Author
that's great! when that's merged will it also be reflected in the latest Docker image?

thecooltechguy
thecooltechguy commented on Dec 15, 2022
thecooltechguy
on Dec 15, 2022 · edited by thecooltechguy
I'm also facing issues trying to use Realtime supabase locally:

I'm using the latest supabase CLI locally w/ the supabase v2 js libraries in react, and I'm finding that realtime subscriptions to DB events isn't working.

In the logs for the realtime container, I see the following 2 messages repeatedly:

11:39:05.548 [warning] [libcluster:fly6pn] unable to connect to :"realtime@23.217.138.110"
11:39:12.111 project=realtime-demo external_id=realtime-demo [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "INSERT", "schema" => "public", "table" => "chat_messages"}}}
In react, the realtime subscription code looks something like:

        const subscription = supabase
            .channel(`public:chat_messages`)
            .on('postgres_changes', { event: 'INSERT', schema: 'public', table: 'chat_messages' }, payload => {
                console.log("REALTIME!");
                console.log({payload});
            })
            .subscribe()

The subscription object has state set to "joined", and the channel shows up if I subsequently call supabase.getChannels().

But for some reason, the payload function is never called and the websocket messages received by the react client from the websocket server indicate the same error message repeatedly:

{"event":"system","payload":{"channel":"public:chat_messages","extension":"postgres_changes","message":"Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{\"event\" => \"\*\", \"schema\" => \"public\", \"table\" => \"chat_messages\"}}}","status":"error"},"ref":null,"topic":"realtime:public:chat_messages"}

Are there any suggested workarounds for local development that needs real-time, until an update is merged? Thanks!

w3b6x9
w3b6x9 commented on Dec 16, 2022
w3b6x9
on Dec 16, 2022
Member
@thecooltechguy did you remember to add the chat_messages table to the supabase_realtime publication?

alter publication supabase_realtime add table chat_messages;
thecooltechguy
thecooltechguy commented on Dec 16, 2022
thecooltechguy
on Dec 16, 2022
@w3b6x9 THANK U SO MUCH! It turned out that I forgot to enable "Realtime" for the table after I upgraded my supabase local setup to use the latest container images. Now, realtime works locally!

chasers
closed this as completedon Dec 16, 2022

w3b6x9
reopened this on Dec 16, 2022
w3b6x9
w3b6x9 commented on Dec 16, 2022
w3b6x9
on Dec 16, 2022 · edited by w3b6x9
Member
@Omarzipan try pointing the Realtime image at v1.0.0-rc.14 and using these env vars starting here: https://github.com/supabase/cli/blob/2283024565952c1ae4a6a9bf144f2abe230e4ce4/internal/start/start.go#L305.

I'm currently updating Realtime's docker-compose.yml so this will be much more straightforward.

bhvngt
bhvngt commented on Dec 17, 2022
bhvngt
on Dec 17, 2022 · edited by bhvngt
try pointing the Realtime image at v1.0.0-rc.14 and using these env vars starting here: https://github.com/supabase/cli/blob/2283024565952c1ae4a6a9bf144f2abe230e4ce4/internal/start/start.go#L305.

@w3b6x9. I have a docker compose setup. When I upgrade the image to v1.0.0-rc.14 from v1.0.0-rc.12 with the above env vars, I get following warning and error. I am using @supabase/supabase-js v2.2.0

supabase-realtime | 05:55:13.943 [error] #PID<0.3017.0> running RealtimeWeb.Endpoint (connection #PID<0.3011.0>, stream id 2) terminated
supabase-realtime | Server: realtime:4000 (http)
supabase-realtime | Request: GET /socket/websocket?apikey=xxxxxxxxx&vsn=1.0.0
supabase-realtime | ** (exit) an exception was raised:
supabase-realtime | ** (Postgrex.Error) ERROR 42P01 (undefined_table) relation "tenants" does not exist
supabase-realtime |
supabase-realtime | query: SELECT t0."id", t0."name", t0."external_id", t0."jwt_secret", t0."postgres_cdc_default", t0."max_concurrent_users", t0."max_events_per_second", t0."inserted_at", t0."updated_at" FROM "tenants" AS t0 WHERE (t0."external_id" = $1)
supabase-realtime | (ecto_sql 3.8.3) lib/ecto/adapters/sql.ex:932: Ecto.Adapters.SQL.raise_sql_call_error/1
supabase-realtime | (ecto_sql 3.8.3) lib/ecto/adapters/sql.ex:847: Ecto.Adapters.SQL.execute/6
supabase-realtime | (ecto 3.8.4) lib/ecto/repo/queryable.ex:221: Ecto.Repo.Queryable.execute/4
supabase-realtime | (ecto 3.8.4) lib/ecto/repo/queryable.ex:19: Ecto.Repo.Queryable.all/3
supabase-realtime | (ecto 3.8.4) lib/ecto/repo/queryable.ex:147: Ecto.Repo.Queryable.one/3
supabase-realtime | (realtime 2.0.0) lib/realtime/api.ex:132: Realtime.Api.get_tenant_by_external_id/1
supabase-realtime | (realtime 2.0.0) lib/realtime_web/channels/user_socket.ex:44: RealtimeWeb.UserSocket.connect/3
supabase-realtime | (phoenix 1.6.13) lib/phoenix/socket.ex:550: Phoenix.Socket.user_connect/6
supabase-realtime | 05:55:13.958 [warning] [libcluster:fly6pn] dns polling strategy is selected, but query or basename param is invalid: %{node_basename: "realtime", query: ""}
FYI - tenants is one of my domain table which is not part of realtime publication. I have only one table that is part of realtime publication. My realtime subscription code is not subscibing to tenants

I also keep getting following warning

06:01:34.137 [warning] [libcluster:fly6pn] dns polling strategy is selected, but query or basename param is invalid: %{node_basename: "realtime", query: ""}
My docker-compose file for realtime is setup as follows

realtime:
container_name: supabase-realtime
image: supabase/realtime:v1.0.0-rc.14
depends_on: - db
restart: unless-stopped
environment:
DB_HOST: db
DB_PORT: 5432
DB_NAME: ${PGDATABASE}
DB_USER: ${PGUSER}
DB_PASSWORD: ${PGPASSWORD}
PORT: 4000
DB_AFTER_CONNECT_QUERY: "SET search_path TO \_realtime"
DB_ENC_KEY: "supabaserealtime"
FLY_ALLOC_ID: "abc123"
FLY_APP_NAME: "realtime"
SECRET_KEY_BASE: "EAx3IQ/wRG1v47ZD4NE4/9RzBI8Jmil3x0yhcW4V2NHBP6c2iPIzwjofi2Ep4HIG"
ERL_AFLAGS: "-proto_dist inet_tcp"
ENABLE_TAILSCALE: false
DNS_NODES: ''
docker compose config with v1.0.0-rc.12 that works well without any code modification is as follows

realtime:
container_name: supabase-realtime
image: supabase/realtime:v1.0.0-rc.12
depends_on: - db
restart: unless-stopped
environment:
DB_HOST: db
DB_PORT: 5432
DB_NAME: ${PGDATABASE}
DB_USER: ${PGUSER}
DB_PASSWORD: ${PGPASSWORD}
DB_SSL: "false"
PORT: 4000
JWT_SECRET: ${JWT_SECRET}
REPLICATION_MODE: RLS
REPLICATION_POLL_INTERVAL: 100
SECURE_CHANNELS: "true"
SLOT_NAME: supabase_realtime_rls
TEMPORARY_SLOT: "true"
command: >
bash -c "./prod/rel/realtime/bin/realtime eval Realtime.Release.migrate
&& ./prod/rel/realtime/bin/realtime start"
nuKs
nuKs commented on Dec 19, 2022
nuKs
on Dec 19, 2022
I have the same issue, except 1.0.0-rc.12 (or even 1.0.0-rc.1) doesn't work either: I get supabase-realtime | bash: line 1: ./prod/rel/realtime/bin/realtime: No such file or directory. + issues with ENABLE_TAILSCALE var not set if I remove the docker-compose command expression.

supabase/realtime:v0.25.1 works fine with the old settings but is not compatible with latest other docker images due to missing postgres procedure.

itisnajim
itisnajim commented on Dec 19, 2022
itisnajim
on Dec 19, 2022 · edited by itisnajim
@bhvngt @w3b6x9 how did you get supabase/realtime:v1.0.0-rc.12 working?, as @nuKs mention ./prod/rel/realtime/bin/realtime: No such file or directory error will be thrown!

w3b6x9
w3b6x9 commented on Dec 20, 2022
w3b6x9
on Dec 20, 2022
Member
Here's what you can do to get it running locally:

Check out branch chore/docker-setup. You can see the changes to docker-compose.yml here: https://github.com/supabase/realtime/pull/377/files#diff-e45e45baeda1c1e73482975a664062aa56f20c03dd9d64a827aba57775bed0d3.
Slightly tweak the docker-compose.yml file so that it points to supabase/realtime:v1.0.0-rc.14:
...

realtime:
depends_on: - db
image: supabase/realtime:v1.0.0-rc.14
container_name: realtime-server
ports: - "4000:4000"
environment:
PORT: 4000
DB_HOST: host.docker.internal
DB_PORT: 5432
DB_USER: postgres
DB_PASSWORD: postgres
DB_NAME: postgres
DB_ENC_KEY: supabaserealtime
DB_AFTER_CONNECT_QUERY: 'SET search_path TO \_realtime'
API_JWT_SECRET: dc447559-996d-4761-a306-f47a5eab1623
FLY_ALLOC_ID: fly123
FLY_APP_NAME: realtime
SECRET_KEY_BASE: UpNVntn3cDxHJpq99YMc1T1AQgQpc8kfYTuRgBiYa15BLrx8etQoXz3gZv1/u2oq
ERL_AFLAGS: -proto_dist inet_tcp
ENABLE_TAILSCALE: false
DNS_NODES: "''"
command: sh -c "/app/bin/migrate && /app/bin/server"
Make a POST request to insert a tenant into the \_realtime.tenant table. Supabase Realtime in production has a separate database for all tenants but for local development we created a \_realtime to make setup easier. The Authorization token is signed with the secret found in the env var API_JWT_SECRET.
curl -X POST \
 -H 'Content-Type: application/json' \
 -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIiLCJpYXQiOjE2NzEyMzc4NzMsImV4cCI6MTcwMjc3Mzk5MywiYXVkIjoiIiwic3ViIjoiIn0.\_ARixa2KFUVsKBf3UGR90qKLCpGjxhKcXY4akVbmeNQ' \
 -d $'{
"tenant" : {
"name": "realtime-dev",
"external_id": "realtime-dev",
"jwt_secret": "a1d99c8b-91b6-47b2-8f3c-aa7d9a9ad20f",
"extensions": [
{
"type": "postgres_cdc_rls",
"settings": {
"db_name": "postgres",
"db_host": "host.docker.internal",
"db_user": "postgres",
"db_password": "postgres",
"db_port": "5432",
"region": "us-west-1",
"poll_interval_ms": 100,
"poll_max_record_bytes": 1048576,
"ip_version": 4
}
}
]
}
}' \
 http://localhost:4000/api/tenants
Create a table in the database that you want Realtime to listen to.
Use a client like @supabase/realtime-js or go to http://localhost:4000/inspector/new. If using the Realtime Inspector, set the Path to ws://realtime-dev.localhost:4000/socket and Token to eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MDMwMjgwODcsInJvbGUiOiJwb3N0Z3JlcyJ9.tz_XJ89gd6bN8MBpCl7afvPrZiBH6RB65iA1FadPT3Y (signed with jwt_secret as part of POST request).
I'm in the process of updating the README and will include the above instructions for easier local setup.

w3b6x9
pinned this issue on Dec 20, 2022

w3b6x9
mentioned this on Dec 20, 2022
Realtime doesn't run on image: realtime/walrus. #378
27 remaining items

w3b6x9
unpinned this issue on Jan 6, 2023
itisnajim
itisnajim commented on Jan 6, 2023
itisnajim
on Jan 6, 2023
@bhvngt sorry to bother you again, i want to ask how did you get rid of this error:
[error] Could not create schema migrations table. ?

Now i have this error message is repeating on the terminal multiple times:

[error] Could not create schema migrations table.
supabase-realtime | {%Postgrex.Error{
supabase-realtime | message: nil,
supabase-realtime | postgres: %{
supabase-realtime | code: :insufficient_privilege,
supabase-realtime | file: "aclchk.c",
supabase-realtime | line: "3447",
supabase-realtime | message: "permission denied for schema realtime",
supabase-realtime | pg_code: "42501",
supabase-realtime | position: "28",
supabase-realtime | routine: "aclcheck_error",
supabase-realtime | severity: "ERROR",
supabase-realtime | unknown: "ERROR"
supabase-realtime | },
supabase-realtime | connection_id: 552,
supabase-realtime | query: nil
supabase-realtime | },
in the flutter console no error(s) shown: flutter: SUBSCRIBED err: null

bhvngt
bhvngt commented on Jan 6, 2023
bhvngt
on Jan 6, 2023
@itisnajim I never got this error. Are you using the same PG_USER to initialise db container as well as to connect to the realtime container?

itisnajim
itisnajim commented on Jan 6, 2023
itisnajim
on Jan 6, 2023
@itisnajim I never got this error. Are you using the same PG_USER to initialise db container as well as to connect to the realtime container?

@bhvngt , its a fresh download, i guess = postgres according to:

https://github.com/supabase/supabase/blob/master/docker/docker-compose.yml#L117

https://github.com/supabase/supabase/blob/master/docker/.env.example#L18

i just created an issue in the main repo
supabase/supabase#11484

bhvngt
bhvngt commented on Jan 6, 2023
bhvngt
on Jan 6, 2023
@itisnajim I am not using the latest download. I had manually created realtime.sql earlier with following sql.

create schema if not exists \_realtime;
create schema if not exists realtime;

create publication supabase_realtime with (publish = 'insert, update, delete');
Since POSTGRES_USER is not set as env variable inside db container, it could be the cause of the error.

Try setting POSTGRES_USER for the db container

itisnajim
itisnajim commented on Jan 6, 2023
itisnajim
on Jan 6, 2023 · edited by itisnajim
i solved by replacing ${POSTGRES_USER} with supabase_admin
in:

realtime:
container_name: realtime-dev.supabase-realtime
image: supabase/realtime:v2.0.2
...
DB_USER: ${POSTGRES_USER} # should be DB_USER: supabase_admin
code line:
https://github.com/supabase/supabase/blob/master/docker/docker-compose.yml#L117

now the docker logs filled with:

realtime-dev.supabase-realtime | 13:47:51.016 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:48:00.546 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:48:09.600 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:48:16.805 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:48:23.356 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:48:31.382 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:48:41.215 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:48:51.012 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:49:00.391 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
realtime-dev.supabase-realtime | 13:49:06.924 project=realtime-dev external_id=realtime-dev [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "_", "schema" => "public"}}}
itisnajim
itisnajim commented on Jan 6, 2023
itisnajim
on Jan 6, 2023
restarting the docker and it shows no [error] Subscribing to PostgreSQL failed: {:error, {:subscription_insert_failed, %{"event" => "\*", "schema" => "public"}}}

itisnajim
mentioned this on Jan 6, 2023
Docker logs: [realtime error] Could not create schema migrations table supabase#11484

mr-moto
mentioned this on Jan 8, 2023
Realtime not working on local ( using CLI ) #415
jsmueller7
jsmueller7 commented on Jun 7, 2023
jsmueller7
on Jun 7, 2023 · edited by jsmueller7
Hi @bhvngt

I also keep getting following warning

06:01:34.137 [warning] [libcluster:fly6pn] dns polling strategy is selected, but query or basename param is invalid: %{node_basename: "realtime", query: ""}
Here too. Every 10 seconds I receive this warning twice.

[warning] [libcluster:fly6pn] dns polling strategy is selected, but query or basename param is invalid: %{node_basename: "realtime", query: nil}
I am using a newer version of realtime (2.13.1) and I haven't actually checkt, if these messages are of any relevance. Did they go away when you changed your config? Are they related to a misconfiguration at all? Has anyone else has them?

lwjameson
mentioned this on Aug 23, 2023
Realtime: Self Hosting - Docker Swarm mode #645

psyrenpark
added a commit that references this issue on Jan 26, 2024
Fix: Realtime Service Issue and StudioBranch Error in Supabase Library

Verified
e7f4cf3

psyrenpark
mentioned this on Jan 26, 2024
fix: Fix for Realtime Functionality in Supabase-on-AWS Description supabase-community/supabase-on-aws#86
AntonOfTheWoods
AntonOfTheWoods commented on Jun 6, 2024
AntonOfTheWoods
on Jun 6, 2024
@jsmueller7 did those warnings turn out to symptomatic of deeper problems or are they "normal" errors?

jsmueller7
jsmueller7 commented on Jun 6, 2024
jsmueller7
on Jun 6, 2024
Hi @AntonOfTheWoods ,
these warnings were due to the fact that I was running Supabase in a cluster and the env DNS_NODES was not configured. So my post has been resolved.
