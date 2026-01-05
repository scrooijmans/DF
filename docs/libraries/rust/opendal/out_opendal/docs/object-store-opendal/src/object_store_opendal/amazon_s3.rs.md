# 

object_store_opendal/

amazon_s3.rs

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
18use crate::OpendalStore;
19use crate::utils::format_object_store_error;
20use object_store::aws::{AmazonS3Builder, AmazonS3ConfigKey};
21use opendal::Operator;
22use opendal::services::S3;
23
24impl OpendalStore {
25    /// Create OpendalStore from object_store Amazon S3 builder.
26    pub fn new_amazon_s3(builder: AmazonS3Builder) -> object_store::Result<OpendalStore> {
27        let mut s3 = S3::default();
28        if let Some(endpoint) = builder.get_config_value(&AmazonS3ConfigKey::Endpoint) {
29            s3 = s3.endpoint(endpoint.as_str());
30        }
31        if let Some(region) = builder.get_config_value(&AmazonS3ConfigKey::Region) {
32            s3 = s3.region(region.as_str());
33        }
34        if let Some(bucket_name) = builder.get_config_value(&AmazonS3ConfigKey::Bucket) {
35            s3 = s3.bucket(bucket_name.as_str());
36        }
37        if let Some(access_key_id) = builder.get_config_value(&AmazonS3ConfigKey::AccessKeyId) {
38            s3 = s3.access_key_id(access_key_id.as_str());
39        }
40        if let Some(secret_access_key) =
41            builder.get_config_value(&AmazonS3ConfigKey::SecretAccessKey)
42        {
43            s3 = s3.secret_access_key(secret_access_key.as_str());
44        }
45        if let Some(token) = builder.get_config_value(&AmazonS3ConfigKey::Token) {
46            s3 = s3.session_token(token.as_str());
47        }
48        if let Some(virtual_hosted_style_request) =
49            builder.get_config_value(&AmazonS3ConfigKey::VirtualHostedStyleRequest)
50        {
51            let r = virtual_hosted_style_request
52                .parse::<bool>()
53                .map_err(|err| object_store::Error::Generic {
54                    store: "s3",
55                    source: Box::new(err),
56                })?;
57            if r {
58                s3 = s3.enable_virtual_host_style();
59            }
60        }
61        if let Some(skip_signature) = builder.get_config_value(&AmazonS3ConfigKey::SkipSignature) {
62            let r = skip_signature
63                .parse::<bool>()
64                .map_err(|err| object_store::Error::Generic {
65                    store: "s3",
66                    source: Box::new(err),
67                })?;
68            if r {
69                s3 = s3.allow_anonymous();
70            }
71        }
72
73        let op = Operator::new(s3)
74            .map_err(|err| format_object_store_error(err, ""))?
75            .finish();
76        Ok(OpendalStore::new(op))
77    }
78}
```
