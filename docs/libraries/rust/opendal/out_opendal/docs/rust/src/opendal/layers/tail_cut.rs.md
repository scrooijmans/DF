# 

opendal/layers/

tail_cut.rs

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
19use std::fmt::Formatter;
20use std::sync::Arc;
21use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
22use std::time::Duration;
23use std::time::Instant;
24
25use crate::raw::*;
26use crate::*;
27
28/// Builder for TailCutLayer.
29///
30/// Use this to configure the layer, then call `build()` to create a layer
31/// that can be cloned and shared across multiple operators.
32///
33/// # Examples
34///
35/// ```no_run
36/// use opendal::layers::TailCutLayer;
37/// use std::time::Duration;
38/// # use opendal::services;
39/// # use opendal::Operator;
40/// # use opendal::Result;
41///
42/// # fn main() -> Result<()> {
43/// let layer = TailCutLayer::builder()
44///     .percentile(95)
45///     .window(Duration::from_secs(60))
46///     .build();
47///
48/// let op = Operator::new(services::Memory::default())?
49///     .layer(layer)
50///     .finish();
51/// # Ok(())
52/// # }
53/// ```
54#[derive(Clone)]
55pub struct TailCutLayerBuilder {
56    percentile: u8,
57    safety_factor: f64,
58    window: Duration,
59    min_samples: usize,
60    min_deadline: Duration,
61    max_deadline: Duration,
62}
63
64impl Default for TailCutLayerBuilder {
65    fn default() -> Self {
66        Self {
67            percentile: 95,
68            safety_factor: 1.3,
69            window: Duration::from_secs(60),
70            min_samples: 200,
71            min_deadline: Duration::from_millis(500),
72            max_deadline: Duration::from_secs(30),
73        }
74    }
75}
76
77impl TailCutLayerBuilder {
78    /// Create a new builder with default settings.
79    pub fn new() -> Self {
80        Self::default()
81    }
82
83    /// Set the percentile threshold (e.g., 95 for P95, 99 for P99).
84    ///
85    /// Requests slower than this percentile Ã safety_factor will be cancelled.
86    ///
87    /// Default: 95
88    ///
89    /// # Panics
90    ///
91    /// Panics if percentile is not between 50 and 99.
92    pub fn percentile(mut self, percentile: u8) -> Self {
93        assert!(
94            (50..=99).contains(&percentile),
95            "percentile must be between 50 and 99"
96        );
97        self.percentile = percentile;
98        self
99    }
100
101    /// Set the safety factor multiplier.
102    ///
103    /// The actual deadline is calculated as: P{percentile} Ã safety_factor.
104    /// A higher value reduces false positives but may miss some long tails.
105    ///
106    /// Default: 1.3 (30% buffer)
107    ///
108    /// # Panics
109    ///
110    /// Panics if factor is not between 1.0 and 5.0.
111    pub fn safety_factor(mut self, factor: f64) -> Self {
112        assert!(
113            (1.0..=5.0).contains(&factor),
114            "safety_factor must be between 1.0 and 5.0"
115        );
116        self.safety_factor = factor;
117        self
118    }
119
120    /// Set the sliding window duration for statistics collection.
121    ///
122    /// Longer windows provide more stable statistics but react slower to changes.
123    /// Shorter windows adapt faster but may be more noisy.
124    ///
125    /// Default: 60 seconds
126    ///
127    /// # Panics
128    ///
129    /// Panics if window is greater than 120 seconds.
130    pub fn window(mut self, window: Duration) -> Self {
131        assert!(
132            window <= Duration::from_secs(120),
133            "window must be <= 120 seconds"
134        );
135        self.window = window;
136        self
137    }
138
139    /// Set the minimum number of samples required before enabling adaptive cancellation.
140    ///
141    /// During cold start (when sample count < min_samples), the layer will not
142    /// cancel any requests to avoid false positives.
143    ///
144    /// Default: 200
145    pub fn min_samples(mut self, min_samples: usize) -> Self {
146        self.min_samples = min_samples;
147        self
148    }
149
150    /// Set the minimum deadline (floor).
151    ///
152    /// Even if calculated deadline is shorter, it will be clamped to this value.
153    /// This prevents overly aggressive cancellation on very fast backends.
154    ///
155    /// Default: 500ms
156    pub fn min_deadline(mut self, deadline: Duration) -> Self {
157        self.min_deadline = deadline;
158        self
159    }
160
161    /// Set the maximum deadline (ceiling).
162    ///
163    /// Even if calculated deadline is longer, it will be clamped to this value.
164    /// This acts as a safety fallback timeout.
165    ///
166    /// Default: 30s
167    pub fn max_deadline(mut self, deadline: Duration) -> Self {
168        self.max_deadline = deadline;
169        self
170    }
171
172    /// Build the layer.
173    ///
174    /// The returned layer can be cloned to share statistics across operators.
175    ///
176    /// # Examples
177    ///
178    /// ```no_run
179    /// use opendal::layers::TailCutLayer;
180    /// use std::time::Duration;
181    /// # use opendal::services;
182    /// # use opendal::Operator;
183    /// # use opendal::Result;
184    ///
185    /// # fn main() -> Result<()> {
186    /// let layer = TailCutLayer::builder()
187    ///     .percentile(95)
188    ///     .window(Duration::from_secs(60))
189    ///     .build();
190    ///
191    /// // Share the layer across operators
192    /// let op1 = Operator::new(services::Memory::default())?
193    ///     .layer(layer.clone())
194    ///     .finish();
195    ///
196    /// let op2 = Operator::new(services::Memory::default())?
197    ///     .layer(layer.clone())
198    ///     .finish();
199    /// // op1 and op2 share the same statistics
200    /// # Ok(())
201    /// # }
202    /// ```
203    pub fn build(self) -> TailCutLayer {
204        TailCutLayer {
205            config: Arc::new(TailCutConfig {
206                percentile: self.percentile,
207                safety_factor: self.safety_factor,
208                window: self.window,
209                min_samples: self.min_samples,
210                min_deadline: self.min_deadline,
211                max_deadline: self.max_deadline,
212            }),
213            stats: Arc::new(TailCutStats::new()),
214        }
215    }
216}
217
218/// Configuration for TailCutLayer (immutable).
219#[derive(Debug)]
220struct TailCutConfig {
221    percentile: u8,
222    safety_factor: f64,
223    window: Duration,
224    min_samples: usize,
225    min_deadline: Duration,
226    max_deadline: Duration,
227}
228
229/// Layer that automatically cancels long-tail requests.
230///
231/// This layer monitors request latency distribution and cancels requests that are
232/// significantly slower than the historical baseline (e.g., slower than P95).
233///
234/// This layer should be created via [`TailCutLayer::builder()`] and can be
235/// cloned to share statistics across multiple operators.
236///
237/// # Examples
238///
239/// ```no_run
240/// use opendal::layers::TailCutLayer;
241/// use std::time::Duration;
242/// # use opendal::services;
243/// # use opendal::Operator;
244/// # use opendal::Result;
245///
246/// # fn main() -> Result<()> {
247/// let layer = TailCutLayer::builder()
248///     .percentile(95)
249///     .safety_factor(1.3)
250///     .window(Duration::from_secs(60))
251///     .build();
252///
253/// let op = Operator::new(services::Memory::default())?
254///     .layer(layer)
255///     .finish();
256/// # Ok(())
257/// # }
258/// ```
259#[derive(Clone)]
260pub struct TailCutLayer {
261    config: Arc<TailCutConfig>,
262    stats: Arc<TailCutStats>,
263}
264
265impl TailCutLayer {
266    /// Create a builder to configure the layer.
267    pub fn builder() -> TailCutLayerBuilder {
268        TailCutLayerBuilder::new()
269    }
270
271    /// Create a layer with default settings.
272    ///
273    /// This is equivalent to `TailCutLayer::builder().build()`.
274    pub fn new() -> Self {
275        Self::builder().build()
276    }
277}
278
279impl Default for TailCutLayer {
280    fn default() -> Self {
281        Self::new()
282    }
283}
284
285impl<A: Access> Layer<A> for TailCutLayer {
286    type LayeredAccess = TailCutAccessor<A>;
287
288    fn layer(&self, inner: A) -> Self::LayeredAccess {
289        TailCutAccessor {
290            inner,
291            config: self.config.clone(),
292            stats: self.stats.clone(),
293        }
294    }
295}
296
297/// Accessor that implements tail cut logic.
298#[derive(Clone)]
299pub struct TailCutAccessor<A: Access> {
300    inner: A,
301    config: Arc<TailCutConfig>,
302    stats: Arc<TailCutStats>,
303}
304
305impl<A: Access> Debug for TailCutAccessor<A> {
306    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
307        f.debug_struct("TailCutAccessor")
308            .field("config", &self.config)
309            .finish_non_exhaustive()
310    }
311}
312
313impl<A: Access> TailCutAccessor<A> {
314    /// Calculate the deadline for a given operation and size.
315    fn calculate_deadline(&self, op: Operation, size: Option<u64>) -> Option<Duration> {
316        let op_stats = self.stats.stats_for(op);
317
318        if op_stats.total_samples(size, self.config.window) < self.config.min_samples {
319            return None;
320        }
321
322        let q = self.config.percentile as f64 / 100.0;
323        let pctl = op_stats.quantile(size, q, self.config.window)?;
324
325        let deadline = Duration::from_secs_f64(pctl.as_secs_f64() * self.config.safety_factor);
326        Some(deadline.clamp(self.config.min_deadline, self.config.max_deadline))
327    }
328
329    async fn with_deadline<F, T>(&self, op: Operation, size: Option<u64>, fut: F) -> Result<T>
330    where
331        F: std::future::Future<Output = Result<T>>,
332    {
333        let start = Instant::now();
334
335        let result = if let Some(deadline) = self.calculate_deadline(op, size) {
336            match tokio::time::timeout(deadline, fut).await {
337                Ok(res) => res,
338                Err(_) => Err(Error::new(ErrorKind::Unexpected, "cancelled by tail cut")
339                    .with_operation(op)
340                    .with_context("percentile", format!("P{}", self.config.percentile))
341                    .with_context("deadline", format!("{:?}", deadline))
342                    .set_temporary()),
343            }
344        } else {
345            fut.await
346        };
347
348        if result.is_ok() {
349            let latency = start.elapsed();
350            self.stats.stats_for(op).record(size, latency);
351        }
352
353        result
354    }
355}
356
357impl<A: Access> LayeredAccess for TailCutAccessor<A> {
358    type Inner = A;
359    type Reader = TailCutWrapper<A::Reader>;
360    type Writer = TailCutWrapper<A::Writer>;
361    type Lister = TailCutWrapper<A::Lister>;
362    type Deleter = TailCutWrapper<A::Deleter>;
363
364    fn inner(&self) -> &Self::Inner {
365        &self.inner
366    }
367
368    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
369        self.with_deadline(
370            Operation::CreateDir,
371            None,
372            self.inner.create_dir(path, args),
373        )
374        .await
375    }
376
377    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
378        let size = args.range().size();
379        self.with_deadline(Operation::Read, size, self.inner.read(path, args))
380            .await
381            .map(|(rp, r)| {
382                (
383                    rp,
384                    TailCutWrapper::new(r, size, self.config.clone(), self.stats.clone()),
385                )
386            })
387    }
388
389    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
390        self.with_deadline(Operation::Write, None, self.inner.write(path, args))
391            .await
392            .map(|(rp, w)| {
393                (
394                    rp,
395                    TailCutWrapper::new(w, None, self.config.clone(), self.stats.clone()),
396                )
397            })
398    }
399
400    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
401        self.with_deadline(Operation::Copy, None, self.inner.copy(from, to, args))
402            .await
403    }
404
405    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
406        self.with_deadline(Operation::Rename, None, self.inner.rename(from, to, args))
407            .await
408    }
409
410    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
411        self.with_deadline(Operation::Stat, None, self.inner.stat(path, args))
412            .await
413    }
414
415    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
416        self.with_deadline(Operation::Delete, None, self.inner.delete())
417            .await
418            .map(|(rp, d)| {
419                (
420                    rp,
421                    TailCutWrapper::new(d, None, self.config.clone(), self.stats.clone()),
422                )
423            })
424    }
425
426    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
427        self.with_deadline(Operation::List, None, self.inner.list(path, args))
428            .await
429            .map(|(rp, l)| {
430                (
431                    rp,
432                    TailCutWrapper::new(l, None, self.config.clone(), self.stats.clone()),
433                )
434            })
435    }
436
437    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
438        self.with_deadline(Operation::Presign, None, self.inner.presign(path, args))
439            .await
440    }
441}
442
443/// Wrapper for IO operations (Reader, Writer, Lister, Deleter).
444pub struct TailCutWrapper<R> {
445    inner: R,
446    size: Option<u64>,
447    config: Arc<TailCutConfig>,
448    stats: Arc<TailCutStats>,
449}
450
451impl<R> TailCutWrapper<R> {
452    fn new(
453        inner: R,
454        size: Option<u64>,
455        config: Arc<TailCutConfig>,
456        stats: Arc<TailCutStats>,
457    ) -> Self {
458        Self {
459            inner,
460            size,
461            config,
462            stats,
463        }
464    }
465
466    fn calculate_deadline(&self, op: Operation) -> Option<Duration> {
467        let op_stats = self.stats.stats_for(op);
468
469        if op_stats.total_samples(self.size, self.config.window) < self.config.min_samples {
470            return None;
471        }
472
473        let q = self.config.percentile as f64 / 100.0;
474        let pctl = op_stats.quantile(self.size, q, self.config.window)?;
475
476        let deadline = Duration::from_secs_f64(pctl.as_secs_f64() * self.config.safety_factor);
477        Some(deadline.clamp(self.config.min_deadline, self.config.max_deadline))
478    }
479
480    #[inline]
481    async fn with_io_deadline<F, T>(
482        deadline: Option<Duration>,
483        percentile: u8,
484        stats: &Arc<TailCutStats>,
485        size: Option<u64>,
486        op: Operation,
487        fut: F,
488    ) -> Result<T>
489    where
490        F: std::future::Future<Output = Result<T>>,
491    {
492        let start = Instant::now();
493
494        let result = if let Some(dl) = deadline {
495            match tokio::time::timeout(dl, fut).await {
496                Ok(res) => res,
497                Err(_) => Err(
498                    Error::new(ErrorKind::Unexpected, "io cancelled by tail cut")
499                        .with_operation(op)
500                        .with_context("percentile", format!("P{}", percentile))
501                        .with_context("deadline", format!("{:?}", dl))
502                        .set_temporary(),
503                ),
504            }
505        } else {
506            fut.await
507        };
508
509        if result.is_ok() {
510            let latency = start.elapsed();
511            stats.stats_for(op).record(size, latency);
512        }
513
514        result
515    }
516}
517
518impl<R: oio::Read> oio::Read for TailCutWrapper<R> {
519    async fn read(&mut self) -> Result<Buffer> {
520        let deadline = self.calculate_deadline(Operation::Read);
521        Self::with_io_deadline(
522            deadline,
523            self.config.percentile,
524            &self.stats,
525            self.size,
526            Operation::Read,
527            self.inner.read(),
528        )
529        .await
530    }
531}
532
533impl<R: oio::Write> oio::Write for TailCutWrapper<R> {
534    async fn write(&mut self, bs: Buffer) -> Result<()> {
535        let deadline = self.calculate_deadline(Operation::Write);
536        Self::with_io_deadline(
537            deadline,
538            self.config.percentile,
539            &self.stats,
540            self.size,
541            Operation::Write,
542            self.inner.write(bs),
543        )
544        .await
545    }
546
547    async fn close(&mut self) -> Result<Metadata> {
548        let deadline = self.calculate_deadline(Operation::Write);
549        Self::with_io_deadline(
550            deadline,
551            self.config.percentile,
552            &self.stats,
553            self.size,
554            Operation::Write,
555            self.inner.close(),
556        )
557        .await
558    }
559
560    async fn abort(&mut self) -> Result<()> {
561        let deadline = self.calculate_deadline(Operation::Write);
562        Self::with_io_deadline(
563            deadline,
564            self.config.percentile,
565            &self.stats,
566            self.size,
567            Operation::Write,
568            self.inner.abort(),
569        )
570        .await
571    }
572}
573
574impl<R: oio::List> oio::List for TailCutWrapper<R> {
575    async fn next(&mut self) -> Result<Option<oio::Entry>> {
576        let deadline = self.calculate_deadline(Operation::List);
577        Self::with_io_deadline(
578            deadline,
579            self.config.percentile,
580            &self.stats,
581            self.size,
582            Operation::List,
583            self.inner.next(),
584        )
585        .await
586    }
587}
588
589impl<R: oio::Delete> oio::Delete for TailCutWrapper<R> {
590    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()> {
591        self.inner.delete(path, args)
592    }
593
594    async fn flush(&mut self) -> Result<usize> {
595        let deadline = self.calculate_deadline(Operation::Delete);
596        Self::with_io_deadline(
597            deadline,
598            self.config.percentile,
599            &self.stats,
600            self.size,
601            Operation::Delete,
602            self.inner.flush(),
603        )
604        .await
605    }
606}
607
608/// Statistics engine for tail cut layer.
609struct TailCutStats {
610    // Statistics for each operation type (7 operations)
611    operations: [Arc<OperationStats>; 7],
612}
613
614impl TailCutStats {
615    fn new() -> Self {
616        Self {
617            operations: std::array::from_fn(|_| Arc::new(OperationStats::new())),
618        }
619    }
620
621    fn stats_for(&self, op: Operation) -> &Arc<OperationStats> {
622        let idx = match op {
623            Operation::Read => 0,
624            Operation::Write => 1,
625            Operation::Stat => 2,
626            Operation::List => 3,
627            Operation::Delete => 4,
628            Operation::Copy => 5,
629            Operation::Rename => 6,
630            _ => 2, // fallback to Stat
631        };
632        &self.operations[idx]
633    }
634}
635
636/// Statistics for a single operation type.
637struct OperationStats {
638    buckets: Vec<SizeBucket>,
639}
640
641impl OperationStats {
642    fn new() -> Self {
643        Self {
644            buckets: vec![
645                SizeBucket::new(0, Some(4 * 1024)),                   // [0, 4KB)
646                SizeBucket::new(4 * 1024, Some(64 * 1024)),           // [4KB, 64KB)
647                SizeBucket::new(64 * 1024, Some(1024 * 1024)),        // [64KB, 1MB)
648                SizeBucket::new(1024 * 1024, Some(16 * 1024 * 1024)), // [1MB, 16MB)
649                SizeBucket::new(16 * 1024 * 1024, Some(256 * 1024 * 1024)), // [16MB, 256MB)
650                SizeBucket::new(256 * 1024 * 1024, None),             // [256MB, â)
651            ],
652        }
653    }
654
655    fn bucket_for(&self, size: Option<u64>) -> &SizeBucket {
656        let size = size.unwrap_or(u64::MAX);
657
658        self.buckets
659            .iter()
660            .find(|b| b.contains(size))
661            .unwrap_or(&self.buckets[self.buckets.len() - 1])
662    }
663
664    fn record(&self, size: Option<u64>, latency: Duration) {
665        self.bucket_for(size).histogram.record(latency);
666    }
667
668    fn quantile(&self, size: Option<u64>, q: f64, window: Duration) -> Option<Duration> {
669        self.bucket_for(size).histogram.quantile(q, window)
670    }
671
672    fn total_samples(&self, size: Option<u64>, window: Duration) -> usize {
673        self.bucket_for(size).histogram.total_samples(window)
674    }
675}
676
677/// Size bucket for categorizing operations by data size.
678struct SizeBucket {
679    min_size: u64,
680    max_size: Option<u64>,
681    histogram: WindowedHistogram,
682}
683
684impl SizeBucket {
685    fn new(min_size: u64, max_size: Option<u64>) -> Self {
686        Self {
687            min_size,
688            max_size,
689            histogram: WindowedHistogram::new(),
690        }
691    }
692
693    fn contains(&self, size: u64) -> bool {
694        size >= self.min_size && self.max_size.is_none_or(|max| size < max)
695    }
696}
697
698const SLICE_DURATION_MS: u64 = 10_000; // 10 seconds per slice
699const NUM_SLICES: usize = 12; // 12 slices = 120 seconds total window
700const NUM_BUCKETS: usize = 17; // 17 buckets covering 1ms to 64s
701
702/// Windowed histogram using lock-free atomic operations.
703struct WindowedHistogram {
704    slices: Box<[TimeSlice; NUM_SLICES]>,
705    current_idx: AtomicUsize,
706    last_rotate: AtomicU64,
707}
708
709impl WindowedHistogram {
710    fn new() -> Self {
711        Self {
712            slices: Box::new(std::array::from_fn(|_| TimeSlice::new())),
713            current_idx: AtomicUsize::new(0),
714            last_rotate: AtomicU64::new(Self::now_ms()),
715        }
716    }
717
718    fn record(&self, latency: Duration) {
719        self.maybe_rotate();
720
721        let bucket_idx = Self::latency_to_bucket(latency);
722        let slice_idx = self.current_idx.load(Ordering::Relaxed);
723
724        self.slices[slice_idx].buckets[bucket_idx].fetch_add(1, Ordering::Relaxed);
725    }
726
727    fn quantile(&self, q: f64, window: Duration) -> Option<Duration> {
728        debug_assert!((0.0..=1.0).contains(&q), "quantile must be in [0, 1]");
729
730        let snapshot = self.snapshot(window);
731        let total: u64 = snapshot.iter().sum();
732
733        if total == 0 {
734            return None;
735        }
736
737        let target = (total as f64 * q).ceil() as u64;
738        let mut cumsum = 0u64;
739
740        for (bucket_idx, &count) in snapshot.iter().enumerate() {
741            cumsum += count;
742            if cumsum >= target {
743                return Some(Self::bucket_to_latency(bucket_idx));
744            }
745        }
746
747        Some(Self::bucket_to_latency(NUM_BUCKETS - 1))
748    }
749
750    fn total_samples(&self, window: Duration) -> usize {
751        self.snapshot(window).iter().map(|&v| v as usize).sum()
752    }
753
754    fn snapshot(&self, window: Duration) -> [u64; NUM_BUCKETS] {
755        let mut result = [0u64; NUM_BUCKETS];
756        let now_ms = Self::now_ms();
757        let window_ms = window.as_millis() as u64;
758
759        for slice in self.slices.iter() {
760            let start = slice.start_epoch_ms.load(Ordering::Acquire);
761
762            if start > 0 && now_ms.saturating_sub(start) < window_ms + SLICE_DURATION_MS {
763                for (i, bucket) in slice.buckets.iter().enumerate() {
764                    result[i] += bucket.load(Ordering::Relaxed);
765                }
766            }
767        }
768
769        result
770    }
771
772    fn maybe_rotate(&self) {
773        let now = Self::now_ms();
774        let last_rotate = self.last_rotate.load(Ordering::Relaxed);
775
776        if now - last_rotate >= SLICE_DURATION_MS
777            && self
778                .last_rotate
779                .compare_exchange(last_rotate, now, Ordering::Release, Ordering::Relaxed)
780                .is_ok()
781        {
782            let old_idx = self.current_idx.load(Ordering::Relaxed);
783            let new_idx = (old_idx + 1) % NUM_SLICES;
784
785            let new_slice = &self.slices[new_idx];
786            new_slice.start_epoch_ms.store(now, Ordering::Release);
787            for bucket in &new_slice.buckets {
788                bucket.store(0, Ordering::Relaxed);
789            }
790
791            self.current_idx.store(new_idx, Ordering::Release);
792        }
793    }
794
795    fn latency_to_bucket(latency: Duration) -> usize {
796        let ms = latency.as_millis() as u64;
797
798        if ms == 0 {
799            return 0;
800        }
801
802        let bucket = 64 - ms.leading_zeros();
803        (bucket as usize).min(NUM_BUCKETS - 1)
804    }
805
806    fn bucket_to_latency(bucket_idx: usize) -> Duration {
807        if bucket_idx == 0 {
808            Duration::from_millis(1)
809        } else if bucket_idx >= NUM_BUCKETS - 1 {
810            Duration::from_secs(64)
811        } else {
812            Duration::from_millis(1u64 << bucket_idx)
813        }
814    }
815
816    fn now_ms() -> u64 {
817        std::time::SystemTime::now()
818            .duration_since(std::time::UNIX_EPOCH)
819            .unwrap()
820            .as_millis() as u64
821    }
822}
823
824/// Time slice in the sliding window.
825struct TimeSlice {
826    // 17 buckets covering 1ms to 64s (logarithmic scale)
827    buckets: [AtomicU64; NUM_BUCKETS],
828    start_epoch_ms: AtomicU64,
829}
830
831impl TimeSlice {
832    fn new() -> Self {
833        Self {
834            buckets: std::array::from_fn(|_| AtomicU64::new(0)),
835            start_epoch_ms: AtomicU64::new(0),
836        }
837    }
838}
839
840#[cfg(test)]
841mod tests {
842    use super::*;
843
844    #[test]
845    fn test_latency_to_bucket() {
846        assert_eq!(
847            WindowedHistogram::latency_to_bucket(Duration::from_millis(0)),
848            0
849        );
850        assert_eq!(
851            WindowedHistogram::latency_to_bucket(Duration::from_millis(1)),
852            1
853        );
854        assert_eq!(
855            WindowedHistogram::latency_to_bucket(Duration::from_millis(2)),
856            2
857        );
858        assert_eq!(
859            WindowedHistogram::latency_to_bucket(Duration::from_millis(4)),
860            3
861        );
862        assert_eq!(
863            WindowedHistogram::latency_to_bucket(Duration::from_millis(8)),
864            4
865        );
866        assert_eq!(
867            WindowedHistogram::latency_to_bucket(Duration::from_millis(500)),
868            9
869        );
870        assert_eq!(
871            WindowedHistogram::latency_to_bucket(Duration::from_secs(1)),
872            10
873        );
874        assert_eq!(
875            WindowedHistogram::latency_to_bucket(Duration::from_secs(2)),
876            11
877        );
878        assert_eq!(
879            WindowedHistogram::latency_to_bucket(Duration::from_secs(64)),
880            16
881        );
882        assert_eq!(
883            WindowedHistogram::latency_to_bucket(Duration::from_secs(1000)),
884            16
885        );
886    }
887
888    #[test]
889    fn test_size_bucket_contains() {
890        let bucket = SizeBucket::new(0, Some(4096));
891        assert!(bucket.contains(0));
892        assert!(bucket.contains(4095));
893        assert!(!bucket.contains(4096));
894
895        let bucket = SizeBucket::new(4096, None);
896        assert!(!bucket.contains(4095));
897        assert!(bucket.contains(4096));
898        assert!(bucket.contains(u64::MAX));
899    }
900
901    #[tokio::test]
902    async fn test_histogram_basic() {
903        let hist = WindowedHistogram::new();
904        let now = WindowedHistogram::now_ms();
905        hist.slices[0].start_epoch_ms.store(now, Ordering::Release);
906
907        hist.record(Duration::from_millis(10));
908        hist.record(Duration::from_millis(20));
909        hist.record(Duration::from_millis(30));
910
911        let samples = hist.total_samples(Duration::from_secs(60));
912        assert_eq!(samples, 3);
913
914        let p50 = hist.quantile(0.5, Duration::from_secs(60));
915        assert!(p50.is_some());
916    }
917
918    #[tokio::test]
919    async fn test_tail_cut_layer_build() {
920        let layer = TailCutLayer::builder()
921            .percentile(95)
922            .safety_factor(1.5)
923            .window(Duration::from_secs(60))
924            .min_samples(100)
925            .min_deadline(Duration::from_millis(200))
926            .max_deadline(Duration::from_secs(20))
927            .build();
928
929        assert_eq!(layer.config.percentile, 95);
930        assert_eq!(layer.config.safety_factor, 1.5);
931        assert_eq!(layer.config.window, Duration::from_secs(60));
932        assert_eq!(layer.config.min_samples, 100);
933        assert_eq!(layer.config.min_deadline, Duration::from_millis(200));
934        assert_eq!(layer.config.max_deadline, Duration::from_secs(20));
935    }
936
937    #[tokio::test]
938    async fn test_layer_clone_shares_stats() {
939        let layer = TailCutLayer::new();
940        let cloned = layer.clone();
941
942        assert!(Arc::ptr_eq(&layer.stats, &cloned.stats));
943    }
944}
```
