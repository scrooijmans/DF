Environment variables not being set in container #385
Closed
Closed
Environment variables not being set in container
#385
@andrepcg
Description
andrepcg
opened on Aug 27, 2024
To Reproduce
Create app
Set environment variables in UI
Open terminal in container
printenv
Confirm variables are not set
Current vs. Expected behavior
Env Variables set in the UI should be set within the container but they're not

Provide environment information
Dokploy Version: v0.7.2
VPS provider: AWS
OS: Debian
Which area(s) are affected? (Select all that apply)
Installation

Additional context
No response

Activity

andrepcg
added 
bug
Something isn't working
 on Aug 27, 2024
Siumauricio
Siumauricio commented on Aug 27, 2024
Siumauricio
on Aug 27, 2024 · edited by Siumauricio
Contributor
Hi @andrepcg, First you need to deploy the application in order to see the enviroments in the terminal, I see one of my projects and the enviroments are visible

image

andrepcg
andrepcg commented on Aug 27, 2024
andrepcg
on Aug 27, 2024
Author
Even after a successful deploy, my ends are still not there. I'm doing a docker composer deploy.

This also raises the question: my container required the env vars to be there before deploying, since it starts an app that needs those vars. I hardcoded those but even after passing a dummy env var in the UI and redeploying, the variables are still not there

Siumauricio
Siumauricio commented on Aug 27, 2024
Siumauricio
on Aug 27, 2024
Contributor
can you share the docker compose file?

Siumauricio
Siumauricio commented on Aug 28, 2024
Siumauricio
on Aug 28, 2024 · edited by Siumauricio
Contributor
I tried to deploy the umami template which have a couple enviroment variables and you can see all the values, this is a docker compose application
Screenshot 2024-08-27 at 3 59 36 PM
The env tab
Screenshot 2024-08-27 at 4 01 01 PM

andrepcg
andrepcg commented on Aug 28, 2024
andrepcg
on Aug 28, 2024
Author
can you share the docker compose file?

services:
  atlantis:
    image: sqill/atlantis-custom
    restart: unless-stopped
    build:
      context: .
      dockerfile: Dockerfile
    command: server --write-git-creds --repo-config=/atlantis/src/repos.yaml
    expose:
      - 4141
Dockerfile

FROM ghcr.io/runatlantis/atlantis:latest

ENV TERRAGRUNT_VERSION="v0.66.3"
ENV DEFAULT_TERRAFORM_VERSION=1.9.4

# copy a terraform binary of the version you need
USER root

# Install terragrunt
RUN curl -o /usr/local/bin/terragrunt -fsSL "https://github.com/gruntwork-io/terragrunt/releases/download/${TERRAGRUNT_VERSION}/terragrunt_linux_amd64" \
    && chmod +x /usr/local/bin/terragrunt

COPY repos.yaml /atlantis/src/repos.yaml
Siumauricio
Siumauricio commented on Aug 28, 2024
Siumauricio
on Aug 28, 2024
Contributor
Internally when we do a deployment we create an .env file where the docker compose.yml file is located, I think you are missing something, maybe you can try to use

env_file:
  - path: .env
Siumauricio
Siumauricio commented on Aug 28, 2024
Siumauricio
on Aug 28, 2024
Contributor
Take a look this example: https://github.com/Dokploy/docker-compose-test/blob/main/docker-compose.yml which is very similar to your usecase

andrepcg
andrepcg commented on Aug 28, 2024
andrepcg
on Aug 28, 2024
Author
Internally when we do a deployment we create an .env file where the docker compose.yml file is located, I think you are missing something, maybe you can try to use

env_file:
  - path: .env
Adding this fixed it. Now variables are appearing in the container. But I wouldn't think I would have to add that to my composer file in order for the vars to show up..


Siumauricio
added 
question
Further information is requested
 and removed 
bug
Something isn't working
 on Aug 28, 2024
Siumauricio
Siumauricio commented on Aug 28, 2024
Siumauricio
on Aug 28, 2024
Contributor
Thanks, If you have any question or anything else feel free to open a new issue.


Siumauricio
closed this as completedon Aug 28, 2024
andrepcg
andrepcg commented on Aug 28, 2024
andrepcg
on Aug 28, 2024 · edited by andrepcg
Author
Thanks, If you have any question or anything else feel free to open a new issue.

Well, I'm not sure my problem is fixed because nowhere it's mentioned that I have to set a env_file: - path: .env for variables to be recognized. Is something wrong with my setup, dockerfile, composer file (here)?

I feel the fix was a hack and something else is going on

Siumauricio
Siumauricio commented on Aug 28, 2024
Siumauricio
on Aug 28, 2024
Contributor
You have 2 ways, the first one you can specify the environment variables manually in the ENVIROMENT section or else using the way I shared with you, again this is not a dokploy bug.

I don't know if you reviewed the example I shared with you? is very similar what you have

https://docs.docker.com/compose/environment-variables/envvars-precedence/

andrepcg
andrepcg commented on Aug 28, 2024
andrepcg
on Aug 28, 2024 · edited by andrepcg
Author
I see. Then I think the UI is missing some sort of clarification because I assume whatever I set on that form is going to be passed into the container. There's no mentions to - path: .env anywhere, not even in the that example you linked. That example shows how to use ARGS but I don't need ARGS passed to the docker file to build the image, I need envs passed directly to the container. Maybe the docs could also include this information

Siumauricio
Siumauricio commented on Aug 28, 2024
Siumauricio
on Aug 28, 2024
Contributor
Create a repository to test this, but again I think this is how docker compose works, we don't implement or modify the behavior of anything.

andrepcg
andrepcg commented on Aug 28, 2024
andrepcg
on Aug 28, 2024 · edited by andrepcg
Author
Create a repository to test this, but again I think this is how docker compose works, we don't implement or modify the behavior of anything.

That's true. But the tool adds stuff to the docker-composer yaml file to reflect chosen options (such as traefik labels). Maybe it could also generate that that - path: .env thingy if the user sets env vars.

But again, thanks a lot for your help

Siumauricio
Siumauricio commented on Aug 28, 2024
Siumauricio
on Aug 28, 2024
Contributor
Yeah we add traefik labels only when you add a domain, in your case where seems you didn't added a domain we will not add anything

staurostriantafyllos
staurostriantafyllos commented on Mar 8
staurostriantafyllos
on Mar 8
I know this issue is closed and a bit old, but I think it’s still an important one.

I totally agree with @andrepcg —once there’s some “magic” in how env vars are exported from the form to a file, that same “magic” should handle them automatically in the auto-generated docker-compose.yml.

By the way, I wouldn’t want to update my docker-compose file every time I add a new env var. Also, using:

args:
  ENV_VARIABLE: ${ENV_VARIABLE}
  NEXT_PUBLIC_ENV_VARIABLE: ${NEXT_PUBLIC_ENV_VARIABLE}
feels a bit hacky.

Would love to hear thoughts on making this smoother.

MuhammadM1998
MuhammadM1998 commented on Jul 28
MuhammadM1998
on Jul 28 · edited by MuhammadM1998
Contributor
Spent the whole day debugging before stumbling over this issue.

You have 2 ways, the first one you can specify the environment variables manually in the "Environment" section, or else using the way I shared with you, again this is not a dokploy bug.

The first sentence implies that setting vars in the environment tab will get them populated inside the container (which is the default by Docker Compose), but this is not the case.

Here I'm specifying my vars in the Environment tab. Opening a container and running env doesn't show my vars. I'm overriding my reverb server port, for example, but the container still starts at the default port for the same reason.

Image
Using this workaround will work only in production but crash locally as we don't have .env files in the root, but inside their corresponding folders in a monorepo

env_file:
  - path: .env
We would've worked around this by adding COMPOSE_ENV_FILES=.env to the custom command in the "Advanced" tab, but unfortunately, it prepends 'docker' to any command, so we can't do that.

I agree with @andrepcg and @staurostriantafyllos, the current env handling is not ideal and can be more intuitive if Dokploy automatically adds the env_file line as it does with Traefik labels.

Or, as a better escape hatch than the above, add a button in the Environment tab to toggle adding the above line and set it to disabled by default for backward compatibility

*******************
is there a way to reload environment variables without rebuilding
title
11 Replies

Henrik
•
7mo ago
Nope. They have to be rebuilt/redeployed. If you have a lengthy build process, you should build the image and push it to a registry instead. That makes it faster to switch to a new one

Sr Izan
OP
•
7mo ago
thank you!

Vishal
•
7mo ago
might be possible if we use .env then you can stop and start your container. try if this works
    env_file:
      - .env

Henrik
•
7mo ago
No it does not work. You have to do docker compose up command for that to take effect. Start and stop will not change the variable

Vishal
•
7mo ago
ok then i might be wrong

Henrik
•
7mo ago
I was a bit unsure about that case, so I just tested it
But env file isn't something that would work with Dokploy? Where should we put it?

Vishal
•
7mo ago
I have see few templates using env_file for example plausible
so i thought if thats added then there must be some usecase within dokploy
you should put that in the raw compose

Henrik
•
7mo ago
You're absolutly rigth. I took a look at the source and indeed it builds a .env file. I didn't know that.

Vishal
•
7mo ago
here's a test file which i found might be useful
https://github.com/Dokploy/dokploy/blob/0a6382a731a901e57830235a27b8fe00426e9333/apps/dokploy/__test__/compose/compose.test.ts#L411

Henrik
•
7mo ago
That will be quite usefull for a few projects. Learn something new every day

Vishal
•
7mo ago
yeah, even i do explore and learning slowly

*********
Reload environment variables upon "Reload" #2547
Open
Open
Reload environment variables upon "Reload"
#2547
@robgraeber
Description
robgraeber
opened 5 days ago
Contributor
What problem will this feature address?
It seems like environment variable changes don't get picked up unless you "Rebuild". Not sure if this a bug or not, but I'd expect "Reload" to be enough.

Describe the solution you'd like
"Reload" will update the environment variables. Alternatively could improve the documentation, e.g. add the disclaimer "Please remember to click 'Reload' after adding, editing, or deleting an environment variable to apply the changes."

Describe alternatives you've considered
Clicking "Rebuild" works.

Additional context
No response

Will you send a PR to implement it?
Yes

Activity

robgraeber
added 
enhancement
New feature or request
 5 days ago
djsisson
djsisson commented 5 days ago
djsisson
5 days ago
@robgraeber env are part of the container hash, the container needs to be rebuilt in order to change them

robgraeber
robgraeber commented 5 days ago
robgraeber
5 days ago
Contributor
Author
@djsisson interesting, I still think this is confusing and at least deserves a notice. Generally other PaaS like Heroku, CapRover, etc only need a restart for environment variable changes.

djsisson
djsisson commented 5 days ago
djsisson
5 days ago
they are probably seperating build and deploy, i.e they build to an image, then it's much simpler to change env, since the image is the same. (unless of course this env is a build argument)

if you are using swarm where you are first building to an image in a registry, and only deploy an image then it's as simple as

docker service update --env-add XXXX=NEWVALUE <service>


