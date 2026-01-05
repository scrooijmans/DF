Title: CORS - Plugin | Kong Docs

Description: The CORS plugin lets you add Cross-Origin Resource Sharing (CORS) to a Service or a Route.          

The gateway to the infrastructure behind tomorrow's AI systems.

Register Now → Close API Summit 2025 banner

Home / Kong Plugin Hub

Edit this Page Edit

Report an Issue Report

CORS
====

Related Documentation

All Gateway Documentation

Made by

Kong Inc.

Supported Gateway Topologies

hybrid db-less traditional

Supported Konnect Deployments

hybrid cloud-gateways serverless

Compatible Protocols

grpc grpcs http https

Minimum Version

Kong Gateway - 1.0

Related Resources

DNS configuration reference

The CORS plugin lets you add Cross-Origin Resource Sharing (CORS) to a Service or a Route. This allows you to automate the configuration of CORS rules, ensuring that your upstreams only accept and share resources with approved sources.

Understanding CORS
------------------

For security purposes a browser will stop requests from accessing URLs on different domains. This is done using CORS, a set of rules for web applications that make requests across origin. CORS works by looking at the HTTP `origin` header of a URL and checking it against a list of allowed headers. An `origin` header can contain the `scheme`, `hostname`, or `port` of the requesting URL. Operations that are restricted to same-origin content can be managed using CORS.

When making a cross-origin request, browsers issue an `origin` request header, and servers must respond with a matching `Access-Control-Allow-Origin` (ACAO) header. If the two headers do not match, the browser will discard the response, and any application components that require that response’s data will not function properly.

For example, the following request and response pairs have matching CORS headers, and will succeed:

```
GET / HTTP/1.1
Host: example.com
Origin: http://example.net

HTTP/1.1 200 OK
Access-Control-Allow-Origin: http://example.net
```

Copied!

```
GET / HTTP/1.1
Host: example.com
Origin: http://example.net

HTTP/1.1 200 OK
Access-Control-Allow-Origin: *
```

Copied!

The requests do not have a matching CORS headers and therefore will fail:

```
GET / HTTP/1.1
Host: example.com
Origin: http://example.net

HTTP/1.1 200 OK
Access-Control-Allow-Origin: http://badbadcors.example
```

Copied!

```
GET / HTTP/1.1
Host: example.com
Origin: http://example.net

HTTP/1.1 200 OK
```

Copied!

Missing CORS headers when CORS headers are expected results in failure.

CORS limitations
----------------

When the client is a browser, the preflight OPTIONS requests defined by the CORS specification have strict rules about which headers can be set. Certain headers, including Host, are classified as forbidden headers, meaning the browser always controls their value and they can’t be customized in code (for example, in JavaScript). As a result, a browser can’t send a custom Host header during a preflight request.

This limitation is important when using the CORS plugin with Routes in Kong Gateway. If a Route is configured to match only on the `hosts` field, the preflight request may not carry the expected Host header, and Kong Gateway may fail to match the Route. This means that the plugin can’t reliably process these requests.

To ensure correct behavior, use the CORS plugin only with Routes that match on paths or methods. The preflight request includes both paths and methods, so Kong Gateway can use these fields for matching.

Related Documentation

All Gateway Documentation

Made by

Kong Inc.

Supported Gateway Topologies

hybrid db-less traditional

Supported Konnect Deployments

hybrid cloud-gateways serverless

Compatible Protocols

grpc grpcs http https

Minimum Version

Kong Gateway - 1.0

Related Resources

DNS configuration reference

*   CORS
*   Understanding CORS
*   CORS limitations

Something wrong?

Report an Issue | Edit this Page

### Help us make these docs great!

Kong Developer docs are open source. If you find these useful and want to make them better, contribute today!

Contribute

### Still need help

Ask in our Forum

Contact Support