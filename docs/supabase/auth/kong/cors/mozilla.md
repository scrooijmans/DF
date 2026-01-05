Preflight request
A CORS preflight request is a CORS request that checks to see if the CORS protocol is understood and a server is aware using specific methods and headers.

It is an OPTIONS request, using two or three HTTP request headers: Access-Control-Request-Method, Origin, and optionally Access-Control-Request-Headers.

A preflight request is automatically issued by a browser and in normal cases, front-end developers don't need to craft such requests themselves. It appears when request is qualified as "to be preflighted" and omitted for simple requests.

For example, a client might be asking a server if it would allow a DELETE request, before sending a DELETE request, by using a preflight request:

http

Copy
OPTIONS /resource/foo
Access-Control-Request-Method: DELETE
Access-Control-Request-Headers: x-requested-with
Origin: https://foo.bar.org
If the server allows it, then it will respond to the preflight request with an Access-Control-Allow-Methods response header, which lists DELETE:

http

Copy
HTTP/1.1 204 No Content
Connection: keep-alive
Access-Control-Allow-Origin: https://foo.bar.org
Access-Control-Allow-Methods: POST, GET, OPTIONS, DELETE
Access-Control-Allow-Headers: X-Requested-With
Access-Control-Max-Age: 86400
The preflight response can be optionally cached for the requests created in the same URL using Access-Control-Max-Age header like in the above example. To cache preflight responses, the browser uses a specific cache that is separate from the general HTTP cache that the browser manages. Preflight responses are never cached in the browser's general HTTP cache.

In this article
See also

OPTIONS request method
Baseline Widely available
The OPTIONS HTTP method requests permitted communication options for a given URL or server. This can be used to test the allowed HTTP methods for a request, or to determine whether a request would succeed when making a CORS preflighted request. A client can specify a URL with this method, or an asterisk (*) to refer to the entire server.

Request has body	May*
Successful response has body	May
Safe	Yes
Idempotent	Yes
Cacheable	No
Allowed in HTML forms	No
* Although an OPTIONS message with a request body is technically allowed, it has no defined semantics. You may include a body in an OPTIONS message as long as you provide a valid Content-Type header, and when you know the server expects it, as behavior is implementation-specific.

In this article
Syntax
Examples
Specifications
Browser compatibility
See also
Syntax
http

Copy
OPTIONS *|<request-target>["?"<query>] HTTP/1.1
The request target may be either in 'asterisk form' * indicating the whole server, or a request target as is common with other methods:

*
Indicates that the client wishes to request OPTIONS for the server as a whole, as opposed to a specific named resource of that server.

<request-target>
Identifies the target resource of the request when combined with the information provided in the Host header. This is an absolute path (e.g., /path/to/file.html) in requests to an origin server, and an absolute URL in requests to proxies (e.g., http://www.example.com/path/to/file.html).

<query> Optional
An optional query component preceded by a question-mark ?. Often used to carry identifying information in the form of key=value pairs.

Examples
Identifying allowed request methods
To find out which request methods a server supports, one can use the curl command-line program to issue an OPTIONS request:

bash

Copy
curl -X OPTIONS https://example.org -i
This creates the following HTTP request:

http

Copy
OPTIONS / HTTP/2
Host: example.org
User-Agent: curl/8.7.1
Accept: */*
The response contains an Allow header that holds the allowed methods:

http

Copy
HTTP/1.1 204 No Content
Allow: OPTIONS, GET, HEAD, POST
Cache-Control: max-age=604800
Date: Thu, 13 Oct 2016 11:45:00 GMT
Server: EOS (lax004/2813)
Preflighted requests in CORS
In CORS, a preflight request is sent with the OPTIONS method so that the server can respond if it is acceptable to send the request. In this example, we will request permission for these parameters:

The Access-Control-Request-Method header sent in the preflight request tells the server that when the actual request is sent, it will have a POST request method.
The Access-Control-Request-Headers header tells the server that when the actual request is sent, it will have the X-PINGOTHER and Content-Type headers.
http

Copy
OPTIONS /resources/post-here/ HTTP/1.1
Host: bar.example
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-us,en;q=0.5
Accept-Encoding: gzip,deflate
Connection: keep-alive
Origin: https://foo.example
Access-Control-Request-Method: POST
Access-Control-Request-Headers: content-type,x-pingother
The server now can respond if it will accept a request under these circumstances. In this example, the server response says that:

Access-Control-Allow-Origin
The https://foo.example origin is permitted to request the bar.example/resources/post-here/ URL via the following:

Access-Control-Allow-Methods
POST, GET, and OPTIONS are permitted methods for the URL. (This header is similar to the Allow response header, but used only for CORS.)

Access-Control-Allow-Headers
X-PINGOTHER and Content-Type are permitted request headers for the URL.

Access-Control-Max-Age
The above permissions may be cached for 86,400 seconds (1 day).

http

Copy
HTTP/1.1 200 OK
Date: Mon, 01 Dec 2008 01:15:39 GMT
Server: Apache/2.0.61 (Unix)
Access-Control-Allow-Origin: https://foo.example
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: X-PINGOTHER, Content-Type
Access-Control-Max-Age: 86400
Vary: Accept-Encoding, Origin
Keep-Alive: timeout=2, max=100
Connection: Keep-Alive

CORS
CORS (Cross-Origin Resource Sharing) is a system, consisting of transmitting HTTP headers, that determines whether browsers block frontend JavaScript code from accessing responses for cross-origin requests.

The same-origin security policy forbids cross-origin access to resources. But CORS gives web servers the ability to say they want to opt into allowing cross-origin access to their resources.

In this article
CORS headers
See also
Couchbase, Inc.
Couchbase Capella
The cloud database platform for modern applications, including mobile and IoT application services.
Start for Free
Ad
Don't want to see ads?
CORS headers
Access-Control-Allow-Origin
Indicates whether the response can be shared.

Access-Control-Allow-Credentials
Indicates whether or not the response to the request can be exposed when the credentials flag is true.

Access-Control-Allow-Headers
Used in response to a preflight request to indicate which HTTP headers can be used when making the actual request.

Access-Control-Allow-Methods
Specifies the method or methods allowed when accessing the resource in response to a preflight request.

Access-Control-Expose-Headers
Indicates which headers can be exposed as part of the response by listing their names.

Access-Control-Max-Age
Indicates how long the results of a preflight request can be cached.

Access-Control-Request-Headers
Used when issuing a preflight request to let the server know which HTTP headers will be used when the actual request is made.

Access-Control-Request-Method
Used when issuing a preflight request to let the server know which HTTP method will be used when the actual request is made.

Origin
Indicates where a fetch originates from.

Timing-Allow-Origin
Specifies origins that are allowed to see values of attributes retrieved via features of the Resource Timing API, which would otherwise be reported as zero due to cross-origin restrictions.


