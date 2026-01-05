​
Builders Berlin Forum banner_02

Subject: Self-Hosted n8n Supabase Node Fails (401 Unauthorized) Despite Working curl/HTTP Request Node
Questions

868
views

6
likes

1
link





read
5 min
Apr 24
22d

Cyril-HBN

2
Apr 24
Hello everyone,

I’ve been struggling with an issue for several days and haven’t found a solution.

My Setup:

I have self-hosted n8n and Supabase on the same VPS.
Each runs in its own Docker containers managed via Docker Compose.
A reverse proxy (based on jwilder/nginx-proxy and jrcs/letsencrypt-nginx-proxy-companion) handles HTTPS access and subdomains.
n8n is accessible and working correctly.
Supabase Studio is accessible via its subdomain (https://supabase.cyriloup.shop).
Both n8n and the Supabase containers (including supabase-kong) are on the same Docker network (web-network).
The Problem:

I cannot connect n8n to the Supabase API using the dedicated n8n Supabase credential/node.

When setting up the Supabase credential in n8n, the connection test fails with the message: “Couldn’t connect with these settings - Unauthorized”.
The supabase-kong container logs consistently show a 401 Unauthorized error for requests originating from n8n (identified by the user agent): GET /rest/v1/ HTTP/1.1" 401 ... "-" "n8n"
Key Troubleshooting Finding:

The issue seems specific to the n8n Supabase integration, as direct API calls work:

Direct curl Test: A test using curl from the host VPS, targeting the public Supabase URL with the correct SERVICE_ROLE_KEY in the apikey header, succeeds (HTTP/2 200 OK).Bashcurl -I -H "apikey: YOUR_SERVICE_ROLE_KEY" https://supabase.cyriloup.shop/rest/v1/This confirms the API key is correct and Kong’s key-auth works for direct external requests.
n8n “HTTP Request” Node Test: Using n8n’s standard “HTTP Request” node (configured with “Header Auth” using the same apikey header and SERVICE_ROLE_KEY) to target https://supabase.cyriloup.shop/rest/v1/ also succeeds (returns Code 200 OK).
This strongly suggests the problem lies specifically with how the n8n Supabase credential type or the Supabase node handles the connection/authentication.

Checks and Attempts Made (without success for the n8n Supabase credential/node connection):

API Key (SERVICE_ROLE_KEY):
Double-checked and carefully copy-pasted multiple times from the Supabase .env file to the n8n credential field (ensuring no extra spaces/characters).
Verified the key is correctly loaded as an environment variable inside the supabase-kong container using docker exec supabase-kong env | grep SUPABASE_SERVICE_KEY.
Host URL in n8n (Supabase API Credential):
Tested with the public URL managed by the proxy: https://supabase.cyriloup.shop.
Tested with the internal Docker service address: http://supabase-kong:8000.
Both result in the same 401 error when used specifically within the Supabase API credential setup.
Supabase/Kong Configuration:
Confirmed ~/docker/supabase/volumes/api/kong.yml exists and is loaded by Kong (visible in logs).
Verified kong.yml contains standard consumers (anon, service_role) with keyauth_credentials using $SUPABASE_ANON_KEY and $SUPABASE_SERVICE_KEY.
Confirmed key-auth and acl plugins are applied to the /rest/v1/ route. Temporarily disabling the acl plugin didn’t help.
(Attempted fix) Corrected the $SUPABASE_ANON_KEY variable mapping for the anon consumer in kong.yml.
Kong Entrypoint:
Tested with the original Supabase entrypoint (using eval) and a simplified version using --declarative_config_string. Both lead to the 401 error only in the Supabase API credential test. (Reverted to the original entrypoint).
Restarts:
Performed full restarts of the Supabase stack (docker-compose down && docker-compose up -d) after changes.
Restarted the entire VPS.
n8n Credential:
Deleted and recreated the Supabase API credential in n8n (e.g., named “Supabase API New”), but the issue persists.
Conclusion:

The problem is isolated to the n8n “Supabase API” credential type or the “Supabase” node itself. Standard authentication using the SERVICE_ROLE_KEY works perfectly fine via curl and n8n’s generic “HTTP Request” node. Kong only rejects the key when the connection attempt originates from n8n’s specific Supabase integration mechanism. This could potentially be a bug within this n8n feature or an issue where n8n misinterprets the configuration or API response.

Technical Information:

Kong Version (used by Supabase): 2.8.1
n8n Version: 1.90.0 (Issue was present also on 1.89.2, updating did not resolve it)
I’m looking for help understanding why the n8n Supabase credential/node fails with a 401 error, while other methods using the exact same API key and URL succeed.

Any ideas or suggestions would be greatly appreciated!

Thanks.



868
views

6
likes

1
link





read
5 min
13 days later
