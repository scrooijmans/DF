# @tauri-apps/plugin-http | Tauri
Make HTTP requests with the Rust backend.

[Security](#security)
---------------------

[Section titled “Security”](#security)

This API has a scope configuration that forces you to restrict the URLs that can be accessed using glob patterns.

For instance, this scope configuration only allows making HTTP requests to all subdomains for `tauri.app` except for `https://private.tauri.app`:

```

{
  "permissions": [
    {
      "identifier": "http:default",
      "allow": [{ "url": "https://*.tauri.app" }],
      "deny": [{ "url": "https://private.tauri.app" }]
    }
  ]
}
```


Trying to execute any API with a URL not configured on the scope results in a promise rejection due to denied access.

[Interfaces](#interfaces)
-------------------------

[Section titled “Interfaces”](#interfaces)

### [ClientOptions](#clientoptions)

[Section titled “ClientOptions”](#clientoptions)

Options to configure the Rust client used to make fetch requests

#### [Since](#since)

[Section titled “Since”](#since)

2.0.0

#### [Properties](#properties)

[Section titled “Properties”](#properties)



* Property:  connectTimeout?
  * Type: number
  * Description: Timeout in milliseconds
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L82
* Property:  danger?
  * Type: DangerousSettings
  * Description: Configuration for dangerous settings on the client such as disabling SSL verification.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L90
* Property:  maxRedirections?
  * Type: number
  * Description: Defines the maximum number of redirects the client should follow. If set to 0, no redirects will be followed.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L80
* Property:  proxy?
  * Type: Proxy
  * Description: Configuration of a proxy that a Client should pass requests to.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L86


* * *

### [DangerousSettings](#dangeroussettings)

[Section titled “DangerousSettings”](#dangeroussettings)

Configuration for dangerous settings on the client such as disabling SSL verification.

#### [Since](#since-1)

[Section titled “Since”](#since-1)

2.3.0

#### [Properties](#properties-1)

[Section titled “Properties”](#properties-1)



* Property:  acceptInvalidCerts?
  * Type: boolean
  * Description: Disables SSL verification.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L102
* Property:  acceptInvalidHostnames?
  * Type: boolean
  * Description: Disables hostname verification.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L106


* * *

### [Proxy](#proxy)

[Section titled “Proxy”](#proxy)

Configuration of a proxy that a Client should pass requests to.

#### [Since](#since-2)

[Section titled “Since”](#since-2)

2.0.0

#### [Properties](#properties-2)

[Section titled “Properties”](#properties-2)



* Property:  all?
  * Type: string | ProxyConfig
  * Description: Proxy all traffic to the passed URL.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L40
* Property:  http?
  * Type: string | ProxyConfig
  * Description: Proxy all HTTP traffic to the passed URL.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L44
* Property:  https?
  * Type: string | ProxyConfig
  * Description: Proxy all HTTPS traffic to the passed URL.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L48


* * *

### [ProxyConfig](#proxyconfig)

[Section titled “ProxyConfig”](#proxyconfig)

#### [Properties](#properties-3)

[Section titled “Properties”](#properties-3)



* Property:  basicAuth?
  * Type: object
  * Description: Set the Proxy-Authorization header using Basic auth.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L59
* Property:  basicAuth.password
  * Type: string
  * Description: -
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L61
* Property:  basicAuth.username
  * Type: string
  * Description: -
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L60
* Property:  noProxy?
  * Type: string
  * Description: A configuration for filtering out requests that shouldn’t be proxied. Entries are expected to be comma-separated (whitespace between entries is ignored)
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L67
* Property:  url
  * Type: string
  * Description: The URL of the proxy server.
  * Defined in: Source: https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L55


[Functions](#functions)
-----------------------

[Section titled “Functions”](#functions)

### [fetch()](#fetch)

[Section titled “fetch()”](#fetch)

```

function fetch(input, init?): Promise<Response>
```


Fetch a resource from the network. It returns a `Promise` that resolves to the `Response` to that `Request`, whether it is successful or not.

#### [Parameters](#parameters)

[Section titled “Parameters”](#parameters)


|Parameter|Type                       |
|---------|---------------------------|
|input    |string | URL | Request     |
|init?    |RequestInit & ClientOptions|


#### [Returns](#returns)

[Section titled “Returns”](#returns)

[`Promise`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Promise)<[`Response`](https://developer.mozilla.org/docs/Web/API/Response)\>

#### [Example](#example)

[Section titled “Example”](#example)

```

const response = await fetch("http://my.json.host/data.json");
console.log(response.status);  // e.g. 200
console.log(response.statusText); // e.g. "OK"
const jsonData = await response.json();
```


#### [Since](#since-3)

[Section titled “Since”](#since-3)

2.0.0

**Source**: [https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L125](https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/http/guest-js/index.ts#L125)

© 2025 Tauri Contributors. CC-BY / MIT