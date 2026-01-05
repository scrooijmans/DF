Title: Instructions | Dokploy

Description: Example to setup a remote server and deploy application in a VPS.

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

DeploymentsInstructionsSecurityValidate

On this page

Multi Server

Instructions
============

Example to setup a remote server and deploy application in a VPS.

Multi server allows you to deploy your apps remotely to different servers without needing to build and run them where the Dokploy UI is installed.

Requirements
------------

1.  To install Dokploy UI, follow the installation guide.

2.  Create an SSH key by going to `/dashboard/settings/ssh-keys` and add a new key. Be sure to copy the public key.

3.  Decide which remote server to deploy your apps on. We recommend these reliable providers:

*   Hostinger Get 20% off with this referral link.
*   DigitalOcean Get $200 credits for free with this referral link.
*   Hetzner Get €20 credits with this referral link.
*   Vultr Referral Link: Referral Link
*   Linode.
*   Scaleway.
*   Google Cloud.
*   AWS.

4.  When creating the server, it should ask for SSH keys. Ideally, use your computer's public key and the key you generated in the previous step. Here's how to add the public key in Hostinger:

The steps are similar across other providers.

5.  Copy the server’s IP address and ensure you know the username (often `root`). Fill in all fields and click `Create`.

6.  To test connectivity, open the server dropdown and click `Enter Terminal`. If everything is correct, you should be able to interact with the remote server.

7.  Click `Setup Server` to proceed. There are two tabs: SSH Keys and Deployments. This guide explains the easy way, but you can follow the manual process via the Dokploy UI if you prefer.

8.  Click `Deployments`, then `Setup Server`. If everything is correct, you should see output similar to this:

You only need to run this setup once. If Dokploy updates later, check the release notes to see if rerunning this command is required.

9.  You're ready to deploy your apps! Let's test it out:

10.  To check which server an app belongs to, you’ll see the server name at the top. If no server is selected, it defaults to `Dokploy Server`. Click `Deploy` to start building your app on the remote server. You can check the `Logs` tab to see the build process. For this example, we’ll use a test repo:  
Repo: `https://github.com/Dokploy/examples.git`  
Branch: `main`  
Build Path: `/astro`

11.  Once the build is done, go to `Domains` and create a free domain. Just click `Create` and you’re good to go! 🎊

Previous

Deployments

Next

Security

### On this page

Requirements