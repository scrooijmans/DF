# 

dav_server_opendalfs/

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
18//! dav-server-opendalfs is an dav-server implementation using opendal.
19//!
20//! This crate can help you to access ANY storage services with the same webdav API.
21//!
22//! ```
23//! use anyhow::Result;
24//! use dav_server::davpath::DavPath;
25//! use dav_server::fs::DavFileSystem;
26//! use dav_server_opendalfs::OpendalFs;
27//! use opendal::services::Memory;
28//! use opendal::Operator;
29//!
30//! #[tokio::test]
31//! async fn test() -> Result<()> {
32//!     let op = Operator::new(Memory::default())?.finish();
33//!
34//!     let webdavfs = OpendalFs::new(op);
35//!
36//!     let metadata = webdavfs
37//!         .metadata(&DavPath::new("/").unwrap())
38//!         .await
39//!         .unwrap();
40//!     println!("{}", metadata.is_dir());
41//!
42//!     Ok(())
43//! }
44//! ```
45
46mod dir;
47mod file;
48mod metadata;
49mod utils;
50
51mod fs;
52pub use fs::OpendalFs;
```
