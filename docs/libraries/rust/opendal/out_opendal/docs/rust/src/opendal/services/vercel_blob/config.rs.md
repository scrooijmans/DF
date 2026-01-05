# 

opendal/services/vercel_blob/

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
21use super::backend::VercelBlobBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for VercelBlob services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct VercelBlobConfig {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    pub root: Option<String>,
34    /// vercel blob token.
35    pub token: Option<String>,
36}
37
38impl Debug for VercelBlobConfig {
39    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
40        let mut ds = f.debug_struct("Config");
41
42        ds.field("root", &self.root);
43
44        ds.finish()
45    }
46}
47
48impl crate::Configurator for VercelBlobConfig {
49    type Builder = VercelBlobBuilder;
50
51    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
52        let mut map = uri.options().clone();
53
54        if let Some(root) = uri.root() {
55            if !root.is_empty() {
56                map.insert("root".to_string(), root.to_string());
57            }
58        }
59
60        Self::from_iter(map)
61    }
62
63    #[allow(deprecated)]
64    fn into_builder(self) -> Self::Builder {
65        VercelBlobBuilder {
66            config: self,
67            http_client: None,
68        }
69    }
70}
71
72#[cfg(test)]
73mod tests {
74    use super::*;
75    use crate::Configurator;
76    use crate::types::OperatorUri;
77
78    #[test]
79    fn from_uri_sets_root() {
80        let uri = OperatorUri::new(
81            "vercel-blob://project-assets/images",
82            Vec::<(String, String)>::new(),
83        )
84        .unwrap();
85
86        let cfg = VercelBlobConfig::from_uri(&uri).unwrap();
87        assert_eq!(cfg.root.as_deref(), Some("images"));
88    }
89}
```
