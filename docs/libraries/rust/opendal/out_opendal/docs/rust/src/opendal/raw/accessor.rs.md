# 

opendal/raw/

accessor.rs

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
19use std::future::ready;
20use std::hash::Hash;
21use std::hash::Hasher;
22use std::mem;
23use std::sync::Arc;
24
25use futures::Future;
26
27use crate::raw::*;
28use crate::*;
29
30/// Underlying trait of all backends for implementers.
31///
32/// The actual data access of storage service happens in Accessor layer.
33/// Every storage supported by OpenDAL must implement [`Access`] but not all
34/// methods of [`Access`] will be implemented according to how the storage service is.
35///
36/// For example, user can not modify the content from one HTTP file server directly.
37/// So [`Http`][crate::services::Http] implements and provides only read related actions.
38///
39/// [`Access`] gives default implementation for all methods which will raise [`ErrorKind::Unsupported`] error.
40/// And what action this [`Access`] supports will be pointed out in [`AccessorInfo`].
41///
42/// # Note
43///
44/// Visit [`internals`][crate::docs::internals] for more tutorials.
45///
46/// # Operations
47///
48/// - Path in args will all be normalized into the same style, services
49///   should handle them based on services' requirement.
50///   - Path that ends with `/` means it's Dir, otherwise, it's File.
51///   - Root dir is `/`
52///   - Path will never be empty.
53/// - Operations without capability requirement like `metadata`, `create` are
54///   basic operations.
55///   - All services must implement them.
56///   - Use `unimplemented!()` if not implemented or can't implement.
57/// - Operations with capability requirement like `presign` are optional operations.
58///   - Services can implement them based on services capabilities.
59///   - The default implementation should return [`ErrorKind::Unsupported`].
60pub trait Access: Send + Sync + Debug + Unpin + 'static {
61    /// Reader is the associated reader returned in `read` operation.
62    type Reader: oio::Read;
63    /// Writer is the associated writer returned in `write` operation.
64    type Writer: oio::Write;
65    /// Lister is the associated lister returned in `list` operation.
66    type Lister: oio::List;
67    /// Deleter is the associated deleter returned in `delete` operation.
68    type Deleter: oio::Delete;
69
70    /// Invoke the `info` operation to get metadata of accessor.
71    ///
72    /// # Notes
73    ///
74    /// This function is required to be implemented.
75    ///
76    /// By returning AccessorInfo, underlying services can declare
77    /// some useful information about itself.
78    ///
79    /// - scheme: declare the scheme of backend.
80    /// - capabilities: declare the capabilities of current backend.
81    fn info(&self) -> Arc<AccessorInfo>;
82
83    /// Invoke the `create` operation on the specified path
84    ///
85    /// Require [`Capability::create_dir`]
86    ///
87    /// # Behavior
88    ///
89    /// - Input path MUST match with EntryMode, DON'T NEED to check mode.
90    /// - Create on existing dir SHOULD succeed.
91    fn create_dir(
92        &self,
93        path: &str,
94        args: OpCreateDir,
95    ) -> impl Future<Output = Result<RpCreateDir>> + MaybeSend {
96        let (_, _) = (path, args);
97
98        ready(Err(Error::new(
99            ErrorKind::Unsupported,
100            "operation is not supported",
101        )))
102    }
103
104    /// Invoke the `stat` operation on the specified path.
105    ///
106    /// Require [`Capability::stat`]
107    ///
108    /// # Behavior
109    ///
110    /// - `stat` empty path means stat backend's root path.
111    /// - `stat` a path endswith "/" means stating a dir.
112    /// - `mode` and `content_length` must be set.
113    fn stat(&self, path: &str, args: OpStat) -> impl Future<Output = Result<RpStat>> + MaybeSend {
114        let (_, _) = (path, args);
115
116        ready(Err(Error::new(
117            ErrorKind::Unsupported,
118            "operation is not supported",
119        )))
120    }
121
122    /// Invoke the `read` operation on the specified path, returns a
123    /// [`Reader`][crate::Reader] if operate successful.
124    ///
125    /// Require [`Capability::read`]
126    ///
127    /// # Behavior
128    ///
129    /// - Input path MUST be file path, DON'T NEED to check mode.
130    /// - The returning content length may be smaller than the range specified.
131    fn read(
132        &self,
133        path: &str,
134        args: OpRead,
135    ) -> impl Future<Output = Result<(RpRead, Self::Reader)>> + MaybeSend {
136        let (_, _) = (path, args);
137
138        ready(Err(Error::new(
139            ErrorKind::Unsupported,
140            "operation is not supported",
141        )))
142    }
143
144    /// Invoke the `write` operation on the specified path, returns a
145    /// written size if operate successful.
146    ///
147    /// Require [`Capability::write`]
148    ///
149    /// # Behavior
150    ///
151    /// - Input path MUST be file path, DON'T NEED to check mode.
152    fn write(
153        &self,
154        path: &str,
155        args: OpWrite,
156    ) -> impl Future<Output = Result<(RpWrite, Self::Writer)>> + MaybeSend {
157        let (_, _) = (path, args);
158
159        ready(Err(Error::new(
160            ErrorKind::Unsupported,
161            "operation is not supported",
162        )))
163    }
164
165    /// Invoke the `delete` operation on the specified path.
166    ///
167    /// Require [`Capability::delete`]
168    ///
169    /// # Behavior
170    ///
171    /// - `delete` is an idempotent operation, it's safe to call `Delete` on the same path multiple times.
172    /// - `delete` SHOULD return `Ok(())` if the path is deleted successfully or not exist.
173    fn delete(&self) -> impl Future<Output = Result<(RpDelete, Self::Deleter)>> + MaybeSend {
174        ready(Err(Error::new(
175            ErrorKind::Unsupported,
176            "operation is not supported",
177        )))
178    }
179
180    /// Invoke the `list` operation on the specified path.
181    ///
182    /// Require [`Capability::list`]
183    ///
184    /// # Behavior
185    ///
186    /// - Input path MUST be dir path, DON'T NEED to check mode.
187    /// - List non-exist dir should return Empty.
188    fn list(
189        &self,
190        path: &str,
191        args: OpList,
192    ) -> impl Future<Output = Result<(RpList, Self::Lister)>> + MaybeSend {
193        let (_, _) = (path, args);
194
195        ready(Err(Error::new(
196            ErrorKind::Unsupported,
197            "operation is not supported",
198        )))
199    }
200
201    /// Invoke the `copy` operation on the specified `from` path and `to` path.
202    ///
203    /// Require [Capability::copy]
204    ///
205    /// # Behaviour
206    ///
207    /// - `from` and `to` MUST be file path, DON'T NEED to check mode.
208    /// - Copy on existing file SHOULD succeed.
209    /// - Copy on existing file SHOULD overwrite and truncate.
210    fn copy(
211        &self,
212        from: &str,
213        to: &str,
214        args: OpCopy,
215    ) -> impl Future<Output = Result<RpCopy>> + MaybeSend {
216        let (_, _, _) = (from, to, args);
217
218        ready(Err(Error::new(
219            ErrorKind::Unsupported,
220            "operation is not supported",
221        )))
222    }
223
224    /// Invoke the `rename` operation on the specified `from` path and `to` path.
225    ///
226    /// Require [Capability::rename]
227    fn rename(
228        &self,
229        from: &str,
230        to: &str,
231        args: OpRename,
232    ) -> impl Future<Output = Result<RpRename>> + MaybeSend {
233        let (_, _, _) = (from, to, args);
234
235        ready(Err(Error::new(
236            ErrorKind::Unsupported,
237            "operation is not supported",
238        )))
239    }
240
241    /// Invoke the `presign` operation on the specified path.
242    ///
243    /// Require [`Capability::presign`]
244    ///
245    /// # Behavior
246    ///
247    /// - This API is optional, return [`std::io::ErrorKind::Unsupported`] if not supported.
248    fn presign(
249        &self,
250        path: &str,
251        args: OpPresign,
252    ) -> impl Future<Output = Result<RpPresign>> + MaybeSend {
253        let (_, _) = (path, args);
254
255        ready(Err(Error::new(
256            ErrorKind::Unsupported,
257            "operation is not supported",
258        )))
259    }
260}
261
262/// `AccessDyn` is the dyn version of [`Access`] make it possible to use as
263/// `Box<dyn AccessDyn>`.
264pub trait AccessDyn: Send + Sync + Debug + Unpin {
265    /// Dyn version of [`Accessor::info`]
266    fn info_dyn(&self) -> Arc<AccessorInfo>;
267    /// Dyn version of [`Accessor::create_dir`]
268    fn create_dir_dyn<'a>(
269        &'a self,
270        path: &'a str,
271        args: OpCreateDir,
272    ) -> BoxedFuture<'a, Result<RpCreateDir>>;
273    /// Dyn version of [`Accessor::stat`]
274    fn stat_dyn<'a>(&'a self, path: &'a str, args: OpStat) -> BoxedFuture<'a, Result<RpStat>>;
275    /// Dyn version of [`Accessor::read`]
276    fn read_dyn<'a>(
277        &'a self,
278        path: &'a str,
279        args: OpRead,
280    ) -> BoxedFuture<'a, Result<(RpRead, oio::Reader)>>;
281    /// Dyn version of [`Accessor::write`]
282    fn write_dyn<'a>(
283        &'a self,
284        path: &'a str,
285        args: OpWrite,
286    ) -> BoxedFuture<'a, Result<(RpWrite, oio::Writer)>>;
287    /// Dyn version of [`Accessor::delete`]
288    fn delete_dyn(&self) -> BoxedFuture<'_, Result<(RpDelete, oio::Deleter)>>;
289    /// Dyn version of [`Accessor::list`]
290    fn list_dyn<'a>(
291        &'a self,
292        path: &'a str,
293        args: OpList,
294    ) -> BoxedFuture<'a, Result<(RpList, oio::Lister)>>;
295    /// Dyn version of [`Accessor::copy`]
296    fn copy_dyn<'a>(
297        &'a self,
298        from: &'a str,
299        to: &'a str,
300        args: OpCopy,
301    ) -> BoxedFuture<'a, Result<RpCopy>>;
302    /// Dyn version of [`Accessor::rename`]
303    fn rename_dyn<'a>(
304        &'a self,
305        from: &'a str,
306        to: &'a str,
307        args: OpRename,
308    ) -> BoxedFuture<'a, Result<RpRename>>;
309    /// Dyn version of [`Accessor::presign`]
310    fn presign_dyn<'a>(
311        &'a self,
312        path: &'a str,
313        args: OpPresign,
314    ) -> BoxedFuture<'a, Result<RpPresign>>;
315}
316
317impl<A: ?Sized> AccessDyn for A
318where
319    A: Access<
320            Reader = oio::Reader,
321            Writer = oio::Writer,
322            Lister = oio::Lister,
323            Deleter = oio::Deleter,
324        >,
325{
326    fn info_dyn(&self) -> Arc<AccessorInfo> {
327        self.info()
328    }
329
330    fn create_dir_dyn<'a>(
331        &'a self,
332        path: &'a str,
333        args: OpCreateDir,
334    ) -> BoxedFuture<'a, Result<RpCreateDir>> {
335        Box::pin(self.create_dir(path, args))
336    }
337
338    fn stat_dyn<'a>(&'a self, path: &'a str, args: OpStat) -> BoxedFuture<'a, Result<RpStat>> {
339        Box::pin(self.stat(path, args))
340    }
341
342    fn read_dyn<'a>(
343        &'a self,
344        path: &'a str,
345        args: OpRead,
346    ) -> BoxedFuture<'a, Result<(RpRead, oio::Reader)>> {
347        Box::pin(self.read(path, args))
348    }
349
350    fn write_dyn<'a>(
351        &'a self,
352        path: &'a str,
353        args: OpWrite,
354    ) -> BoxedFuture<'a, Result<(RpWrite, oio::Writer)>> {
355        Box::pin(self.write(path, args))
356    }
357
358    fn delete_dyn(&self) -> BoxedFuture<'_, Result<(RpDelete, oio::Deleter)>> {
359        Box::pin(self.delete())
360    }
361
362    fn list_dyn<'a>(
363        &'a self,
364        path: &'a str,
365        args: OpList,
366    ) -> BoxedFuture<'a, Result<(RpList, oio::Lister)>> {
367        Box::pin(self.list(path, args))
368    }
369
370    fn copy_dyn<'a>(
371        &'a self,
372        from: &'a str,
373        to: &'a str,
374        args: OpCopy,
375    ) -> BoxedFuture<'a, Result<RpCopy>> {
376        Box::pin(self.copy(from, to, args))
377    }
378
379    fn rename_dyn<'a>(
380        &'a self,
381        from: &'a str,
382        to: &'a str,
383        args: OpRename,
384    ) -> BoxedFuture<'a, Result<RpRename>> {
385        Box::pin(self.rename(from, to, args))
386    }
387
388    fn presign_dyn<'a>(
389        &'a self,
390        path: &'a str,
391        args: OpPresign,
392    ) -> BoxedFuture<'a, Result<RpPresign>> {
393        Box::pin(self.presign(path, args))
394    }
395}
396
397impl Access for dyn AccessDyn {
398    type Reader = oio::Reader;
399    type Writer = oio::Writer;
400    type Deleter = oio::Deleter;
401    type Lister = oio::Lister;
402
403    fn info(&self) -> Arc<AccessorInfo> {
404        self.info_dyn()
405    }
406
407    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
408        self.create_dir_dyn(path, args).await
409    }
410
411    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
412        self.stat_dyn(path, args).await
413    }
414
415    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
416        self.read_dyn(path, args).await
417    }
418
419    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
420        self.write_dyn(path, args).await
421    }
422
423    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
424        self.delete_dyn().await
425    }
426
427    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
428        self.list_dyn(path, args).await
429    }
430
431    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
432        self.copy_dyn(from, to, args).await
433    }
434
435    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
436        self.rename_dyn(from, to, args).await
437    }
438
439    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
440        self.presign_dyn(path, args).await
441    }
442}
443
444/// Dummy implementation of accessor.
445impl Access for () {
446    type Reader = ();
447    type Writer = ();
448    type Lister = ();
449    type Deleter = ();
450
451    fn info(&self) -> Arc<AccessorInfo> {
452        let ai = AccessorInfo::default();
453        ai.set_scheme("dummy")
454            .set_root("")
455            .set_name("dummy")
456            .set_native_capability(Capability::default());
457        ai.into()
458    }
459}
460
461/// All functions in `Accessor` only requires `&self`, so it's safe to implement
462/// `Accessor` for `Arc<impl Access>`.
463// If we use async fn directly, some weird higher rank trait bound error (`Send`/`Accessor` impl not general enough) will happen.
464// Probably related to https://github.com/rust-lang/rust/issues/96865
465#[allow(clippy::manual_async_fn)]
466impl<T: Access + ?Sized> Access for Arc<T> {
467    type Reader = T::Reader;
468    type Writer = T::Writer;
469    type Lister = T::Lister;
470    type Deleter = T::Deleter;
471
472    fn info(&self) -> Arc<AccessorInfo> {
473        self.as_ref().info()
474    }
475
476    fn create_dir(
477        &self,
478        path: &str,
479        args: OpCreateDir,
480    ) -> impl Future<Output = Result<RpCreateDir>> + MaybeSend {
481        async move { self.as_ref().create_dir(path, args).await }
482    }
483
484    fn stat(&self, path: &str, args: OpStat) -> impl Future<Output = Result<RpStat>> + MaybeSend {
485        async move { self.as_ref().stat(path, args).await }
486    }
487
488    fn read(
489        &self,
490        path: &str,
491        args: OpRead,
492    ) -> impl Future<Output = Result<(RpRead, Self::Reader)>> + MaybeSend {
493        async move { self.as_ref().read(path, args).await }
494    }
495
496    fn write(
497        &self,
498        path: &str,
499        args: OpWrite,
500    ) -> impl Future<Output = Result<(RpWrite, Self::Writer)>> + MaybeSend {
501        async move { self.as_ref().write(path, args).await }
502    }
503
504    fn delete(&self) -> impl Future<Output = Result<(RpDelete, Self::Deleter)>> + MaybeSend {
505        async move { self.as_ref().delete().await }
506    }
507
508    fn list(
509        &self,
510        path: &str,
511        args: OpList,
512    ) -> impl Future<Output = Result<(RpList, Self::Lister)>> + MaybeSend {
513        async move { self.as_ref().list(path, args).await }
514    }
515
516    fn copy(
517        &self,
518        from: &str,
519        to: &str,
520        args: OpCopy,
521    ) -> impl Future<Output = Result<RpCopy>> + MaybeSend {
522        async move { self.as_ref().copy(from, to, args).await }
523    }
524
525    fn rename(
526        &self,
527        from: &str,
528        to: &str,
529        args: OpRename,
530    ) -> impl Future<Output = Result<RpRename>> + MaybeSend {
531        async move { self.as_ref().rename(from, to, args).await }
532    }
533
534    fn presign(
535        &self,
536        path: &str,
537        args: OpPresign,
538    ) -> impl Future<Output = Result<RpPresign>> + MaybeSend {
539        async move { self.as_ref().presign(path, args).await }
540    }
541}
542
543/// Accessor is the type erased accessor with `Arc<dyn Accessor>`.
544pub type Accessor = Arc<dyn AccessDyn>;
545
546#[derive(Debug)]
547struct AccessorInfoInner {
548    scheme: &'static str,
549    root: Arc<str>,
550    name: Arc<str>,
551
552    native_capability: Capability,
553    full_capability: Capability,
554
555    http_client: HttpClient,
556    executor: Executor,
557}
558
559impl Default for AccessorInfoInner {
560    fn default() -> Self {
561        Self {
562            scheme: "unknown",
563            root: Arc::from(""),
564            name: Arc::from(""),
565            native_capability: Capability::default(),
566            full_capability: Capability::default(),
567            http_client: HttpClient::default(),
568            executor: Executor::default(),
569        }
570    }
571}
572
573/// Info for the accessor. Users can use this struct to retrieve information about the underlying backend.
574///
575/// This struct is intentionally not implemented with `Clone` to ensure that all accesses
576/// within the same operator, access layers, and services use the same instance of `AccessorInfo`.
577/// This is especially important for `HttpClient` and `Executor`.
578///
579/// ## Panic Safety
580///
581/// All methods provided by `AccessorInfo` will safely handle lock poisoning scenarios.
582/// If the inner `RwLock` is poisoned (which happens when another thread panicked while holding
583/// the write lock), this method will gracefully continue execution.
584///
585/// - For read operations, the method will return the current state.
586/// - For write operations, the method will do nothing.
587///
588/// ## Maintain Notes
589///
590/// We are using `std::sync::RwLock` to provide thread-safe access to the inner data.
591///
592/// I have performed [the bench across different arc-swap alike crates](https://github.com/krdln/arc-swap-benches):
593///
594/// ```txt
595/// test arcswap                    ... bench:          14.85 ns/iter (+/- 0.33)
596/// test arcswap_full               ... bench:         128.27 ns/iter (+/- 4.30)
597/// test baseline                   ... bench:          11.33 ns/iter (+/- 0.76)
598/// test mutex_4                    ... bench:         296.73 ns/iter (+/- 49.96)
599/// test mutex_unconteded           ... bench:          13.26 ns/iter (+/- 0.56)
600/// test rwlock_fast_4              ... bench:         201.60 ns/iter (+/- 7.47)
601/// test rwlock_fast_uncontended    ... bench:          12.77 ns/iter (+/- 0.37)
602/// test rwlock_parking_4           ... bench:         232.02 ns/iter (+/- 11.14)
603/// test rwlock_parking_uncontended ... bench:          13.18 ns/iter (+/- 0.39)
604/// test rwlock_std_4               ... bench:         219.56 ns/iter (+/- 5.56)
605/// test rwlock_std_uncontended     ... bench:          13.55 ns/iter (+/- 0.33)
606/// ```
607///
608/// The results show that as long as there aren't too many uncontended accesses, `RwLock` is the
609/// best choice, allowing for fast access and the ability to modify partial data without cloning
610/// everything.
611///
612/// And it's true: we only update and modify the internal data in a few instances, such as when
613/// building an operator or applying new layers.
614#[derive(Debug, Default)]
615pub struct AccessorInfo {
616    inner: std::sync::RwLock<AccessorInfoInner>,
617}
618
619impl PartialEq for AccessorInfo {
620    fn eq(&self, other: &Self) -> bool {
621        self.scheme() == other.scheme()
622            && self.root() == other.root()
623            && self.name() == other.name()
624    }
625}
626
627impl Eq for AccessorInfo {}
628
629impl Hash for AccessorInfo {
630    fn hash<H: Hasher>(&self, state: &mut H) {
631        self.scheme().hash(state);
632        self.root().hash(state);
633        self.name().hash(state);
634    }
635}
636
637impl AccessorInfo {
638    /// Scheme of backend.
639    ///
640    /// # Panic Safety
641    ///
642    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
643    /// this method will gracefully continue execution by simply returning the current scheme.
644    pub fn scheme(&self) -> &'static str {
645        match self.inner.read() {
646            Ok(v) => v.scheme,
647            Err(err) => err.get_ref().scheme,
648        }
649    }
650
651    /// Set scheme for backend.
652    ///
653    /// # Panic Safety
654    ///
655    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
656    /// this method will gracefully continue execution by simply skipping the update operation
657    /// rather than propagating the panic.
658    pub fn set_scheme(&self, scheme: &'static str) -> &Self {
659        if let Ok(mut v) = self.inner.write() {
660            v.scheme = scheme;
661        }
662
663        self
664    }
665
666    /// Root of backend, will be in format like `/path/to/dir/`
667    ///
668    /// # Panic Safety
669    ///
670    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
671    /// this method will gracefully continue execution by simply returning the current root.
672    pub fn root(&self) -> Arc<str> {
673        match self.inner.read() {
674            Ok(v) => v.root.clone(),
675            Err(err) => err.get_ref().root.clone(),
676        }
677    }
678
679    /// Set root for backend.
680    ///
681    /// Note: input root must be normalized.
682    ///
683    /// # Panic Safety
684    ///
685    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
686    /// this method will gracefully continue execution by simply skipping the update operation
687    /// rather than propagating the panic.
688    pub fn set_root(&self, root: &str) -> &Self {
689        if let Ok(mut v) = self.inner.write() {
690            v.root = Arc::from(root);
691        }
692
693        self
694    }
695
696    /// Name of backend, could be empty if underlying backend doesn't have namespace concept.
697    ///
698    /// For example:
699    ///
700    /// - `s3` => bucket name
701    /// - `azblob` => container name
702    /// - `azdfs` => filesystem name
703    /// - `azfile` => share name
704    ///
705    /// # Panic Safety
706    ///
707    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
708    /// this method will gracefully continue execution by simply returning the current scheme.
709    pub fn name(&self) -> Arc<str> {
710        match self.inner.read() {
711            Ok(v) => v.name.clone(),
712            Err(err) => err.get_ref().name.clone(),
713        }
714    }
715
716    /// Set name of this backend.
717    ///
718    /// # Panic Safety
719    ///
720    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
721    /// this method will gracefully continue execution by simply skipping the update operation
722    /// rather than propagating the panic.
723    pub fn set_name(&self, name: &str) -> &Self {
724        if let Ok(mut v) = self.inner.write() {
725            v.name = Arc::from(name)
726        }
727
728        self
729    }
730
731    /// Get backend's native capabilities.
732    ///
733    /// # Panic Safety
734    ///
735    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
736    /// this method will gracefully continue execution by simply returning the current native capability.
737    pub fn native_capability(&self) -> Capability {
738        match self.inner.read() {
739            Ok(v) => v.native_capability,
740            Err(err) => err.get_ref().native_capability,
741        }
742    }
743
744    /// Set native capabilities for service.
745    ///
746    /// # NOTES
747    ///
748    /// Set native capability will also flush the full capability. The only way to change
749    /// full_capability is via `update_full_capability`.
750    ///
751    /// # Panic Safety
752    ///
753    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
754    /// this method will gracefully continue execution by simply skipping the update operation
755    /// rather than propagating the panic.
756    pub fn set_native_capability(&self, capability: Capability) -> &Self {
757        if let Ok(mut v) = self.inner.write() {
758            v.native_capability = capability;
759            v.full_capability = capability;
760        }
761
762        self
763    }
764
765    /// Get service's full capabilities.
766    ///
767    /// # Panic Safety
768    ///
769    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
770    /// this method will gracefully continue execution by simply returning the current native capability.
771    pub fn full_capability(&self) -> Capability {
772        match self.inner.read() {
773            Ok(v) => v.full_capability,
774            Err(err) => err.get_ref().full_capability,
775        }
776    }
777
778    /// Update service's full capabilities.
779    ///
780    /// # Panic Safety
781    ///
782    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
783    /// this method will gracefully continue execution by simply skipping the update operation
784    /// rather than propagating the panic.
785    pub fn update_full_capability(&self, f: impl FnOnce(Capability) -> Capability) -> &Self {
786        if let Ok(mut v) = self.inner.write() {
787            v.full_capability = f(v.full_capability);
788        }
789
790        self
791    }
792
793    /// Get http client from the context.
794    ///
795    /// # Panic Safety
796    ///
797    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
798    /// this method will gracefully continue execution by simply returning the current http client.
799    pub fn http_client(&self) -> HttpClient {
800        match self.inner.read() {
801            Ok(v) => v.http_client.clone(),
802            Err(err) => err.get_ref().http_client.clone(),
803        }
804    }
805
806    /// Update http client for the context.
807    ///
808    /// # Note
809    ///
810    /// Requests must be forwarded to the old HTTP client after the update. Otherwise, features such as retry, tracing, and metrics may not function properly.
811    ///
812    /// # Panic Safety
813    ///
814    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
815    /// this method will gracefully continue execution by simply skipping the update operation.
816    pub fn update_http_client(&self, f: impl FnOnce(HttpClient) -> HttpClient) -> &Self {
817        if let Ok(mut v) = self.inner.write() {
818            let client = mem::take(&mut v.http_client);
819            v.http_client = f(client);
820        }
821
822        self
823    }
824
825    /// Get executor from the context.
826    ///
827    /// # Panic Safety
828    ///
829    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
830    /// this method will gracefully continue execution by simply returning the current executor.
831    pub fn executor(&self) -> Executor {
832        match self.inner.read() {
833            Ok(v) => v.executor.clone(),
834            Err(err) => err.get_ref().executor.clone(),
835        }
836    }
837
838    /// Update executor for the context.
839    ///
840    /// # Note
841    ///
842    /// Tasks must be forwarded to the old executor after the update. Otherwise, features such as retry, timeout, and metrics may not function properly.
843    ///
844    /// # Panic Safety
845    ///
846    /// This method safely handles lock poisoning scenarios. If the inner `RwLock` is poisoned,
847    /// this method will gracefully continue execution by simply skipping the update operation.
848    pub fn update_executor(&self, f: impl FnOnce(Executor) -> Executor) -> &Self {
849        if let Ok(mut v) = self.inner.write() {
850            let executor = mem::take(&mut v.executor);
851            v.executor = f(executor);
852        }
853
854        self
855    }
856}
```
