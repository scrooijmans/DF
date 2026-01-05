Title: Environment Variables | Dokploy

Description: Dokploy allows you to create and manage shared environment variables for your projects.

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

Databases

Open Source Templates

Examples

Auto DeploySchedule JobsVolume BackupsProvidersWatch PathsAdvanced

Cluster

Multi Server

On this page

Environment Variables
=====================

Dokploy allows you to create and manage shared environment variables for your projects.

Overview
--------

Shared environment variables allow you to:

*   Define values once and use them across multiple services
*   Maintain configuration consistency between services
*   Centrally manage sensitive information

Practical Example
-----------------

Let's consider a common scenario where you have:

*   A PostgreSQL database
*   Two services that need to connect to this database

### 1\. Define Shared Variable

In the project's shared variables section, define:

```
DATABASE_URL=postgresql://postgres:postgres@database:5432/postgres
```

### 2\. Use the Variable in Services

In each service's environment variables tab, reference the shared variable:

```
DATABASE_URL=${{project.DATABASE_URL}}
```

Dokploy will automatically replace `${{project.DATABASE_URL}}` with the value defined in the project's shared variables.

Best Practices
--------------

*   Use shared variables for credentials and configurations that repeat across services
*   Keep descriptive variable names
*   Document the purpose of each variable for easier maintenance

You can use shared enviroment variables in all the services available in dokploy.

Previous

Backups

Next

Domains

### On this page

OverviewPractical Example1\. Define Shared Variable2\. Use the Variable in ServicesBest Practices