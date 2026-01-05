# 

opendal/services/huggingface/

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
18use std::fmt::Debug;
19use std::fmt::Formatter;
20use std::sync::Arc;
21
22use bytes::Buf;
23use http::Response;
24use http::StatusCode;
25use log::debug;
26
27use super::HUGGINGFACE_SCHEME;
28use super::core::HuggingfaceCore;
29use super::core::HuggingfaceStatus;
30use super::error::parse_error;
31use super::lister::HuggingfaceLister;
32use crate::raw::*;
33use crate::services::HuggingfaceConfig;
34use crate::*;
35
36/// [Huggingface](https://huggingface.co/docs/huggingface_hub/package_reference/hf_api)'s API support.
37#[doc = include_str!("docs.md")]
38#[derive(Default, Clone)]
39pub struct HuggingfaceBuilder {
40    pub(super) config: HuggingfaceConfig,
41}
42
43impl Debug for HuggingfaceBuilder {
44    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
45        let mut ds = f.debug_struct("Builder");
46
47        ds.field("config", &self.config);
48        ds.finish()
49    }
50}
51
52impl HuggingfaceBuilder {
53    /// Set repo type of this backend. Default is model.
54    ///
55    /// Available values:
56    /// - model
57    /// - dataset
58    ///
59    /// Currently, only models and datasets are supported.
60    /// [Reference](https://huggingface.co/docs/hub/repositories)
61    pub fn repo_type(mut self, repo_type: &str) -> Self {
62        if !repo_type.is_empty() {
63            self.config.repo_type = Some(repo_type.to_string());
64        }
65        self
66    }
67
68    /// Set repo id of this backend. This is required.
69    ///
70    /// Repo id consists of the account name and the repository name.
71    ///
72    /// For example, model's repo id looks like:
73    /// - meta-llama/Llama-2-7b
74    ///
75    /// Dataset's repo id looks like:
76    /// - databricks/databricks-dolly-15k
77    pub fn repo_id(mut self, repo_id: &str) -> Self {
78        if !repo_id.is_empty() {
79            self.config.repo_id = Some(repo_id.to_string());
80        }
81        self
82    }
83
84    /// Set revision of this backend. Default is main.
85    ///
86    /// Revision can be a branch name or a commit hash.
87    ///
88    /// For example, revision can be:
89    /// - main
90    /// - 1d0c4eb
91    pub fn revision(mut self, revision: &str) -> Self {
92        if !revision.is_empty() {
93            self.config.revision = Some(revision.to_string());
94        }
95        self
96    }
97
98    /// Set root of this backend.
99    ///
100    /// All operations will happen under this root.
101    pub fn root(mut self, root: &str) -> Self {
102        self.config.root = if root.is_empty() {
103            None
104        } else {
105            Some(root.to_string())
106        };
107
108        self
109    }
110
111    /// Set the token of this backend.
112    ///
113    /// This is optional.
114    pub fn token(mut self, token: &str) -> Self {
115        if !token.is_empty() {
116            self.config.token = Some(token.to_string());
117        }
118        self
119    }
120}
121
122impl Builder for HuggingfaceBuilder {
123    type Config = HuggingfaceConfig;
124
125    /// Build a HuggingfaceBackend.
126    fn build(self) -> Result<impl Access> {
127        debug!("backend build started: {:?}", &self);
128
129        let repo_type = match self.config.repo_type.as_deref() {
130            Some("model") => Ok(RepoType::Model),
131            Some("dataset") => Ok(RepoType::Dataset),
132            Some("space") => Err(Error::new(
133                ErrorKind::ConfigInvalid,
134                "repo type \"space\" is unsupported",
135            )),
136            Some(repo_type) => Err(Error::new(
137                ErrorKind::ConfigInvalid,
138                format!("unknown repo_type: {repo_type}").as_str(),
139            )
140            .with_operation("Builder::build")
141            .with_context("service", Scheme::Huggingface)),
142            None => Ok(RepoType::Model),
143        }?;
144        debug!("backend use repo_type: {:?}", &repo_type);
145
146        let repo_id = match &self.config.repo_id {
147            Some(repo_id) => Ok(repo_id.clone()),
148            None => Err(Error::new(ErrorKind::ConfigInvalid, "repo_id is empty")
149                .with_operation("Builder::build")
150                .with_context("service", Scheme::Huggingface)),
151        }?;
152        debug!("backend use repo_id: {}", &repo_id);
153
154        let revision = match &self.config.revision {
155            Some(revision) => revision.clone(),
156            None => "main".to_string(),
157        };
158        debug!("backend use revision: {}", &revision);
159
160        let root = normalize_root(&self.config.root.unwrap_or_default());
161        debug!("backend use root: {}", &root);
162
163        let token = self.config.token.as_ref().cloned();
164
165        Ok(HuggingfaceBackend {
166            core: Arc::new(HuggingfaceCore {
167                info: {
168                    let am = AccessorInfo::default();
169                    am.set_scheme(HUGGINGFACE_SCHEME)
170                        .set_native_capability(Capability {
171                            stat: true,
172
173                            read: true,
174
175                            list: true,
176                            list_with_recursive: true,
177
178                            shared: true,
179
180                            ..Default::default()
181                        });
182                    am.into()
183                },
184                repo_type,
185                repo_id,
186                revision,
187                root,
188                token,
189            }),
190        })
191    }
192}
193
194/// Backend for Huggingface service
195#[derive(Debug, Clone)]
196pub struct HuggingfaceBackend {
197    core: Arc<HuggingfaceCore>,
198}
199
200impl Access for HuggingfaceBackend {
201    type Reader = HttpBody;
202    type Writer = ();
203    type Lister = oio::PageLister<HuggingfaceLister>;
204    type Deleter = ();
205
206    fn info(&self) -> Arc<AccessorInfo> {
207        self.core.info.clone()
208    }
209
210    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
211        // Stat root always returns a DIR.
212        if path == "/" {
213            return Ok(RpStat::new(Metadata::new(EntryMode::DIR)));
214        }
215
216        let resp = self.core.hf_path_info(path).await?;
217
218        let status = resp.status();
219
220        match status {
221            StatusCode::OK => {
222                let mut meta = parse_into_metadata(path, resp.headers())?;
223                let bs = resp.into_body();
224
225                let decoded_response: Vec<HuggingfaceStatus> =
226                    serde_json::from_reader(bs.reader()).map_err(new_json_deserialize_error)?;
227
228                // NOTE: if the file is not found, the server will return 200 with an empty array
229                if let Some(status) = decoded_response.first() {
230                    if let Some(commit_info) = status.last_commit.as_ref() {
231                        meta.set_last_modified(commit_info.date.parse::<Timestamp>()?);
232                    }
233
234                    meta.set_content_length(status.size);
235
236                    match status.type_.as_str() {
237                        "directory" => meta.set_mode(EntryMode::DIR),
238                        "file" => meta.set_mode(EntryMode::FILE),
239                        _ => return Err(Error::new(ErrorKind::Unexpected, "unknown status type")),
240                    };
241                } else {
242                    return Err(Error::new(ErrorKind::NotFound, "path not found"));
243                }
244
245                Ok(RpStat::new(meta))
246            }
247            _ => Err(parse_error(resp)),
248        }
249    }
250
251    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
252        let resp = self.core.hf_resolve(path, args.range(), &args).await?;
253
254        let status = resp.status();
255
256        match status {
257            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
258                Ok((RpRead::default(), resp.into_body()))
259            }
260            _ => {
261                let (part, mut body) = resp.into_parts();
262                let buf = body.to_buffer().await?;
263                Err(parse_error(Response::from_parts(part, buf)))
264            }
265        }
266    }
267
268    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
269        let l = HuggingfaceLister::new(self.core.clone(), path.to_string(), args.recursive());
270
271        Ok((RpList::default(), oio::PageLister::new(l)))
272    }
273}
274
275/// Repository type of Huggingface. Currently, we only support `model` and `dataset`.
276/// [Reference](https://huggingface.co/docs/hub/repositories)
277#[derive(Debug, Clone, Copy)]
278pub enum RepoType {
279    Model,
280    Dataset,
281}
```
