# 

opendal/services/ipmfs/

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
20use super::builder::IpmfsBuilder;
21use serde::Deserialize;
22use serde::Serialize;
23
24/// Config for IPFS MFS support.
25#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
26#[serde(default)]
27#[non_exhaustive]
28pub struct IpmfsConfig {
29    /// Root for ipfs.
30    pub root: Option<String>,
31    /// Endpoint for ipfs.
32    pub endpoint: Option<String>,
33}
34
35impl crate::Configurator for IpmfsConfig {
36    type Builder = IpmfsBuilder;
37
38    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
39        let authority = uri.authority().ok_or_else(|| {
40            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
41                .with_context("service", crate::Scheme::Ipmfs)
42        })?;
43
44        let mut map = uri.options().clone();
45        map.insert("endpoint".to_string(), format!("http://{authority}"));
46
47        if let Some(root) = uri.root() {
48            if !root.is_empty() {
49                map.insert("root".to_string(), root.to_string());
50            }
51        }
52
53        Self::from_iter(map)
54    }
55
56    #[allow(deprecated)]
57    fn into_builder(self) -> Self::Builder {
58        IpmfsBuilder {
59            config: self,
60            http_client: None,
61        }
62    }
63}
64
65#[cfg(test)]
66mod tests {
67    use super::*;
68    use crate::Configurator;
69    use crate::types::OperatorUri;
70
71    #[test]
72    fn from_uri_sets_endpoint_and_root() {
73        let uri = OperatorUri::new(
74            "ipmfs://localhost:5001/mfs/path",
75            Vec::<(String, String)>::new(),
76        )
77        .unwrap();
78
79        let cfg = IpmfsConfig::from_uri(&uri).unwrap();
80        assert_eq!(cfg.endpoint.as_deref(), Some("http://localhost:5001"));
81        assert_eq!(cfg.root.as_deref(), Some("mfs/path"));
82    }
83}
```
