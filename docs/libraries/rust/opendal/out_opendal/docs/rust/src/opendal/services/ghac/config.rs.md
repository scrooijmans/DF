# 

opendal/services/ghac/

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
19
20use super::backend::GhacBuilder;
21use serde::Deserialize;
22use serde::Serialize;
23
24/// Config for GitHub Action Cache Services support.
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26#[serde(default)]
27#[non_exhaustive]
28pub struct GhacConfig {
29    /// The root path for ghac.
30    pub root: Option<String>,
31    /// The version that used by cache.
32    pub version: Option<String>,
33    /// The endpoint for ghac service.
34    pub endpoint: Option<String>,
35    /// The runtime token for ghac service.
36    pub runtime_token: Option<String>,
37}
38
39impl crate::Configurator for GhacConfig {
40    type Builder = GhacBuilder;
41
42    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
43        let mut map = uri.options().clone();
44
45        if let Some(authority) = uri.authority() {
46            map.insert("endpoint".to_string(), format!("https://{authority}"));
47        }
48
49        if let Some(path) = uri.root() {
50            if map.contains_key("version") {
51                if !path.is_empty() {
52                    map.insert("root".to_string(), path.to_string());
53                }
54            } else if let Some((version, rest)) = path.split_once('/') {
55                if !version.is_empty() {
56                    map.insert("version".to_string(), version.to_string());
57                }
58                if !rest.is_empty() {
59                    map.insert("root".to_string(), rest.to_string());
60                }
61            } else if !path.is_empty() {
62                map.insert("version".to_string(), path.to_string());
63            }
64        }
65
66        Self::from_iter(map)
67    }
68
69    #[allow(deprecated)]
70    fn into_builder(self) -> Self::Builder {
71        GhacBuilder {
72            config: self,
73            http_client: None,
74        }
75    }
76}
77
78#[cfg(test)]
79mod tests {
80    use super::*;
81    use crate::Configurator;
82    use crate::types::OperatorUri;
83
84    #[test]
85    fn from_uri_sets_endpoint_version_and_root() {
86        let uri = OperatorUri::new(
87            "ghac://cache.githubactions.io/v1/cache-prefix",
88            Vec::<(String, String)>::new(),
89        )
90        .unwrap();
91
92        let cfg = GhacConfig::from_uri(&uri).unwrap();
93        assert_eq!(
94            cfg.endpoint.as_deref(),
95            Some("https://cache.githubactions.io")
96        );
97        assert_eq!(cfg.version.as_deref(), Some("v1"));
98        assert_eq!(cfg.root.as_deref(), Some("cache-prefix"));
99    }
100
101    #[test]
102    fn from_uri_respects_version_override() {
103        let uri = OperatorUri::new(
104            "ghac://cache.githubactions.io/cache-prefix",
105            vec![("version".to_string(), "v2".to_string())],
106        )
107        .unwrap();
108
109        let cfg = GhacConfig::from_uri(&uri).unwrap();
110        assert_eq!(cfg.version.as_deref(), Some("v2"));
111        assert_eq!(cfg.root.as_deref(), Some("cache-prefix"));
112    }
113}
```
