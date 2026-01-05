# 

opendal/services/ghac/

backend.rs

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
18use std::env;
19use std::sync::Arc;
20
21use http::Response;
22use http::StatusCode;
23use log::debug;
24use sha2::Digest;
25
26use super::GHAC_SCHEME;
27use super::core::*;
28use super::error::parse_error;
29use super::writer::GhacWriter;
30use crate::raw::*;
31use crate::services::GhacConfig;
32use crate::services::ghac::core::GhacCore;
33use crate::*;
34fn value_or_env(
35    explicit_value: Option<String>,
36    env_var_name: &str,
37    operation: &'static str,
38) -> Result<String> {
39    if let Some(value) = explicit_value {
40        return Ok(value);
41    }
42
43    env::var(env_var_name).map_err(|err| {
44        let text = format!("{env_var_name} not found, maybe not in github action environment?");
45        Error::new(ErrorKind::ConfigInvalid, text)
46            .with_operation(operation)
47            .set_source(err)
48    })
49}
50
51/// GitHub Action Cache Services support.
52#[doc = include_str!("docs.md")]
53#[derive(Debug, Default)]
54pub struct GhacBuilder {
55    pub(super) config: GhacConfig,
56
57    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
58    pub(super) http_client: Option<HttpClient>,
59}
60
61impl GhacBuilder {
62    /// set the working directory root of backend
63    pub fn root(mut self, root: &str) -> Self {
64        self.config.root = if root.is_empty() {
65            None
66        } else {
67            Some(root.to_string())
68        };
69
70        self
71    }
72
73    /// set the version that used by cache.
74    ///
75    /// The version is the unique value that provides namespacing.
76    /// It's better to make sure this value is only used by this backend.
77    ///
78    /// If not set, we will use `opendal` as default.
79    pub fn version(mut self, version: &str) -> Self {
80        if !version.is_empty() {
81            self.config.version = Some(version.to_string())
82        }
83
84        self
85    }
86
87    /// Set the endpoint for ghac service.
88    ///
89    /// For example, this is provided as the `ACTIONS_CACHE_URL` environment variable by the GHA runner.
90    ///
91    /// Default: the value of the `ACTIONS_CACHE_URL` environment variable.
92    pub fn endpoint(mut self, endpoint: &str) -> Self {
93        if !endpoint.is_empty() {
94            self.config.endpoint = Some(endpoint.to_string())
95        }
96        self
97    }
98
99    /// Set the runtime token for ghac service.
100    ///
101    /// For example, this is provided as the `ACTIONS_RUNTIME_TOKEN` environment variable by the GHA
102    /// runner.
103    ///
104    /// Default: the value of the `ACTIONS_RUNTIME_TOKEN` environment variable.
105    pub fn runtime_token(mut self, runtime_token: &str) -> Self {
106        if !runtime_token.is_empty() {
107            self.config.runtime_token = Some(runtime_token.to_string())
108        }
109        self
110    }
111
112    /// Specify the http client that used by this service.
113    ///
114    /// # Notes
115    ///
116    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
117    /// during minor updates.
118    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
119    #[allow(deprecated)]
120    pub fn http_client(mut self, client: HttpClient) -> Self {
121        self.http_client = Some(client);
122        self
123    }
124}
125
126impl Builder for GhacBuilder {
127    type Config = GhacConfig;
128
129    fn build(self) -> Result<impl Access> {
130        debug!("backend build started: {self:?}");
131
132        let root = normalize_root(&self.config.root.unwrap_or_default());
133        debug!("backend use root {root}");
134
135        let service_version = get_cache_service_version();
136        debug!("backend use service version {service_version:?}");
137
138        let mut version = self
139            .config
140            .version
141            .clone()
142            .unwrap_or_else(|| "opendal".to_string());
143        debug!("backend use version {version}");
144        // ghac requires to use hex digest of Sha256 as version.
145        if matches!(service_version, GhacVersion::V2) {
146            let hash = sha2::Sha256::digest(&version);
147            version = format!("{hash:x}");
148        }
149
150        let cache_url = self
151            .config
152            .endpoint
153            .unwrap_or_else(|| get_cache_service_url(service_version));
154        if cache_url.is_empty() {
155            return Err(Error::new(
156                ErrorKind::ConfigInvalid,
157                "cache url for ghac not found, maybe not in github action environment?".to_string(),
158            ));
159        }
160
161        let core = GhacCore {
162            info: {
163                let am = AccessorInfo::default();
164                am.set_scheme(GHAC_SCHEME)
165                    .set_root(&root)
166                    .set_name(&version)
167                    .set_native_capability(Capability {
168                        stat: true,
169
170                        read: true,
171
172                        write: true,
173                        write_can_multi: true,
174
175                        shared: true,
176
177                        ..Default::default()
178                    });
179
180                // allow deprecated api here for compatibility
181                #[allow(deprecated)]
182                if let Some(client) = self.http_client {
183                    am.update_http_client(|_| client);
184                }
185
186                am.into()
187            },
188            root,
189
190            cache_url,
191            catch_token: value_or_env(
192                self.config.runtime_token,
193                ACTIONS_RUNTIME_TOKEN,
194                "Builder::build",
195            )?,
196            version,
197
198            service_version,
199        };
200
201        Ok(GhacBackend {
202            core: Arc::new(core),
203        })
204    }
205}
206
207/// Backend for github action cache services.
208#[derive(Debug, Clone)]
209pub struct GhacBackend {
210    core: Arc<GhacCore>,
211}
212
213impl Access for GhacBackend {
214    type Reader = HttpBody;
215    type Writer = GhacWriter;
216    type Lister = ();
217    type Deleter = ();
218
219    fn info(&self) -> Arc<AccessorInfo> {
220        self.core.info.clone()
221    }
222
223    /// Some self-hosted GHES instances are backed by AWS S3 services which only returns
224    /// signed url with `GET` method. So we will use `GET` with empty range to simulate
225    /// `HEAD` instead.
226    ///
227    /// In this way, we can support both self-hosted GHES and `github.com`.
228    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
229        let resp = self.core.ghac_stat(path).await?;
230
231        let status = resp.status();
232        match status {
233            StatusCode::OK | StatusCode::PARTIAL_CONTENT | StatusCode::RANGE_NOT_SATISFIABLE => {
234                let mut meta = parse_into_metadata(path, resp.headers())?;
235                // Correct content length via returning content range.
236                meta.set_content_length(
237                    meta.content_range()
238                        .expect("content range must be valid")
239                        .size()
240                        .expect("content range must contains size"),
241                );
242
243                Ok(RpStat::new(meta))
244            }
245            _ => Err(parse_error(resp)),
246        }
247    }
248
249    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
250        let resp = self.core.ghac_read(path, args.range()).await?;
251
252        let status = resp.status();
253        match status {
254            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
255                Ok((RpRead::default(), resp.into_body()))
256            }
257            _ => {
258                let (part, mut body) = resp.into_parts();
259                let buf = body.to_buffer().await?;
260                Err(parse_error(Response::from_parts(part, buf)))
261            }
262        }
263    }
264
265    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
266        let url = self.core.ghac_get_upload_url(path).await?;
267
268        Ok((
269            RpWrite::default(),
270            GhacWriter::new(self.core.clone(), path.to_string(), url)?,
271        ))
272    }
273}
```
