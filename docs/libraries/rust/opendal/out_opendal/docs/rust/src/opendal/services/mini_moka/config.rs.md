# 

opendal/services/mini_moka/

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
21use super::backend::MiniMokaBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for mini-moka support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct MiniMokaConfig {
30    /// Sets the max capacity of the cache.
31    ///
32    /// Refer to [`mini-moka::sync::CacheBuilder::max_capacity`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.max_capacity)
33    pub max_capacity: Option<u64>,
34    /// Sets the time to live of the cache.
35    ///
36    /// Refer to [`mini-moka::sync::CacheBuilder::time_to_live`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.time_to_live)
37    pub time_to_live: Option<String>,
38    /// Sets the time to idle of the cache.
39    ///
40    /// Refer to [`mini-moka::sync::CacheBuilder::time_to_idle`](https://docs.rs/mini-moka/latest/mini_moka/sync/struct.CacheBuilder.html#method.time_to_idle)
41    pub time_to_idle: Option<String>,
42
43    /// root path of this backend
44    pub root: Option<String>,
45}
46
47impl Debug for MiniMokaConfig {
48    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
49        f.debug_struct("MiniMokaConfig")
50            .field("max_capacity", &self.max_capacity)
51            .field("time_to_live", &self.time_to_live)
52            .field("time_to_idle", &self.time_to_idle)
53            .field("root", &self.root)
54            .finish()
55    }
56}
57
58impl crate::Configurator for MiniMokaConfig {
59    type Builder = MiniMokaBuilder;
60
61    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
62        let mut map = uri.options().clone();
63
64        if let Some(root) = uri.root() {
65            if !root.is_empty() {
66                map.insert("root".to_string(), root.to_string());
67            }
68        }
69
70        Self::from_iter(map)
71    }
72
73    fn into_builder(self) -> Self::Builder {
74        MiniMokaBuilder { config: self }
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
85    fn from_uri_sets_root_and_preserves_ttl() {
86        let uri = OperatorUri::new(
87            "mini-moka:///session",
88            vec![("time_to_live".to_string(), "300s".to_string())],
89        )
90        .unwrap();
91
92        let cfg = MiniMokaConfig::from_uri(&uri).unwrap();
93        assert_eq!(cfg.root.as_deref(), Some("session"));
94        assert!(cfg.time_to_live.is_some());
95    }
96}
```
