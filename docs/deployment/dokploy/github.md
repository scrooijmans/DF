Title: GitHub | Dokploy

Description: Configure GitHub repositories for deployments. This includes setting up access tokens, repository names, and branches.

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

BitbucketDocker RegistryGiteaGitHubGitlab

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

Git Sources

GitHub
======

Configure GitHub repositories for deployments. This includes setting up access tokens, repository names, and branches.

Dokploy offer a way to connect your Github Repository to your Dokploy panel, you can use organizations or personal accounts.

Go to `Git` and select `Github` as the source, then you can use the following options:

*   **Organization**: Select the organization that you want to connect to Dokploy.
*   **Personal Account(Default)**: Select the account that you want to connect to Dokploy.

Follow the steps to connect your Github account to Dokploy.

1.  Click on `Create Github App` to create a new Github App.
2.  Set Github App Name: eg. `Dokploy-Github-App`. make sure this name is unique.
3.  Click on `Create Github App`, then you will redirect to the `Git` section of Dokploy.
4.  Now it will show a `Install` Button, click on it.
5.  You can select the repositories that you want to dokploy be able to access, you can choose select all repositories or select specific repositories.
6.  Click on `Install & Authorize` to install the Dokploy App.
7.  You will be redirected to the `Git` section of Dokploy.
8.  Now you can use the repositories from your Github Account in `Applications` or `Docker Compose` services.

When you use this method, By default you will have Automatic deployments on each push you make to your repository.

Clarification on Automatic Deployments
--------------------------------------

By default, Dokploy will automatically deploy your application on the Branch you have selected.

eg. Let's suppose you have a `application` in this way:

Repository: `my-app` Branch: `feature`

If you try to make a push on another branch eg. `main`, Dokploy will not automatically deploy your application, because your application have selected `feature` as the Branch.

In the case you want to have multiple applications in the same repository, eg. (development, staging, production), you can create 3 `Applications` in Dokploy and select the branch in each of them.

This is very usefull if you want to have multiple environments for the same application.

Previous

Gitea

Next

Gitlab

### On this page

Clarification on Automatic Deployments