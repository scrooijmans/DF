# 

opendal/services/dropbox/

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
21use tokio::sync::Mutex;
22
23use super::DROPBOX_SCHEME;
24use super::backend::DropboxBackend;
25use super::core::DropboxCore;
26use super::core::DropboxSigner;
27use crate::raw::*;
28use crate::services::DropboxConfig;
29use crate::*;
30
31/// [Dropbox](https://www.dropbox.com/) backend support.
32#[doc = include_str!("docs.md")]
33#[derive(Default)]
34pub struct DropboxBuilder {
35    pub(super) config: DropboxConfig,
36
37    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
38    pub(super) http_client: Option<HttpClient>,
39}
40
41impl Debug for DropboxBuilder {
42    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
43        f.debug_struct("Builder")
44            .field("root", &self.config.root)
45            .finish()
46    }
47}
48
49impl DropboxBuilder {
50    /// Set the root directory for dropbox.
51    ///
52    /// Default to `/` if not set.
53    pub fn root(mut self, root: &str) -> Self {
54        self.config.root = if root.is_empty() {
55            None
56        } else {
57            Some(root.to_string())
58        };
59
60        self
61    }
62
63    /// Access token is used for temporary access to the Dropbox API.
64    ///
65    /// You can get the access token from [Dropbox App Console](https://www.dropbox.com/developers/apps)
66    ///
67    /// NOTE: this token will be expired in 4 hours.
68    /// If you are trying to use the Dropbox service in a long time, please set a refresh_token instead.
69    pub fn access_token(mut self, access_token: &str) -> Self {
70        self.config.access_token = Some(access_token.to_string());
71        self
72    }
73
74    /// Refresh token is used for long term access to the Dropbox API.
75    ///
76    /// You can get the refresh token via OAuth 2.0 Flow of Dropbox.
77    ///
78    /// OpenDAL will use this refresh token to get a new access token when the old one is expired.
79    pub fn refresh_token(mut self, refresh_token: &str) -> Self {
80        self.config.refresh_token = Some(refresh_token.to_string());
81        self
82    }
83
84    /// Set the client id for Dropbox.
85    ///
86    /// This is required for OAuth 2.0 Flow to refresh the access token.
87    pub fn client_id(mut self, client_id: &str) -> Self {
88        self.config.client_id = Some(client_id.to_string());
89        self
90    }
91
92    /// Set the client secret for Dropbox.
93    ///
94    /// This is required for OAuth 2.0 Flow with refresh the access token.
95    pub fn client_secret(mut self, client_secret: &str) -> Self {
96        self.config.client_secret = Some(client_secret.to_string());
97        self
98    }
99
100    /// Specify the http client that used by this service.
101    ///
102    /// # Notes
103    ///
104    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
105    /// during minor updates.
106    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
107    #[allow(deprecated)]
108    pub fn http_client(mut self, http_client: HttpClient) -> Self {
109        self.http_client = Some(http_client);
110        self
111    }
112}
113
114impl Builder for DropboxBuilder {
115    type Config = DropboxConfig;
116
117    fn build(self) -> Result<impl Access> {
118        let root = normalize_root(&self.config.root.unwrap_or_default());
119
120        let signer = match (self.config.access_token, self.config.refresh_token) {
121            (Some(access_token), None) => DropboxSigner {
122                access_token,
123                // We will never expire user specified token.
124                expires_in: Timestamp::MAX,
125                ..Default::default()
126            },
127            (None, Some(refresh_token)) => {
128                let client_id = self.config.client_id.ok_or_else(|| {
129                    Error::new(
130                        ErrorKind::ConfigInvalid,
131                        "client_id must be set when refresh_token is set",
132                    )
133                    .with_context("service", Scheme::Dropbox)
134                })?;
135                let client_secret = self.config.client_secret.ok_or_else(|| {
136                    Error::new(
137                        ErrorKind::ConfigInvalid,
138                        "client_secret must be set when refresh_token is set",
139                    )
140                    .with_context("service", Scheme::Dropbox)
141                })?;
142
143                DropboxSigner {
144                    refresh_token,
145                    client_id,
146                    client_secret,
147                    ..Default::default()
148                }
149            }
150            (Some(_), Some(_)) => {
151                return Err(Error::new(
152                    ErrorKind::ConfigInvalid,
153                    "access_token and refresh_token can not be set at the same time",
154                )
155                .with_context("service", Scheme::Dropbox));
156            }
157            (None, None) => {
158                return Err(Error::new(
159                    ErrorKind::ConfigInvalid,
160                    "access_token or refresh_token must be set",
161                )
162                .with_context("service", Scheme::Dropbox));
163            }
164        };
165
166        Ok(DropboxBackend {
167            core: Arc::new(DropboxCore {
168                info: {
169                    let am = AccessorInfo::default();
170                    am.set_scheme(DROPBOX_SCHEME)
171                        .set_root(&root)
172                        .set_native_capability(Capability {
173                            stat: true,
174
175                            read: true,
176
177                            write: true,
178
179                            create_dir: true,
180
181                            delete: true,
182
183                            list: true,
184                            list_with_recursive: true,
185
186                            copy: true,
187
188                            rename: true,
189
190                            shared: true,
191
192                            ..Default::default()
193                        });
194
195                    // allow deprecated api here for compatibility
196                    #[allow(deprecated)]
197                    if let Some(client) = self.http_client {
198                        am.update_http_client(|_| client);
199                    }
200
201                    am.into()
202                },
203                root,
204                signer: Arc::new(Mutex::new(signer)),
205            }),
206        })
207    }
208}
```
