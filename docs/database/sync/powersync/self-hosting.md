Title: Getting Started - PowerSync

Description: Self-host PowerSync in your own infrastructure (PowerSync Open Edition or PowerSync Enterprise Self-Hosted Edition).

PowerSync home page

Search...

⌘K

Search...

Navigation

Self-Hosting

Getting Started

Documentation

Client SDKs

Self-Hosting

Tutorials

Resources

*   GitHub
*   Community Discord
*   PowerSync Dashboard
*   PowerSync Website

##### Self-Hosting

*   Getting Started

*   Installation

*   Local Development

*   Lifecycle / Maintenance

*   Enterprise

*   Appendix

On this page

*   Overview Video (1 minute)
*   Demo Project (5 minutes)
*   Local Development With Docker Compose (variable)
*   Deploy PowerSync on Coolify (30 minutes)
*   Full Installation (1 hour)

The PowerSync Open Edition is currently considered a beta release as it still requires more detailed documentation and guides.From a stability perspective the Open Edition is production-ready as it uses the same codebase as our Cloud version.Please reach out on our Discord if you have any questions not yet covered in these docs.

Subscribe to monthly product update emails
------------------------------------------

The PowerSync Service can be self-hosted using Docker. It is published to Docker Hub as journeyapps/powersync-service Note that the PowerSync Dashboard is currently not available in the PowerSync Open Edition. We have five starting points, detailed below:

​

Overview Video (1 minute)
------------------------------

This video provides a quick introduction to the PowerSync Open Edition:

​

Demo Project (5 minutes)
-----------------------------

The quickest way to get a feel for the system is to run our example project on your development machine using Docker Compose. You can find it here:

GitHub - powersync-ja/self-host-demo
------------------------------------

​

Local Development With Docker Compose (variable)
-----------------------------------------------------

If you plan to self-host for development purposes only, we have a section describing how to easily do this using Docker Compose: Local Development

​

Deploy PowerSync on Coolify (30 minutes)
---------------------------------------------

See our integration guide for deploying the PowerSync Service on Coolify. This can simplify the setup and management of the deployment.

​

Full Installation (1 hour)
-------------------------------

See our Installation section for instructions to run the PowerSync Service in a production environment.

Suggest editsRaise issue

Overview

Assistant

Responses are generated using AI and may contain mistakes.

Title: Local Development - PowerSync

Description: Using Docker Compose to simplify your local development stack

PowerSync home page

Search...

⌘K

Search...

Navigation

Self-Hosting

Local Development

Documentation

Client SDKs

Self-Hosting

Tutorials

Resources

*   GitHub
*   Community Discord
*   PowerSync Dashboard
*   PowerSync Website

##### Self-Hosting

*   Getting Started

*   Installation

*   Local Development

*   Lifecycle / Maintenance

*   Enterprise

*   Appendix

It’s possible to host the full PowerSync Service stack on your development machine using pure Docker, but Docker Compose can simplify things. Below is a minimal Docker Compose setup for self-hosting PowerSync on your development machine. Note that Docker Compose is primarily designed for development and testing environments.

1.  Create a new folder to work in:

Copy

```
mkdir powersync-service && cd powersync-service
```

1.  Create a `docker-compose.yml` file with the below contents:

Copy

```
services:
powersync:
restart: unless-stopped
depends_on:
mongo-rs-init:
condition: service_completed_successfully
postgres: # This is not required, but is nice to have
condition: service_healthy
image: journeyapps/powersync-service:latest
command: ["start", "-r", "unified"]
volumes:
- ./config/config.yaml:/config/config.yaml
environment:
POWERSYNC_CONFIG_PATH: /config/config.yaml
ports:
- 8080:8080

postgres:
image: postgres:latest
restart: always
environment:
- POSTGRES_USER=postgres
- POSTGRES_DB=postgres
- POSTGRES_PASSWORD=postgres
- PGPORT=5432
volumes:
- pg_data:/var/lib/postgresql/data
ports:
- "5432:5432"
command: ["postgres", "-c", "wal_level=logical"]
healthcheck:
test: ["CMD-SHELL", "pg_isready -U postgres -d postgres"]
interval: 5s
timeout: 5s
retries: 5

# MongoDB Service used internally
mongo:
image: mongo:7.0
command: --replSet rs0 --bind_ip_all --quiet
restart: unless-stopped
ports:
- 27017:27017
volumes:
- mongo_storage:/data/db
# Initializes the MongoDB replica set. This service will not usually be actively running
mongo-rs-init:
image: mongo:7.0
depends_on:
- mongo
restart: on-failure
entrypoint:
- bash
- -c
- 'mongosh --host mongo:27017 --eval ''try{rs.status().ok && quit(0)} catch {} rs.initiate({_id: "rs0", version: 1, members: [{ _id: 0, host : "mongo:27017" }]})'''

volumes:
mongo_storage:
pg_data:

```

1.  Create a config volume that contains a `config.yaml` file, this configured the PowerSync Service itself

Copy

```
mkdir config && cd config
```

Put the below into `/config/config.yaml` :

Copy

```
replication:
connections:
- type: postgresql
uri: postgresql://postgres:postgres@postgres:5432/postgres

# SSL settings
sslmode: disable # 'verify-full' (default) or 'verify-ca' or 'disable'

# Connection settings for sync bucket storage
storage:
type: mongodb
uri: mongodb://mongo:27017/powersync_demo

# The port which the PowerSync API server will listen on
port: 8080

# Specify sync rules
sync_rules:
# TODO use specific sync rules here
content: |
bucket_definitions:
global:
data:
- SELECT * FROM lists
- SELECT * FROM todos

# Settings for client authentication
client_auth:
# Enable this if using Supabase Auth
supabase: false

# JWKS URIs can be specified here.
jwks_uri: [TODO]

# JWKS audience
audience: ['powersync-dev', 'powersync']

# Settings for telemetry reporting
# See https://docs.powersync.com/self-hosting/telemetry
telemetry:
# Opt out of reporting anonymized usage metrics to PowerSync telemetry service
disable_telemetry_sharing: false
```

For some additional details on this file, see PowerSync Service Setup. Next, the `client_auth` sections needs to be completed. The PowerSync Service can verify JWTs from client applications using either HMAC (HS\*) or RSA (RS\*) based algorithms. It can also obtain the necessary settings from Supabase automatically if you are using it.

1.  In the case of Supabase, simply set the `supabase` key to `true`
2.  In the case of HS\* algorithms, specify the secret as base64 encoded in the `k`field.
3.  In the case of RS\* based algorithms, the public key(s) can be specified either by supplying a JWKS URI or hard coding the public key in the config file.
1.  If using a JWKS URI, we have an example endpoint available here; ensure that your response looks similar.
2.  If hardcoding, see the syntax below. We also have an example key generator script.

example of hard coded HS256 config

Copy

```
client_auth:
jwks:
keys:
- kty: 'RSA'
n: 't-3d9e6XGtVsDB49CxVPn6P4OK6ir-wHP0CtTTq3VK6ofz2TWNrcHbCks6MszyWuBN1qb1ir_qudwwIeS69InEFm9WOYG1jIp6OBUNY4LPvkWfhSqcU6BasRAkYllC65CnSiVuTs4TlVgE-CBZQwQCvyrYgQgczC-GnI2HEB2SGTnXnBTXmAFEAd7xh_IROURZm1C6RnD2fXmiR1PxJsBn1w2hWYk0L8rQPlkthXwHNKd964rDir2qSTzVaHVvrFaxKiTlTe8uP4PR6OZT4pE0NDI2KNkyPauIeXp8HrwpZiUd8Znc8LQ-mj-hxfxtynYhxvcd6O_jyxa_41wjPeeQ'
e: 'AQAB'
alg: 'RS256'
kid: 'powersync-0abb8a873a'
```

Suggest editsRaise issue

App Backend SetupOverview

Assistant

Responses are generated using AI and may contain mistakes.