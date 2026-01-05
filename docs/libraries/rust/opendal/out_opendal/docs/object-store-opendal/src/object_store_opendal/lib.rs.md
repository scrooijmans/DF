# 

object_store_opendal/

lib.rs

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
18//! object_store_opendal is an object store implementation using opendal.
19//!
20//! This crate can help you to access 30 more storage services with the same object_store API.
21//!
22//! ```no_run
23//! use std::sync::Arc;
24//!
25//! use bytes::Bytes;
26//! use object_store::path::Path;
27//! use object_store::ObjectStore;
28//! use object_store_opendal::OpendalStore;
29//! use opendal::services::S3;
30//! use opendal::{Builder, Operator};
31//!
32//! #[tokio::main]
33//! async fn main() {
34//!    let builder = S3::default()
35//!     .access_key_id("my_access_key")
36//!     .secret_access_key("my_secret_key")
37//!     .endpoint("my_endpoint")
38//!     .region("my_region");
39//!
40//!     // Create a new operator
41//!     let operator = Operator::new(builder).unwrap().finish();
42//!
43//!     // Create a new object store
44//!     let object_store = Arc::new(OpendalStore::new(operator));
45//!
46//!     let path = Path::from("data/nested/test.txt");
47//!     let bytes = Bytes::from_static(b"hello, world! I am nested.");
48//!
49//!     object_store.put(&path, bytes.clone().into()).await.unwrap();
50//!
51//!     let content = object_store
52//!         .get(&path)
53//!         .await
54//!         .unwrap()
55//!         .bytes()
56//!         .await
57//!         .unwrap();
58//!
59//!     assert_eq!(content, bytes);
60//! }
61//! ```
62
63mod store;
64pub use store::OpendalStore;
65
66mod utils;
67
68#[cfg(feature = "services-s3")]
69mod amazon_s3;
70
71mod service;
72
73pub use service::{ObjectStoreBuilder, ObjectStoreService};
74
75// Make sure `send_wrapper` works as expected
76#[cfg(all(feature = "send_wrapper", test))]
77mod assert_send {
78    use object_store::{ObjectStore, PutPayload};
79    use opendal::Operator;
80
81    #[allow(dead_code)]
82    fn assert_send<T: Send>(_: T) {}
83
84    #[allow(dead_code)]
85    fn assertion() {
86        let op = Operator::new(opendal::services::Memory::default())
87            .unwrap()
88            .finish();
89        let store = super::OpendalStore::new(op);
90        assert_send(store.put(&"test".into(), PutPayload::new()));
91        assert_send(store.get(&"test".into()));
92        assert_send(store.get_range(&"test".into(), 0..1));
93        assert_send(store.head(&"test".into()));
94        assert_send(store.delete(&"test".into()));
95        assert_send(store.list(None));
96        assert_send(store.list_with_offset(None, &"test".into()));
97        assert_send(store.list_with_delimiter(None));
98    }
99}
100
101fn timestamp_to_datetime(ts: opendal::raw::Timestamp) -> Option<chrono::DateTime<chrono::Utc>> {
102    let jiff_ts = ts.into_inner();
103    chrono::DateTime::<chrono::Utc>::from_timestamp(
104        jiff_ts.as_second(),
105        jiff_ts.subsec_nanosecond() as u32,
106    )
107}
108
109fn datetime_to_timestamp(dt: chrono::DateTime<chrono::Utc>) -> Option<opendal::raw::Timestamp> {
110    opendal::raw::Timestamp::new(dt.timestamp(), dt.timestamp_subsec_nanos() as i32).ok()
111}
```
