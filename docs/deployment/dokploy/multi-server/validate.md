# Validate | Dokploy
Validate your remote server deployment

Dokploy requires the following 7 components to be properly configured for the multi-server feature:

1.  **Docker Installed**: Docker must be installed on the remote server.
2.  **RClone Installed**: RClone must be installed on the remote server.
3.  **Nixpacks Installed**: Nixpacks must be installed on the remote server.
4.  **Buildpacks Installed**: Buildpacks must be installed on the remote server.
5.  **Docker Swarm Initialized**: Docker Swarm must be initialized on the remote server.
6.  **Dokploy Network Created**: A Docker network for Dokploy must be created on the remote server.
7.  **Main Directory Created**: A directory must be created on the remote server to store applications.

Once all requirements are met, you will see a green checkmark next to each item in the validation section.

![Multi-Server Setup](https://docs.dokploy.com/_next/image?url=%2Fassets%2Fimages%2Fserver-validate.png&w=3840&q=75)