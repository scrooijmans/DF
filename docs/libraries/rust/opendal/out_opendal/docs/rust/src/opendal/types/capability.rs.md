# 

opendal/types/

capability.rs

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
19
20/// Capability defines the supported operations and their constraints for a storage Operator.
21///
22/// # Overview
23///
24/// This structure provides a comprehensive description of an Operator's capabilities,
25/// including:
26///
27/// - Basic operations support (read, write, delete, etc.)
28/// - Advanced operation variants (conditional operations, metadata handling)
29/// - Operational constraints (size limits, batch limitations)
30///
31/// # Capability Types
32///
33/// Every operator maintains two capability sets:
34///
35/// 1. [`OperatorInfo::native_capability`][crate::OperatorInfo::native_capability]:
36///    Represents operations natively supported by the storage backend.
37///
38/// 2. [`OperatorInfo::full_capability`][crate::OperatorInfo::full_capability]:
39///    Represents all available operations, including those implemented through
40///    alternative mechanisms.
41///
42/// # Implementation Details
43///
44/// Some operations might be available even when not natively supported by the
45/// backend. For example:
46///
47/// - Blocking operations are provided through the BlockingLayer
48///
49/// Developers should:
50/// - Use `full_capability` to determine available operations
51/// - Use `native_capability` to identify optimized operations
52///
53/// # Field Naming Conventions
54///
55/// Fields follow these naming patterns:
56///
57/// - Basic operations: Simple lowercase (e.g., `read`, `write`)
58/// - Compound operations: Underscore-separated (e.g., `presign_read`)
59/// - Variants: Capability description (e.g., `write_can_empty`)
60/// - Parameterized operations: With-style (e.g., `read_with_if_match`)
61/// - Limitations: Constraint description (e.g., `write_multi_max_size`)
62/// - Metadata Results: Returning metadata capabilities (e.g., `stat_has_content_length`)
63///
64/// All capability fields are public and can be accessed directly.
65#[derive(Copy, Clone, Default)]
66pub struct Capability {
67    /// Indicates if the operator supports metadata retrieval operations.
68    pub stat: bool,
69    /// Indicates if conditional stat operations using If-Match are supported.
70    pub stat_with_if_match: bool,
71    /// Indicates if conditional stat operations using If-None-Match are supported.
72    pub stat_with_if_none_match: bool,
73    /// Indicates if conditional stat operations using If-Modified-Since are supported.
74    pub stat_with_if_modified_since: bool,
75    /// Indicates if conditional stat operations using If-Unmodified-Since are supported.
76    pub stat_with_if_unmodified_since: bool,
77    /// Indicates if Cache-Control header override is supported during stat operations.
78    pub stat_with_override_cache_control: bool,
79    /// Indicates if Content-Disposition header override is supported during stat operations.
80    pub stat_with_override_content_disposition: bool,
81    /// Indicates if Content-Type header override is supported during stat operations.
82    pub stat_with_override_content_type: bool,
83    /// Indicates if versions stat operations are supported.
84    pub stat_with_version: bool,
85
86    /// Indicates if the operator supports read operations.
87    pub read: bool,
88    /// Indicates if conditional read operations using If-Match are supported.
89    pub read_with_if_match: bool,
90    /// Indicates if conditional read operations using If-None-Match are supported.
91    pub read_with_if_none_match: bool,
92    /// Indicates if conditional read operations using If-Modified-Since are supported.
93    pub read_with_if_modified_since: bool,
94    /// Indicates if conditional read operations using If-Unmodified-Since are supported.
95    pub read_with_if_unmodified_since: bool,
96    /// Indicates if Cache-Control header override is supported during read operations.
97    pub read_with_override_cache_control: bool,
98    /// Indicates if Content-Disposition header override is supported during read operations.
99    pub read_with_override_content_disposition: bool,
100    /// Indicates if Content-Type header override is supported during read operations.
101    pub read_with_override_content_type: bool,
102    /// Indicates if versions read operations are supported.
103    pub read_with_version: bool,
104
105    /// Indicates if the operator supports write operations.
106    pub write: bool,
107    /// Indicates if multiple write operations can be performed on the same object.
108    pub write_can_multi: bool,
109    /// Indicates if writing empty content is supported.
110    pub write_can_empty: bool,
111    /// Indicates if append operations are supported.
112    pub write_can_append: bool,
113    /// Indicates if Content-Type can be specified during write operations.
114    pub write_with_content_type: bool,
115    /// Indicates if Content-Disposition can be specified during write operations.
116    pub write_with_content_disposition: bool,
117    /// Indicates if Content-Encoding can be specified during write operations.
118    pub write_with_content_encoding: bool,
119    /// Indicates if Cache-Control can be specified during write operations.
120    pub write_with_cache_control: bool,
121    /// Indicates if conditional write operations using If-Match are supported.
122    pub write_with_if_match: bool,
123    /// Indicates if conditional write operations using If-None-Match are supported.
124    pub write_with_if_none_match: bool,
125    /// Indicates if write operations can be conditional on object non-existence.
126    pub write_with_if_not_exists: bool,
127    /// Indicates if custom user metadata can be attached during write operations.
128    pub write_with_user_metadata: bool,
129    /// Maximum size supported for multipart uploads.
130    /// For example, AWS S3 supports up to 5GiB per part in multipart uploads.
131    pub write_multi_max_size: Option<usize>,
132    /// Minimum size required for multipart uploads (except for the last part).
133    /// For example, AWS S3 requires at least 5MiB per part.
134    pub write_multi_min_size: Option<usize>,
135    /// Maximum total size supported for write operations.
136    /// For example, Cloudflare D1 has a 1MB total size limit.
137    pub write_total_max_size: Option<usize>,
138
139    /// Indicates if directory creation is supported.
140    pub create_dir: bool,
141
142    /// Indicates if delete operations are supported.
143    pub delete: bool,
144    /// Indicates if versions delete operations are supported.
145    pub delete_with_version: bool,
146    /// Maximum size supported for single delete operations.
147    pub delete_max_size: Option<usize>,
148
149    /// Indicates if copy operations are supported.
150    pub copy: bool,
151    /// Indicates if conditional copy operations with if-not-exists are supported.
152    pub copy_with_if_not_exists: bool,
153
154    /// Indicates if rename operations are supported.
155    pub rename: bool,
156
157    /// Indicates if list operations are supported.
158    pub list: bool,
159    /// Indicates if list operations support result limiting.
160    pub list_with_limit: bool,
161    /// Indicates if list operations support continuation from a specific point.
162    pub list_with_start_after: bool,
163    /// Indicates if recursive listing is supported.
164    pub list_with_recursive: bool,
165    /// Indicates if versions listing is supported.
166    #[deprecated(since = "0.51.1", note = "use with_versions instead")]
167    pub list_with_version: bool,
168    /// Indicates if listing with versions included is supported.
169    pub list_with_versions: bool,
170    /// Indicates if listing with deleted files included is supported.
171    pub list_with_deleted: bool,
172
173    /// Indicates if presigned URL generation is supported.
174    pub presign: bool,
175    /// Indicates if presigned URLs for read operations are supported.
176    pub presign_read: bool,
177    /// Indicates if presigned URLs for stat operations are supported.
178    pub presign_stat: bool,
179    /// Indicates if presigned URLs for write operations are supported.
180    pub presign_write: bool,
181    /// Indicates if presigned URLs for delete operations are supported.
182    pub presign_delete: bool,
183
184    /// Indicate if the operator supports shared access.
185    pub shared: bool,
186}
187
188impl Debug for Capability {
189    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
190        // NOTE: All services in opendal are readable.
191        if self.read {
192            f.write_str("Read")?;
193        }
194        if self.write {
195            f.write_str("| Write")?;
196        }
197        if self.list {
198            f.write_str("| List")?;
199        }
200        if self.presign {
201            f.write_str("| Presign")?;
202        }
203        if self.shared {
204            f.write_str("| Shared")?;
205        }
206
207        Ok(())
208    }
209}
```
