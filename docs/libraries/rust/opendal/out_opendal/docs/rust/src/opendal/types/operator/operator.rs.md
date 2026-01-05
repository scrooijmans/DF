# 

opendal/types/operator/

operator.rs

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
18use std::future::Future;
19use std::time::Duration;
20
21use futures::Stream;
22use futures::StreamExt;
23use futures::TryStreamExt;
24
25use crate::operator_futures::*;
26use crate::raw::oio::DeleteDyn;
27use crate::raw::*;
28use crate::types::delete::Deleter;
29use crate::*;
30
31/// The `Operator` serves as the entry point for all public asynchronous APIs.
32///
33/// For more details about the `Operator`, refer to the [`concepts`][crate::docs::concepts] section.
34///
35/// All cloned `Operator` instances share the same internal state, such as
36/// `HttpClient` and `Runtime`. Some layers may modify the internal state of
37/// the `Operator` too like inject logging and metrics for `HttpClient`.
38///
39/// ## Build
40///
41/// Users can initialize an `Operator` through the following methods:
42///
43/// - [`Operator::new`]: Creates an operator using a [`services`] builder, such as [`services::S3`].
44/// - [`Operator::from_config`]: Creates an operator using a [`services`] configuration, such as [`services::S3Config`].
45/// - [`Operator::from_iter`]: Creates an operator from an iterator of configuration key-value pairs.
46///
47/// ```
48/// # use anyhow::Result;
49/// use opendal::services::Memory;
50/// use opendal::Operator;
51/// async fn test() -> Result<()> {
52///     // Build an `Operator` to start operating the storage.
53///     let _: Operator = Operator::new(Memory::default())?.finish();
54///
55///     Ok(())
56/// }
57/// ```
58///
59/// ## Layer
60///
61/// After the operator is built, users can add the layers they need on top of it.
62///
63/// OpenDAL offers various layers for users to choose from, such as `RetryLayer`, `LoggingLayer`, and more. Visit [`layers`] for further details.
64///
65/// Please note that `Layer` can modify internal contexts such as `HttpClient`
66/// and `Runtime` for all clones of given operator. Therefore, it is recommended
67/// to add layers before interacting with the storage. Adding or duplicating
68/// layers after accessing the storage may result in unexpected behavior.
69///
70/// ```
71/// # use anyhow::Result;
72/// use opendal::layers::RetryLayer;
73/// use opendal::services::Memory;
74/// use opendal::Operator;
75/// async fn test() -> Result<()> {
76///     let op: Operator = Operator::new(Memory::default())?.finish();
77///
78///     // OpenDAL will retry failed operations now.
79///     let op = op.layer(RetryLayer::default());
80///
81///     Ok(())
82/// }
83/// ```
84///
85/// ## Operate
86///
87/// After the operator is built and the layers are added, users can start operating the storage.
88///
89/// The operator is `Send`, `Sync`, and `Clone`. It has no internal state, and all APIs only take
90/// a `&self` reference, making it safe to share the operator across threads.
91///
92/// Operator provides a consistent API pattern for data operations. For reading operations, it exposes:
93///
94/// - [`Operator::read`]: Executes a read operation.
95/// - [`Operator::read_with`]: Executes a read operation with additional options using the builder pattern.
96/// - [`Operator::read_options`]: Executes a read operation with extra options provided via a [`options::ReadOptions`] struct.
97/// - [`Operator::reader`]: Creates a reader for streaming data, allowing for flexible access.
98/// - [`Operator::reader_with`]: Creates a reader with advanced options using the builder pattern.
99/// - [`Operator::reader_options`]: Creates a reader with extra options provided via a [`options::ReadOptions`] struct.
100///
101/// The [`Reader`] created by [`Operator`] supports custom read control methods and can be converted
102/// into [`futures::AsyncRead`] or [`futures::Stream`] for broader ecosystem compatibility.
103///
104/// ```no_run
105/// use opendal::layers::LoggingLayer;
106/// use opendal::options;
107/// use opendal::services;
108/// use opendal::Operator;
109/// use opendal::Result;
110///
111/// #[tokio::main]
112/// async fn main() -> Result<()> {
113///     // Pick a builder and configure it.
114///     let mut builder = services::S3::default().bucket("test");
115///
116///     // Init an operator
117///     let op = Operator::new(builder)?
118///         // Init with logging layer enabled.
119///         .layer(LoggingLayer::default())
120///         .finish();
121///
122///     // Fetch this file's metadata
123///     let meta = op.stat("hello.txt").await?;
124///     let length = meta.content_length();
125///
126///     // Read data from `hello.txt` with options.
127///     let bs = op
128///         .read_with("hello.txt")
129///         .range(0..8 * 1024 * 1024)
130///         .chunk(1024 * 1024)
131///         .concurrent(4)
132///         .await?;
133///
134///     // The same to:
135///     let bs = op
136///         .read_options("hello.txt", options::ReadOptions {
137///             range: (0..8 * 1024 * 1024).into(),
138///             chunk: Some(1024 * 1024),
139///             concurrent: 4,
140///             ..Default::default()
141///         })
142///         .await?;
143///
144///     Ok(())
145/// }
146/// ```
147#[derive(Clone, Debug)]
148pub struct Operator {
149    // accessor is what Operator delegates for
150    accessor: Accessor,
151}
152
153/// # Operator basic API.
154impl Operator {
155    /// Fetch the internal accessor.
156    pub fn inner(&self) -> &Accessor {
157        &self.accessor
158    }
159
160    /// Convert inner accessor into operator.
161    pub fn from_inner(accessor: Accessor) -> Self {
162        Self { accessor }
163    }
164
165    /// Convert operator into inner accessor.
166    pub fn into_inner(self) -> Accessor {
167        self.accessor
168    }
169
170    /// Get information of underlying accessor.
171    ///
172    /// # Examples
173    ///
174    /// ```
175    /// # use std::sync::Arc;
176    /// # use anyhow::Result;
177    /// use opendal::Operator;
178    ///
179    /// # async fn test(op: Operator) -> Result<()> {
180    /// let info = op.info();
181    /// # Ok(())
182    /// # }
183    /// ```
184    pub fn info(&self) -> OperatorInfo {
185        OperatorInfo::new(self.accessor.info())
186    }
187
188    /// Get the executor used by current operator.
189    pub fn executor(&self) -> Executor {
190        self.accessor.info().executor()
191    }
192
193    /// Update executor for the context.
194    ///
195    /// All cloned `Operator` instances share the same internal state, such as
196    /// `HttpClient` and `Runtime`. Some layers may modify the internal state of
197    /// the `Operator` too like inject logging and metrics for `HttpClient`.
198    ///
199    /// # Note
200    ///
201    /// Tasks must be forwarded to the old executor after the update. Otherwise, features such as retry, timeout, and metrics may not function properly.
202    pub fn update_executor(&self, f: impl FnOnce(Executor) -> Executor) {
203        self.accessor.info().update_executor(f);
204    }
205
206    /// Get the http client used by current operator.
207    #[deprecated(
208        since = "0.54.0",
209        note = "Use HttpClientLayer instead. This method will be removed in next version."
210    )]
211    pub fn http_client(&self) -> HttpClient {
212        self.accessor.info().http_client()
213    }
214
215    /// Update http client for the context.
216    ///
217    /// All cloned `Operator` instances share the same internal state, such as
218    /// `HttpClient` and `Runtime`. Some layers may modify the internal state of
219    /// the `Operator` too like inject logging and metrics for `HttpClient`.
220    ///
221    /// # Note
222    ///
223    /// Tasks must be forwarded to the old executor after the update. Otherwise, features such as retry, timeout, and metrics may not function properly.
224    ///
225    /// # Deprecated
226    ///
227    /// This method is deprecated since v0.54.0. Use [`HttpClientLayer`] instead.
228    ///
229    /// ## Migration Example
230    ///
231    /// Instead of:
232    /// ```ignore
233    /// let operator = Operator::new(service)?;
234    /// operator.update_http_client(|_| custom_client);
235    /// ```
236    ///
237    /// Use:
238    /// ```ignore
239    /// use opendal::layers::HttpClientLayer;
240    ///
241    /// let operator = Operator::new(service)?
242    ///     .layer(HttpClientLayer::new(custom_client))
243    ///     .finish();
244    /// ```
245    ///
246    /// [`HttpClientLayer`]: crate::layers::HttpClientLayer
247    #[deprecated(
248        since = "0.54.0",
249        note = "Use HttpClientLayer instead. This method will be removed in next version"
250    )]
251    pub fn update_http_client(&self, f: impl FnOnce(HttpClient) -> HttpClient) {
252        self.accessor.info().update_http_client(f);
253    }
254}
255
256/// # Operator async API.
257impl Operator {
258    /// Check if this operator can work correctly.
259    ///
260    /// We will send a `list` request to path and return any errors we met.
261    ///
262    /// ```
263    /// # use std::sync::Arc;
264    /// # use anyhow::Result;
265    /// use opendal::Operator;
266    ///
267    /// # async fn test(op: Operator) -> Result<()> {
268    /// op.check().await?;
269    /// # Ok(())
270    /// # }
271    /// ```
272    pub async fn check(&self) -> Result<()> {
273        let mut ds = self.lister_with("/").limit(1).await?;
274
275        match ds.next().await {
276            Some(Err(e)) if e.kind() != ErrorKind::NotFound => Err(e),
277            _ => Ok(()),
278        }
279    }
280
281    /// Retrieve the metadata for the specified path.
282    ///
283    /// # Notes
284    ///
285    /// ## Extra Options
286    ///
287    /// [`Operator::stat`] is a wrapper around [`Operator::stat_with`] that uses no additional options.
288    /// To specify extra options such as `if_match` and `if_none_match`, please use [`Operator::stat_with`] instead.
289    ///
290    /// # Examples
291    ///
292    /// ## Check if file exists
293    ///
294    /// ```
295    /// # use anyhow::Result;
296    /// # use futures::io;
297    /// # use opendal::Operator;
298    /// use opendal::ErrorKind;
299    /// #
300    /// # async fn test(op: Operator) -> Result<()> {
301    /// if let Err(e) = op.stat("test").await {
302    ///     if e.kind() == ErrorKind::NotFound {
303    ///         println!("file not exist")
304    ///     }
305    /// }
306    /// # Ok(())
307    /// # }
308    /// ```
309    pub async fn stat(&self, path: &str) -> Result<Metadata> {
310        self.stat_with(path).await
311    }
312
313    /// Retrieve the metadata of the specified path with additional options.
314    ///
315    /// # Options
316    ///
317    /// Check [`options::StatOptions`] for all available options.
318    ///
319    /// # Examples
320    ///
321    /// ## Get metadata while `ETag` matches
322    ///
323    /// `stat_with` will
324    ///
325    /// - return `Ok(metadata)` if `ETag` matches
326    /// - return `Err(error)` and `error.kind() == ErrorKind::ConditionNotMatch` if file exists but
327    ///   `ETag` mismatch
328    /// - return `Err(err)` if other errors occur, for example, `NotFound`.
329    ///
330    /// ```
331    /// # use anyhow::Result;
332    /// # use futures::io;
333    /// # use opendal::Operator;
334    /// use opendal::ErrorKind;
335    /// #
336    /// # async fn test(op: Operator) -> Result<()> {
337    /// if let Err(e) = op.stat_with("test").if_match("<etag>").await {
338    ///     if e.kind() == ErrorKind::ConditionNotMatch {
339    ///         println!("file exists, but etag mismatch")
340    ///     }
341    ///     if e.kind() == ErrorKind::NotFound {
342    ///         println!("file not exist")
343    ///     }
344    /// }
345    /// # Ok(())
346    /// # }
347    /// ```
348    pub fn stat_with(&self, path: &str) -> FutureStat<impl Future<Output = Result<Metadata>>> {
349        let path = normalize_path(path);
350        OperatorFuture::new(
351            self.inner().clone(),
352            path,
353            options::StatOptions::default(),
354            Self::stat_inner,
355        )
356    }
357
358    /// Retrieve the metadata of the specified path with additional options.
359    ///
360    /// # Examples
361    ///
362    /// ## Get metadata while `ETag` matches
363    ///
364    /// `stat_with` will
365    ///
366    /// - return `Ok(metadata)` if `ETag` matches
367    /// - return `Err(error)` and `error.kind() == ErrorKind::ConditionNotMatch` if file exists but
368    ///   `ETag` mismatch
369    /// - return `Err(err)` if other errors occur, for example, `NotFound`.
370    ///
371    /// ```
372    /// # use anyhow::Result;
373    /// # use futures::io;
374    /// # use opendal::Operator;
375    /// use opendal::options;
376    /// use opendal::ErrorKind;
377    /// #
378    /// # async fn test(op: Operator) -> Result<()> {
379    /// let res = op
380    ///     .stat_options("test", options::StatOptions {
381    ///         if_match: Some("<etag>".to_string()),
382    ///         ..Default::default()
383    ///     })
384    ///     .await;
385    /// if let Err(e) = res {
386    ///     if e.kind() == ErrorKind::ConditionNotMatch {
387    ///         println!("file exists, but etag mismatch")
388    ///     }
389    ///     if e.kind() == ErrorKind::NotFound {
390    ///         println!("file not exist")
391    ///     }
392    /// }
393    /// # Ok(())
394    /// # }
395    /// ```
396    pub async fn stat_options(&self, path: &str, opts: options::StatOptions) -> Result<Metadata> {
397        let path = normalize_path(path);
398        Self::stat_inner(self.accessor.clone(), path, opts).await
399    }
400
401    #[inline]
402    async fn stat_inner(
403        acc: Accessor,
404        path: String,
405        opts: options::StatOptions,
406    ) -> Result<Metadata> {
407        let rp = acc.stat(&path, opts.into()).await?;
408        Ok(rp.into_metadata())
409    }
410
411    /// Check whether this path exists.
412    ///
413    /// # Example
414    ///
415    /// ```
416    /// use anyhow::Result;
417    /// use futures::io;
418    /// use opendal::Operator;
419    ///
420    /// async fn test(op: Operator) -> Result<()> {
421    ///     let _ = op.exists("test").await?;
422    ///
423    ///     Ok(())
424    /// }
425    /// ```
426    pub async fn exists(&self, path: &str) -> Result<bool> {
427        let r = self.stat(path).await;
428        match r {
429            Ok(_) => Ok(true),
430            Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
431            Err(err) => Err(err),
432        }
433    }
434
435    /// Create a directory at the specified path.
436    ///
437    /// # Notes
438    ///
439    /// To specify that a path is a directory, you must include a trailing slash (/).
440    /// Omitting the trailing slash may cause OpenDAL to return a `NotADirectory` error.
441    ///
442    /// # Behavior
443    ///
444    /// - Creating a directory that already exists will succeed.
445    /// - Directory creation is always recursive, functioning like `mkdir -p`.
446    ///
447    /// # Examples
448    ///
449    /// ```
450    /// # use opendal::Result;
451    /// # use opendal::Operator;
452    /// # async fn test(op: Operator) -> Result<()> {
453    /// op.create_dir("path/to/dir/").await?;
454    /// # Ok(())
455    /// # }
456    /// ```
457    pub async fn create_dir(&self, path: &str) -> Result<()> {
458        let path = normalize_path(path);
459
460        if !validate_path(&path, EntryMode::DIR) {
461            return Err(Error::new(
462                ErrorKind::NotADirectory,
463                "the path trying to create should end with `/`",
464            )
465            .with_operation("create_dir")
466            .with_context("service", self.inner().info().scheme())
467            .with_context("path", &path));
468        }
469
470        self.inner().create_dir(&path, OpCreateDir::new()).await?;
471
472        Ok(())
473    }
474
475    /// Read the entire file into bytes from given path.
476    ///
477    /// # Notes
478    ///
479    /// ## Additional Options
480    ///
481    /// [`Operator::read`] is a simplified method that does not support additional options. To access features like `range` and `if_match`, please use [`Operator::read_with`] or [`Operator::read_options`] instead.
482    ///
483    /// ## Streaming Read
484    ///
485    /// This function reads all content into memory at once. For more precise memory management or to read big file lazily, please use [`Operator::reader`].
486    ///
487    /// # Examples
488    ///
489    /// ```
490    /// # use opendal::Result;
491    /// # use opendal::Operator;
492    /// # use futures::TryStreamExt;
493    /// # async fn test(op: Operator) -> Result<()> {
494    /// let bs = op.read("path/to/file").await?;
495    /// # Ok(())
496    /// # }
497    /// ```
498    pub async fn read(&self, path: &str) -> Result<Buffer> {
499        self.read_options(path, options::ReadOptions::default())
500            .await
501    }
502
503    /// Read the entire file into bytes from given path with additional options.
504    ///
505    /// # Notes
506    ///
507    /// ## Streaming Read
508    ///
509    /// This function reads all content into memory at once. For more precise memory management or to read big file lazily, please use [`Operator::reader`].
510    ///
511    /// # Options
512    ///
513    /// Visit [`options::ReadOptions`] for all available options.
514    ///
515    /// # Examples
516    ///
517    /// Read the first 10 bytes of a file:
518    ///
519    /// ```
520    /// # use opendal::Result;
521    /// # use opendal::Operator;
522    /// # async fn test(op: Operator) -> Result<()> {
523    /// let bs = op.read_with("path/to/file").range(0..10).await?;
524    /// # Ok(())
525    /// # }
526    /// ```
527    pub fn read_with(&self, path: &str) -> FutureRead<impl Future<Output = Result<Buffer>>> {
528        let path = normalize_path(path);
529
530        OperatorFuture::new(
531            self.inner().clone(),
532            path,
533            options::ReadOptions::default(),
534            Self::read_inner,
535        )
536    }
537
538    /// Read the entire file into bytes from given path with additional options.
539    ///
540    /// # Notes
541    ///
542    /// ## Streaming Read
543    ///
544    /// This function reads all content into memory at once. For more precise memory management or to read big file lazily, please use [`Operator::reader`].
545    ///
546    /// # Examples
547    ///
548    /// Read the first 10 bytes of a file:
549    ///
550    /// ```
551    /// # use opendal::Result;
552    /// # use opendal::Operator;
553    /// use opendal::options;
554    /// # async fn test(op: Operator) -> Result<()> {
555    /// let bs = op
556    ///     .read_options("path/to/file", options::ReadOptions {
557    ///         range: (0..10).into(),
558    ///         ..Default::default()
559    ///     })
560    ///     .await?;
561    /// # Ok(())
562    /// # }
563    /// ```
564    pub async fn read_options(&self, path: &str, opts: options::ReadOptions) -> Result<Buffer> {
565        let path = normalize_path(path);
566        Self::read_inner(self.inner().clone(), path, opts).await
567    }
568
569    #[inline]
570    async fn read_inner(acc: Accessor, path: String, opts: options::ReadOptions) -> Result<Buffer> {
571        if !validate_path(&path, EntryMode::FILE) {
572            return Err(
573                Error::new(ErrorKind::IsADirectory, "read path is a directory")
574                    .with_operation("read")
575                    .with_context("service", acc.info().scheme())
576                    .with_context("path", &path),
577            );
578        }
579
580        let (args, opts) = opts.into();
581        let range = args.range();
582        let context = ReadContext::new(acc, path, args, opts);
583        let r = Reader::new(context);
584        let buf = r.read(range.to_range()).await?;
585        Ok(buf)
586    }
587
588    /// Create a new reader of given path.
589    ///
590    /// # Notes
591    ///
592    /// ## Extra Options
593    ///
594    /// [`Operator::reader`] is a simplified method without any options. To use additional options such as `concurrent` or `if_match`, please use [`Operator::reader_with`] or [`Operator::reader_options`] instead.
595    ///
596    /// # Examples
597    ///
598    /// ```
599    /// # use opendal::Result;
600    /// # use opendal::Operator;
601    /// # use futures::TryStreamExt;
602    /// # use opendal::Scheme;
603    /// # async fn test(op: Operator) -> Result<()> {
604    /// let r = op.reader("path/to/file").await?;
605    /// // Read the first 10 bytes of the file
606    /// let data = r.read(0..10).await?;
607    /// # Ok(())
608    /// # }
609    /// ```
610    pub async fn reader(&self, path: &str) -> Result<Reader> {
611        self.reader_with(path).await
612    }
613
614    /// Create a new reader of given path with additional options.
615    ///
616    /// # Options
617    ///
618    /// Visit [`options::ReaderOptions`] for all available options.
619    ///
620    /// # Examples
621    ///
622    /// Create a reader with a specific version ID:
623    ///
624    /// ```
625    /// # use opendal::Result;
626    /// # use opendal::Operator;
627    /// # use opendal::Scheme;
628    /// # async fn test(op: Operator) -> Result<()> {
629    /// let r = op.reader_with("path/to/file").version("version_id").await?;
630    /// // Read the first 10 bytes of the file
631    /// let data = r.read(0..10).await?;
632    /// # Ok(())
633    /// # }
634    /// ```
635    pub fn reader_with(&self, path: &str) -> FutureReader<impl Future<Output = Result<Reader>>> {
636        let path = normalize_path(path);
637
638        OperatorFuture::new(
639            self.inner().clone(),
640            path,
641            options::ReaderOptions::default(),
642            Self::reader_inner,
643        )
644    }
645
646    /// Create a new reader of given path with additional options.
647    ///
648    /// # Examples
649    ///
650    /// Create a reader with a specific version ID:
651    ///
652    /// ```
653    /// # use opendal::Result;
654    /// # use opendal::Operator;
655    /// use opendal::options;
656    /// # async fn test(op: Operator) -> Result<()> {
657    /// let r = op
658    ///     .reader_options("path/to/file", options::ReaderOptions {
659    ///         version: Some("version_id".to_string()),
660    ///         ..Default::default()
661    ///     })
662    ///     .await?;
663    /// // Read the first 10 bytes of the file
664    /// let data = r.read(0..10).await?;
665    /// # Ok(())
666    /// # }
667    /// ```
668    pub async fn reader_options(&self, path: &str, opts: options::ReaderOptions) -> Result<Reader> {
669        let path = normalize_path(path);
670        Self::reader_inner(self.inner().clone(), path, opts).await
671    }
672
673    /// Allow this unused async since we don't want
674    /// to change our public API.
675    #[allow(clippy::unused_async)]
676    #[inline]
677    async fn reader_inner(
678        acc: Accessor,
679        path: String,
680        options: options::ReaderOptions,
681    ) -> Result<Reader> {
682        if !validate_path(&path, EntryMode::FILE) {
683            return Err(
684                Error::new(ErrorKind::IsADirectory, "read path is a directory")
685                    .with_operation("Operator::reader")
686                    .with_context("service", acc.info().scheme())
687                    .with_context("path", path),
688            );
689        }
690
691        let (args, opts) = options.into();
692        let context = ReadContext::new(acc, path, args, opts);
693        Ok(Reader::new(context))
694    }
695
696    /// Write all data to the specified path at once.
697    ///
698    /// # Notes
699    ///
700    /// Visit [`performance::concurrent_write`][crate::docs::performance::concurrent_write] for more details on concurrent writes.
701    ///
702    /// ## Extra Options
703    ///
704    /// [`Operator::write`] is a simplified method that does not include additional options.
705    /// For advanced features such as `chunk` and `concurrent`, use [`Operator::write_with`] or [`Operator::write_options`] instead.
706    ///
707    /// ## Streaming Write
708    ///
709    /// This method executes a single bulk write operation. For more precise memory management
710    /// or to write data in a streaming fashion, use [`Operator::writer`] instead.
711    ///
712    /// ## Multipart Uploads
713    ///
714    /// OpenDAL offers multipart upload capabilities through the [`Writer`] abstraction,
715    /// automatically managing all upload details for you. You can fine-tune the upload process
716    /// by adjusting the `chunk` size and the number of `concurrent` operations using [`Operator::writer_with`].
717    ///
718    /// # Examples
719    ///
720    /// ```
721    /// # use opendal::Result;
722    /// # use opendal::Operator;
723    /// # use futures::StreamExt;
724    /// # use futures::SinkExt;
725    /// use bytes::Bytes;
726    ///
727    /// # async fn test(op: Operator) -> Result<()> {
728    /// op.write("path/to/file", vec![0; 4096]).await?;
729    /// # Ok(())
730    /// # }
731    /// ```
732    pub async fn write(&self, path: &str, bs: impl Into<Buffer>) -> Result<Metadata> {
733        let bs = bs.into();
734        self.write_with(path, bs).await
735    }
736
737    /// Write all data to the specified path at once with additional options.
738    ///
739    /// # Notes
740    ///
741    /// Visit [`performance::concurrent_write`][crate::docs::performance::concurrent_write] for more details on concurrent writes.
742    ///
743    /// ## Streaming Write
744    ///
745    /// This method executes a single bulk write operation. For more precise memory management
746    /// or to write data in a streaming fashion, use [`Operator::writer`] instead.
747    ///
748    /// ## Multipart Uploads
749    ///
750    /// OpenDAL offers multipart upload capabilities through the [`Writer`] abstraction,
751    /// automatically managing all upload details for you. You can fine-tune the upload process
752    /// by adjusting the `chunk` size and the number of `concurrent` operations using [`Operator::writer_with`].
753    ///
754    /// # Options
755    ///
756    /// Visit [`options::WriteOptions`] for all available options.
757    ///
758    /// # Examples
759    ///
760    /// Write data to a file only when it does not already exist:
761    ///
762    /// ```
763    /// # use opendal::Result;
764    /// # use opendal::Operator;
765    /// use bytes::Bytes;
766    ///
767    /// # async fn test(op: Operator) -> Result<()> {
768    /// let _ = op
769    ///     .write_with("path/to/file", vec![0; 4096])
770    ///     .if_not_exists(true)
771    ///     .await?;
772    /// # Ok(())
773    /// # }
774    /// ```
775    pub fn write_with(
776        &self,
777        path: &str,
778        bs: impl Into<Buffer>,
779    ) -> FutureWrite<impl Future<Output = Result<Metadata>>> {
780        let path = normalize_path(path);
781        let bs = bs.into();
782
783        OperatorFuture::new(
784            self.inner().clone(),
785            path,
786            (options::WriteOptions::default(), bs),
787            |inner, path, (opts, bs)| Self::write_inner(inner, path, bs, opts),
788        )
789    }
790
791    /// Write all data to the specified path at once with additional options.
792    ///
793    /// # Notes
794    ///
795    /// Visit [`performance::concurrent_write`][crate::docs::performance::concurrent_write] for more details on concurrent writes.
796    ///
797    /// ## Streaming Write
798    ///
799    /// This method executes a single bulk write operation. For more precise memory management
800    /// or to write data in a streaming fashion, use [`Operator::writer`] instead.
801    ///
802    /// ## Multipart Uploads
803    ///
804    /// OpenDAL offers multipart upload capabilities through the [`Writer`] abstraction,
805    /// automatically managing all upload details for you. You can fine-tune the upload process
806    /// by adjusting the `chunk` size and the number of `concurrent` operations using [`Operator::writer_with`].
807    ///
808    /// # Examples
809    ///
810    /// Write data to a file only when it does not already exist:
811    ///
812    /// ```
813    /// # use opendal::Result;
814    /// # use opendal::Operator;
815    /// use opendal::options;
816    ///
817    /// # async fn test(op: Operator) -> Result<()> {
818    /// let _ = op
819    ///     .write_options("path/to/file", vec![0; 4096], options::WriteOptions {
820    ///         if_not_exists: true,
821    ///         ..Default::default()
822    ///     })
823    ///     .await?;
824    /// # Ok(())
825    /// # }
826    /// ```
827    pub async fn write_options(
828        &self,
829        path: &str,
830        bs: impl Into<Buffer>,
831        opts: options::WriteOptions,
832    ) -> Result<Metadata> {
833        let path = normalize_path(path);
834        Self::write_inner(self.inner().clone(), path, bs.into(), opts).await
835    }
836
837    #[inline]
838    async fn write_inner(
839        acc: Accessor,
840        path: String,
841        bs: Buffer,
842        opts: options::WriteOptions,
843    ) -> Result<Metadata> {
844        if !validate_path(&path, EntryMode::FILE) {
845            return Err(
846                Error::new(ErrorKind::IsADirectory, "write path is a directory")
847                    .with_operation("Operator::write")
848                    .with_context("service", acc.info().scheme())
849                    .with_context("path", &path),
850            );
851        }
852
853        let (args, opts) = opts.into();
854        let context = WriteContext::new(acc, path, args, opts);
855        let mut w = Writer::new(context).await?;
856        w.write(bs).await?;
857        w.close().await
858    }
859
860    /// Create a new writer of given path.
861    ///
862    /// # Notes
863    ///
864    /// ## Writer Features
865    ///
866    /// The writer provides several powerful capabilities:
867    /// - Streaming writes for continuous data transfer
868    /// - Automatic multipart upload handling
869    /// - Memory-efficient chunk-based writing
870    ///
871    /// ## Extra Options
872    ///
873    /// [`Operator::writer`] is a simplified version that does not include additional options.
874    /// For advanced features such as `chunk` and `concurrent`, use [`Operator::writer_with`] or [`Operator::writer_options`] instead.
875    ///
876    /// # Examples
877    ///
878    /// ```
879    /// # use opendal::Result;
880    /// # use opendal::Operator;
881    /// use bytes::Bytes;
882    ///
883    /// # async fn test(op: Operator) -> Result<()> {
884    /// let mut w = op.writer("path/to/file").await?;
885    /// w.write(vec![0; 4096]).await?;
886    /// w.write(vec![1; 4096]).await?;
887    /// w.close().await?;
888    /// # Ok(())
889    /// # }
890    /// ```
891    pub async fn writer(&self, path: &str) -> Result<Writer> {
892        self.writer_with(path).await
893    }
894
895    /// Create a new writer of given path with additional options.
896    ///
897    /// # Notes
898    ///
899    /// ## Writer Features
900    ///
901    /// The writer provides several powerful capabilities:
902    /// - Streaming writes for continuous data transfer
903    /// - Automatic multipart upload handling
904    /// - Memory-efficient chunk-based writing
905    ///
906    /// ## Chunk Size Handling
907    ///
908    /// Storage services often have specific requirements for chunk sizes:
909    /// - Services like `s3` may return `EntityTooSmall` errors for undersized chunks
910    /// - Using small chunks in cloud storage services can lead to increased costs
911    ///
912    /// OpenDAL automatically determines optimal chunk sizes based on the service's
913    /// [Capability](crate::types::Capability). However, you can override this by explicitly
914    /// setting the `chunk` parameter.
915    ///
916    /// Visit [`performance::concurrent_write`][crate::docs::performance::concurrent_write] for more details on concurrent writes.
917    ///
918    /// # Examples
919    ///
920    /// ```
921    /// # use opendal::Result;
922    /// # use opendal::Operator;
923    /// use bytes::Bytes;
924    ///
925    /// # async fn test(op: Operator) -> Result<()> {
926    /// let mut w = op
927    ///     .writer_with("path/to/file")
928    ///     .chunk(4 * 1024 * 1024)
929    ///     .concurrent(8)
930    ///     .await?;
931    /// w.write(vec![0; 4096]).await?;
932    /// w.write(vec![1; 4096]).await?;
933    /// w.close().await?;
934    /// # Ok(())
935    /// # }
936    /// ```
937    pub fn writer_with(&self, path: &str) -> FutureWriter<impl Future<Output = Result<Writer>>> {
938        let path = normalize_path(path);
939
940        OperatorFuture::new(
941            self.inner().clone(),
942            path,
943            options::WriteOptions::default(),
944            Self::writer_inner,
945        )
946    }
947
948    /// Create a new writer of given path with additional options.
949    ///
950    /// # Notes
951    ///
952    /// ## Writer Features
953    ///
954    /// The writer provides several powerful capabilities:
955    /// - Streaming writes for continuous data transfer
956    /// - Automatic multipart upload handling
957    /// - Memory-efficient chunk-based writing
958    ///
959    /// ## Chunk Size Handling
960    ///
961    /// Storage services often have specific requirements for chunk sizes:
962    /// - Services like `s3` may return `EntityTooSmall` errors for undersized chunks
963    /// - Using small chunks in cloud storage services can lead to increased costs
964    ///
965    /// OpenDAL automatically determines optimal chunk sizes based on the service's
966    /// [Capability](crate::types::Capability). However, you can override this by explicitly
967    /// setting the `chunk` parameter.
968    ///
969    /// Visit [`performance::concurrent_write`][crate::docs::performance::concurrent_write] for more details on concurrent writes.
970    ///
971    /// # Examples
972    ///
973    /// Write data to a file in 4MiB chunk size and at 8 concurrency:
974    ///
975    /// ```
976    /// # use opendal::Result;
977    /// # use opendal::Operator;
978    /// use bytes::Bytes;
979    ///
980    /// # async fn test(op: Operator) -> Result<()> {
981    /// let mut w = op
982    ///     .writer_with("path/to/file")
983    ///     .chunk(4 * 1024 * 1024)
984    ///     .concurrent(8)
985    ///     .await?;
986    /// w.write(vec![0; 4096]).await?;
987    /// w.write(vec![1; 4096]).await?;
988    /// w.close().await?;
989    /// # Ok(())
990    /// # }
991    /// ```
992    pub async fn writer_options(&self, path: &str, opts: options::WriteOptions) -> Result<Writer> {
993        let path = normalize_path(path);
994        Self::writer_inner(self.inner().clone(), path, opts).await
995    }
996
997    #[inline]
998    async fn writer_inner(
999        acc: Accessor,
1000        path: String,
1001        opts: options::WriteOptions,
1002    ) -> Result<Writer> {
1003        if !validate_path(&path, EntryMode::FILE) {
1004            return Err(
1005                Error::new(ErrorKind::IsADirectory, "write path is a directory")
1006                    .with_operation("Operator::writer")
1007                    .with_context("service", acc.info().scheme())
1008                    .with_context("path", &path),
1009            );
1010        }
1011
1012        let (args, opts) = opts.into();
1013        let context = WriteContext::new(acc, path, args, opts);
1014        let w = Writer::new(context).await?;
1015        Ok(w)
1016    }
1017
1018    /// Copy a file from `from` to `to`.
1019    ///
1020    /// # Notes
1021    ///
1022    /// - `from` and `to` must be a file.
1023    /// - `to` will be overwritten if it exists.
1024    /// - If `from` and `to` are the same,  an `IsSameFile` error will occur.
1025    /// - `copy` is idempotent. For same `from` and `to` input, the result will be the same.
1026    ///
1027    /// # Examples
1028    ///
1029    /// ```
1030    /// # use opendal::Result;
1031    /// # use opendal::Operator;
1032    ///
1033    /// # async fn test(op: Operator) -> Result<()> {
1034    /// op.copy("path/to/file", "path/to/file2").await?;
1035    /// # Ok(())
1036    /// # }
1037    /// ```
1038    pub async fn copy(&self, from: &str, to: &str) -> Result<()> {
1039        let from = normalize_path(from);
1040
1041        if !validate_path(&from, EntryMode::FILE) {
1042            return Err(
1043                Error::new(ErrorKind::IsADirectory, "from path is a directory")
1044                    .with_operation("Operator::copy")
1045                    .with_context("service", self.info().scheme())
1046                    .with_context("from", from),
1047            );
1048        }
1049
1050        let to = normalize_path(to);
1051
1052        if !validate_path(&to, EntryMode::FILE) {
1053            return Err(
1054                Error::new(ErrorKind::IsADirectory, "to path is a directory")
1055                    .with_operation("Operator::copy")
1056                    .with_context("service", self.info().scheme())
1057                    .with_context("to", to),
1058            );
1059        }
1060
1061        if from == to {
1062            return Err(
1063                Error::new(ErrorKind::IsSameFile, "from and to paths are same")
1064                    .with_operation("Operator::copy")
1065                    .with_context("service", self.info().scheme())
1066                    .with_context("from", from)
1067                    .with_context("to", to),
1068            );
1069        }
1070
1071        self.inner().copy(&from, &to, OpCopy::new()).await?;
1072
1073        Ok(())
1074    }
1075
1076    /// Copy a file from `from` to `to` with additional options.
1077    ///
1078    /// # Notes
1079    ///
1080    /// - `from` and `to` must be a file.
1081    /// - If `from` and `to` are the same, an `IsSameFile` error will occur.
1082    /// - `copy` is idempotent. For same `from` and `to` input, the result will be the same.
1083    ///
1084    /// # Options
1085    ///
1086    /// Visit [`options::CopyOptions`] for all available options.
1087    ///
1088    /// # Examples
1089    ///
1090    /// Copy a file only if the destination doesn't exist:
1091    ///
1092    /// ```
1093    /// # use opendal::Result;
1094    /// # use opendal::Operator;
1095    ///
1096    /// # async fn test(op: Operator) -> Result<()> {
1097    /// op.copy_with("path/to/file", "path/to/file2")
1098    ///     .if_not_exists(true)
1099    ///     .await?;
1100    /// # Ok(())
1101    /// # }
1102    /// ```
1103    pub fn copy_with(&self, from: &str, to: &str) -> FutureCopy<impl Future<Output = Result<()>>> {
1104        let from = normalize_path(from);
1105        let to = normalize_path(to);
1106
1107        OperatorFuture::new(
1108            self.inner().clone(),
1109            from,
1110            (options::CopyOptions::default(), to),
1111            Self::copy_inner,
1112        )
1113    }
1114
1115    /// Copy a file from `from` to `to` with additional options.
1116    ///
1117    /// # Notes
1118    ///
1119    /// - `from` and `to` must be a file.
1120    /// - If `from` and `to` are the same, an `IsSameFile` error will occur.
1121    /// - `copy` is idempotent. For same `from` and `to` input, the result will be the same.
1122    ///
1123    /// # Options
1124    ///
1125    /// Check [`options::CopyOptions`] for all available options.
1126    ///
1127    /// # Examples
1128    ///
1129    /// Copy a file only if the destination doesn't exist:
1130    ///
1131    /// ```
1132    /// # use opendal::Result;
1133    /// # use opendal::Operator;
1134    /// # use opendal::options::CopyOptions;
1135    ///
1136    /// # async fn test(op: Operator) -> Result<()> {
1137    /// let mut opts = CopyOptions::default();
1138    /// opts.if_not_exists = true;
1139    /// op.copy_options("path/to/file", "path/to/file2", opts).await?;
1140    /// # Ok(())
1141    /// # }
1142    /// ```
1143    pub async fn copy_options(
1144        &self,
1145        from: &str,
1146        to: &str,
1147        opts: impl Into<options::CopyOptions>,
1148    ) -> Result<()> {
1149        let from = normalize_path(from);
1150        let to = normalize_path(to);
1151        let opts = opts.into();
1152
1153        Self::copy_inner(self.inner().clone(), from, (opts, to)).await
1154    }
1155
1156    async fn copy_inner(
1157        acc: Accessor,
1158        from: String,
1159        (opts, to): (options::CopyOptions, String),
1160    ) -> Result<()> {
1161        if !validate_path(&from, EntryMode::FILE) {
1162            return Err(
1163                Error::new(ErrorKind::IsADirectory, "from path is a directory")
1164                    .with_operation("Operator::copy")
1165                    .with_context("service", acc.info().scheme())
1166                    .with_context("from", from),
1167            );
1168        }
1169
1170        if !validate_path(&to, EntryMode::FILE) {
1171            return Err(
1172                Error::new(ErrorKind::IsADirectory, "to path is a directory")
1173                    .with_operation("Operator::copy")
1174                    .with_context("service", acc.info().scheme())
1175                    .with_context("to", to),
1176            );
1177        }
1178
1179        if from == to {
1180            return Err(
1181                Error::new(ErrorKind::IsSameFile, "from and to paths are same")
1182                    .with_operation("Operator::copy")
1183                    .with_context("service", acc.info().scheme())
1184                    .with_context("from", &from)
1185                    .with_context("to", &to),
1186            );
1187        }
1188
1189        let mut op = OpCopy::new();
1190        if opts.if_not_exists {
1191            op = op.with_if_not_exists(true);
1192        }
1193
1194        acc.copy(&from, &to, op).await.map(|_| ())
1195    }
1196
1197    /// Rename a file from `from` to `to`.
1198    ///
1199    /// # Notes
1200    ///
1201    /// - `from` and `to` must be a file.
1202    /// - `to` will be overwritten if it exists.
1203    /// - If `from` and `to` are the same, an `IsSameFile` error will occur.
1204    ///
1205    /// # Examples
1206    ///
1207    /// ```
1208    /// # use opendal::Result;
1209    /// # use opendal::Operator;
1210    ///
1211    /// # async fn test(op: Operator) -> Result<()> {
1212    /// op.rename("path/to/file", "path/to/file2").await?;
1213    /// # Ok(())
1214    /// # }
1215    /// ```
1216    pub async fn rename(&self, from: &str, to: &str) -> Result<()> {
1217        let from = normalize_path(from);
1218
1219        if !validate_path(&from, EntryMode::FILE) {
1220            return Err(
1221                Error::new(ErrorKind::IsADirectory, "from path is a directory")
1222                    .with_operation("Operator::move_")
1223                    .with_context("service", self.info().scheme())
1224                    .with_context("from", from),
1225            );
1226        }
1227
1228        let to = normalize_path(to);
1229
1230        if !validate_path(&to, EntryMode::FILE) {
1231            return Err(
1232                Error::new(ErrorKind::IsADirectory, "to path is a directory")
1233                    .with_operation("Operator::move_")
1234                    .with_context("service", self.info().scheme())
1235                    .with_context("to", to),
1236            );
1237        }
1238
1239        if from == to {
1240            return Err(
1241                Error::new(ErrorKind::IsSameFile, "from and to paths are same")
1242                    .with_operation("Operator::move_")
1243                    .with_context("service", self.info().scheme())
1244                    .with_context("from", from)
1245                    .with_context("to", to),
1246            );
1247        }
1248
1249        self.inner().rename(&from, &to, OpRename::new()).await?;
1250
1251        Ok(())
1252    }
1253
1254    /// Delete the given path.
1255    ///
1256    /// # Notes
1257    ///
1258    /// - Deleting a file that does not exist won't return errors.
1259    ///
1260    /// # Examples
1261    ///
1262    /// ```
1263    /// # use anyhow::Result;
1264    /// # use futures::io;
1265    /// # use opendal::Operator;
1266    /// # async fn test(op: Operator) -> Result<()> {
1267    /// op.delete("test").await?;
1268    /// # Ok(())
1269    /// # }
1270    /// ```
1271    pub async fn delete(&self, path: &str) -> Result<()> {
1272        self.delete_with(path).await
1273    }
1274
1275    /// Delete the given path with additional options.
1276    ///
1277    /// # Notes
1278    ///
1279    /// - Deleting a file that does not exist won't return errors.
1280    ///
1281    /// # Options
1282    ///
1283    /// Visit [`options::DeleteOptions`] for all available options.
1284    ///
1285    /// # Examples
1286    ///
1287    /// Delete a specific version of a file:
1288    ///
1289    /// ```
1290    /// # use opendal::Result;
1291    /// # use opendal::Operator;
1292    ///
1293    /// # async fn test(op: Operator, version: &str) -> Result<()> {
1294    /// op.delete_with("path/to/file").version(version).await?;
1295    /// # Ok(())
1296    /// # }
1297    /// ```
1298    pub fn delete_with(&self, path: &str) -> FutureDelete<impl Future<Output = Result<()>>> {
1299        let path = normalize_path(path);
1300
1301        OperatorFuture::new(
1302            self.inner().clone(),
1303            path,
1304            options::DeleteOptions::default(),
1305            Self::delete_inner,
1306        )
1307    }
1308
1309    /// Delete the given path with additional options.
1310    ///
1311    /// # Notes
1312    ///
1313    /// - Deleting a file that does not exist won't return errors.
1314    ///
1315    /// # Examples
1316    ///
1317    /// Delete a specific version of a file:
1318    ///
1319    /// ```
1320    /// # use opendal::Result;
1321    /// # use opendal::Operator;
1322    /// use opendal::options;
1323    ///
1324    /// # async fn test(op: Operator, version: &str) -> Result<()> {
1325    /// op.delete_options("path/to/file", options::DeleteOptions {
1326    ///     version: Some(version.to_string()),
1327    ///     ..Default::default()
1328    /// })
1329    /// .await?;
1330    /// # Ok(())
1331    /// # }
1332    /// ```
1333    pub async fn delete_options(&self, path: &str, opts: options::DeleteOptions) -> Result<()> {
1334        let path = normalize_path(path);
1335        Self::delete_inner(self.inner().clone(), path, opts).await
1336    }
1337
1338    async fn delete_inner(acc: Accessor, path: String, opts: options::DeleteOptions) -> Result<()> {
1339        let (_, mut deleter) = acc.delete_dyn().await?;
1340        let args = opts.into();
1341        deleter.delete_dyn(&path, args)?;
1342        deleter.flush_dyn().await?;
1343        Ok(())
1344    }
1345
1346    /// Delete an infallible iterator of paths.
1347    ///
1348    /// Also see:
1349    ///
1350    /// - [`Operator::delete_try_iter`]: delete a fallible iterator of paths.
1351    /// - [`Operator::delete_stream`]: delete an infallible stream of paths.
1352    /// - [`Operator::delete_try_stream`]: delete a fallible stream of paths.
1353    pub async fn delete_iter<I, D>(&self, iter: I) -> Result<()>
1354    where
1355        I: IntoIterator<Item = D>,
1356        D: IntoDeleteInput,
1357    {
1358        let mut deleter = self.deleter().await?;
1359        deleter.delete_iter(iter).await?;
1360        deleter.close().await?;
1361        Ok(())
1362    }
1363
1364    /// Delete a fallible iterator of paths.
1365    ///
1366    /// Also see:
1367    ///
1368    /// - [`Operator::delete_iter`]: delete an infallible iterator of paths.
1369    /// - [`Operator::delete_stream`]: delete an infallible stream of paths.
1370    /// - [`Operator::delete_try_stream`]: delete a fallible stream of paths.
1371    pub async fn delete_try_iter<I, D>(&self, try_iter: I) -> Result<()>
1372    where
1373        I: IntoIterator<Item = Result<D>>,
1374        D: IntoDeleteInput,
1375    {
1376        let mut deleter = self.deleter().await?;
1377        deleter.delete_try_iter(try_iter).await?;
1378        deleter.close().await?;
1379        Ok(())
1380    }
1381
1382    /// Delete an infallible stream of paths.
1383    ///
1384    /// Also see:
1385    ///
1386    /// - [`Operator::delete_iter`]: delete an infallible iterator of paths.
1387    /// - [`Operator::delete_try_iter`]: delete a fallible iterator of paths.
1388    /// - [`Operator::delete_try_stream`]: delete a fallible stream of paths.
1389    pub async fn delete_stream<S, D>(&self, stream: S) -> Result<()>
1390    where
1391        S: Stream<Item = D>,
1392        D: IntoDeleteInput,
1393    {
1394        let mut deleter = self.deleter().await?;
1395        deleter.delete_stream(stream).await?;
1396        deleter.close().await?;
1397        Ok(())
1398    }
1399
1400    /// Delete a fallible stream of paths.
1401    ///
1402    /// Also see:
1403    ///
1404    /// - [`Operator::delete_iter`]: delete an infallible iterator of paths.
1405    /// - [`Operator::delete_try_iter`]: delete a fallible iterator of paths.
1406    /// - [`Operator::delete_stream`]: delete an infallible stream of paths.
1407    pub async fn delete_try_stream<S, D>(&self, try_stream: S) -> Result<()>
1408    where
1409        S: Stream<Item = Result<D>>,
1410        D: IntoDeleteInput,
1411    {
1412        let mut deleter = self.deleter().await?;
1413        deleter.delete_try_stream(try_stream).await?;
1414        deleter.close().await?;
1415        Ok(())
1416    }
1417
1418    /// Create a [`Deleter`] to continuously remove content from storage.
1419    ///
1420    /// It leverages batch deletion capabilities provided by storage services for efficient removal.
1421    ///
1422    /// Users can have more control over the deletion process by using [`Deleter`] directly.
1423    pub async fn deleter(&self) -> Result<Deleter> {
1424        Deleter::create(self.inner().clone()).await
1425    }
1426
1427    /// Remove the path and all nested dirs and files recursively.
1428    ///
1429    /// # Notes
1430    ///
1431    /// If underlying services support delete in batch, we will use batch
1432    /// delete instead.
1433    ///
1434    /// # Examples
1435    ///
1436    /// ```
1437    /// # use anyhow::Result;
1438    /// # use futures::io;
1439    /// # use opendal::Operator;
1440    /// #
1441    /// # async fn test(op: Operator) -> Result<()> {
1442    /// op.remove_all("path/to/dir").await?;
1443    /// # Ok(())
1444    /// # }
1445    /// ```
1446    pub async fn remove_all(&self, path: &str) -> Result<()> {
1447        match self.stat(path).await {
1448            // If object exists.
1449            Ok(metadata) => {
1450                // If the object is a file, we can delete it.
1451                if metadata.mode() != EntryMode::DIR {
1452                    self.delete(path).await?;
1453                    // There may still be objects prefixed with the path in some backend, so we can't return here.
1454                }
1455            }
1456
1457            // If dir not found, it may be a prefix in object store like S3,
1458            // and we still need to delete objects under the prefix.
1459            Err(e) if e.kind() == ErrorKind::NotFound => {}
1460
1461            // Pass on any other error.
1462            Err(e) => return Err(e),
1463        };
1464
1465        let lister = self.lister_with(path).recursive(true).await?;
1466        self.delete_try_stream(lister).await?;
1467        Ok(())
1468    }
1469
1470    /// List entries in the parent directory that start with the specified `path`.
1471    ///
1472    /// # Notes
1473    ///
1474    /// ## Recursively List
1475    ///
1476    /// This function only reads the immediate children of the specified directory.
1477    /// To list all entries recursively, use `Operator::list_with("path").recursive(true)` instead.
1478    ///
1479    /// ## Streaming List
1480    ///
1481    /// This function reads all entries in the specified directory. If the directory contains many entries, this process may take a long time and use significant memory.
1482    ///
1483    /// To prevent this, consider using [`Operator::lister`] to stream the entries instead.
1484    ///
1485    /// # Examples
1486    ///
1487    /// This example will list all entries under the dir `path/to/dir/`.
1488    ///
1489    /// ```
1490    /// # use anyhow::Result;
1491    /// use opendal::EntryMode;
1492    /// use opendal::Operator;
1493    /// # async fn test(op: Operator) -> Result<()> {
1494    /// let mut entries = op.list("path/to/dir/").await?;
1495    /// for entry in entries {
1496    ///     match entry.metadata().mode() {
1497    ///         EntryMode::FILE => {
1498    ///             println!("Handling file")
1499    ///         }
1500    ///         EntryMode::DIR => {
1501    ///             println!("Handling dir {}", entry.path())
1502    ///         }
1503    ///         EntryMode::Unknown => continue,
1504    ///     }
1505    /// }
1506    /// # Ok(())
1507    /// # }
1508    /// ```
1509    pub async fn list(&self, path: &str) -> Result<Vec<Entry>> {
1510        self.list_with(path).await
1511    }
1512
1513    /// List entries in the parent directory that start with the specified `path` with additional options.
1514    ///
1515    /// # Notes
1516    ///
1517    /// ## Streaming List
1518    ///
1519    /// This function reads all entries in the specified directory. If the directory contains many entries, this process may take a long time and use significant memory.
1520    ///
1521    /// To prevent this, consider using [`Operator::lister`] to stream the entries instead.
1522    ///
1523    /// # Options
1524    ///
1525    /// Visit [`options::ListOptions`] for all available options.
1526    ///
1527    /// # Examples
1528    ///
1529    /// This example will list all entries recursively under the prefix `path/to/prefix`.
1530    ///
1531    /// ```
1532    /// # use anyhow::Result;
1533    /// use opendal::EntryMode;
1534    /// use opendal::Operator;
1535    /// # async fn test(op: Operator) -> Result<()> {
1536    /// let mut entries = op.list_with("path/to/prefix").recursive(true).await?;
1537    /// for entry in entries {
1538    ///     match entry.metadata().mode() {
1539    ///         EntryMode::FILE => {
1540    ///             println!("Handling file")
1541    ///         }
1542    ///         EntryMode::DIR => {
1543    ///             println!("Handling dir like start a new list via meta.path()")
1544    ///         }
1545    ///         EntryMode::Unknown => continue,
1546    ///     }
1547    /// }
1548    /// # Ok(())
1549    /// # }
1550    /// ```
1551    pub fn list_with(&self, path: &str) -> FutureList<impl Future<Output = Result<Vec<Entry>>>> {
1552        let path = normalize_path(path);
1553
1554        OperatorFuture::new(
1555            self.inner().clone(),
1556            path,
1557            options::ListOptions::default(),
1558            Self::list_inner,
1559        )
1560    }
1561
1562    /// List entries in the parent directory that start with the specified `path` with additional options.
1563    ///
1564    /// # Notes
1565    ///
1566    /// ## Streaming List
1567    ///
1568    /// This function reads all entries in the specified directory. If the directory contains many entries, this process may take a long time and use significant memory.
1569    ///
1570    /// To prevent this, consider using [`Operator::lister`] to stream the entries instead.
1571    ///
1572    /// # Options
1573    ///
1574    /// Visit [`options::ListOptions`] for all available options.
1575    ///
1576    /// # Examples
1577    ///
1578    /// This example will list all entries recursively under the prefix `path/to/prefix`.
1579    ///
1580    /// ```
1581    /// # use anyhow::Result;
1582    /// use opendal::options;
1583    /// use opendal::EntryMode;
1584    /// use opendal::Operator;
1585    /// # async fn test(op: Operator) -> Result<()> {
1586    /// let mut entries = op
1587    ///     .list_options("path/to/prefix", options::ListOptions {
1588    ///         recursive: true,
1589    ///         ..Default::default()
1590    ///     })
1591    ///     .await?;
1592    /// for entry in entries {
1593    ///     match entry.metadata().mode() {
1594    ///         EntryMode::FILE => {
1595    ///             println!("Handling file")
1596    ///         }
1597    ///         EntryMode::DIR => {
1598    ///             println!("Handling dir like start a new list via meta.path()")
1599    ///         }
1600    ///         EntryMode::Unknown => continue,
1601    ///     }
1602    /// }
1603    /// # Ok(())
1604    /// # }
1605    /// ```
1606    pub async fn list_options(&self, path: &str, opts: options::ListOptions) -> Result<Vec<Entry>> {
1607        let path = normalize_path(path);
1608        Self::list_inner(self.inner().clone(), path, opts).await
1609    }
1610
1611    #[inline]
1612    async fn list_inner(
1613        acc: Accessor,
1614        path: String,
1615        opts: options::ListOptions,
1616    ) -> Result<Vec<Entry>> {
1617        let args = opts.into();
1618        let lister = Lister::create(acc, &path, args).await?;
1619        lister.try_collect().await
1620    }
1621
1622    /// Create a new lister to list entries that starts with given `path` in parent dir.
1623    ///
1624    /// # Notes
1625    ///
1626    /// ## Recursively list
1627    ///
1628    /// This function only reads the immediate children of the specified directory.
1629    /// To retrieve all entries recursively, use [`Operator::lister_with`] with `recursive(true)` instead.
1630    ///
1631    /// # Examples
1632    ///
1633    /// ```
1634    /// # use anyhow::Result;
1635    /// # use futures::io;
1636    /// use futures::TryStreamExt;
1637    /// use opendal::EntryMode;
1638    /// use opendal::Operator;
1639    /// # async fn test(op: Operator) -> Result<()> {
1640    /// let mut ds = op.lister("path/to/dir/").await?;
1641    /// while let Some(mut de) = ds.try_next().await? {
1642    ///     match de.metadata().mode() {
1643    ///         EntryMode::FILE => {
1644    ///             println!("Handling file")
1645    ///         }
1646    ///         EntryMode::DIR => {
1647    ///             println!("Handling dir like start a new list via meta.path()")
1648    ///         }
1649    ///         EntryMode::Unknown => continue,
1650    ///     }
1651    /// }
1652    /// # Ok(())
1653    /// # }
1654    /// ```
1655    pub async fn lister(&self, path: &str) -> Result<Lister> {
1656        self.lister_with(path).await
1657    }
1658
1659    /// Create a new lister to list entries that starts with given `path` in parent dir with additional options.
1660    ///
1661    /// # Options
1662    ///
1663    /// Visit [`options::ListOptions`] for all available options.
1664    ///
1665    /// # Examples
1666    ///
1667    /// ## List all files recursively
1668    ///
1669    /// ```
1670    /// # use anyhow::Result;
1671    /// use futures::TryStreamExt;
1672    /// use opendal::EntryMode;
1673    /// use opendal::Operator;
1674    /// # async fn test(op: Operator) -> Result<()> {
1675    /// let mut lister = op.lister_with("path/to/dir/").recursive(true).await?;
1676    /// while let Some(mut entry) = lister.try_next().await? {
1677    ///     match entry.metadata().mode() {
1678    ///         EntryMode::FILE => {
1679    ///             println!("Handling file {}", entry.path())
1680    ///         }
1681    ///         EntryMode::DIR => {
1682    ///             println!("Handling dir {}", entry.path())
1683    ///         }
1684    ///         EntryMode::Unknown => continue,
1685    ///     }
1686    /// }
1687    /// # Ok(())
1688    /// # }
1689    /// ```
1690    pub fn lister_with(&self, path: &str) -> FutureLister<impl Future<Output = Result<Lister>>> {
1691        let path = normalize_path(path);
1692
1693        OperatorFuture::new(
1694            self.inner().clone(),
1695            path,
1696            options::ListOptions::default(),
1697            Self::lister_inner,
1698        )
1699    }
1700
1701    /// Create a new lister to list entries that starts with given `path` in parent dir with additional options.
1702    ///
1703    /// # Examples
1704    ///
1705    /// ## List all files recursively
1706    ///
1707    /// ```
1708    /// # use anyhow::Result;
1709    /// use futures::TryStreamExt;
1710    /// use opendal::options;
1711    /// use opendal::EntryMode;
1712    /// use opendal::Operator;
1713    /// # async fn test(op: Operator) -> Result<()> {
1714    /// let mut lister = op
1715    ///     .lister_options("path/to/dir/", options::ListOptions {
1716    ///         recursive: true,
1717    ///         ..Default::default()
1718    ///     })
1719    ///     .await?;
1720    /// while let Some(mut entry) = lister.try_next().await? {
1721    ///     match entry.metadata().mode() {
1722    ///         EntryMode::FILE => {
1723    ///             println!("Handling file {}", entry.path())
1724    ///         }
1725    ///         EntryMode::DIR => {
1726    ///             println!("Handling dir {}", entry.path())
1727    ///         }
1728    ///         EntryMode::Unknown => continue,
1729    ///     }
1730    /// }
1731    /// # Ok(())
1732    /// # }
1733    /// ```
1734    pub async fn lister_options(&self, path: &str, opts: options::ListOptions) -> Result<Lister> {
1735        let path = normalize_path(path);
1736        Self::lister_inner(self.inner().clone(), path, opts).await
1737    }
1738
1739    #[inline]
1740    async fn lister_inner(
1741        acc: Accessor,
1742        path: String,
1743        opts: options::ListOptions,
1744    ) -> Result<Lister> {
1745        let args = opts.into();
1746        let lister = Lister::create(acc, &path, args).await?;
1747        Ok(lister)
1748    }
1749}
1750
1751/// Operator presign API.
1752impl Operator {
1753    /// Presign an operation for stat(head).
1754    ///
1755    /// # Example
1756    ///
1757    /// ```
1758    /// use anyhow::Result;
1759    /// use futures::io;
1760    /// use opendal::Operator;
1761    /// use std::time::Duration;
1762    ///
1763    /// async fn test(op: Operator) -> Result<()> {
1764    ///     let signed_req = op.presign_stat("test",Duration::from_secs(3600)).await?;
1765    ///     let req = http::Request::builder()
1766    ///         .method(signed_req.method())
1767    ///         .uri(signed_req.uri())
1768    ///         .body(())?;
1769    ///
1770    /// #    Ok(())
1771    /// # }
1772    /// ```
1773    pub async fn presign_stat(&self, path: &str, expire: Duration) -> Result<PresignedRequest> {
1774        let path = normalize_path(path);
1775
1776        let op = OpPresign::new(OpStat::new(), expire);
1777
1778        let rp = self.inner().presign(&path, op).await?;
1779        Ok(rp.into_presigned_request())
1780    }
1781
1782    /// Presign an operation for stat(head).
1783    ///
1784    /// # Example
1785    ///
1786    /// ```
1787    /// use anyhow::Result;
1788    /// use futures::io;
1789    /// use opendal::Operator;
1790    /// use std::time::Duration;
1791    ///
1792    /// async fn test(op: Operator) -> Result<()> {
1793    ///     let signed_req = op.presign_stat_with("test",Duration::from_secs(3600)).override_content_disposition("attachment; filename=\"othertext.txt\"").await?;
1794    /// #    Ok(())
1795    /// # }
1796    /// ```
1797    pub fn presign_stat_with(
1798        &self,
1799        path: &str,
1800        expire: Duration,
1801    ) -> FuturePresignStat<impl Future<Output = Result<PresignedRequest>>> {
1802        let path = normalize_path(path);
1803
1804        OperatorFuture::new(
1805            self.inner().clone(),
1806            path,
1807            (options::StatOptions::default(), expire),
1808            Self::presign_stat_inner,
1809        )
1810    }
1811
1812    /// Presign an operation for stat(head) with additional options.
1813    ///
1814    /// # Options
1815    ///
1816    /// Visit [`options::StatOptions`] for all available options.
1817    ///
1818    /// # Example
1819    ///
1820    /// ```
1821    /// use anyhow::Result;
1822    /// use opendal::Operator;
1823    /// use opendal::options;
1824    /// use std::time::Duration;
1825    ///
1826    /// async fn test(op: Operator) -> Result<()> {
1827    ///     let signed_req = op.presign_stat_options(
1828    ///         "test",
1829    ///         Duration::from_secs(3600),
1830    ///         options::StatOptions {
1831    ///             if_match: Some("<etag>".to_string()),
1832    ///             ..Default::default()
1833    ///         }
1834    ///     ).await?;
1835    ///     let req = http::Request::builder()
1836    ///         .method(signed_req.method())
1837    ///         .uri(signed_req.uri())
1838    ///         .body(())?;
1839    ///
1840    /// #    Ok(())
1841    /// # }
1842    /// ```
1843    pub async fn presign_stat_options(
1844        &self,
1845        path: &str,
1846        expire: Duration,
1847        opts: options::StatOptions,
1848    ) -> Result<PresignedRequest> {
1849        let path = normalize_path(path);
1850        Self::presign_stat_inner(self.inner().clone(), path, (opts, expire)).await
1851    }
1852
1853    #[inline]
1854    async fn presign_stat_inner(
1855        acc: Accessor,
1856        path: String,
1857        (opts, expire): (options::StatOptions, Duration),
1858    ) -> Result<PresignedRequest> {
1859        let op = OpPresign::new(OpStat::from(opts), expire);
1860        let rp = acc.presign(&path, op).await?;
1861        Ok(rp.into_presigned_request())
1862    }
1863
1864    /// Presign an operation for read.
1865    ///
1866    /// # Notes
1867    ///
1868    /// ## Extra Options
1869    ///
1870    /// `presign_read` is a wrapper of [`Self::presign_read_with`] without any options. To use
1871    /// extra options like `override_content_disposition`, please use [`Self::presign_read_with`] or
1872    /// [`Self::presign_read_options] instead.
1873    ///
1874    /// # Example
1875    ///
1876    /// ```
1877    /// use anyhow::Result;
1878    /// use futures::io;
1879    /// use opendal::Operator;
1880    /// use std::time::Duration;
1881    ///
1882    /// async fn test(op: Operator) -> Result<()> {
1883    ///     let signed_req = op.presign_read("test.txt", Duration::from_secs(3600)).await?;
1884    /// #    Ok(())
1885    /// # }
1886    /// ```
1887    ///
1888    /// - `signed_req.method()`: `GET`
1889    /// - `signed_req.uri()`: `https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>`
1890    /// - `signed_req.headers()`: `{ "host": "s3.amazonaws.com" }`
1891    ///
1892    /// We can download this file via `curl` or other tools without credentials:
1893    ///
1894    /// ```shell
1895    /// curl "https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>" -O /tmp/test.txt
1896    /// ```
1897    pub async fn presign_read(&self, path: &str, expire: Duration) -> Result<PresignedRequest> {
1898        let path = normalize_path(path);
1899
1900        let op = OpPresign::new(OpRead::new(), expire);
1901
1902        let rp = self.inner().presign(&path, op).await?;
1903        Ok(rp.into_presigned_request())
1904    }
1905
1906    /// Presign an operation for read with extra options.
1907    ///
1908    /// # Options
1909    ///
1910    /// Visit [`options::ReadOptions`] for all available options.
1911    ///
1912    /// # Example
1913    ///
1914    /// ```
1915    /// use std::time::Duration;
1916    ///
1917    /// use anyhow::Result;
1918    /// use futures::io;
1919    /// use opendal::Operator;
1920    ///
1921    /// async fn test(op: Operator) -> Result<()> {
1922    ///     let signed_req = op
1923    ///         .presign_read_with("test.txt", Duration::from_secs(3600))
1924    ///         .override_content_type("text/plain")
1925    ///         .await?;
1926    ///     Ok(())
1927    /// }
1928    /// ```
1929    pub fn presign_read_with(
1930        &self,
1931        path: &str,
1932        expire: Duration,
1933    ) -> FuturePresignRead<impl Future<Output = Result<PresignedRequest>>> {
1934        let path = normalize_path(path);
1935
1936        OperatorFuture::new(
1937            self.inner().clone(),
1938            path,
1939            (options::ReadOptions::default(), expire),
1940            Self::presign_read_inner,
1941        )
1942    }
1943
1944    /// Presign an operation for read with additional options.
1945    ///
1946    /// # Options
1947    ///
1948    /// Visit [`options::ReadOptions`] for all available options.
1949    ///
1950    /// # Example
1951    ///
1952    /// ```
1953    /// use anyhow::Result;
1954    /// use opendal::Operator;
1955    /// use opendal::options;
1956    /// use std::time::Duration;
1957    ///
1958    /// async fn test(op: Operator) -> Result<()> {
1959    ///     let signed_req = op.presign_read_options(
1960    ///         "file",
1961    ///         Duration::from_secs(3600),
1962    ///         options::ReadOptions {
1963    ///             override_content_disposition: Some("attachment; filename=\"othertext.txt\"".to_string()),
1964    ///             ..Default::default()
1965    ///         }
1966    ///     ).await?;
1967    ///     let req = http::Request::builder()
1968    ///         .method(signed_req.method())
1969    ///         .uri(signed_req.uri())
1970    ///         .body(())?;
1971    ///
1972    /// #    Ok(())
1973    /// # }
1974    /// ```
1975    pub async fn presign_read_options(
1976        &self,
1977        path: &str,
1978        expire: Duration,
1979        opts: options::ReadOptions,
1980    ) -> Result<PresignedRequest> {
1981        let path = normalize_path(path);
1982        Self::presign_read_inner(self.inner().clone(), path, (opts, expire)).await
1983    }
1984
1985    #[inline]
1986    async fn presign_read_inner(
1987        acc: Accessor,
1988        path: String,
1989        (opts, expire): (options::ReadOptions, Duration),
1990    ) -> Result<PresignedRequest> {
1991        let (op_read, _) = opts.into();
1992        let op = OpPresign::new(op_read, expire);
1993        let rp = acc.presign(&path, op).await?;
1994        Ok(rp.into_presigned_request())
1995    }
1996
1997    /// Presign an operation for write.
1998    ///
1999    /// # Notes
2000    ///
2001    /// ## Extra Options
2002    ///
2003    /// `presign_write` is a wrapper of [`Self::presign_write_with`] without any options. To use
2004    /// extra options like `content_type`, please use [`Self::presign_write_with`] or
2005    /// [`Self::presign_write_options`] instead.
2006    ///
2007    /// # Example
2008    ///
2009    /// ```
2010    /// use std::time::Duration;
2011    ///
2012    /// use anyhow::Result;
2013    /// use opendal::Operator;
2014    ///
2015    /// async fn test(op: Operator) -> Result<()> {
2016    ///     let signed_req = op
2017    ///         .presign_write("test.txt", Duration::from_secs(3600))
2018    ///         .await?;
2019    ///     Ok(())
2020    /// }
2021    /// ```
2022    ///
2023    /// - `signed_req.method()`: `PUT`
2024    /// - `signed_req.uri()`: `https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>`
2025    /// - `signed_req.headers()`: `{ "host": "s3.amazonaws.com" }`
2026    ///
2027    /// We can upload file as this file via `curl` or other tools without credential:
2028    ///
2029    /// ```shell
2030    /// curl -X PUT "https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>" -d "Hello, World!"
2031    /// ```
2032    pub async fn presign_write(&self, path: &str, expire: Duration) -> Result<PresignedRequest> {
2033        self.presign_write_with(path, expire).await
2034    }
2035
2036    /// Presign an operation for write with extra options.
2037    ///
2038    /// # Options
2039    ///
2040    /// Visit [`options::WriteOptions`] for all available options.
2041    ///
2042    /// # Example
2043    ///
2044    /// ```
2045    /// use std::time::Duration;
2046    ///
2047    /// use anyhow::Result;
2048    /// use opendal::Operator;
2049    ///
2050    /// async fn test(op: Operator) -> Result<()> {
2051    ///     let signed_req = op
2052    ///         .presign_write_with("test", Duration::from_secs(3600))
2053    ///         .cache_control("no-store")
2054    ///         .await?;
2055    ///     let req = http::Request::builder()
2056    ///         .method(signed_req.method())
2057    ///         .uri(signed_req.uri())
2058    ///         .body(())?;
2059    ///
2060    ///     Ok(())
2061    /// }
2062    /// ```
2063    pub fn presign_write_with(
2064        &self,
2065        path: &str,
2066        expire: Duration,
2067    ) -> FuturePresignWrite<impl Future<Output = Result<PresignedRequest>>> {
2068        let path = normalize_path(path);
2069
2070        OperatorFuture::new(
2071            self.inner().clone(),
2072            path,
2073            (options::WriteOptions::default(), expire),
2074            Self::presign_write_inner,
2075        )
2076    }
2077
2078    /// Presign an operation for write with additional options.
2079    ///
2080    /// # Options
2081    ///
2082    /// Check [`options::WriteOptions`] for all available options.
2083    ///
2084    /// # Example
2085    ///
2086    /// ```
2087    /// use anyhow::Result;
2088    /// use opendal::Operator;
2089    /// use opendal::options;
2090    /// use std::time::Duration;
2091    ///
2092    /// async fn test(op: Operator) -> Result<()> {
2093    ///     let signed_req = op.presign_write_options(
2094    ///         "file",
2095    ///         Duration::from_secs(3600),
2096    ///         options::WriteOptions {
2097    ///             content_type: Some("application/json".to_string()),
2098    ///             cache_control: Some("max-age=3600".to_string()),
2099    ///             if_not_exists: true,
2100    ///             ..Default::default()
2101    ///         }
2102    ///     ).await?;
2103    ///     let req = http::Request::builder()
2104    ///         .method(signed_req.method())
2105    ///         .uri(signed_req.uri())
2106    ///         .body(())?;
2107    ///
2108    /// #    Ok(())
2109    /// # }
2110    /// ```
2111    pub async fn presign_write_options(
2112        &self,
2113        path: &str,
2114        expire: Duration,
2115        opts: options::WriteOptions,
2116    ) -> Result<PresignedRequest> {
2117        let path = normalize_path(path);
2118        Self::presign_write_inner(self.inner().clone(), path, (opts, expire)).await
2119    }
2120
2121    #[inline]
2122    async fn presign_write_inner(
2123        acc: Accessor,
2124        path: String,
2125        (opts, expire): (options::WriteOptions, Duration),
2126    ) -> Result<PresignedRequest> {
2127        let (op_write, _) = opts.into();
2128        let op = OpPresign::new(op_write, expire);
2129        let rp = acc.presign(&path, op).await?;
2130        Ok(rp.into_presigned_request())
2131    }
2132
2133    /// Presign an operation for delete.
2134    ///
2135    /// # Notes
2136    ///
2137    /// ## Extra Options
2138    ///
2139    /// `presign_delete` is a wrapper of [`Self::presign_delete_with`] without any options.
2140    ///
2141    /// # Example
2142    ///
2143    /// ```
2144    /// use std::time::Duration;
2145    ///
2146    /// use anyhow::Result;
2147    /// use opendal::Operator;
2148    ///
2149    /// async fn test(op: Operator) -> Result<()> {
2150    ///     let signed_req = op
2151    ///         .presign_delete("test.txt", Duration::from_secs(3600))
2152    ///         .await?;
2153    ///     Ok(())
2154    /// }
2155    /// ```
2156    ///
2157    /// - `signed_req.method()`: `DELETE`
2158    /// - `signed_req.uri()`: `https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>`
2159    /// - `signed_req.headers()`: `{ "host": "s3.amazonaws.com" }`
2160    ///
2161    /// We can delete file as this file via `curl` or other tools without credential:
2162    ///
2163    /// ```shell
2164    /// curl -X DELETE "https://s3.amazonaws.com/examplebucket/test.txt?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=access_key_id/20130721/us-east-1/s3/aws4_request&X-Amz-Date=20130721T201207Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=<signature-value>"
2165    /// ```
2166    pub async fn presign_delete(&self, path: &str, expire: Duration) -> Result<PresignedRequest> {
2167        self.presign_delete_with(path, expire).await
2168    }
2169
2170    /// Presign an operation for delete without extra options.
2171    pub fn presign_delete_with(
2172        &self,
2173        path: &str,
2174        expire: Duration,
2175    ) -> FuturePresignDelete<impl Future<Output = Result<PresignedRequest>>> {
2176        let path = normalize_path(path);
2177
2178        OperatorFuture::new(
2179            self.inner().clone(),
2180            path,
2181            (options::DeleteOptions::default(), expire),
2182            Self::presign_delete_inner,
2183        )
2184    }
2185
2186    /// Presign an operation for delete with additional options.
2187    ///
2188    /// # Options
2189    ///
2190    /// Visit [`options::DeleteOptions`] for all available options.
2191    ///
2192    /// # Example
2193    ///
2194    /// ```
2195    /// use anyhow::Result;
2196    /// use opendal::Operator;
2197    /// use opendal::options;
2198    /// use std::time::Duration;
2199    ///
2200    /// async fn test(op: Operator) -> Result<()> {
2201    ///     let signed_req = op.presign_delete_options(
2202    ///         "path/to/file",
2203    ///         Duration::from_secs(3600),
2204    ///         options::DeleteOptions {
2205    ///             ..Default::default()
2206    ///         }
2207    ///     ).await?;
2208    ///     let req = http::Request::builder()
2209    ///         .method(signed_req.method())
2210    ///         .uri(signed_req.uri())
2211    ///         .body(())?;
2212    ///
2213    /// #    Ok(())
2214    /// # }
2215    /// ```
2216    pub async fn presign_delete_options(
2217        &self,
2218        path: &str,
2219        expire: Duration,
2220        opts: options::DeleteOptions,
2221    ) -> Result<PresignedRequest> {
2222        let path = normalize_path(path);
2223        Self::presign_delete_inner(self.inner().clone(), path, (opts, expire)).await
2224    }
2225
2226    #[inline]
2227    async fn presign_delete_inner(
2228        acc: Accessor,
2229        path: String,
2230        (opts, expire): (options::DeleteOptions, Duration),
2231    ) -> Result<PresignedRequest> {
2232        let op = OpPresign::new(OpDelete::from(opts), expire);
2233        let rp = acc.presign(&path, op).await?;
2234        Ok(rp.into_presigned_request())
2235    }
2236}
```
