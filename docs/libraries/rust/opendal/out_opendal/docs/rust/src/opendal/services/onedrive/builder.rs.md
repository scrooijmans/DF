# 

opendal/services/onedrive/

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
19use services::onedrive::core::OneDriveCore;
20use services::onedrive::core::OneDriveSigner;
21use std::fmt::Debug;
22use std::fmt::Formatter;
23use std::sync::Arc;
24use tokio::sync::Mutex;
25
26use super::ONEDRIVE_SCHEME;
27use super::backend::OnedriveBackend;
28use crate::Scheme;
29use crate::raw::Access;
30use crate::raw::AccessorInfo;
31use crate::raw::HttpClient;
32use crate::raw::Timestamp;
33use crate::raw::normalize_root;
34use crate::services::OnedriveConfig;
35use crate::*;
36
37/// Microsoft [OneDrive](https://onedrive.com) backend support.
38#[doc = include_str!("docs.md")]
39#[derive(Default)]
40pub struct OnedriveBuilder {
41    pub(super) config: OnedriveConfig,
42    pub(super) http_client: Option<HttpClient>,
43}
44
45impl Debug for OnedriveBuilder {
46    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
47        f.debug_struct("Backend")
48            .field("config", &self.config)
49            .finish()
50    }
51}
52
53impl OnedriveBuilder {
54    /// Set root path of OneDrive folder.
55    pub fn root(mut self, root: &str) -> Self {
56        self.config.root = if root.is_empty() {
57            None
58        } else {
59            Some(root.to_string())
60        };
61
62        self
63    }
64
65    /// Specify the http client that used by this service.
66    ///
67    /// # Notes
68    ///
69    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
70    /// during minor updates.
71    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
72    #[allow(deprecated)]
73    pub fn http_client(mut self, http_client: HttpClient) -> Self {
74        self.http_client = Some(http_client);
75        self
76    }
77
78    /// Set the access token for a time limited access to Microsoft Graph API (also OneDrive).
79    ///
80    /// Microsoft Graph API uses a typical OAuth 2.0 flow for authentication and authorization.
81    /// You can get a access token from [Microsoft Graph Explore](https://developer.microsoft.com/en-us/graph/graph-explorer).
82    ///
83    /// # Note
84    ///
85    /// - An access token is short-lived.
86    /// - Use a refresh_token if you want to use OneDrive API for an extended period of time.
87    pub fn access_token(mut self, access_token: &str) -> Self {
88        self.config.access_token = Some(access_token.to_string());
89        self
90    }
91
92    /// Set the refresh token for long term access to Microsoft Graph API.
93    ///
94    /// OpenDAL will use a refresh token to maintain a fresh access token automatically.
95    ///
96    /// # Note
97    ///
98    /// - A refresh token is available through a OAuth 2.0 flow, with an additional scope `offline_access`.
99    pub fn refresh_token(mut self, refresh_token: &str) -> Self {
100        self.config.refresh_token = Some(refresh_token.to_string());
101        self
102    }
103
104    /// Set the client_id for a Microsoft Graph API application (available though Azure's registration portal)
105    ///
106    /// Required when using the refresh token.
107    pub fn client_id(mut self, client_id: &str) -> Self {
108        self.config.client_id = Some(client_id.to_string());
109        self
110    }
111
112    /// Set the client_secret for a Microsoft Graph API application
113    ///
114    /// Required for Web app when using the refresh token.
115    /// Don't use a client secret when use in a native app since the native app can't store the secret reliably.
116    pub fn client_secret(mut self, client_secret: &str) -> Self {
117        self.config.client_secret = Some(client_secret.to_string());
118        self
119    }
120
121    /// Enable versioning support for OneDrive
122    pub fn enable_versioning(mut self, enabled: bool) -> Self {
123        self.config.enable_versioning = enabled;
124        self
125    }
126}
127
128impl Builder for OnedriveBuilder {
129    type Config = OnedriveConfig;
130
131    fn build(self) -> Result<impl Access> {
132        let root = normalize_root(&self.config.root.unwrap_or_default());
133        debug!("backend use root {root}");
134
135        let info = AccessorInfo::default();
136        info.set_scheme(ONEDRIVE_SCHEME)
137            .set_root(&root)
138            .set_native_capability(Capability {
139                read: true,
140                read_with_if_none_match: true,
141
142                write: true,
143                write_with_if_match: true,
144                // OneDrive supports the file size up to 250GB
145                // Read more at https://support.microsoft.com/en-us/office/restrictions-and-limitations-in-onedrive-and-sharepoint-64883a5d-228e-48f5-b3d2-eb39e07630fa#individualfilesize
146                // However, we can't enable this, otherwise OpenDAL behavior tests will try to test creating huge
147                // file up to this size.
148                // write_total_max_size: Some(250 * 1024 * 1024 * 1024),
149                copy: true,
150                rename: true,
151
152                stat: true,
153                stat_with_if_none_match: true,
154                stat_with_version: self.config.enable_versioning,
155
156                delete: true,
157                create_dir: true,
158
159                list: true,
160                list_with_limit: true,
161                list_with_versions: self.config.enable_versioning,
162
163                shared: true,
164
165                ..Default::default()
166            });
167
168        // allow deprecated api here for compatibility
169        #[allow(deprecated)]
170        if let Some(client) = self.http_client {
171            info.update_http_client(|_| client);
172        }
173
174        let accessor_info = Arc::new(info);
175        let mut signer = OneDriveSigner::new(accessor_info.clone());
176
177        // Requires OAuth 2.0 tokens:
178        // - `access_token` (the short-lived token)
179        // - `refresh_token` flow (the long term token)
180        // to be mutually exclusive for setting up for implementation simplicity
181        match (self.config.access_token, self.config.refresh_token) {
182            (Some(access_token), None) => {
183                signer.access_token = access_token;
184                signer.expires_in = Timestamp::MAX;
185            }
186            (None, Some(refresh_token)) => {
187                let client_id = self.config.client_id.ok_or_else(|| {
188                    Error::new(
189                        ErrorKind::ConfigInvalid,
190                        "client_id must be set when refresh_token is set",
191                    )
192                    .with_context("service", Scheme::Onedrive)
193                })?;
194
195                signer.refresh_token = refresh_token;
196                signer.client_id = client_id;
197                if let Some(client_secret) = self.config.client_secret {
198                    signer.client_secret = client_secret;
199                }
200            }
201            (Some(_), Some(_)) => {
202                return Err(Error::new(
203                    ErrorKind::ConfigInvalid,
204                    "access_token and refresh_token cannot be set at the same time",
205                )
206                .with_context("service", Scheme::Onedrive));
207            }
208            (None, None) => {
209                return Err(Error::new(
210                    ErrorKind::ConfigInvalid,
211                    "access_token or refresh_token must be set",
212                )
213                .with_context("service", Scheme::Onedrive));
214            }
215        };
216
217        let core = Arc::new(OneDriveCore {
218            info: accessor_info,
219            root,
220            signer: Arc::new(Mutex::new(signer)),
221        });
222
223        Ok(OnedriveBackend { core })
224    }
225}
```
