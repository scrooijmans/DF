# 

opendal/services/gdrive/

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
18use log::debug;
19use std::fmt::Debug;
20use std::fmt::Formatter;
21use std::sync::Arc;
22use tokio::sync::Mutex;
23
24use super::GDRIVE_SCHEME;
25use super::backend::GdriveBackend;
26use super::core::GdriveCore;
27use super::core::GdrivePathQuery;
28use super::core::GdriveSigner;
29use crate::Scheme;
30use crate::raw::Access;
31use crate::raw::AccessorInfo;
32use crate::raw::HttpClient;
33use crate::raw::PathCacher;
34use crate::raw::Timestamp;
35use crate::raw::normalize_root;
36use crate::services::GdriveConfig;
37use crate::*;
38
39/// [GoogleDrive](https://drive.google.com/) backend support.
40#[derive(Default)]
41#[doc = include_str!("docs.md")]
42pub struct GdriveBuilder {
43    pub(super) config: GdriveConfig,
44
45    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
46    pub(super) http_client: Option<HttpClient>,
47}
48
49impl Debug for GdriveBuilder {
50    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
51        f.debug_struct("Backend")
52            .field("config", &self.config)
53            .finish()
54    }
55}
56
57impl GdriveBuilder {
58    /// Set root path of GoogleDrive folder.
59    pub fn root(mut self, root: &str) -> Self {
60        self.config.root = if root.is_empty() {
61            None
62        } else {
63            Some(root.to_string())
64        };
65
66        self
67    }
68
69    /// Access token is used for temporary access to the GoogleDrive API.
70    ///
71    /// You can get the access token from [GoogleDrive App Console](https://console.cloud.google.com/apis/credentials)
72    /// or [GoogleDrive OAuth2 Playground](https://developers.google.com/oauthplayground/)
73    ///
74    /// # Note
75    ///
76    /// - An access token is valid for 1 hour.
77    /// - If you want to use the access token for a long time,
78    ///   you can use the refresh token to get a new access token.
79    pub fn access_token(mut self, access_token: &str) -> Self {
80        self.config.access_token = Some(access_token.to_string());
81        self
82    }
83
84    /// Refresh token is used for long term access to the GoogleDrive API.
85    ///
86    /// You can get the refresh token via OAuth 2.0 Flow of GoogleDrive API.
87    ///
88    /// OpenDAL will use this refresh token to get a new access token when the old one is expired.
89    pub fn refresh_token(mut self, refresh_token: &str) -> Self {
90        self.config.refresh_token = Some(refresh_token.to_string());
91        self
92    }
93
94    /// Set the client id for GoogleDrive.
95    ///
96    /// This is required for OAuth 2.0 Flow to refresh the access token.
97    pub fn client_id(mut self, client_id: &str) -> Self {
98        self.config.client_id = Some(client_id.to_string());
99        self
100    }
101
102    /// Set the client secret for GoogleDrive.
103    ///
104    /// This is required for OAuth 2.0 Flow with refresh the access token.
105    pub fn client_secret(mut self, client_secret: &str) -> Self {
106        self.config.client_secret = Some(client_secret.to_string());
107        self
108    }
109
110    /// Specify the http client that used by this service.
111    ///
112    /// # Notes
113    ///
114    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
115    /// during minor updates.
116    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
117    #[allow(deprecated)]
118    pub fn http_client(mut self, http_client: HttpClient) -> Self {
119        self.http_client = Some(http_client);
120        self
121    }
122}
123
124impl Builder for GdriveBuilder {
125    type Config = GdriveConfig;
126
127    fn build(self) -> Result<impl Access> {
128        let root = normalize_root(&self.config.root.unwrap_or_default());
129        debug!("backend use root {root}");
130
131        let info = AccessorInfo::default();
132        info.set_scheme(GDRIVE_SCHEME)
133            .set_root(&root)
134            .set_native_capability(Capability {
135                stat: true,
136
137                read: true,
138
139                list: true,
140
141                write: true,
142
143                create_dir: true,
144                delete: true,
145                rename: true,
146                copy: true,
147
148                shared: true,
149
150                ..Default::default()
151            });
152
153        // allow deprecated api here for compatibility
154        #[allow(deprecated)]
155        if let Some(client) = self.http_client {
156            info.update_http_client(|_| client);
157        }
158
159        let accessor_info = Arc::new(info);
160        let mut signer = GdriveSigner::new(accessor_info.clone());
161        match (self.config.access_token, self.config.refresh_token) {
162            (Some(access_token), None) => {
163                signer.access_token = access_token;
164                // We will never expire user specified access token.
165                signer.expires_in = Timestamp::MAX;
166            }
167            (None, Some(refresh_token)) => {
168                let client_id = self.config.client_id.ok_or_else(|| {
169                    Error::new(
170                        ErrorKind::ConfigInvalid,
171                        "client_id must be set when refresh_token is set",
172                    )
173                    .with_context("service", Scheme::Gdrive)
174                })?;
175                let client_secret = self.config.client_secret.ok_or_else(|| {
176                    Error::new(
177                        ErrorKind::ConfigInvalid,
178                        "client_secret must be set when refresh_token is set",
179                    )
180                    .with_context("service", Scheme::Gdrive)
181                })?;
182
183                signer.refresh_token = refresh_token;
184                signer.client_id = client_id;
185                signer.client_secret = client_secret;
186            }
187            (Some(_), Some(_)) => {
188                return Err(Error::new(
189                    ErrorKind::ConfigInvalid,
190                    "access_token and refresh_token cannot be set at the same time",
191                )
192                .with_context("service", Scheme::Gdrive));
193            }
194            (None, None) => {
195                return Err(Error::new(
196                    ErrorKind::ConfigInvalid,
197                    "access_token or refresh_token must be set",
198                )
199                .with_context("service", Scheme::Gdrive));
200            }
201        };
202
203        let signer = Arc::new(Mutex::new(signer));
204
205        Ok(GdriveBackend {
206            core: Arc::new(GdriveCore {
207                info: accessor_info.clone(),
208                root,
209                signer: signer.clone(),
210                path_cache: PathCacher::new(GdrivePathQuery::new(accessor_info, signer))
211                    .with_lock(),
212            }),
213        })
214    }
215}
```
