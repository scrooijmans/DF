# 

opendal/services/foundationdb/

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
18use serde::Deserialize;
19use serde::Serialize;
20
21use super::backend::FoundationdbBuilder;
22
23/// [foundationdb](https://www.foundationdb.org/) service support.
24///Config for FoundationDB.
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26#[serde(default)]
27#[non_exhaustive]
28pub struct FoundationdbConfig {
29    ///root of the backend.
30    pub root: Option<String>,
31    ///config_path for the backend.
32    pub config_path: Option<String>,
33}
34
35impl crate::Configurator for FoundationdbConfig {
36    type Builder = FoundationdbBuilder;
37
38    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
39        let mut map = uri.options().clone();
40
41        if let Some(path) = uri.root() {
42            if !path.is_empty() {
43                map.entry("config_path".to_string())
44                    .or_insert_with(|| format!("/{path}"));
45            }
46        }
47
48        Self::from_iter(map)
49    }
50
51    fn into_builder(self) -> Self::Builder {
52        FoundationdbBuilder { config: self }
53    }
54}
55
56#[cfg(test)]
57mod tests {
58    use super::*;
59    use crate::Configurator;
60    use crate::types::OperatorUri;
61
62    #[test]
63    fn from_uri_sets_config_path_and_root() {
64        let uri = OperatorUri::new(
65            "foundationdb:///etc/foundationdb/fdb.cluster?root=data",
66            Vec::<(String, String)>::new(),
67        )
68        .unwrap();
69
70        let cfg = FoundationdbConfig::from_uri(&uri).unwrap();
71        assert_eq!(
72            cfg.config_path.as_deref(),
73            Some("/etc/foundationdb/fdb.cluster")
74        );
75        assert_eq!(cfg.root.as_deref(), Some("data"));
76    }
77}
```
