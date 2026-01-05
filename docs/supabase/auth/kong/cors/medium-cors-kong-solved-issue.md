If you’re here then I guess you already know what CORS is (hint: Cross-Origin Resource Sharing). After all, no one reads about CORS just for fun, right? :). If you do need some background, this is a good article. In any case, this blog post will reveal most of it anyway.

The challenge
We have an Analysis application: this is the main application our clients interact with and it serves most of the UI. This is obviously the first application we ever coded in Kenshoo, our beloved monolith, long before we switched to microservices architecture.

Like most of the industry, it’s been a couple of years now since we’ve switched to microservices architecture. One of our challenges was to serve a UI whose entire back end is in a dedicated microservice (or multiple microservices), from within our Analysis application, which is our main application, as mentioned.

Several issues come up with such an integration, CORS being one of the main obstacles, authentication and authorization being another, and a couple more that I will spare you.

Press enter or click to view image in full size

Over the years we’ve tried to solve this in different ways, some of which are more in the “creativity” realm rather than the “right solution” realm (iFrames, using the Analysis application as a gateway, etc.). Other solutions required a lot of work or configuration in the different microservices or web-servers that are part of the flow. So we always ended up with one-off solutions on a case-by-case basis, rather than a generic solution that wouldn’t require any adjustments on every component or microservice we’re adding to the stack.

Our requirements
So what were our requirements for a real generic solution? The main ones were:

The microservice should have only one authentication/authorization flow, no matter the origin of the request. That said, it should still have the ability to distinguish between the two flows if needed.
Little to no changes in pre-existing microservices.
Adding new routes which serve such a UI flow should only require a simple configuration by the developer.
Work across our various environments such as local & dev. environment, staging, production, etc. Each of these have slightly different characteristics such as different domains.
A way to monitor the usage of the different routes and distinguish between an interactive user flow and an API accessing the same route.
The game changer: API gateway
When we added an API gateway to our stack to expose public APIs, it suddenly hit us: This is a component designed to encapsulate things like authentication, CORS, routing, auditing and more. On top of that, no matter what development, configuration or any other adjustments it will require, it will all be in a single place.

In this post, I’ll share how we used Kong (our API gateway) to solve the issue and will discuss the main configurations required for this.

A word about exploratory testing
Obviously there’s a lot of knowledge out there about CORS and authentication designs, but two important things we learned are:

There are quite a lot of moving parts such as HTTP client, web-server config, cookies, API gateway, identity provider, and microservice-side authorization.
It can be confusing.
On top of that, every company has its own network architecture, which might include several domains, different domains for different environments (production, staging, etc.), additional proxies/reverse proxies, and so on. This obviously makes it even more confusing.

So the first thing we did, and my first tip for you, is to create a dedicated exploratory testing environment. Our environment included:

Analysis application
Microservice
API gateway
Dedicated UI that works with the API gateway, issues requests with different HTTP methods, and is served from the Analysis application.
The exploratory testing environment let us change and fine-tune the configuration gradually, until we reached the desired state (and then automated it, of course).

The solution
Below you can find the full flow and all the relevant components. The pink boxes represent the relevant components that I’ll elaborate on with more information about the required configuration.

Press enter or click to view image in full size

IdP (Identity Provider)
The identity provider is in charge of authentication and serves both Interactive logins from the UI and API flows.
On successful login/authentication, the IdP creates an Auth JWT token and returns it to the requester.
For interactive users we also set a cookie with the Auth JWT token, which we will later use in the API gateway.
You want to make sure that the following applies to this cookie you set:
It is secured.
It is configured as HttpOnly, which means that it is not accessible by js client side code.
You set it on the top/parent domain: If, for example, your parent domain is kenshoo.com and your IdP is on auth.kenshoo.com, make sure the domain where you set the cookie is .kenshoo.com and NOT auth.kenshoo.com. If you set it on the subdomain (auth.kenshoo.com), then when the UI code calls your API gateway the cookie will not be sent as part of the request.

UI/client-side code
Requests from the UI will go to the API Gateway (services.kenshoo.com in our case) instead of the specific microservice.
You’ll need to make sure that the HTTP Requests are being built properly with isCredentials/withCredentials/credentials (all are different names of the same property) set to true. This tells the HTTP client to include cookies as part of the request (Here’s an explanation and an example of how to do it with axios).
It means that the Auth JWT token set to .kenshoo.com will be sent as part of the request to any kenshoo.com subdomain.

API gateway (Kong)
In Kong, we used 3 different plugins (all are community plugins) to support the full flow.

JWT Plugin:

The JWT plugin is the first plugin that is being invoked/called when Kong receives a request.
By default, it will verify the JWT Token provided on the HTTP “Authorization” header.
However, you can configure the plugin to get the token from a cookie in case an Authorization header is missing.
For the interactive user flow, it is quite difficult and cumbersome to make sure that every request will have the “Authorization” header with the token value, so we added the cookie name to the JWT plugin configuration, which nailed the initial authentication step.
Serverless Functions Plugin:

Now we have both an API (consisting of an “Authorization” header) and interactive (consisting of a cookie with the Auth JWT token) flows which will eventually get to our microservice.
When we add another layer of authentication and/or authorization in the microservice side, we would need to support two types of requests (with header and with cookie).
To avoid this, and keep the microservices (and their authorization) agnostic to whether the request came from an interactive user or API, we decided to add a simple piece of logic in the API Gateway side that would enable copying the Auth JWT Token from the cookie and add it to an “Authorization” header when relevant.
To do that, we used the Serverless Functions Plugin which enable to run custom Lua code from Kong during the access phase.
The code snippet is as simple as this small function:
Press enter or click to view image in full size

CORS Plugin

This plugin, as its name states :), let’s you configure the API gateway behavior to support Cross-Origin Resource Sharing (CORS). Apart from enabling the plugin, below are the main configurations we used.

Credentials: This config needs to be set to true in case you want the request to transfer cookies. As indicated above, since we use a cookie to send over the Auth token we need the request to include it and also the server side (Kong) to support it.

Supported Origins:

In order to support CORS, you must make sure that responses to the user’s browser indicate which origins/domains are allowed to get this response.
In other words: my browser is currently in the domain something.kenshoo.com and is sending a request to something-else.kenshoo.com. When something-else.kenshoo.com response to the request it needs to indicate that something.kenshoo.com is allowed to consume this response, otherwise the browser will fail the response and will show an error in the console.
This list of supported origins is being transferred using an HTTP Response Header called Access-Control-Allow-Origin.
By adding supported origins to the CORS Plugin we instruct Kong to add this header with the relevant values to the response when needed.
You can add “*” as the supported origins, which means “everything”. This is generally not recommended unless absolutely required by the use case. However, this configuration will not work when you pass cookies/tokens (use isCredentials=true), in which case, you will have to provide specific origins.
Preflight Continue:

The purpose of a CORS preflight request is to check whether the CORS protocol is understood by the server for specific methods and headers. The most common use case is when you use PUT and DELETE HTTP methods.
When using these methods, the browser sends a request with HTTP method OPTIONS (a preflight), and only if the OPTIONS request is properly handled, it issues another request with the desired PUT/DELETE methods.
We set the Preflight Continue configuration in the plugin to false, which means: we don’t want the preflight requests to be sent from Kong to the upstream service but rather let Kong deal with it. This saves us from adding code in our microservices that would support such preflight requests.
Supported Methods: Similarly to the Supported Origins configuration, this configuration adds the Access-Control-Allow-Methods Response Header which indicates what are the allowed methods for the resource we are accessing. This Response Header is being added as a response to a preflight request.

And… that’s pretty much it!. Now we can expose the same routes both as UI or API without any configuration overhead whatsoever.

Some Conclusions and tips
Building a proper isolated exploratory testing environment was essential in order to get the right configuration/setup.
Of course all of the above can also be achieved without an API gateway. However, using an API gateway, we managed to encapsulate the CORS config and behavior in one place.
We kept the microservices indifferent to whether the request originated from UI or from API in terms of authentication/authorization (both flows pass an authorization header). By the way, we can still distinguish between the two flows if needed since we added this indication as a field in the JWT token.
API
Gateway
Cors
