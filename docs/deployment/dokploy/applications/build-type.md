# Build Type | Dokploy
Learn about the different build types available in Dokploy, including Nixpacks, Dockerfile, and Buildpack options.

Dokploy offers three distinct build types for deploying applications, each suited to different development needs and preferences.

### [Nixpacks](#nixpacks)

This is the default build type in Dokploy. When you select Nixpacks, Dokploy builds your application as a Nixpack, which is optimized for ease of use and efficiency.

Nixpacks expose multiples variables to be configured via environment variables. All of these variables are prefixed with `NIXPACKS_`, you can define them in the `Environment Variables` tab.



* Variable: NIXPACKS_INSTALL_CMD
  * Description: Override the install command to use
* Variable: NIXPACKS_BUILD_CMD
  * Description: Override the build command to use
* Variable: NIXPACKS_START_CMD
  * Description: Override command to run when starting the container
* Variable: NIXPACKS_PKGS
  * Description: Add additional Nix packages to install
* Variable: NIXPACKS_APT_PKGS
  * Description: Add additional Apt packages to install (comma delimited)
* Variable: NIXPACKS_LIBS
  * Description: Add additional Nix libraries to make available
* Variable: NIXPACKS_INSTALL_CACHE_DIRS
  * Description: Add additional directories to cache during the install phase
* Variable: NIXPACKS_BUILD_CACHE_DIRS
  * Description: Add additional directories to cache during the build phase
* Variable: NIXPACKS_NO_CACHE
  * Description: Disable caching for the build
* Variable: NIXPACKS_CONFIG_FILE
  * Description: Location of the Nixpacks configuration file relative to the root of the app
* Variable: NIXPACKS_DEBIAN
  * Description: Enable Debian base image, used for supporting OpenSSL 1.1


If you need more manage about nixpacks process, you can create a `nixpacks.toml` file in the root of your application you can read here [Nixpacks Configuration](https://nixpacks.com/docs/configuration/file).

Nixpacks support monorepo such as NX Monorepo, Turborepo, Moon Repo, you can read more about it [here](https://nixpacks.com/docs/providers/node#build).

You can read more about Nixpacks [here](https://nixpacks.com/).

Since Nixpacks have a [static builder](https://nixpacks.com/docs/providers/staticfile) Dokploy expose a field called `Publish Directory` where basically you can specify the directory that you want to publish after the build process is finished, example:

Astro applications after you build it usually create a `dist` directory, so you can specify the `dist` directory as the publish directory and then Dokploy will copy all the files in the `dist` directory to the root of your application, and will use a NGINX Optimized Dockerfile to run your application.

### [Railpack (NEW)](#railpack-new)

Railpack is a new build type optimized and is the successor of Nixpacks.

Railpack exposes multiple Build Variables, you can define them in the `Environment Variables` tab.



* Name: BUILD_CMD
  * Description: Set the command to run for the build step. This overwrites any commands that come from providers
* Name: START_CMD
  * Description: Set the command to run when the container starts
* Name: PACKAGES
  * Description: Install additional Mise packages. In the format pkg@version. The latest version is used if not provided.
* Name: BUILD_APT_PACKAGES
  * Description: Install additional Apt packages during build
* Name: DEPLOY_APT_PACKAGES
  * Description: Install additional Apt packages in the final image


you can read more about Railpack [here](https://railpack.com/config/environment-variables).

Railpack supports Nodejs, Python, Go, PHP, Go, StaticFile, Shell Scripts.

### [Dockerfile](#dockerfile)

If your project includes a Dockerfile, you can specify its path. Dokploy will use this Dockerfile to build your application directly, giving you full control over the build environment and dependencies

Dokploy expose 3 Fields to be configured:

*   `Dockerfile Path (Required)`: The path to the Dockerfile to use for building the application, eg. If your Dockerfile is in the root of your application you can just specify the `Dockerfile` file.
*   `Docker Context Path`: This is where the Dockerfile is located, eg. If your Dockerfile is in the root of your application you can just specify the `.` (dot) character, is basically to tell docker what context will use to build your application, you can read [Dockerfile Context](https://docs.docker.com/build/concepts/context/) for more information.
*   `Docker Build Stage`: This is the build stage to use for building the application, eg. If you want to use the `builder` stage you can specify the `builder` stage, read more about build stages [here](https://docs.docker.com/build/building/multi-stage/).

### [Buildpack](#buildpack)

Dokploy supports two types of buildpacks:

*   **Heroku**: Adapted from Heroku's popular cloud platform, these buildpacks are designed for compatibility and ease of migration, you can optional specify the Heroku Version to use, by default Dokploy will use the 24.
*   **Paketo**: Provides cloud-native buildpacks that leverage modern standards and practices for building applications.

By choosing the appropriate build type, you can tailor the deployment process to best fit your application's requirements and your operational preferences.

### [Static](#static)

Static build type is used to server static applications, it will use a NGINX Optimized Dockerfile to run your application.

Dokploy will copy everything from the `Root` directory and will mount it to the `/usr/share/nginx/html` directory, and will use a NGINX Optimized Dockerfile to run your application.

Ensure to use the port `80` when creating a domain.

*   For prototyping and development purposes, we recommend using the `Nixpacks` build type.
*   For production purposes, we recommend follow this [Production Guide](https://docs.dokploy.com/docs/core/applications/going-production) to have a rock solid deployment.
*   For static applications, we recommend using the `Static` build type.