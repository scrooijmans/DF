# 

opendal/services/dashmap/

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
18use super::backend::DashmapBuilder;
19use serde::Deserialize;
20use serde::Serialize;
21use std::fmt::Debug;
22
23/// Config for Dashmap services support.
24#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
25#[serde(default)]
26#[non_exhaustive]
27pub struct DashmapConfig {
28    /// root path of this backend
29    pub root: Option<String>,
30}
31
32impl Debug for DashmapConfig {
33    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
34        f.debug_struct("DashmapConfig")
35            .field("root", &self.root)
36            .finish_non_exhaustive()
37    }
38}
39
40impl crate::Configurator for DashmapConfig {
41    type Builder = DashmapBuilder;
42    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
43        let mut map = uri.options().clone();
44
45        if let Some(root) = uri.root() {
46            if !root.is_empty() {
47                map.insert("root".to_string(), root.to_string());
48            }
49        }
50
51        Self::from_iter(map)
52    }
53
54    fn into_builder(self) -> Self::Builder {
55        DashmapBuilder { config: self }
56    }
57}
58
59#[cfg(test)]
60mod tests {
61    use super::*;
62    use crate::Configurator;
63    use crate::types::OperatorUri;
64
65    #[test]
66    fn from_uri_sets_root() {
67        let uri = OperatorUri::new("dashmap:///cache", Vec::<(String, String)>::new()).unwrap();
68
69        let cfg = DashmapConfig::from_uri(&uri).unwrap();
70        assert_eq!(cfg.root.as_deref(), Some("cache"));
71    }
72}
```
