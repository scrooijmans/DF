# 

opendal/services/s3/

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
21use super::backend::S3Builder;
22use serde::Deserialize;
23use serde::Serialize;
24
25/// Config for Aws S3 and compatible services (including minio, digitalocean space, Tencent Cloud Object Storage(COS) and so on) support.
26#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
27#[serde(default)]
28#[non_exhaustive]
29pub struct S3Config {
30    /// root of this backend.
31    ///
32    /// All operations will happen under this root.
33    ///
34    /// default to `/` if not set.
35    pub root: Option<String>,
36    /// bucket name of this backend.
37    ///
38    /// required.
39    #[serde(alias = "aws_bucket", alias = "aws_bucket_name", alias = "bucket_name")]
40    pub bucket: String,
41    /// is bucket versioning enabled for this bucket
42    pub enable_versioning: bool,
43    /// endpoint of this backend.
44    ///
45    /// Endpoint must be full uri, e.g.
46    ///
47    /// - AWS S3: `https://s3.amazonaws.com` or `https://s3.{region}.amazonaws.com`
48    /// - Cloudflare R2: `https://<ACCOUNT_ID>.r2.cloudflarestorage.com`
49    /// - Aliyun OSS: `https://{region}.aliyuncs.com`
50    /// - Tencent COS: `https://cos.{region}.myqcloud.com`
51    /// - Minio: `http://127.0.0.1:9000`
52    ///
53    /// If user inputs endpoint without scheme like "s3.amazonaws.com", we
54    /// will prepend "https://" before it.
55    ///
56    /// - If endpoint is set, we will take user's input first.
57    /// - If not, we will try to load it from environment.
58    /// - If still not set, default to `https://s3.amazonaws.com`.
59    #[serde(
60        alias = "aws_endpoint",
61        alias = "aws_endpoint_url",
62        alias = "endpoint_url"
63    )]
64    pub endpoint: Option<String>,
65    /// Region represent the signing region of this endpoint. This is required
66    /// if you are using the default AWS S3 endpoint.
67    ///
68    /// If using a custom endpoint,
69    /// - If region is set, we will take user's input first.
70    /// - If not, we will try to load it from environment.
71    #[serde(alias = "aws_region")]
72    pub region: Option<String>,
73
74    /// access_key_id of this backend.
75    ///
76    /// - If access_key_id is set, we will take user's input first.
77    /// - If not, we will try to load it from environment.
78    #[serde(alias = "aws_access_key_id")]
79    pub access_key_id: Option<String>,
80    /// secret_access_key of this backend.
81    ///
82    /// - If secret_access_key is set, we will take user's input first.
83    /// - If not, we will try to load it from environment.
84    #[serde(alias = "aws_secret_access_key")]
85    pub secret_access_key: Option<String>,
86    /// session_token (aka, security token) of this backend.
87    ///
88    /// This token will expire after sometime, it's recommended to set session_token
89    /// by hand.
90    #[serde(alias = "aws_session_token", alias = "aws_token", alias = "token")]
91    pub session_token: Option<String>,
92    /// role_arn for this backend.
93    ///
94    /// If `role_arn` is set, we will use already known config as source
95    /// credential to assume role with `role_arn`.
96    pub role_arn: Option<String>,
97    /// external_id for this backend.
98    pub external_id: Option<String>,
99    /// role_session_name for this backend.
100    pub role_session_name: Option<String>,
101    /// Disable config load so that opendal will not load config from
102    /// environment.
103    ///
104    /// For examples:
105    ///
106    /// - envs like `AWS_ACCESS_KEY_ID`
107    /// - files like `~/.aws/config`
108    pub disable_config_load: bool,
109    /// Disable load credential from ec2 metadata.
110    ///
111    /// This option is used to disable the default behavior of opendal
112    /// to load credential from ec2 metadata, a.k.a, IMDSv2
113    pub disable_ec2_metadata: bool,
114    /// Allow anonymous will allow opendal to send request without signing
115    /// when credential is not loaded.
116    pub allow_anonymous: bool,
117    /// server_side_encryption for this backend.
118    ///
119    /// Available values: `AES256`, `aws:kms`.
120    #[serde(alias = "aws_server_side_encryption")]
121    pub server_side_encryption: Option<String>,
122    /// server_side_encryption_aws_kms_key_id for this backend
123    ///
124    /// - If `server_side_encryption` set to `aws:kms`, and `server_side_encryption_aws_kms_key_id`
125    ///   is not set, S3 will use aws managed kms key to encrypt data.
126    /// - If `server_side_encryption` set to `aws:kms`, and `server_side_encryption_aws_kms_key_id`
127    ///   is a valid kms key id, S3 will use the provided kms key to encrypt data.
128    /// - If the `server_side_encryption_aws_kms_key_id` is invalid or not found, an error will be
129    ///   returned.
130    /// - If `server_side_encryption` is not `aws:kms`, setting `server_side_encryption_aws_kms_key_id`
131    ///   is a noop.
132    #[serde(alias = "aws_sse_kms_key_id")]
133    pub server_side_encryption_aws_kms_key_id: Option<String>,
134    /// server_side_encryption_customer_algorithm for this backend.
135    ///
136    /// Available values: `AES256`.
137    pub server_side_encryption_customer_algorithm: Option<String>,
138    /// server_side_encryption_customer_key for this backend.
139    ///
140    /// Value: BASE64-encoded key that matches algorithm specified in
141    /// `server_side_encryption_customer_algorithm`.
142    #[serde(alias = "aws_sse_customer_key_base64")]
143    pub server_side_encryption_customer_key: Option<String>,
144    /// Set server_side_encryption_customer_key_md5 for this backend.
145    ///
146    /// Value: MD5 digest of key specified in `server_side_encryption_customer_key`.
147    pub server_side_encryption_customer_key_md5: Option<String>,
148    /// default storage_class for this backend.
149    ///
150    /// Available values:
151    /// - `DEEP_ARCHIVE`
152    /// - `GLACIER`
153    /// - `GLACIER_IR`
154    /// - `INTELLIGENT_TIERING`
155    /// - `ONEZONE_IA`
156    /// - `EXPRESS_ONEZONE`
157    /// - `OUTPOSTS`
158    /// - `REDUCED_REDUNDANCY`
159    /// - `STANDARD`
160    /// - `STANDARD_IA`
161    ///
162    /// S3 compatible services don't support all of them
163    pub default_storage_class: Option<String>,
164    /// Enable virtual host style so that opendal will send API requests
165    /// in virtual host style instead of path style.
166    ///
167    /// - By default, opendal will send API to `https://s3.us-east-1.amazonaws.com/bucket_name`
168    /// - Enabled, opendal will send API to `https://bucket_name.s3.us-east-1.amazonaws.com`
169    #[serde(
170        alias = "aws_virtual_hosted_style_request",
171        alias = "virtual_hosted_style_request"
172    )]
173    pub enable_virtual_host_style: bool,
174    /// Set maximum batch operations of this backend.
175    ///
176    /// Some compatible services have a limit on the number of operations in a batch request.
177    /// For example, R2 could return `Internal Error` while batch delete 1000 files.
178    ///
179    /// Please tune this value based on services' document.
180    #[deprecated(
181        since = "0.52.0",
182        note = "Please use `delete_max_size` instead of `batch_max_operations`"
183    )]
184    pub batch_max_operations: Option<usize>,
185    /// Set the maximum delete size of this backend.
186    ///
187    /// Some compatible services have a limit on the number of operations in a batch request.
188    /// For example, R2 could return `Internal Error` while batch delete 1000 files.
189    ///
190    /// Please tune this value based on services' document.
191    pub delete_max_size: Option<usize>,
192    /// Disable stat with override so that opendal will not send stat request with override queries.
193    ///
194    /// For example, R2 doesn't support stat with `response_content_type` query.
195    pub disable_stat_with_override: bool,
196    /// Checksum Algorithm to use when sending checksums in HTTP headers.
197    /// This is necessary when writing to AWS S3 Buckets with Object Lock enabled for example.
198    ///
199    /// Available options:
200    /// - "crc32c"
201    #[serde(alias = "aws_checksum_algorithm")]
202    pub checksum_algorithm: Option<String>,
203    /// Disable write with if match so that opendal will not send write request with if match headers.
204    ///
205    /// For example, Ceph RADOS S3 doesn't support write with if match.
206    pub disable_write_with_if_match: bool,
207
208    /// Enable write with append so that opendal will send write request with append headers.
209    pub enable_write_with_append: bool,
210
211    /// OpenDAL uses List Objects V2 by default to list objects.
212    /// However, some legacy services do not yet support V2.
213    /// This option allows users to switch back to the older List Objects V1.
214    pub disable_list_objects_v2: bool,
215
216    /// Indicates whether the client agrees to pay for the requests made to the S3 bucket.
217    #[serde(alias = "aws_request_payer", alias = "request_payer")]
218    pub enable_request_payer: bool,
219}
220
221impl Debug for S3Config {
222    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
223        let mut d = f.debug_struct("S3Config");
224
225        d.field("root", &self.root)
226            .field("bucket", &self.bucket)
227            .field("endpoint", &self.endpoint)
228            .field("region", &self.region);
229
230        d.finish_non_exhaustive()
231    }
232}
233
234impl crate::Configurator for S3Config {
235    type Builder = S3Builder;
236
237    fn from_uri(uri: &crate::types::OperatorUri) -> crate::Result<Self> {
238        let mut map = uri.options().clone();
239
240        if let Some(name) = uri.name() {
241            map.insert("bucket".to_string(), name.to_string());
242        }
243
244        if let Some(root) = uri.root() {
245            map.insert("root".to_string(), root.to_string());
246        }
247
248        Self::from_iter(map)
249    }
250
251    #[allow(deprecated)]
252    fn into_builder(self) -> Self::Builder {
253        S3Builder {
254            config: self,
255            customized_credential_load: None,
256
257            http_client: None,
258        }
259    }
260}
261
262#[cfg(test)]
263mod tests {
264    use super::*;
265    use crate::Configurator;
266    use crate::types::OperatorUri;
267
268    #[test]
269    fn test_s3_config_original_field_names() {
270        let json = r#"{
271            "bucket": "test-bucket",
272            "access_key_id": "test-key",
273            "secret_access_key": "test-secret",
274            "region": "us-west-2",
275            "endpoint": "https://s3.amazonaws.com",
276            "session_token": "test-token"
277        }"#;
278
279        let config: S3Config = serde_json::from_str(json).unwrap();
280        assert_eq!(config.bucket, "test-bucket");
281        assert_eq!(config.access_key_id, Some("test-key".to_string()));
282        assert_eq!(config.secret_access_key, Some("test-secret".to_string()));
283        assert_eq!(config.region, Some("us-west-2".to_string()));
284        assert_eq!(
285            config.endpoint,
286            Some("https://s3.amazonaws.com".to_string())
287        );
288        assert_eq!(config.session_token, Some("test-token".to_string()));
289    }
290
291    #[test]
292    fn test_s3_config_aws_prefixed_aliases() {
293        let json = r#"{
294            "aws_bucket": "test-bucket",
295            "aws_access_key_id": "test-key",
296            "aws_secret_access_key": "test-secret",
297            "aws_region": "us-west-2",
298            "aws_endpoint": "https://s3.amazonaws.com",
299            "aws_session_token": "test-token"
300        }"#;
301
302        let config: S3Config = serde_json::from_str(json).unwrap();
303        assert_eq!(config.bucket, "test-bucket");
304        assert_eq!(config.access_key_id, Some("test-key".to_string()));
305        assert_eq!(config.secret_access_key, Some("test-secret".to_string()));
306        assert_eq!(config.region, Some("us-west-2".to_string()));
307        assert_eq!(
308            config.endpoint,
309            Some("https://s3.amazonaws.com".to_string())
310        );
311        assert_eq!(config.session_token, Some("test-token".to_string()));
312    }
313
314    #[test]
315    fn test_s3_config_additional_aliases() {
316        let json = r#"{
317            "bucket_name": "test-bucket",
318            "token": "test-token",
319            "endpoint_url": "https://s3.amazonaws.com",
320            "virtual_hosted_style_request": true,
321            "aws_checksum_algorithm": "crc32c",
322            "request_payer": true
323        }"#;
324
325        let config: S3Config = serde_json::from_str(json).unwrap();
326        assert_eq!(config.bucket, "test-bucket");
327        assert_eq!(config.session_token, Some("test-token".to_string()));
328        assert_eq!(
329            config.endpoint,
330            Some("https://s3.amazonaws.com".to_string())
331        );
332        assert!(config.enable_virtual_host_style);
333        assert_eq!(config.checksum_algorithm, Some("crc32c".to_string()));
334        assert!(config.enable_request_payer);
335    }
336
337    #[test]
338    fn test_s3_config_encryption_aliases() {
339        let json = r#"{
340            "bucket": "test-bucket",
341            "aws_server_side_encryption": "aws:kms",
342            "aws_sse_kms_key_id": "test-kms-key",
343            "aws_sse_customer_key_base64": "dGVzdC1jdXN0b21lci1rZXk="
344        }"#;
345
346        let config: S3Config = serde_json::from_str(json).unwrap();
347        assert_eq!(config.bucket, "test-bucket");
348        assert_eq!(config.server_side_encryption, Some("aws:kms".to_string()));
349        assert_eq!(
350            config.server_side_encryption_aws_kms_key_id,
351            Some("test-kms-key".to_string())
352        );
353        assert_eq!(
354            config.server_side_encryption_customer_key,
355            Some("dGVzdC1jdXN0b21lci1rZXk=".to_string())
356        );
357    }
358
359    #[test]
360    fn from_uri_extracts_bucket_and_root() {
361        let uri = OperatorUri::new(
362            "s3://example-bucket/path/to/root",
363            Vec::<(String, String)>::new(),
364        )
365        .unwrap();
366        let cfg = S3Config::from_uri(&uri).unwrap();
367        assert_eq!(cfg.bucket, "example-bucket");
368        assert_eq!(cfg.root.as_deref(), Some("path/to/root"));
369    }
370}
```
