# 

opendal/raw/

futures_util.rs

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
18use std::collections::VecDeque;
19use std::sync::Arc;
20use std::sync::atomic::AtomicUsize;
21use std::sync::atomic::Ordering;
22
23use futures::FutureExt;
24
25use crate::*;
26
27/// BoxedFuture is the type alias of [`futures::future::BoxFuture`].
28#[cfg(not(target_arch = "wasm32"))]
29pub type BoxedFuture<'a, T> = futures::future::BoxFuture<'a, T>;
30#[cfg(target_arch = "wasm32")]
31/// BoxedFuture is the type alias of [`futures::future::LocalBoxFuture`].
32pub type BoxedFuture<'a, T> = futures::future::LocalBoxFuture<'a, T>;
33
34/// BoxedStaticFuture is the type alias of [`futures::future::BoxFuture`].
35#[cfg(not(target_arch = "wasm32"))]
36pub type BoxedStaticFuture<T> = futures::future::BoxFuture<'static, T>;
37#[cfg(target_arch = "wasm32")]
38/// BoxedStaticFuture is the type alias of [`futures::future::LocalBoxFuture`].
39pub type BoxedStaticFuture<T> = futures::future::LocalBoxFuture<'static, T>;
40
41/// MaybeSend is a marker to determine whether a type is `Send` or not.
42/// We use this trait to wrap the `Send` requirement for wasm32 target.
43///
44/// # Safety
45///
46/// [`MaybeSend`] is equivalent to `Send` on non-wasm32 target.
47/// And it's empty trait on wasm32 target to indicate that a type is not `Send`.
48#[cfg(not(target_arch = "wasm32"))]
49pub trait MaybeSend: Send {}
50
51/// MaybeSend is a marker to determine whether a type is `Send` or not.
52/// We use this trait to wrap the `Send` requirement for wasm32 target.
53///
54/// # Safety
55///
56/// [`MaybeSend`] is equivalent to `Send` on non-wasm32 target.
57/// And it's empty trait on wasm32 target to indicate that a type is not `Send`.
58#[cfg(target_arch = "wasm32")]
59pub trait MaybeSend {}
60
61#[cfg(not(target_arch = "wasm32"))]
62impl<T: Send> MaybeSend for T {}
63#[cfg(target_arch = "wasm32")]
64impl<T> MaybeSend for T {}
65
66/// ConcurrentTasks is used to execute tasks concurrently.
67///
68/// ConcurrentTasks has two generic types:
69///
70/// - `I` represents the input type of the task.
71/// - `O` represents the output type of the task.
72///
73/// # Implementation Notes
74///
75/// The code patterns below are intentional; please do not modify them unless you fully understand these notes.
76///
77/// ```skip
78///  let (i, o) = self
79///     .tasks
80///     .front_mut()                                        // Use `front_mut` instead of `pop_front`
81///     .expect("tasks must be available")
82///     .await;
83/// ...
84/// match o {
85///     Ok(o) => {
86///         let _ = self.tasks.pop_front();                 // `pop_front` after got `Ok(o)`
87///         self.results.push_back(o)
88///     }
89///     Err(err) => {
90///         if err.is_temporary() {
91///             let task = self.create_task(i);
92///             self.tasks
93///                 .front_mut()
94///                 .expect("tasks must be available")
95///                 .replace(task)                          // Use replace here to instead of `push_front`
96///         } else {
97///             self.clear();
98///             self.errored = true;
99///         }
100///         return Err(err);
101///     }
102/// }
103/// ```
104///
105/// Please keep in mind that there is no guarantee the task will be `await`ed until completion. It's possible
106/// the task may be dropped before it resolves. Therefore, we should keep the `Task` in the `tasks` queue until
107/// it is resolved.
108///
109/// For example, users may have a timeout for the task, and the task will be dropped if it exceeds the timeout.
110/// If we `pop_front` the task before it resolves, the task will be canceled and the result will be lost.
111pub struct ConcurrentTasks<I, O> {
112    /// The executor to execute the tasks.
113    ///
114    /// If user doesn't provide an executor, the tasks will be executed with the default executor.
115    executor: Executor,
116    /// The factory to create the task.
117    ///
118    /// Caller of ConcurrentTasks must provides a factory to create the task for executing.
119    ///
120    /// The factory must accept an input and return a future that resolves to a tuple of input and
121    /// output result. If the given result is error, the error will be returned to users and the
122    /// task will be retried.
123    factory: fn(I) -> BoxedStaticFuture<(I, Result<O>)>,
124
125    /// `tasks` holds the ongoing tasks.
126    ///
127    /// Please keep in mind that all tasks are running in the background by `Executor`. We only need
128    /// to poll the tasks to see if they are ready.
129    ///
130    /// Dropping task without `await` it will cancel the task.
131    tasks: VecDeque<Task<(I, Result<O>)>>,
132    /// `results` stores the successful results.
133    results: VecDeque<O>,
134
135    /// The maximum number of concurrent tasks.
136    concurrent: usize,
137    /// The maximum number of completed tasks that can be buffered.
138    prefetch: usize,
139    /// Tracks the number of tasks that have finished execution but have not yet been collected.
140    /// This count is subtracted from the total concurrency capacity, ensuring that the system
141    /// always schedules new tasks to maintain the user's desired concurrency level.
142    ///
143    /// Example: If `concurrency = 10` and `completed_but_unretrieved = 3`,
144    ///          the system can still spawn 7 new tasks (since 3 slots are "logically occupied"
145    ///          by uncollected results).
146    completed_but_unretrieved: Arc<AtomicUsize>,
147    /// hitting the last unrecoverable error.
148    ///
149    /// If concurrent tasks hit an unrecoverable error, it will stop executing new tasks and return
150    /// an unrecoverable error to users.
151    errored: bool,
152}
153
154impl<I: Send + 'static, O: Send + 'static> ConcurrentTasks<I, O> {
155    /// Create a new concurrent tasks with given executor, concurrent, prefetch and factory.
156    ///
157    /// The factory is a function pointer that shouldn't capture any context.
158    pub fn new(
159        executor: Executor,
160        concurrent: usize,
161        prefetch: usize,
162        factory: fn(I) -> BoxedStaticFuture<(I, Result<O>)>,
163    ) -> Self {
164        Self {
165            executor,
166            factory,
167
168            tasks: VecDeque::with_capacity(concurrent),
169            results: VecDeque::with_capacity(concurrent),
170            concurrent,
171            prefetch,
172            completed_but_unretrieved: Arc::default(),
173            errored: false,
174        }
175    }
176
177    /// Return true if the tasks are running concurrently.
178    #[inline]
179    fn is_concurrent(&self) -> bool {
180        self.concurrent > 1
181    }
182
183    /// Clear all tasks and results.
184    ///
185    /// All ongoing tasks will be canceled.
186    pub fn clear(&mut self) {
187        self.tasks.clear();
188        self.results.clear();
189    }
190
191    /// Check if there are remaining space to push new tasks.
192    #[inline]
193    pub fn has_remaining(&self) -> bool {
194        let completed = self.completed_but_unretrieved.load(Ordering::Relaxed);
195        // Allow up to `prefetch` completed tasks to be buffered
196        self.tasks.len() < self.concurrent + completed.min(self.prefetch)
197    }
198
199    /// Chunk if there are remaining results to fetch.
200    #[inline]
201    pub fn has_result(&self) -> bool {
202        !self.results.is_empty()
203    }
204
205    /// Create a task with given input.
206    pub fn create_task(&self, input: I) -> Task<(I, Result<O>)> {
207        let completed = self.completed_but_unretrieved.clone();
208
209        let fut = (self.factory)(input).inspect(move |_| {
210            completed.fetch_add(1, Ordering::Relaxed);
211        });
212
213        self.executor.execute(fut)
214    }
215
216    /// Execute the task with given input.
217    ///
218    /// - Execute the task in the current thread if is not concurrent.
219    /// - Execute the task in the background if there are available slots.
220    /// - Await the first task in the queue if there is no available slots.
221    pub async fn execute(&mut self, input: I) -> Result<()> {
222        if self.errored {
223            return Err(Error::new(
224                ErrorKind::Unexpected,
225                "concurrent tasks met an unrecoverable error",
226            ));
227        }
228
229        // Short path for non-concurrent case.
230        if !self.is_concurrent() {
231            let (_, o) = (self.factory)(input).await;
232            return match o {
233                Ok(o) => {
234                    self.results.push_back(o);
235                    Ok(())
236                }
237                // We don't need to rebuild the future if it's not concurrent.
238                Err(err) => Err(err),
239            };
240        }
241
242        if !self.has_remaining() {
243            let (i, o) = self
244                .tasks
245                .front_mut()
246                .expect("tasks must be available")
247                .await;
248            self.completed_but_unretrieved
249                .fetch_sub(1, Ordering::Relaxed);
250            match o {
251                Ok(o) => {
252                    let _ = self.tasks.pop_front();
253                    self.results.push_back(o)
254                }
255                Err(err) => {
256                    // Retry this task if the error is temporary
257                    if err.is_temporary() {
258                        let task = self.create_task(i);
259                        self.tasks
260                            .front_mut()
261                            .expect("tasks must be available")
262                            .replace(task)
263                    } else {
264                        self.clear();
265                        self.errored = true;
266                    }
267                    return Err(err);
268                }
269            }
270        }
271
272        self.tasks.push_back(self.create_task(input));
273        Ok(())
274    }
275
276    /// Fetch the successful result from the result queue.
277    pub async fn next(&mut self) -> Option<Result<O>> {
278        if self.errored {
279            return Some(Err(Error::new(
280                ErrorKind::Unexpected,
281                "concurrent tasks met an unrecoverable error",
282            )));
283        }
284
285        if let Some(result) = self.results.pop_front() {
286            return Some(Ok(result));
287        }
288
289        if let Some(task) = self.tasks.front_mut() {
290            let (i, o) = task.await;
291            self.completed_but_unretrieved
292                .fetch_sub(1, Ordering::Relaxed);
293            return match o {
294                Ok(o) => {
295                    let _ = self.tasks.pop_front();
296                    Some(Ok(o))
297                }
298                Err(err) => {
299                    // Retry this task if the error is temporary
300                    if err.is_temporary() {
301                        let task = self.create_task(i);
302                        self.tasks
303                            .front_mut()
304                            .expect("tasks must be available")
305                            .replace(task)
306                    } else {
307                        self.clear();
308                        self.errored = true;
309                    }
310                    Some(Err(err))
311                }
312            };
313        }
314
315        None
316    }
317}
318
319#[cfg(test)]
320mod tests {
321    use std::time::Duration;
322
323    use pretty_assertions::assert_eq;
324    use rand::Rng;
325    use tokio::time::sleep;
326
327    use super::*;
328
329    #[tokio::test]
330    async fn test_concurrent_tasks() {
331        let executor = Executor::new();
332
333        let mut tasks = ConcurrentTasks::new(executor, 16, 8, |(i, dur)| {
334            Box::pin(async move {
335                sleep(dur).await;
336
337                // 5% rate to fail.
338                if rand::thread_rng().gen_range(0..100) > 90 {
339                    return (
340                        (i, dur),
341                        Err(Error::new(ErrorKind::Unexpected, "I'm lucky").set_temporary()),
342                    );
343                }
344                ((i, dur), Ok(i))
345            })
346        });
347
348        let mut ans = vec![];
349
350        for i in 0..10240 {
351            // Sleep up to 10ms
352            let dur = Duration::from_millis(rand::thread_rng().gen_range(0..10));
353            loop {
354                let res = tasks.execute((i, dur)).await;
355                if res.is_ok() {
356                    break;
357                }
358            }
359        }
360
361        loop {
362            match tasks.next().await.transpose() {
363                Ok(Some(i)) => ans.push(i),
364                Ok(None) => break,
365                Err(_) => continue,
366            }
367        }
368
369        assert_eq!(ans, (0..10240).collect::<Vec<_>>())
370    }
371
372    #[tokio::test]
373    async fn test_prefetch_backpressure() {
374        let executor = Executor::new();
375        let concurrent = 4;
376        let prefetch = 2;
377
378        // Create a slower task to ensure they don't complete immediately
379        let mut tasks = ConcurrentTasks::new(executor, concurrent, prefetch, |i: usize| {
380            Box::pin(async move {
381                sleep(Duration::from_millis(100)).await;
382                (i, Ok(i))
383            })
384        });
385
386        // Initially, we should have space for concurrent tasks
387        assert!(tasks.has_remaining(), "Should have space initially");
388
389        // Submit concurrent tasks
390        for i in 0..concurrent {
391            assert!(tasks.has_remaining(), "Should have space for task {i}");
392            tasks.execute(i).await.unwrap();
393        }
394
395        // Now we shouldn't have any more space (since no tasks have completed yet)
396        assert!(
397            !tasks.has_remaining(),
398            "Should not have space after submitting concurrent tasks"
399        );
400
401        // Wait for some tasks to complete
402        sleep(Duration::from_millis(150)).await;
403
404        // Now we should have space up to prefetch limit
405        for i in concurrent..concurrent + prefetch {
406            assert!(
407                tasks.has_remaining(),
408                "Should have space for prefetch task {i}"
409            );
410            tasks.execute(i).await.unwrap();
411        }
412
413        // Now has_remaining should return false
414        assert!(
415            !tasks.has_remaining(),
416            "Should not have remaining space after filling up prefetch buffer"
417        );
418
419        // Retrieve one result
420        let result = tasks.next().await;
421        assert!(result.is_some());
422
423        // Now there should be space for one more task
424        assert!(
425            tasks.has_remaining(),
426            "Should have remaining space after retrieving one result"
427        );
428    }
429
430    #[tokio::test]
431    async fn test_prefetch_zero() {
432        let executor = Executor::new();
433        let concurrent = 4;
434        let prefetch = 0; // No prefetching allowed
435
436        let mut tasks = ConcurrentTasks::new(executor, concurrent, prefetch, |i: usize| {
437            Box::pin(async move {
438                sleep(Duration::from_millis(10)).await;
439                (i, Ok(i))
440            })
441        });
442
443        // With prefetch=0, we can only submit up to concurrent tasks
444        for i in 0..concurrent {
445            tasks.execute(i).await.unwrap();
446        }
447
448        // Should not have space for more
449        assert!(
450            !tasks.has_remaining(),
451            "Should not have remaining space with prefetch=0"
452        );
453
454        // Retrieve one result
455        let result = tasks.next().await;
456        assert!(result.is_some());
457
458        // Now there should be space for exactly one more task
459        assert!(
460            tasks.has_remaining(),
461            "Should have remaining space after retrieving one result"
462        );
463
464        // Execute one more
465        tasks.execute(concurrent).await.unwrap();
466
467        // Should be full again
468        assert!(!tasks.has_remaining(), "Should be full again");
469    }
470}
```
