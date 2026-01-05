Title: Docker Compose | Dokploy

Description: Learn how to use Docker Compose with Dokploy

Keywords: dokploy,vps,open source,cloud,self hosting,free

Dokploy

Core

The core of Dokploy

Search

⌘K

WebsiteDiscordSupportGithubBlog

Introduction

Welcome to DokployArchitecture of DokployFeaturesInstallationManual InstallationReset Password & 2FAUninstallVideosComparisonGoodiesTroubleshootingCloud

Dokploy CloudMonitoringServer

S3 Destinations

Git Sources

Users

Notifications

Registry

SSH KeysCertificatesBackupsServices

Environment Variables

Domains

Applications

Docker Compose

DomainsExampleUtilities

Databases

Open Source Templates

Examples

Auto DeploySchedule JobsVolume BackupsProvidersWatch PathsAdvanced

Cluster

Multi Server

On this page

Docker Compose
==============

Learn how to use Docker Compose with Dokploy

Dokploy integrates with Docker Compose and Docker Stack to provide flexible deployment solutions. Whether you are developing locally or deploying at scale, Dokploy facilitates application management through these powerful Docker tools.

### Configuration Methods

Dokploy provides two methods for creating Docker Compose configurations:

*   **Docker Compose**: Ideal for standard Docker Compose configurations.
*   **Stack**: Geared towards orchestrating applications using Docker Swarm. Note that some Docker Compose features, such as `build`, are not available in this mode.

### General

Configure the source of your code, the way your application is built, and also manage actions like deploying, updating, and deleting your application, and stopping it.

### Enviroment

A code editor within Dokploy allows you to specify environment variables for your Docker Compose file. By default, Dokploy creates a `.env` file in the specified Docker Compose file path.

### Monitoring

Monitor each service individually within Dokploy. If your application consists of multiple services, each can be monitored separately to ensure optimal performance.

### Logs

Access detailed logs for each service through the Dokploy log viewer, which can help in troubleshooting and ensuring the stability of your services.

### Deployments

You can view the last 10 deployments of your application. When you deploy your application in real time, a new deployment record will be created and it will gradually show you how your application is being built.

We also offer a button to cancel deployments that are in queue. Note that those in progress cannot be canceled.

We provide a webhook so that you can trigger your own deployments by pushing to your GitHub, Gitea, GitLab, Bitbucket repository.

### Advanced

This section provides advanced configuration options for experienced users. It includes tools for custom commands within the container and volumes.

*   **Command**: Dokploy has a defined command to run the Docker Compose file, ensuring complete control through the UI. However, you can append flags or options to the command.
*   **Volumes**: To ensure data persistence across deployments, configure storage volumes for your application.

Volumes

Docker volumes are a way to persist data generated and used by Docker containers. They are particularly useful for maintaining data between container restarts or for sharing data among different containers.

To bind a volume to the host machine, you can use the following syntax in your docker-compose.yml file, but this way will clean up the volumes when a new deployment is made:

```
volumes:
- "/folder:/path/in/container" ❌
```

It's recommended to use the ../files folder to ensure your data persists between deployments. For example:

```
volumes:
- "../files/my-database:/var/lib/mysql" ✅
- "../files/my-configs:/etc/my-app/config" ✅
```

Previous

Zero Downtime

Next

Domains

### On this page

Configuration MethodsGeneralEnviromentMonitoringLogsDeploymentsAdvanced

Title: Domains | Dokploy

Description: Configure domains for your Docker Compose application.

Keywords: dokploy,vps,open source,cloud,self hosting,free

Dokploy

Core

The core of Dokploy

Search

⌘K

WebsiteDiscordSupportGithubBlog

Introduction

Welcome to DokployArchitecture of DokployFeaturesInstallationManual InstallationReset Password & 2FAUninstallVideosComparisonGoodiesTroubleshootingCloud

Dokploy CloudMonitoringServer

S3 Destinations

Git Sources

Users

Notifications

Registry

SSH KeysCertificatesBackupsServices

Environment Variables

Domains

Applications

Docker Compose

DomainsExampleUtilities

Databases

Open Source Templates

Examples

Auto DeploySchedule JobsVolume BackupsProvidersWatch PathsAdvanced

Cluster

Multi Server

On this page

Docker Compose

Domains
=======

Configure domains for your Docker Compose application.

When using Docker Compose, adding a domain to a service is a straightforward process. This guide will walk you through the necessary steps to configure manual domains for your application.

Key Steps:

1.  Add the service to the `dokploy-network`.
2.  Use Traefik labels to configure routing.

Attention

Since v0.7.0 Dokploy support domains natively. This means that you can configure your domain directly in the Dokploy UI, without doing the rest of the steps check on the domains section.

Example Scenario

Let's consider an application with three components: a frontend, a backend, and a database. We'll start with a basic Docker Compose file and then enhance it with domain configuration.

```
version: "3.8"

services:
frontend:
build:
context: ./frontend
dockerfile: Dockerfile
volumes:
- ./frontend:/app
ports:
- "3000:3000"
depends_on:
- backend

backend:
build:
context: ./backend
dockerfile: Dockerfile
volumes:
- ./backend:/app
ports:
- "5000:5000"
environment:
- DATABASE_URL=postgres://postgres:password@database:5432/mydatabase
depends_on:
- database

database:
image: postgres:13
environment:
POSTGRES_USER: postgres
POSTGRES_PASSWORD: password
POSTGRES_DB: mydatabase
volumes:
- db-data:/var/lib/postgresql/data

volumes:
db-data:
```

Step 1: Add the Network
-----------------------

First, we'll add the dokploy-network to our services:

```
version: "3.8"

services:
frontend:
# ... (previous configuration)
networks:
- dokploy-network

backend:
# ... (previous configuration)
networks:
- dokploy-network

database:
# ... (previous configuration)
networks:
- dokploy-network

volumes:
db-data:

networks:
dokploy-network:
external: true
```

Step 2: Configuring Traefik Labels

Now, let's add Traefik labels to route domains to our services. We'll focus on the frontend and backend services:

```
version: "3.8"

services:
frontend:
build:
context: ./frontend
dockerfile: Dockerfile
volumes:
- ./frontend:/app
expose:
- 3000
depends_on:
- backend
networks:
- dokploy-network
labels:
- traefik.enable=true
- traefik.http.routers.frontend-app.rule=Host(`frontend.dokploy.com`)
- traefik.http.routers.frontend-app.entrypoints=web
- traefik.http.services.frontend-app.loadbalancer.server.port=3000

backend:
build:
context: ./backend
dockerfile: Dockerfile
volumes:
- ./backend:/app
expose:
- 5000
environment:
- DATABASE_URL=postgres://postgres:password@database:5432/mydatabase
depends_on:
- database
networks:
- dokploy-network
labels:
- traefik.enable=true
- traefik.http.routers.backend-app.rule=Host(`backend.dokploy.com`)
- traefik.http.routers.backend-app.entrypoints=web
- traefik.http.services.backend-app.loadbalancer.server.port=5000

database:
# ... (same as before)

volumes:
db-data:

networks:
dokploy-network:
external: true
```

Understanding Traefik Labels

1.  `traefik.enable=true` Enables Traefik routing for the service.
2.  `traefik.http.routers.<UNIQUE-RULE>.rule=Host('your-domain.dokploy.com')` Specifies the domain for the service
3.  `traefik.http.routers.<UNIQUE-RULE>.entrypoints=web` Sets the service to be accessible via HTTP.
4.  `traefik.http.services.<UNIQUE-RULE>.loadbalancer.server.port=3000` Specifies the port your service is using internally.

**Note**: Replace `<UNIQUE-RULE>` with a unique identifier for each service (e.g., frontend-app, backend-app, etc.).

Important Considerations
------------------------

1.  **Port Exposure**: Use `expose` instead of `ports` to limit port access to the container network, avoiding exposure to the host machine.
2.  **DNS Configuration**: Ensure you create `A` records pointing to your domain in your DNS Provider Settings.
3.  **HTTPS**: For HTTPS, you can use Let's Encrypt or other SSL/TLS certificates.

Deployment
----------

With these configurations in place, you're now ready to deploy your application using Docker Compose. This setup should be sufficient to get your services up and running with custom domain routing through Traefik.

If you have any further questions or need assistance, join our Discord server and we'll be happy to help.

Previous

Docker Compose

Next

Example

### On this page

Title: Example | Dokploy

Description: Learn how to use Docker Compose with Dokploy

Keywords: dokploy,vps,open source,cloud,self hosting,free

Dokploy

Core

The core of Dokploy

Search

⌘K

WebsiteDiscordSupportGithubBlog

Introduction

Welcome to DokployArchitecture of DokployFeaturesInstallationManual InstallationReset Password & 2FAUninstallVideosComparisonGoodiesTroubleshootingCloud

Dokploy CloudMonitoringServer

S3 Destinations

Git Sources

Users

Notifications

Registry

SSH KeysCertificatesBackupsServices

Environment Variables

Domains

Applications

Docker Compose

DomainsExampleUtilities

Databases

Open Source Templates

Examples

Auto DeploySchedule JobsVolume BackupsProvidersWatch PathsAdvanced

Cluster

Multi Server

On this page

Docker Compose

Example
=======

Learn how to use Docker Compose with Dokploy

Tutorial
--------

In this tutorial, we will create a simple application using Docker Compose and route the traffic to an accessible domain.

### Steps

1.  Create a new project.
2.  Create a new service `Compose` and select the Compose Type `Docker Compose`.
3.  Fork this repository: Repo.
4.  Select Provider type: GitHub or Git.
5.  Select the repository: `Dokploy/docker-compose-test`.
6.  Select the branch: `main`.
7.  Set the Compose Path to `./docker-compose.yml` and save. 

### Updating Your `docker-compose.yml`

Add the following to your existing `docker-compose.yml` file:

1.  Add the network `dokploy-network` to each service.
2.  Add labels for Traefik to make the service accessible through the domain.

Example:

Let's modify the following compose file to make it work with Dokploy:

```
version: "3"

services:
next-app:
build:
context: ./next-app
dockerfile: prod.Dockerfile
args:
ENV_VARIABLE: ${ENV_VARIABLE}
NEXT_PUBLIC_ENV_VARIABLE: ${NEXT_PUBLIC_ENV_VARIABLE}
restart: always
ports:
- 3000:3000
networks:
- my_network
networks:
my_network:
external: true
```

Updated version with dokploy-network and Traefik labels:

Don't set container\_name property to the each service, it will cause issues with logs, metrics and other features

```
version: "3"

services:
next-app:
build:
context: ./next-app
dockerfile: prod.Dockerfile
args:
ENV_VARIABLE: ${ENV_VARIABLE}
NEXT_PUBLIC_ENV_VARIABLE: ${NEXT_PUBLIC_ENV_VARIABLE}
restart: always
ports:
- 3000
networks:
- dokploy-network
labels:
- "traefik.enable=true"
- "traefik.http.routers.<unique-name>.rule=Host(`your-domain.com`)"
- "traefik.http.routers.<unique-name>.entrypoints=websecure"
- "traefik.http.routers.<unique-name>.tls.certResolver=letsencrypt"
- "traefik.http.services.<unique-name>.loadbalancer.server.port=3000"
networks:
dokploy-network:
external: true
```

Make sure to point the A record to the domain you want to use for your service.

Deploy the application by clicking on "deploy" and wait for the deployment to complete. Then give Traefik about 10 seconds to generate the certificates. You can then access the application through the domain you have set.

**Tips**:

1.  Set unique names for each router: `traefik.http.routers.<unique-name>`
2.  Set unique names for each service: `traefik.http.services.<unique-name>`
3.  Ensure the network is linked to the `dokploy-network`
4.  Set the entry point to websecure and the certificate resolver to letsencrypt to generate certificates.

Previous

Domains

Next

Utilities

### On this page

TutorialStepsUpdating Your `docker-compose.yml`

Title: Utilities | Dokploy

Description: Utilities for your Docker Compose application

Keywords: dokploy,vps,open source,cloud,self hosting,free

Dokploy

Core

The core of Dokploy

Search

⌘K

WebsiteDiscordSupportGithubBlog

Introduction

Welcome to DokployArchitecture of DokployFeaturesInstallationManual InstallationReset Password & 2FAUninstallVideosComparisonGoodiesTroubleshootingCloud

Dokploy CloudMonitoringServer

S3 Destinations

Git Sources

Users

Notifications

Registry

SSH KeysCertificatesBackupsServices

Environment Variables

Domains

Applications

Docker Compose

DomainsExampleUtilities

Databases

Open Source Templates

Examples

Auto DeploySchedule JobsVolume BackupsProvidersWatch PathsAdvanced

Cluster

Multi Server

On this page

Docker Compose

Utilities
=========

Utilities for your Docker Compose application

Dokploy provides a set of utilities to enhance your Docker Compose application deployment experience.

Isolated Deployments
--------------------

All open source templates come with this feature enabled by default.

This feature allows you to deploy your application in a separate network, isolated from other applications. This isolation is particularly useful when you need to deploy multiple instances of the same application.

For example, if you want to deploy two WordPress instances, you would typically encounter service naming conflicts since they share the same network (dokploy-network). Docker doesn't allow services with identical names in the same network. Consider this typical WordPress service:

```
services:
wordpress:
image: wordpress:latest
ports:
- "80"
```

When Isolated Deployments is enabled, Dokploy will:

1.  Add a prefix to all your defined networks
2.  Create a network based on your service's appName and associate it with each service in your compose file
3.  Connect the Traefik load balancer to this isolated network, maintaining service isolation while ensuring proper routing

When using this feature, you don't need to explicitly define dokploy-network in your networks section, as isolation is handled automatically.

Randomize Compose
-----------------

Dokploy offers functionality to randomize various compose properties:

1.  Volumes
2.  Networks
3.  Services
4.  Configs
5.  Secrets

You can specify a custom prefix that will be used as a suffix for each compose property.

Note: If both Isolated Deployments and Randomize Compose are enabled, the Isolated Deployments configuration takes precedence.

Previous

Example

Next

Databases

### On this page

Isolated DeploymentsRandomize Compose
