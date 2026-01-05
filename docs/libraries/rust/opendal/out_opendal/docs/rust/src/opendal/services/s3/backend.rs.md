# 

opendal/services/s3/

backend.rs

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
18use std::collections::HashMap;
19use std::fmt::Debug;
20use std::fmt::Formatter;
21use std::fmt::Write;
22use std::str::FromStr;
23use std::sync::Arc;
24use std::sync::LazyLock;
25use std::sync::atomic::AtomicBool;
26
27use base64::Engine;
28use base64::prelude::BASE64_STANDARD;
29use constants::X_AMZ_META_PREFIX;
30use constants::X_AMZ_VERSION_ID;
31use http::Response;
32use http::StatusCode;
33use log::debug;
34use log::warn;
35use md5::Digest;
36use md5::Md5;
37use reqsign::AwsAssumeRoleLoader;
38use reqsign::AwsConfig;
39use reqsign::AwsCredentialLoad;
40use reqsign::AwsDefaultLoader;
41use reqsign::AwsV4Signer;
42use reqwest::Url;
43
44use super::S3_SCHEME;
45use super::core::*;
46use super::delete::S3Deleter;
47use super::error::parse_error;
48use super::lister::S3ListerV1;
49use super::lister::S3ListerV2;
50use super::lister::S3Listers;
51use super::lister::S3ObjectVersionsLister;
52use super::writer::S3Writer;
53use super::writer::S3Writers;
54use crate::raw::oio::PageLister;
55use crate::raw::*;
56use crate::services::S3Config;
57use crate::*;
58
59/// Allow constructing correct region endpoint if user gives a global endpoint.
60static ENDPOINT_TEMPLATES: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
61    let mut m = HashMap::new();
62    // AWS S3 Service.
63    m.insert(
64        "https://s3.amazonaws.com",
65        "https://s3.{region}.amazonaws.com",
66    );
67    m
68});
69
70const DEFAULT_BATCH_MAX_OPERATIONS: usize = 1000;
71
72/// Aws S3 and compatible services (including minio, digitalocean space, Tencent Cloud Object Storage(COS) and so on) support.
73/// For more information about s3-compatible services, refer to [Compatible Services](#compatible-services).
74#[doc = include_str!("docs.md")]
75#[doc = include_str!("compatible_services.md")]
76#[derive(Default)]
77pub struct S3Builder {
78    pub(super) config: S3Config,
79
80    pub(super) customized_credential_load: Option<Box<dyn AwsCredentialLoad>>,
81
82    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
83    pub(super) http_client: Option<HttpClient>,
84}
85
86impl Debug for S3Builder {
87    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
88        let mut d = f.debug_struct("S3Builder");
89
90        d.field("config", &self.config);
91        d.finish_non_exhaustive()
92    }
93}
94
95impl S3Builder {
96    /// Set root of this backend.
97    ///
98    /// All operations will happen under this root.
99    pub fn root(mut self, root: &str) -> Self {
100        self.config.root = if root.is_empty() {
101            None
102        } else {
103            Some(root.to_string())
104        };
105
106        self
107    }
108
109    /// Set bucket name of this backend.
110    pub fn bucket(mut self, bucket: &str) -> Self {
111        self.config.bucket = bucket.to_string();
112
113        self
114    }
115
116    /// Set endpoint of this backend.
117    ///
118    /// Endpoint must be full uri, e.g.
119    ///
120    /// - AWS S3: `https://s3.amazonaws.com` or `https://s3.{region}.amazonaws.com`
121    /// - Cloudflare R2: `https://<ACCOUNT_ID>.r2.cloudflarestorage.com`
122    /// - Aliyun OSS: `https://{region}.aliyuncs.com`
123    /// - Tencent COS: `https://cos.{region}.myqcloud.com`
124    /// - Minio: `http://127.0.0.1:9000`
125    ///
126    /// If user inputs endpoint without scheme like "s3.amazonaws.com", we
127    /// will prepend "https://" before it.
128    pub fn endpoint(mut self, endpoint: &str) -> Self {
129        if !endpoint.is_empty() {
130            // Trim trailing `/` so that we can accept `http://127.0.0.1:9000/`
131            self.config.endpoint = Some(endpoint.trim_end_matches('/').to_string())
132        }
133
134        self
135    }
136
137    /// Region represent the signing region of this endpoint. This is required
138    /// if you are using the default AWS S3 endpoint.
139    ///
140    /// If using a custom endpoint,
141    /// - If region is set, we will take user's input first.
142    /// - If not, we will try to load it from environment.
143    pub fn region(mut self, region: &str) -> Self {
144        if !region.is_empty() {
145            self.config.region = Some(region.to_string())
146        }
147
148        self
149    }
150
151    /// Set access_key_id of this backend.
152    ///
153    /// - If access_key_id is set, we will take user's input first.
154    /// - If not, we will try to load it from environment.
155    pub fn access_key_id(mut self, v: &str) -> Self {
156        if !v.is_empty() {
157            self.config.access_key_id = Some(v.to_string())
158        }
159
160        self
161    }
162
163    /// Set secret_access_key of this backend.
164    ///
165    /// - If secret_access_key is set, we will take user's input first.
166    /// - If not, we will try to load it from environment.
167    pub fn secret_access_key(mut self, v: &str) -> Self {
168        if !v.is_empty() {
169            self.config.secret_access_key = Some(v.to_string())
170        }
171
172        self
173    }
174
175    /// Set role_arn for this backend.
176    ///
177    /// If `role_arn` is set, we will use already known config as source
178    /// credential to assume role with `role_arn`.
179    pub fn role_arn(mut self, v: &str) -> Self {
180        if !v.is_empty() {
181            self.config.role_arn = Some(v.to_string())
182        }
183
184        self
185    }
186
187    /// Set external_id for this backend.
188    pub fn external_id(mut self, v: &str) -> Self {
189        if !v.is_empty() {
190            self.config.external_id = Some(v.to_string())
191        }
192
193        self
194    }
195
196    /// Set role_session_name for this backend.
197    pub fn role_session_name(mut self, v: &str) -> Self {
198        if !v.is_empty() {
199            self.config.role_session_name = Some(v.to_string())
200        }
201
202        self
203    }
204
205    /// Set default storage_class for this backend.
206    ///
207    /// Available values:
208    /// - `DEEP_ARCHIVE`
209    /// - `GLACIER`
210    /// - `GLACIER_IR`
211    /// - `INTELLIGENT_TIERING`
212    /// - `ONEZONE_IA`
213    /// - `OUTPOSTS`
214    /// - `REDUCED_REDUNDANCY`
215    /// - `STANDARD`
216    /// - `STANDARD_IA`
217    pub fn default_storage_class(mut self, v: &str) -> Self {
218        if !v.is_empty() {
219            self.config.default_storage_class = Some(v.to_string())
220        }
221
222        self
223    }
224
225    /// Set server_side_encryption for this backend.
226    ///
227    /// Available values: `AES256`, `aws:kms`.
228    ///
229    /// # Note
230    ///
231    /// This function is the low-level setting for SSE related features.
232    ///
233    /// SSE related options should be set carefully to make them works.
234    /// Please use `server_side_encryption_with_*` helpers if even possible.
235    pub fn server_side_encryption(mut self, v: &str) -> Self {
236        if !v.is_empty() {
237            self.config.server_side_encryption = Some(v.to_string())
238        }
239
240        self
241    }
242
243    /// Set server_side_encryption_aws_kms_key_id for this backend
244    ///
245    /// - If `server_side_encryption` set to `aws:kms`, and `server_side_encryption_aws_kms_key_id`
246    ///   is not set, S3 will use aws managed kms key to encrypt data.
247    /// - If `server_side_encryption` set to `aws:kms`, and `server_side_encryption_aws_kms_key_id`
248    ///   is a valid kms key id, S3 will use the provided kms key to encrypt data.
249    /// - If the `server_side_encryption_aws_kms_key_id` is invalid or not found, an error will be
250    ///   returned.
251    /// - If `server_side_encryption` is not `aws:kms`, setting `server_side_encryption_aws_kms_key_id` is a noop.
252    ///
253    /// # Note
254    ///
255    /// This function is the low-level setting for SSE related features.
256    ///
257    /// SSE related options should be set carefully to make them works.
258    /// Please use `server_side_encryption_with_*` helpers if even possible.
259    pub fn server_side_encryption_aws_kms_key_id(mut self, v: &str) -> Self {
260        if !v.is_empty() {
261            self.config.server_side_encryption_aws_kms_key_id = Some(v.to_string())
262        }
263
264        self
265    }
266
267    /// Set server_side_encryption_customer_algorithm for this backend.
268    ///
269    /// Available values: `AES256`.
270    ///
271    /// # Note
272    ///
273    /// This function is the low-level setting for SSE related features.
274    ///
275    /// SSE related options should be set carefully to make them works.
276    /// Please use `server_side_encryption_with_*` helpers if even possible.
277    pub fn server_side_encryption_customer_algorithm(mut self, v: &str) -> Self {
278        if !v.is_empty() {
279            self.config.server_side_encryption_customer_algorithm = Some(v.to_string())
280        }
281
282        self
283    }
284
285    /// Set server_side_encryption_customer_key for this backend.
286    ///
287    /// # Args
288    ///
289    /// `v`: base64 encoded key that matches algorithm specified in
290    /// `server_side_encryption_customer_algorithm`.
291    ///
292    /// # Note
293    ///
294    /// This function is the low-level setting for SSE related features.
295    ///
296    /// SSE related options should be set carefully to make them works.
297    /// Please use `server_side_encryption_with_*` helpers if even possible.
298    pub fn server_side_encryption_customer_key(mut self, v: &str) -> Self {
299        if !v.is_empty() {
300            self.config.server_side_encryption_customer_key = Some(v.to_string())
301        }
302
303        self
304    }
305
306    /// Set server_side_encryption_customer_key_md5 for this backend.
307    ///
308    /// # Args
309    ///
310    /// `v`: MD5 digest of key specified in `server_side_encryption_customer_key`.
311    ///
312    /// # Note
313    ///
314    /// This function is the low-level setting for SSE related features.
315    ///
316    /// SSE related options should be set carefully to make them works.
317    /// Please use `server_side_encryption_with_*` helpers if even possible.
318    pub fn server_side_encryption_customer_key_md5(mut self, v: &str) -> Self {
319        if !v.is_empty() {
320            self.config.server_side_encryption_customer_key_md5 = Some(v.to_string())
321        }
322
323        self
324    }
325
326    /// Enable server side encryption with aws managed kms key
327    ///
328    /// As known as: SSE-KMS
329    ///
330    /// NOTE: This function should not be used along with other `server_side_encryption_with_` functions.
331    pub fn server_side_encryption_with_aws_managed_kms_key(mut self) -> Self {
332        self.config.server_side_encryption = Some("aws:kms".to_string());
333        self
334    }
335
336    /// Enable server side encryption with customer managed kms key
337    ///
338    /// As known as: SSE-KMS
339    ///
340    /// NOTE: This function should not be used along with other `server_side_encryption_with_` functions.
341    pub fn server_side_encryption_with_customer_managed_kms_key(
342        mut self,
343        aws_kms_key_id: &str,
344    ) -> Self {
345        self.config.server_side_encryption = Some("aws:kms".to_string());
346        self.config.server_side_encryption_aws_kms_key_id = Some(aws_kms_key_id.to_string());
347        self
348    }
349
350    /// Enable server side encryption with s3 managed key
351    ///
352    /// As known as: SSE-S3
353    ///
354    /// NOTE: This function should not be used along with other `server_side_encryption_with_` functions.
355    pub fn server_side_encryption_with_s3_key(mut self) -> Self {
356        self.config.server_side_encryption = Some("AES256".to_string());
357        self
358    }
359
360    /// Enable server side encryption with customer key.
361    ///
362    /// As known as: SSE-C
363    ///
364    /// NOTE: This function should not be used along with other `server_side_encryption_with_` functions.
365    pub fn server_side_encryption_with_customer_key(mut self, algorithm: &str, key: &[u8]) -> Self {
366        self.config.server_side_encryption_customer_algorithm = Some(algorithm.to_string());
367        self.config.server_side_encryption_customer_key = Some(BASE64_STANDARD.encode(key));
368        self.config.server_side_encryption_customer_key_md5 =
369            Some(BASE64_STANDARD.encode(Md5::digest(key).as_slice()));
370        self
371    }
372
373    /// Set temporary credential used in AWS S3 connections
374    ///
375    /// # Warning
376    ///
377    /// session token's lifetime is short and requires users to refresh in time.
378    pub fn session_token(mut self, token: &str) -> Self {
379        if !token.is_empty() {
380            self.config.session_token = Some(token.to_string());
381        }
382        self
383    }
384
385    /// Set temporary credential used in AWS S3 connections
386    #[deprecated(note = "Please use `session_token` instead")]
387    pub fn security_token(self, token: &str) -> Self {
388        self.session_token(token)
389    }
390
391    /// Disable config load so that opendal will not load config from
392    /// environment.
393    ///
394    /// For examples:
395    ///
396    /// - envs like `AWS_ACCESS_KEY_ID`
397    /// - files like `~/.aws/config`
398    pub fn disable_config_load(mut self) -> Self {
399        self.config.disable_config_load = true;
400        self
401    }
402
403    /// Disable list objects v2 so that opendal will not use the older
404    /// List Objects V1 to list objects.
405    ///
406    /// By default, OpenDAL uses List Objects V2 to list objects. However,
407    /// some legacy services do not yet support V2.
408    pub fn disable_list_objects_v2(mut self) -> Self {
409        self.config.disable_list_objects_v2 = true;
410        self
411    }
412
413    /// Enable request payer so that OpenDAL will send requests with `x-amz-request-payer` header.
414    ///
415    /// With this option the client accepts to pay for the request and data transfer costs.
416    pub fn enable_request_payer(mut self) -> Self {
417        self.config.enable_request_payer = true;
418        self
419    }
420
421    /// Disable load credential from ec2 metadata.
422    ///
423    /// This option is used to disable the default behavior of opendal
424    /// to load credential from ec2 metadata, a.k.a, IMDSv2
425    pub fn disable_ec2_metadata(mut self) -> Self {
426        self.config.disable_ec2_metadata = true;
427        self
428    }
429
430    /// Allow anonymous will allow opendal to send request without signing
431    /// when credential is not loaded.
432    pub fn allow_anonymous(mut self) -> Self {
433        self.config.allow_anonymous = true;
434        self
435    }
436
437    /// Enable virtual host style so that opendal will send API requests
438    /// in virtual host style instead of path style.
439    ///
440    /// - By default, opendal will send API to `https://s3.us-east-1.amazonaws.com/bucket_name`
441    /// - Enabled, opendal will send API to `https://bucket_name.s3.us-east-1.amazonaws.com`
442    pub fn enable_virtual_host_style(mut self) -> Self {
443        self.config.enable_virtual_host_style = true;
444        self
445    }
446
447    /// Disable stat with override so that opendal will not send stat request with override queries.
448    ///
449    /// For example, R2 doesn't support stat with `response_content_type` query.
450    pub fn disable_stat_with_override(mut self) -> Self {
451        self.config.disable_stat_with_override = true;
452        self
453    }
454
455    /// Adding a customized credential load for service.
456    ///
457    /// If customized_credential_load has been set, we will ignore all other
458    /// credential load methods.
459    pub fn customized_credential_load(mut self, cred: Box<dyn AwsCredentialLoad>) -> Self {
460        self.customized_credential_load = Some(cred);
461        self
462    }
463
464    /// Specify the http client that used by this service.
465    ///
466    /// # Notes
467    ///
468    /// This API is part of OpenDAL's Raw API. `HttpClient` could be changed
469    /// during minor updates.
470    #[deprecated(since = "0.53.0", note = "Use `Operator::update_http_client` instead")]
471    #[allow(deprecated)]
472    pub fn http_client(mut self, client: HttpClient) -> Self {
473        self.http_client = Some(client);
474        self
475    }
476
477    /// Set bucket versioning status for this backend
478    pub fn enable_versioning(mut self, enabled: bool) -> Self {
479        self.config.enable_versioning = enabled;
480
481        self
482    }
483
484    /// Check if `bucket` is valid
485    /// `bucket` must be not empty and if `enable_virtual_host_style` is true
486    /// it couldn't contain dot(.) character
487    fn is_bucket_valid(&self) -> bool {
488        if self.config.bucket.is_empty() {
489            return false;
490        }
491        // If enable virtual host style, `bucket` will reside in domain part,
492        // for example `https://bucket_name.s3.us-east-1.amazonaws.com`,
493        // so `bucket` with dot can't be recognized correctly for this format.
494        if self.config.enable_virtual_host_style && self.config.bucket.contains('.') {
495            return false;
496        }
497        true
498    }
499
500    /// Build endpoint with given region.
501    fn build_endpoint(&self, region: &str) -> String {
502        let bucket = {
503            debug_assert!(self.is_bucket_valid(), "bucket must be valid");
504
505            self.config.bucket.as_str()
506        };
507
508        let mut endpoint = match &self.config.endpoint {
509            Some(endpoint) => {
510                if endpoint.starts_with("http") {
511                    endpoint.to_string()
512                } else {
513                    // Prefix https if endpoint doesn't start with scheme.
514                    format!("https://{endpoint}")
515                }
516            }
517            None => "https://s3.amazonaws.com".to_string(),
518        };
519
520        // If endpoint contains bucket name, we should trim them.
521        endpoint = endpoint.replace(&format!("//{bucket}."), "//");
522
523        // Omit default ports if specified.
524        if let Ok(url) = Url::from_str(&endpoint) {
525            // Remove the trailing `/` of root path.
526            endpoint = url.to_string().trim_end_matches('/').to_string();
527        }
528
529        // Update with endpoint templates.
530        endpoint = if let Some(template) = ENDPOINT_TEMPLATES.get(endpoint.as_str()) {
531            template.replace("{region}", region)
532        } else {
533            // If we don't know where about this endpoint, just leave
534            // them as it.
535            endpoint.to_string()
536        };
537
538        // Apply virtual host style.
539        if self.config.enable_virtual_host_style {
540            endpoint = endpoint.replace("//", &format!("//{bucket}."))
541        } else {
542            write!(endpoint, "/{bucket}").expect("write into string must succeed");
543        };
544
545        endpoint
546    }
547
548    /// Set maximum batch operations of this backend.
549    #[deprecated(
550        since = "0.52.0",
551        note = "Please use `delete_max_size` instead of `batch_max_operations`"
552    )]
553    pub fn batch_max_operations(mut self, batch_max_operations: usize) -> Self {
554        self.config.delete_max_size = Some(batch_max_operations);
555
556        self
557    }
558
559    /// Set maximum delete operations of this backend.
560    pub fn delete_max_size(mut self, delete_max_size: usize) -> Self {
561        self.config.delete_max_size = Some(delete_max_size);
562
563        self
564    }
565
566    /// Set checksum algorithm of this backend.
567    /// This is necessary when writing to AWS S3 Buckets with Object Lock enabled for example.
568    ///
569    /// Available options:
570    /// - "crc32c"
571    pub fn checksum_algorithm(mut self, checksum_algorithm: &str) -> Self {
572        self.config.checksum_algorithm = Some(checksum_algorithm.to_string());
573
574        self
575    }
576
577    /// Disable write with if match so that opendal will not send write request with if match headers.
578    pub fn disable_write_with_if_match(mut self) -> Self {
579        self.config.disable_write_with_if_match = true;
580        self
581    }
582
583    /// Enable write with append so that opendal will send write request with append headers.
584    pub fn enable_write_with_append(mut self) -> Self {
585        self.config.enable_write_with_append = true;
586        self
587    }
588
589    /// Detect region of S3 bucket.
590    ///
591    /// # Args
592    ///
593    /// - endpoint: the endpoint of S3 service
594    /// - bucket: the bucket of S3 service
595    ///
596    /// # Return
597    ///
598    /// - `Some(region)` means we detect the region successfully
599    /// - `None` means we can't detect the region or meeting errors.
600    ///
601    /// # Notes
602    ///
603    /// We will try to detect region by the following methods.
604    ///
605    /// - Match endpoint with given rules to get region
606    ///   - Cloudflare R2
607    ///   - AWS S3
608    ///   - Aliyun OSS
609    /// - Send a `HEAD` request to endpoint with bucket name to get `x-amz-bucket-region`.
610    ///
611    /// # Examples
612    ///
613    /// ```no_run
614    /// use opendal::services::S3;
615    ///
616    /// # async fn example() {
617    /// let region: Option<String> = S3::detect_region("https://s3.amazonaws.com", "example").await;
618    /// # }
619    /// ```
620    ///
621    /// # Reference
622    ///
623    /// - [Amazon S3 HeadBucket API](https://docs.aws.amazon.com/zh_cn/AmazonS3/latest/API/API_HeadBucket.html)
624    pub async fn detect_region(endpoint: &str, bucket: &str) -> Option<String> {
625        // Remove the possible trailing `/` in endpoint.
626        let endpoint = endpoint.trim_end_matches('/');
627
628        // Make sure the endpoint contains the scheme.
629        let mut endpoint = if endpoint.starts_with("http") {
630            endpoint.to_string()
631        } else {
632            // Prefix https if endpoint doesn't start with scheme.
633            format!("https://{endpoint}")
634        };
635
636        // Remove bucket name from endpoint.
637        endpoint = endpoint.replace(&format!("//{bucket}."), "//");
638        let url = format!("{endpoint}/{bucket}");
639
640        debug!("detect region with url: {url}");
641
642        // Try to detect region by endpoint.
643
644        // If this bucket is R2, we can return auto directly.
645        //
646        // Reference: <https://developers.cloudflare.com/r2/api/s3/api/>
647        if endpoint.ends_with("r2.cloudflarestorage.com") {
648            return Some("auto".to_string());
649        }
650
651        // If this bucket is AWS, we can try to match the endpoint.
652        if let Some(v) = endpoint.strip_prefix("https://s3.") {
653            if let Some(region) = v.strip_suffix(".amazonaws.com") {
654                return Some(region.to_string());
655            }
656        }
657
658        // If this bucket is OSS, we can try to match the endpoint.
659        //
660        // - `oss-ap-southeast-1.aliyuncs.com` => `oss-ap-southeast-1`
661        // - `oss-cn-hangzhou-internal.aliyuncs.com` => `oss-cn-hangzhou`
662        if let Some(v) = endpoint.strip_prefix("https://") {
663            if let Some(region) = v.strip_suffix(".aliyuncs.com") {
664                return Some(region.to_string());
665            }
666
667            if let Some(region) = v.strip_suffix("-internal.aliyuncs.com") {
668                return Some(region.to_string());
669            }
670        }
671
672        // Try to detect region by HeadBucket.
673        let req = http::Request::head(&url).body(Buffer::new()).ok()?;
674
675        let client = HttpClient::new().ok()?;
676        let res = client
677            .send(req)
678            .await
679            .map_err(|err| warn!("detect region failed for: {err:?}"))
680            .ok()?;
681
682        debug!(
683            "auto detect region got response: status {:?}, header: {:?}",
684            res.status(),
685            res.headers()
686        );
687
688        // Get region from response header no matter status code.
689        if let Some(header) = res.headers().get("x-amz-bucket-region") {
690            if let Ok(regin) = header.to_str() {
691                return Some(regin.to_string());
692            }
693        }
694
695        // Status code is 403 or 200 means we already visit the correct
696        // region, we can use the default region directly.
697        if res.status() == StatusCode::FORBIDDEN || res.status() == StatusCode::OK {
698            return Some("us-east-1".to_string());
699        }
700
701        None
702    }
703}
704
705impl Builder for S3Builder {
706    type Config = S3Config;
707
708    fn build(mut self) -> Result<impl Access> {
709        debug!("backend build started: {:?}", &self);
710
711        let root = normalize_root(&self.config.root.clone().unwrap_or_default());
712        debug!("backend use root {}", &root);
713
714        // Handle bucket name.
715        let bucket = if self.is_bucket_valid() {
716            Ok(&self.config.bucket)
717        } else {
718            Err(
719                Error::new(ErrorKind::ConfigInvalid, "The bucket is misconfigured")
720                    .with_context("service", Scheme::S3),
721            )
722        }?;
723        debug!("backend use bucket {}", &bucket);
724
725        let default_storage_class = match &self.config.default_storage_class {
726            None => None,
727            Some(v) => Some(
728                build_header_value(v).map_err(|err| err.with_context("key", "storage_class"))?,
729            ),
730        };
731
732        let server_side_encryption = match &self.config.server_side_encryption {
733            None => None,
734            Some(v) => Some(
735                build_header_value(v)
736                    .map_err(|err| err.with_context("key", "server_side_encryption"))?,
737            ),
738        };
739
740        let server_side_encryption_aws_kms_key_id =
741            match &self.config.server_side_encryption_aws_kms_key_id {
742                None => None,
743                Some(v) => Some(build_header_value(v).map_err(|err| {
744                    err.with_context("key", "server_side_encryption_aws_kms_key_id")
745                })?),
746            };
747
748        let server_side_encryption_customer_algorithm =
749            match &self.config.server_side_encryption_customer_algorithm {
750                None => None,
751                Some(v) => Some(build_header_value(v).map_err(|err| {
752                    err.with_context("key", "server_side_encryption_customer_algorithm")
753                })?),
754            };
755
756        let server_side_encryption_customer_key =
757            match &self.config.server_side_encryption_customer_key {
758                None => None,
759                Some(v) => Some(build_header_value(v).map_err(|err| {
760                    err.with_context("key", "server_side_encryption_customer_key")
761                })?),
762            };
763
764        let server_side_encryption_customer_key_md5 =
765            match &self.config.server_side_encryption_customer_key_md5 {
766                None => None,
767                Some(v) => Some(build_header_value(v).map_err(|err| {
768                    err.with_context("key", "server_side_encryption_customer_key_md5")
769                })?),
770            };
771
772        let checksum_algorithm = match self.config.checksum_algorithm.as_deref() {
773            Some("crc32c") => Some(ChecksumAlgorithm::Crc32c),
774            None => None,
775            v => {
776                return Err(Error::new(
777                    ErrorKind::ConfigInvalid,
778                    format!("{v:?} is not a supported checksum_algorithm."),
779                ));
780            }
781        };
782
783        // This is our current config.
784        let mut cfg = AwsConfig::default();
785        if !self.config.disable_config_load {
786            #[cfg(not(target_arch = "wasm32"))]
787            {
788                cfg = cfg.from_profile();
789                cfg = cfg.from_env();
790            }
791        }
792
793        if let Some(ref v) = self.config.region {
794            cfg.region = Some(v.to_string());
795        }
796
797        if cfg.region.is_none() {
798            return Err(Error::new(
799                ErrorKind::ConfigInvalid,
800                "region is missing. Please find it by S3::detect_region() or set them in env.",
801            )
802            .with_operation("Builder::build")
803            .with_context("service", Scheme::S3));
804        }
805
806        let region = cfg.region.to_owned().unwrap();
807        debug!("backend use region: {region}");
808
809        // Retain the user's endpoint if it exists; otherwise, try loading it from the environment.
810        self.config.endpoint = self.config.endpoint.or_else(|| cfg.endpoint_url.clone());
811
812        // Building endpoint.
813        let endpoint = self.build_endpoint(&region);
814        debug!("backend use endpoint: {endpoint}");
815
816        // Setting all value from user input if available.
817        if let Some(v) = self.config.access_key_id {
818            cfg.access_key_id = Some(v)
819        }
820        if let Some(v) = self.config.secret_access_key {
821            cfg.secret_access_key = Some(v)
822        }
823        if let Some(v) = self.config.session_token {
824            cfg.session_token = Some(v)
825        }
826
827        let mut loader: Option<Box<dyn AwsCredentialLoad>> = None;
828        // If customized_credential_load is set, we will use it.
829        if let Some(v) = self.customized_credential_load {
830            loader = Some(v);
831        }
832
833        // If role_arn is set, we must use AssumeRoleLoad.
834        if let Some(role_arn) = self.config.role_arn {
835            // use current env as source credential loader.
836            let default_loader =
837                AwsDefaultLoader::new(GLOBAL_REQWEST_CLIENT.clone().clone(), cfg.clone());
838
839            // Build the config for assume role.
840            let mut assume_role_cfg = AwsConfig {
841                region: Some(region.clone()),
842                role_arn: Some(role_arn),
843                external_id: self.config.external_id.clone(),
844                sts_regional_endpoints: "regional".to_string(),
845                ..Default::default()
846            };
847
848            // override default role_session_name if set
849            if let Some(name) = self.config.role_session_name {
850                assume_role_cfg.role_session_name = name;
851            }
852
853            let assume_role_loader = AwsAssumeRoleLoader::new(
854                GLOBAL_REQWEST_CLIENT.clone().clone(),
855                assume_role_cfg,
856                Box::new(default_loader),
857            )
858            .map_err(|err| {
859                Error::new(
860                    ErrorKind::ConfigInvalid,
861                    "The assume_role_loader is misconfigured",
862                )
863                .with_context("service", Scheme::S3)
864                .set_source(err)
865            })?;
866            loader = Some(Box::new(assume_role_loader));
867        }
868        // If loader is not set, we will use default loader.
869        let loader = match loader {
870            Some(v) => v,
871            None => {
872                let mut default_loader =
873                    AwsDefaultLoader::new(GLOBAL_REQWEST_CLIENT.clone().clone(), cfg);
874                if self.config.disable_ec2_metadata {
875                    default_loader = default_loader.with_disable_ec2_metadata();
876                }
877
878                Box::new(default_loader)
879            }
880        };
881
882        let signer = AwsV4Signer::new("s3", &region);
883
884        let delete_max_size = self
885            .config
886            .delete_max_size
887            .unwrap_or(DEFAULT_BATCH_MAX_OPERATIONS);
888
889        Ok(S3Backend {
890            core: Arc::new(S3Core {
891                info: {
892                    let am = AccessorInfo::default();
893                    am.set_scheme(S3_SCHEME)
894                        .set_root(&root)
895                        .set_name(bucket)
896                        .set_native_capability(Capability {
897                            stat: true,
898                            stat_with_if_match: true,
899                            stat_with_if_none_match: true,
900                            stat_with_if_modified_since: true,
901                            stat_with_if_unmodified_since: true,
902                            stat_with_override_cache_control: !self
903                                .config
904                                .disable_stat_with_override,
905                            stat_with_override_content_disposition: !self
906                                .config
907                                .disable_stat_with_override,
908                            stat_with_override_content_type: !self
909                                .config
910                                .disable_stat_with_override,
911                            stat_with_version: self.config.enable_versioning,
912
913                            read: true,
914                            read_with_if_match: true,
915                            read_with_if_none_match: true,
916                            read_with_if_modified_since: true,
917                            read_with_if_unmodified_since: true,
918                            read_with_override_cache_control: true,
919                            read_with_override_content_disposition: true,
920                            read_with_override_content_type: true,
921                            read_with_version: self.config.enable_versioning,
922
923                            write: true,
924                            write_can_empty: true,
925                            write_can_multi: true,
926                            write_can_append: self.config.enable_write_with_append,
927
928                            write_with_cache_control: true,
929                            write_with_content_type: true,
930                            write_with_content_encoding: true,
931                            write_with_if_match: !self.config.disable_write_with_if_match,
932                            write_with_if_not_exists: true,
933                            write_with_user_metadata: true,
934
935                            // The min multipart size of S3 is 5 MiB.
936                            //
937                            // ref: <https://docs.aws.amazon.com/AmazonS3/latest/userguide/qfacts.html>
938                            write_multi_min_size: Some(5 * 1024 * 1024),
939                            // The max multipart size of S3 is 5 GiB.
940                            //
941                            // ref: <https://docs.aws.amazon.com/AmazonS3/latest/userguide/qfacts.html>
942                            write_multi_max_size: if cfg!(target_pointer_width = "64") {
943                                Some(5 * 1024 * 1024 * 1024)
944                            } else {
945                                Some(usize::MAX)
946                            },
947
948                            delete: true,
949                            delete_max_size: Some(delete_max_size),
950                            delete_with_version: self.config.enable_versioning,
951
952                            copy: true,
953
954                            list: true,
955                            list_with_limit: true,
956                            list_with_start_after: true,
957                            list_with_recursive: true,
958                            list_with_versions: self.config.enable_versioning,
959                            list_with_deleted: self.config.enable_versioning,
960
961                            presign: true,
962                            presign_stat: true,
963                            presign_read: true,
964                            presign_write: true,
965
966                            shared: true,
967
968                            ..Default::default()
969                        });
970
971                    // allow deprecated api here for compatibility
972                    #[allow(deprecated)]
973                    if let Some(client) = self.http_client {
974                        am.update_http_client(|_| client);
975                    }
976
977                    am.into()
978                },
979                bucket: bucket.to_string(),
980                endpoint,
981                root,
982                server_side_encryption,
983                server_side_encryption_aws_kms_key_id,
984                server_side_encryption_customer_algorithm,
985                server_side_encryption_customer_key,
986                server_side_encryption_customer_key_md5,
987                default_storage_class,
988                allow_anonymous: self.config.allow_anonymous,
989                disable_list_objects_v2: self.config.disable_list_objects_v2,
990                enable_request_payer: self.config.enable_request_payer,
991                signer,
992                loader,
993                credential_loaded: AtomicBool::new(false),
994                checksum_algorithm,
995            }),
996        })
997    }
998}
999
1000/// Backend for s3 services.
1001#[derive(Debug, Clone)]
1002pub struct S3Backend {
1003    core: Arc<S3Core>,
1004}
1005
1006impl Access for S3Backend {
1007    type Reader = HttpBody;
1008    type Writer = S3Writers;
1009    type Lister = S3Listers;
1010    type Deleter = oio::BatchDeleter<S3Deleter>;
1011
1012    fn info(&self) -> Arc<AccessorInfo> {
1013        self.core.info.clone()
1014    }
1015
1016    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
1017        let resp = self.core.s3_head_object(path, args).await?;
1018
1019        let status = resp.status();
1020
1021        match status {
1022            StatusCode::OK => {
1023                let headers = resp.headers();
1024                let mut meta = parse_into_metadata(path, headers)?;
1025
1026                let user_meta = parse_prefixed_headers(headers, X_AMZ_META_PREFIX);
1027                if !user_meta.is_empty() {
1028                    meta = meta.with_user_metadata(user_meta);
1029                }
1030
1031                if let Some(v) = parse_header_to_str(headers, X_AMZ_VERSION_ID)? {
1032                    meta.set_version(v);
1033                }
1034
1035                Ok(RpStat::new(meta))
1036            }
1037            _ => Err(parse_error(resp)),
1038        }
1039    }
1040
1041    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
1042        let resp = self.core.s3_get_object(path, args.range(), &args).await?;
1043
1044        let status = resp.status();
1045        match status {
1046            StatusCode::OK | StatusCode::PARTIAL_CONTENT => {
1047                Ok((RpRead::default(), resp.into_body()))
1048            }
1049            _ => {
1050                let (part, mut body) = resp.into_parts();
1051                let buf = body.to_buffer().await?;
1052                Err(parse_error(Response::from_parts(part, buf)))
1053            }
1054        }
1055    }
1056
1057    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
1058        let writer = S3Writer::new(self.core.clone(), path, args.clone());
1059
1060        let w = if args.append() {
1061            S3Writers::Two(oio::AppendWriter::new(writer))
1062        } else {
1063            S3Writers::One(oio::MultipartWriter::new(
1064                self.core.info.clone(),
1065                writer,
1066                args.concurrent(),
1067            ))
1068        };
1069
1070        Ok((RpWrite::default(), w))
1071    }
1072
1073    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
1074        Ok((
1075            RpDelete::default(),
1076            oio::BatchDeleter::new(S3Deleter::new(self.core.clone())),
1077        ))
1078    }
1079
1080    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
1081        let l = if args.versions() || args.deleted() {
1082            ThreeWays::Three(PageLister::new(S3ObjectVersionsLister::new(
1083                self.core.clone(),
1084                path,
1085                args,
1086            )))
1087        } else if self.core.disable_list_objects_v2 {
1088            ThreeWays::One(PageLister::new(S3ListerV1::new(
1089                self.core.clone(),
1090                path,
1091                args,
1092            )))
1093        } else {
1094            ThreeWays::Two(PageLister::new(S3ListerV2::new(
1095                self.core.clone(),
1096                path,
1097                args,
1098            )))
1099        };
1100
1101        Ok((RpList::default(), l))
1102    }
1103
1104    async fn copy(&self, from: &str, to: &str, _args: OpCopy) -> Result<RpCopy> {
1105        let resp = self.core.s3_copy_object(from, to).await?;
1106
1107        let status = resp.status();
1108
1109        match status {
1110            StatusCode::OK => Ok(RpCopy::default()),
1111            _ => Err(parse_error(resp)),
1112        }
1113    }
1114
1115    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
1116        let (expire, op) = args.into_parts();
1117        // We will not send this request out, just for signing.
1118        let req = match op {
1119            PresignOperation::Stat(v) => self.core.s3_head_object_request(path, v),
1120            PresignOperation::Read(v) => {
1121                self.core
1122                    .s3_get_object_request(path, BytesRange::default(), &v)
1123            }
1124            PresignOperation::Write(_) => {
1125                self.core
1126                    .s3_put_object_request(path, None, &OpWrite::default(), Buffer::new())
1127            }
1128            PresignOperation::Delete(_) => Err(Error::new(
1129                ErrorKind::Unsupported,
1130                "operation is not supported",
1131            )),
1132        };
1133        let mut req = req?;
1134
1135        self.core.sign_query(&mut req, expire).await?;
1136
1137        // We don't need this request anymore, consume it directly.
1138        let (parts, _) = req.into_parts();
1139
1140        Ok(RpPresign::new(PresignedRequest::new(
1141            parts.method,
1142            parts.uri,
1143            parts.headers,
1144        )))
1145    }
1146}
```
