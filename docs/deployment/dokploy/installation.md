Title: Installation | Dokploy

Description: Get Dokploy up and running on your server within minutes with this easy-to-follow installation guide.

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

Installation
============

Get Dokploy up and running on your server within minutes with this easy-to-follow installation guide.

Follow these steps in order to set up Dokploy locally and deploy it to your server, effectively managing Docker containers and applications:

You need to follow this steps in the same order:

1.  Virtual Private Server (VPS)

Virtual Private Server (VPS)
----------------------------

There are multiple VPS providers to choose from:

We have tested on the following Linux Distros:

*   Ubuntu 24.04 LTS
*   Ubuntu 23.10
*   Ubuntu 22.04 LTS
*   Ubuntu 20.04 LTS
*   Ubuntu 18.04 LTS
*   Debian 12
*   Debian 11
*   Debian 10
*   Fedora 40
*   Centos 9
*   Centos 8

### Providers

*   Hostinger Get 20% Discount using this referral link: Referral Link
*   AmericanCloud Receive 20$ credits for free: Referral Link
*   Teramont Get 15% discount for free: Referral Link
*   Hetzner Get 20€ credits for free with this referral link: Referral Link
*   DigitalOcean Get 200$ credits for free with this referral link: Referral Link
*   Vultr Referral Link: Referral Link
*   Linode
*   Scaleway
*   Google Cloud
*   AWS

### Requirements

To ensure a smooth experience with Dokploy, your server should have at least 2GB of RAM and 30GB of disk space. This specification helps to handle the resources consumed by Docker during builds and prevents system freezes.

**Suggestion:** For cost efficiency with reliable service, we recommend Hetzner as the best value-for-money VPS provider.

### Docker

Dokploy utilizes Docker, so it is essential to have Docker installed on your server. If Docker is not already installed, Dokploy's installation script will install it automatically. Use the following command to install Dokploy:

Dokploy Cloud: Use Dokploy directly without worrying about maintenance or updates. Enjoy a hassle-free experience with Dokploy Cloud. Sign up

```
curl -sSL https://dokploy.com/install.sh | sh
```

Completing the Setup
--------------------

After running the installation script, Dokploy and its dependencies will be set up on your server. Here's how to finalize the setup and start using Dokploy:

### Accessing Dokploy

Open your web browser and navigate to `http://your-ip-from-your-vps:3000`. You will be directed to the initial setup page where you can configure the administrative account for Dokploy.

### Initial Configuration

1.  **Create an Admin Account:** Fill in the necessary details to set up your administrator account. This account will be the admin account for Dokploy.

Previous

Features

Next

Manual Installation

### On this page

Virtual Private Server (VPS)ProvidersRequirementsDockerCompleting the SetupAccessing DokployInitial Configuration