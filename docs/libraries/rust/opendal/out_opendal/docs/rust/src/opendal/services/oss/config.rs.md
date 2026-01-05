# 

opendal/services/oss/

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
21use super::backend::OssBuilder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Aliyun Object Storage Service (OSS) support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct OssConfig {
30    /// Root for oss.
31    pub root: Option<String>,
32
33    /// Endpoint for oss.
34    pub endpoint: Option<String>,
35    /// Presign endpoint for oss.
36    pub presign_endpoint: Option<String>,
37    /// Bucket for oss.
38    pub bucket: String,
39    /// Addressing style for oss.
40    pub addressing_style: Option<String>,
41    /// Pre sign addressing style for oss.
42    pub presign_addressing_style: Option<String>,
43
44    /// is bucket versioning enabled for this bucket
45    pub enable_versioning: bool,
46
47    // OSS features
48    /// Server side encryption for oss.
49    pub server_side_encryption: Option<String>,
50    /// Server side encryption key id for oss.
51    pub server_side_encryption_key_id: Option<String>,
52    /// Allow anonymous for oss.
53    pub allow_anonymous: bool,
54
55    // authenticate options
56    /// Access key id for oss.
57    ///
58    /// - this field if it's `is_some`
59    /// - env value: [`ALIBABA_CLOUD_ACCESS_KEY_ID`]
60    pub access_key_id: Option<String>,
61    /// Access key secret for oss.
62    ///
63    /// - this field if it's `is_some`
64    /// - env value: [`ALIBABA_CLOUD_ACCESS_KEY_SECRET`]
65    pub access_key_secret: Option<String>,
66    /// `security_token` will be loaded from
67    ///
68    /// - this field if it's `is_some`
69    /// - env value: [`ALIBABA_CLOUD_SECURITY_TOKEN`]
70    pub security_token: Option<String>,
71    /// The size of max batch operations.
72    #[deprecated(
73        since = "0.52.0",
74        note = "Please use `delete_max_size` instead of `batch_max_operations`"
75    )]
76    pub batch_max_operations: Option<usize>,
77    /// The size of max delete operations.
78    pub delete_max_size: Option<usize>,
79    /// If `role_arn` is set, we will use already known config as source
80    /// credential to assume role with `role_arn`.
81    ///
82    /// - this field if it's `is_some`
83    /// - env value: [`ALIBABA_CLOUD_ROLE_ARN`]
84    pub role_arn: Option<String>,
85    /// role_session_name for this backend.
86    pub role_session_name: Option<String>,
87    /// `oidc_provider_arn` will be loaded from
88    ///
89    /// - this field if it's `is_some`
90    /// - env value: [`ALIBABA_CLOUD_OIDC_PROVIDER_ARN`]
91    pub oidc_provider_arn: Option<String>,
92    /// `oidc_token_file` will be loaded from
93    ///
94    /// - this field if it's `is_some`
95    /// - env value: [`ALIBABA_CLOUD_OIDC_TOKEN_FILE`]
96    pub oidc_token_file: Option<String>,
97    /// `sts_endpoint` will be loaded from
98    ///
99    /// - this field if it's `is_some`
100    /// - env value: [`ALIBABA_CLOUD_STS_ENDPOINT`]
101    pub sts_endpoint: Option<String>,
102}
103
104impl Debug for OssConfig {
105    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
106        let mut d = f.debug_struct("Builder");
107        d.field("root", &self.root)
108            .field("bucket", &self.bucket)
109            .field("endpoint", &self.endpoint)
110            .field("allow_anonymous", &self.allow_anonymous);
111
112        d.finish_non_exhaustive()
113    }
114}
115
116impl crate::Configurator for OssConfig {
117    type Builder = OssBuilder;
118
119    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
120        let mut map = uri.options().clone();
121
122        if let Some(name) = uri.name() {
123            map.insert("bucket".to_string(), name.to_string());
124        }
125
126        if let Some(root) = uri.root() {
127            map.insert("root".to_string(), root.to_string());
128        }
129
130        Self::from_iter(map)
131    }
132
133    #[allow(deprecated)]
134    fn into_builder(self) -> Self::Builder {
135        OssBuilder {
136            config: self,
137
138            http_client: None,
139        }
140    }
141}
142
143#[cfg(test)]
144mod tests {
145    use super::*;
146    use crate::Configurator;
147    use crate::types::OperatorUri;
148
149    #[test]
150    fn from_uri_extracts_bucket_and_root() {
151        let uri = OperatorUri::new(
152            "oss://example-bucket/path/to/root",
153            Vec::<(String, String)>::new(),
154        )
155        .unwrap();
156        let cfg = OssConfig::from_uri(&uri).unwrap();
157        assert_eq!(cfg.bucket, "example-bucket");
158        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
159    }
160}
```
