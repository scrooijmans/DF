# 

opendal/services/

mod.rs

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
18//! Services will provide builders to build underlying backends.
19//!
20//! More ongoing services support is tracked at [opendal#5](https://github.com/apache/opendal/issues/5). Please feel free to submit issues if there are services not covered.
21
22#[cfg(feature = "services-aliyun-drive")]
23mod aliyun_drive;
24#[cfg(feature = "services-aliyun-drive")]
25pub use aliyun_drive::*;
26
27#[cfg(feature = "services-alluxio")]
28mod alluxio;
29#[cfg(feature = "services-alluxio")]
30pub use alluxio::*;
31
32#[cfg(feature = "services-azblob")]
33mod azblob;
34#[cfg(feature = "services-azblob")]
35pub use azblob::*;
36
37#[cfg(feature = "services-azdls")]
38mod azdls;
39#[cfg(feature = "services-azdls")]
40pub use azdls::*;
41
42#[cfg(feature = "services-azfile")]
43mod azfile;
44#[cfg(feature = "services-azfile")]
45pub use azfile::*;
46
47#[cfg(feature = "services-b2")]
48mod b2;
49#[cfg(feature = "services-b2")]
50pub use b2::*;
51
52#[cfg(feature = "services-cacache")]
53mod cacache;
54#[cfg(feature = "services-cacache")]
55pub use self::cacache::*;
56
57#[cfg(feature = "services-cloudflare-kv")]
58mod cloudflare_kv;
59#[cfg(feature = "services-cloudflare-kv")]
60pub use self::cloudflare_kv::*;
61
62#[cfg(feature = "services-compfs")]
63mod compfs;
64#[cfg(feature = "services-compfs")]
65pub use compfs::*;
66
67#[cfg(feature = "services-cos")]
68mod cos;
69#[cfg(feature = "services-cos")]
70pub use cos::*;
71
72#[cfg(feature = "services-d1")]
73mod d1;
74#[cfg(feature = "services-d1")]
75pub use self::d1::*;
76
77#[cfg(feature = "services-dashmap")]
78mod dashmap;
79#[cfg(feature = "services-dashmap")]
80pub use self::dashmap::*;
81
82#[cfg(feature = "services-dbfs")]
83mod dbfs;
84#[cfg(feature = "services-dbfs")]
85pub use self::dbfs::*;
86
87#[cfg(feature = "services-dropbox")]
88mod dropbox;
89#[cfg(feature = "services-dropbox")]
90pub use dropbox::*;
91
92#[cfg(feature = "services-etcd")]
93mod etcd;
94#[cfg(feature = "services-etcd")]
95pub use self::etcd::*;
96
97#[cfg(feature = "services-foundationdb")]
98mod foundationdb;
99#[cfg(feature = "services-foundationdb")]
100pub use self::foundationdb::*;
101
102#[cfg(feature = "services-fs")]
103mod fs;
104#[cfg(feature = "services-fs")]
105pub use fs::*;
106
107#[cfg(feature = "services-ftp")]
108mod ftp;
109#[cfg(feature = "services-ftp")]
110pub use ftp::*;
111
112#[cfg(feature = "services-gcs")]
113mod gcs;
114#[cfg(feature = "services-gcs")]
115pub use gcs::*;
116
117#[cfg(feature = "services-gdrive")]
118mod gdrive;
119#[cfg(feature = "services-gdrive")]
120pub use gdrive::*;
121
122#[cfg(feature = "services-ghac")]
123mod ghac;
124#[cfg(feature = "services-ghac")]
125pub use ghac::*;
126
127#[cfg(feature = "services-github")]
128mod github;
129#[cfg(feature = "services-github")]
130pub use github::*;
131
132#[cfg(feature = "services-gridfs")]
133mod gridfs;
134#[cfg(feature = "services-gridfs")]
135pub use gridfs::*;
136
137#[cfg(feature = "services-hdfs")]
138mod hdfs;
139#[cfg(feature = "services-hdfs")]
140pub use self::hdfs::*;
141
142#[cfg(feature = "services-hdfs-native")]
143mod hdfs_native;
144#[cfg(feature = "services-hdfs-native")]
145pub use hdfs_native::*;
146
147#[cfg(feature = "services-http")]
148mod http;
149#[cfg(feature = "services-http")]
150pub use self::http::*;
151
152#[cfg(feature = "services-huggingface")]
153mod huggingface;
154#[cfg(feature = "services-huggingface")]
155pub use huggingface::*;
156
157#[cfg(feature = "services-ipfs")]
158mod ipfs;
159#[cfg(feature = "services-ipfs")]
160pub use self::ipfs::*;
161
162#[cfg(feature = "services-ipmfs")]
163mod ipmfs;
164#[cfg(feature = "services-ipmfs")]
165pub use ipmfs::*;
166
167#[cfg(feature = "services-koofr")]
168mod koofr;
169#[cfg(feature = "services-koofr")]
170pub use koofr::*;
171
172#[cfg(feature = "services-lakefs")]
173mod lakefs;
174#[cfg(feature = "services-lakefs")]
175pub use lakefs::*;
176
177#[cfg(feature = "services-memcached")]
178mod memcached;
179#[cfg(feature = "services-memcached")]
180pub use memcached::*;
181
182#[cfg(feature = "services-memory")]
183mod memory;
184#[cfg(feature = "services-memory")]
185pub use self::memory::*;
186
187#[cfg(feature = "services-mini-moka")]
188mod mini_moka;
189#[cfg(feature = "services-mini-moka")]
190pub use self::mini_moka::*;
191
192#[cfg(feature = "services-moka")]
193mod moka;
194#[cfg(feature = "services-moka")]
195pub use self::moka::*;
196
197#[cfg(feature = "services-mongodb")]
198mod mongodb;
199#[cfg(feature = "services-mongodb")]
200pub use self::mongodb::*;
201
202#[cfg(feature = "services-monoiofs")]
203mod monoiofs;
204#[cfg(feature = "services-monoiofs")]
205pub use monoiofs::*;
206
207#[cfg(feature = "services-mysql")]
208mod mysql;
209#[cfg(feature = "services-mysql")]
210pub use self::mysql::*;
211
212#[cfg(feature = "services-obs")]
213mod obs;
214#[cfg(feature = "services-obs")]
215pub use obs::*;
216
217#[cfg(feature = "services-onedrive")]
218mod onedrive;
219#[cfg(feature = "services-onedrive")]
220pub use onedrive::*;
221
222#[cfg(feature = "services-oss")]
223mod oss;
224#[cfg(feature = "services-oss")]
225pub use oss::*;
226
227#[cfg(feature = "services-pcloud")]
228mod pcloud;
229#[cfg(feature = "services-pcloud")]
230pub use pcloud::*;
231
232#[cfg(feature = "services-persy")]
233mod persy;
234#[cfg(feature = "services-persy")]
235pub use self::persy::*;
236
237#[cfg(feature = "services-postgresql")]
238mod postgresql;
239#[cfg(feature = "services-postgresql")]
240pub use self::postgresql::*;
241
242#[cfg(feature = "services-redb")]
243mod redb;
244#[cfg(feature = "services-redb")]
245pub use self::redb::*;
246
247#[cfg(feature = "services-redis")]
248mod redis;
249#[cfg(feature = "services-redis")]
250pub use self::redis::*;
251
252#[cfg(feature = "services-rocksdb")]
253mod rocksdb;
254#[cfg(feature = "services-rocksdb")]
255pub use self::rocksdb::*;
256
257#[cfg(feature = "services-s3")]
258mod s3;
259#[cfg(feature = "services-s3")]
260pub use s3::*;
261
262#[cfg(feature = "services-seafile")]
263mod seafile;
264#[cfg(feature = "services-seafile")]
265pub use seafile::*;
266
267#[cfg(feature = "services-sftp")]
268mod sftp;
269#[cfg(feature = "services-sftp")]
270pub use sftp::*;
271
272#[cfg(feature = "services-sled")]
273mod sled;
274#[cfg(feature = "services-sled")]
275pub use self::sled::*;
276
277#[cfg(feature = "services-sqlite")]
278mod sqlite;
279#[cfg(feature = "services-sqlite")]
280pub use self::sqlite::*;
281
282#[cfg(feature = "services-surrealdb")]
283mod surrealdb;
284#[cfg(feature = "services-surrealdb")]
285pub use surrealdb::*;
286
287#[cfg(feature = "services-swift")]
288mod swift;
289#[cfg(feature = "services-swift")]
290pub use self::swift::*;
291
292#[cfg(feature = "services-tikv")]
293mod tikv;
294#[cfg(feature = "services-tikv")]
295pub use self::tikv::*;
296
297#[cfg(feature = "services-upyun")]
298mod upyun;
299#[cfg(feature = "services-upyun")]
300pub use upyun::*;
301
302#[cfg(feature = "services-vercel-artifacts")]
303mod vercel_artifacts;
304#[cfg(feature = "services-vercel-artifacts")]
305pub use vercel_artifacts::*;
306
307#[cfg(feature = "services-vercel-blob")]
308mod vercel_blob;
309#[cfg(feature = "services-vercel-blob")]
310pub use vercel_blob::*;
311
312#[cfg(feature = "services-webdav")]
313mod webdav;
314#[cfg(feature = "services-webdav")]
315pub use webdav::*;
316
317#[cfg(feature = "services-webhdfs")]
318mod webhdfs;
319#[cfg(feature = "services-webhdfs")]
320pub use webhdfs::*;
321
322#[cfg(feature = "services-yandex-disk")]
323mod yandex_disk;
324#[cfg(feature = "services-yandex-disk")]
325pub use yandex_disk::*;
326
327#[cfg(all(target_arch = "wasm32", feature = "services-opfs"))]
328mod opfs;
```
