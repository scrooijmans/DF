<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/broadcast/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/broadcast/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

- <a href="https://supabase.com/docs/guides/getting-started" class="inline-flex items-center justify-center text-sm focus:outline-none focus:bg-accent focus:text-accent-foreground disabled:opacity-50 disabled:pointer-events-none hover:bg-accent data-[state=open]:bg-accent/50 data-[active]:bg-accent/50 group w-max p-2 bg-transparent border-0 border-b-2 border-transparent font-normal rounded-none text-foreground-light hover:text-foreground data-[state=open]:!text-foreground data-[radix-collection-item]:focus-visible:ring-2 data-[radix-collection-item]:focus-visible:ring-foreground-lighter data-[radix-collection-item]:focus-visible:text-foreground h-full focus-visible:rounded !shadow-none outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 focus-visible:outline-brand-600" data-radix-collection-item="">Start</a>
- Products <img src="out_realtime/broadcast/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Build <img src="out_realtime/broadcast/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Manage <img src="out_realtime/broadcast/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Reference <img src="out_realtime/broadcast/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />
- Resources <img src="out_realtime/broadcast/index_media/d0b2fdb36dc677fe80987a3cb0969f4e561fcadc.svg" class="lucide lucide-chevron-down relative top-[1px] ml-1 h-3 w-3 transition duration-200 group-data-[state=open]:rotate-180" />

<a href="https://supabase.com/docs" class="relative justify-center cursor-pointer space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground bg-alternative dark:bg-muted hover:bg-selection border-strong hover:border-stronger focus-visible:outline-brand-600 data-[state=open]:bg-selection data-[state=open]:outline-brand-600 data-[state=open]:border-button-hover flex shrink-0 items-center w-fit !bg-transparent !border-none !shadow-none"><img src="out_realtime/broadcast/index_media/870911359a3625198fe1f51ab0aa042d69dfaeb3.svg" class="hidden dark:block !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" /><img src="out_realtime/broadcast/index_media/5195658f5cc2028b0cfcfc50e6d9cf71452e1e35.svg" class="block dark:hidden !m-0" style="color:transparent" data-nimg="1" decoding="async" loading="eager" width="96" height="18" alt="Supabase wordmark" />DOCS</a>

<img src="out_realtime/broadcast/index_media/b05287a98e0c3df939d0ba4572c4c97583f8282c.svg" class="lucide lucide-search" />

Search docs...

<img src="out_realtime/broadcast/index_media/ddb7ce303f8bb684228548e111cd4f23b5fac1d4.svg" class="lucide lucide-command" />K

<img src="out_realtime/broadcast/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

<img src="out_realtime/broadcast/index_media/7fda350c91a8ac58e9b5292c5ec238ec14e020e3.svg" class="lucide lucide-menu" />

Realtime

# 

Broadcast

## 

Send low-latency messages using the client libs, REST, or your Database.

------------------------------------------------------------------------

You can use Realtime Broadcast to send low-latency messages between users. Messages can be sent using the client libraries, REST APIs, or directly from your database.

## Subscribe to messages<a href="https://supabase.com/docs/guides/realtime/broadcast#subscribe-to-messages" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can use the Supabase client libraries to receive Broadcast messages.

### Initialize the client<a href="https://supabase.com/docs/guides/realtime/broadcast#initialize-the-client" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Go to your Supabase project's [API Settings](https://supabase.com/dashboard/project/_/settings/api) and grab the `URL` and `anon` public API key.

JavaScript

Dart

Swift

Kotlin

Python

``` flex
123456import { createClient } from '@supabase/supabase-js'const SUPABASE_URL = 'https://<project>.supabase.co'const SUPABASE_KEY = '<sb_publishable_... or anon key>'const supabase = createClient(SUPABASE_URL, SUPABASE_KEY)
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Receiving Broadcast messages<a href="https://supabase.com/docs/guides/realtime/broadcast#receiving-broadcast-messages" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can provide a callback for the `broadcast` channel to receive messages. This example will receive any `broadcast` messages that are sent to `test-channel`:

JavaScript

Dart

Swift

Kotlin

Python

``` flex
12345678910111213141516// Join a room/topic. Can be anything except for 'realtime'.const myChannel = supabase.channel('test-channel')// Simple function to log any messages we receivefunction messageReceived(payload) {  console.log(payload)}// Subscribe to the ChannelmyChannel  .on(    'broadcast',    { event: 'shout' }, // Listen for "shout". Can be "*" to listen to all events    (payload) => messageReceived(payload)  )  .subscribe()
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

## Send messages<a href="https://supabase.com/docs/guides/realtime/broadcast#send-messages" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

### Broadcast using the client libraries<a href="https://supabase.com/docs/guides/realtime/broadcast#broadcast-using-the-client-libraries" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can use the Supabase client libraries to send Broadcast messages.

JavaScript

Dart

Swift

Kotlin

Python

``` flex
12345678910111213141516171819202122232425262728const myChannel = supabase.channel('test-channel')/** * Sending a message before subscribing will use HTTP */myChannel  .send({    type: 'broadcast',    event: 'shout',    payload: { message: 'Hi' },  })  .then((resp) => console.log(resp))/** * Sending a message after subscribing will use Websockets */myChannel.subscribe((status) => {  if (status !== 'SUBSCRIBED') {    return null  }  myChannel.send({    type: 'broadcast',    event: 'shout',    payload: { message: 'Hi' },  })})
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Broadcast from the Database<a href="https://supabase.com/docs/guides/realtime/broadcast#broadcast-from-the-database" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

<img src="out_realtime/broadcast/index_media/6da381d878739d1e834db715446f01b35a34b568.svg" class="w-6 h-6" />

This feature is in Public Beta. [Submit a support ticket](https://supabase.help) if you have any issues.

<img src="out_realtime/broadcast/index_media/0f7fc93538e91ade401009300159ebc5054e285d.svg" class="w-6 h-6" />

All the messages sent using Broadcast from the Database are stored in `realtime.messages` table and will be deleted after 3 days.

You can send messages directly from your database using the `realtime.send()` function:

``` flex
1234567select  realtime.send(    jsonb_build_object('hello', 'world'), -- JSONB Payload    'event', -- Event name    'topic', -- Topic    false -- Public / Private flag  );
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

It's a common use case to broadcast messages when a record is created, updated, or deleted. We provide a helper function specific to this use case, `realtime.broadcast_changes()`. For more details, check out the [Subscribing to Database Changes](https://supabase.com/docs/guides/realtime/subscribing-to-database-changes) guide.

### Broadcast using the REST API<a href="https://supabase.com/docs/guides/realtime/broadcast#broadcast-using-the-rest-api" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can send a Broadcast message by making an HTTP request to Realtime servers.

cURL

POST

``` flex
12345678910111213curl -v \-H 'apikey: <SUPABASE_TOKEN>' \-H 'Content-Type: application/json' \--data-raw '{  "messages": [    {      "topic": "test",      "event": "event",      "payload": { "test": "test" }    }  ]}' \'https://<PROJECT_REF>.supabase.co/realtime/v1/api/broadcast'
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

## Broadcast options<a href="https://supabase.com/docs/guides/realtime/broadcast#broadcast-options" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can pass configuration options while initializing the Supabase Client.

### Self-send messages<a href="https://supabase.com/docs/guides/realtime/broadcast#self-send-messages" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

JavaScript

Dart

Swift

Kotlin

Python

By default, broadcast messages are only sent to other clients. You can broadcast messages back to the sender by setting Broadcast's `self` parameter to `true`.

``` flex
1234567891011121314151617181920const myChannel = supabase.channel('room-2', {  config: {    broadcast: { self: true },  },})myChannel.on(  'broadcast',  { event: 'test-my-messages' },  (payload) => console.log(payload))myChannel.subscribe((status) => {  if (status !== 'SUBSCRIBED') { return }  myChannel.send({    type: 'broadcast',    event: 'test-my-messages',    payload: { message: 'talking to myself' },  })})
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Acknowledge messages<a href="https://supabase.com/docs/guides/realtime/broadcast#acknowledge-messages" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

JavaScript

Dart

Swift

Kotlin

Python

You can confirm that the Realtime servers have received your message by setting Broadcast's `ack` config to `true`.

``` flex
1234567891011121314151617const myChannel = supabase.channel('room-3', {  config: {    broadcast: { ack: true },  },})myChannel.subscribe(async (status) => {  if (status !== 'SUBSCRIBED') { return }  const serverResponse = await myChannel.send({    type: 'broadcast',    event: 'acknowledge',    payload: {},  })  console.log('serverResponse', serverResponse)})
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

Use this to guarantee that the server has received the message before resolving `channelD.send`'s promise. If the `ack` config is not set to `true` when creating the channel, the promise returned by `channelD.send` will resolve immediately.

### Send messages using REST calls<a href="https://supabase.com/docs/guides/realtime/broadcast#send-messages-using-rest-calls" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

You can also send a Broadcast message by making an HTTP request to Realtime servers. This is useful when you want to send messages from your server or client without having to first establish a WebSocket connection.

JavaScript

Dart

Swift

Kotlin

Python

<img src="out_realtime/broadcast/index_media/0f7fc93538e91ade401009300159ebc5054e285d.svg" class="w-6 h-6" />

This is currently available only in the Supabase JavaScript client version 2.37.0 and later.

``` flex
123456789101112131415const channel = supabase.channel('test-channel')// No need to subscribe to channelchannel  .send({    type: 'broadcast',    event: 'test',    payload: { message: 'Hi' },  })  .then((resp) => console.log(resp))// Remember to clean up the channelsupabase.removeChannel(channel)
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

## Trigger broadcast messages from your database<a href="https://supabase.com/docs/guides/realtime/broadcast#trigger-broadcast-messages-from-your-database" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

### How it works<a href="https://supabase.com/docs/guides/realtime/broadcast#how-it-works" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Broadcast Changes allows you to trigger messages from your database. To achieve it Realtime is directly reading your WAL (Write Append Log) file using a publication against the `realtime.messages` table so whenever a new insert happens a message is sent to connected users.

It uses partitioned tables per day which allows the deletion your previous messages in a performant way by dropping the physical tables of this partitioned table. Tables older than 3 days old are deleted.

Broadcasting from the database works like a client-side broadcast, using WebSockets to send JSON packages. [Realtime Authorization](https://supabase.com/docs/guides/realtime/authorization) is required and enabled by default to protect your data.

The database broadcast feature provides two functions to help you send messages:

- `realtime.send` will insert a message into realtime.messages without a specific format.
- `realtime.broadcast_changes` will insert a message with the required fields to emit database changes to clients. This helps you set up triggers on your tables to emit changes.

### Broadcasting a message from your database<a href="https://supabase.com/docs/guides/realtime/broadcast#broadcasting-a-message-from-your-database" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

The `realtime.send` function provides the most flexibility by allowing you to broadcast messages from your database without a specific format. This allows you to use database broadcast for messages that aren't necessarily tied to the shape of a Postgres row change.

``` flex
123456SELECT realtime.send ( '{}'::jsonb, -- JSONB Payload   'event', -- Event name  'topic', -- Topic   FALSE -- Public / Private flag);
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

### Broadcast record changes<a href="https://supabase.com/docs/guides/realtime/broadcast#broadcast-record-changes" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

#### Setup realtime authorization<a href="https://supabase.com/docs/guides/realtime/broadcast#setup-realtime-authorization" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Realtime Authorization is required and enabled by default. To allow your users to listen to messages from topics, create a RLS (Row Level Security) policy:

``` flex
12345CREATE POLICY "authenticated can receive broadcasts"ON "realtime"."messages"FOR SELECTTO authenticatedUSING ( true );
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

See the [Realtime Authorization](https://supabase.com/docs/guides/realtime/authorization) docs to learn how to set up more specific policies.

#### Set up trigger function<a href="https://supabase.com/docs/guides/realtime/broadcast#set-up-trigger-function" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

First, set up a trigger function that uses `realtime.broadcast_changes` to insert an event whenever it is triggered. The event is set up to include data on the schema, table, operation, and field changes that triggered it.

For this example use case, we want to have a topic with the name `topic:<record id>` to which we're going to broadcast events.

``` flex
1234567891011121314151617CREATE OR REPLACE FUNCTION public.your_table_changes()RETURNS triggerSECURITY DEFINER SET search_path = ''AS $$BEGIN    PERFORM realtime.broadcast_changes(     'topic:' || NEW.id::text,   -- topic         TG_OP,                          -- event        TG_OP,                          -- operation        TG_TABLE_NAME,                  -- table        TG_TABLE_SCHEMA,                -- schema           NEW,                            -- new record           OLD                             -- old record        );    RETURN NULL;END;$$ LANGUAGE plpgsql;
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

Of note are the Postgres native trigger special variables used:

- `TG_OP` - the operation that triggered the function
- `TG_TABLE_NAME` - the table that caused the trigger
- `TG_TABLE_SCHEMA` - the schema of the table that caused the trigger invocation
- `NEW` - the record after the change
- `OLD` - the record before the change

You can read more about them in this [guide](https://www.postgresql.org/docs/current/plpgsql-trigger.html#PLPGSQL-DML-TRIGGER).

#### Set up trigger<a href="https://supabase.com/docs/guides/realtime/broadcast#set-up-trigger" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Next, set up a trigger so the function runs whenever your target table has a change.

``` flex
1234CREATE TRIGGER broadcast_changes_for_your_table_triggerAFTER INSERT OR UPDATE OR DELETE ON public.your_tableFOR EACH ROWEXECUTE FUNCTION your_table_changes ();
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

As you can see, it will be broadcasting all operations so our users will receive events when records are inserted, updated or deleted from `public.your_table` .

#### Listen on client side<a href="https://supabase.com/docs/guides/realtime/broadcast#listen-on-client-side" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Finally, client side will requires to be set up to listen to the topic `topic:<record id>` to receive the events.

``` flex
12345678const gameId = 'id'await supabase.realtime.setAuth() // Needed for Realtime Authorizationconst changes = supabase  .channel(`topic:${gameId}`)  .on('broadcast', { event: 'INSERT' }, (payload) => console.log(payload))  .on('broadcast', { event: 'UPDATE' }, (payload) => console.log(payload))  .on('broadcast', { event: 'DELETE' }, (payload) => console.log(payload))  .subscribe()
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

## Broadcast replay<a href="https://supabase.com/docs/guides/realtime/broadcast#broadcast-replay" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

<img src="out_realtime/broadcast/index_media/6da381d878739d1e834db715446f01b35a34b568.svg" class="w-6 h-6" />

This feature is currently in Public Alpha. If you have any issues [submit a support ticket](https://supabase.help).

### How it works<a href="https://supabase.com/docs/guides/realtime/broadcast#how-it-works" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

Broadcast Replay enables **private** channels to access messages that were sent earlier. Only messages published via [Broadcast From the Database](https://supabase.com/docs/guides/realtime/broadcast#broadcast-from-the-database) are available for replay.

You can configure replay with the following options:

- **`since`** (Required): The epoch timestamp in milliseconds (e.g., `1697472000000`), specifying the earliest point from which messages should be retrieved.
- **`limit`** (Optional): The number of messages to return. This must be a positive integer, with a maximum value of 25.

JavaScript

Dart

Swift

Kotlin

Python

<img src="out_realtime/broadcast/index_media/0f7fc93538e91ade401009300159ebc5054e285d.svg" class="w-6 h-6" />

This is currently available only in the Supabase JavaScript client version 2.74.0 and later.

``` flex
123456789101112131415161718192021const config = {  private: true,  broadcast: {    replay: {      since: 1697472000000, // Unix timestamp in milliseconds      limit: 10    }  }}const channel = supabase.channel('main:room', { config })// Broadcast callback receives meta fieldchannel.on('broadcast', { event: 'position' }, (payload) => {  if (payload?.meta?.replayed) {    console.log('Replayed message: ', payload)  } else {    console.log('This is a new message', payload)  }  // ...}).subscribe()
```

<img src="out_realtime/broadcast/index_media/d9c4b7add2b04096741724568af26673b4f446a9.svg" class="lucide lucide-copy text-lighter" />

#### When to use Broadcast replay<a href="https://supabase.com/docs/guides/realtime/broadcast#when-to-use-broadcast-replay" class="ml-2 opacity-0 group-hover:opacity-100 transition" aria-hidden="true">#</a>

A few common use cases for Broadcast Replay include:

- Displaying the most recent messages from a chat room
- Loading the last events that happened during a sports event
- Ensuring users always see the latest events after a page reload or network interruption
- Highlighting the most recent sections that changed in a web page

<a href="https://github.com/supabase/supabase/blob/master/apps/docs/content/guides/realtime/broadcast.mdx" class="w-fit flex items-center gap-1 text-sm text-scale-1000 hover:text-scale-1200 transition-colors" rel="noreferrer noopener edit" target="_blank">Edit this page on GitHub <img src="out_realtime/broadcast/index_media/3ee4434fa0936084ada9f9535617163941b979af.svg" class="lucide lucide-external-link" /></a>

### Is this helpful?

<img src="out_realtime/broadcast/index_media/82954652faa5c1357b3f1fbee5560e79717901b0.svg" class="lucide lucide-x text-current" />No

<img src="out_realtime/broadcast/index_media/a1e9c777d4da09bddca000e120dc325dadfad412.svg" class="lucide lucide-check" />Yes

- <img src="out_realtime/broadcast/index_media/fc3b667770ecf58c18a57c9707eeb4ef5cdb7b79.svg" class="lucide lucide-life-buoy" />

  Need some help?

  <a href="https://supabase.com/support" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Contact support</a>

- <img src="out_realtime/broadcast/index_media/6a48d9bf47efc6f6cc9f9198a5d9bd82a4eb61d0.svg" class="lucide lucide-flask-conical" />

  Latest product updates?

  <a href="https://supabase.com/changelog" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">See Changelog</a>

- <img src="out_realtime/broadcast/index_media/dca6f8c719c0f568123d76d384df2e5f750a66c1.svg" class="lucide lucide-circle-check-big" />

  Something's not right?

  <a href="https://status.supabase.com/" class="text-brand-link hover:underline" rel="noreferrer noopener" target="_blank">Check system status</a>

------------------------------------------------------------------------

<a href="https://supabase.com/" class="text-xs text-foreground-lighter">© Supabase Inc</a>—<a href="https://github.com/supabase/supabase/blob/master/apps/docs/DEVELOPERS.md" class="text-xs text-foreground-lighter hover:underline">Contributing</a><a href="https://github.com/supabase/supabase/blob/master/apps/docs/CONTRIBUTING.md" class="text-xs text-foreground-lighter hover:underline">Author Styleguide</a><a href="https://supabase.com/open-source" class="text-xs text-foreground-lighter hover:underline">Open Source</a><a href="https://supabase.com/supasquad" class="text-xs text-foreground-lighter hover:underline">SupaSquad</a>

Privacy Settings

<a href="https://github.com/supabase/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">GitHub<img src="out_realtime/broadcast/index_media/6506f47eb1cac63154be703b9bea8227d8f97784.svg" /></a><a href="https://twitter.com/supabase" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Twitter<img src="out_realtime/broadcast/index_media/a14586df53b1ba626e64e260906d5d432bb45ac0.svg" /></a><a href="https://discord.supabase.com/" class="relative justify-center cursor-pointer inline-flex items-center space-x-2 text-center font-regular ease-out duration-200 rounded-md outline-none transition-all outline-0 focus-visible:outline-4 focus-visible:outline-offset-1 border text-foreground hover:bg-surface-300 shadow-none focus-visible:outline-border-strong data-[state=open]:bg-surface-300 data-[state=open]:outline-border-strong border-transparent text-xs px-2.5 py-1 h-[26px]" data-size="tiny" rel="noreferrer noopener" target="_blank" type="button">Discord<img src="out_realtime/broadcast/index_media/aa0c9be8f4e5ebd1a79bf4ab7db7c857ba11e057.svg" /></a>
