

I confirm this is a bug with Supabase, not with my own application.

I confirm I have searched the Docs, GitHub Discussions, and Discord.
Describe the bug
I have followed all setup (here) and I keep having the follwing error after login :
Kong Error Invalid authentication credentials.

To Reproduce
Steps to reproduce the behavior, please provide code snippets or a repository:

Go to my proxy URL 'https://supabase..com/' (in .env is the value of SITE_URL)
Enter credentials (DASHBAOARD_USERNAME and DASHBOARD_PASSWORD)
See error Kong Error Invalid authentication credentials.
Expected behavior
Access to the Dashboard.

Screenshots
If applicable, add screens
supabase1
supabase2
hots to help explain your problem.

System information
OS:
PRETTY_NAME="Ubuntu 23.10"
NAME="Ubuntu"
VERSION_ID="23.10"
VERSION="23.10 (Mantic Minotaur)"
VERSION_CODENAME=mantic
ID=ubuntu
ID_LIKE=debian
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
UBUNTU_CODENAME=mantic
LOGO=ubuntu-logo

Browser Chrome, Chrome private, Firefox, Firefox private

Version of supabase-js: latest

Additional context
Here is the docker compose ps result :
NAME                             IMAGE                              COMMAND                                                                                                                                                 SERVICE     CREATED       STATUS                      PORTS realtime-dev.supabase-realtime   supabase/realtime:v2.25.35         "/usr/bin/tini -s -g -- /app/limits.sh sh -c '/app/bin/migrate && /app/bin/realtime eval 'Realtime.Release.seeds(Realtime.Repo)' && /app/bin/server'"   realtime    4 hours ago   Up 17 minutes (healthy) supabase-analytics               supabase/logflare:1.4.0            "sh run.sh"                                                                                                                                             analytics   4 hours ago   Up 17 minutes (healthy)     0.0.0.0:4000->4000/tcp, :::4000->4000/tcp supabase-auth                    supabase/gotrue:v2.125.1           "gotrue"                                                                                                                                                auth        4 hours ago   Up 17 minutes (healthy) supabase-db                      supabase/postgres:15.1.0.117       "docker-entrypoint.sh postgres -c config_file=/etc/postgresql/postgresql.conf -c log_min_messages=fatal"                                                db          4 hours ago   Up 17 minutes (healthy)     0.0.0.0:5432->5432/tcp, :::5432->5432/tcp supabase-edge-functions          supabase/edge-runtime:v1.29.1      "edge-runtime start --main-service /home/deno/functions/main"                                                                                           functions   4 hours ago   Up 17 minutes supabase-kong                    kong:2.8.1                         "bash -c 'eval \"echo \\\"$(cat ~/temp.yml)\\\"\" > ~/kong.yml && /docker-entrypoint.sh kong docker-start'"                                             kong        4 hours ago   Up 17 minutes (healthy)     0.0.0.0:8000->8000/tcp, :::8000->8000/tcp, 8001/tcp, 0.0.0.0:8443->8443/tcp, :::8443->8443/tcp, 8444/tcp supabase-meta                    supabase/postgres-meta:v0.75.0     "docker-entrypoint.sh npm run start"                                                                                                                    meta        4 hours ago   Up 17 minutes (healthy)     8080/tcp supabase-rest                    postgrest/postgrest:v11.2.2        "postgrest"                                                                                                                                             rest        4 hours ago   Up 17 minutes               3000/tcp supabase-storage                 supabase/storage-api:v0.43.11      "docker-entrypoint.sh node dist/server.js"                                                                                                              storage     4 hours ago   Up 17 minutes (healthy)     5000/tcp supabase-studio                  supabase/studio:20231123-64a766a   "docker-entrypoint.sh node apps/studio/server.js"                                                                                                       studio      4 hours ago   Up 17 minutes (unhealthy)   3000/tcp supabase-vector                  timberio/vector:0.28.1-alpine      "/usr/local/bin/vector --config etc/vector/vector.yml"                                                                                                  vector      4 hours ago   Up 17 minutes (healthy) 

Activity

tagNC
added 
bug
Something isn't working
 on Dec 14, 2023

saltcod
added 
auth
All thing Supabase Auth related
 on Dec 14, 2023
tagNC
tagNC commented on Dec 20, 2023
tagNC
on Dec 20, 2023
Author
I could notice that as soon as I try to update DASHBOARD_USERNAME and DASHBOARD_PASSWORD in .env file, the error is happening.

tagNC
tagNC commented on Dec 21, 2023
tagNC
on Dec 21, 2023
Author
Changing credentials in the kong.yml and let credentials blank in .env is make it work.


tagNC
closed this as completedon Dec 21, 2023
zj1123581321
zj1123581321 commented on Jun 25, 2024
zj1123581321
on Jun 25, 2024
Changing credentials in the kong.yml and let credentials blank in .env is make it work.

You mean:
volumes/api/kong.yml should change to this?

consumers:
  - username: DASHBOARD
  - username: anon
    keyauth_credentials:
      - key: <your_generated_anon_key>
  - username: service_role
    keyauth_credentials:
      - key: <your_generated_service_role_key>
and let .env change to this:

############
# Secrets
# YOU MUST CHANGE THESE BEFORE GOING INTO PRODUCTION
############

POSTGRES_PASSWORD=your-super-secret-and-long-postgres-password
JWT_SECRET=your-super-secret-jwt-token-with-at-least-32-characters-long
ANON_KEY=
SERVICE_ROLE_KEY=
DASHBOARD_USERNAME=supabase
DASHBOARD_PASSWORD=this_password_is_insecure_and_should_be_updated
I'm not certain if I've grasped it correctly. I'm facing the same problem and haven't resolved it yet.

looking forward to your response and greatly appreciate it!

Stokestack
Stokestack commented on Jul 10, 2024
Stokestack
on Jul 10, 2024
The problem I have is that there's no way to clear this error. I actually did enter wrong credentials, but the page offers no way to try again, and clearing browser data associated with localhost doesn't clear it.

So now there's no way to access the dashboard.

Glinte
Glinte commented on Aug 5, 2024
Glinte
on Aug 5, 2024
Seems like you can get around it by putting the login credentials in the url: https://stackoverflow.com/questions/50528467/how-to-add-login-credentials-to-url
i.e. http://username:password@127.0.0.1:8000/

Stokestack
Stokestack commented on Aug 5, 2024
Stokestack
on Aug 5, 2024
Thanks for the temporary workaround.

Why was this closed as completed? What was changed?

Khaled-Mo99
Khaled-Mo99 commented on Nov 14, 2024
Khaled-Mo99
on Nov 14, 2024 · edited by Khaled-Mo99
I'm still facing the same issue, and even the workaround isn't working for me.

Stokestack
Stokestack commented on Nov 15, 2024
Stokestack
on Nov 15, 2024
Other people continue to encounter this problem

qxygene1
qxygene1 commented on Nov 20, 2024
qxygene1
on Nov 20, 2024 · edited by qxygene1
just change password and leave username as supabase

Stokestack
Stokestack commented on Nov 21, 2024
Stokestack
on Nov 21, 2024
You can't, because the log-in fields aren't presented anymore.

qxygene1
qxygene1 commented on Nov 21, 2024
qxygene1
on Nov 21, 2024
You can't, because the log-in fields aren't presented anymore.

worked for me

Stokestack
Stokestack commented on Nov 21, 2024
Stokestack
on Nov 21, 2024 · edited by Stokestack
Well, not for others. Hence this bug report. Look at the screen shot. Is there any place to enter anything?

freedomanonyme
freedomanonyme commented on Dec 5, 2024
freedomanonyme
on Dec 5, 2024 · edited by freedomanonyme
Well, this is a false start for me with Supabase. I don’t have the time to deal with this kind of issue. UPDATE: Gave it another try. The issue occurs when I change the credentials. I tried adding the credentials in the Kong file rather than the .env file, but the same problem persists. What junk! ...an we are 1 year later!

c-o-l-i-n
c-o-l-i-n commented on Jan 3
c-o-l-i-n
on Jan 3
This workaround worked for me:

Open the Supabase dashboard with the username and password in the URL.
This takes you to the dashboard, but none of the data loads properly.
eg: https://username:password@your-supabase-dashboard-domain.com

Close that tab, and then open a new tab without the username and password.
Now the data should load!
eg: https://your-supabase-dashboard-domain.com

djbouti
djbouti commented on Feb 4
djbouti
on Feb 4
Ran into this github pages, I have to tell you guys to chill. It is simple authentication, like, really simple that it read the cookies first. If your cookies are 'invalid password' cookies, then it will always show invalid authentication. WHY? Because it read the cookies before taking in your input (password).

Clear the cookies for the supabase (usually localhost:8000), close the browser, reopen the browser and go back to your supabase (usually localhost:8000) and this time make sure to input it correctly.

You are welcome.

bastien-naturavelo
bastien-naturavelo commented on Feb 12
bastien-naturavelo
on Feb 12 · edited by bastien-naturavelo
Well, it's pretty random, on a computer i'm logged in with one browser, on a different browser it never showed me the popup to login and there's no cookie attached, it doesn't work either on private browsing.
And on another computer, i get the login but the credentials are not working anymore. It's disturbing.
i had to update ./docker/volumes/api/kong.yml and restart to have a credential working
https://supabase.com/docs/guides/self-hosting/docker#dashboard-authentication

amgawishx
amgawishx commented on Feb 14
amgawishx
on Feb 14 · edited by amgawishx
This issue still occurs, why was it closed? why don't they fix it? why the work arounds? there is obviously something wrong here.

jljl1337
jljl1337 commented on Mar 9
jljl1337
on Mar 9
It might not be directly relevant, but does it make more sense to expose the dashboard on a different port? Anyone with the public url can try to login to the dashboard.

shizuo-x
shizuo-x commented on Apr 8
shizuo-x
on Apr 8
This workaround worked for me:

1. Open the Supabase dashboard **with the username and password in the URL**.
   This takes you to the dashboard, but none of the data loads properly.
   _eg: https://username:password@your-supabase-dashboard-domain.com_

2. Close that tab, and then open a _new_ tab **without** the username and password.
   Now the data should load!
   _eg: https://your-supabase-dashboard-domain.com_
works, I swear man such a bs issue, I respect everyone but imagine a noob like me trying to do something in life only to get to this seemingly, easily, fixable problem; but waste hours to troubleshoot.

lucca-rodrigues
lucca-rodrigues commented on May 1
lucca-rodrigues
on May 1
Apparently this happens if you are already logged in with the old credentials (before changing)

I had the same problem, and when accessing in an incognito tab I was able to log in normally with the new credentials

infiniteAppsUG
infiniteAppsUG commented on May 13
infiniteAppsUG
on May 13
Apparently this happens if you are already logged in with the old credentials (before changing)

I had the same problem, and when accessing in an incognito tab I was able to log in normally with the new credentials

THIS IS THE ISSUE. LAYER 8 :D thanks for the hint. Worked out of the box

cjfagerstrom
cjfagerstrom commented on May 29
cjfagerstrom
on May 29 · edited by cjfagerstrom
I had to remove special characters from my user/pass as none of the above worked. I then ran docker compose down -v and docker compose up -d. I had already tried this command just prior to changing the user/pass and after removing the following from user: @ - _ and the same plus a * from pass, the issue was resolved. Special char not being escaped??

JamesMowery
JamesMowery commented on Jul 9
JamesMowery
on Jul 9 · edited by JamesMowery
This is still a mess. I have cleared cookies, restarted browser, and I can't get it to allow me on the login page, just because I screwed up typing in the credentials one single time. Please fix this!

Or, better yet, why not have an actual normal login screen?

