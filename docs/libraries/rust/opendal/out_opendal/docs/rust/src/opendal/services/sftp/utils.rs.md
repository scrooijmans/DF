# 

opendal/services/sftp/

utils.rs

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
18use openssh_sftp_client::metadata::MetaData as SftpMeta;
19
20use crate::EntryMode;
21use crate::Metadata;
22use crate::raw::Timestamp;
23
24/// REMOVE ME: we should not implement `From<SftpMeta> for Metadata`.
25impl From<SftpMeta> for Metadata {
26    fn from(meta: SftpMeta) -> Self {
27        let mode = meta
28            .file_type()
29            .map(|filetype| {
30                if filetype.is_file() {
31                    EntryMode::FILE
32                } else if filetype.is_dir() {
33                    EntryMode::DIR
34                } else {
35                    EntryMode::Unknown
36                }
37            })
38            .unwrap_or(EntryMode::Unknown);
39
40        let mut metadata = Metadata::new(mode);
41
42        if let Some(size) = meta.len() {
43            metadata.set_content_length(size);
44        }
45
46        if let Some(modified) = meta.modified() {
47            if let Ok(m) = Timestamp::try_from(modified.as_system_time()) {
48                metadata.set_last_modified(m);
49            }
50        }
51
52        metadata
53    }
54}
```
