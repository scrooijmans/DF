Ugh, I think I've just solved this for myself. I'm using react for my frontend, so this is maybe not going to work for you folks. In React, Strict Mode mounts and unmounts components twice and I suppose this was causing the conflict for the websocket (quickly tearing down the first connection, to call for a second one - and the first one being closed already causing the error) - after not enabling Strict Mode the error was fixed for me (for now).

---

Anyone else struggling with Supabase Realtime reliability in Next.js?
realtime
I'm building a restaurant ordering system and I'm having serious reliability issues with Supabase Realtime. I'm selfhosted on Coolify, version: image: 'supabase/studio:2025.06.02-sha-8f2993d' . The connection is extremely unreliable - it works for a while, then suddenly stops receiving new orders, which is obviously critical for a restaurant. For user order tracking perspective same behaviour.

The pattern:

Yesterday: It was still partially working yesterday, for example

Today: constant WebSocket connection failures right after someone places an order

Error messages I'm getting:

the connection to wss://myurl.com/realtime/v1/websocket?apikey=... was interrupted while the page was loading
Firefox can't establish a connection to the server at wss://myurl.com/realtime/v1/websocket?apikey=...
The connection to wss://myurl.com/realtime/v1/websocket?apikey=... was interrupted while the page was loading
Current behavior:

Max 1 update comes through after page reload, then same error again

Same issue on mobile Chrome

Happens across different browsers/devices

I seeing the updates if I connect to the channel via Supabase user interface

My code (simplified):

useEffect(() => {
const channel = supabase.channel(`realtime_orders_channel`);

const subscribeToChannel = () => {
channel
.on(
'postgres_changes',
{
event: '\*',
schema: 'public',
table: 'order',
filter: `restaurant_id=eq.${restaurantID}`,
},
(payload) => {
console.log('Change received, refreshing page...', payload);
router.refresh();
}
).subscribe();
};

// ... rest of the logic
}, []);
Questions:

Is anyone else experiencing similar reliability issues with Supabase Realtime?

For production restaurant systems, should I be looking at alternatives?

Any proven patterns for handling these WebSocket interruptions?

Any help or shared experiences would be greatly appreciated! 🙏

Also dealing with connection drops when browser goes to background/device sleeps - is this a known limitation?

Upvote
4

Downvote

18
Go to comments

Share

Report
Report
u/knutmt avatar
knutmt
•
Promoted

codehooks.io - Build and deploy an API before breakfast
Elegant and simple APIs for your backend needs
Query and Manage data easily
Manage settings of your backend
See deployed endpoints and client code examples
View live logs from the backend
Manage project settings, invite collaborators

codehooks.io
Learn More
Share your thoughts
Sort by:

Best

Search Comments
Expand comment search
Comments Section
u/Saladtoes avatar
Saladtoes
•
5mo ago
Definitely do not recommend using Postgres changes. Nothing but trouble. One extra line of code in your creation logic (or use a trigger) to use broadcast instead. Pretty sure they officially recommend this now.

As for the connectivity issues, anything weird about your network setup (office firewalls, adblockers, pinhole, anything?)? Looks like you’re saying you have a custom domain, are you paying for it and have it set up on supabase? Not really sure how self hosting works in coolify, but lots of VPS providers will close connections that aren’t chatting. So maybe send pings to keep connection open or something.

Are you using SSR? Sometimes SSR interjects itself in annoying and mysterious ways.

You will be fighting WebSocket suspension from the client side forever. Locking, switching tabs, minimizing - all good reasons for a browser to pause connections/background work. If you need real persistent connections for long running background work, you’re in the wrong place. Every device and browser can be different. If you as a developer could force this behavior, my phone battery would last like 5 minutes with my 106 tabs open. Unless you are in desktop kiosk mode with a device that will never lock or minimize, I wouldn’t hope for this to work.

I would evaluate if you truly need real-time. In my experience, five second polling is just fine for most things. And that’s literally just using tanstack query and setting the refresh interval. It will recover from tab switching and device sleep every time. If I was a restaurant taking orders, I don’t think I would ever think that a potential five second delay was anything but real time.

Upvote
10

Downvote

Reply

Award

Share

u/xGanbattex avatar
xGanbattex
OP
•
4mo ago
Thanks so much for the tips I understand the problem much better now.
I'm hosting my VPS on Hetzner, and I'm running Supabase via Coolify, mapped to a custom domain.
Coolify uses Traefik (default) and Caddy proxies.

As for the project, yes, I'm currently using SSR for data fetching (if that’s what you meant). Since there's not much interactivity, I fetch the data on the page level and use router.refresh() when there are changes.
I saw this technique in an older Supabase video.

I also tried managing it via state, where the payload would be added directly to the state without needing extra fetches, but I had the same issue there too. So I stuck with the SSR + refresh method since it's simpler and more readable from a code perspective.

Basically, I’m facing two main issues:

After ordering a dish, like I mentioned earlier, the realtime connection often drops. From the moment the order is submitted, the user only get one update at most, and only for around 3–5 seconds. Two days ago this didn’t happen at all, yesterday it did, and today it’s working again — so it's inconsistent. I even thought maybe it’s a bandwidth limit issue on my side, but that’s not the case. Also the VPS has enough memory etc...

The other main problem (which you also pointed out) is that even when realtime does work, it randomly disconnects, even when the screen is on and the device is active.

I wish there were better resources explaining how Supabase Realtime actually works under the hood in real projects, because everywhere I look I just find basic todo app examples.
It’s also frustrating that it doesn’t auto-reconnect when the connection drops, I’d expect that behavior by default.

I’m wondering: how do people use this in production?

Thanks a lot for the TanStack tip too! I’ve just added the broadcast logic to my code and I’ll test it in the coming days.
I’ve also added a realtime connection status indicator to the site, so now it’s easier to see if/when a manual refresh is needed.
(I didn’t make the auto-refresh fully automatic, since if reconnection fails, it would cause an infinite reload loop.)

If this approach doesn’t work, I’ll fall back to using TanStack entirely.

Upvote
2

Downvote

Reply

Award

Share

u/Saladtoes avatar
Saladtoes
•
4mo ago
In production, I host on supabase and it hasn’t given me any issues. I use broadcast with approximately 1/s updates of a 1-5KB JSON payload. Only maybe 150 users, with about 30 different streams. When I used Postgres changes, it was really bad. Like messages would be delayed, then come in all at once, but out of order. I think it’s just the nature of WAL/replication. Broadcast never skips a beat.

If you want to know how it works under the hood, you’re in luck because it’s open source. https://github.com/supabase/realtime

Upvote
2

Downvote

Reply

Award

Share

u/xGanbattex avatar
xGanbattex
OP
•
4mo ago
Thank you!

Upvote
1

Downvote

Reply

Award

Share

exclaim_bot
•
4mo ago
Thank you!

You're welcome!

Upvote
1

Downvote

Reply

Award

Share

u/xGanbattex avatar
xGanbattex
OP
•
4mo ago
So in the end, I gave up on Broadcast too and went with React Query today. Unfortunately, Broadcast also was just as unreliable for me.

What really pushed me over the edge was that it often didn’t even connect to the channel, even after refreshing the browser on both my PC and phone, the same issue persisted. And then, when I tried it from a different browser, like Firefox, it connected without any problem. It had worked the day before in Chrome on the same devices, but the next day, it was broken again.

When it didn’t connect, I checked my server load, which was nearly zero, and there was still plenty of room for more realtime connections.

This whole thing is just incomprehensible to me, but I’ve had enough of it, next time if I want to use realtime I will definitely go with CONVEX.

Upvote
1

Downvote

Reply

Award

Share

u/Saladtoes avatar
Saladtoes
•
4mo ago
Very odd. Still sounds to me like your VPS is killing off your websocket connections. No real evidence for that, this just smells like a network issue to me.

If you’re interesting in playing more with a totally different solution, check out NATS. It’s like if MQTT and RabbitMQ had a baby that was super portable, supports websockets, and runs in the browser. Still might be overkill for your app (REST/database is just too good), but I have had really good luck with it. It support real-time streaming with “eventual consistency”, resilient to network outages, disconnections, etc. The NATS server can keep track of ensuring every client eventually receives every message, even if offline for hours, days, whatever. And it is extremely portable and easy to deploy on any target.
