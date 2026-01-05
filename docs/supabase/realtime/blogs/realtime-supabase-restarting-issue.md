How to Fix Supabase Realtime Service Restarting Issue in Self-Hosted Environments
Dileesha Weliwaththa
Dileesha Weliwaththa

Follow
3 min read
·
Sep 12, 2024
50




Press enter or click to view image in full size

Introduction
While working with Supabase in a self-hosted environment, I encountered an issue where the Realtime service kept restarting repeatedly. This disrupted the backend functionality, leading to downtime for applications relying on real-time features. After researching and experimenting with different solutions, I managed to resolve the issue by fine-tuning the Docker Compose setup. Here’s a detailed walkthrough of the problem, the troubleshooting process, and the final solution.

Problem
I was running Supabase Realtime v2.30.34 in a self-hosted environment with the following Docker container configuration:

realtime-dev.supabase-realtime   supabase/realtime:v2.30.34    Restarting (1) 41 seconds ago
The Supabase Realtime service was in a restart loop, and the logs showed an issue with the RLIMIT_NOFILE unbound variable. Below is a snippet of the logs:

/app/run.sh: line 6: RLIMIT_NOFILE: unbound variable
+ ulimit -n
1048576
This error indicated a problem with the file descriptor limits inside the Docker container, which was causing the service to fail to start.

NOTE : Use this to get the stat of the realtime service

docker stats realtime-dev.supabase-realtime
Initial Attempt at a Solution
I tried various approaches to resolve this, including:

Setting ulimit in the Docker Compose configuration.
Modifying the environment variables to include RLIMIT_NOFILE.
Increasing healthcheck timeouts and intervals.
Explicitly setting ulimit in the Docker command.
However, none of these solutions resolved the issue. The service continued to restart, and the logs continued to show the same error message.

The Solution: Updating to the Latest Supabase Configuration
After some more digging, I found the latest version of the Supabase Docker Compose file on their official GitHub repository. The configuration included a more refined setup for the Realtime service, which ultimately resolved the issue.

Here’s the working Docker Compose configuration:

realtime:
  # Container name for realtime
  container_name: realtime-dev.supabase-realtime
  image: supabase/realtime:v2.30.34
  depends_on:
    db:
      condition: service_healthy
    analytics:
      condition: service_healthy
  healthcheck:
    test: [ "CMD", "curl", "-sSfL", "--head", "-o", "/dev/null", "-H", "Authorization: Bearer ${ANON_KEY}", "http://localhost:4000/api/tenants/realtime-dev/health" ]
    timeout: 5s
    interval: 5s
    retries: 3
  restart: unless-stopped
  environment:
    PORT: 4000
    DB_HOST: ${POSTGRES_HOST}
    DB_PORT: ${POSTGRES_PORT}
    DB_USER: supabase_admin
    DB_PASSWORD: ${POSTGRES_PASSWORD}
    DB_NAME: ${POSTGRES_DB}
    DB_AFTER_CONNECT_QUERY: 'SET search_path TO _realtime'
    DB_ENC_KEY: supabaserealtime
    API_JWT_SECRET: ${JWT_SECRET}
    SECRET_KEY_BASE: UpNVntn3cDxHJpq99YMc1T1AQgQpc8kfYTuRgBiYa15BLrx8etQoXz3gZv1/u2oq
    ERL_AFLAGS: -proto_dist inet_tcp
    DNS_NODES: "''"
    RLIMIT_NOFILE: "10000"
    APP_NAME: realtime
    SEED_SELF_HOST: true
https://github.com/supabase/supabase/blob/master/docker/docker-compose.yml

Key Changes
Here’s what worked:

Setting RLIMIT_NOFILE to a lower value: In the updated configuration, the RLIMIT_NOFILE was set to 10000, which is a more reasonable number of file descriptors for most systems. This change fixed the unbound variable issue and stabilized the service.
Adding SEED_SELF_HOST: true: This option ensures that the service is properly seeded when running in a self-hosted environment. It's essential for the initialization of the real-time functionality.
Refining the healthcheck: The healthcheck parameters were adjusted to ensure the service is healthy before starting. The URL and timeout were set according to the most recent recommendations from Supabase.
Conclusion
If you are self-hosting Supabase and encounter issues with the Realtime service restarting, I recommend referring to the latest Docker Compose configuration provided by the Supabase team. By adjusting the RLIMIT_NOFILE and ensuring proper environment variables, you can ensure the service runs smoothly.

For those who are maintaining a self-hosted Supabase instance, always stay updated with the official repository, as the configuration might evolve to include fixes for such issues.