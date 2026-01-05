# 

opendal/services/cacache/

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
20use super::backend::CacacheBuilder;
21use serde::Deserialize;
22use serde::Serialize;
23
24/// cacache service support.
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26pub struct CacacheConfig {
27    /// That path to the cacache data directory.
28    pub datadir: Option<String>,
29}
30
31impl crate::Configurator for CacacheConfig {
32    type Builder = CacacheBuilder;
33    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
34        let mut map = uri.options().clone();
35
36        if let Some(root) = uri.root() {
37            if !root.is_empty() {
38                map.entry("datadir".to_string())
39                    .or_insert_with(|| format!("/{root}"));
40            }
41        }
42
43        Self::from_iter(map)
44    }
45
46    fn into_builder(self) -> Self::Builder {
47        CacacheBuilder { config: self }
48    }
49}
50
51#[cfg(test)]
52mod tests {
53    use super::*;
54    use crate::Configurator;
55    use crate::types::OperatorUri;
56
57    #[test]
58    fn from_uri_sets_datadir_from_authority() {
59        let uri = OperatorUri::new(
60            "cacache:///var/cache/opendal",
61            Vec::<(String, String)>::new(),
62        )
63        .unwrap();
64
65        let cfg = CacacheConfig::from_uri(&uri).unwrap();
66        assert_eq!(cfg.datadir.as_deref(), Some("/var/cache/opendal"));
67    }
68
69    #[test]
70    fn from_uri_falls_back_to_path() {
71        let uri = OperatorUri::new("cacache:///tmp/cache", Vec::<(String, String)>::new()).unwrap();
72
73        let cfg = CacacheConfig::from_uri(&uri).unwrap();
74        assert_eq!(cfg.datadir.as_deref(), Some("/tmp/cache"));
75    }
76}
```
