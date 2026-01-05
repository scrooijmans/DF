# Going Production | Dokploy
Learn how to deploy your application in production in Dokploy.

By default, dokploy offer multiple [Builds Types](https://docs.dokploy.com/docs/core/applications/build-type) to deploy your application, the most common is `nixpacks` and `heroku buildpacks` however this also comes with problems, first is the resources that are required to build your application which some times can lead to timeout on your server or even freezeing your server and all your application will be down for this reasson, this is mainly problem from `Docker` since the comsumption of resources such as RAM, CPU is very high to build an application.

You have two options to solve this problem:

1.  Increase the resources of your server CPU, RAM, Disk (Probably is not a good idea and cheapest solution)
2.  Build & Publish the application in a CI/CD pipeline eg. Github Actions, Gitlab CI, Gitea Actions, etc. (Recommended)

### [Build & Publish the application in a CI/CD pipeline](#build--publish-the-application-in-a-cicd-pipeline)

We will use Github Actions as an example, but you can use any CI/CD pipeline that you want.

We will use the following configuration:

1.  **Use Git Provider in Your Application**:
    *   Repository: `https://github.com/Dokploy/production-example`
    *   Branch: `main`
    *   Build path: `/`

The repo have everything you need, however you can follow the same idea for your own applications.

3.  The repository already have a Dockerfile, so we will use that, in the case your application is different create your own Dockerfile is required for this guide.
4.  We will use `Dockerhub` as an example, but you can use any container registry that you want.
5.  Make sure to create the repository in the `Dockerhub` , `namespace` is your username and `repository` is `example`.
6.  Create a new Github Actions workflow in `.github/workflows/deploy.yml`
7.  Add the following code to the workflow:

8.  Create your own Dockerfile, in this case we will use the `Dockerfile` from the repository.

9.  Now when you make a commit to your repository, the workflow will be triggered and the application will build and push to `Dockerhub`.
10.  Now let's create application in Dokploy.
11.  In `Source Type` select `Docker`
12.  In the docker image field enter `namespace/example:latest`
13.  Click on `Save`.
14.  Click on `Deploy`.
15.  Go to `Domains` and click `Dices` icon to generate a domain and the port set to `3000`.
16.  Now you can access your application.

### [Auto deploy](#auto-deploy)

When using Dockerhub as a registry you can also enable auto deploy, this will automatically deploy your application whenever you push to your repository.

To setup auto deploys for Dockerhub, follow the steps below:

1.  Go to your application and select `Deployments` tab.
2.  Copy the `Webhook URL`.
3.  Go to your Dockerhub repository and select `Webhooks` tab.
4.  Set a name for the webhook and paste the `Webhook URL` copied in step 2.
5.  That's it, now every time you push to your repository, your application will trigger a deployment in dokploy.

The deployment will trigger only if the `Tag` matches the one specified in Dokploy.

#### [External Registry](#external-registry)

If you have a registry that is not Dockerhub, you can trigger a deployment after pushing to your repository in Github Actions.

Your workflow will look like this:

This method use the [Api Method](about:/docs/core/auto-deploy#api-method) to trigger a deployment.

You can also use this Github Action [Action](https://github.com/marketplace/actions/dokploy-deployment) to automate the deployment.

When using Dokploy you can also configure healthchecks and rollbacks, this will allow you to configure your application to be able to recover from failures.

In the repo we are using from the `Step 1.` we have a healthcheck endpoint `/health` that returns a 200 status code and running in the port 3000.

Go to `Advanced` Tab and go to Cluster Settings and enter to `Swarm Settings`

There are a couple options that you can use, in this case we will focus on `Health Check` and `Update Config`.

Make sure the API Route exists in your application

Now in the `Update Config`

Now when the application is getting unhealthy response from the health check, the container will rollback to the previous version.

Paste the following code:

Now you everything a production ready application with automated deployments, zero downtime, rollbacks and healthchecks.

We recommend strongly to use this approach in production since this will make your server never build the application, will only in charge of the deployment keeping your server without any downtime.