# 

opendal/services/memory/

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
20use super::backend::MemoryBuilder;
21use serde::Deserialize;
22use serde::Serialize;
23
24/// Config for memory.
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26#[serde(default)]
27#[non_exhaustive]
28pub struct MemoryConfig {
29    /// root of the backend.
30    pub root: Option<String>,
31}
32
33impl crate::Configurator for MemoryConfig {
34    type Builder = MemoryBuilder;
35
36    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
37        let mut map = uri.options().clone();
38        if !map.contains_key("root") {
39            if let Some(root) = uri.root().filter(|v| !v.is_empty()) {
40                map.insert("root".to_string(), root.to_string());
41            }
42        }
43
44        Self::from_iter(map)
45    }
46
47    fn into_builder(self) -> Self::Builder {
48        MemoryBuilder { config: self }
49    }
50}
51
52#[cfg(test)]
53mod tests {
54    use super::*;
55    use crate::Configurator;
56    use crate::types::OperatorUri;
57
58    #[test]
59    fn from_uri_extracts_root() {
60        let uri =
61            OperatorUri::new("memory:///path/to/root", Vec::<(String, String)>::new()).unwrap();
62        let cfg = MemoryConfig::from_uri(&uri).unwrap();
63        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
64    }
65}
```
