# 

opendal/services/tikv/

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
21use serde::Deserialize;
22use serde::Serialize;
23
24use super::backend::TikvBuilder;
25
26/// Config for Tikv services support.
27#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
28#[serde(default)]
29#[non_exhaustive]
30pub struct TikvConfig {
31    /// network address of the TiKV service.
32    pub endpoints: Option<Vec<String>>,
33    /// whether using insecure connection to TiKV
34    pub insecure: bool,
35    /// certificate authority file path
36    pub ca_path: Option<String>,
37    /// cert path
38    pub cert_path: Option<String>,
39    /// key path
40    pub key_path: Option<String>,
41}
42
43impl Debug for TikvConfig {
44    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
45        let mut d = f.debug_struct("TikvConfig");
46
47        d.field("endpoints", &self.endpoints)
48            .field("insecure", &self.insecure)
49            .field("ca_path", &self.ca_path)
50            .field("cert_path", &self.cert_path)
51            .field("key_path", &self.key_path)
52            .finish()
53    }
54}
55
56impl crate::Configurator for TikvConfig {
57    type Builder = TikvBuilder;
58
59    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
60        let map = uri.options().clone();
61
62        let mut endpoints = Vec::new();
63        if let Some(authority) = uri.authority() {
64            if !authority.is_empty() {
65                endpoints.push(authority.to_string());
66            }
67        }
68
69        if let Some(path) = uri.root() {
70            for segment in path.split('/') {
71                for endpoint in segment.split(',') {
72                    let trimmed = endpoint.trim();
73                    if !trimmed.is_empty() {
74                        endpoints.push(trimmed.to_string());
75                    }
76                }
77            }
78        }
79
80        let mut cfg = Self::from_iter(map)?;
81
82        if !endpoints.is_empty() {
83            if let Some(existing) = cfg.endpoints.as_mut() {
84                existing.extend(endpoints);
85            } else {
86                cfg.endpoints = Some(endpoints);
87            }
88        }
89
90        Ok(cfg)
91    }
92
93    fn into_builder(self) -> Self::Builder {
94        TikvBuilder { config: self }
95    }
96}
97
98#[cfg(test)]
99mod tests {
100    use super::*;
101    use crate::Configurator;
102    use crate::types::OperatorUri;
103
104    #[test]
105    fn from_uri_collects_endpoints() {
106        let uri = OperatorUri::new(
107            "tikv://pd1:2379/pd2:2379,pd3:2379",
108            Vec::<(String, String)>::new(),
109        )
110        .unwrap();
111
112        let cfg = TikvConfig::from_uri(&uri).unwrap();
113        assert_eq!(
114            cfg.endpoints,
115            Some(vec![
116                "pd1:2379".to_string(),
117                "pd2:2379".to_string(),
118                "pd3:2379".to_string()
119            ])
120        );
121    }
122}
```
