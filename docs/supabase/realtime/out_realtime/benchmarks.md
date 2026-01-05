<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/benchmarks/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/benchmarks/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

- <a href="https://supabase.com/docs/guides/getting-started" class="inline-flex items-center justify-center text-sm focus:outline-none focus:bg-accent focus:text-accent-foreground disabled:opacity-50 disabled:pointer-events-none hover:bg-accent data-[state=open]:bg-accent/50 data-[active]:bg-accent/50 group w-max p-2 bg-transparent border-0 border-b-2 border-transparent font-normal rounded-none text-foreground-light hover:text-foreground data-[state=open]:!text-foreground data-[radix-collection-item]:focus-visible:ring-2 data-[radix-collection-item]:focus-visible:ring-foreground-lighter data-[radix-collection-item]:focus-visible:text-foreground h-full focus-visible:rounded !shadow-none outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 focus-visible:outline-brand-600" data-radix-collection-item="">Start</a>
- Products <img src="out_realtime/benchmarks/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Build <img src="out_realtime/benchmarks/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Manage <img src="out_realtime/benchmarks/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Reference <img src="out_realtime/benchmarks/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Resources <img src="out_realtime/benchmarks/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />

<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/benchmarks/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/benchmarks/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

<img src="out_realtime/benchmarks/index_media/b05287a98e0c3df939d0ba4572c4c97583f8282c.svg" class="lucide lucide-search" />

Search docs...

<img src="out_realtime/benchmarks/index_media/ddb7ce303f8bb684228548e111cd4f23b5fac1d4.svg" class="lucide lucide-command" />K

<img src="out_realtime/benchmarks/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

<img src="out_realtime/benchmarks/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

Realtime

# 

Benchmarks

## 

Scalability Benchmarks for Supabase Realtime.

------------------------------------------------------------------------

This guide explores the scalability of Realtime's features: Broadcast, Presence, and Postgres Changes.

## Methodology<a href="https://supabase.com/docs/guides/realtime/benchmarks#methodology" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

- The benchmarks are conducted using k6, an open-source load testing tool, against a Realtime Cluster deployed on AWS.
- The cluster configurations use 2-6 nodes, tested in both single-region and multi-region setups, all connected to a single Supabase project.
- The load generators (k6 servers) are deployed on AWS to minimize network latency impact on the results.
- Tests are executed with a full load from the start without warm-up runs.

The metrics collected include: message throughput, latency percentiles, CPU and memory utilization, and connection success rates. Note that performance in production environments may vary based on factors such as network conditions, hardware specifications, and specific usage patterns.

## Workloads<a href="https://supabase.com/docs/guides/realtime/benchmarks#workloads" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

The proposed workloads are designed to demonstrate Supabase Realtime's throughput and scalability. These benchmarks focus on core functionality and common usage patterns. The benchmarking results include the following workloads:

1.  **Broadcast Performance**
2.  **Payload Size Impact on Broadcast**
3.  **Large-Scale Broadcasting**
4.  **Authentication and New Connection Rate**
5.  **Database Events**

## Results<a href="https://supabase.com/docs/guides/realtime/benchmarks#results" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

### Broadcast: Using WebSockets<a href="https://supabase.com/docs/guides/realtime/benchmarks#broadcast-using-websockets" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

This workload evaluates the system's capacity to handle multiple concurrent WebSocket connections and sending Broadcast messages via the WebSocket. Each virtual user (VU) in the test:

- Establishes and maintains a WebSocket connection
- Joins two distinct channels:
  - An echo channel (1 user per channel) for direct message reflection
  - A broadcast channel (6 users per channel) for group communication
- Generates traffic by sending 2 messages per second to each joined channel for 10 minutes

![Broadcast Performance](out_realtime/benchmarks/index_media/e89d291ee0d7fce8e4233f8dac6546adf1332873.png)

| Metric              | Value                   |
|---------------------|-------------------------|
| Concurrent Users    | 32_000                  |
| Total Channel Joins | 64_000                  |
| Message Throughput  | 224_000 msgs/sec        |
| Median Latency      | 6 ms                    |
| Latency (p95)       | 28 ms                   |
| Latency (p99)       | 213 ms                  |
| Data Received       | 6.4 MB/s (7.9 GB total) |
| Data Sent           | 23 KB/s (28 MB total)   |
| New Connection Rate | 320 conn/sec            |
| Channel Join Rate   | 640 joins/sec           |

### Broadcast: Using the database<a href="https://supabase.com/docs/guides/realtime/benchmarks#broadcast-using-the-database" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

This workload evaluates the system's capacity to send Broadcast messages from the database using the `realtime.broadcast_changes` function. Each virtual user (VU) in the test:

- Establishes and maintains a WebSocket connection
- Joins a distinct channel:
  - A single channel (100 users per channel) for group communication
- Database has a trigger set to run `realtime.broadcast_changes` on every insert
- Database triggers 10_000 inserts per second

![Broadcast from Database Performance](out_realtime/benchmarks/index_media/706868fc84cb2f60afac7371ce0b67bd3670cf70.png)

| Metric              | Value                  |
|---------------------|------------------------|
| Concurrent Users    | 80_000                 |
| Total Channel Joins | 160_000                |
| Message Throughput  | 10_000 msgs/sec        |
| Median Latency      | 46 ms                  |
| Latency (p95)       | 132 ms                 |
| Latency (p99)       | 159 ms                 |
| Data Received       | 1.7 MB/s (42 GB total) |
| Data Sent           | 0.4 MB/s (4 GB total)  |
| New Connection Rate | 2000 conn/sec          |
| Channel Join Rate   | 4000 joins/sec         |

### Broadcast: Impact of payload size<a href="https://supabase.com/docs/guides/realtime/benchmarks#broadcast-impact-of-payload-size" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

This workload tests the system's performance with different message payload sizes to understand how data volume affects throughput and latency. Each virtual user (VU) follows the same connection pattern as the broadcast test, but with varying message sizes:

- Establishes and maintains a WebSocket connection
- Joins two distinct channels:
  - An echo channel (1 user per channel) for direct message reflection
  - A broadcast channel (6 users per channel) for group communication
- Sends messages with payloads of 1KB, 10KB, and 50KB
- Generates traffic by sending 2 messages per second to each joined channel for 5 minutes

#### 1KB payload<a href="https://supabase.com/docs/guides/realtime/benchmarks#1kb-payload" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

![1KB Payload Broadcast Performance](out_realtime/benchmarks/index_media/c1853a6a87930b50f429eb8b45e2f5aad52b0f3a.png)

#### 10KB payload<a href="https://supabase.com/docs/guides/realtime/benchmarks#10kb-payload" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

![10KB Payload Broadcast Performance](out_realtime/benchmarks/index_media/d0d91eb7493185dc86e22c2356182fe967f1b722.png)

#### 50KB payload<a href="https://supabase.com/docs/guides/realtime/benchmarks#50kb-payload" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

![50KB Payload Broadcast Performance](out_realtime/benchmarks/index_media/8a0256a5262f00fa62287bc7fea3e07d234f89dd.png)

| Metric | 1KB Payload | 10KB Payload | 50KB Payload | 50KB Payload (Reduced Load) |
|----|----|----|----|----|
| Concurrent Users | 4_000 | 4_000 | 4_000 | 2_000 |
| Message Throughput | 28_000 msgs/sec | 28_000 msgs/sec | 28_000 msgs/sec | 14_000 msgs/sec |
| Median Latency | 13 ms | 16 ms | 27 ms | 19 ms |
| Latency (p95) | 36 ms | 42 ms | 81 ms | 39 ms |
| Latency (p99) | 85 ms | 93 ms | 146 ms | 82 ms |
| Data Received | 31.2 MB/s (10.4 GB) | 268 MB/s (72 GB) | 1284 MB/s (348 GB) | 644 MB/s (176 GB) |
| Data Sent | 9.2 MB/s (3.1 GB) | 76 MB/s (20.8 GB) | 384 MB/s (104 GB) | 192 MB/s (52 GB) |

> Note: The final column shows results with reduced load (2,000 users) for the 50KB payload test, demonstrating how the system performs with larger payloads under different concurrency levels.

### Broadcast: Scalability scenarios<a href="https://supabase.com/docs/guides/realtime/benchmarks#broadcast-scalability-scenarios" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

This workload demonstrates Realtime's capability to handle high-scale scenarios with a large number of concurrent users and broadcast channels. The test simulates a scenario where each user participates in group communications with periodic message broadcasts. Each virtual user (VU):

- Establishes and maintains a WebSocket connection (30-120 minutes)
- Joins 2 broadcast channels
- Sends 1 message per minute to each joined channel
- Each message is broadcast to 100 other users

![Large Broadcast Performance](out_realtime/benchmarks/index_media/7580861bd5c1b936020de3d1db3cc83907cffb50.png)

| Metric              | Value              |
|---------------------|--------------------|
| Concurrent Users    | 250_000            |
| Total Channel Joins | 500_000            |
| Users per Channel   | 100                |
| Message Throughput  | \>800_000 msgs/sec |
| Median Latency      | 58 ms              |
| Latency (p95)       | 279 ms             |
| Latency (p99)       | 508 ms             |
| Data Received       | 68 MB/s (600 GB)   |
| Data Sent           | 0.64 MB/s (5.7 GB) |

### Realtime Auth<a href="https://supabase.com/docs/guides/realtime/benchmarks#realtime-auth" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

This workload demonstrates Realtime's capability to handle large amounts of new connections per second and channel joins per second with Authentication Row Level Security (RLS) enabled for these channels. The test simulates a scenario where large volumes of users connect to realtime and participate in auth protected communications. Each virtual user (VU):

- Establishes and maintains a WebSocket connection (2.5 minutes)
- Joins 2 broadcast channels
- Sends 1 message per minute to each joined channel
- Each message is broadcast to 100 other users

![Broadcast Auth Performance](out_realtime/benchmarks/index_media/1708b8a185c2ab4c739b618170b0d3a5685bc059.png)

| Metric              | Value              |
|---------------------|--------------------|
| Concurrent Users    | 50_000             |
| Total Channel Joins | 100_000            |
| Users per Channel   | 100                |
| Message Throughput  | \>150_000 msgs/sec |
| New Connection Rate | 500 conn/sec       |
| Channel Join Rate   | 1000 joins/sec     |
| Median Latency      | 19 ms              |
| Latency (p95)       | 49 ms              |
| Latency (p99)       | 96 ms              |

### Postgres Changes<a href="https://supabase.com/docs/guides/realtime/benchmarks#postgres-changes" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Realtime systems usually require forethought because of their scaling dynamics. For the `Postgres Changes` feature, every change event must be checked to see if the subscribed user has access. For instance, if you have 100 users subscribed to a table where you make a single insert, it will then trigger 100 "reads": one for each user.

There can be a database bottleneck which limits message throughput. If your database cannot authorize the changes rapidly enough, the changes will be delayed until you receive a timeout.

Database changes are processed on a single thread to maintain the change order. That means compute upgrades don't have a large effect on the performance of Postgres change subscriptions. You can estimate the expected maximum throughput for your database below.

If you are using Postgres Changes at scale, you should consider using a separate "public" table without RLS and filters. Alternatively, you can use Realtime server-side only and then re-stream the changes to your clients using a Realtime Broadcast.

Enter your database settings to estimate the maximum throughput for your instance:

Don't forget to run your own benchmarks to make sure that the performance is acceptable for your use case.

Supabase continues to make improvements to Realtime's Postgres Changes. If you are uncertain about your use case performance, reach out using the [Support Form](https://supabase.com/dashboard/support/new). The support team can advise on the best solution for each use-case.

<a href="https://github.com/supabase/supabase/blob/master/apps/docs/content/guides/realtime/benchmarks.mdx" class="w-fit flex items-center gap-1 text-sm text-scale-1000 hover:text-scale-1200 transition-colors" rel="noreferrer noopener edit" target="_blank">Edit this page on GitHub <img src="out_realtime/benchmarks/index_media/3ee4434fa0936084ada9f9535617163941b979af.svg" class="lucide lucide-external-link" /></a>

### Is this helpful?

<img src="out_realtime/benchmarks/index_media/82954652faa5c1357b3f1fbee5560e79717901b0.svg" class="lucide lucide-x text-current" />No

<img src="out_realtime/benchmarks/index_media/a1e9c777d4da09bddca000e120dc325dadfad412.svg" class="lucide lucide-check" />Yes

- <img src="out_realtime/benchmarks/index_media/fc3b667770ecf58c18a57c9707eeb4ef5cdb7b79.svg" class="lucide lucide-life-buoy" />

  Need some help?

  <a href="https://supabase.com/support" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Contact support</a>

- <img src="out_realtime/benchmarks/index_media/6a48d9bf47efc6f6cc9f9198a5d9bd82a4eb61d0.svg" class="lucide lucide-flask-conical" />

  Latest product updates?

  <a href="https://supabase.com/changelog" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">See Changelog</a>

- <img src="out_realtime/benchmarks/index_media/dca6f8c719c0f568123d76d384df2e5f750a66c1.svg" class="lucide lucide-circle-check-big" />

  Something's not right?

  <a href="https://status.supabase.com/" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Check system status</a>

------------------------------------------------------------------------

<a href="https://supabase.com/" class="text-xs text-foreground-lighter">© Supabase Inc</a>—<a href="https://github.com/supabase/supabase/blob/master/apps/docs/DEVELOPERS.md" class="text-xs text-foreground-lighter hover:underline">Contributing</a><a href="https://github.com/supabase/supabase/blob/master/apps/docs/CONTRIBUTING.md" class="text-xs text-foreground-lighter hover:underline">Author Styleguide</a><a href="https://supabase.com/open-source" class="text-xs text-foreground-lighter hover:underline">Open Source</a><a href="https://supabase.com/supasquad" class="text-xs text-foreground-lighter hover:underline">SupaSquad</a>

Privacy Settings

<a href="https://github.com/supabase/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">GitHub<img src="out_realtime/benchmarks/index_media/6506f47eb1cac63154be703b9bea8227d8f97784.svg" /></a><a href="https://twitter.com/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Twitter<img src="out_realtime/benchmarks/index_media/a14586df53b1ba626e64e260906d5d432bb45ac0.svg" /></a><a href="https://discord.supabase.com/" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Discord<img src="out_realtime/benchmarks/index_media/aa0c9be8f4e5ebd1a79bf4ab7db7c857ba11e057.svg" /></a>
