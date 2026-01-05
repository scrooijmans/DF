# 

opendal/services/gcs/

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
21use super::backend::GcsBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// [Google Cloud Storage](https://cloud.google.com/storage) services support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct GcsConfig {
30    /// root URI, all operations happens under `root`
31    pub root: Option<String>,
32    /// bucket name
33    #[serde(
34        alias = "google_bucket",
35        alias = "google_bucket_name",
36        alias = "bucket_name"
37    )]
38    pub bucket: String,
39    /// endpoint URI of GCS service,
40    /// default is `https://storage.googleapis.com`
41    pub endpoint: Option<String>,
42    /// Scope for gcs.
43    pub scope: Option<String>,
44    /// Service Account for gcs.
45    #[serde(
46        alias = "google_service_account",
47        alias = "google_service_account_path",
48        alias = "service_account_path"
49    )]
50    pub service_account: Option<String>,
51    /// Credentials string for GCS service OAuth2 authentication.
52    #[serde(alias = "google_service_account_key", alias = "service_account_key")]
53    pub credential: Option<String>,
54    /// Local path to credentials file for GCS service OAuth2 authentication.
55    #[serde(alias = "google_application_credentials")]
56    pub credential_path: Option<String>,
57    /// The predefined acl for GCS.
58    pub predefined_acl: Option<String>,
59    /// The default storage class used by gcs.
60    pub default_storage_class: Option<String>,
61    /// Allow opendal to send requests without signing when credentials are not
62    /// loaded.
63    #[serde(alias = "google_skip_signature", alias = "skip_signature")]
64    pub allow_anonymous: bool,
65    /// Disable attempting to load credentials from the GCE metadata server when
66    /// running within Google Cloud.
67    pub disable_vm_metadata: bool,
68    /// Disable loading configuration from the environment.
69    pub disable_config_load: bool,
70    /// A Google Cloud OAuth2 token.
71    ///
72    /// Takes precedence over `credential` and `credential_path`.
73    pub token: Option<String>,
74}
75
76impl Debug for GcsConfig {
77    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
78        f.debug_struct("GcsConfig")
79            .field("root", &self.root)
80            .field("bucket", &self.bucket)
81            .field("endpoint", &self.endpoint)
82            .field("scope", &self.scope)
83            .finish_non_exhaustive()
84    }
85}
86
87impl crate::Configurator for GcsConfig {
88    type Builder = GcsBuilder;
89
90    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
91        let mut map = uri.options().clone();
92
93        if let Some(name) = uri.name() {
94            map.insert("bucket".to_string(), name.to_string());
95        }
96
97        if let Some(root) = uri.root() {
98            map.insert("root".to_string(), root.to_string());
99        }
100
101        Self::from_iter(map)
102    }
103
104    #[allow(deprecated)]
105    fn into_builder(self) -> Self::Builder {
106        GcsBuilder {
107            config: self,
108            http_client: None,
109            customized_token_loader: None,
110        }
111    }
112}
113
114#[cfg(test)]
115mod tests {
116    use super::*;
117    use crate::Configurator;
118    use crate::types::OperatorUri;
119
120    #[test]
121    fn test_bucket_aliases() {
122        let config_json = r#"{"google_bucket": "test-bucket"}"#;
123        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
124        assert_eq!("test-bucket", config.bucket);
125
126        let config_json = r#"{"google_bucket_name": "test-bucket-name"}"#;
127        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
128        assert_eq!("test-bucket-name", config.bucket);
129
130        let config_json = r#"{"bucket_name": "test-bucket-alias"}"#;
131        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
132        assert_eq!("test-bucket-alias", config.bucket);
133    }
134
135    #[test]
136    fn test_service_account_aliases() {
137        let config_json = r#"{"google_service_account": "/path/to/sa.json"}"#;
138        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
139        assert_eq!(Some("/path/to/sa.json".to_string()), config.service_account);
140
141        let config_json = r#"{"google_service_account_path": "/path/to/sa2.json"}"#;
142        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
143        assert_eq!(
144            Some("/path/to/sa2.json".to_string()),
145            config.service_account
146        );
147
148        let config_json = r#"{"service_account_path": "/path/to/sa3.json"}"#;
149        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
150        assert_eq!(
151            Some("/path/to/sa3.json".to_string()),
152            config.service_account
153        );
154    }
155
156    #[test]
157    fn test_credential_aliases() {
158        let config_json = r#"{"google_service_account_key": "key-content"}"#;
159        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
160        assert_eq!(Some("key-content".to_string()), config.credential);
161
162        let config_json = r#"{"service_account_key": "key-content-2"}"#;
163        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
164        assert_eq!(Some("key-content-2".to_string()), config.credential);
165    }
166
167    #[test]
168    fn test_credential_path_aliases() {
169        let config_json = r#"{"google_application_credentials": "/path/to/app.json"}"#;
170        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
171        assert_eq!(
172            Some("/path/to/app.json".to_string()),
173            config.credential_path
174        );
175    }
176
177    #[test]
178    fn test_allow_anonymous_aliases() {
179        let config_json = r#"{"google_skip_signature": true}"#;
180        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
181        assert!(config.allow_anonymous);
182
183        let config_json = r#"{"skip_signature": true}"#;
184        let config: GcsConfig = serde_json::from_str(config_json).unwrap();
185        assert!(config.allow_anonymous);
186    }
187
188    #[test]
189    fn from_uri_extracts_bucket_and_root() {
190        let uri = OperatorUri::new(
191            "gcs://example-bucket/path/to/root",
192            Vec::<(String, String)>::new(),
193        )
194        .unwrap();
195        let cfg = GcsConfig::from_uri(&uri).unwrap();
196        assert_eq!(cfg.bucket, "example-bucket");
197        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
198    }
199}
```
