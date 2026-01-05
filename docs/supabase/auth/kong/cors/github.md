Title: Supabase CORS Error on REST API Requests: No 'Access-Control-Allow-Origin' header is present · Issue #7627 · supabase/supabase · GitHub

Description: Bug report Describe the bug I am using the Supabase SaaS service and supabase-js for my latest web application that I intend to launch soon. I am experiencing a CORS issue for new user accounts of my app, whereby a new user account recei...              

Skip to content  

You signed in with another tab or window. Reload to refresh your session. You signed out in another tab or window. Reload to refresh your session. You switched accounts on another tab or window. Reload to refresh your session. Dismiss alert

supabase / **supabase** Public

*   ### Uh oh!

There was an error while loading. Please reload this page.

*   Notifications You must be signed in to change notification settings
*   Fork 9.8k
*   Star 88.6k

Supabase CORS Error on REST API Requests: No 'Access-Control-Allow-Origin' header is present #7627
==================================================================================================

New issue

Copy link

New issue

Copy link

Closed

Closed

Supabase CORS Error on REST API Requests: No 'Access-Control-Allow-Origin' header is present#7627

Copy link

Labels

bugSomething isn't workingSomething isn't workingwontfixThis will not be worked onThis will not be worked on

Description
-----------

caleb531

opened on Jul 8, 2022

Issue body actions

Bug report
==========

Describe the bug
----------------

I am using the Supabase SaaS service and supabase-js for my latest web application that I intend to launch soon. I am experiencing a CORS issue for new user accounts of my app, whereby a new user account receives a 400 CORS response code when calling my public DB tables via the REST API.

Upon further inspection, it appears that the `access-control-allow-origin` header is present on the OPTIONS response, but _not_ the GET/POST response. Conversely, for users where requests have completed successfully, the `access-control-*` headers are present on both preflight and non-preflight requests. This issue currently applies to both GET and POST requests.

The interesting thing is, once the CORS issue occurs for a new user account I just created, the CORS errors will persist for the lifetime of that user account. I've tried clearing cookies, localStorage, and even switched browsers—nothing resolves the CORS issue for an affected user.

If I clear browsing data and log into a different user I created previously, the 400 CORS errors go away while I am signed into that user account.

It's an issue I have been unable to reproduce locally, and I haven't been able to draw any inferences from my `auth.users` table on https://app.supabase.com/.

Expected behavior
-----------------

The GET/POST request should include CORS headers, and said requests should complete successfully for all users. I suspect this is an issue with Supabase platform's server-side, as I cannot control their response headers otherwise.

Screenshots
-----------

See above.

System information
------------------

*   OS: macOS Monterey 12.4
*   Browser: Tested on Google Chrome 103.0.5060.114 and Brave browser (v1.40.113 Chromium: 103.0.5060.114)
*   Version of supabase-js: v1.35.4
*   Version of Node.js: v16 (hosted on Vercel)

Additional context
------------------

I've checked my API Logs in my Supabase control panel, but they don't reveal much.

👍React with 👍1AshkanArabim

Metadata
--------

Metadata
--------

### Assignees

No one assigned

### Labels

bugSomething isn't workingSomething isn't workingwontfixThis will not be worked onThis will not be worked on

### Type

No type

### Projects

No projects

### Milestone

No milestone

### Relationships

None yet

### Development

No branches or pull requests

Issue actions
-------------

You can’t perform that action at this time.

**************************************************************************************************
CORs error when calling a deployed edge function in supabase from the browser
Asked 2 years, 3 months ago
Modified 1 year ago
Viewed 1k times
0

I have an edge function deployed with the following code:

import { serve } from "server"
import * as cryptojs from "crypto-js";

import { config } from "dotenv";

// Load the environment variables from .env file
await config();

const corsHeaders = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Headers': 'authorization, x-client-info, apikey, content-type',
}

serve(async (req) => {
  // This is needed if you're planning to invoke your function from a browser.
  if (req.method === 'OPTIONS') {
    return new Response('ok', { headers: corsHeaders })
  }

  try {

    ...
    ... 
    ...

    return new Response(
      JSON.stringify(data), {
        status: 200,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
      }
    )
  } catch (error) {
    return new Response(
      JSON.stringify({ error: error.message || error.toString() }), {
        status: 500,
        headers: { ...corsHeaders, 'Content-Type': 'application/json' },
      }
    );
  }
})
As you can see, corsHeaders is applied to enable invocation from the browser which is called using the supabase libary:

const response = await supabase.functions.invoke(
                "retrieve-payment-link", {
                    body: {
                        id: payment_link_id,
                    }
                }
            );
and was deployed with the following command:  npx supabase functions deploy retrieve-payment-link --import-map supabase/functions/import_map.json and yet I'm still having this error Cross-Origin Request Blocked: The Same Origin Policy disallows reading the remote resource. Running in local doesn't have any issues, only in production. How do I fix this?

javascriptsupabasedeno
Share
Improve this question
Follow
edited Jun 12, 2023 at 3:21
asked Jun 12, 2023 at 2:45
neil_ruaro's user avatar
neil_ruaro
52611 gold badge88 silver badges2929 bronze badges
I think there is an error in production that prevents this code from loading. A dependency problem most likely. Have you seen the logs? Can you add a log statement? – 
yxre
 CommentedJun 12, 2023 at 2:57
please add the exact error you are receiving - I suspect your request will result in requiring an actual host in 'Access-Control-Allow-Origin' (i.e. not *) - but it's not clear from the code as I can't see what the error states – 
Jaromanda X
 CommentedJun 12, 2023 at 3:08 
On the frontend side I'm getting this Cross-Origin Request Blocked: The Same Origin Policy disallows reading the remote resource. Upon checking logs, I'm getting this on invocation: CPU execution time exceeded – 
neil_ruaro
 CommentedJun 12, 2023 at 3:18 
please add the exact error you are receiving to the question - where it should've been in the first place :p – 
Jaromanda X
 CommentedJun 12, 2023 at 3:20
Do you want to remove all the code inside try except for the return statement so that you can isolate the issue? From the CPU execution time exceeded error message, it seems like your function execution is taking some time. – 
dshukertjr
 CommentedJun 12, 2023 at 6:04
Add a comment
2 Answers
Sorted by:

Highest score (default)
0

In local, the Docker Containers were not updated after the Supabase CLI update. Plase run supabase stop and supabase start to restart and download the containers again.

Share
Improve this answer
Follow
answered Jul 17, 2023 at 15:13
Asier Alcaide's user avatar
Asier Alcaide
311 bronze badge
1
Your answer could be improved with additional supporting information. Please edit to add further details, such as citations or documentation, so that others can confirm that your answer is correct. You can find more information on how to write good answers in the help center. – 
Community
Bot
 CommentedJul 21, 2023 at 14:34
Add a comment
0

I encountered that issue recently as well. I resolved it by setting the headers on the backend like this:

    Deno.serve(async (req) => {
    
        const headers: Record<string, string> = {
            "Content-Type": "application/json",
        };

        headers["Access-Control-Allow-Origin"] = "*";
        headers["Access-Control-Allow-Methods"] =
            "GET, POST, PUT, DELETE, OPTIONS";
        headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization";
        headers["Access-Control-Allow-Credentials"] = "true";

        if (req.method === "OPTIONS") {
            // Handle preflight requests
            return new Response(null, { status: 204, headers });
        } 
    
// Remaining code

    return new Response(
            JSON.stringify({ data}),
            {
                headers,
            },
        );

}
From the frontend side, I didn't use Supabase.functions.invoke because it consistently throws a CORS error, which I suspect is a bug in Supabase. Instead, I made requests to the Supabase Edge function directly like this:

         let requestOptions = {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization:"Bearer "+ token,
          },
        };
    
        const options = {
        ...requestOptions,
        body: JSON.stringify({ amount }),
      };
      const res = await fetch(
        "edge_function_url",
        options
      );

  const extractData = await res.json();
I hope, this code will be helpful.
************************************************************************************************

getfitdotus

dealing with cors
I have a existing web app I created that uses react Dom JS. The backend is node. I implemented jwt with cookies for refresh tokens. Now I have started building a react native app version to use the same backend. My issue is dealing with cors. Anyone have any experience with this? Should I implement a proxy?

Is there something I am missing ?

I have a axios request like so.

  const response = await axios.post("https://xxx.xxxx.us:8000/login", data, {
    headers: {
      "Content-Type": "application/json",
     
    },
    
  });
tried fetch just to experiment same outcome.

get network error, null. now if I try it on a test api works no problem. But from insomnia I can get the proper response. Granted that host is in the cors origin. the backend does not even show the route was hit. never gets past cors I believe.

Thanks


Upvote
8

Downvote

24
Go to comments


Share

Report
Report
Comments Section
Single comment thread
See full discussion
u/archihector avatar
archihector
•
1y ago
But what if we are launching the react native project on web (browser)? I am doing that and is giving my, apparently, a CORS error. (I am using Supabase btw)

track me

Upvote
2

Downvote

Reply
reply

Award

Share

*****************************************
supa and cors, frustrating
This is my code, the entirety of it. I have spent hours trying to get passed the cors issue. I"ve installed the corps npm even. THis is all one page, I thought, that it would be more simple that way. I'm ust trying to insert some objects in supabase.



ERRORS


picture-in-picture.js:262 Fetch finished loading: GET "chrome-extension://jffbochibkahlbbmanpmndnhmeliecah/config.json".

(anonymous) @ picture-in-picture.js:262

Show 1 more frame

Show less

index.html:58 Uncaught ReferenceError: res is not defined

at HTMLFormElement.<anonymous> (index.html:58:2)

(anonymous) @ index.html:58

index.html:1 Access to fetch at 'https://lbntulezexltpxjbzwtd.supabase.co/books' from origin 'http://127.0.0.1:5500' has been blocked by CORS policy: No 'Access-Control-Allow-Origin' header is present on the requested resource. If an opaque response serves your needs, set the request's mode to 'no-cors' to fetch the resource with CORS disabled.

index.html:45

POST https://lbntulezexltpxjbzwtd.supabase.co/books net::ERR_FAILED 404 (Not Found)

(anonymous) @ index.html:45

index.html:45

Uncaught (in promise) TypeError: Failed to fetch

at HTMLFormElement.<anonymous> (index.html:45:2)

(anonymous) @ index.html:45

Promise.then (async)

(anonymous) @ index.html:57

index.html:45 Fetch failed loading: POST "https://lbntulezexltpxjbzwtd.supabase.co/books".



CODE



<!DOCTYPE html>

<html lang="en">

<head>

<meta charset="UTF-8">

<meta name="viewport" content="width=device-width, initial-scale=1.0">

<title>Books Table</title>

</head>

<body>

<form class="form">

<label for="title">Title:</label><br>

<input type="text" id="title" name="title"><br>

<label for="author">Author:</label><br>

<input type="text" id="author" name="author"><br>

<label for="genre">Genre:</label><br>

<input type="text" id="genre" name="genre"><br>

<button type="submit" class=".btn">Submit</button>

</form>

<script src="https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2"></script>

<script> const form1 = document.querySelector('.form'); form1.addEventListener('submit', event => { event.preventDefault(); const formData = new FormData(form1); const data = Object.fromEntries(formData); // const cors = require('cors'); // app.use(cors()); fetch('https://lbntulezexltpxjbzwtd.supabase.co/books', { method:'POST', headers: { "Access-Control-Allow-Credentials": "true", "Access-Control-Allow-Methods": "POST", "Access-Control-Allow-Origin": '\*', "Access-Control-Allow-Headers": 'authorization, x-client-info, apikey, content-type', "apikey": 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImxibnR1bGV6ZXhsdHB4amJ6d3RkIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDY0NjE0NTAsImV4cCI6MjAyMjAzNzQ1MH0.QTB6yIrvhDicxLQtEbyQi36rBfPSzMWtB8BltL2fSdY', "Content-Type": 'application/json' }, body: JSON.stringify(data) }).then (res => res.json()) res.setHeader ( { 'context-type': 'text/html', "Access-Control-Allow-Credentials": "true", "Access-Control-Allow-Origin": '\*', "Access-Control-Allow-Headers": 'authorization, x-client-info, apikey, content-type', "Access-Control-Allow-Methods": "POST" }); }); </script>

</body> </html>


Upvote
2

Downvote

23
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
Join the conversation
Sort by:

Best

Search Comments
Expand comment search
Comments Section
u/SideLow2446 avatar
SideLow2446
•
2y ago
I think the url should be should be ...supabase.co/rest/v1/books

track me


Upvote
2

Downvote

Reply
reply

Award

Share

BrendanH117
•
2y ago
This is probably your answer, OP ^ To expand on it, you can go to your dashboard > API Docs and pick Bash as your language. You'll see the full endpoint there https://imgur.com/a/lJtWuAr


Upvote
1

Downvote

Reply
reply

Award

Share

xTheBenjamin
•
2y ago
I'm not the most knowledgeable person on this topic but to clarify, you are trying to send the data to an edge function? Because if so, there may be some config needed on client-side and I do not know much about that but there must definitely be some code in the edge function that returns the cors header on an OPTIONS request. Therefore, if my assumtion is correct, please provide your edge function code or have a look at this: https://supabase.com/docs/guides/functions/cors



Upvote
1

Downvote

Reply
reply

Award

Share

darbokredshrirt
OP
•
2y ago
i dont know what an edge function is. i'm just trying to make a web app that will send data to my db table in supabase.



Upvote
1

Downvote

Reply
reply

Award

Share

xTheBenjamin
•
2y ago
I understand. I cannot help you in that case though, sorry!


Upvote
1

Downvote

Reply
reply

Award

Share

cardyet
•
2y ago
It's hard to read the unformatted code, but are you confusing backend and frontend code. You're calling the Supabase API directly from the frontend, yep? Or you've built an API layer that you're calling?

On the client, you shouldn't have to do anything cors related, that's specified by the server, do you have to specify allow domains in the console or something (I don't think so though).



Upvote
1

Downvote

Reply
reply

Award

Share

darbokredshrirt
OP
•
2y ago
i thought it would be easier to put the HTML and the script in the same file to avoid import/export. I was trying to do it as simple as possible. clearly I did not.


Upvote
1

Downvote

Reply
reply

Award

Share

u/lenovo avatar
lenovo
• Official
•
Promoted

Das ist nicht nur der Start einer neuen PC-Ära. Das ist IHR Neustart. Auf dem Lenovo Yoga Slim 7i mit Intel® Core™ Ultra Prozessor können Sie KI-gestützte Kreativtools schneller ausführen. Ihr Intel KI-PC macht es möglich. KI ist jetzt persönlich.
Learn More
lenovo.com
Clickable image which will reveal the video player: Das ist nicht nur der Start einer neuen PC-Ära. Das ist IHR Neustart. Auf dem Lenovo Yoga Slim 7i mit Intel® Core™ Ultra Prozessor können Sie KI-gestützte Kreativtools schneller ausführen. Ihr Intel KI-PC macht es möglich. KI ist jetzt persönlich.
Collapse video player

0:00 / 0:00




void_-_null
•
2y ago
why are you using requests directly, instead of client



Upvote
1

Downvote

Reply
reply

Award

Share

darbokredshrirt
OP
•
2y ago
I dont know any better, self teaching and just havent discovered some of that yet.



Upvote
1

Downvote

Reply
reply

Award

Share

void_-_null
•
2y ago
please go through docs, there are pretty much every snippet provided be it rest calls, subscriptions.



Upvote
1

Downvote

Reply
reply

Award

Share

darbokredshrirt
OP
•
2y ago
I'm trying to use a combination of reading, youtube and asking questions. Sometimes I understand what I'm reading and sometimes I dont. I've been able to get the GET request to work but POST , I just can't seem to get from submitted form data and having it put into the DB table. I've tried in astro, svelte and no framework at all.



Upvote
1

Downvote

Reply
reply

Award

Share

void_-_null
•
2y ago
It's great that you are trying all by yourself, but I'd suggest going through "supabase getting started" docs instead of youtube.

https://supabase.com/docs/reference/javascript/insert



Upvote
1

Downvote

Reply
reply

Award

Share

darbokredshrirt
OP
•
2y ago
so that's the part the would be in the server part of client/server I assume. I just need to figure out how to pass it data from a form, which I guess I'd use form action or fetch?



Upvote
1

Downvote

Reply
reply

Award

Share

void_-_null
•
2y ago
No. this is supposed to be written on client



Upvote
1

Downvote

Reply
reply

Award

Share

darbokredshrirt
OP
•
2y ago
this is where form values would be passed?

.insert({ id: 1, name: 'Denmark' })



Upvote
1

Downvote

Reply
reply

Award

Share

void_-_null
•
2y ago
replace your fetch call by .insert(), id and name are example columns. use your columns instead



Upvote
1

Downvote

Reply
reply

Award

Share

darbokredshrirt
OP
•
2y ago
so i'd have the submit button on the form some how connected to the insert statement?
****************************************************************************************************
CORS issues with supabase edge functions
Hello fellow devs,
I am looking if someone with a bit more experience and knowledge can provide me some insight into this issue. I am actually in a process of learning React and building a first app for which I use supabase, so I am by far not an expert in development, especially not with servers and serverless functions. But for my app, I really need url metadata scraping functionality, and I managed to implement it on my local server in Node env.

But since my whole app is built in supabase, I figured to utilise supabase edge functions. After hours of trials and errors, I managed to bypass cors for my tryout function. (also, I never managed to bypas cors in local development, but only after I deployed my function).

const corsHeaders = {
"Access-Control-Allow-Origin": "*",
"Access-Control-Allow-Methods": "GET, POST, OPTIONS",
"Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
};
console.log(`Function "browser-with-cors" up and running!`)
Deno.serve(async (req) => {

if (req.method === 'OPTIONS') {
return new Response("ok", { headers: corsHeaders })
}
try {
const { name } = await req.json()
const data = {
message: `Hello ${name}!`,
}
return new Response(JSON.stringify(data), {
headers: { ...corsHeaders, 'Content-Type': 'application/json' },
status: 200,
})
} catch (error) {
return new Response(JSON.stringify({ error: error.message }), {
headers: { ...corsHeaders, 'Content-Type': 'application/json' },
status: 400,
})
}
})

SO this function when deployed, have no CORS issues.

So, for my actual function,

import { getOGTags } from 'https://deno.land/x/opengraph@v1.0.0/mod.ts';
const corsHeaders = {
"Access-Control-Allow-Origin": "*",
"Access-Control-Allow-Methods": "GET, POST, OPTIONS",
"Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
};
console.log(`Function "fetch-metadata" up and running!`);
Deno.serve(async (req) => {
if (req.method === 'OPTIONS') {

return new Response('OK', { headers: corsHeaders });
}
try {
const requestJson = await req.json();
const url = requestJson.url;
if (!url) {
throw new Error("URL parameter is required.");
}
const metadata = await getOGTags(url);
return new Response(JSON.stringify(metadata), {
headers: { ...corsHeaders, 'Content-Type': 'application/json' },
status: 200,
});
} catch (error) {
console.error(error);
return new Response(JSON.stringify({ error: error.message }), {
headers: { ...corsHeaders, 'Content-Type': 'application/json' },
status: 400,
});
}
});

And this function, when deployed, following same process like the first one, get this well known console error, "has been blocked by CORS policy: Response to preflight request doesn't pass access control check: It does not have HTTP ok status."

Tbh, I have no idea how to even approach debugging it. To, they are handling cors the same way, so I not sure is it my mistake somewhere, or maybe these edge functions are not reliable as I tought.

I had no issues with cors handling it in node env.

So any feedback, info, is very much appreciated. I am almost done with the app, but stuck at this part.

Thank you and happy coding.





Upvote
1

Downvote

5
Go to comments


Share

Report
Report
u/SamsungGermany avatar
SamsungGermany
•
Promoted

Hol dir Beauty-Tipps in Echtzeit: Einfach die Kamera des neuen Galaxy Z Fold7 für Google Gemini Live freigeben und beraten lassen.
Learn More
samsung.com
Clickable image which will reveal the video player: Hol dir Beauty-Tipps in Echtzeit: Einfach die Kamera des neuen Galaxy Z Fold7 für Google Gemini Live freigeben und beraten lassen.
Collapse video player

0:00 / 0:00




Join the conversation
Sort by:

Best

Search Comments
Expand comment search
Comments Section
u/Hot_Chemical_2376 avatar
Hot_Chemical_2376
•
1y ago
Im just high: but have you tried lowercase "ok" on options response?

track me


Upvote
1

Downvote

Reply
reply

Award

Share

u/Acrobatic_Ad_7594 avatar
Acrobatic_Ad_7594
OP
•
1y ago
As soon as I woke up, I ran to my PC hoping that this oversight was the issue, but unfortunately not.

The most annoying part is that one actually works.

Maybe I should just follow the advice from my friend, and skip this, not wasting any more time, since I am still learning, and I shouldn't be stuck on these more complex features like edge functions.

But thanks anyways, I was really hoping for that simple solution you suggested is the issue. 😅



Upvote
2

Downvote

Reply
reply

Award

Share

Techuila
•
1y ago
Start from testing it manually using `curl`. Your edge function should return an `ok` response.
`curl -X OPTIONS -H "Origin: http://localhost:<port-number>" -H "Access-Control-Request-Method: POST" "<your-edge-function-url>" -v`


Upvote
1

Downvote

Reply
reply

Award

Share

kenweego
•
1y ago
Last time I had cors issues it was because I want passing my cors object to ALl my responses.


Upvote
1

Downvote

Reply
reply

Award

Share

u/Sad-Marionberry5926 avatar
Sad-Marionberry5926
•
1y ago
Try and download the function to check if it even exists.
The functions will show up in edge but not be active or able to download if the size is too big.

I had a similar issue. Sadly edge functions only support around up to 20mb.
****************************************************************************************************



****************************************************************************************************


Summary
The appropriate status code for a preflight request is 204 as per the updated Mozilla guidelines (https://developer.mozilla.org/en-US/docs/Glossary/Preflight_request). We previously used to send HTTP 200.

Full changelog
return 204 for preflight response instead of 200
update tests to accommodate status code change
Issues resolved
Fix #6051

thehanimo added 3 commits 5 years ago
@thehanimo
fix(cors) send HTTP 204 on Preflight request 
9c175d6
@thehanimo
tests(cors) update Preflight request tests
6ca1277
@thehanimo
fix(cors) fix variable name
d545e1f
@locao
Contributor
locao commented on Aug 11, 2020
Hi @thehanimo, thanks for your submission!

Can you fix the test that is failing, please?

spec/03-plugins/13-cors/01-access_spec.lua:681:31: expected ')' near 'res'
@locao locao added the pr/changes requested label on Aug 11, 2020
thehanimo added 4 commits 5 years ago
@thehanimo
tests(cors) fix syntax error
aa1ee63
@thehanimo
tests(cors) assert Content-Length as nil for HTTP 204
1fdb11a
@thehanimo
tests(cors) add Access-Control-Request-Method header to OPTIONS request
9e57f8d
@thehanimo
tests(cors) fix HTTP response for non-preflight OPTIONS request
cd3bdd2
@thehanimo
Contributor
Author
thehanimo commented on Aug 11, 2020
@locao made a few fixes. Travis CI fails with the following:

spec/03-plugins/01-tcp-log/01-tcp-log_spec.lua:242: in function <spec/03-plugins/01-tcp-log/01-tcp-log_spec.lua:208>
Could this be a one-off issue where the latencies don't add up?

thehanimo added 3 commits 5 years ago
@thehanimo
Trigger Travis CI Rebuild
5e9036b
@thehanimo
Trigger Travis CI Rebuild
14b6845
@thehanimo
Trigger Travis CI Rebuild
f45c949
@thehanimo
Contributor
Author
thehanimo commented on Aug 11, 2020
Turns out it was :)

Do take a look @locao!

@locao locao added pr/please review and removed pr/changes requested labels on Aug 12, 2020
@hishamhm
Contributor
hishamhm commented on Aug 17, 2020
Change looks correct according to the MDN recommendations. But if I'm reading the spec correctly it seems to allow both 200 and 204 as valid responses: https://fetch.spec.whatwg.org/ — and it looks like in the past the choice of value has caused compatibility issues, however:

var corsOptions = {
  origin: 'http://example.com',
  optionsSuccessStatus: 200 // some legacy browsers (IE11, various SmartTVs) choke on 204
}
(source)

Before we change this value around, is there a current need to make it return 204 (i.e. are there clients currently failing given the 200 response), or is this goal of this change exclusively to align with the MDN recommendation?

I'm trying to assess whether changing this in a patch release would cause surprising behavior or breakage for users relying on the current behavior.

@thehanimo
Contributor
Author
thehanimo commented on Aug 17, 2020
@hishamhm I guess this change was solely aimed at aligning with the MDN recommendations. Not sure if there's a need. @dzmitry-lahoda, was there something specific you had in mind?

@dzmitry-lahoda
dzmitry-lahoda commented on Aug 17, 2020
MDN - yes. And MS did the change in ASP.NET either.

@kikito kikito added the plugins/cors label on Sep 23, 2020
@kanongil kanongil mentioned this pull request on Sep 26, 2020
Proposal to add route option for CORS preflight status code hapijs/hapi#4165
Closed
kikito
kikito requested changes on Dec 2, 2020
Member
kikito left a comment • 
Because of the reason mentioned by @hisham, this can potentially break backwards compatibility for users already using the plugin, and consuming it via a plugin.

We can conform with MDN recommendations and still be backwards compatible by adding a config option. Something like:

--  kong/plugins/cors/schema.lua

return {
  name = "cors",
  fields = {
    ...
    { config = {
        type = "record",
        fields = {
        ...
           { preflight_status = {
               type = "integer",
               required = true,
               default = 200,
               one_of = { 200, 204 }, },
               ...
}
And use conf.preflight_status as the status. This way people can "opt in" to the MDN recommendations without breaking backwards compatibility for others.

@kikito kikito added pending author feedback and removed pr/please review labels on Dec 9, 2020
@stale
stale bot commented on Apr 9, 2021
This issue has been automatically marked as stale because it has not had recent activity. It will be closed if no further activity occurs. Thank you for your contributions.

@stale stale bot added the stale label on Apr 9, 2021
@stale stale bot closed this on Apr 16, 2021
@ms2008 ms2008 mentioned this pull request on May 17, 2023
fix(cors) follow mozilla guidelines for preflight request #4029
 Merged
Merge info
Closed with unmerged commits
This pull request is closed.

****************
