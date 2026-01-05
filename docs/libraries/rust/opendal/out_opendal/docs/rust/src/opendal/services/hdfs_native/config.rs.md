# 

opendal/services/hdfs_native/

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
21use super::backend::HdfsNativeBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for HdfsNative services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct HdfsNativeConfig {
30    /// work dir of this backend
31    pub root: Option<String>,
32    /// name_node of this backend
33    pub name_node: Option<String>,
34    /// enable the append capacity
35    pub enable_append: bool,
36}
37
38impl Debug for HdfsNativeConfig {
39    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
40        f.debug_struct("HdfsNativeConfig")
41            .field("root", &self.root)
42            .field("name_node", &self.name_node)
43            .field("enable_append", &self.enable_append)
44            .finish_non_exhaustive()
45    }
46}
47
48impl crate::Configurator for HdfsNativeConfig {
49    type Builder = HdfsNativeBuilder;
50
51    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
52        let authority = uri.authority().ok_or_else(|| {
53            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
54                .with_context("service", crate::Scheme::HdfsNative)
55        })?;
56
57        let mut map = uri.options().clone();
58        map.insert("name_node".to_string(), format!("hdfs://{authority}"));
59
60        if let Some(root) = uri.root() {
61            if !root.is_empty() {
62                map.insert("root".to_string(), root.to_string());
63            }
64        }
65
66        Self::from_iter(map)
67    }
68
69    fn into_builder(self) -> Self::Builder {
70        HdfsNativeBuilder { config: self }
71    }
72}
73
74#[cfg(test)]
75mod tests {
76    use super::*;
77    use crate::Configurator;
78    use crate::types::OperatorUri;
79
80    #[test]
81    fn from_uri_sets_name_node_and_root() {
82        let uri = OperatorUri::new(
83            "hdfs-native://namenode:9000/user/project",
84            Vec::<(String, String)>::new(),
85        )
86        .unwrap();
87
88        let cfg = HdfsNativeConfig::from_uri(&uri).unwrap();
89        assert_eq!(cfg.name_node.as_deref(), Some("hdfs://namenode:9000"));
90        assert_eq!(cfg.root.as_deref(), Some("user/project"));
91    }
92}
```
