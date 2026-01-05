# 

opendal/services/hdfs/

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
21use super::backend::HdfsBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// [Hadoop Distributed File System (HDFSâ¢)](https://hadoop.apache.org/) support.
26///
27/// Config for Hdfs services support.
28#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
29#[serde(default)]
30#[non_exhaustive]
31pub struct HdfsConfig {
32    /// work dir of this backend
33    pub root: Option<String>,
34    /// name node of this backend
35    pub name_node: Option<String>,
36    /// kerberos_ticket_cache_path of this backend
37    pub kerberos_ticket_cache_path: Option<String>,
38    /// user of this backend
39    pub user: Option<String>,
40    /// enable the append capacity
41    pub enable_append: bool,
42    /// atomic_write_dir of this backend
43    pub atomic_write_dir: Option<String>,
44}
45
46impl Debug for HdfsConfig {
47    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
48        f.debug_struct("HdfsConfig")
49            .field("root", &self.root)
50            .field("name_node", &self.name_node)
51            .field(
52                "kerberos_ticket_cache_path",
53                &self.kerberos_ticket_cache_path,
54            )
55            .field("user", &self.user)
56            .field("enable_append", &self.enable_append)
57            .field("atomic_write_dir", &self.atomic_write_dir)
58            .finish_non_exhaustive()
59    }
60}
61
62impl crate::Configurator for HdfsConfig {
63    type Builder = HdfsBuilder;
64
65    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
66        let authority = uri.authority().ok_or_else(|| {
67            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
68                .with_context("service", crate::Scheme::Hdfs)
69        })?;
70
71        let mut map = uri.options().clone();
72        map.insert("name_node".to_string(), format!("hdfs://{authority}"));
73
74        if let Some(root) = uri.root() {
75            if !root.is_empty() {
76                map.insert("root".to_string(), root.to_string());
77            }
78        }
79
80        Self::from_iter(map)
81    }
82
83    fn into_builder(self) -> Self::Builder {
84        HdfsBuilder { config: self }
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
95    fn from_uri_sets_name_node_and_root() {
96        let uri = OperatorUri::new(
97            "hdfs://cluster.local:8020/user/data",
98            Vec::<(String, String)>::new(),
99        )
100        .unwrap();
101
102        let cfg = HdfsConfig::from_uri(&uri).unwrap();
103        assert_eq!(cfg.name_node.as_deref(), Some("hdfs://cluster.local:8020"));
104        assert_eq!(cfg.root.as_deref(), Some("user/data"));
105    }
106}
```
