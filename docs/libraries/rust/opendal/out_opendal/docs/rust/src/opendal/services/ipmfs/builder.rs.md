# 

opendal/services/ipmfs/

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
18use std::sync::Arc;
19
20use log::debug;
21
22use super::IPMFS_SCHEME;
23use super::backend::IpmfsBackend;
24use super::core::IpmfsCore;
25use crate::raw::*;
26use crate::services::IpmfsConfig;
27use crate::*;
28
29/// IPFS file system support based on [IPFS MFS](https://docs.ipfs.tech/concepts/file-systems/) API.
30///
31/// # Capabilities
32///
33/// This service can be used to:
34///
35/// - [x] read
36/// - [x] write
37/// - [x] list
38/// - [ ] presign
39/// - [ ] blocking
40///
41/// # Configuration
42///
43/// - `root`: Set the work directory for backend
44/// - `endpoint`: Customizable endpoint setting
45///
46/// You can refer to [`IpmfsBuilder`]'s docs for more information
47///
48/// # Example
49///
50/// ## Via Builder
51///
52/// ```no_run
53/// use anyhow::Result;
54/// use opendal::services::Ipmfs;
55/// use opendal::Operator;
56///
57/// #[tokio::main]
58/// async fn main() -> Result<()> {
59///     // create backend builder
60///     let mut builder = Ipmfs::default()
61///         // set the storage bucket for OpenDAL
62///         .endpoint("http://127.0.0.1:5001");
63///
64///     let op: Operator = Operator::new(builder)?.finish();
65///
66///     Ok(())
67/// }
68/// ```
69#[derive(Default, Debug)]
70pub struct IpmfsBuilder {
71    pub(super) config: IpmfsConfig,
72
73    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
74    pub(super) http_client: Option<HttpClient>,
75}
76
77impl IpmfsBuilder {
78    /// Set root for ipfs.
79    pub fn root(mut self, root: &str) -> Self {
80        self.config.root = if root.is_empty() {
81            None
82        } else {
83            Some(root.to_string())
84        };
85
86        self
87    }
88
89    /// Set endpoint for ipfs.
90    ///
91    /// Default: http://localhost:5001
92    pub fn endpoint(mut self, endpoint: &str) -> Self {
93        self.config.endpoint = if endpoint.is_empty() {
94            None
95        } else {
96            Some(endpoint.to_string())
97        };
98        self
99    }
100
101    /// Specify the http client that used by this service.
102    ///
103    /// # Notes
104    ///
105    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
106    /// during minor updates.
107    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
108    #[allow(deprecated)]
109    pub fn http_client(mut self, client: HttpClient) -> Self {
110        self.http_client = Some(client);
111        self
112    }
113}
114
115impl Builder for IpmfsBuilder {
116    type Config = IpmfsConfig;
117
118    fn build(self) -> Result<impl Access> {
119        let root = normalize_root(&self.config.root.unwrap_or_default());
120        debug!("backend use root {root}");
121
122        let endpoint = self
123            .config
124            .endpoint
125            .clone()
126            .unwrap_or_else(|| "http://localhost:5001".to_string());
127
128        let info = AccessorInfo::default();
129        info.set_scheme(IPMFS_SCHEME)
130            .set_root(&root)
131            .set_native_capability(Capability {
132                stat: true,
133
134                read: true,
135
136                write: true,
137                delete: true,
138
139                list: true,
140
141                shared: true,
142
143                ..Default::default()
144            });
145
146        let accessor_info = Arc::new(info);
147        let core = Arc::new(IpmfsCore {
148            info: accessor_info,
149            root: root.to_string(),
150            endpoint: endpoint.to_string(),
151        });
152
153        Ok(IpmfsBackend { core })
154    }
155}
```
