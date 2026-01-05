# 

opendal/types/delete/

deleter.rs

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
18use std::pin::pin;
19
20use futures::Stream;
21use futures::StreamExt;
22
23use crate::raw::oio::DeleteDyn;
24use crate::raw::*;
25use crate::*;
26
27/// Deleter is designed to continuously remove content from storage.
28///
29/// It leverages batch deletion capabilities provided by storage services for efficient removal.
30///
31/// # Usage
32///
33/// [`Deleter`] provides several ways to delete files:
34///
35/// ## Direct Deletion
36///
37/// Use the `delete` method to remove a single file:
38///
39/// ```rust
40/// use opendal::Operator;
41/// use opendal::Result;
42///
43/// async fn example(op: Operator) -> Result<()> {
44///     let mut d = op.deleter().await?;
45///     d.delete("path/to/file").await?;
46///     d.close().await?;
47///     Ok(())
48/// }
49/// ```
50///
51/// Delete multiple files via a stream:
52///
53/// ```rust
54/// use futures::stream;
55/// use opendal::Operator;
56/// use opendal::Result;
57///
58/// async fn example(op: Operator) -> Result<()> {
59///     let mut d = op.deleter().await?;
60///     d.delete_stream(stream::iter(vec!["path/to/file"])).await?;
61///     d.close().await?;
62///     Ok(())
63/// }
64/// ```
65///
66/// ## Using as a Sink
67///
68/// Deleter can be used as a Sink for file deletion:
69///
70/// ```rust
71/// use futures::stream;
72/// use futures::Sink;
73/// use futures::SinkExt;
74/// use opendal::Operator;
75/// use opendal::Result;
76///
77/// async fn example(op: Operator) -> Result<()> {
78///     let mut sink = op.deleter().await?.into_sink();
79///     sink.send("path/to/file").await?;
80///     sink.close().await?;
81///     Ok(())
82/// }
83/// ```
84pub struct Deleter {
85    deleter: oio::Deleter,
86
87    max_size: usize,
88    cur_size: usize,
89}
90
91impl Deleter {
92    pub(crate) async fn create(acc: Accessor) -> Result<Self> {
93        let max_size = acc.info().full_capability().delete_max_size.unwrap_or(1);
94        let (_, deleter) = acc.delete().await?;
95
96        Ok(Self {
97            deleter,
98            max_size,
99            cur_size: 0,
100        })
101    }
102
103    /// Delete a path.
104    pub async fn delete(&mut self, input: impl IntoDeleteInput) -> Result<()> {
105        if self.cur_size >= self.max_size {
106            let deleted = self.deleter.flush_dyn().await?;
107            self.cur_size -= deleted;
108        }
109
110        let input = input.into_delete_input();
111        let mut op = OpDelete::default();
112        if let Some(version) = &input.version {
113            op = op.with_version(version);
114        }
115
116        self.deleter.delete_dyn(&input.path, op)?;
117        self.cur_size += 1;
118        Ok(())
119    }
120
121    /// Delete an infallible iterator of paths.
122    ///
123    /// Also see:
124    ///
125    /// - [`Deleter::delete_try_iter`]: delete a fallible iterator of paths.
126    /// - [`Deleter::delete_stream`]: delete an infallible stream of paths.
127    /// - [`Deleter::delete_try_stream`]: delete a fallible stream of paths.
128    pub async fn delete_iter<I, D>(&mut self, iter: I) -> Result<()>
129    where
130        I: IntoIterator<Item = D>,
131        D: IntoDeleteInput,
132    {
133        for entry in iter {
134            self.delete(entry).await?;
135        }
136        Ok(())
137    }
138
139    /// Delete a fallible iterator of paths.
140    ///
141    /// Also see:
142    ///
143    /// - [`Deleter::delete_iter`]: delete an infallible iterator of paths.
144    /// - [`Deleter::delete_stream`]: delete an infallible stream of paths.
145    /// - [`Deleter::delete_try_stream`]: delete a fallible stream of paths.
146    pub async fn delete_try_iter<I, D>(&mut self, try_iter: I) -> Result<()>
147    where
148        I: IntoIterator<Item = Result<D>>,
149        D: IntoDeleteInput,
150    {
151        for entry in try_iter {
152            self.delete(entry?).await?;
153        }
154
155        Ok(())
156    }
157
158    /// Delete an infallible stream of paths.
159    ///
160    /// Also see:
161    ///
162    /// - [`Deleter::delete_iter`]: delete an infallible iterator of paths.
163    /// - [`Deleter::delete_try_iter`]: delete a fallible iterator of paths.
164    /// - [`Deleter::delete_try_stream`]: delete a fallible stream of paths.
165    pub async fn delete_stream<S, D>(&mut self, stream: S) -> Result<()>
166    where
167        S: Stream<Item = D>,
168        D: IntoDeleteInput,
169    {
170        let mut stream = pin!(stream);
171        while let Some(entry) = stream.next().await {
172            self.delete(entry).await?;
173        }
174
175        Ok(())
176    }
177
178    /// Delete a fallible stream of paths.
179    ///
180    /// Also see:
181    ///
182    /// - [`Deleter::delete_iter`]: delete an infallible iterator of paths.
183    /// - [`Deleter::delete_try_iter`]: delete a fallible iterator of paths.
184    /// - [`Deleter::delete_stream`]: delete an infallible stream of paths.
185    pub async fn delete_try_stream<S, D>(&mut self, try_stream: S) -> Result<()>
186    where
187        S: Stream<Item = Result<D>>,
188        D: IntoDeleteInput,
189    {
190        let mut stream = pin!(try_stream);
191        while let Some(entry) = stream.next().await.transpose()? {
192            self.delete(entry).await?;
193        }
194
195        Ok(())
196    }
197
198    /// Flush the deleter, returns the number of deleted paths.
199    pub async fn flush(&mut self) -> Result<usize> {
200        let deleted = self.deleter.flush_dyn().await?;
201        self.cur_size -= deleted;
202        Ok(deleted)
203    }
204
205    /// Close the deleter, this will flush the deleter and wait until all paths are deleted.
206    pub async fn close(&mut self) -> Result<()> {
207        loop {
208            self.flush().await?;
209            if self.cur_size == 0 {
210                break;
211            }
212        }
213        Ok(())
214    }
215
216    /// Convert the deleter into a sink.
217    pub fn into_sink<T: IntoDeleteInput>(self) -> FuturesDeleteSink<T> {
218        FuturesDeleteSink::new(self)
219    }
220}
```
