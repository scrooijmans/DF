<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/presence/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/presence/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

- <a href="https://supabase.com/docs/guides/getting-started" class="inline-flex items-center justify-center text-sm focus:outline-none focus:bg-accent focus:text-accent-foreground disabled:opacity-50 disabled:pointer-events-none hover:bg-accent data-[state=open]:bg-accent/50 data-[active]:bg-accent/50 group w-max p-2 bg-transparent border-0 border-b-2 border-transparent font-normal rounded-none text-foreground-light hover:text-foreground data-[state=open]:!text-foreground data-[radix-collection-item]:focus-visible:ring-2 data-[radix-collection-item]:focus-visible:ring-foreground-lighter data-[radix-collection-item]:focus-visible:text-foreground h-full focus-visible:rounded !shadow-none outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 focus-visible:outline-brand-600" data-radix-collection-item="">Start</a>
- Products <img src="out_realtime/presence/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Build <img src="out_realtime/presence/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Manage <img src="out_realtime/presence/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Reference <img src="out_realtime/presence/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Resources <img src="out_realtime/presence/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />

<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/presence/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/presence/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

<img src="out_realtime/presence/index_media/b05287a98e0c3df939d0ba4572c4c97583f8282c.svg" class="lucide lucide-search" />

Search docs...

<img src="out_realtime/presence/index_media/ddb7ce303f8bb684228548e111cd4f23b5fac1d4.svg" class="lucide lucide-command" />K

<img src="out_realtime/presence/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

<img src="out_realtime/presence/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

Realtime

# 

Presence

## 

Share state between users with Realtime Presence.

------------------------------------------------------------------------

Let's explore how to implement Realtime Presence to track state between multiple users.

## Usage<a href="https://supabase.com/docs/guides/realtime/presence#usage" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can use the Supabase client libraries to track Presence state between users.

### Initialize the client<a href="https://supabase.com/docs/guides/realtime/presence#initialize-the-client" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Go to your Supabase project's [API Settings](https://supabase.com/dashboard/project/_/settings/api) and grab the `URL` and `anon` public API key.

JavaScript

Dart

Swift

Kotlin

Python

``` flex
123456import { createClient } from '@supabase/supabase-js'const SUPABASE_URL = 'https://<project>.supabase.co'const SUPABASE_KEY = '<sb_publishable_... or anon key>'const supabase = createClient(SUPABASE_URL, SUPABASE_KEY)
```

<img src="out_realtime/presence/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Sync and track state<a href="https://supabase.com/docs/guides/realtime/presence#sync-and-track-state" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

JavaScript

Dart

Swift

Kotlin

Python

Listen to the `sync`, `join`, and `leave` events triggered whenever any client joins or leaves the channel or changes their slice of state:

``` flex
1234567891011121314const roomOne = supabase.channel('room_01')roomOne  .on('presence', { event: 'sync' }, () => {    const newState = roomOne.presenceState()    console.log('sync', newState)  })  .on('presence', { event: 'join' }, ({ key, newPresences }) => {    console.log('join', key, newPresences)  })  .on('presence', { event: 'leave' }, ({ key, leftPresences }) => {    console.log('leave', key, leftPresences)  })  .subscribe()
```

<img src="out_realtime/presence/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Sending state<a href="https://supabase.com/docs/guides/realtime/presence#sending-state" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can send state to all subscribers using `track()`:

JavaScript

Dart

Swift

Kotlin

Python

``` flex
12345678910111213const roomOne = supabase.channel('room_01')const userStatus = {  user: 'user-1',  online_at: new Date().toISOString(),}roomOne.subscribe(async (status) => {  if (status !== 'SUBSCRIBED') { return }  const presenceTrackStatus = await roomOne.track(userStatus)  console.log(presenceTrackStatus)})
```

<img src="out_realtime/presence/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

A client will receive state from any other client that is subscribed to the same topic (in this case `room_01`). It will also automatically trigger its own `sync` and `join` event handlers.

### Stop tracking<a href="https://supabase.com/docs/guides/realtime/presence#stop-tracking" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can stop tracking presence using the `untrack()` method. This will trigger the `sync` and `leave` event handlers.

JavaScript

Dart

Swift

Kotlin

Python

``` flex
123456const untrackPresence = async () => {  const presenceUntrackStatus = await roomOne.untrack()  console.log(presenceUntrackStatus)}untrackPresence()
```

<img src="out_realtime/presence/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

## Presence options<a href="https://supabase.com/docs/guides/realtime/presence#presence-options" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can pass configuration options while initializing the Supabase Client.

### Presence key<a href="https://supabase.com/docs/guides/realtime/presence#presence-key" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

By default, Presence will generate a unique `UUIDv1` key on the server to track a client channel's state. If you prefer, you can provide a custom key when creating the channel. This key should be unique among clients.

JavaScript

Dart

Swift

Kotlin

Python

``` flex
12345678910import { createClient } from '@supabase/supabase-js'const supabase = createClient('SUPABASE_URL', 'SUPABASE_PUBLISHABLE_KEY')const channelC = supabase.channel('test', {  config: {    presence: {      key: 'userId-123',    },  },})
```

<img src="out_realtime/presence/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

<a href="https://github.com/supabase/supabase/blob/master/apps/docs/content/guides/realtime/presence.mdx" class="w-fit flex items-center gap-1 text-sm text-scale-1000 hover:text-scale-1200 transition-colors" rel="noreferrer noopener edit" target="_blank">Edit this page on GitHub <img src="out_realtime/presence/index_media/3ee4434fa0936084ada9f9535617163941b979af.svg" class="lucide lucide-external-link" /></a>

### Is this helpful?

<img src="out_realtime/presence/index_media/82954652faa5c1357b3f1fbee5560e79717901b0.svg" class="lucide lucide-x text-current" />No

<img src="out_realtime/presence/index_media/a1e9c777d4da09bddca000e120dc325dadfad412.svg" class="lucide lucide-check" />Yes

- <img src="out_realtime/presence/index_media/fc3b667770ecf58c18a57c9707eeb4ef5cdb7b79.svg" class="lucide lucide-life-buoy" />

  Need some help?

  <a href="https://supabase.com/support" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Contact support</a>

- <img src="out_realtime/presence/index_media/6a48d9bf47efc6f6cc9f9198a5d9bd82a4eb61d0.svg" class="lucide lucide-flask-conical" />

  Latest product updates?

  <a href="https://supabase.com/changelog" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">See Changelog</a>

- <img src="out_realtime/presence/index_media/dca6f8c719c0f568123d76d384df2e5f750a66c1.svg" class="lucide lucide-circle-check-big" />

  Something's not right?

  <a href="https://status.supabase.com/" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Check system status</a>

------------------------------------------------------------------------

<a href="https://supabase.com/" class="text-xs text-foreground-lighter">© Supabase Inc</a>—<a href="https://github.com/supabase/supabase/blob/master/apps/docs/DEVELOPERS.md" class="text-xs text-foreground-lighter hover:underline">Contributing</a><a href="https://github.com/supabase/supabase/blob/master/apps/docs/CONTRIBUTING.md" class="text-xs text-foreground-lighter hover:underline">Author Styleguide</a><a href="https://supabase.com/open-source" class="text-xs text-foreground-lighter hover:underline">Open Source</a><a href="https://supabase.com/supasquad" class="text-xs text-foreground-lighter hover:underline">SupaSquad</a>

Privacy Settings

<a href="https://github.com/supabase/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">GitHub<img src="out_realtime/presence/index_media/6506f47eb1cac63154be703b9bea8227d8f97784.svg" /></a><a href="https://twitter.com/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Twitter<img src="out_realtime/presence/index_media/a14586df53b1ba626e64e260906d5d432bb45ac0.svg" /></a><a href="https://discord.supabase.com/" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Discord<img src="out_realtime/presence/index_media/aa0c9be8f4e5ebd1a79bf4ab7db7c857ba11e057.svg" /></a>
