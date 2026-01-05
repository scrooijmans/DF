# Module http Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/http/mod.rs.html#18-293" class="src">Source</a>

Available on **crate feature `http`** only.

Expand description

An object store implementation for generic HTTP servers

This follows [rfc2518](https://datatracker.ietf.org/doc/html/rfc2518) commonly known as [WebDAV](https://en.wikipedia.org/wiki/WebDAV)

Basic get support will work out of the box with most HTTP servers, even those that don’t explicitly support [rfc2518](https://datatracker.ietf.org/doc/html/rfc2518)

Other operations such as list, delete, copy, etc… will likely require server-side configuration. A list of HTTP servers with support can be found [here](https://wiki.archlinux.org/title/WebDAV#Server)

Multipart uploads are not currently supported

## Structs<a href="https://docs.rs/object_store/latest/object_store/http/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpBuilder.html" class="struct" title="struct object_store::http::HttpBuilder">HttpBuilder</a>  
Configure a connection to a generic HTTP server

<a href="https://docs.rs/object_store/latest/object_store/http/struct.HttpStore.html" class="struct" title="struct object_store::http::HttpStore">HttpStore</a>  
An [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") implementation for generic HTTP servers
