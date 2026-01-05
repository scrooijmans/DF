# 

opendal/services/etcd/

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
21use super::backend::EtcdBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Etcd services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct EtcdConfig {
30    /// network address of the Etcd services.
31    /// If use https, must set TLS options: `ca_path`, `cert_path`, `key_path`.
32    /// e.g. "127.0.0.1:23790,127.0.0.1:23791,127.0.0.1:23792" or "http://127.0.0.1:23790,http://127.0.0.1:23791,http://127.0.0.1:23792" or "https://127.0.0.1:23790,https://127.0.0.1:23791,https://127.0.0.1:23792"
33    ///
34    /// default is "http://127.0.0.1:2379"
35    pub endpoints: Option<String>,
36    /// the username to connect etcd service.
37    ///
38    /// default is None
39    pub username: Option<String>,
40    /// the password for authentication
41    ///
42    /// default is None
43    pub password: Option<String>,
44    /// the working directory of the etcd service. Can be "/path/to/dir"
45    ///
46    /// default is "/"
47    pub root: Option<String>,
48    /// certificate authority file path
49    ///
50    /// default is None
51    pub ca_path: Option<String>,
52    /// cert path
53    ///
54    /// default is None
55    pub cert_path: Option<String>,
56    /// key path
57    ///
58    /// default is None
59    pub key_path: Option<String>,
60}
61
62impl Debug for EtcdConfig {
63    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
64        let mut ds = f.debug_struct("EtcdConfig");
65
66        ds.field("root", &self.root);
67        if let Some(endpoints) = self.endpoints.clone() {
68            ds.field("endpoints", &endpoints);
69        }
70        if let Some(username) = self.username.clone() {
71            ds.field("username", &username);
72        }
73        if self.password.is_some() {
74            ds.field("password", &"<redacted>");
75        }
76        if let Some(ca_path) = self.ca_path.clone() {
77            ds.field("ca_path", &ca_path);
78        }
79        if let Some(cert_path) = self.cert_path.clone() {
80            ds.field("cert_path", &cert_path);
81        }
82        if let Some(key_path) = self.key_path.clone() {
83            ds.field("key_path", &key_path);
84        }
85        ds.finish()
86    }
87}
88
89impl crate::Configurator for EtcdConfig {
90    type Builder = EtcdBuilder;
91    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
92        let mut map = uri.options().clone();
93
94        if let Some(authority) = uri.authority() {
95            map.entry("endpoints".to_string())
96                .or_insert_with(|| format!("http://{authority}"));
97        }
98
99        if let Some(root) = uri.root() {
100            if !root.is_empty() {
101                map.insert("root".to_string(), root.to_string());
102            }
103        }
104
105        Self::from_iter(map)
106    }
107
108    fn into_builder(self) -> Self::Builder {
109        EtcdBuilder { config: self }
110    }
111}
112
113#[cfg(test)]
114mod tests {
115    use super::*;
116    use crate::Configurator;
117    use crate::types::OperatorUri;
118
119    #[test]
120    fn from_uri_sets_endpoints_and_root() {
121        let uri = OperatorUri::new(
122            "etcd://127.0.0.1:2379/app/config",
123            Vec::<(String, String)>::new(),
124        )
125        .unwrap();
126
127        let cfg = EtcdConfig::from_uri(&uri).unwrap();
128        assert_eq!(cfg.endpoints.as_deref(), Some("http://127.0.0.1:2379"));
129        assert_eq!(cfg.root.as_deref(), Some("app/config"));
130    }
131}
```
