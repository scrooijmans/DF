# 

opendal/services/moka/

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
21use super::backend::MokaBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Moka services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct MokaConfig {
30    /// Name for this cache instance.
31    pub name: Option<String>,
32    /// Sets the max capacity of the cache.
33    ///
34    /// Refer to [`moka::future::CacheBuilder::max_capacity`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.max_capacity)
35    pub max_capacity: Option<u64>,
36    /// Sets the time to live of the cache.
37    ///
38    /// Refer to [`moka::future::CacheBuilder::time_to_live`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.time_to_live)
39    pub time_to_live: Option<String>,
40    /// Sets the time to idle of the cache.
41    ///
42    /// Refer to [`moka::future::CacheBuilder::time_to_idle`](https://docs.rs/moka/latest/moka/future/struct.CacheBuilder.html#method.time_to_idle)
43    pub time_to_idle: Option<String>,
44
45    /// root path of this backend
46    pub root: Option<String>,
47}
48
49impl Debug for MokaConfig {
50    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
51        f.debug_struct("MokaConfig")
52            .field("name", &self.name)
53            .field("max_capacity", &self.max_capacity)
54            .field("time_to_live", &self.time_to_live)
55            .field("time_to_idle", &self.time_to_idle)
56            .field("root", &self.root)
57            .finish_non_exhaustive()
58    }
59}
60
61impl crate::Configurator for MokaConfig {
62    type Builder = MokaBuilder;
63
64    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
65        let mut map = uri.options().clone();
66
67        if let Some(name) = uri.option("name") {
68            map.insert("name".to_string(), name.to_string());
69        }
70
71        if let Some(root) = uri.root() {
72            if !root.is_empty() {
73                map.insert("root".to_string(), root.to_string());
74            }
75        }
76
77        Self::from_iter(map)
78    }
79
80    fn into_builder(self) -> Self::Builder {
81        MokaBuilder {
82            config: self,
83            ..Default::default()
84        }
85    }
86}
87
88#[cfg(test)]
89mod tests {
90    use super::*;
91    use crate::Configurator;
92    use crate::types::OperatorUri;
93
94    #[test]
95    fn from_uri_sets_name_and_root() {
96        let uri =
97            OperatorUri::new("moka:///cache?name=session", Vec::<(String, String)>::new()).unwrap();
98
99        let cfg = MokaConfig::from_uri(&uri).unwrap();
100        assert_eq!(cfg.name.as_deref(), Some("session"));
101        assert_eq!(cfg.root.as_deref(), Some("cache"));
102    }
103}
```
