# 

opendal/services/sled/

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
21use super::backend::SledBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Sled services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct SledConfig {
30    /// That path to the sled data directory.
31    pub datadir: Option<String>,
32    /// The root for sled.
33    pub root: Option<String>,
34    /// The tree for sled.
35    pub tree: Option<String>,
36}
37
38impl Debug for SledConfig {
39    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
40        f.debug_struct("SledConfig")
41            .field("datadir", &self.datadir)
42            .field("root", &self.root)
43            .field("tree", &self.tree)
44            .finish()
45    }
46}
47
48impl crate::Configurator for SledConfig {
49    type Builder = SledBuilder;
50    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
51        let mut map = uri.options().clone();
52
53        if let Some(path) = uri.root() {
54            if !path.is_empty() {
55                map.entry("datadir".to_string())
56                    .or_insert_with(|| format!("/{path}"));
57            }
58        }
59
60        Self::from_iter(map)
61    }
62
63    fn into_builder(self) -> Self::Builder {
64        SledBuilder { config: self }
65    }
66}
67
68#[cfg(test)]
69mod tests {
70    use super::*;
71    use crate::Configurator;
72    use crate::types::OperatorUri;
73
74    #[test]
75    fn from_uri_sets_datadir_tree_and_root() {
76        let uri = OperatorUri::new(
77            "sled:///var/data/sled?tree=cache&root=items",
78            Vec::<(String, String)>::new(),
79        )
80        .unwrap();
81
82        let cfg = SledConfig::from_uri(&uri).unwrap();
83        assert_eq!(cfg.datadir.as_deref(), Some("/var/data/sled"));
84        assert_eq!(cfg.tree.as_deref(), Some("cache"));
85        assert_eq!(cfg.root.as_deref(), Some("items"));
86    }
87}
```
