# 

opendal/blocking/

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
18use tokio::runtime::Handle;
19
20use crate::Operator as AsyncOperator;
21use crate::types::IntoOperatorUri;
22use crate::*;
23
24/// Use OpenDAL in blocking context.
25///
26/// # Notes
27///
28/// blocking::Operator is a wrapper around [`AsyncOperator`]. It calls async runtimes' `block_on` API to spawn blocking tasks.
29/// Please avoid using blocking::Operator in async context.
30///
31/// # Examples
32///
33/// ## Init in async context
34///
35/// blocking::Operator will use current async context's runtime to handle the async calls.
36///
37/// This is just for initialization. You must use `blocking::Operator` in blocking context.
38///
39/// ```rust,no_run
40/// # use opendal::services;
41/// # use opendal::blocking;
42/// # use opendal::Operator;
43/// # use opendal::Result;
44///
45/// #[tokio::main]
46/// async fn main() -> Result<()> {
47///     // Create fs backend builder.
48///     let mut builder = services::S3::default().bucket("test").region("us-east-1");
49///     let op = Operator::new(builder)?.finish();
50///
51///     // Build an `blocking::Operator` with blocking layer to start operating the storage.
52///     let _: blocking::Operator = blocking::Operator::new(op)?;
53///
54///     Ok(())
55/// }
56/// ```
57///
58/// ## In async context with blocking functions
59///
60/// If `blocking::Operator` is called in blocking function, please fetch a [`tokio::runtime::EnterGuard`]
61/// first. You can use [`Handle::try_current`] first to get the handle and then call [`Handle::enter`].
62/// This often happens in the case that async function calls blocking function.
63///
64/// ```rust,no_run
65/// # use opendal::services;
66/// # use opendal::blocking;
67/// # use opendal::Operator;
68/// # use opendal::Result;
69///
70/// #[tokio::main]
71/// async fn main() -> Result<()> {
72///     let _ = blocking_fn()?;
73///     Ok(())
74/// }
75///
76/// fn blocking_fn() -> Result<blocking::Operator> {
77///     // Create fs backend builder.
78///     let mut builder = services::S3::default().bucket("test").region("us-east-1");
79///     let op = Operator::new(builder)?.finish();
80///
81///     let handle = tokio::runtime::Handle::try_current().unwrap();
82///     let _guard = handle.enter();
83///     // Build an `blocking::Operator` to start operating the storage.
84///     let op: blocking::Operator = blocking::Operator::new(op)?;
85///     Ok(op)
86/// }
87/// ```
88///
89/// ## In blocking context
90///
91/// In a pure blocking context, we can create a runtime and use it to create the `blocking::Operator`.
92///
93/// > The following code uses a global statically created runtime as an example, please manage the
94/// > runtime on demand.
95///
96/// ```rust,no_run
97/// # use std::sync::LazyLock;
98/// # use opendal::services;
99/// # use opendal::blocking;
100/// # use opendal::Operator;
101/// # use opendal::Result;
102///
103/// static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
104///     tokio::runtime::Builder::new_multi_thread()
105///         .enable_all()
106///         .build()
107///         .unwrap()
108/// });
109///
110/// fn main() -> Result<()> {
111///     // Create fs backend builder.
112///     let mut builder = services::S3::default().bucket("test").region("us-east-1");
113///     let op = Operator::new(builder)?.finish();
114///
115///     // Fetch the `EnterGuard` from global runtime.
116///     let _guard = RUNTIME.enter();
117///     // Build an `blocking::Operator` with blocking layer to start operating the storage.
118///     let _: blocking::Operator = blocking::Operator::new(op)?;
119///
120///     Ok(())
121/// }
122/// ```
123#[derive(Clone, Debug)]
124pub struct Operator {
125    handle: tokio::runtime::Handle,
126    op: AsyncOperator,
127}
128
129impl Operator {
130    /// Create a new `BlockingLayer` with the current runtime's handle
131    pub fn new(op: AsyncOperator) -> Result<Self> {
132        Ok(Self {
133            handle: Handle::try_current()
134                .map_err(|_| Error::new(ErrorKind::Unexpected, "failed to get current handle"))?,
135            op,
136        })
137    }
138
139    /// Create a blocking operator from URI based configuration.
140    pub fn from_uri(uri: impl IntoOperatorUri) -> Result<Self> {
141        let op = AsyncOperator::from_uri(uri)?;
142        Self::new(op)
143    }
144
145    /// Get information of underlying accessor.
146    ///
147    /// # Examples
148    ///
149    /// ```
150    /// # use std::sync::Arc;
151    /// use opendal::blocking;
152    /// # use anyhow::Result;
153    /// use opendal::blocking::Operator;
154    ///
155    /// # fn test(op: blocking::Operator) -> Result<()> {
156    /// let info = op.info();
157    /// # Ok(())
158    /// # }
159    /// ```
160    pub fn info(&self) -> OperatorInfo {
161        self.op.info()
162    }
163}
164
165/// # Operator blocking API.
166impl Operator {
167    /// Get given path's metadata.
168    ///
169    /// # Behavior
170    ///
171    /// ## Services that support `create_dir`
172    ///
173    /// `test` and `test/` may vary in some services such as S3. However, on a local file system,
174    /// they're identical. Therefore, the behavior of `stat("test")` and `stat("test/")` might differ
175    /// in certain edge cases. Always use `stat("test/")` when you need to access a directory if possible.
176    ///
177    /// Here are the behavior list:
178    ///
179    /// | Case                   | Path            | Result                                     |
180    /// |------------------------|-----------------|--------------------------------------------|
181    /// | stat existing dir      | `abc/`          | Metadata with dir mode                     |
182    /// | stat existing file     | `abc/def_file`  | Metadata with file mode                    |
183    /// | stat dir without `/`   | `abc/def_dir`   | Error `NotFound` or metadata with dir mode |
184    /// | stat file with `/`     | `abc/def_file/` | Error `NotFound`                           |
185    /// | stat not existing path | `xyz`           | Error `NotFound`                           |
186    ///
187    /// Refer to [RFC: List Prefix][crate::docs::rfcs::rfc_3243_list_prefix] for more details.
188    ///
189    /// ## Services that not support `create_dir`
190    ///
191    /// For services that not support `create_dir`, `stat("test/")` will return `NotFound` even
192    /// when `test/abc` exists since the service won't have the concept of dir. There is nothing
193    /// we can do about this.
194    ///
195    /// # Examples
196    ///
197    /// ## Check if file exists
198    ///
199    /// ```
200    /// # use anyhow::Result;
201    /// # use futures::io;
202    /// use opendal::blocking;
203    /// # use opendal::blocking::Operator;
204    /// use opendal::ErrorKind;
205    /// #
206    /// # fn test(op: blocking::Operator) -> Result<()> {
207    /// if let Err(e) = op.stat("test") {
208    ///     if e.kind() == ErrorKind::NotFound {
209    ///         println!("file not exist")
210    ///     }
211    /// }
212    /// # Ok(())
213    /// # }
214    /// ```
215    pub fn stat(&self, path: &str) -> Result<Metadata> {
216        self.stat_options(path, options::StatOptions::default())
217    }
218
219    /// Get given path's metadata with extra options.
220    ///
221    /// # Behavior
222    ///
223    /// ## Services that support `create_dir`
224    ///
225    /// `test` and `test/` may vary in some services such as S3. However, on a local file system,
226    /// they're identical. Therefore, the behavior of `stat("test")` and `stat("test/")` might differ
227    /// in certain edge cases. Always use `stat("test/")` when you need to access a directory if possible.
228    ///
229    /// Here are the behavior list:
230    ///
231    /// | Case                   | Path            | Result                                     |
232    /// |------------------------|-----------------|--------------------------------------------|
233    /// | stat existing dir      | `abc/`          | Metadata with dir mode                     |
234    /// | stat existing file     | `abc/def_file`  | Metadata with file mode                    |
235    /// | stat dir without `/`   | `abc/def_dir`   | Error `NotFound` or metadata with dir mode |
236    /// | stat file with `/`     | `abc/def_file/` | Error `NotFound`                           |
237    /// | stat not existing path | `xyz`           | Error `NotFound`                           |
238    ///
239    /// Refer to [RFC: List Prefix][crate::docs::rfcs::rfc_3243_list_prefix] for more details.
240    ///
241    /// ## Services that not support `create_dir`
242    ///
243    /// For services that not support `create_dir`, `stat("test/")` will return `NotFound` even
244    /// when `test/abc` exists since the service won't have the concept of dir. There is nothing
245    /// we can do about this.
246    pub fn stat_options(&self, path: &str, opts: options::StatOptions) -> Result<Metadata> {
247        self.handle.block_on(self.op.stat_options(path, opts))
248    }
249
250    /// Check if this path exists or not.
251    ///
252    /// # Example
253    ///
254    /// ```no_run
255    /// use anyhow::Result;
256    /// use opendal::blocking;
257    /// use opendal::blocking::Operator;
258    /// fn test(op: blocking::Operator) -> Result<()> {
259    ///     let _ = op.exists("test")?;
260    ///
261    ///     Ok(())
262    /// }
263    /// ```
264    pub fn exists(&self, path: &str) -> Result<bool> {
265        let r = self.stat(path);
266        match r {
267            Ok(_) => Ok(true),
268            Err(err) => match err.kind() {
269                ErrorKind::NotFound => Ok(false),
270                _ => Err(err),
271            },
272        }
273    }
274
275    /// Create a dir at given path.
276    ///
277    /// # Notes
278    ///
279    /// To indicate that a path is a directory, it is compulsory to include
280    /// a trailing / in the path. Failure to do so may result in
281    /// `NotADirectory` error being returned by OpenDAL.
282    ///
283    /// # Behavior
284    ///
285    /// - Create on existing dir will succeed.
286    /// - Create dir is always recursive, works like `mkdir -p`
287    ///
288    /// # Examples
289    ///
290    /// ```no_run
291    /// # use opendal::Result;
292    /// use opendal::blocking;
293    /// # use opendal::blocking::Operator;
294    /// # use futures::TryStreamExt;
295    /// # fn test(op: blocking::Operator) -> Result<()> {
296    /// op.create_dir("path/to/dir/")?;
297    /// # Ok(())
298    /// # }
299    /// ```
300    pub fn create_dir(&self, path: &str) -> Result<()> {
301        self.handle.block_on(self.op.create_dir(path))
302    }
303
304    /// Read the whole path into a bytes.
305    ///
306    /// This function will allocate a new bytes internally. For more precise memory control or
307    /// reading data lazily, please use [`blocking::Operator::reader`]
308    ///
309    /// # Examples
310    ///
311    /// ```no_run
312    /// # use opendal::Result;
313    /// use opendal::blocking;
314    /// # use opendal::blocking::Operator;
315    /// #
316    /// # fn test(op: blocking::Operator) -> Result<()> {
317    /// let bs = op.read("path/to/file")?;
318    /// # Ok(())
319    /// # }
320    /// ```
321    pub fn read(&self, path: &str) -> Result<Buffer> {
322        self.read_options(path, options::ReadOptions::default())
323    }
324
325    /// Read the whole path into a bytes with extra options.
326    ///
327    /// This function will allocate a new bytes internally. For more precise memory control or
328    /// reading data lazily, please use [`blocking::Operator::reader`]
329    pub fn read_options(&self, path: &str, opts: options::ReadOptions) -> Result<Buffer> {
330        self.handle.block_on(self.op.read_options(path, opts))
331    }
332
333    /// Create a new reader which can read the whole path.
334    ///
335    /// # Examples
336    ///
337    /// ```no_run
338    /// # use opendal::Result;
339    /// use opendal::blocking;
340    /// # use opendal::blocking::Operator;
341    /// # use futures::TryStreamExt;
342    /// # fn test(op: blocking::Operator) -> Result<()> {
343    /// let r = op.reader("path/to/file")?;
344    /// # Ok(())
345    /// # }
346    /// ```
347    pub fn reader(&self, path: &str) -> Result<blocking::Reader> {
348        self.reader_options(path, options::ReaderOptions::default())
349    }
350
351    /// Create a new reader with extra options
352    pub fn reader_options(
353        &self,
354        path: &str,
355        opts: options::ReaderOptions,
356    ) -> Result<blocking::Reader> {
357        let r = self.handle.block_on(self.op.reader_options(path, opts))?;
358        Ok(blocking::Reader::new(self.handle.clone(), r))
359    }
360
361    /// Write bytes into given path.
362    ///
363    /// # Notes
364    ///
365    /// - Write will make sure all bytes has been written, or an error will be returned.
366    ///
367    /// # Examples
368    ///
369    /// ```no_run
370    /// # use opendal::Result;
371    /// # use opendal::blocking::Operator;
372    /// # use futures::StreamExt;
373    /// # use futures::SinkExt;
374    /// use bytes::Bytes;
375    /// use opendal::blocking;
376    ///
377    /// # fn test(op: blocking::Operator) -> Result<()> {
378    /// op.write("path/to/file", vec![0; 4096])?;
379    /// # Ok(())
380    /// # }
381    /// ```
382    pub fn write(&self, path: &str, bs: impl Into<Buffer>) -> Result<Metadata> {
383        self.write_options(path, bs, options::WriteOptions::default())
384    }
385
386    /// Write data with options.
387    ///
388    /// # Notes
389    ///
390    /// - Write will make sure all bytes has been written, or an error will be returned.
391    pub fn write_options(
392        &self,
393        path: &str,
394        bs: impl Into<Buffer>,
395        opts: options::WriteOptions,
396    ) -> Result<Metadata> {
397        self.handle.block_on(self.op.write_options(path, bs, opts))
398    }
399
400    /// Write multiple bytes into given path.
401    ///
402    /// # Notes
403    ///
404    /// - Write will make sure all bytes has been written, or an error will be returned.
405    ///
406    /// # Examples
407    ///
408    /// ```no_run
409    /// # use opendal::Result;
410    /// # use opendal::blocking;
411    /// # use opendal::blocking::Operator;
412    /// # use futures::StreamExt;
413    /// # use futures::SinkExt;
414    /// use bytes::Bytes;
415    ///
416    /// # fn test(op: blocking::Operator) -> Result<()> {
417    /// let mut w = op.writer("path/to/file")?;
418    /// w.write(vec![0; 4096])?;
419    /// w.write(vec![1; 4096])?;
420    /// w.close()?;
421    /// # Ok(())
422    /// # }
423    /// ```
424    pub fn writer(&self, path: &str) -> Result<blocking::Writer> {
425        self.writer_options(path, options::WriteOptions::default())
426    }
427
428    /// Create a new writer with extra options
429    pub fn writer_options(
430        &self,
431        path: &str,
432        opts: options::WriteOptions,
433    ) -> Result<blocking::Writer> {
434        let w = self.handle.block_on(self.op.writer_options(path, opts))?;
435        Ok(blocking::Writer::new(self.handle.clone(), w))
436    }
437
438    /// Copy a file from `from` to `to`.
439    ///
440    /// # Notes
441    ///
442    /// - `from` and `to` must be a file.
443    /// - `to` will be overwritten if it exists.
444    /// - If `from` and `to` are the same, nothing will happen.
445    /// - `copy` is idempotent. For same `from` and `to` input, the result will be the same.
446    ///
447    /// # Examples
448    ///
449    /// ```
450    /// # use opendal::Result;
451    /// use opendal::blocking;
452    /// # use opendal::blocking::Operator;
453    ///
454    /// # fn test(op: blocking::Operator) -> Result<()> {
455    /// op.copy("path/to/file", "path/to/file2")?;
456    /// # Ok(())
457    /// # }
458    /// ```
459    pub fn copy(&self, from: &str, to: &str) -> Result<()> {
460        self.handle.block_on(self.op.copy(from, to))
461    }
462
463    /// Rename a file from `from` to `to`.
464    ///
465    /// # Notes
466    ///
467    /// - `from` and `to` must be a file.
468    /// - `to` will be overwritten if it exists.
469    /// - If `from` and `to` are the same, a `IsSameFile` error will occur.
470    ///
471    /// # Examples
472    ///
473    /// ```
474    /// # use opendal::Result;
475    /// use opendal::blocking;
476    /// # use opendal::blocking::Operator;
477    ///
478    /// # fn test(op: blocking::Operator) -> Result<()> {
479    /// op.rename("path/to/file", "path/to/file2")?;
480    /// # Ok(())
481    /// # }
482    /// ```
483    pub fn rename(&self, from: &str, to: &str) -> Result<()> {
484        self.handle.block_on(self.op.rename(from, to))
485    }
486
487    /// Delete given path.
488    ///
489    /// # Notes
490    ///
491    /// - Delete not existing error won't return errors.
492    ///
493    /// # Examples
494    ///
495    /// ```no_run
496    /// # use anyhow::Result;
497    /// # use futures::io;
498    /// use opendal::blocking;
499    /// # use opendal::blocking::Operator;
500    /// # fn test(op: blocking::Operator) -> Result<()> {
501    /// op.delete("path/to/file")?;
502    /// # Ok(())
503    /// # }
504    /// ```
505    pub fn delete(&self, path: &str) -> Result<()> {
506        self.delete_options(path, options::DeleteOptions::default())
507    }
508
509    /// Delete given path with options.
510    ///
511    /// # Notes
512    ///
513    /// - Delete not existing error won't return errors.
514    pub fn delete_options(&self, path: &str, opts: options::DeleteOptions) -> Result<()> {
515        self.handle.block_on(self.op.delete_options(path, opts))
516    }
517
518    /// Delete an infallible iterator of paths.
519    ///
520    /// Also see:
521    ///
522    /// - [`blocking::Operator::delete_try_iter`]: delete an fallible iterator of paths.
523    pub fn delete_iter<I, D>(&self, iter: I) -> Result<()>
524    where
525        I: IntoIterator<Item = D>,
526        D: IntoDeleteInput,
527    {
528        self.handle.block_on(self.op.delete_iter(iter))
529    }
530
531    /// Delete a fallible iterator of paths.
532    ///
533    /// Also see:
534    ///
535    /// - [`blocking::Operator::delete_iter`]: delete an infallible iterator of paths.
536    pub fn delete_try_iter<I, D>(&self, try_iter: I) -> Result<()>
537    where
538        I: IntoIterator<Item = Result<D>>,
539        D: IntoDeleteInput,
540    {
541        self.handle.block_on(self.op.delete_try_iter(try_iter))
542    }
543
544    /// Create a [`BlockingDeleter`] to continuously remove content from storage.
545    ///
546    /// It leverages batch deletion capabilities provided by storage services for efficient removal.
547    ///
548    /// Users can have more control over the deletion process by using [`BlockingDeleter`] directly.
549    pub fn deleter(&self) -> Result<blocking::Deleter> {
550        blocking::Deleter::create(
551            self.handle.clone(),
552            self.handle.block_on(self.op.deleter())?,
553        )
554    }
555
556    /// Remove the path and all nested dirs and files recursively.
557    ///
558    /// # Notes
559    ///
560    /// We don't support batch delete now.
561    ///
562    /// # Examples
563    ///
564    /// ```
565    /// # use anyhow::Result;
566    /// # use futures::io;
567    /// use opendal::blocking;
568    /// # use opendal::blocking::Operator;
569    /// # fn test(op: blocking::Operator) -> Result<()> {
570    /// op.remove_all("path/to/dir")?;
571    /// # Ok(())
572    /// # }
573    /// ```
574    pub fn remove_all(&self, path: &str) -> Result<()> {
575        self.handle.block_on(self.op.remove_all(path))
576    }
577
578    /// List entries that starts with given `path` in parent dir.
579    ///
580    /// # Notes
581    ///
582    /// ## Recursively List
583    ///
584    /// This function only read the children of the given directory. To read
585    /// all entries recursively, use `blocking::Operator::list_options("path", opts)`
586    /// instead.
587    ///
588    /// ## Streaming List
589    ///
590    /// This function will read all entries in the given directory. It could
591    /// take very long time and consume a lot of memory if the directory
592    /// contains a lot of entries.
593    ///
594    /// In order to avoid this, you can use [`blocking::Operator::lister`] to list entries in
595    /// a streaming way.
596    ///
597    /// # Examples
598    ///
599    /// ```no_run
600    /// # use anyhow::Result;
601    /// use opendal::blocking;
602    /// use opendal::blocking::Operator;
603    /// use opendal::EntryMode;
604    /// #  fn test(op: blocking::Operator) -> Result<()> {
605    /// let mut entries = op.list("path/to/dir/")?;
606    /// for entry in entries {
607    ///     match entry.metadata().mode() {
608    ///         EntryMode::FILE => {
609    ///             println!("Handling file")
610    ///         }
611    ///         EntryMode::DIR => {
612    ///             println!("Handling dir {}", entry.path())
613    ///         }
614    ///         EntryMode::Unknown => continue,
615    ///     }
616    /// }
617    /// # Ok(())
618    /// # }
619    /// ```
620    pub fn list(&self, path: &str) -> Result<Vec<Entry>> {
621        self.list_options(path, options::ListOptions::default())
622    }
623
624    /// List entries that starts with given `path` in parent dir. with options.
625    ///
626    /// # Notes
627    ///
628    /// ## Streaming List
629    ///
630    /// This function will read all entries in the given directory. It could
631    /// take very long time and consume a lot of memory if the directory
632    /// contains a lot of entries.
633    ///
634    /// In order to avoid this, you can use [`blocking::Operator::lister`] to list entries in
635    /// a streaming way.
636    pub fn list_options(&self, path: &str, opts: options::ListOptions) -> Result<Vec<Entry>> {
637        self.handle.block_on(self.op.list_options(path, opts))
638    }
639
640    /// List entries that starts with given `path` in parent dir.
641    ///
642    /// This function will create a new [`BlockingLister`] to list entries. Users can stop listing
643    /// via dropping this [`Lister`].
644    ///
645    /// # Notes
646    ///
647    /// ## Recursively List
648    ///
649    /// This function only read the children of the given directory. To read
650    /// all entries recursively, use [`blocking::Operator::lister_with`] and `delimiter("")`
651    /// instead.
652    ///
653    /// # Examples
654    ///
655    /// ```no_run
656    /// # use anyhow::Result;
657    /// # use futures::io;
658    /// use futures::TryStreamExt;
659    /// use opendal::blocking;
660    /// use opendal::blocking::Operator;
661    /// use opendal::EntryMode;
662    /// # fn test(op: blocking::Operator) -> Result<()> {
663    /// let mut ds = op.lister("path/to/dir/")?;
664    /// for de in ds {
665    ///     let de = de?;
666    ///     match de.metadata().mode() {
667    ///         EntryMode::FILE => {
668    ///             println!("Handling file")
669    ///         }
670    ///         EntryMode::DIR => {
671    ///             println!("Handling dir like start a new list via meta.path()")
672    ///         }
673    ///         EntryMode::Unknown => continue,
674    ///     }
675    /// }
676    /// # Ok(())
677    /// # }
678    /// ```
679    pub fn lister(&self, path: &str) -> Result<blocking::Lister> {
680        self.lister_options(path, options::ListOptions::default())
681    }
682
683    /// List entries within a given directory as an iterator with options.
684    ///
685    /// This function will create a new handle to list entries.
686    ///
687    /// An error will be returned if given path doesn't end with `/`.
688    pub fn lister_options(
689        &self,
690        path: &str,
691        opts: options::ListOptions,
692    ) -> Result<blocking::Lister> {
693        let l = self.handle.block_on(self.op.lister_options(path, opts))?;
694        Ok(blocking::Lister::new(self.handle.clone(), l))
695    }
696
697    /// Check if this operator can work correctly.
698    ///
699    /// We will send a `list` request to path and return any errors we met.
700    ///
701    /// ```
702    /// # use std::sync::Arc;
703    /// # use anyhow::Result;
704    /// use opendal::blocking;
705    /// use opendal::blocking::Operator;
706    /// use opendal::ErrorKind;
707    ///
708    /// # fn test(op: blocking::Operator) -> Result<()> {
709    /// op.check()?;
710    /// # Ok(())
711    /// # }
712    /// ```
713    pub fn check(&self) -> Result<()> {
714        let mut ds = self.lister("/")?;
715
716        match ds.next() {
717            Some(Err(e)) if e.kind() != ErrorKind::NotFound => Err(e),
718            _ => Ok(()),
719        }
720    }
721}
722
723impl From<Operator> for AsyncOperator {
724    fn from(val: Operator) -> Self {
725        val.op
726    }
727}
```
