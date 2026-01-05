# 

opendal/services/vercel_artifacts/

builder.rs

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
18use std::fmt::Debug;
19use std::fmt::Formatter;
20use std::sync::Arc;
21
22use super::VERCEL_ARTIFACTS_SCHEME;
23use super::backend::VercelArtifactsBackend;
24use super::core::VercelArtifactsCore;
25use crate::raw::Access;
26use crate::raw::AccessorInfo;
27use crate::raw::HttpClient;
28use crate::services::VercelArtifactsConfig;
29use crate::*;
30
31/// [Vercel Cache](https://vercel.com/docs/concepts/monorepos/remote-caching) backend support.
32#[doc = include_str!("docs.md")]
33#[derive(Default)]
34pub struct VercelArtifactsBuilder {
35    pub(super) config: VercelArtifactsConfig,
36
37    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
38    pub(super) http_client: Option<HttpClient>,
39}
40
41impl Debug for VercelArtifactsBuilder {
42    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
43        let mut d = f.debug_struct("VercelArtifactsBuilder");
44        d.field("config", &self.config);
45        d.finish_non_exhaustive()
46    }
47}
48
49impl VercelArtifactsBuilder {
50    /// set the bearer access token for Vercel
51    ///
52    /// default: no access token, which leads to failure
53    pub fn access_token(mut self, access_token: &str) -> Self {
54        self.config.access_token = Some(access_token.to_string());
55        self
56    }
57
58    /// Specify the http client that used by this service.
59    ///
60    /// # Notes
61    ///
62    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
63    /// during minor updates.
64    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
65    #[allow(deprecated)]
66    pub fn http_client(mut self, http_client: HttpClient) -> Self {
67        self.http_client = Some(http_client);
68        self
69    }
70}
71
72impl Builder for VercelArtifactsBuilder {
73    type Config = VercelArtifactsConfig;
74
75    fn build(self) -> Result<impl Access> {
76        let info = AccessorInfo::default();
77        info.set_scheme(VERCEL_ARTIFACTS_SCHEME)
78            .set_native_capability(Capability {
79                stat: true,
80
81                read: true,
82
83                write: true,
84
85                shared: true,
86
87                ..Default::default()
88            });
89
90        // allow deprecated api here for compatibility
91        #[allow(deprecated)]
92        if let Some(client) = self.http_client {
93            info.update_http_client(|_| client);
94        }
95
96        match self.config.access_token.clone() {
97            Some(access_token) => Ok(VercelArtifactsBackend {
98                core: Arc::new(VercelArtifactsCore {
99                    info: Arc::new(info),
100                    access_token,
101                }),
102            }),
103            None => Err(Error::new(ErrorKind::ConfigInvalid, "access_token not set")),
104        }
105    }
106}
```
