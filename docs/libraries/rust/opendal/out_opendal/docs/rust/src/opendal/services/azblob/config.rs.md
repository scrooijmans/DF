# 

opendal/services/azblob/

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
21use super::backend::AzblobBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Azure Storage Blob services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27pub struct AzblobConfig {
28    /// The root of Azblob service backend.
29    ///
30    /// All operations will happen under this root.
31    pub root: Option<String>,
32
33    /// The container name of Azblob service backend.
34    #[serde(alias = "azure_container_name", alias = "container_name")]
35    pub container: String,
36
37    /// The endpoint of Azblob service backend.
38    ///
39    /// Endpoint must be full uri, e.g.
40    ///
41    /// - Azblob: `https://accountname.blob.core.windows.net`
42    /// - Azurite: `http://127.0.0.1:10000/devstoreaccount1`
43    #[serde(alias = "azure_storage_endpoint", alias = "azure_endpoint")]
44    pub endpoint: Option<String>,
45
46    /// The account name of Azblob service backend.
47    #[serde(alias = "azure_storage_account_name")]
48    pub account_name: Option<String>,
49
50    /// The account key of Azblob service backend.
51    #[serde(
52        alias = "azure_storage_account_key",
53        alias = "azure_storage_access_key",
54        alias = "azure_storage_master_key",
55        alias = "access_key",
56        alias = "master_key"
57    )]
58    pub account_key: Option<String>,
59
60    /// The encryption key of Azblob service backend.
61    pub encryption_key: Option<String>,
62
63    /// The encryption key sha256 of Azblob service backend.
64    pub encryption_key_sha256: Option<String>,
65
66    /// The encryption algorithm of Azblob service backend.
67    pub encryption_algorithm: Option<String>,
68
69    /// The sas token of Azblob service backend.
70    #[serde(
71        alias = "azure_storage_sas_key",
72        alias = "azure_storage_sas_token",
73        alias = "sas_key"
74    )]
75    pub sas_token: Option<String>,
76
77    /// The maximum batch operations of Azblob service backend.
78    pub batch_max_operations: Option<usize>,
79}
80
81impl Debug for AzblobConfig {
82    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
83        let mut ds = f.debug_struct("AzblobConfig");
84
85        ds.field("root", &self.root);
86        ds.field("container", &self.container);
87        ds.field("endpoint", &self.endpoint);
88
89        if self.account_name.is_some() {
90            ds.field("account_name", &"<redacted>");
91        }
92        if self.account_key.is_some() {
93            ds.field("account_key", &"<redacted>");
94        }
95        if self.sas_token.is_some() {
96            ds.field("sas_token", &"<redacted>");
97        }
98
99        ds.finish()
100    }
101}
102
103impl crate::Configurator for AzblobConfig {
104    type Builder = AzblobBuilder;
105
106    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
107        let mut map = uri.options().clone();
108
109        if let Some(container) = uri.name() {
110            map.insert("container".to_string(), container.to_string());
111        }
112
113        if let Some(root) = uri.root() {
114            map.insert("root".to_string(), root.to_string());
115        }
116
117        Self::from_iter(map)
118    }
119
120    #[allow(deprecated)]
121    fn into_builder(self) -> Self::Builder {
122        AzblobBuilder {
123            config: self,
124
125            http_client: None,
126        }
127    }
128}
129
130#[cfg(test)]
131mod tests {
132    use super::*;
133    use crate::Configurator;
134    use crate::types::OperatorUri;
135
136    #[test]
137    fn test_container_name_aliases() {
138        let json = r#"{"container": "test-container"}"#;
139        let config: AzblobConfig = serde_json::from_str(json).unwrap();
140        assert_eq!(config.container, "test-container");
141
142        let json = r#"{"azure_container_name": "test-container"}"#;
143        let config: AzblobConfig = serde_json::from_str(json).unwrap();
144        assert_eq!(config.container, "test-container");
145
146        let json = r#"{"container_name": "test-container"}"#;
147        let config: AzblobConfig = serde_json::from_str(json).unwrap();
148        assert_eq!(config.container, "test-container");
149    }
150
151    #[test]
152    fn test_account_name_aliases() {
153        let json = r#"{"container": "test", "account_name": "testaccount"}"#;
154        let config: AzblobConfig = serde_json::from_str(json).unwrap();
155        assert_eq!(config.account_name, Some("testaccount".to_string()));
156
157        let json = r#"{"container": "test", "azure_storage_account_name": "testaccount-azure"}"#;
158        let config: AzblobConfig = serde_json::from_str(json).unwrap();
159        assert_eq!(config.account_name, Some("testaccount-azure".to_string()));
160    }
161
162    #[test]
163    fn test_account_key_aliases() {
164        let json = r#"{"container": "test", "account_key": "dGVzdGtleQ=="}"#;
165        let config: AzblobConfig = serde_json::from_str(json).unwrap();
166        assert_eq!(config.account_key, Some("dGVzdGtleQ==".to_string()));
167
168        let json = r#"{"container": "test", "azure_storage_account_key": "dGVzdGtleQ=="}"#;
169        let config: AzblobConfig = serde_json::from_str(json).unwrap();
170        assert_eq!(config.account_key, Some("dGVzdGtleQ==".to_string()));
171
172        let json = r#"{"container": "test", "azure_storage_access_key": "dGVzdGtleQ=="}"#;
173        let config: AzblobConfig = serde_json::from_str(json).unwrap();
174        assert_eq!(config.account_key, Some("dGVzdGtleQ==".to_string()));
175
176        let json = r#"{"container": "test", "azure_storage_master_key": "dGVzdGtleQ=="}"#;
177        let config: AzblobConfig = serde_json::from_str(json).unwrap();
178        assert_eq!(config.account_key, Some("dGVzdGtleQ==".to_string()));
179
180        let json = r#"{"container": "test", "access_key": "dGVzdGtleQ=="}"#;
181        let config: AzblobConfig = serde_json::from_str(json).unwrap();
182        assert_eq!(config.account_key, Some("dGVzdGtleQ==".to_string()));
183
184        let json = r#"{"container": "test", "master_key": "dGVzdGtleQ=="}"#;
185        let config: AzblobConfig = serde_json::from_str(json).unwrap();
186        assert_eq!(config.account_key, Some("dGVzdGtleQ==".to_string()));
187    }
188
189    #[test]
190    fn test_sas_token_aliases() {
191        let json = r#"{"container": "test", "sas_token": "test-token"}"#;
192        let config: AzblobConfig = serde_json::from_str(json).unwrap();
193        assert_eq!(config.sas_token, Some("test-token".to_string()));
194
195        let json = r#"{"container": "test", "azure_storage_sas_key": "test-token"}"#;
196        let config: AzblobConfig = serde_json::from_str(json).unwrap();
197        assert_eq!(config.sas_token, Some("test-token".to_string()));
198
199        let json = r#"{"container": "test", "azure_storage_sas_token": "test-token"}"#;
200        let config: AzblobConfig = serde_json::from_str(json).unwrap();
201        assert_eq!(config.sas_token, Some("test-token".to_string()));
202
203        let json = r#"{"container": "test", "sas_key": "test-token"}"#;
204        let config: AzblobConfig = serde_json::from_str(json).unwrap();
205        assert_eq!(config.sas_token, Some("test-token".to_string()));
206    }
207
208    #[test]
209    fn test_endpoint_aliases() {
210        let json = r#"{"container": "test", "endpoint": "https://test.blob.core.windows.net"}"#;
211        let config: AzblobConfig = serde_json::from_str(json).unwrap();
212        assert_eq!(
213            config.endpoint,
214            Some("https://test.blob.core.windows.net".to_string())
215        );
216
217        let json = r#"{"container": "test", "azure_storage_endpoint": "https://test.blob.core.windows.net"}"#;
218        let config: AzblobConfig = serde_json::from_str(json).unwrap();
219        assert_eq!(
220            config.endpoint,
221            Some("https://test.blob.core.windows.net".to_string())
222        );
223
224        let json =
225            r#"{"container": "test", "azure_endpoint": "https://test.blob.core.windows.net"}"#;
226        let config: AzblobConfig = serde_json::from_str(json).unwrap();
227        assert_eq!(
228            config.endpoint,
229            Some("https://test.blob.core.windows.net".to_string())
230        );
231    }
232
233    #[test]
234    fn from_uri_with_host_container() {
235        let uri = OperatorUri::new(
236            "azblob://my-container/path/to/root",
237            Vec::<(String, String)>::new(),
238        )
239        .unwrap();
240        let cfg = AzblobConfig::from_uri(&uri).unwrap();
241        assert_eq!(cfg.container, "my-container");
242        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
243    }
244
245    #[test]
246    fn from_uri_with_path_container() {
247        let uri = OperatorUri::new(
248            "azblob://my-container/nested/root",
249            Vec::<(String, String)>::new(),
250        )
251        .unwrap();
252        let cfg = AzblobConfig::from_uri(&uri).unwrap();
253        assert_eq!(cfg.container, "my-container");
254        assert_eq!(cfg.root.as_deref(), Some("nested/root"));
255    }
256}
```
