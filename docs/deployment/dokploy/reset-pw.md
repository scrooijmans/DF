Title: Reset Password & 2FA | Dokploy

Description: Reset your password to access your Dokploy account and disable 2FA.

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

Reset Password & 2FA
====================

Reset your password to access your Dokploy account and disable 2FA.

Reset Password
--------------

To reset your password, follow these steps:

Log in to your VPS.

Run the command below to get the container ID of the dokploy container.

```
docker ps
```

Run command below to open a shell in the dokploy container.

```
docker exec -it <container-id> bash -c "pnpm run reset-password"
```

It will display a random password. Copy it and use it to access again to the dashboard.

Reset 2FA
---------

To disable 2FA, follow these steps:

To reset your 2FA, follow these steps:

Log in to your VPS.

Run the command below to get the container ID of the dokploy container.

```
docker ps
```

Run command below to open a shell in the dokploy container.

```
docker exec -it <container-id> bash -c "pnpm run reset-2fa"
```

You can now logic again without having to supply a 2FA code.

Previous

Manual Installation

Next

Uninstall

### On this page

Reset PasswordReset 2FA