# 

opendal/services/ipfs/

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
22use http::Response;
23use http::StatusCode;
24use log::debug;
25use prost::Message;
26
27use super::IPFS_SCHEME;
28use super::core::IpfsCore;
29use super::error::parse_error;
30use super::ipld::PBNode;
31use crate::raw::*;
32use crate::services::IpfsConfig;
33use crate::*;
34
35/// IPFS file system support based on [IPFS HTTP Gateway](https://docs.ipfs.tech/concepts/ipfs-gateway/).
36#[doc = include_str!("docs.md")]
37#[derive(Default, Clone, Debug)]
38pub struct IpfsBuilder {
39    pub(super) config: IpfsConfig,
40
41    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
42    pub(super) http_client: Option<HttpClient>,
43}
44
45impl IpfsBuilder {
46    /// Set root of ipfs backend.
47    ///
48    /// Root must be a valid ipfs address like the following:
49    ///
50    /// - `/ipfs/QmPpCt1aYGb9JWJRmXRUnmJtVgeFFTJGzWFYEEX7bo9zGJ/` (IPFS with CID v0)
51    /// - `/ipfs/bafybeibozpulxtpv5nhfa2ue3dcjx23ndh3gwr5vwllk7ptoyfwnfjjr4q/` (IPFS with  CID v1)
52    /// - `/ipns/opendal.apache.org/` (IPNS)
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
63    /// Set endpoint if ipfs backend.
64    ///
65    /// Endpoint must be a valid ipfs gateway which passed the [IPFS Gateway Checker](https://ipfs.github.io/public-gateway-checker/)
66    ///
67    /// Popular choices including:
68    ///
69    /// - `https://ipfs.io`
70    /// - `https://w3s.link`
71    /// - `https://dweb.link`
72    /// - `https://cloudflare-ipfs.com`
73    /// - `http://127.0.0.1:8080` (ipfs daemon in local)
74    pub fn endpoint(mut self, endpoint: &str) -> Self {
75        if !endpoint.is_empty() {
76            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
77            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string());
78        }
79
80        self
81    }
82
83    /// Specify the http client that used by this service.
84    ///
85    /// # Notes
86    ///
87    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
88    /// during minor updates.
89    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
90    #[allow(deprecated)]
91    pub fn http_client(mut self, client: HttpClient) -> Self {
92        self.http_client = Some(client);
93        self
94    }
95}
96
97impl Builder for IpfsBuilder {
98    type Config = IpfsConfig;
99
100    fn build(self) -> Result<impl Access> {
101        debug!("backend build started: {:?}", &self);
102
103        let root = normalize_root(&self.config.root.unwrap_or_default());
104        if !root.starts_with("/ipfs/") && !root.starts_with("/ipns/") {
105            return Err(Error::new(
106                ErrorKind::ConfigInvalid,
107                "root must start with /ipfs/ or /ipns/",
108            )
109            .with_context("service", Scheme::Ipfs)
110            .with_context("root", &root));
111        }
112        debug!("backend use root {root}");
113
114        let endpoint = match &self.config.endpoint {
115            Some(endpoint) => Ok(endpoint.clone()),
116            None => Err(Error::new(ErrorKind::ConfigInvalid, "endpoint is empty")
117                .with_context("service", Scheme::Ipfs)
118                .with_context("root", &root)),
119        }?;
120        debug!("backend use endpoint {}", &endpoint);
121
122        let info = AccessorInfo::default();
123        info.set_scheme(IPFS_SCHEME)
124            .set_root(&root)
125            .set_native_capability(Capability {
126                stat: true,
127
128                read: true,
129
130                list: true,
131
132                shared: true,
133
134                ..Default::default()
135            });
136
137        let accessor_info = Arc::new(info);
138        let core = Arc::new(IpfsCore {
139            info: accessor_info,
140            root,
141            endpoint,
142        });
143
144        Ok(IpfsBackend { core })
145    }
146}
147
148/// Backend for IPFS.
149#[derive(Clone)]
150pub struct IpfsBackend {
151    core: Arc<IpfsCore>,
152}
153
154impl Debug for IpfsBackend {
155    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
156        f.debug_struct("IpfsBackend")
157            .field("core", &self.core)
158            .finish()
159    }
160}
161
162impl Access for IpfsBackend {
163    type Reader = HttpBody;
164    type Writer = ();
165    type Lister = oio::PageLister<DirStream>;
166    type Deleter = ();
167
168    fn info(&self) -> Arc<AccessorInfo> {
169        self.core.info.clone()
170    }
171
172    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
173        let metadata = self.core.ipfs_stat(path).await?;
174        Ok(RpStat::new(metadata))
175    }
176
177    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
178        let resp = self.core.ipfs_get(path, args.range()).await?;
179
180        let status = resp.status();
181
182        match status {
183            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
184                Ok((RpRead::default(), resp.into_body()))
185            }
186            _ => {
187                let (part, mut body) = resp.into_parts();
188                let buf = body.to_buffer().await?;
189                Err(parse_error(Response::from_parts(part, buf)))
190            }
191        }
192    }
193
194    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
195        let l = DirStream::new(self.core.clone(), path);
196        Ok((RpList::default(), oio::PageLister::new(l)))
197    }
198}
199
200pub struct DirStream {
201    core: Arc<IpfsCore>,
202    path: String,
203}
204
205impl DirStream {
206    fn new(core: Arc<IpfsCore>, path: &str) -> Self {
207        Self {
208            core,
209            path: path.to_string(),
210        }
211    }
212}
213
214impl oio::PageList for DirStream {
215    async fn next_page(&self, ctx: &mut oio::PageContext) -> Result<()> {
216        let resp = self.core.ipfs_list(&self.path).await?;
217
218        if resp.status() != StatusCode::OK {
219            return Err(parse_error(resp));
220        }
221
222        let bs = resp.into_body();
223        let pb_node = PBNode::decode(bs).map_err(|e| {
224            Error::new(ErrorKind::Unexpected, "deserialize protobuf from response").set_source(e)
225        })?;
226
227        let names = pb_node
228            .links
229            .into_iter()
230            .map(|v| v.name.unwrap())
231            .collect::<Vec<String>>();
232
233        for mut name in names {
234            let meta = self.core.ipfs_stat(&name).await?;
235
236            if meta.mode().is_dir() {
237                name += "/";
238            }
239
240            ctx.entries.push_back(oio::Entry::new(&name, meta))
241        }
242
243        ctx.done = true;
244        Ok(())
245    }
246}
```
