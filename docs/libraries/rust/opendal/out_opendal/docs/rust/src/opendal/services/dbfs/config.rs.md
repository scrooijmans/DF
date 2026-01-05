# 

opendal/services/dbfs/

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
21use super::backend::DbfsBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// [Dbfs](https://docs.databricks.com/api/azure/workspace/dbfs)'s REST API support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27pub struct DbfsConfig {
28    /// The root for dbfs.
29    pub root: Option<String>,
30    /// The endpoint for dbfs.
31    pub endpoint: Option<String>,
32    /// The token for dbfs.
33    pub token: Option<String>,
34}
35
36impl Debug for DbfsConfig {
37    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
38        let mut ds = f.debug_struct("DbfsConfig");
39
40        ds.field("root", &self.root);
41        ds.field("endpoint", &self.endpoint);
42
43        if self.token.is_some() {
44            ds.field("token", &"<redacted>");
45        }
46
47        ds.finish()
48    }
49}
50
51impl crate::Configurator for DbfsConfig {
52    type Builder = DbfsBuilder;
53
54    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
55        let authority = uri.authority().ok_or_else(|| {
56            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
57                .with_context("service", crate::Scheme::Dbfs)
58        })?;
59
60        let mut map = uri.options().clone();
61        map.insert("endpoint".to_string(), format!("https://{authority}"));
62
63        if let Some(root) = uri.root() {
64            if !root.is_empty() {
65                map.insert("root".to_string(), root.to_string());
66            }
67        }
68
69        Self::from_iter(map)
70    }
71
72    fn into_builder(self) -> Self::Builder {
73        DbfsBuilder { config: self }
74    }
75}
76
77#[cfg(test)]
78mod tests {
79    use super::*;
80    use crate::Configurator;
81    use crate::types::OperatorUri;
82
83    #[test]
84    fn from_uri_sets_endpoint_and_root() {
85        let uri = OperatorUri::new(
86            "dbfs://adb-1234567.azuredatabricks.net/api/2.0/dbfs/root",
87            Vec::<(String, String)>::new(),
88        )
89        .unwrap();
90
91        let cfg = DbfsConfig::from_uri(&uri).unwrap();
92        assert_eq!(
93            cfg.endpoint.as_deref(),
94            Some("https://adb-1234567.azuredatabricks.net")
95        );
96        assert_eq!(cfg.root.as_deref(), Some("api/2.0/dbfs/root"));
97    }
98}
```
