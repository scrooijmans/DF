# 

opendal/services/azdls/

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
21use super::backend::AzdlsBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Azure Data Lake Storage Gen2 Support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27pub struct AzdlsConfig {
28    /// Root of this backend.
29    pub root: Option<String>,
30    /// Filesystem name of this backend.
31    pub filesystem: String,
32    /// Endpoint of this backend.
33    pub endpoint: Option<String>,
34    /// Account name of this backend.
35    pub account_name: Option<String>,
36    /// Account key of this backend.
37    /// - required for shared_key authentication
38    pub account_key: Option<String>,
39    /// client_secret
40    /// The client secret of the service principal.
41    /// - required for client_credentials authentication
42    pub client_secret: Option<String>,
43    /// tenant_id
44    /// The tenant id of the service principal.
45    /// - required for client_credentials authentication
46    pub tenant_id: Option<String>,
47    /// client_id
48    /// The client id of the service principal.
49    /// - required for client_credentials authentication
50    pub client_id: Option<String>,
51    /// sas_token
52    /// The shared access signature token.
53    /// - required for sas authentication
54    pub sas_token: Option<String>,
55    /// authority_host
56    /// The authority host of the service principal.
57    /// - required for client_credentials authentication
58    /// - default value: `https://login.microsoftonline.com`
59    pub authority_host: Option<String>,
60}
61
62impl Debug for AzdlsConfig {
63    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
64        let mut ds = f.debug_struct("AzdlsConfig");
65
66        ds.field("root", &self.root);
67        ds.field("filesystem", &self.filesystem);
68        ds.field("endpoint", &self.endpoint);
69
70        if self.account_name.is_some() {
71            ds.field("account_name", &"<redacted>");
72        }
73        if self.account_key.is_some() {
74            ds.field("account_key", &"<redacted>");
75        }
76        if self.client_secret.is_some() {
77            ds.field("client_secret", &"<redacted>");
78        }
79        if self.tenant_id.is_some() {
80            ds.field("tenant_id", &"<redacted>");
81        }
82        if self.client_id.is_some() {
83            ds.field("client_id", &"<redacted>");
84        }
85        if self.sas_token.is_some() {
86            ds.field("sas_token", &"<redacted>");
87        }
88        ds.finish()
89    }
90}
91
92impl crate::Configurator for AzdlsConfig {
93    type Builder = AzdlsBuilder;
94
95    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
96        let authority = uri.authority().ok_or_else(|| {
97            crate::Error::new(crate::ErrorKind::ConfigInvalid, "uri authority is required")
98                .with_context("service", crate::Scheme::Azdls)
99        })?;
100
101        let mut map = uri.options().clone();
102        map.insert("endpoint".to_string(), format!("https://{authority}"));
103
104        if let Some(host) = uri.name() {
105            if let Some(account) = host.split('.').next() {
106                if !account.is_empty() {
107                    map.entry("account_name".to_string())
108                        .or_insert_with(|| account.to_string());
109                }
110            }
111        }
112
113        if let Some(root) = uri.root() {
114            if let Some((filesystem, rest)) = root.split_once('/') {
115                if filesystem.is_empty() {
116                    return Err(crate::Error::new(
117                        crate::ErrorKind::ConfigInvalid,
118                        "filesystem is required in uri path",
119                    )
120                    .with_context("service", crate::Scheme::Azdls));
121                }
122                map.insert("filesystem".to_string(), filesystem.to_string());
123                if !rest.is_empty() {
124                    map.insert("root".to_string(), rest.to_string());
125                }
126            } else if !root.is_empty() {
127                map.insert("filesystem".to_string(), root.to_string());
128            }
129        }
130
131        if !map.contains_key("filesystem") {
132            return Err(crate::Error::new(
133                crate::ErrorKind::ConfigInvalid,
134                "filesystem is required",
135            )
136            .with_context("service", crate::Scheme::Azdls));
137        }
138
139        Self::from_iter(map)
140    }
141
142    #[allow(deprecated)]
143    fn into_builder(self) -> Self::Builder {
144        AzdlsBuilder {
145            config: self,
146            http_client: None,
147        }
148    }
149}
150
151#[cfg(test)]
152mod tests {
153    use super::*;
154    use crate::Configurator;
155    use crate::types::OperatorUri;
156
157    #[test]
158    fn from_uri_sets_endpoint_filesystem_root_and_account() {
159        let uri = OperatorUri::new(
160            "azdls://account.dfs.core.windows.net/fs/data/2024",
161            Vec::<(String, String)>::new(),
162        )
163        .unwrap();
164
165        let cfg = AzdlsConfig::from_uri(&uri).unwrap();
166        assert_eq!(
167            cfg.endpoint.as_deref(),
168            Some("https://account.dfs.core.windows.net")
169        );
170        assert_eq!(cfg.filesystem, "fs".to_string());
171        assert_eq!(cfg.root.as_deref(), Some("data/2024"));
172        assert_eq!(cfg.account_name.as_deref(), Some("account"));
173    }
174
175    #[test]
176    fn from_uri_accepts_filesystem_from_query() {
177        let uri = OperatorUri::new(
178            "azdls://account.dfs.core.windows.net",
179            vec![("filesystem".to_string(), "logs".to_string())],
180        )
181        .unwrap();
182
183        let cfg = AzdlsConfig::from_uri(&uri).unwrap();
184        assert_eq!(cfg.filesystem, "logs".to_string());
185    }
186}
```
