# 

opendal/raw/http_util/

error.rs

``` rust
1// Licensed to the Apache Software Foundation (ASF) under one
2// or more contributor license agreements.  See the NOTICE file
3// distributed with this work for additional information
4// regarding copyright ownership.  The ASF licenses this file
5// to you under the Apache License, Version 2.0 (the
6// "License"); you may not use this file except in compliance
7// with the License.  You may obtain a copy of the License at
8//
9//   http://www.apache.org/licenses/LICENSE-2.0
10//
11// Unless required by applicable law or agreed to in writing,
12// software distributed under the License is distributed on an
13// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
14// KIND, either express or implied.  See the License for the
15// specific language governing permissions and limitations
16// under the License.
17
18use http::Uri;
19use http::response::Parts;
20
21use crate::Error;
22use crate::ErrorKind;
23
24/// Create a new error happened during building request.
25pub fn new_request_build_error(err: http::Error) -> Error {
26    Error::new(ErrorKind::Unexpected, "building http request")
27        .with_operation("http::Request::build")
28        .set_source(err)
29}
30
31/// Create a new error happened during signing request.
32pub fn new_request_credential_error(err: anyhow::Error) -> Error {
33    Error::new(
34        ErrorKind::Unexpected,
35        "loading credential to sign http request",
36    )
37    .set_temporary()
38    .with_operation("reqsign::LoadCredential")
39    .set_source(err)
40}
41
42/// Create a new error happened during signing request.
43pub fn new_request_sign_error(err: anyhow::Error) -> Error {
44    Error::new(ErrorKind::Unexpected, "signing http request")
45        .with_operation("reqsign::Sign")
46        .set_source(err)
47}
48
49/// Add response context to error.
50///
51/// This helper function will:
52///
53/// - remove sensitive or useless headers from parts.
54/// - fetch uri if parts extensions contains `Uri`.
55pub fn with_error_response_context(mut err: Error, mut parts: Parts) -> Error {
56    if let Some(uri) = parts.extensions.get::<Uri>() {
57        err = err.with_context("uri", uri.to_string());
58    }
59
60    // The following headers may contains sensitive information.
61    parts.headers.remove("Set-Cookie");
62    parts.headers.remove("WWW-Authenticate");
63    parts.headers.remove("Proxy-Authenticate");
64
65    err = err.with_context("response", format!("{parts:?}"));
66
67    err
68}
```
