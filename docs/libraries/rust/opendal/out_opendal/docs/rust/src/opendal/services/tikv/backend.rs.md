# 

opendal/services/tikv/

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
22use tokio::sync::OnceCell;
23
24use super::config::TikvConfig;
25use super::core::*;
26use super::deleter::TikvDeleter;
27use super::writer::TikvWriter;
28use crate::raw::*;
29use crate::*;
30
31/// TiKV backend builder
32#[doc = include_str!("docs.md")]
33#[derive(Clone, Default)]
34pub struct TikvBuilder {
35    pub(super) config: TikvConfig,
36}
37
38impl Debug for TikvBuilder {
39    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
40        let mut d = f.debug_struct("TikvBuilder");
41
42        d.field("config", &self.config);
43        d.finish_non_exhaustive()
44    }
45}
46
47impl TikvBuilder {
48    /// Set the network address of the TiKV service.
49    pub fn endpoints(mut self, endpoints: Vec<String>) -> Self {
50        if !endpoints.is_empty() {
51            self.config.endpoints = Some(endpoints)
52        }
53        self
54    }
55
56    /// Set the insecure connection to TiKV.
57    pub fn insecure(mut self) -> Self {
58        self.config.insecure = true;
59        self
60    }
61
62    /// Set the certificate authority file path.
63    pub fn ca_path(mut self, ca_path: &str) -> Self {
64        if !ca_path.is_empty() {
65            self.config.ca_path = Some(ca_path.to_string())
66        }
67        self
68    }
69
70    /// Set the certificate file path.
71    pub fn cert_path(mut self, cert_path: &str) -> Self {
72        if !cert_path.is_empty() {
73            self.config.cert_path = Some(cert_path.to_string())
74        }
75        self
76    }
77
78    /// Set the key file path.
79    pub fn key_path(mut self, key_path: &str) -> Self {
80        if !key_path.is_empty() {
81            self.config.key_path = Some(key_path.to_string())
82        }
83        self
84    }
85}
86
87impl Builder for TikvBuilder {
88    type Config = TikvConfig;
89
90    fn build(self) -> Result<impl Access> {
91        let endpoints = self.config.endpoints.ok_or_else(|| {
92            Error::new(
93                ErrorKind::ConfigInvalid,
94                "endpoints is required but not set",
95            )
96            .with_context("service", Scheme::Tikv)
97        })?;
98
99        if self.config.insecure
100            && (self.config.ca_path.is_some()
101                || self.config.key_path.is_some()
102                || self.config.cert_path.is_some())
103        {
104            return Err(
105                Error::new(ErrorKind::ConfigInvalid, "invalid tls configuration")
106                    .with_context("service", Scheme::Tikv)
107                    .with_context("endpoints", format!("{endpoints:?}")),
108            )?;
109        }
110
111        Ok(TikvBackend::new(TikvCore {
112            client: OnceCell::new(),
113            endpoints,
114            insecure: self.config.insecure,
115            ca_path: self.config.ca_path.clone(),
116            cert_path: self.config.cert_path.clone(),
117            key_path: self.config.key_path.clone(),
118        }))
119    }
120}
121
122/// Backend for TiKV service
123#[derive(Clone, Debug)]
124pub struct TikvBackend {
125    core: Arc<TikvCore>,
126    root: String,
127    info: Arc<AccessorInfo>,
128}
129
130impl TikvBackend {
131    fn new(core: TikvCore) -> Self {
132        let info = AccessorInfo::default();
133        info.set_scheme(Scheme::Tikv.into_static());
134        info.set_name("TiKV");
135        info.set_root("/");
136        info.set_native_capability(Capability {
137            read: true,
138            stat: true,
139            write: true,
140            write_can_empty: true,
141            delete: true,
142            shared: true,
143            ..Default::default()
144        });
145
146        Self {
147            core: Arc::new(core),
148            root: "/".to_string(),
149            info: Arc::new(info),
150        }
151    }
152}
153
154impl Access for TikvBackend {
155    type Reader = Buffer;
156    type Writer = TikvWriter;
157    type Lister = ();
158    type Deleter = oio::OneShotDeleter<TikvDeleter>;
159
160    fn info(&self) -> Arc<AccessorInfo> {
161        self.info.clone()
162    }
163
164    async fn stat(&self, path: &str, _: OpStat) -> Result<RpStat> {
165        let p = build_abs_path(&self.root, path);
166
167        if p == build_abs_path(&self.root, "") {
168            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
169        } else {
170            let bs = self.core.get(&p).await?;
171            match bs {
172                Some(bs) => Ok(RpStat::new(
173                    Metadata::new(EntryMode::FILE).with_content_length(bs.len() as u64),
174                )),
175                None => Err(Error::new(ErrorKind::NotFound, "kv not found in tikv")),
176            }
177        }
178    }
179
180    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
181        let p = build_abs_path(&self.root, path);
182        let bs = match self.core.get(&p).await? {
183            Some(bs) => bs,
184            None => return Err(Error::new(ErrorKind::NotFound, "kv not found in tikv")),
185        };
186        Ok((RpRead::new(), bs.slice(args.range().to_range_as_usize())))
187    }
188
189    async fn write(&self, path: &str, _: OpWrite) -> Result<(RpWrite, Self::Writer)> {
190        let p = build_abs_path(&self.root, path);
191        Ok((RpWrite::new(), TikvWriter::new(self.core.clone(), p)))
192    }
193
194    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
195        Ok((
196            RpDelete::default(),
197            oio::OneShotDeleter::new(TikvDeleter::new(self.core.clone(), self.root.clone())),
198        ))
199    }
200
201    async fn list(&self, path: &str, _: OpList) -> Result<(RpList, Self::Lister)> {
202        let _ = build_abs_path(&self.root, path);
203        Ok((RpList::default(), ()))
204    }
205}
```
