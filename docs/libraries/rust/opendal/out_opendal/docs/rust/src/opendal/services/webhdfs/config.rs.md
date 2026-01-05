# 

opendal/services/webhdfs/

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
21use super::backend::WebhdfsBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for WebHDFS support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct WebhdfsConfig {
30    /// Root for webhdfs.
31    pub root: Option<String>,
32    /// Endpoint for webhdfs.
33    pub endpoint: Option<String>,
34    /// Name of the user for webhdfs.
35    pub user_name: Option<String>,
36    /// Delegation token for webhdfs.
37    pub delegation: Option<String>,
38    /// Disable batch listing
39    pub disable_list_batch: bool,
40    /// atomic_write_dir of this backend
41    pub atomic_write_dir: Option<String>,
42}
43
44impl Debug for WebhdfsConfig {
45    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
46        f.debug_struct("WebhdfsConfig")
47            .field("root", &self.root)
48            .field("endpoint", &self.endpoint)
49            .field("user_name", &self.user_name)
50            .field("atomic_write_dir", &self.atomic_write_dir)
51            .finish_non_exhaustive()
52    }
53}
54
55impl crate::Configurator for WebhdfsConfig {
56    type Builder = WebhdfsBuilder;
57
58    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
59        let authority = uri.authority().ok_or_else(|| {
60            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
61                .with_context("service", crate::Scheme::Webhdfs)
62        })?;
63
64        let mut map = uri.options().clone();
65        map.insert("endpoint".to_string(), format!("http://{authority}"));
66
67        if let Some(root) = uri.root() {
68            if !root.is_empty() {
69                map.insert("root".to_string(), root.to_string());
70            }
71        }
72
73        Self::from_iter(map)
74    }
75
76    fn into_builder(self) -> Self::Builder {
77        WebhdfsBuilder { config: self }
78    }
79}
80
81#[cfg(test)]
82mod tests {
83    use super::*;
84    use crate::Configurator;
85    use crate::types::OperatorUri;
86
87    #[test]
88    fn from_uri_sets_endpoint_and_root() {
89        let uri = OperatorUri::new(
90            "webhdfs://namenode.example.com:50070/user/hadoop/data",
91            vec![("user_name".to_string(), "hadoop".to_string())],
92        )
93        .unwrap();
94
95        let cfg = WebhdfsConfig::from_uri(&uri).unwrap();
96        assert_eq!(
97            cfg.endpoint.as_deref(),
98            Some("http://namenode.example.com:50070")
99        );
100        assert_eq!(cfg.root.as_deref(), Some("user/hadoop/data"));
101        assert_eq!(cfg.user_name.as_deref(), Some("hadoop"));
102    }
103}
```
