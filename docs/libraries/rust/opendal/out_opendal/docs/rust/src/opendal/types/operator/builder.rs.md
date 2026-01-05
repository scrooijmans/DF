# 

opendal/types/operator/

builder.rs

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
19use std::sync::Arc;
20
21use crate::layers::*;
22use crate::raw::*;
23use crate::types::IntoOperatorUri;
24use crate::*;
25
26/// # Operator build API
27///
28/// Operator should be built via [`OperatorBuilder`]. We recommend to use [`Operator::new`] to get started:
29///
30/// ```
31/// # use anyhow::Result;
32/// use opendal::services::Fs;
33/// use opendal::Operator;
34/// async fn test() -> Result<()> {
35///     // Create fs backend builder.
36///     let builder = Fs::default().root("/tmp");
37///
38///     // Build an `Operator` to start operating the storage.
39///     let op: Operator = Operator::new(builder)?.finish();
40///
41///     Ok(())
42/// }
43/// ```
44impl Operator {
45    /// Create a new operator with input builder.
46    ///
47    /// OpenDAL will call `builder.build()` internally, so we don't need
48    /// to import `opendal::Builder` trait.
49    ///
50    /// # Examples
51    ///
52    /// Read more backend init examples in [examples](https://github.com/apache/opendal/tree/main/examples).
53    ///
54    /// ```
55    /// # use anyhow::Result;
56    /// use opendal::services::Fs;
57    /// use opendal::Operator;
58    /// async fn test() -> Result<()> {
59    ///     // Create fs backend builder.
60    ///     let builder = Fs::default().root("/tmp");
61    ///
62    ///     // Build an `Operator` to start operating the storage.
63    ///     let op: Operator = Operator::new(builder)?.finish();
64    ///
65    ///     Ok(())
66    /// }
67    /// ```
68    #[allow(clippy::new_ret_no_self)]
69    pub fn new<B: Builder>(ab: B) -> Result<OperatorBuilder<impl Access>> {
70        let acc = ab.build()?;
71        Ok(OperatorBuilder::new(acc))
72    }
73
74    /// Create a new operator from given config.
75    ///
76    /// # Examples
77    ///
78    /// ```
79    /// # use anyhow::Result;
80    /// use std::collections::HashMap;
81    ///
82    /// use opendal::services::MemoryConfig;
83    /// use opendal::Operator;
84    /// async fn test() -> Result<()> {
85    ///     let cfg = MemoryConfig::default();
86    ///
87    ///     // Build an `Operator` to start operating the storage.
88    ///     let op: Operator = Operator::from_config(cfg)?.finish();
89    ///
90    ///     Ok(())
91    /// }
92    /// ```
93    pub fn from_config<C: Configurator>(cfg: C) -> Result<OperatorBuilder<impl Access>> {
94        let builder = cfg.into_builder();
95        let acc = builder.build()?;
96        Ok(OperatorBuilder::new(acc))
97    }
98
99    /// Create a new operator from given iterator in static dispatch.
100    ///
101    /// # Notes
102    ///
103    /// `from_iter` generates a `OperatorBuilder` which allows adding layer in zero-cost way.
104    ///
105    /// # Examples
106    ///
107    /// ```
108    /// # use anyhow::Result;
109    /// use std::collections::HashMap;
110    ///
111    /// use opendal::services::Fs;
112    /// use opendal::Operator;
113    /// async fn test() -> Result<()> {
114    ///     let map = HashMap::from([
115    ///         // Set the root for fs, all operations will happen under this root.
116    ///         //
117    ///         // NOTE: the root must be absolute path.
118    ///         ("root".to_string(), "/tmp".to_string()),
119    ///     ]);
120    ///
121    ///     // Build an `Operator` to start operating the storage.
122    ///     let op: Operator = Operator::from_iter::<Fs>(map)?.finish();
123    ///
124    ///     Ok(())
125    /// }
126    /// ```
127    #[allow(clippy::should_implement_trait)]
128    pub fn from_iter<B: Builder>(
129        iter: impl IntoIterator<Item = (String, String)>,
130    ) -> Result<OperatorBuilder<impl Access>> {
131        let builder = B::Config::from_iter(iter)?.into_builder();
132        let acc = builder.build()?;
133        Ok(OperatorBuilder::new(acc))
134    }
135
136    /// Create a new operator by parsing configuration from a URI.
137    ///
138    /// # Examples
139    ///
140    /// ```
141    /// # use anyhow::Result;
142    /// use opendal::Operator;
143    ///
144    /// # fn example() -> Result<()> {
145    /// let op = Operator::from_uri("memory://localhost/")?;
146    /// # let _ = op;
147    /// # Ok(())
148    /// # }
149    /// ```
150    pub fn from_uri(uri: impl IntoOperatorUri) -> Result<Operator> {
151        crate::DEFAULT_OPERATOR_REGISTRY.load(uri)
152    }
153
154    /// Create a new operator via given scheme and iterator of config value in dynamic dispatch.
155    ///
156    /// # Notes
157    ///
158    /// `via_iter` generates a `Operator` which allows building operator without generic type.
159    ///
160    /// # Examples
161    ///
162    /// ```
163    /// # use anyhow::Result;
164    /// use std::collections::HashMap;
165    ///
166    /// use opendal::Operator;
167    /// use opendal::Scheme;
168    /// async fn test() -> Result<()> {
169    ///     let map = [
170    ///         // Set the root for fs, all operations will happen under this root.
171    ///         //
172    ///         // NOTE: the root must be absolute path.
173    ///         ("root".to_string(), "/tmp".to_string()),
174    ///     ];
175    ///
176    ///     // Build an `Operator` to start operating the storage.
177    ///     let op: Operator = Operator::via_iter(Scheme::Fs, map)?;
178    ///
179    ///     Ok(())
180    /// }
181    /// ```
182    #[allow(unused_variables, unreachable_code)]
183    pub fn via_iter(
184        scheme: Scheme,
185        iter: impl IntoIterator<Item = (String, String)>,
186    ) -> Result<Operator> {
187        let op = match scheme {
188            #[cfg(feature = "services-aliyun-drive")]
189            Scheme::AliyunDrive => Self::from_iter::<services::AliyunDrive>(iter)?.finish(),
190            #[cfg(feature = "services-alluxio")]
191            Scheme::Alluxio => Self::from_iter::<services::Alluxio>(iter)?.finish(),
192            #[cfg(feature = "services-cloudflare-kv")]
193            Scheme::CloudflareKv => Self::from_iter::<services::CloudflareKv>(iter)?.finish(),
194            #[cfg(feature = "services-compfs")]
195            Scheme::Compfs => Self::from_iter::<services::Compfs>(iter)?.finish(),
196            #[cfg(feature = "services-upyun")]
197            Scheme::Upyun => Self::from_iter::<services::Upyun>(iter)?.finish(),
198            #[cfg(feature = "services-koofr")]
199            Scheme::Koofr => Self::from_iter::<services::Koofr>(iter)?.finish(),
200            #[cfg(feature = "services-yandex-disk")]
201            Scheme::YandexDisk => Self::from_iter::<services::YandexDisk>(iter)?.finish(),
202            #[cfg(feature = "services-pcloud")]
203            Scheme::Pcloud => Self::from_iter::<services::Pcloud>(iter)?.finish(),
204            #[cfg(feature = "services-azblob")]
205            Scheme::Azblob => Self::from_iter::<services::Azblob>(iter)?.finish(),
206            #[cfg(feature = "services-azdls")]
207            Scheme::Azdls => Self::from_iter::<services::Azdls>(iter)?.finish(),
208            #[cfg(feature = "services-azfile")]
209            Scheme::Azfile => Self::from_iter::<services::Azfile>(iter)?.finish(),
210            #[cfg(feature = "services-b2")]
211            Scheme::B2 => Self::from_iter::<services::B2>(iter)?.finish(),
212            #[cfg(feature = "services-cacache")]
213            Scheme::Cacache => Self::from_iter::<services::Cacache>(iter)?.finish(),
214            #[cfg(feature = "services-cos")]
215            Scheme::Cos => Self::from_iter::<services::Cos>(iter)?.finish(),
216            #[cfg(feature = "services-d1")]
217            Scheme::D1 => Self::from_iter::<services::D1>(iter)?.finish(),
218            #[cfg(feature = "services-dashmap")]
219            Scheme::Dashmap => Self::from_iter::<services::Dashmap>(iter)?.finish(),
220            #[cfg(feature = "services-dropbox")]
221            Scheme::Dropbox => Self::from_iter::<services::Dropbox>(iter)?.finish(),
222            #[cfg(feature = "services-etcd")]
223            Scheme::Etcd => Self::from_iter::<services::Etcd>(iter)?.finish(),
224            #[cfg(feature = "services-foundationdb")]
225            Scheme::Foundationdb => Self::from_iter::<services::Foundationdb>(iter)?.finish(),
226            #[cfg(feature = "services-fs")]
227            Scheme::Fs => Self::from_iter::<services::Fs>(iter)?.finish(),
228            #[cfg(feature = "services-ftp")]
229            Scheme::Ftp => Self::from_iter::<services::Ftp>(iter)?.finish(),
230            #[cfg(feature = "services-gcs")]
231            Scheme::Gcs => Self::from_iter::<services::Gcs>(iter)?.finish(),
232            #[cfg(feature = "services-ghac")]
233            Scheme::Ghac => Self::from_iter::<services::Ghac>(iter)?.finish(),
234            #[cfg(feature = "services-gridfs")]
235            Scheme::Gridfs => Self::from_iter::<services::Gridfs>(iter)?.finish(),
236            #[cfg(feature = "services-github")]
237            Scheme::Github => Self::from_iter::<services::Github>(iter)?.finish(),
238            #[cfg(feature = "services-hdfs")]
239            Scheme::Hdfs => Self::from_iter::<services::Hdfs>(iter)?.finish(),
240            #[cfg(feature = "services-http")]
241            Scheme::Http => Self::from_iter::<services::Http>(iter)?.finish(),
242            #[cfg(feature = "services-huggingface")]
243            Scheme::Huggingface => Self::from_iter::<services::Huggingface>(iter)?.finish(),
244            #[cfg(feature = "services-ipfs")]
245            Scheme::Ipfs => Self::from_iter::<services::Ipfs>(iter)?.finish(),
246            #[cfg(feature = "services-ipmfs")]
247            Scheme::Ipmfs => Self::from_iter::<services::Ipmfs>(iter)?.finish(),
248            #[cfg(feature = "services-memcached")]
249            Scheme::Memcached => Self::from_iter::<services::Memcached>(iter)?.finish(),
250            #[cfg(feature = "services-memory")]
251            Scheme::Memory => Self::from_iter::<services::Memory>(iter)?.finish(),
252            #[cfg(feature = "services-mini-moka")]
253            Scheme::MiniMoka => Self::from_iter::<services::MiniMoka>(iter)?.finish(),
254            #[cfg(feature = "services-moka")]
255            Scheme::Moka => Self::from_iter::<services::Moka>(iter)?.finish(),
256            #[cfg(feature = "services-monoiofs")]
257            Scheme::Monoiofs => Self::from_iter::<services::Monoiofs>(iter)?.finish(),
258            #[cfg(feature = "services-mysql")]
259            Scheme::Mysql => Self::from_iter::<services::Mysql>(iter)?.finish(),
260            #[cfg(feature = "services-obs")]
261            Scheme::Obs => Self::from_iter::<services::Obs>(iter)?.finish(),
262            #[cfg(feature = "services-onedrive")]
263            Scheme::Onedrive => Self::from_iter::<services::Onedrive>(iter)?.finish(),
264            #[cfg(feature = "services-postgresql")]
265            Scheme::Postgresql => Self::from_iter::<services::Postgresql>(iter)?.finish(),
266            #[cfg(feature = "services-gdrive")]
267            Scheme::Gdrive => Self::from_iter::<services::Gdrive>(iter)?.finish(),
268            #[cfg(feature = "services-oss")]
269            Scheme::Oss => Self::from_iter::<services::Oss>(iter)?.finish(),
270            #[cfg(feature = "services-persy")]
271            Scheme::Persy => Self::from_iter::<services::Persy>(iter)?.finish(),
272            #[cfg(feature = "services-redis")]
273            Scheme::Redis => Self::from_iter::<services::Redis>(iter)?.finish(),
274            #[cfg(feature = "services-rocksdb")]
275            Scheme::Rocksdb => Self::from_iter::<services::Rocksdb>(iter)?.finish(),
276            #[cfg(feature = "services-s3")]
277            Scheme::S3 => Self::from_iter::<services::S3>(iter)?.finish(),
278            #[cfg(feature = "services-seafile")]
279            Scheme::Seafile => Self::from_iter::<services::Seafile>(iter)?.finish(),
280            #[cfg(feature = "services-sftp")]
281            Scheme::Sftp => Self::from_iter::<services::Sftp>(iter)?.finish(),
282            #[cfg(feature = "services-sled")]
283            Scheme::Sled => Self::from_iter::<services::Sled>(iter)?.finish(),
284            #[cfg(feature = "services-sqlite")]
285            Scheme::Sqlite => Self::from_iter::<services::Sqlite>(iter)?.finish(),
286            #[cfg(feature = "services-swift")]
287            Scheme::Swift => Self::from_iter::<services::Swift>(iter)?.finish(),
288            #[cfg(feature = "services-tikv")]
289            Scheme::Tikv => Self::from_iter::<services::Tikv>(iter)?.finish(),
290            #[cfg(feature = "services-vercel-artifacts")]
291            Scheme::VercelArtifacts => Self::from_iter::<services::VercelArtifacts>(iter)?.finish(),
292            #[cfg(feature = "services-vercel-blob")]
293            Scheme::VercelBlob => Self::from_iter::<services::VercelBlob>(iter)?.finish(),
294            #[cfg(feature = "services-webdav")]
295            Scheme::Webdav => Self::from_iter::<services::Webdav>(iter)?.finish(),
296            #[cfg(feature = "services-webhdfs")]
297            Scheme::Webhdfs => Self::from_iter::<services::Webhdfs>(iter)?.finish(),
298            #[cfg(feature = "services-redb")]
299            Scheme::Redb => Self::from_iter::<services::Redb>(iter)?.finish(),
300            #[cfg(feature = "services-mongodb")]
301            Scheme::Mongodb => Self::from_iter::<services::Mongodb>(iter)?.finish(),
302            #[cfg(feature = "services-hdfs-native")]
303            Scheme::HdfsNative => Self::from_iter::<services::HdfsNative>(iter)?.finish(),
304            #[cfg(feature = "services-lakefs")]
305            Scheme::Lakefs => Self::from_iter::<services::Lakefs>(iter)?.finish(),
306            v => {
307                return Err(Error::new(
308                    ErrorKind::Unsupported,
309                    "scheme is not enabled or supported",
310                )
311                .with_context("scheme", v));
312            }
313        };
314
315        Ok(op)
316    }
317
318    /// Create a new operator from given map.
319    ///
320    /// # Notes
321    ///
322    /// from_map is using static dispatch layers which is zero cost. via_map is
323    /// using dynamic dispatch layers which has a bit runtime overhead with an
324    /// extra vtable lookup and unable to inline. But from_map requires generic
325    /// type parameter which is not always easy to be used.
326    ///
327    /// # Examples
328    ///
329    /// ```
330    /// # use anyhow::Result;
331    /// use std::collections::HashMap;
332    ///
333    /// use opendal::services::Memory;
334    /// use opendal::Operator;
335    /// async fn test() -> Result<()> {
336    ///     let map = HashMap::new();
337    ///
338    ///     // Build an `Operator` to start operating the storage.
339    ///     let op: Operator = Operator::from_map::<Memory>(map)?.finish();
340    ///
341    ///     Ok(())
342    /// }
343    /// ```
344    #[deprecated = "use from_iter instead"]
345    pub fn from_map<B: Builder>(
346        map: HashMap<String, String>,
347    ) -> Result<OperatorBuilder<impl Access>> {
348        Self::from_iter::<B>(map)
349    }
350
351    /// Create a new operator from given scheme and map.
352    ///
353    /// # Notes
354    ///
355    /// from_map is using static dispatch layers which is zero cost. via_map is
356    /// using dynamic dispatch layers which has a bit runtime overhead with an
357    /// extra vtable lookup and unable to inline. But from_map requires generic
358    /// type parameter which is not always easy to be used.
359    ///
360    /// # Examples
361    ///
362    /// ```
363    /// # use anyhow::Result;
364    /// use std::collections::HashMap;
365    ///
366    /// use opendal::Operator;
367    /// use opendal::Scheme;
368    /// async fn test() -> Result<()> {
369    ///     let map = HashMap::new();
370    ///
371    ///     // Build an `Operator` to start operating the storage.
372    ///     let op: Operator = Operator::via_map(Scheme::Memory, map)?;
373    ///
374    ///     Ok(())
375    /// }
376    /// ```
377    #[deprecated = "use via_iter instead"]
378    pub fn via_map(scheme: Scheme, map: HashMap<String, String>) -> Result<Operator> {
379        Self::via_iter(scheme, map)
380    }
381
382    /// Create a new layer with dynamic dispatch.
383    ///
384    /// Please note that `Layer` can modify internal contexts such as `HttpClient`
385    /// and `Runtime` for the operator. Therefore, it is recommended to add layers
386    /// before interacting with the storage. Adding or duplicating layers after
387    /// accessing the storage may result in unexpected behavior.
388    ///
389    /// # Notes
390    ///
391    /// `OperatorBuilder::layer()` is using static dispatch which is zero
392    /// cost. `Operator::layer()` is using dynamic dispatch which has a
393    /// bit runtime overhead with an extra vtable lookup and unable to
394    /// inline.
395    ///
396    /// It's always recommended to use `OperatorBuilder::layer()` instead.
397    ///
398    /// # Examples
399    ///
400    /// ```no_run
401    /// # use std::sync::Arc;
402    /// # use anyhow::Result;
403    /// use opendal::layers::LoggingLayer;
404    /// use opendal::services::Memory;
405    /// use opendal::Operator;
406    ///
407    /// # async fn test() -> Result<()> {
408    /// let op = Operator::new(Memory::default())?.finish();
409    /// let op = op.layer(LoggingLayer::default());
410    /// // All operations will go through the new_layer
411    /// let _ = op.read("test_file").await?;
412    /// # Ok(())
413    /// # }
414    /// ```
415    #[must_use]
416    pub fn layer<L: Layer<Accessor>>(self, layer: L) -> Self {
417        Self::from_inner(Arc::new(
418            TypeEraseLayer.layer(layer.layer(self.into_inner())),
419        ))
420    }
421}
422
423/// OperatorBuilder is a typed builder to build an Operator.
424///
425/// # Notes
426///
427/// OpenDAL uses static dispatch internally and only performs dynamic
428/// dispatch at the outmost type erase layer. OperatorBuilder is the only
429/// public API provided by OpenDAL come with generic parameters.
430///
431/// It's required to call `finish` after the operator built.
432///
433/// # Examples
434///
435/// For users who want to support many services, we can build a helper function like the following:
436///
437/// ```
438/// use std::collections::HashMap;
439///
440/// use opendal::layers::LoggingLayer;
441/// use opendal::layers::RetryLayer;
442/// use opendal::services;
443/// use opendal::Builder;
444/// use opendal::Operator;
445/// use opendal::Result;
446/// use opendal::Scheme;
447///
448/// fn init_service<B: Builder>(cfg: HashMap<String, String>) -> Result<Operator> {
449///     let op = Operator::from_map::<B>(cfg)?
450///         .layer(LoggingLayer::default())
451///         .layer(RetryLayer::new())
452///         .finish();
453///
454///     Ok(op)
455/// }
456///
457/// async fn init(scheme: Scheme, cfg: HashMap<String, String>) -> Result<()> {
458///     let _ = match scheme {
459///         Scheme::Memory => init_service::<services::Memory>(cfg)?,
460///         _ => todo!(),
461///     };
462///
463///     Ok(())
464/// }
465/// ```
466pub struct OperatorBuilder<A: Access> {
467    accessor: A,
468}
469
470impl<A: Access> OperatorBuilder<A> {
471    /// Create a new operator builder.
472    #[allow(clippy::new_ret_no_self)]
473    pub fn new(accessor: A) -> OperatorBuilder<impl Access> {
474        // Make sure error context layer has been attached.
475        OperatorBuilder { accessor }
476            .layer(ErrorContextLayer)
477            .layer(CompleteLayer)
478            .layer(CorrectnessCheckLayer)
479    }
480
481    /// Create a new layer with static dispatch.
482    ///
483    /// # Notes
484    ///
485    /// `OperatorBuilder::layer()` is using static dispatch which is zero
486    /// cost. `Operator::layer()` is using dynamic dispatch which has a
487    /// bit runtime overhead with an extra vtable lookup and unable to
488    /// inline.
489    ///
490    /// It's always recommended to use `OperatorBuilder::layer()` instead.
491    ///
492    /// # Examples
493    ///
494    /// ```no_run
495    /// # use std::sync::Arc;
496    /// # use anyhow::Result;
497    /// use opendal::layers::LoggingLayer;
498    /// use opendal::services::Memory;
499    /// use opendal::Operator;
500    ///
501    /// # async fn test() -> Result<()> {
502    /// let op = Operator::new(Memory::default())?
503    ///     .layer(LoggingLayer::default())
504    ///     .finish();
505    /// // All operations will go through the new_layer
506    /// let _ = op.read("test_file").await?;
507    /// # Ok(())
508    /// # }
509    /// ```
510    #[must_use]
511    pub fn layer<L: Layer<A>>(self, layer: L) -> OperatorBuilder<L::LayeredAccess> {
512        OperatorBuilder {
513            accessor: layer.layer(self.accessor),
514        }
515    }
516
517    /// Finish the building to construct an Operator.
518    pub fn finish(self) -> Operator {
519        let ob = self.layer(TypeEraseLayer);
520        Operator::from_inner(Arc::new(ob.accessor) as Accessor)
521    }
522}
```
