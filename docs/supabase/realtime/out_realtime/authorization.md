<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/authorization/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/authorization/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

- <a href="https://supabase.com/docs/guides/getting-started" class="inline-flex items-center justify-center text-sm focus:outline-none focus:bg-accent focus:text-accent-foreground disabled:opacity-50 disabled:pointer-events-none hover:bg-accent data-[state=open]:bg-accent/50 data-[active]:bg-accent/50 group w-max p-2 bg-transparent border-0 border-b-2 border-transparent font-normal rounded-none text-foreground-light hover:text-foreground data-[state=open]:!text-foreground data-[radix-collection-item]:focus-visible:ring-2 data-[radix-collection-item]:focus-visible:ring-foreground-lighter data-[radix-collection-item]:focus-visible:text-foreground h-full focus-visible:rounded !shadow-none outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 focus-visible:outline-brand-600" data-radix-collection-item="">Start</a>
- Products <img src="out_realtime/authorization/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Build <img src="out_realtime/authorization/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Manage <img src="out_realtime/authorization/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Reference <img src="out_realtime/authorization/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Resources <img src="out_realtime/authorization/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />

<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/authorization/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/authorization/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

<img src="out_realtime/authorization/index_media/b05287a98e0c3df939d0ba4572c4c97583f8282c.svg" class="lucide lucide-search" />

Search docs...

<img src="out_realtime/authorization/index_media/ddb7ce303f8bb684228548e111cd4f23b5fac1d4.svg" class="lucide lucide-command" />K

<img src="out_realtime/authorization/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

<img src="out_realtime/authorization/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

Realtime

# 

Realtime Authorization

------------------------------------------------------------------------

You can control client access to Realtime [Broadcast](https://supabase.com/docs/guides/realtime/broadcast) and [Presence](https://supabase.com/docs/guides/realtime/presence) by adding Row Level Security policies to the `realtime.messages` table. Each RLS policy can map to a specific action a client can take:

- Control which clients can broadcast to a Channel
- Control which clients can receive broadcasts from a Channel
- Control which clients can publish their presence to a Channel
- Control which clients can receive messages about the presence of other clients

<img src="out_realtime/authorization/index_media/6da381d878739d1e834db715446f01b35a34b568.svg" class="w-6 h-6" />

Realtime Authorization is in Public Beta. To use Authorization for your Realtime Channels, use `supabase-js` version `v2.44.0` or later.

<img src="out_realtime/authorization/index_media/0f7fc93538e91ade401009300159ebc5054e285d.svg" class="w-6 h-6" />

To enforce private channels you need to disable the 'Allow public access' setting in [Realtime Settings](https://supabase.com/dashboard/project/_/realtime/settings)

## How it works<a href="https://supabase.com/docs/guides/realtime/authorization#how-it-works" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Realtime uses the `messages` table in your database's `realtime` schema to generate access policies for your clients when they connect to a Channel topic.

By creating RLS policies on the `realtime.messages` table you can control the access users have to a Channel topic, and features within a Channel topic.

The validation is done when the user connects. When their WebSocket connection is established and a Channel topic is joined, their permissions are calculated based on:

- The RLS policies on the `realtime.messages` table
- The user information sent as part of their [Auth JWT](https://supabase.com/docs/guides/auth/jwts)
- The request headers
- The Channel topic the user is trying to connect to

When Realtime generates a policy for a client it performs a query on the `realtime.messages` table and then rolls it back. Realtime does not store any messages in your `realtime.messages` table.

Using Realtime Authorization involves two steps:

- In your database, create RLS policies on the `realtime.messages`
- In your client, instantiate the Realtime Channel with the `config` option `private: true`

<img src="out_realtime/authorization/index_media/6da381d878739d1e834db715446f01b35a34b568.svg" class="w-6 h-6" />

Increased RLS complexity can impact database performance and connection time, leading to higher connection latency and decreased join rates.

## Accessing request information<a href="https://supabase.com/docs/guides/realtime/authorization#accessing-request-information" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

### `realtime.topic`<a href="https://supabase.com/docs/guides/realtime/authorization#realtimetopic" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can use the `realtime.topic` helper function when writing RLS policies. It returns the Channel topic the user is attempting to connect to.

``` flex
1234567create policy "authenticated can read all messages on topic"on "realtime"."messages"for selectto authenticatedusing (  (select realtime.topic()) = 'room-1');
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### JWT claims<a href="https://supabase.com/docs/guides/realtime/authorization#jwt-claims" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

The user claims can be accessed using the `current_setting` function. The claims are available as a JSON object in the `request.jwt.claims` setting.

``` flex
12345678create policy "authenticated with supabase.io email can read all"on "realtime"."messages"for selectto authenticatedusing (  -- Only users with the email claim ending with @supabase.io  (((current_setting('request.jwt.claims'))::json ->> 'email') ~~ '%@supabase.io'));
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

## Examples<a href="https://supabase.com/docs/guides/realtime/authorization#examples" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

The following examples use this schema:

``` flex
1234567891011121314151617181920212223create table public.rooms (    id bigint generated by default as identity primary key,    topic text not null unique);alter table public.rooms enable row level security;create table public.profiles (  id uuid not null references auth.users on delete cascade,  email text NOT NULL,  primary key (id));alter table public.profiles enable row level security;create table public.rooms_users (  user_id uuid references auth.users (id),  room_topic text references public.rooms (topic),  created_at timestamptz default current_timestamp);alter table public.rooms_users enable row level security;
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Broadcast<a href="https://supabase.com/docs/guides/realtime/authorization#broadcast" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

The `extension` field on the `realtime.messages` table records the message type. For Broadcast messages, the value of `realtime.messages.extension` is `broadcast`. You can check for this in your RLS policies.

#### Allow a user to join (and read) a Broadcast topic<a href="https://supabase.com/docs/guides/realtime/authorization#allow-a-user-to-join-and-read-a-broadcast-topic" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

To join a Broadcast Channel, a user must have at least one read or write permission on the Channel topic.

Here, we allow reads (`select`s) for users who are linked to the requested topic within the relationship table `public.room_users`:

``` flex
12345678910111213141516create policy "authenticated can receive broadcast"on "realtime"."messages"for selectto authenticatedusing (exists (    select      user_id    from      rooms_users    where      user_id = (select auth.uid())      and topic = (select realtime.topic())      and realtime.messages.extension in ('broadcast')  ));
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

Then, to join a topic with RLS enabled, instantiate the Channel with the `private` option set to `true`.

JavaScript

Dart

Swift

Kotlin

Python

``` flex
12345678910111213const channel = supabase.channel('room-1', {  config: { private: true },})channel  .on('broadcast', { event: 'test' }, (payload) => console.log(payload))  .subscribe((status, err) => {    if (status === 'SUBSCRIBED') {      console.log('Connected!')    } else {      console.error(err)    }  })
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

#### Allow a user to send a Broadcast message<a href="https://supabase.com/docs/guides/realtime/authorization#allow-a-user-to-send-a-broadcast-message" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

To authorize sending Broadcast messages, create a policy for `insert` where the value of `realtime.messages.extension` is `broadcast`.

Here, we allow writes (sends) for users who are linked to the requested topic within the relationship table `public.room_users`:

``` flex
12345678910111213141516create policy "authenticated can send broadcast on topic"on "realtime"."messages"for insertto authenticatedwith check (  exists (    select      user_id    from      rooms_users    where      user_id = (select auth.uid())      and topic = (select realtime.topic())      and realtime.messages.extension in ('broadcast')  ));
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Presence<a href="https://supabase.com/docs/guides/realtime/authorization#presence" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

The `extension` field on the `realtime.messages` table records the message type. For Presence messages, the value of `realtime.messages.extension` is `presence`. You can check for this in your RLS policies.

#### Allow users to listen to Presence messages on a Channel<a href="https://supabase.com/docs/guides/realtime/authorization#allow-users-to-listen-to-presence-messages-on-a-channel" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Create a policy for `select` on `realtime.messages` where `realtime.messages.extension` is `presence`.

``` flex
12345678910111213141516create policy "authenticated can listen to presence in topic"on "realtime"."messages"for selectto authenticatedusing (  exists (    select      user_id    from      rooms_users    where      user_id = (select auth.uid())      and topic = (select realtime.topic())      and realtime.messages.extension in ('presence')  ));
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

#### Allow users to send Presence messages on a channel<a href="https://supabase.com/docs/guides/realtime/authorization#allow-users-to-send-presence-messages-on-a-channel" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

To update the Presence status for a user create a policy for `insert` on `realtime.messages` where the value of `realtime.messages.extension` is `presence`.

``` flex
12345678910111213141516create policy "authenticated can track presence on topic"on "realtime"."messages"for insertto authenticatedwith check (  exists (    select      user_id    from      rooms_users    where      user_id = (select auth.uid())      and name = (select realtime.topic())      and realtime.messages.extension in ('presence')  ));
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Presence and Broadcast<a href="https://supabase.com/docs/guides/realtime/authorization#presence-and-broadcast" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Authorize both Presence and Broadcast by including both extensions in the `where` filter.

#### Broadcast and Presence read<a href="https://supabase.com/docs/guides/realtime/authorization#broadcast-and-presence-read" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Authorize Presence and Broadcast read in one RLS policy.

``` flex
12345678910111213141516create policy "authenticated can listen to broadcast and presence on topic"on "realtime"."messages"for selectto authenticatedusing (  exists (    select      user_id    from      rooms_users    where      user_id = (select auth.uid())      and topic = (select realtime.topic())      and realtime.messages.extension in ('broadcast', 'presence')  ));
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

#### Broadcast and Presence write<a href="https://supabase.com/docs/guides/realtime/authorization#broadcast-and-presence-write" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Authorize Presence and Broadcast write in one RLS policy.

``` flex
12345678910111213141516create policy "authenticated can send broadcast and presence on topic"on "realtime"."messages"for insertto authenticatedwith check (  exists (    select      user_id    from      rooms_users    where      user_id = (select auth.uid())      and name = (select realtime.topic())      and realtime.messages.extension in ('broadcast', 'presence')  ));
```

<img src="out_realtime/authorization/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

## Interaction with Postgres Changes<a href="https://supabase.com/docs/guides/realtime/authorization#interaction-with-postgres-changes" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Realtime Postgres Changes are separate from Channel authorization. The `private` Channel option does not apply to Postgres Changes.

When using Postgres Changes with RLS, database records are sent only to clients who are allowed to read them based on your RLS policies.

## Updating RLS policies<a href="https://supabase.com/docs/guides/realtime/authorization#updating-rls-policies" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Client access policies are cached for the duration of the connection. Your database is not queried for every Channel message.

Realtime updates the access policy cache for a client based on your RLS policies when:

- A client connects to Realtime and subscribes to a Channel
- A new JWT is sent to Realtime from a client via the [`access_token` message](https://supabase.com/docs/guides/realtime/protocol#access-token)

If a new JWT is never received on the Channel, the client will be disconnected when the JWT expires.

Make sure to keep the JWT expiration window short.

<a href="https://github.com/supabase/supabase/blob/master/apps/docs/content/guides/realtime/authorization.mdx" class="w-fit flex items-center gap-1 text-sm text-scale-1000 hover:text-scale-1200 transition-colors" rel="noreferrer noopener edit" target="_blank">Edit this page on GitHub <img src="out_realtime/authorization/index_media/3ee4434fa0936084ada9f9535617163941b979af.svg" class="lucide lucide-external-link" /></a>

### Is this helpful?

<img src="out_realtime/authorization/index_media/82954652faa5c1357b3f1fbee5560e79717901b0.svg" class="lucide lucide-x text-current" />No

<img src="out_realtime/authorization/index_media/a1e9c777d4da09bddca000e120dc325dadfad412.svg" class="lucide lucide-check" />Yes

- <img src="out_realtime/authorization/index_media/fc3b667770ecf58c18a57c9707eeb4ef5cdb7b79.svg" class="lucide lucide-life-buoy" />

  Need some help?

  <a href="https://supabase.com/support" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Contact support</a>

- <img src="out_realtime/authorization/index_media/6a48d9bf47efc6f6cc9f9198a5d9bd82a4eb61d0.svg" class="lucide lucide-flask-conical" />

  Latest product updates?

  <a href="https://supabase.com/changelog" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">See Changelog</a>

- <img src="out_realtime/authorization/index_media/dca6f8c719c0f568123d76d384df2e5f750a66c1.svg" class="lucide lucide-circle-check-big" />

  Something's not right?

  <a href="https://status.supabase.com/" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Check system status</a>

------------------------------------------------------------------------

<a href="https://supabase.com/" class="text-xs text-foreground-lighter">© Supabase Inc</a>—<a href="https://github.com/supabase/supabase/blob/master/apps/docs/DEVELOPERS.md" class="text-xs text-foreground-lighter hover:underline">Contributing</a><a href="https://github.com/supabase/supabase/blob/master/apps/docs/CONTRIBUTING.md" class="text-xs text-foreground-lighter hover:underline">Author Styleguide</a><a href="https://supabase.com/open-source" class="text-xs text-foreground-lighter hover:underline">Open Source</a><a href="https://supabase.com/supasquad" class="text-xs text-foreground-lighter hover:underline">SupaSquad</a>

Privacy Settings

<a href="https://github.com/supabase/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">GitHub<img src="out_realtime/authorization/index_media/6506f47eb1cac63154be703b9bea8227d8f97784.svg" /></a><a href="https://twitter.com/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Twitter<img src="out_realtime/authorization/index_media/a14586df53b1ba626e64e260906d5d432bb45ac0.svg" /></a><a href="https://discord.supabase.com/" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Discord<img src="out_realtime/authorization/index_media/aa0c9be8f4e5ebd1a79bf4ab7db7c857ba11e057.svg" /></a>
