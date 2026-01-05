# 

opendal/services/compfs/

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
20use super::backend::CompfsBuilder;
21use serde::Deserialize;
22use serde::Serialize;
23
24/// compio-based file system support.
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26pub struct CompfsConfig {
27    /// root of this backend.
28    ///
29    /// All operations will happen under this root.
30    pub root: Option<String>,
31}
32
33impl crate::Configurator for CompfsConfig {
34    type Builder = CompfsBuilder;
35
36    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
37        let mut map = uri.options().clone();
38
39        if let Some(root) = uri.root() {
40            if !root.is_empty() {
41                map.insert("root".to_string(), root.to_string());
42            }
43        }
44
45        Self::from_iter(map)
46    }
47
48    fn into_builder(self) -> Self::Builder {
49        CompfsBuilder { config: self }
50    }
51}
52
53#[cfg(test)]
54mod tests {
55    use super::*;
56    use crate::Configurator;
57    use crate::types::OperatorUri;
58
59    #[test]
60    fn from_uri_sets_root() {
61        let uri = OperatorUri::new("compfs:///workdir", Vec::<(String, String)>::new()).unwrap();
62
63        let cfg = CompfsConfig::from_uri(&uri).unwrap();
64        assert_eq!(cfg.root.as_deref(), Some("workdir"));
65    }
66}
```
