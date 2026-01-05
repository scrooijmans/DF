# 

opendal/services/redb/

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
20use super::backend::RedbBuilder;
21use serde::Deserialize;
22use serde::Serialize;
23
24/// Config for redb service support.
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26#[serde(default)]
27#[non_exhaustive]
28pub struct RedbConfig {
29    /// path to the redb data directory.
30    pub datadir: Option<String>,
31    /// The root for redb.
32    pub root: Option<String>,
33    /// The table name for redb.
34    pub table: Option<String>,
35}
36
37impl crate::Configurator for RedbConfig {
38    type Builder = RedbBuilder;
39    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
40        let mut map = uri.options().clone();
41
42        if let Some(path) = uri.root() {
43            if !path.is_empty() {
44                map.entry("datadir".to_string())
45                    .or_insert_with(|| format!("/{path}"));
46            }
47        }
48
49        Self::from_iter(map)
50    }
51
52    fn into_builder(self) -> Self::Builder {
53        RedbBuilder {
54            config: self,
55            database: None,
56        }
57    }
58}
59
60#[cfg(test)]
61mod tests {
62    use super::*;
63    use crate::Configurator;
64    use crate::types::OperatorUri;
65
66    #[test]
67    fn from_uri_sets_datadir_table_and_root() {
68        let uri = OperatorUri::new(
69            "redb:///tmp/redb?table=op_table&root=cache",
70            Vec::<(String, String)>::new(),
71        )
72        .unwrap();
73
74        let cfg = RedbConfig::from_uri(&uri).unwrap();
75        assert_eq!(cfg.datadir.as_deref(), Some("/tmp/redb"));
76        assert_eq!(cfg.table.as_deref(), Some("op_table"));
77        assert_eq!(cfg.root.as_deref(), Some("cache"));
78    }
79}
```
