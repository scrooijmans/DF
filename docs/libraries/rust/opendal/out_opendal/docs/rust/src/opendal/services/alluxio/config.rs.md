# 

opendal/services/alluxio/

config.rs

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
20
21use super::backend::AlluxioBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for alluxio services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct AlluxioConfig {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    ///
34    /// default to `/` if not set.
35    pub root: Option<String>,
36    /// endpoint of this backend.
37    ///
38    /// Endpoint must be full uri, mostly like `http://127.0.0.1:39999`.
39    pub endpoint: Option<String>,
40}
41
42impl Debug for AlluxioConfig {
43    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
44        let mut d = f.debug_struct("AlluxioConfig");
45
46        d.field("root", &self.root)
47            .field("endpoint", &self.endpoint);
48
49        d.finish_non_exhaustive()
50    }
51}
52
53impl crate::Configurator for AlluxioConfig {
54    type Builder = AlluxioBuilder;
55
56    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
57        let authority = uri.authority().ok_or_else(|| {
58            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
59                .with_context("service", crate::Scheme::Alluxio)
60        })?;
61
62        let mut map = uri.options().clone();
63        map.insert("endpoint".to_string(), format!("http://{authority}"));
64
65        if let Some(root) = uri.root() {
66            if !root.is_empty() {
67                map.insert("root".to_string(), root.to_string());
68            }
69        }
70
71        Self::from_iter(map)
72    }
73
74    #[allow(deprecated)]
75    fn into_builder(self) -> Self::Builder {
76        AlluxioBuilder {
77            config: self,
78            http_client: None,
79        }
80    }
81}
82
83#[cfg(test)]
84mod tests {
85    use super::*;
86    use crate::Configurator;
87    use crate::types::OperatorUri;
88
89    #[test]
90    fn from_uri_sets_endpoint_and_root() {
91        let uri = OperatorUri::new(
92            "alluxio://127.0.0.1:39999/data/raw",
93            Vec::<(String, String)>::new(),
94        )
95        .unwrap();
96
97        let cfg = AlluxioConfig::from_uri(&uri).unwrap();
98        assert_eq!(cfg.endpoint.as_deref(), Some("http://127.0.0.1:39999"));
99        assert_eq!(cfg.root.as_deref(), Some("data/raw"));
100    }
101}
```
