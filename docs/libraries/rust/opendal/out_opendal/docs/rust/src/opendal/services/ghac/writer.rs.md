# 

opendal/services/ghac/

writer.rs

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
18use std::str::FromStr;
19use std::sync::Arc;
20
21use super::core::*;
22use super::error::parse_error;
23use crate::raw::*;
24use crate::services::core::AzblobCore;
25use crate::services::writer::AzblobWriter;
26use crate::*;
27
28pub type GhacWriter = TwoWays<GhacWriterV1, GhacWriterV2>;
29
30impl GhacWriter {
31    /// TODO: maybe we can move the signed url logic to azblob service instead.
32    pub fn new(core: Arc<GhacCore>, write_path: String, url: String) -> Result<Self> {
33        match core.service_version {
34            GhacVersion::V1 => Ok(TwoWays::One(GhacWriterV1 {
35                core,
36                path: write_path,
37                url,
38                size: 0,
39            })),
40            GhacVersion::V2 => {
41                let uri = http::Uri::from_str(&url)
42                    .map_err(new_http_uri_invalid_error)?
43                    .into_parts();
44                let (Some(scheme), Some(authority), Some(pq)) =
45                    (uri.scheme, uri.authority, uri.path_and_query)
46                else {
47                    return Err(Error::new(
48                        ErrorKind::Unexpected,
49                        "ghac returns invalid signed url",
50                    )
51                    .with_context("url", &url));
52                };
53                let endpoint = format!("{scheme}://{authority}");
54                let Some((container, path)) = pq.path().trim_matches('/').split_once("/") else {
55                    return Err(Error::new(
56                        ErrorKind::Unexpected,
57                        "ghac returns invalid signed url that bucket or path is missing",
58                    )
59                    .with_context("url", &url));
60                };
61                let Some(query) = pq.query() else {
62                    return Err(Error::new(
63                        ErrorKind::Unexpected,
64                        "ghac returns invalid signed url that sas is missing",
65                    )
66                    .with_context("url", &url));
67                };
68                let azure_core = Arc::new(AzblobCore {
69                    info: {
70                        let am = AccessorInfo::default();
71                        am.set_scheme("azblob")
72                            .set_root("/")
73                            .set_name(container)
74                            .set_native_capability(Capability {
75                                stat: true,
76                                stat_with_if_match: true,
77                                stat_with_if_none_match: true,
78
79                                read: true,
80
81                                read_with_if_match: true,
82                                read_with_if_none_match: true,
83                                read_with_override_content_disposition: true,
84                                read_with_if_modified_since: true,
85                                read_with_if_unmodified_since: true,
86
87                                write: true,
88                                write_can_append: true,
89                                write_can_empty: true,
90                                write_can_multi: true,
91                                write_with_cache_control: true,
92                                write_with_content_type: true,
93                                write_with_if_not_exists: true,
94                                write_with_if_none_match: true,
95                                write_with_user_metadata: true,
96
97                                copy: true,
98
99                                list: true,
100                                list_with_recursive: true,
101
102                                shared: true,
103
104                                ..Default::default()
105                            });
106
107                        am.into()
108                    },
109                    container: container.to_string(),
110                    root: "/".to_string(),
111                    endpoint,
112                    encryption_key: None,
113                    encryption_key_sha256: None,
114                    encryption_algorithm: None,
115                    loader: {
116                        let config = reqsign::AzureStorageConfig {
117                            sas_token: Some(query.to_string()),
118                            ..Default::default()
119                        };
120                        reqsign::AzureStorageLoader::new(config)
121                    },
122                    signer: { reqsign::AzureStorageSigner::new() },
123                });
124                let w = AzblobWriter::new(azure_core, OpWrite::default(), path.to_string());
125                let writer = oio::BlockWriter::new(core.info.clone(), w, 4);
126                Ok(TwoWays::Two(GhacWriterV2 {
127                    core,
128                    writer,
129                    path: write_path,
130                    url,
131                    size: 0,
132                }))
133            }
134        }
135    }
136}
137
138pub struct GhacWriterV1 {
139    core: Arc<GhacCore>,
140
141    path: String,
142    url: String,
143    size: u64,
144}
145
146impl oio::Write for GhacWriterV1 {
147    async fn write(&mut self, bs: Buffer) -> Result<()> {
148        let size = bs.len() as u64;
149        let offset = self.size;
150
151        let resp = self.core.ghac_v1_write(&self.url, size, offset, bs).await?;
152        if !resp.status().is_success() {
153            return Err(parse_error(resp).map(|err| err.with_operation("Backend::ghac_upload")));
154        }
155        self.size += size;
156        Ok(())
157    }
158
159    async fn abort(&mut self) -> Result<()> {
160        Ok(())
161    }
162
163    async fn close(&mut self) -> Result<Metadata> {
164        self.core
165            .ghac_finalize_upload(&self.path, &self.url, self.size)
166            .await?;
167        Ok(Metadata::default().with_content_length(self.size))
168    }
169}
170
171pub struct GhacWriterV2 {
172    core: Arc<GhacCore>,
173    writer: oio::BlockWriter<AzblobWriter>,
174
175    path: String,
176    url: String,
177    size: u64,
178}
179
180impl oio::Write for GhacWriterV2 {
181    async fn write(&mut self, bs: Buffer) -> Result<()> {
182        let size = bs.len() as u64;
183
184        self.writer.write(bs).await?;
185        self.size += size;
186        Ok(())
187    }
188
189    async fn close(&mut self) -> Result<Metadata> {
190        self.writer.close().await?;
191        let _ = self
192            .core
193            .ghac_finalize_upload(&self.path, &self.url, self.size)
194            .await;
195        Ok(Metadata::default().with_content_length(self.size))
196    }
197
198    async fn abort(&mut self) -> Result<()> {
199        Ok(())
200    }
201}
```
