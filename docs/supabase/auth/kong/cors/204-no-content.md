Title: 204 No Content - HTTP Guides

Description: Learn what the HTTP 204 No Content status code means, when it is used, and how it relates to CORS preflight requests.

Website published: 2023-08-29

Website last modified: 2023-08-31

The HTTP 204 status code means a request was successful, and the server doesn’t have a response body (payload) to return.

This status code is useful when clients don’t need or servers don’t have a response body. Some real-life examples:

*   deleting a record via the API (using the HTTP DELETE method)
*   saving file progress in the document editor (for example, Google Docs)
*   CORS preflight requests

HTTP spec states that the response is terminated by the first empty line after the header fields. Most HTTP clients indeed ignore the response body upon encountering a `204 No Content` response. Despite that, some web servers still include it.

CORS preflight requests
-----------------------

`204 No Content` is commonly used as a response to CORS preflight requests. Clients use preflight requests to check if servers can handle actual CORS requests:

```
OPTIONS /fly HTTP/1.1
Origin: https://example.com
Access-Control-Request-Method: DELETE
Access-Control-Request-Headers: Content-Type
```

If the server can understand CORS and allows the client to send a DELETE request, for example, it will respond with a `204 No Content` and CORS-related headers:

```
HTTP/1.1 204 No Content
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: POST, GET, OPTIONS, DELETE
Access-Control-Allow-Headers: Content-Type
Access-Control-Max-Age: 86400
```

However, if the server isn’t properly configured to process `OPTIONS` requests, it will throw a 405 Method Not Allowed error.