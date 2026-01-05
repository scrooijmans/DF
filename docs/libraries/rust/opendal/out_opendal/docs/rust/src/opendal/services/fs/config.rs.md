# 

opendal/services/fs/

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
20use super::backend::FsBuilder;
21use serde::Deserialize;
22use serde::Serialize;
23
24/// config for file system
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26#[serde(default)]
27#[non_exhaustive]
28pub struct FsConfig {
29    /// root dir for backend
30    pub root: Option<String>,
31
32    /// tmp dir for atomic write
33    pub atomic_write_dir: Option<String>,
34}
35
36impl crate::Configurator for FsConfig {
37    type Builder = FsBuilder;
38
39    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
40        let mut map = uri.options().clone();
41
42        if let Some(root) = uri.root().filter(|v| !v.is_empty()) {
43            map.entry("root".to_string())
44                .or_insert_with(|| format!("/{}", root));
45        }
46
47        Self::from_iter(map)
48    }
49
50    fn into_builder(self) -> Self::Builder {
51        FsBuilder { config: self }
52    }
53}
54
55#[cfg(test)]
56mod tests {
57    use super::*;
58    use crate::Configurator;
59    use crate::types::OperatorUri;
60
61    #[test]
62    fn from_uri_extracts_root() {
63        let uri = OperatorUri::new("fs:///tmp/data", Vec::<(String, String)>::new()).unwrap();
64        let cfg = FsConfig::from_uri(&uri).unwrap();
65        assert_eq!(cfg.root.as_deref(), Some("/tmp/data"));
66    }
67}
```
