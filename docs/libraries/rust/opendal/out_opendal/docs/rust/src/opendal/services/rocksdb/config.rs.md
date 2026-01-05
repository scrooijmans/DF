# 

opendal/services/rocksdb/

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
20use super::backend::RocksdbBuilder;
21use serde::Deserialize;
22use serde::Serialize;
23
24/// Config for Rocksdb Service.
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26#[serde(default)]
27#[non_exhaustive]
28pub struct RocksdbConfig {
29    /// The path to the rocksdb data directory.
30    pub datadir: Option<String>,
31    /// the working directory of the service. Can be "/path/to/dir"
32    ///
33    /// default is "/"
34    pub root: Option<String>,
35}
36
37impl crate::Configurator for RocksdbConfig {
38    type Builder = RocksdbBuilder;
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
53        RocksdbBuilder { config: self }
54    }
55}
56
57#[cfg(test)]
58mod tests {
59    use super::*;
60    use crate::Configurator;
61    use crate::types::OperatorUri;
62
63    #[test]
64    fn from_uri_sets_datadir_and_root() {
65        let uri = OperatorUri::new(
66            "rocksdb:///var/db?root=namespace",
67            Vec::<(String, String)>::new(),
68        )
69        .unwrap();
70
71        let cfg = RocksdbConfig::from_uri(&uri).unwrap();
72        assert_eq!(cfg.datadir.as_deref(), Some("/var/db"));
73        assert_eq!(cfg.root.as_deref(), Some("namespace"));
74    }
75}
```
