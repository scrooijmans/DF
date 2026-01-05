# 

opendal/types/

scheme.rs

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
18use std::collections::HashSet;
19use std::fmt::Display;
20use std::fmt::Formatter;
21use std::str::FromStr;
22
23use crate::Error;
24
25/// Services that OpenDAL supports
26///
27/// # Notes
28///
29/// - Scheme is `non_exhaustive`, new variant COULD be added at any time.
30/// - New variant SHOULD be added in alphabet orders,
31/// - Users MUST NOT relay on its order.
32#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
33#[non_exhaustive]
34pub enum Scheme {
35    /// [aliyun-drive][crate::services::AliyunDrive]: Aliyun Drive services.
36    AliyunDrive,
37    /// [atomicserver][crate::services::Atomicserver]: Atomicserver services.
38    Atomicserver,
39    /// [azblob][crate::services::Azblob]: Azure Storage Blob services.
40    Azblob,
41    /// [Azdls][crate::services::Azdls]: Azure Data Lake Storage Gen2.
42    Azdls,
43    /// [B2][crate::services::B2]: Backblaze B2 Services.
44    B2,
45    /// [Compfs][crate::services::Compfs]: Compio fs Services.
46    Compfs,
47    /// [Seafile][crate::services::Seafile]: Seafile Services.
48    Seafile,
49    /// [Upyun][crate::services::Upyun]: Upyun Services.
50    Upyun,
51    /// [VercelBlob][crate::services::VercelBlob]: VercelBlob Services.
52    VercelBlob,
53    /// [YandexDisk][crate::services::YandexDisk]: YandexDisk Services.
54    YandexDisk,
55    /// [Pcloud][crate::services::Pcloud]: Pcloud Services.
56    Pcloud,
57    /// [Koofr][crate::services::Koofr]: Koofr Services.
58    Koofr,
59    /// [cacache][crate::services::Cacache]: cacache backend support.
60    Cacache,
61    /// [cloudflare-kv][crate::services::CloudflareKv]: Cloudflare KV services.
62    CloudflareKv,
63    /// [cos][crate::services::Cos]: Tencent Cloud Object Storage services.
64    Cos,
65    /// [d1][crate::services::D1]: D1 services
66    D1,
67    /// [dashmap][crate::services::Dashmap]: dashmap backend support.
68    Dashmap,
69    /// [etcd][crate::services::Etcd]: Etcd Services
70    Etcd,
71    /// [foundationdb][crate::services::Foundationdb]: Foundationdb services.
72    Foundationdb,
73    /// [dbfs][crate::services::Dbfs]: DBFS backend support.
74    Dbfs,
75    /// [fs][crate::services::Fs]: POSIX-like file system.
76    Fs,
77    /// [ftp][crate::services::Ftp]: FTP backend.
78    Ftp,
79    /// [gcs][crate::services::Gcs]: Google Cloud Storage backend.
80    Gcs,
81    /// [ghac][crate::services::Ghac]: GitHub Action Cache services.
82    Ghac,
83    /// [hdfs][crate::services::Hdfs]: Hadoop Distributed File System.
84    Hdfs,
85    /// [http][crate::services::Http]: HTTP backend.
86    Http,
87    /// [huggingface][crate::services::Huggingface]: Huggingface services.
88    Huggingface,
89    /// [alluxio][crate::services::Alluxio]: Alluxio services.
90    Alluxio,
91
92    /// [ipmfs][crate::services::Ipfs]: IPFS HTTP Gateway
93    Ipfs,
94    /// [ipmfs][crate::services::Ipmfs]: IPFS mutable file system
95    Ipmfs,
96    /// [icloud][crate::services::Icloud]: APPLE icloud services.
97    Icloud,
98    /// [memcached][crate::services::Memcached]: Memcached service support.
99    Memcached,
100    /// [memory][crate::services::Memory]: In memory backend support.
101    Memory,
102    /// [mini-moka][crate::services::MiniMoka]: Mini Moka backend support.
103    MiniMoka,
104    /// [moka][crate::services::Moka]: moka backend support.
105    Moka,
106    /// [monoiofs][crate::services::Monoiofs]: monoio fs services.
107    Monoiofs,
108    /// [obs][crate::services::Obs]: Huawei Cloud OBS services.
109    Obs,
110    /// [onedrive][crate::services::Onedrive]: Microsoft OneDrive services.
111    Onedrive,
112    /// [gdrive][crate::services::Gdrive]: GoogleDrive services.
113    Gdrive,
114    /// [dropbox][crate::services::Dropbox]: Dropbox services.
115    Dropbox,
116    /// [oss][crate::services::Oss]: Aliyun Object Storage Services
117    Oss,
118    /// [persy][crate::services::Persy]: persy backend support.
119    Persy,
120    /// [redis][crate::services::Redis]: Redis services
121    Redis,
122    /// [postgresql][crate::services::Postgresql]: Postgresql services
123    Postgresql,
124    /// [mysql][crate::services::Mysql]: Mysql services
125    Mysql,
126    /// [sqlite][crate::services::Sqlite]: Sqlite services
127    Sqlite,
128    /// [rocksdb][crate::services::Rocksdb]: RocksDB services
129    Rocksdb,
130    /// [s3][crate::services::S3]: AWS S3 alike services.
131    S3,
132    /// [sftp][crate::services::Sftp]: SFTP services
133    Sftp,
134    /// [sled][crate::services::Sled]: Sled services
135    Sled,
136    /// [swift][crate::services::Swift]: Swift backend support.
137    Swift,
138    /// [Vercel Artifacts][crate::services::VercelArtifacts]: Vercel Artifacts service, as known as Vercel Remote Caching.
139    VercelArtifacts,
140    /// [webdav][crate::services::Webdav]: WebDAV support.
141    Webdav,
142    /// [webhdfs][crate::services::Webhdfs]: WebHDFS RESTful API Services
143    Webhdfs,
144    /// [redb][crate::services::Redb]: Redb Services
145    Redb,
146    /// [tikv][crate::services::Tikv]: Tikv Services
147    Tikv,
148    /// [azfile][crate::services::Azfile]: Azfile Services
149    Azfile,
150    /// [mongodb](crate::services::Mongodb): MongoDB Services
151    Mongodb,
152    /// [gridfs](crate::services::Gridfs): MongoDB Gridfs Services
153    Gridfs,
154    /// [Github Contents][crate::services::Github]: Github contents support.
155    Github,
156    /// [Native HDFS](crate::services::HdfsNative): Hdfs Native service, using rust hdfs-native client for hdfs
157    HdfsNative,
158    /// [surrealdb](crate::services::Surrealdb): Surrealdb Services
159    Surrealdb,
160    /// [lakefs](crate::services::Lakefs): LakeFS Services
161    Lakefs,
162    /// [NebulaGraph](crate::services::NebulaGraph): NebulaGraph Services
163    NebulaGraph,
164    /// Custom that allow users to implement services outside OpenDAL.
165    ///
166    /// # NOTE
167    ///
168    /// - Custom must not overwrite any existing services name.
169    /// - Custom must be in lower case.
170    Custom(&'static str),
171}
172
173impl Scheme {
174    /// Convert self into static str.
175    pub fn into_static(self) -> &'static str {
176        self.into()
177    }
178
179    /// Get all enabled schemes.
180    ///
181    /// OpenDAL could be compiled with different features, which will enable different schemes.
182    /// This function returns all enabled schemes so users can make decisions based on it.
183    ///
184    /// # Examples
185    ///
186    /// ```rust,no_run
187    /// use opendal::Scheme;
188    ///
189    /// let enabled_schemes = Scheme::enabled();
190    /// if !enabled_schemes.contains(&Scheme::Memory) {
191    ///     panic!("s3 support is not enabled")
192    /// }
193    /// ```
194    pub fn enabled() -> HashSet<Scheme> {
195        HashSet::from([
196            #[cfg(feature = "services-aliyun-drive")]
197            Scheme::AliyunDrive,
198            #[cfg(feature = "services-alluxio")]
199            Scheme::Alluxio,
200            #[cfg(feature = "services-azblob")]
201            Scheme::Azblob,
202            #[cfg(feature = "services-azdls")]
203            Scheme::Azdls,
204            #[cfg(feature = "services-azfile")]
205            Scheme::Azfile,
206            #[cfg(feature = "services-b2")]
207            Scheme::B2,
208            #[cfg(feature = "services-cacache")]
209            Scheme::Cacache,
210            #[cfg(feature = "services-cos")]
211            Scheme::Cos,
212            #[cfg(feature = "services-compfs")]
213            Scheme::Compfs,
214            #[cfg(feature = "services-dashmap")]
215            Scheme::Dashmap,
216            #[cfg(feature = "services-dropbox")]
217            Scheme::Dropbox,
218            #[cfg(feature = "services-etcd")]
219            Scheme::Etcd,
220            #[cfg(feature = "services-foundationdb")]
221            Scheme::Foundationdb,
222            #[cfg(feature = "services-fs")]
223            Scheme::Fs,
224            #[cfg(feature = "services-ftp")]
225            Scheme::Ftp,
226            #[cfg(feature = "services-gcs")]
227            Scheme::Gcs,
228            #[cfg(feature = "services-ghac")]
229            Scheme::Ghac,
230            #[cfg(feature = "services-hdfs")]
231            Scheme::Hdfs,
232            #[cfg(feature = "services-http")]
233            Scheme::Http,
234            #[cfg(feature = "services-huggingface")]
235            Scheme::Huggingface,
236            #[cfg(feature = "services-ipfs")]
237            Scheme::Ipfs,
238            #[cfg(feature = "services-ipmfs")]
239            Scheme::Ipmfs,
240            #[cfg(feature = "services-memcached")]
241            Scheme::Memcached,
242            #[cfg(feature = "services-memory")]
243            Scheme::Memory,
244            #[cfg(feature = "services-mini-moka")]
245            Scheme::MiniMoka,
246            #[cfg(feature = "services-moka")]
247            Scheme::Moka,
248            #[cfg(feature = "services-monoiofs")]
249            Scheme::Monoiofs,
250            #[cfg(feature = "services-mysql")]
251            Scheme::Mysql,
252            #[cfg(feature = "services-obs")]
253            Scheme::Obs,
254            #[cfg(feature = "services-onedrive")]
255            Scheme::Onedrive,
256            #[cfg(feature = "services-postgresql")]
257            Scheme::Postgresql,
258            #[cfg(feature = "services-gdrive")]
259            Scheme::Gdrive,
260            #[cfg(feature = "services-oss")]
261            Scheme::Oss,
262            #[cfg(feature = "services-persy")]
263            Scheme::Persy,
264            #[cfg(feature = "services-redis")]
265            Scheme::Redis,
266            #[cfg(feature = "services-rocksdb")]
267            Scheme::Rocksdb,
268            #[cfg(feature = "services-s3")]
269            Scheme::S3,
270            #[cfg(feature = "services-seafile")]
271            Scheme::Seafile,
272            #[cfg(feature = "services-upyun")]
273            Scheme::Upyun,
274            #[cfg(feature = "services-yandex-disk")]
275            Scheme::YandexDisk,
276            #[cfg(feature = "services-pcloud")]
277            Scheme::Pcloud,
278            #[cfg(feature = "services-sftp")]
279            Scheme::Sftp,
280            #[cfg(feature = "services-sled")]
281            Scheme::Sled,
282            #[cfg(feature = "services-sqlite")]
283            Scheme::Sqlite,
284            #[cfg(feature = "services-swift")]
285            Scheme::Swift,
286            #[cfg(feature = "services-tikv")]
287            Scheme::Tikv,
288            #[cfg(feature = "services-vercel-artifacts")]
289            Scheme::VercelArtifacts,
290            #[cfg(feature = "services-vercel-blob")]
291            Scheme::VercelBlob,
292            #[cfg(feature = "services-webdav")]
293            Scheme::Webdav,
294            #[cfg(feature = "services-webhdfs")]
295            Scheme::Webhdfs,
296            #[cfg(feature = "services-redb")]
297            Scheme::Redb,
298            #[cfg(feature = "services-mongodb")]
299            Scheme::Mongodb,
300            #[cfg(feature = "services-hdfs-native")]
301            Scheme::HdfsNative,
302            #[cfg(feature = "services-surrealdb")]
303            Scheme::Surrealdb,
304            #[cfg(feature = "services-lakefs")]
305            Scheme::Lakefs,
306            #[cfg(feature = "services-cloudflare-kv")]
307            Scheme::CloudflareKv,
308        ])
309    }
310}
311
312impl Default for Scheme {
313    fn default() -> Self {
314        Self::Memory
315    }
316}
317
318impl Display for Scheme {
319    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
320        write!(f, "{}", self.into_static())
321    }
322}
323
324impl FromStr for Scheme {
325    type Err = Error;
326
327    fn from_str(s: &str) -> Result<Self, Self::Err> {
328        let s = s.to_lowercase();
329        match s.as_str() {
330            "aliyun-drive" | "aliyun_drive" => Ok(Scheme::AliyunDrive),
331            "azblob" => Ok(Scheme::Azblob),
332            "alluxio" => Ok(Scheme::Alluxio),
333            // Notes:
334            //
335            // OpenDAL used to call `azdls` as `azdfs`, we keep it for backward compatibility.
336            // And abfs is widely used in hadoop ecosystem, keep it for easy to use.
337            "azdls" | "azdfs" | "abfs" => Ok(Scheme::Azdls),
338            "b2" => Ok(Scheme::B2),
339            "cacache" => Ok(Scheme::Cacache),
340            "compfs" => Ok(Scheme::Compfs),
341            "cloudflare-kv" | "cloudflare_kv" => Ok(Scheme::CloudflareKv),
342            "cos" => Ok(Scheme::Cos),
343            "d1" => Ok(Scheme::D1),
344            "dashmap" => Ok(Scheme::Dashmap),
345            "dropbox" => Ok(Scheme::Dropbox),
346            "etcd" => Ok(Scheme::Etcd),
347            "dbfs" => Ok(Scheme::Dbfs),
348            "fs" => Ok(Scheme::Fs),
349            "gcs" => Ok(Scheme::Gcs),
350            "gdrive" => Ok(Scheme::Gdrive),
351            "ghac" => Ok(Scheme::Ghac),
352            "gridfs" => Ok(Scheme::Gridfs),
353            "github" => Ok(Scheme::Github),
354            "hdfs" => Ok(Scheme::Hdfs),
355            "http" | "https" => Ok(Scheme::Http),
356            "huggingface" | "hf" => Ok(Scheme::Huggingface),
357            "ftp" | "ftps" => Ok(Scheme::Ftp),
358            "ipfs" | "ipns" => Ok(Scheme::Ipfs),
359            "ipmfs" => Ok(Scheme::Ipmfs),
360            "koofr" => Ok(Scheme::Koofr),
361            "memcached" => Ok(Scheme::Memcached),
362            "memory" => Ok(Scheme::Memory),
363            "mysql" => Ok(Scheme::Mysql),
364            "sqlite" => Ok(Scheme::Sqlite),
365            "mini-moka" | "mini_moka" => Ok(Scheme::MiniMoka),
366            "moka" => Ok(Scheme::Moka),
367            "monoiofs" => Ok(Scheme::Monoiofs),
368            "obs" => Ok(Scheme::Obs),
369            "onedrive" => Ok(Scheme::Onedrive),
370            "persy" => Ok(Scheme::Persy),
371            "postgresql" => Ok(Scheme::Postgresql),
372            "redb" => Ok(Scheme::Redb),
373            "redis" => Ok(Scheme::Redis),
374            "rocksdb" => Ok(Scheme::Rocksdb),
375            "s3" => Ok(Scheme::S3),
376            "seafile" => Ok(Scheme::Seafile),
377            "upyun" => Ok(Scheme::Upyun),
378            "yandex-disk" | "yandex_disk" => Ok(Scheme::YandexDisk),
379            "pcloud" => Ok(Scheme::Pcloud),
380            "sftp" => Ok(Scheme::Sftp),
381            "sled" => Ok(Scheme::Sled),
382            "swift" => Ok(Scheme::Swift),
383            "oss" => Ok(Scheme::Oss),
384            "vercel-artifacts" | "vercel_artifacts" => Ok(Scheme::VercelArtifacts),
385            "vercel-blob" | "vercel_blob" => Ok(Scheme::VercelBlob),
386            "webdav" => Ok(Scheme::Webdav),
387            "webhdfs" => Ok(Scheme::Webhdfs),
388            "tikv" => Ok(Scheme::Tikv),
389            "azfile" => Ok(Scheme::Azfile),
390            "mongodb" => Ok(Scheme::Mongodb),
391            "hdfs-native" | "hdfs_native" => Ok(Scheme::HdfsNative),
392            "surrealdb" => Ok(Scheme::Surrealdb),
393            "lakefs" => Ok(Scheme::Lakefs),
394            "nebula-graph" | "nebula_graph" => Ok(Scheme::NebulaGraph),
395            _ => Ok(Scheme::Custom(Box::leak(s.into_boxed_str()))),
396        }
397    }
398}
399
400impl From<Scheme> for &'static str {
401    fn from(v: Scheme) -> Self {
402        match v {
403            Scheme::AliyunDrive => "aliyun-drive",
404            Scheme::Atomicserver => "atomicserver",
405            Scheme::Azblob => "azblob",
406            Scheme::Azdls => "azdls",
407            Scheme::B2 => "b2",
408            Scheme::Cacache => "cacache",
409            Scheme::CloudflareKv => "cloudflare-kv",
410            Scheme::Cos => "cos",
411            Scheme::Compfs => "compfs",
412            Scheme::D1 => "d1",
413            Scheme::Dashmap => "dashmap",
414            Scheme::Etcd => "etcd",
415            Scheme::Dbfs => "dbfs",
416            Scheme::Fs => "fs",
417            Scheme::Gcs => "gcs",
418            Scheme::Ghac => "ghac",
419            Scheme::Gridfs => "gridfs",
420            Scheme::Hdfs => "hdfs",
421            Scheme::Http => "http",
422            Scheme::Huggingface => "huggingface",
423            Scheme::Foundationdb => "foundationdb",
424            Scheme::Ftp => "ftp",
425            Scheme::Ipfs => "ipfs",
426            Scheme::Ipmfs => "ipmfs",
427            Scheme::Icloud => "icloud",
428            Scheme::Koofr => "koofr",
429            Scheme::Memcached => "memcached",
430            Scheme::Memory => "memory",
431            Scheme::MiniMoka => "mini-moka",
432            Scheme::Moka => "moka",
433            Scheme::Monoiofs => "monoiofs",
434            Scheme::Obs => "obs",
435            Scheme::Onedrive => "onedrive",
436            Scheme::Persy => "persy",
437            Scheme::Postgresql => "postgresql",
438            Scheme::Mysql => "mysql",
439            Scheme::Gdrive => "gdrive",
440            Scheme::Github => "github",
441            Scheme::Dropbox => "dropbox",
442            Scheme::Redis => "redis",
443            Scheme::Rocksdb => "rocksdb",
444            Scheme::S3 => "s3",
445            Scheme::Seafile => "seafile",
446            Scheme::Sftp => "sftp",
447            Scheme::Sled => "sled",
448            Scheme::Swift => "swift",
449            Scheme::VercelArtifacts => "vercel-artifacts",
450            Scheme::VercelBlob => "vercel-blob",
451            Scheme::Oss => "oss",
452            Scheme::Webdav => "webdav",
453            Scheme::Webhdfs => "webhdfs",
454            Scheme::Redb => "redb",
455            Scheme::Tikv => "tikv",
456            Scheme::Azfile => "azfile",
457            Scheme::Sqlite => "sqlite",
458            Scheme::Mongodb => "mongodb",
459            Scheme::Alluxio => "alluxio",
460            Scheme::Upyun => "upyun",
461            Scheme::YandexDisk => "yandex-disk",
462            Scheme::Pcloud => "pcloud",
463            Scheme::HdfsNative => "hdfs-native",
464            Scheme::Surrealdb => "surrealdb",
465            Scheme::Lakefs => "lakefs",
466            Scheme::NebulaGraph => "nebula-graph",
467            Scheme::Custom(v) => v,
468        }
469    }
470}
471
472impl From<Scheme> for String {
473    fn from(v: Scheme) -> Self {
474        v.into_static().to_string()
475    }
476}
```
