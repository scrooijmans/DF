Title: Coolify - Hetzner Docs

Coolify
=======

Last change on 2025-08-04 • Created on 2024-11-05 • ID: CL-610D9

Coolify turns your server into an open-source & self-hostable Heroku / Netlify / Vercel alternative.

You can install Coolify via the Hetzner Console or the Hetzner Cloud API.

Getting Started
---------------

Create your server as usual using the Hetzner Console. As an alternative to the operating system, you can choose an app that you would like to have pre-installed.

Coolify will then be preinstalled on the server, but it will not yet be activated.

To activate Coolify, please log in to your server:

*   Use an _SSH key_ if you were provided one when you created your server.
*   Use the _root-password_ which we sent to you via email when you created your server; use this if you did not get an SSH key.

This will activate Coolify and display the URL of the administration interface.

Hetzner Cloud API
-----------------

Instead of using the Hetzner Console, you can use the Hetzner Cloud API to set up a server with Coolify.

*   For example, via a curl command from the command line:

```text
curl \
-X POST \
-H "Authorization: Bearer $API_TOKEN" \
-H "Content-Type: application/json" \
-d '{"name":"my-coolify-server", "server_type":"cpx11", "image":"coolify"}' \
'https://api.hetzner.cloud/v1/servers'
```

*   Or via hcloud-cli

```text
hcloud server create --name my-coolify-server --type cpx11 --image coolify
```

Image content
-------------

### Operating system

*    Ubuntu 24.04

### Installed packages

This image contains Docker and all other listed applications as Docker containers.

| NAME | LICENSE |
| --- | --- |
| Docker | GPLv3 (Apache 2.0) |
| Coolify | AGPLv3 |

Links
-----

For more information about the installed packages, see the official documentation:

*   Docker
*   Coolify

For more information about Hetzner Cloud and Hetzner Cloud Apps, please see our official documentation:

*   Hetzner Cloud Documentation
*   Hetzner Cloud API

###### Table of Contents