    Hacker Newsnew | past | comments | ask | show | jobs | submit	login

Supabase Storage now supports the S3 protocol (supabase.com)
501 points by inian on April 19, 2024 | hide | past | favorite | 187 comments

kiwicopple on April 19, 2024 | next [–]

hey hn, supabase ceo here
For background: we have a storage product for large files (like photos, videos, etc). The storage paths are mapped into your Postgres database so that you can create per-user access rules (using Postgres RLS)

This update adds S3 compatibility, which means that you can use it with thousands of tools that already support the protocol.

I'm also pretty excited about the possibilities for data scientists/engineers. We can do neat things like dump postgres tables in to Storage (parquet) and you can connect DuckDB/Clickhouse directly to them. We have a few ideas that we'll experiment with to make this easy

Let us know if you have any questions - the engineers will also monitor the discussion

devjume on April 19, 2024 | prev | next [–]

This is great news. Now I can utilize any CDN provider that supports S3. Like bunny.net [1] which has image optimization, just like Supabase does but with better pricing and features.
I have been developing with Supabase past two months. I would say there are still some rough corners in general and some basic features missing. Example Supabase storage has no direct support for metadata [2][3].

Overall I like the launch week and development they are doing. But more attention to basic features and little details would be needed because implementing workarounds for basic stuff is not ideal.

[1] https://bunny.net/ [2] https://github.com/orgs/supabase/discussions/5479 [3] https://github.com/supabase/storage/issues/439

kiwicopple on April 19, 2024 | parent | next [–]

> I can utilize any CDN provider that supports S3. Like bunny.net
> Bunny is a great product. I'm glad this release makes that possible for you and I imagine this was one of the reasons the rest of the community wanted it too

> But more attention to basic features and little details

This is what we spend most of our time doing, but you won't hear about it because they aren't HN-worthy.

> no direct support for metadata

Fabrizio tells me this is next on the list. I understand it's frustrating, but there is a workaround - store metadata in the postgres database (I know, not ideal but still usable). We're getting through requests as fast as we can.

fenos on April 19, 2024 | root | parent | next [–]

This is indeed at the very top of the list :)

giancarlostoro on April 20, 2024 | parent | prev | next [–]

I've not done a whole lot with S3 but is this due to it being easy to sync between storage providers that support S3 or something?
I'm more used to Azure Blob Storage than anything, so I'm OOL on what people do other than store files on S3.

inian on April 19, 2024 | prev | next [–]

Here is the example of the DuckDB querying parquet files directly from Storage because it supports the S3 protocol now - https://github.com/TylerHillery/supabase-storage-duckdb-demo
https://www.youtube.com/watch?v=diL00ZZ-q50

cmollis on April 19, 2024 | parent | next [–]

Yes. Duckdb works very well with parquet scans on s3 right now.

iamcreasy on April 21, 2024 | root | parent | next [–]

Does it work well with Hive tables storing parquet files on s3?

Rapzid on April 19, 2024 | prev | next [–]

I like to Lob my BLOBs into PG's storage. You need that 1-2TB of RDS storage for the IOPS anyway; might as well fill it up.
Large object crew, who's with me?!

vbezhenar on April 20, 2024 | parent | next [–]

I don't. S3-compatible storages usually are significantly cheaper, allow to offload HTTP requests. Also huge databases make backups and recoveries slow.
The only upside of storing blobs in the database is transactional semantics. Buf if you're fine with some theoretical trash in S3, that's trivially implemented with proper ordering.

abraae on April 20, 2024 | root | parent | next [–]

> The only upside of storing blobs in the database is transactional semantics.
> Plenty more advantages than that. E.g for SaaS you can deep copy an entire tenant including their digital assets. Much easier copying with just "insert into ... Select from" than having to copy S3 objects.

mdaniel on April 20, 2024 | root | parent | next [–]

Or the opposite side of that, deletion. Pruning 38TB worth of S3 would be an unholy exercise unless the keys are 1TB each. I am aware of Lifecycle Policies, but they are a gargantuan PITA, IMHO

iamcreasy on April 21, 2024 | root | parent | prev | next [–]

What do you mean by transactional semantics?

_boffin_ on April 22, 2024 | root | parent | next [–]

ACID, I presume

dymk on April 19, 2024 | parent | prev | next [–]

38TB of large objects stored in Postgres right here

Rapzid on April 19, 2024 | root | parent | next [–]

A hero appears!
The client I use currently, npgsql, supports proper streaming so I've created a FS->BLOB->PG storage abstraction. Streamy, dreamy goodness. McStreamy.

iamcreasy on April 21, 2024 | root | parent | next [–]

I am confused and curious. Can you please elaborate on how streaming is related to pg storage?

Rapzid on April 21, 2024 | root | parent | next [–]

If you can't stream data the user is sending through the server to the database you have to buffer the entire payload in memory. This doesn't scale well but can work for smaller payloads.
You CAN stream bytea but:

- Good luck finding a client that supports it(npgsql does actually)

- It doesn't support support start the read/write at any location other than the start. So you'd have to basically build the Lob(Large Object) functionality yourself on top to support upload resume and other use cases.

With Lob even if your client doesn't support streaming, you can mimick it by writing chunks into the Lob.

iamcreasy on April 23, 2024 | root | parent | next [–]

What kind of streaming from user are we talking about? User interaction, such as mouse move, clicks?
Does each interaction essentially create a SQL update statement? And are you trying to avoid buffering those statements?

kiwicopple on April 19, 2024 | parent | prev | next [–]

that's not how this works. files are stored in s3, metadata in postgres

Rapzid on April 19, 2024 | root | parent | next [–]

Sad.
J/K. It could be a really good back-end option for Supabase's S3 front end. A lot of PG clients don't support proper "streaming" and looking at the codebase it's TypeScript.. postgres.js is the only client nearing "performant" I'm aware of(last I looked) on Node.js but it's not clear it supports streaming outside "Copy" per the docs. Support could be added to the client if missing.

Edit: Actually it could be a good option for your normal uploads too. Docs talk about it being ideal for 6Mb or smaller files? Are you using bytea or otherwise needing to buffer the full upload/download in memory? Streaming with Lob would resolve that, and you can compute incremental hash sums for etags and etc. Lob has downsides and limitations but for a very large number of people it has many great benefits that can carry them very far and potentially all the way.

tln on April 19, 2024 | root | parent | prev | next [–]

Will the files get deleted with ON CASCADE DELETE somehow? That would be awesome.
Then for GDPR, when you delete a user, the associated storage can be deleted.

One could cobble this together with triggers, some kind of external process, and probably repetititious code so there is one table of metadata per "owning" id, although it would be nice to be packaged.

inian on April 20, 2024 | root | parent | next [–]

We have discussed this internally before since we have seen some users delete the metadata in the storage schema and expect the underlying object to be deleted too and if we should convert our entire storage server to just be a Postgres extension.
The source of truth also matters here - if it's the database or the underlying s3 bucket. I think having the underlying storage bucket to be the source of truth would be more useful. In that scenario we would sync the metadata in the database to match what's actually being stored and if we notice metadata of a object missing, we add that in as opposed to deleting the object in storage. This would make it easier for you to bring in your own s3 bucket with existing data and attach it to Supabase storage.

hobs on April 20, 2024 | root | parent | next [–]

This falls in line with how SQL Server did its FileStream stuff, but it was so clunky nobody used it except for some madmen.

Rapzid on April 20, 2024 | root | parent | prev | next [–]

That's an interesting concept. Of course you don't want to delete your record of the blob until the blog is deleted.. A trigger could add a "job" to delete the blob into another table when the file record is deleted though..

code_biologist on April 19, 2024 | parent | prev | next [–]

Lol. The most PG blob storage I've used in prod was a couple hundred GB. It was a hack and the performance wasn't ideal, but the alternatives were more complicated. Simple is good.

Rapzid on April 19, 2024 | root | parent | next [–]

Yeah, it's a great place to start. I took the time to implement streaming reads/write via npgsql's client support for it (it can stream records, and of course the Lob storage is broken into page sized rows) and performance is pretty darn good.

yoavm on April 19, 2024 | prev | next [–]

This looks great! How easy is it to self host Supabase? Is it more like "we're open-source, but good luck getting this deployed!", or can someone really build on Supabase and if things get a little too expensive it's easy enough to self-host the whole thing and just switch over? I wonder if people are doing that.

kiwicopple on April 19, 2024 | parent | next [–]

self-hosting docs are here: https://supabase.com/docs/guides/self-hosting/docker
And a 5-min demo video with Digital Ocean: https://www.youtube.com/watch?v=FqiQKRKsfZE&embeds_referring...

Anyone who is familiar with basic server management skills will have no problem self-hosting. every tool in the supabase stack[0] is a docker image and works in isolation. If you just want to use this Storage Engine, it's on docker-hub (supabase/storage-api). Example with MinIO: https://github.com/supabase/storage/blob/master/docker-compo...

[0] architecture: https://supabase.com/docs/guides/getting-started/architectur...

replwoacause on April 20, 2024 | root | parent | next [–]

I was pretty unhappy with the self hosted offering. It’s neutered compared to the cloud, which was disappointing.

kiwicopple on April 20, 2024 | root | parent | next [–]

> neutered
> Can you share more? There is nothing missing, it’s all the same code that we run in production (besides some login changes to keep it decoupled from the platform)

replwoacause on April 20, 2024 | root | parent | next [–]

Sure. I wouldn't quite agree with you that "there is nothing missing" though.
https://github.com/orgs/supabase/discussions/4444 https://github.com/orgs/supabase/discussions/4421

kiwicopple on April 20, 2024 | root | parent | next [–]

thanks for the links

> https://github.com/orgs/supabase/discussions/4444

This discussion is about configuration: self-hosters configure configure their services in code[0], using .env vars / docker-compose.yml and should be checking it into source control. Self-hosters are not missing any functionality - the opposite actually: we have a subset of configuration items available on the platform

I get that it's different to the platform, but it's (IMO) the more "correct" way - all configuration should be version controlled

> https://github.com/orgs/supabase/discussions/4421

This one is about Auth providers. They are also configuration: https://github.com/supabase/auth?tab=readme-ov-file#configur...

[0] Configuration in code: https://supabase.com/docs/guides/self-hosting/docker#configu...

[1] Terraform: https://supabase.com/docs/guides/platform/terraform

kiwicopple on April 21, 2024 | root | parent | next [–]

(FYI: I saw a deleted comment that this requires a restart: the same is true on the platform)

tmountain on April 20, 2024 | root | parent | prev | next [–]

I believe that the self hosted version is missing auth hooks.
https://www.reddit.com/r/Supabase/comments/1bpaq9w/comment/k...

kiwicopple on April 21, 2024 | root | parent | next [–]

i believe the docker image ID on the docker-compose hasn’t been updated to the latest stable version. I’ll make a PR later when I’m at my computer (or anyone else is welcome to and I can review/merge)

tmountain on April 21, 2024 | root | parent | next [–]

Awesome, thanks for following up and for making a great product!

kiwicopple on April 22, 2024 | root | parent | next [–]

I checked with the team - it was already in the self-hosted version.
There are some guides for using this in the CLI docs:

https://supabase.com/docs/guides/auth/auth-hooks#local-devel...

(I'll find a way to consolidate these docs so the work across CLI + docker-compose)

asciii on April 21, 2024 | root | parent | prev | next [–]

Following...

ihateolives on April 20, 2024 | root | parent | prev | next [–]

From top of my head: reports and multiple projects. Also I kept getting weird "fetch failed" errors when creating rules or adding users from gui working behind reverse proxy so I gave up on the end.

zipping1549 on April 20, 2024 | parent | prev | next [–]

Some may disagree but in my experience Supabase was definitely challenging to selfhost. Don't get me wrong; I'm pretty confident with selfhosting but Supabase was definitely on the hard side.
Pocketbase being literally single-binary doesn't make Supabase look good either, although funtionalities differ.

kiwicopple on April 22, 2024 | root | parent | next [–]

Yes, we have a vastly different architecture[0] from Pocketbase. We choose individual tools based on their scaling characteristics and give you the flexibility to add/remove tools as you see fit.
I doubt we can ever squeeze the "supabase stack" into a single binary. This undoubtedly makes things more difficult for self-hosters. Just self-hosting Postgres can be a challenge for many. We will trying to make it easier, but it will never be as simple as Pocketbase.

[0] https://supabase.com/docs/guides/getting-started/architectur...

brap on April 19, 2024 | prev | next [–]

Always thought it’s kind of odd how the proprietary API of AWS S3 became sort of the de-facto industry standard

bdcravens on April 19, 2024 | parent | next [–]

S3 is one of the original AWS services (SQS predates it), and has been around for 18 years.
The idea of a propriety API becoming the industry defacto standard isn't uncommon. The same thing happened with Microsoft's XMLHttpRequest.

crubier on April 19, 2024 | root | parent | next [–]

Also, the S3 API is simple and makes sense, no need to reinvent something different just for the pleasure of it

ovaistariq on April 20, 2024 | root | parent | next [–]

There are valid reasons for extending and redoing some parts of the API. I will give you one example. Suppose you want to extend list objects to support ordering by last modified or you want to support filtering of objects by user metadata. Right now doing this is quite clunky via headers.

ovaistariq on April 20, 2024 | parent | prev | next [–]

Supporting an existing API provides interoperability which is beneficial for the users. So that way if there is a better storage service it’s easier to adopt it. However, the S3 API compatibility can be a hindrance when you want to innovate and provide additional features and functionality. In our case, providing additional features [1] [2] while continuing to be S3 API compatible has forced us to rely on custom headers.
[1] https://www.tigrisdata.com/docs/objects/conditionals/ [2] https://www.tigrisdata.com/docs/objects/caching/#caching-on-...

garbanz0 on April 19, 2024 | parent | prev | next [–]

Same thing seems to be happening with openai api

mmcwilliams on April 19, 2024 | parent | prev | next [–]

I might be misremembering this but I was under the impression that Ceph offered the same or very similar object storage API prior to Amazon building S3.

mousetree on April 19, 2024 | parent | prev | next [–]

Because that's where most of the industry store their data.

brap on April 19, 2024 | root | parent | next [–]

Yeah I understand how it came to be, it’s just kind of an uncommon situation

moduspol on April 19, 2024 | parent | prev | next [–]

Yeah--though I guess kudos to AWS for not being litigious about it.

jimmySixDOF on April 19, 2024 | prev | next [–]

Supabase also announced this week Oriole (the team not just the table storage plugin) is joining them so I guess this is part of the same story. Anyway it's nice timing I was thinking about a hookup to Cloudflare R2 for something and this may be the way.

kiwicopple on April 19, 2024 | parent | next [–]

Oriole are joining to work on the OrioleDB postgres extension. That's slightly different to this release:

- This: for managing large files in s3 (videos, images, etc).

- Oriole: a postgres extension that's a "drop-in replacement" for the default storage engine

We also hope that the team can help develop Pluggable Storage in Postgres with the rest of the community. From the blog post[0]:

> Pluggable Storage gives developers the ability to use different storage engines for different tables within the same database. This system is available in MySQL, which uses the InnoDB as the default storage engine since MySQL 5.5 (replacing MyISAM). Oriole aims to be a drop-in replacement for Postgres' default storage engine and supports similar use-cases with improved performance. Other storage engines, to name a few possibilities, could implement columnar storage for OLAP workloads, highly compressed timeseries storage for event data, or compressed storage for minimizing disk usage.

Tangentially: we have a working prototype for decoupled storage and compute using the Oriole extension (also in the blog post). This stores Postgres data in s3 and there could be some inter-play with this release in the future

[0] https://supabase.com/blog/supabase-aquires-oriole

kabes on April 20, 2024 | root | parent | next [–]

What's the point of acquiring them instead of just sponsering the project? I'm trying to understand supabase's angle here and if this is good or bad news for non-supabase postgres users.

kiwicopple on April 21, 2024 | root | parent | next [–]

We sponsored/invested in them previously (over half a million USD). The benefit of acquiring is that the team can focus on the technology rather than the business/operations around it.
I hope this is the best outcome for the industry. If there are other postgres companies that would like to use oriole, feel free to reach out. It’s 100% open source and we’d love more design partners

jonplackett on April 19, 2024 | prev | next [–]

Dear supabase. Please don’t get bought out by anyone and ruined. I’ve built too many websites with a supabase backend now to go back.

kiwicopple on April 19, 2024 | parent | next [–]

we don't have any plans to get bought.
we only have plans to keep pushing open standards/tools - hopefully we have enough of a track record here that it doesn't feel like lip service

mort96 on April 19, 2024 | root | parent | next [–]

Is it even up to you? Isn't it up to your Board of Directors (i.e investors) in the end?

kiwicopple on April 19, 2024 | root | parent | next [–]

we (the founders) control the board

jacob_rezi on April 19, 2024 | root | parent | prev | next [–]

Who controls the board, controls the company

asciii on April 21, 2024 | root | parent | next [–]

the data must flow!

jerrygenser on April 19, 2024 | root | parent | prev | next [–]

I can believe there are no plans, right now. But having raised over $100mm, the VCs will want to liquidate their holdings eventually. They have to answer to their LPs after all, and be able to raise their next funds.
The primary options I can think of that are not full acquisition are: - company buys back stock - VC sells on secondary market - IPO

The much more common and more likely option for these VCs to make the multiple or home run on their return is going to be to 10x+ their money by having a first or second tier cloud provider buy you.

I think there's a 10% chance that a deal with Google is done in the future, so their portfolio has Firebase for NoSQL and Firebase for SQL.

jchanimal on April 19, 2024 | root | parent | next [–]

Having founded a database company that IPO'd (Couchbase) and seeing the kinds of customer relationships Supabase is making, an IPO seems a reasonable outcome.

spxneo on April 19, 2024 | root | parent | prev | next [–]

well before there was supabase I would use Firebase
so it would serve Google well if they matched what supabase is doing or bought them out

jonplackett on April 19, 2024 | root | parent | prev | next [–]

Absolutely. I am so impressed with SB. It’s like you read my mind and then make what I’ll need before I realise.
(This is not a paid promotion)

kiwicopple on April 19, 2024 | root | parent | next [–]

> like you read my mind
> we receive a lot of community feedback and ultimately there are only a few "primitives" developers need to solve 90% of their problems

I think we're inching closer to the complete set of primitives which can be used to combine into second-order primitives (eg: queues/search/maps can all be "wrappers" around the primitives we already provide)

jonplackett on April 19, 2024 | root | parent | next [–]

That's a neat way of thinking about it.
Thanks for an awesome product. Please also never get bought or make plans to in the future, or if you really, really, really have to then please not by google.

robertlagrant on April 19, 2024 | root | parent | prev | next [–]

As long as you make it so if you do get bought a team of you can always fork and move on, it's about the best anyone can hope for.

tap-snap-or-nap on April 19, 2024 | root | parent | prev | next [–]

plans\*
\*Subject to change

gherkinnn on April 19, 2024 | parent | prev | next [–]

This is my biggest reservation towards Supabase. Google bought Firebase in 2014. I've seen Vercel run Nextjs in to the ground and fuck up their pricing for some short-term gains. And Figma almost got bought by Adobe. I have a hard time trusting products with heavy VC backing.

MatthiasPortzel on April 19, 2024 | root | parent | next [–]

I’m not defending Vercel or VC backed companies per se, but I don’t understand your comments towards Vercel. They still offer a generous hobby plan and Next.js is still actively maintained open source software that supports self-hosting.
Heroku might be a better example of a company that was acquired and the shut down their free plan.

theturtletalks on April 19, 2024 | root | parent | next [–]

Supabase and Next.js are not the same. Supabase can be self-hosted but it’s not easy to do so and a lot of moving parts. Most of my Next.js apps are not even on Vercel. They can easily be deployed to Netlify, Railway, Render, etc.

ihateolives on April 19, 2024 | root | parent | next [–]

Supabase self-hosting is not difficult, but it makes no sense since some important features are missing, like reports and multiple projects.

spxneo on April 19, 2024 | root | parent | prev | next [–]

You know the whole point of YC companies is to flip their equity on the public market right and then moving on to the next one?

superb_dev on April 19, 2024 | root | parent | next [–]

We know, but it screws over your existing customers when a very helpful tool is turned over to a greedy investment firm that’s gonna gut the product seeking the highest return

paradite on April 19, 2024 | root | parent | prev | next [–]

I think Vercel and Next.js are built by the same group of people, the same people who made Now.sh, started the company (Zeit), then changed product name to Now.sh, then changed company and product name to Vercel.

cqqxo4zV46cp on April 19, 2024 | root | parent | next [–]

Yes. That doesn’t mean that they haven’t ran it into the ground.

paradite on April 19, 2024 | root | parent | next [–]

Tell me about it.
My simple SSG Next.js static site loads much slower than my Jekyll site on GitHub pages.

And I can't figure out how to improve its speed or disable the "ISG" feature that I believe is to be blamed for the poor performance.

eropple on April 19, 2024 | root | parent | next [–]

Not defending NextJS, I'm pretty out on it myself, but ISG requires a server to run. It pregenerates static content for defined pages and serves that until revalidating. If you've built a fully static bundle, nothing exists that would handle that incremental/revalidating logic.

paradite on April 19, 2024 | root | parent | next [–]

I understand that ISG requires a server (Node.js runtime) to run it, that's why I want to disable it for my SSG Next.js static site, but I can't figure out how. I just want static hosting like S3+Cloudfront which is much faster.

theturtletalks on April 19, 2024 | root | parent | next [–]

You need to use static export:
https://nextjs.org/docs/app/building-your-application/deploy...

paradite on April 19, 2024 | root | parent | next [–]

Cool, so that's what I was missing all along!
Unfortunately looks like I can't use it now since I am using `next-plausible` which does require a Node.js proxy...

spxneo on April 19, 2024 | root | parent | prev | next [–]

big reason on why I decided on Flutter

quest88 on April 19, 2024 | root | parent | prev | next [–]

What's the actual complaint here , other then company A buys company B.

rrr_oh_man on April 19, 2024 | root | parent | next [–]

Buy & Kill
https://insights.som.yale.edu/insights/wave-of-acquisitions-...

spxneo on April 19, 2024 | root | parent | prev | next [–]

the only reason i am using supabase is that its cheap and i can export it to postgres, thats it.
i know that one day these founders will exit and it will be sold to AWS, Google or Microsoft cloud.

so its a bit of a gamble but that risk is offset by its super cheap cost and ability to export out the data and swap out the pieces (more work but at that point you should be cashflow positive to pay engineers to do it for you).

hacker_88 on April 19, 2024 | root | parent | prev | next [–]

Wasn't there Parse from firebase

josephd79 on April 20, 2024 | root | parent | next [–]

Parse was bought by Facebook and not much longer shut it down right?

hacker_88 on April 20, 2024 | root | parent | next [–]

I am confused , there was also Fabric.io bought by Twitter and then put into Firebase by Google .

zackmorris on April 19, 2024 | root | parent | prev | next [–]

Firebase was such a terrible loss. I had been following it quite closely on its mailing list until the Google takeover, then it seemed like progress slowed to a halt. Also having big brother watching a common bootstrap framework's data like that, used by countless MVP apps, doesn't exactly inspire confidence, but that's of course why they bought it.
At the time, the most requested feature was a push notification mechanism, because implementing that on iOS had a steep learning curve and was not cross-platform. Then probably some more advanced rules to be able to do more functional-style permissions, possibly with variables, although they had just rolled out an upgraded rules syntax. And also having a symlink metaphor for nodes might have been nice, so that subtrees could reflect changes to others like a spreadsheet, for auto-normalization without duplicate logic. And they hadn't yet implemented an incremental/diff mechanism to only download what's needed at app startup, so larger databases could be slow to load. I don't remember if writes were durable enough to survive driving through a tunnel and relaunching the app while disconnected from the internet either. I'm going from memory and am surely forgetting something.

Does anyone know if any/all of the issues have been implemented/fixed yet? I'd bet money that the more obvious ones from a first-principles approach have not, because ensh!ttification. Nobody's at the wheel to implement these things, and of course there's no budget for them anyway, because the trillions of dollars go to bowing to advertisers or training AI or whatnot.

IMHO the one true mature web database will be distributed via something like Raft, have rich access rules, be log based with (at least) SQL/HTTP/JSON interfaces to the last-known state and access to the underlying sets selection/filtering/aggregation logic/language, support nested transactions or have all equivalent use-cases provided by atomic operations with examples, be fully indexed by default with no penalty for row or column based queries (to support both table and document-oriented patterns and even software transactional memories - STMs), have column and possibly even row views (not just table views), use a copy-on-write mechanism internally like Clojure's STM for mostly O(1) speed, be evented with smart merge conflicts to avoid excessive callbacks, preferable with a synchronized clock timestamp ordered lexicographically:

https://firebase.blog/posts/2015/02/the-2120-ways-to-ensure-...

I'm not even sure that the newest UUID formats get that right:

https://uuid6.github.io/uuid6-ietf-draft/

Loosely this next-gen web database would be ACID enough for business and realtime enough for gaming, probably through an immediate event callback for dead reckoning, with an optional "final" argument to know when the data has reached consensus and was committed, with visibility based on the rules system. Basically as fast as Redis but durable.

A runner up was the now (possibly) defunct RethinkDB. Also PouchDB/PouchBase, a web interface for CouchDB.

I haven't had time to play with Supabase yet, so any insights into whether it can do these things would be much appreciated!

codextremist on April 19, 2024 | parent | prev | next [–]

Never used Supabase before but I'm very much comfortable with their underlying stack. I use a combination of postgres, PostgREST, PLv8 and Auth0 to achieve nearly the same thing.

pbronez on April 19, 2024 | root | parent | next [–]

I was unfamiliar with PLv8, found this:
“”” PLV8 is a trusted Javascript language extension for PostgreSQL. It can be used for stored procedures, triggers, etc.

PLV8 works with most versions of Postgres, but works best with 13 and above, including 14, [15], and 16. “””

https://plv8.github.io/

foerster on April 19, 2024 | parent | prev | next [–]

I'm a bit terrified of this as well. I have built a profitable product on the platform, and it were to drastically change or go away, I'd be hosed.

nextaccountic on April 19, 2024 | prev | next [–]

A question about implementation, is the data really stored in a Postgres database? Do you support transactional updates like atomically updating two files at once?
Is there a Postgres storage backend optimized for storing large files?

fenos on April 19, 2024 | parent | next [–]

We do not store the files in Postgres, the files are stored in a managed S3 bucket.
We store the metadata of the objects and buckets in Postgres so that you can easily query it with SQL. You can also implement access control with RLS to allow access to certain resources.

It is not currently possible to guarantee atomicity on 2 different file uploads since each file is uploaded on a single request, this seems a more high-level functionality that could be implemented at the application level

nextaccountic on April 19, 2024 | root | parent | next [–]

Oh.
So this is like, S3 on top of S3? That's interesting.

fenos on April 19, 2024 | root | parent | next [–]

Yes indeed! I would call it S3 on steroids!
Currently, it happens to be S3 to S3, but you could write an adapter, let's say GoogleCloudStorage and it will become S3 -> GoogleCloudStorage, or any other type of underline Storage.

Additionally, we provide a special way of authenticating to Supabase S3 using the SessionToken, which allows you to scope S3 operations to your users specific access control

https://supabase.com/docs/guides/storage/s3/authentication#s...

pelagicAustral on April 19, 2024 | root | parent | next [–]

What about for second tier cloud providers like Linode, Vultr or UpCloud, they all offer S3 compatible object storage, will I need to write an adaptor for these or will it work just fine in lieu of their S3 compatibility?

fenos on April 19, 2024 | root | parent | next [–]

Our S3 Driver is compatible with any S3 Compatible object Storage so you don’t have to write one :)

SOLAR_FIELDS on April 19, 2024 | root | parent | next [–]

Gentle reminder here that S3 compatability is a sliding window and without further couching of the term it’s more of a marketing term than anything for vendors. What do I mean by this statement? I mean that you can go to cloud vendor Foo and they can tell you they offer s3 compatible api’s or clients but then you find out they only support the most basic of operations, like 30% of the API. Vendor Bar might support 50% of the api and Baz 80%.
In a lot of cases, if your use case is simple, 30% is enough if you’re doing the most common GET and PUT operations etc. But all it takes is one unsupported call in your desired workflow to rule out that vendor as an option until such time that said API is supported. My main beef with this is that there’s no easy way to tell usually unless the vendor provides a support matrix that you have to map to the operations you need, like this: https://docs.storj.io/dcs/api/s3/s3-compatibility. If no such matrix is provided on both the client side and server side you have no easy way to tell if it will even work without wiring things in and attempting to actually execute the code.

One thing to note is that it’s quite unrealistic for vendors to strive for 100% compat - there’s some AWS specific stuff in the API that will basically never be relevant for anyone other than AWS. But the current situation of Wild West could stand for some significant improvement

inian on April 20, 2024 | root | parent | next [–]

I agree that s3 compatibility is a bit of a moving target and we would not implement any of the AWS specific actions.
We are transparent with what's the level of compatibility - https://supabase.com/docs/guides/storage/s3/compatibility

The most often used APIs are covered but if something is missing, let me know!

pbronez on April 19, 2024 | root | parent | prev | next [–]

I’m confused about what directions this goes.
The announcement is that Supabase now supports (user) —s3 protocol—> (Supabase)

Above you say that (Supabase) —Supabase S3 Driver—> (AWS S3)

Are you further saying that that (Supabase) —Supabase S3 Driver—> (any S3 compatible storage provider) ? If so, how does the user configure that?

It seems more likely that you mean that for any application with the architecture (user) —s3 protocol—> (any S3 compatible storage provider), Supabase can now be swapped in as that storage target.

nemothekid on April 19, 2024 | root | parent | next [–]

As I understand it, supabase is open source. So the hosted supabase is
(user) -> s3 protocol -> (Supabase) -> (AWS S3)

you could fork (or contribute) a database driver for any s3 compatible backend of choice.

(user) -> s3 protocl -> (pbronez-base) -> (GCP Cloud Storage)

kiwicopple on April 19, 2024 | root | parent | prev | next [–]

in case it's not clear why this is required, some of the things the storage engine handles are:
image transformations, caching, automatic cache-busting, multiple protocols, metadata management, postgres compatibility, multipart uploads, compatibility across storage backends, etc

Havoc on April 20, 2024 | prev | next [–]

What are the chances of Supabase doing a license change? Seems to be fashionable these days so always a little wary of building on these sort of platforms

ezekg on April 20, 2024 | parent | next [–]

Seeing as Neon, Nile, Citus, etc. are all open source, I highly doubt it. But who knows. In the end, most license changes are blown out of proportion by a vocal minority and largely have zero effect on 99.9% of users.

kiwicopple on April 22, 2024 | parent | prev | next [–]

Unfortunately there isn't a way to prove our intentions to remain open source
That said, I think Supabase is much more de-risked from this happening because we aim to support existing tools with a strong preference of tools that are controlled by foundations rather than commercial entities. For example, 2 of the main tools:

- Postgres (PostgreSQL license)

- PostgREST (MIT license)

Every tool/library/extension that we develop and release ourselves is either MIT, Apache2, or PostgreSQL

Havoc on April 22, 2024 | root | parent | next [–]

Thanks for the response. Seems reasonable

madsbuch on April 19, 2024 | prev | next [–]

This is really nice to see! I asked about that feature almost 2 years ago as we wanted to use Supabase for everything. Unfortunately there were no plans back then to support it, so we had to use another provider for object storage.
Congrats on the release!

pull_my_finger on April 20, 2024 | prev | next [–]

Is there a formal s3 protocol spec or do these companies try to reverse engineer/feature match what AWS provides?

starttoaster on April 20, 2024 | parent | next [–]

The S3 API isn't unknown or anything, the client library SDKs all being open source. So I'd imagine a software developer writing a tool that aims to be S3 API-compliant would use one of the open source SDKs, and write their API while making requests to it locally through a client from one of the Amazon SDKs. Not trivial effort, but also pretty easy to imagine how you'd start off. If a client function from the SDK doesn't work with your API, you write your API handler to handle the HTTP request that function makes until it is supported.
I have wondered if Amazon has some additional tooling for other software providers to make their own S3-compliant APIs, but I don't know what Amazon's motivation would be to help make it easier for people to switch between other vendors. Whereas the incentive is much more obvious for other software vendors to make their own APIs S3-compliant. So I've so far imagined it is a similar process to how I described above, instead.

inian on April 21, 2024 | parent | prev | next [–]

The S3 API reference [1] is closest to a formal spec there is. The request, response and the error codes are pretty well documented.
[1]: https://docs.aws.amazon.com/AmazonS3/latest/API/API_Operatio...

snthpy on April 21, 2024 | root | parent | next [–]

Thanks for this. Do you know how are redirects handled?
I searched on that page and didn't find anything but I've seen it mentioned elsewhere that they are catered for but I haven't found any documentation for that.

I'm particularly interested in the temporary ones - 302, 303 and 307.

swyx on April 21, 2024 | root | parent | prev | next [–]

perhaps you could open source the test suite as a standalone thing so that other s3 compatible apis can prove they match the supabase standard

avodonosov on April 19, 2024 | prev | next [–]

The article does not mention: do you support pre-signed URLs?
https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-...

fenos on April 19, 2024 | parent | next [–]

Thanks for asking this, we do not support signed URLs just yet, but it will be added in the next iteration

avodonosov on April 19, 2024 | root | parent | next [–]

Presigned URLs are useful because client app can upload/download directly from S3, saving the app server from this traffic. Does Row-Level Security achieve the same benefit?

fenos on April 19, 2024 | root | parent | next [–]

Yes, I agree! Although I should have specified that we support signed URL https://supabase.com/docs/reference/javascript/storage-from-... just not in the S3 protocol just yet :)

dmattia on April 19, 2024 | root | parent | next [–]

Thanks for confirming! Could you maybe update https://supabase.com/docs/guides/storage/s3/compatibility to include s3-esque presigning as a feature to track?

arxpoetica on April 19, 2024 | root | parent | prev | next [–]

Can you give an indication of what "next iteration" means in terms of timeline (even just ballpark)?

JoshTriplett on April 19, 2024 | prev | next [–]

You specifically say "for large files". What's your bandwidth and latency like for small files (e.g. 20-20480 bytes), and how does it compare to raw S3's bandwidth and latency for small files?

egorr on April 19, 2024 | parent | next [–]

hey, supabase engineer here; we didn’t check that out with files that small, but thanks for the idea, i will try it out
the only thing i can say related to the topic is that s3 multipart outperforms other methods for files larger than 50mb significantly, but tends to have similar or slightly slower speeds compared to s3 regular upload via supabase or simplest supabase storage upload for files with size about and less than 50mb.

s3-multipart is indeed the fastest way to upload file to supabase with speeds up to 100mb/s(115 even) for files>500mb. But for files about 5mb or less you are not going to need to change anything in your upload logic just for performance cause you won’t notice any difference probably

everything mentioned here is for upload only

fenos on April 19, 2024 | parent | prev | next [–]

You can think of the Storage product as an upload server that sits in front of S3.
Generally, you would want to place an upload server to accept uploads from your customers, that is because you want to do some sort of file validation, access control or anything else once the file is uploaded. The nice thing is that we run Storage within the same AWS network, so the upload latency is as small as it can be.

In terms of serving files, we provide a CDN out-of-the-box for any files that you upload to Storage, minimising latencies geographically

Rapzid on April 19, 2024 | root | parent | next [–]

> Generally, you would want to place an upload server to accept uploads from your customers
> A common pattern on AWS is to not handle the upload on your own servers. Checks are made ahead of time, conditions baked into the signed URL, and processing is handled after the fact via bucket events.

fenos on April 19, 2024 | root | parent | next [–]

That is also a common pattern I agree, both ways are fine if the upload server is optimised accordingly :)

ovaistariq on April 20, 2024 | parent | prev | next [–]

S3 is not known for performing well with such small files. Is this primarily how your dataset on S3 looks like?

kaliqt on April 19, 2024 | prev | next [–]

I tried to migrate from Firebase once and it wasn't really straightforward and decided against doing it, I think you guys (if you haven't already) should make migration plugins a first class priority that "just works" as the amount of real revenue generating production projects on Firebase and similar are of a much higher number. It's a no-brainer that many of them may want to switch if it were safe and simple to do so.

kiwicopple on April 19, 2024 | parent | next [–]

we have some guides (eg: https://supabase.com/docs/guides/resources/migrating-to-supa...)
also some community tools: https://github.com/supabase-community/firebase-to-supabase

we often help companies migrating from firebase to supabase - usually they want to take advantage of Postgres with similar tooling.

gime_tree_fiddy on April 19, 2024 | prev | next [–]

Shouldn't it be API rather than protocol?
Also my sympathies for having to support the so-called "S3 standard/protocol".

fenos on April 19, 2024 | parent | next [–]

Yes, both can be fine :) after all, a Protocol can be interpreted as a Standardised API which the client and server interact with, it can be low-level or high-level.
I hope you like the addition and we have the implementation all open-source on the Supabase Storage server

preommr on April 19, 2024 | parent | prev | next [–]

I think that protocol is appropriate here since s3 resources are often represented by a s3:// url where the scheme part of the url is often used to represent the protocol.

ovaistariq on April 20, 2024 | root | parent | next [–]

S3 uses HTTP protocol for communication between client and server. The `s3://` paths make it seem like it’s a protocol but that’s just a way to represent the path on the s3 client to differentiate it from a local file path.

spacebanana7 on April 19, 2024 | prev | next [–]

I wish supabase had more default integrations with CDNs, transactional email services and domain registrars.
I'd happily pay a 50% markup for the sake of having everything in one place.

tootie on April 19, 2024 | parent | next [–]

At the same time, I worry about them drifting too far from their core mission. I think Vercel and Netlify kinda went that way and when you look at their suite of features, you just have to ask why would I not just use AWS directly.

kiwicopple on April 19, 2024 | parent | prev | next [–]

are there any specific integrations that you want? (eg: which companies?)
we have a built-in CDN[0] and we have some existing integrations for transactional emails [1]

[0] Smart CDN: https://supabase.com/docs/guides/storage/cdn/smart-cdn

[1] Integrations: https://supabase.com/partners/integrations

spacebanana7 on April 19, 2024 | root | parent | next [–]

Thanks for the link about integrations! I wasn't aware about the resend one for transactional email.
It'd be nice to have an integration with a domain register, like Ghandi.net or Namecheap. Ideally with the cost coming through as an item in my Supabase bill.

zenorocha on April 19, 2024 | root | parent | next [–]

Resend founder here - feel free to try out the integration with Supabase and let me know if you have any questions

kiwicopple on April 19, 2024 | root | parent | prev | next [–]

Thanks for the recommendations - I’ll see what this would involve

simonbarker87 on April 19, 2024 | prev | next [–]

Supabase is great and I’ve used it for a number of projects over the years both with a backend alongside it or direct from the client with RLS.
There are some weird edges (well really just faff) around auth with the JS library but if nothing else they are by far the cheapest hosted SQL offering I can find so any faff you don’t want to deal with there’s an excellent database right there to allow you to roll your own (assuming you have a backend server alongside it)

animeshjain on April 19, 2024 | prev | next [–]

Is there any request pricing (I could not find a mention to it on the pricing page). Could be quite compelling for some use-cases if request pricing is free.

inian on April 20, 2024 | parent | next [–]

There is no per request pricing.

fswd on April 20, 2024 | prev | next [–]

I just finished implementing S3 file upload in nextjs to cloudflare R2 with a supabase backend. Wish I had been lazy and waited a day!

tootie on April 19, 2024 | prev | next [–]

One of the big wins we get from AWS is that you can do things like load structured data files (csv, parquet) from S3 directly in Redshift using SQL queries.
https://docs.aws.amazon.com/redshift/latest/dg/t_loading-tab...

inian on April 21, 2024 | parent | next [–]

This is indeed pretty cool. They also have the `aws_s3` extension [1] for doing the same thing inside Postgres. Unfortunately, the extension isn't open source.
[1]: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_...

kiwicopple on April 22, 2024 | root | parent | next [–]

we might be able to achieve the same thing now with our S3 Wrapper:
https://supabase.github.io/wrappers/s3/

mritchie712 on April 19, 2024 | prev | next [–]

Very cool and very good timing. We just released better support to directly query S3[0] in Definite[1]. It's powered by duckdb.
0 - https://www.youtube.com/watch?v=yrrCQnfKEig

1 - https://www.definite.app/

jarpineh on April 19, 2024 | prev | next [–]

Hi, a question, but first some background. I've been looking at solutions to store columnar data with versioning, essentially Parquet. But, I'd also like to store PDFs, CSVs, images, and such for our ML workflows. I wonder if now, that Supabase is getting better for data science DuckDB crowd, could Supabase be that one solution for all this?

kiwicopple on April 19, 2024 | parent | next [–]

> Parquet. But, I'd also like to store PDFs, CSVs, images
> yes, you can store all of these in Supabase Storage and it will probably "just work" with the tools that you already use (since most tools are s3-compatible)

Here is an example of one of our Data Engineers querying parquet with DuckDB: https://www.youtube.com/watch?v=diL00ZZ-q50

We're very open to feedback here - if you find any rough edges let us know and we can work on it (github issues are easiest)

jarpineh on April 19, 2024 | root | parent | next [–]

Well, this is great news. I'll take "just works" guarantee any day ;)
We have yet to make a commitment to any one product. Having Postgres there is a big plus for me. I'll have to see about doing a test or two.

Bnjoroge on April 19, 2024 | parent | prev | next [–]

you should look at lance(https://lancedb.github.io/lance/)

filleokus on April 19, 2024 | prev | next [–]

Do you think Supabase Storage (now or in the future) could be an attractive standalone S3 provider as an alternative to e.g MinIO?

kiwicopple on April 19, 2024 | parent | next [–]

It's more of a "accessibility layer" on top of S3 or any other s3-compatible backend (which means that it also works with MinIO out-of-the-box [0])
I don't think we'll ever build the underlying storage layer. I'm a big fan of what the Tigris[1] team have built if you're looking for other good s3 alternatives

[0] https://github.com/supabase/storage/blob/master/docker-compo...

[1] Tigris: https://tigrisdata.com

masfuerte on April 19, 2024 | root | parent | next [–]

I'm still confused. Who is paying Amazon? Is it

1. You buy the storage from Amazon so I pay you and don't interact with Amazon at all or

2. I buy storage from Amazon and you provide managed access to it?

kiwicopple on April 22, 2024 | root | parent | next [–]

it's 1 - you pay us, we pay AWS.
In the future we will likely provide "bring your own s3" (2)

ovaistariq on April 20, 2024 | root | parent | prev | next [–]

I appreciate the callout to Tigris. Now it’s super easy to use Tigris with Supabase for all those files.

mattgreenrocks on April 19, 2024 | prev | next [–]

Just commenting to say I really appreciate your business model. Whereas most businesses actively seek to build moats to maintain competitive advantage and locking people in, actions like this paint a different picture of Supabase. I'll be swapping out the API my app uses for Supabase storage to switch it to an S3 API this weekend in case I ever need to switch it.
My only real qualm at this point is mapping JS entities using the JS DB API makes it hard to use camelcase field names due to PG reasons I can't recall. I'm not sure what a fix for that would look like.

Keep up the good work.

kiwicopple on April 20, 2024 | parent | next [–]

> mapping JS entities using the JS DB API makes it hard to use camelcase field names due to PG reasons I can't recall
> Postgres requires you to "quoteCamelCase". There are some good JS libs to map between snake case and camel case. FWIW, I like a mix of both in my code: snake_case indicates it's a database property, and camelCase indicates its a JS property. Try it out - it might grow on you :)

ntry01 on April 19, 2024 | prev | next [–]

This is great news, and I agree with everyone in the thread - Supabase is a great product.
Does this mean that Supabase (via S3 protocol) supports file download streaming using an API now?

As far as I know, it was not achievable before and the only solution was to create a signed URL and stream using HTTP.

fenos on April 19, 2024 | parent | next [–]

Yes, absolutely! You can download files as streams and make use of Range requests too.
The good news is that the Standard API is also supporting stream!

foerster on April 19, 2024 | prev | next [–]

no feedback on this in particular, but I love supabase. I use it for several projects and it's been great.
I was hesitant to use triggers and PG functions initially, but after I got my migrations sorted out, it's been pretty awesome.

SOLAR_FIELDS on April 19, 2024 | parent | next [–]

Do you manage your functions and triggers through source code? What framework do you use to do that? I like Supabase but it’s desire to default to native pg stuff for a lot of that has kind of steered me away from using it for more complex projects where you need to use sprocs to retrieve data and pgtap to test them, because hiding away business logic in the db like that is viewed as an anti pattern in a lot of organizations. I love it for simple CRUD apps though, the kind where the default postgrest functionality is mostly enough and having to drop into a sproc or build a view is rarely necessary.
I think if there was a tightly integrated framework for managing the state of all of these various triggers, views, functions and sproc through source and integrating them into the normal SDLC it would be a more appealing sell for complex projects

inian on April 21, 2024 | root | parent | next [–]

The Supabase CLI [1] provides a way for you to manage functions, triggers and anything else in your Postgres database as a migration. These migrations would be checked into your source control.
You can then take it a step further but opting-in to use Branching [2] to better manage environments. We just opened up the Branching feature to everyone [3].

[1]: https://supabase.com/docs/guides/cli/local-development#datab... [2]: https://supabase.com/docs/guides/platform/branching [3]: https://supabase.com/blog/branching-publicly-available

SOLAR_FIELDS on April 21, 2024 | root | parent | next [–]

Thanks for the response. I don’t think I was super clear about what I meant - I’m more talking about the following scenario:
Let’s say that we are using the Typescript sdk to make our app and need to do some fancy aggregation on a table that isn’t supported by Postgrest natively (specifically we can’t retrieve the data with Postgrest out of the box with its typical setup). Postgrest tells us that in this case we can do two things: create a view or make a Postgres function. Each has their pros and cons, but with either choice now we have this problem: some of our business logic is in sproc/function/view and some of it is in Typescript. In a typical setup using an ORM it would all be in Typescript.

The conventional wisdom is that db’s are dumb state holders and all of the business logic goes in the app - Supabase attempts to turn this on its head and say no actually it’s ok to store business logic in the db. But now if we do that we have a new problem: we don’t have a blessed path forward for where the line is on what goes where anymore. We don’t have good patterns for storing and managing this so other developers understand where to put things and how to work with our app anymore, because it no longer holds the principle of least astonishment. That’s what I mean by framework in this context.

Maybe all that is necessary here is a battle tested example project that demonstrates the “correct” way to make this demarcation. But as-is it steers me away from using Supabase for more complex projects if I even think they will need something that Postgrest won’t support without making a view or sproc/function

pier25 on April 20, 2024 | prev | next [–]

At $0.1/GB of egress it’s not super attractive compared to B2 or R2 for anything but trivial projects.
I wish they would offer a plan with just the pg database.

Any news on pricing of Fly PG?

inian on April 20, 2024 | parent | next [–]

We are hosted on aws and are just passing the cost over to our users. We make no margin on egress fees. Deploying storage on other clouds including Fly.io is planned.
We are actively working on our Fly integration. At the start, the pricing is going to be exactly the same as our hosted platform on aws - https://supabase.com/docs/guides/platform/fly-postgres#prici...

pier25 on April 20, 2024 | root | parent | next [–]

Thanks. So a user that only wants pg has to pay for storage etc?

sgt on April 19, 2024 | prev | next [–]

Can Supabase host static content yet (in a decent way)?

fenos on April 19, 2024 | parent | next [–]

We don’t support static website hosting just yet - might happen in the future :)

kiwicopple on April 19, 2024 | root | parent | next [–]

just a small elaboration:
You can serve HTML content if you have a custom domain enabled. We don't plan to do anything beyond that - there are already some great platforms for website hosting, and we have enough problems to solve with Postgres

sgt on April 19, 2024 | root | parent | prev | next [–]

That would be cool, at least for someone building a small app with just one tiny SaaS bill to worry about. Even better; using the free tier for an app that is a very very small niche.

iamcreasy on April 19, 2024 | prev | next [–]

In this setup, can postgresql query data stored in object storage? i.e. Hive/Icebarg table.

WhitneyLand on April 19, 2024 | prev | next [–]

Is iOS support a priority for supabase?

kiwicopple on April 19, 2024 | parent | next [–]

Yes, we made it an official library this week: https://supabase.com/blog/supabase-swift

poxrud on April 19, 2024 | prev | next [–]

Do you support S3 event notifications?

inian on April 21, 2024 | parent | next [–]

We don't support S3 event notifications directly, but you achieve similar functionality by using Database Webhooks [1]. You can trigger any HTTP endpoint or a Supabase Edge function by adding a trigger to the objects table [3] in the Storage schema.
[1]: https://supabase.com/docs/guides/database/webhooks [2]: https://supabase.com/docs/guides/functions [3]: https://supabase.com/docs/guides/storage/schema/design

bdcravens on April 19, 2024 | parent | prev | next [–]

"S3 protocol" typically refers to object storage read/write/delete, not additional service APIs. Support in other S3-compatible vendors varies, often with a different payload (though a translation wrapper shouldn't be too difficult to implement)
https://developers.cloudflare.com/r2/buckets/event-notificat...

https://docs.digitalocean.com/reference/api/spaces-api/

withinboredom on April 19, 2024 | prev | next [–]

Now we just need flutterflow to get off the firebase bandwagon.

kiwicopple on April 19, 2024 | parent | next [–]

(supabase team member) Firebase is an amazing tool for building fast. I want Supabase to be a "tool for everyone", but ultimately giving developers choices between various technologies is a good thing for developers. I think it's great that Flutterflow support both Firebase & Supabase.
I know Flutterflow's Firebase integration is a bit more polished so hopefully we can work closer with the FF team to make our integration more seamless

denysvitali on April 19, 2024 | prev | next [–]

Friendly reminder that Supabase is really cool, and if you haven't tried it out you should do it (everything can be self hosted and they have generous free tiers!)

isoprophlex on April 19, 2024 | parent | next [–]

Plus, with all the inflated vc money fueled hype on vector databases, they seem to have the only offering in this space that actually makes sense to me. With them you can store your embeddings close to all the rest of your data, in a single postgres db.

BoorishBears on April 19, 2024 | root | parent | next [–]

pgVector is literally everywhere: https://github.com/pgvector/pgvector/issues/54

kiwicopple on April 19, 2024 | parent | prev | next [–]

thanks for taking time to make a comment. it's nice to get some kind words between the feedback (which we also like)

joshxyz on April 21, 2024 | parent | prev | next [–]

their team is crazy good

stephen37 on April 19, 2024 | prev [–]

Cool to see that Supabase is adding S3 protocol! Nice to see more and more storage solutions available.
We, at Milvus, I've integrated S3, Parquet and other ones to make it possible for developers to use their data no matter what they use.

For those who have used both, how do you find the performance and ease of integration compares between Supabase and other solutions like Milvus that have had these features for some time?

teaearlgraycold on April 19, 2024 | parent | next [–]

Just make your own post if you’re selling your product

jongjong on April 21, 2024 | parent | prev [–]

Nice. Will check out Milvus.
