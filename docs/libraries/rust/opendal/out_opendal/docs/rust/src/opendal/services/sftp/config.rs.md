# 

opendal/services/sftp/

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
21use super::backend::SftpBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Sftp Service support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct SftpConfig {
30    /// endpoint of this backend
31    pub endpoint: Option<String>,
32    /// root of this backend
33    pub root: Option<String>,
34    /// user of this backend
35    pub user: Option<String>,
36    /// key of this backend
37    pub key: Option<String>,
38    /// known_hosts_strategy of this backend
39    pub known_hosts_strategy: Option<String>,
40    /// enable_copy of this backend
41    pub enable_copy: bool,
42}
43
44impl Debug for SftpConfig {
45    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
46        f.debug_struct("SftpConfig")
47            .field("endpoint", &self.endpoint)
48            .field("root", &self.root)
49            .finish_non_exhaustive()
50    }
51}
52
53impl crate::Configurator for SftpConfig {
54    type Builder = SftpBuilder;
55
56    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
57        let authority = uri.authority().ok_or_else(|| {
58            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
59                .with_context("service", crate::Scheme::Sftp)
60        })?;
61
62        let mut map = uri.options().clone();
63        map.insert("endpoint".to_string(), authority.to_string());
64
65        if let Some(root) = uri.root() {
66            map.insert("root".to_string(), root.to_string());
67        }
68
69        Self::from_iter(map)
70    }
71
72    fn into_builder(self) -> Self::Builder {
73        SftpBuilder { config: self }
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
86            "sftp://sftp.example.com/home/alice",
87            Vec::<(String, String)>::new(),
88        )
89        .unwrap();
90
91        let cfg = SftpConfig::from_uri(&uri).unwrap();
92        assert_eq!(cfg.endpoint.as_deref(), Some("sftp.example.com"));
93        assert_eq!(cfg.root.as_deref(), Some("home/alice"));
94    }
95
96    #[test]
97    fn from_uri_applies_connection_overrides() {
98        let uri = OperatorUri::new(
99            "sftp://host",
100            vec![
101                ("user".to_string(), "alice".to_string()),
102                ("key".to_string(), "/home/alice/.ssh/id_rsa".to_string()),
103                ("known_hosts_strategy".to_string(), "accept".to_string()),
104            ],
105        )
106        .unwrap();
107
108        let cfg = SftpConfig::from_uri(&uri).unwrap();
109        assert_eq!(cfg.endpoint.as_deref(), Some("host"));
110        assert_eq!(cfg.user.as_deref(), Some("alice"));
111        assert_eq!(cfg.key.as_deref(), Some("/home/alice/.ssh/id_rsa"));
112        assert_eq!(cfg.known_hosts_strategy.as_deref(), Some("accept"));
113    }
114}
```
