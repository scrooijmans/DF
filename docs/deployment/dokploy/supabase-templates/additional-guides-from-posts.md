Issue:
I have deployed a Dokploy template for Supabase. But the Storage is not loading. On console it shows 500 error. I tried adding domains for the services, still no luck.

Solution:
I managed to fix this. What you need is just set your environemnt variables and change the SUPABASE_HOST with your domain you added on DOMAINs tab of dokploy. You may also add additional_domains there. Next in the general tab, set set the compose file and set SUPABASE_PUBLIC_URL, API_EXTERNAL_URL, GOTRUE_SITE_URL and any URL using with SSL with https:{variable}. Save. Next important thing, go to Environment tab again and copy the JWT_SECRET and go to https://supabase.com/docs/guides/self-hosting/docker#updating-your-services and scroll to generate section, paste your JWT and select service key and then generate a new service_key. Now paste that key into your envirnoment and save. Redeploy or restart. Now, it will work again. Hope it helped.




Issue:
To Reproduce
Create a project
create a template service
choose supabase
see the error mentioned below
Current vs. Expected behavior
it should pull all the images and run normally, instead im getting this in the logs:

Initializing deployment
Creating File 'docker-compose.yml' to /etc/dokploy/compose/dd-supabase-9qcbed/code: ✅
File 'docker-compose.yml' created: ✅
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║ App Name: dd-supabase-9qcbed                                                 ║
║ Build Compose 🐳                                                             ║
║ Detected: 1 mounts 📂                                                        ║
║ Command: docker compose -p dd-supabase-9qcbed -f docker-compose.yml up -d    ║
║ --build --remove-orphans                                                     ║
║ Source Type: docker raw ✅                                                   ║
║ Compose Type: docker-compose ✅                                              ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
time="2025-03-12T06:52:05Z" level=warning msg="/etc/dokploy/compose/dd-supabase-9qcbed/code/docker-compose.yml: the attribute `version` is obsolete, it will be ignored, please remove it to avoid potential confusion"
Volume "dd-supabase-9qcbed_db-config-dd-supabase-9qcbed"  Creating
Volume "dd-supabase-9qcbed_db-config-dd-supabase-9qcbed"  Created
Container supabase-vector  Creating
Container supabase-imgproxy  Creating
Container supabase-imgproxy  Created
Container supabase-vector  Created
Container supabase-db  Creating
Container supabase-db  Created
Container supabase-analytics  Creating
Container supabase-analytics  Created
Container supabase-kong  Creating
Container supabase-studio  Creating
Container supabase-auth  Creating
Container supabase-edge-functions  Creating
Container realtime-dev.supabase-realtime  Creating
Container supabase-rest  Creating
Container supabase-meta  Creating
Container supabase-studio  Created
Container supabase-kong  Created
Container supabase-rest  Created
Container supabase-storage  Creating
Container supabase-auth  Created
Container supabase-edge-functions  Created
Container realtime-dev.supabase-realtime  Created
Container supabase-meta  Created
Container supabase-storage  Created
Container supabase-imgproxy  Starting
Container supabase-vector  Starting
Container supabase-vector  Started
Container supabase-vector  Waiting
Container supabase-imgproxy  Started
Container supabase-vector  Error
dependency failed to start: container supabase-vector exited (78)
Error ❌ time="2025-03-12T06:52:05Z" level=warning msg="/etc/dokploy/compose/dd-supabase-9qcbed/code/docker-compose.yml: the attribute `version` is obsolete, it will be ignored, please remove it to avoid potential confusion"
Volume "dd-supabase-9qcbed_db-config-dd-supabase-9qcbed"  Creating
Volume "dd-supabase-9qcbed_db-config-dd-supabase-9qcbed"  Created
Container supabase-vector  Creating
Container supabase-imgproxy  Creating
Container supabase-imgproxy  Created
Container supabase-vector  Created
Container supabase-db  Creating
Container supabase-db  Created
Container supabase-analytics  Creating
Container supabase-analytics  Created
Container supabase-kong  Creating
Container supabase-studio  Creating
Container supabase-auth  Creating
Container supabase-edge-functions  Creating
Container realtime-dev.supabase-realtime  Creating
Container supabase-rest  Creating
Container supabase-meta  Creating
Container supabase-studio  Created
Container supabase-kong  Created
Container supabase-rest  Created
Container supabase-storage  Creating
Container supabase-auth  Created
Container supabase-edge-functions  Created
Container realtime-dev.supabase-realtime  Created
Container supabase-meta  Created
Container supabase-storage  Created
Container supabase-imgproxy  Starting
Container supabase-vector  Starting
Container supabase-vector  Started
Container supabase-vector  Waiting
Container supabase-imgproxy  Started
Container supabase-vector  Error
dependency failed to start: container supabase-vector exited (78)
Answer:
the solution is just fork the supabase repo, and run the compose .. the template from dokploy is not working

********************************************************************************************************************************************************************