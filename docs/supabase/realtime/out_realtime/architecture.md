<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/architecture/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/architecture/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

- <a href="https://supabase.com/docs/guides/getting-started" class="inline-flex items-center justify-center text-sm focus:outline-none focus:bg-accent focus:text-accent-foreground disabled:opacity-50 disabled:pointer-events-none hover:bg-accent data-[state=open]:bg-accent/50 data-[active]:bg-accent/50 group w-max p-2 bg-transparent border-0 border-b-2 border-transparent font-normal rounded-none text-foreground-light hover:text-foreground data-[state=open]:!text-foreground data-[radix-collection-item]:focus-visible:ring-2 data-[radix-collection-item]:focus-visible:ring-foreground-lighter data-[radix-collection-item]:focus-visible:text-foreground h-full focus-visible:rounded !shadow-none outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 focus-visible:outline-brand-600" data-radix-collection-item="">Start</a>
- Products <img src="out_realtime/architecture/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Build <img src="out_realtime/architecture/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Manage <img src="out_realtime/architecture/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Reference <img src="out_realtime/architecture/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Resources <img src="out_realtime/architecture/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />

<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/architecture/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/architecture/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

<img src="out_realtime/architecture/index_media/b05287a98e0c3df939d0ba4572c4c97583f8282c.svg" class="lucide lucide-search" />

Search docs...

<img src="out_realtime/architecture/index_media/ddb7ce303f8bb684228548e111cd4f23b5fac1d4.svg" class="lucide lucide-command" />K

<img src="out_realtime/architecture/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

<img src="out_realtime/architecture/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

Realtime

# 

Realtime Architecture

------------------------------------------------------------------------

Realtime is a globally distributed Elixir cluster. Clients can connect to any node in the cluster via WebSockets and send messages to any other client connected to the cluster.

Realtime is written in [Elixir](https://elixir-lang.org/), which compiles to [Erlang](https://www.erlang.org/), and utilizes many tools the [Phoenix Framework](https://www.phoenixframework.org/) provides out of the box.

## Elixir & Phoenix<a href="https://supabase.com/docs/guides/realtime/architecture#elixir--phoenix" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Phoenix is fast and able to handle millions of concurrent connections.

Phoenix can handle many concurrent connections because Elixir provides lightweight processes (not OS processes) to work with.

Client-facing WebSocket servers need to handle many concurrent connections. Elixir & Phoenix let the Supabase Realtime cluster do this easily.

## Channels<a href="https://supabase.com/docs/guides/realtime/architecture#channels" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Channels are implemented using [Phoenix Channels](https://hexdocs.pm/phoenix/channels.html) which uses [Phoenix.PubSub](https://hexdocs.pm/phoenix_pubsub/Phoenix.PubSub.html) with the default `Phoenix.PubSub.PG2` adapter.

The PG2 adapter utilizes Erlang [process groups](https://www.erlang.org/docs/18/man/pg2.html) to implement the PubSub model where a publisher can send messages to many subscribers.

## Global cluster<a href="https://supabase.com/docs/guides/realtime/architecture#global-cluster" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Presence is an in-memory key-value store backed by a CRDT. When a user is connected to the cluster the state of that user is sent to all connected Realtime nodes.

Broadcast lets you send a message from any connected client to a Channel. Any other client connected to that same Channel will receive that message.

This works globally. A client connected to a Realtime node in the United States can send a message to another client connected to a node in Singapore. Connect two clients to the same Realtime Channel and they'll all receive the same messages.

Broadcast is useful for getting messages to users in the same location very quickly. If a group of clients are connected to a node in Singapore, the message only needs to go to that Realtime node in Singapore and back down. If users are close to a Realtime node they'll get Broadcast messages in the time it takes to ping the cluster.

Thanks to the Realtime cluster, you (an amazing Supabase user) don't have to think about which regions your clients are connected to.

If you're using Broadcast, Presence, or streaming database changes, messages will always get to your users via the shortest path possible.

## Connecting to a database<a href="https://supabase.com/docs/guides/realtime/architecture#connecting-to-a-database" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Realtime allows you to listen to changes from your Postgres database. When a new client connects to Realtime and initializes the `postgres_changes` Realtime Extension the cluster will connect to your Postgres database and start streaming changes from a replication slot.

Realtime knows the region your database is in, and connects to it from the closest region possible.

Every Realtime region has at least two nodes so if one node goes offline the other node should reconnect and start streaming changes again.

## Broadcast from Postgres<a href="https://supabase.com/docs/guides/realtime/architecture#broadcast-from-postgres" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Realtime Broadcast sends messages when changes happen in your database. Behind the scenes, Realtime creates a publication on the `realtime.messages` table. It then reads the Write-Ahead Log (WAL) file for this table, and sends a message whenever an insert happens. Messages are sent as JSON packages over WebSockets.

The `realtime.messages` table is partitioned by day. This allows old messages to be deleted performantly, by dropping old partitions. Partitions are retained for 3 days before being deleted.

Broadcast uses [Realtime Authorization](https://supabase.com/docs/guides/realtime/authorization) by default to protect your data.

## Streaming the Write-Ahead Log<a href="https://supabase.com/docs/guides/realtime/architecture#streaming-the-write-ahead-log" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

A Postgres logical replication slot is acquired when connecting to your database.

Realtime delivers changes by polling the replication slot and appending channel subscription IDs to each wal record.

Subscription IDs are Erlang processes representing underlying sockets on the cluster. These IDs are globally unique and messages to processes are routed automatically by the Erlang virtual machine.

After receiving results from the polling query, with subscription IDs appended, Realtime delivers records to those clients.

<a href="https://github.com/supabase/supabase/blob/master/apps/docs/content/guides/realtime/architecture.mdx" class="w-fit flex items-center gap-1 text-sm text-scale-1000 hover:text-scale-1200 transition-colors" rel="noreferrer noopener edit" target="_blank">Edit this page on GitHub <img src="out_realtime/architecture/index_media/3ee4434fa0936084ada9f9535617163941b979af.svg" class="lucide lucide-external-link" /></a>

### Is this helpful?

<img src="out_realtime/architecture/index_media/82954652faa5c1357b3f1fbee5560e79717901b0.svg" class="lucide lucide-x text-current" />No

<img src="out_realtime/architecture/index_media/a1e9c777d4da09bddca000e120dc325dadfad412.svg" class="lucide lucide-check" />Yes

- <img src="out_realtime/architecture/index_media/fc3b667770ecf58c18a57c9707eeb4ef5cdb7b79.svg" class="lucide lucide-life-buoy" />

  Need some help?

  <a href="https://supabase.com/support" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Contact support</a>

- <img src="out_realtime/architecture/index_media/6a48d9bf47efc6f6cc9f9198a5d9bd82a4eb61d0.svg" class="lucide lucide-flask-conical" />

  Latest product updates?

  <a href="https://supabase.com/changelog" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">See Changelog</a>

- <img src="out_realtime/architecture/index_media/dca6f8c719c0f568123d76d384df2e5f750a66c1.svg" class="lucide lucide-circle-check-big" />

  Something's not right?

  <a href="https://status.supabase.com/" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Check system status</a>

------------------------------------------------------------------------

<a href="https://supabase.com/" class="text-xs text-foreground-lighter">© Supabase Inc</a>—<a href="https://github.com/supabase/supabase/blob/master/apps/docs/DEVELOPERS.md" class="text-xs text-foreground-lighter hover:underline">Contributing</a><a href="https://github.com/supabase/supabase/blob/master/apps/docs/CONTRIBUTING.md" class="text-xs text-foreground-lighter hover:underline">Author Styleguide</a><a href="https://supabase.com/open-source" class="text-xs text-foreground-lighter hover:underline">Open Source</a><a href="https://supabase.com/supasquad" class="text-xs text-foreground-lighter hover:underline">SupaSquad</a>

Privacy Settings

<a href="https://github.com/supabase/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">GitHub<img src="out_realtime/architecture/index_media/6506f47eb1cac63154be703b9bea8227d8f97784.svg" /></a><a href="https://twitter.com/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Twitter<img src="out_realtime/architecture/index_media/a14586df53b1ba626e64e260906d5d432bb45ac0.svg" /></a><a href="https://discord.supabase.com/" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Discord<img src="out_realtime/architecture/index_media/aa0c9be8f4e5ebd1a79bf4ab7db7c857ba11e057.svg" /></a>
