# 

opendal/raw/

layer.rs

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
19use std::sync::Arc;
20
21use futures::Future;
22
23use crate::raw::*;
24use crate::*;
25
26/// Layer is used to intercept the operations on the underlying storage.
27///
28/// Struct that implement this trait must accept input `Arc<dyn Accessor>` as inner,
29/// and returns a new `Arc<dyn Accessor>` as output.
30///
31/// All functions in `Accessor` requires `&self`, so it's implementer's responsibility
32/// to maintain the internal mutability. Please also keep in mind that `Accessor`
33/// requires `Send` and `Sync`.
34///
35/// # Notes
36///
37/// ## Inner
38///
39/// It's required to implement `fn inner() -> Option<Arc<dyn Accessor>>` for layer's accessors.
40///
41/// By implement this method, all API calls will be forwarded to inner accessor instead.
42///
43/// # Examples
44///
45/// ```
46/// use std::sync::Arc;
47///
48/// use opendal::raw::*;
49/// use opendal::*;
50///
51/// /// Implement the real accessor logic here.
52/// #[derive(Debug)]
53/// struct TraceAccessor<A: Access> {
54///     inner: A,
55/// }
56///
57/// impl<A: Access> LayeredAccess for TraceAccessor<A> {
58///     type Inner = A;
59///     type Reader = A::Reader;
60///     type Writer = A::Writer;
61///     type Lister = A::Lister;
62///     type Deleter = A::Deleter;
63///
64///     fn inner(&self) -> &Self::Inner {
65///         &self.inner
66///     }
67///
68///     async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
69///         self.inner.read(path, args).await
70///     }
71///
72///     async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
73///         self.inner.write(path, args).await
74///     }
75///
76///     async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
77///         self.inner.list(path, args).await
78///     }
79///
80///     async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
81///         self.inner.delete().await
82///     }
83/// }
84///
85/// /// The public struct that exposed to users.
86/// ///
87/// /// Will be used like `op.layer(TraceLayer)`
88/// struct TraceLayer;
89///
90/// impl<A: Access> Layer<A> for TraceLayer {
91///     type LayeredAccess = TraceAccessor<A>;
92///
93///     fn layer(&self, inner: A) -> Self::LayeredAccess {
94///         TraceAccessor { inner }
95///     }
96/// }
97/// ```
98pub trait Layer<A: Access> {
99    /// The layered accessor that returned by this layer.
100    type LayeredAccess: Access;
101
102    /// Intercept the operations on the underlying storage.
103    fn layer(&self, inner: A) -> Self::LayeredAccess;
104}
105
106/// LayeredAccess is layered accessor that forward all not implemented
107/// method to inner.
108#[allow(missing_docs)]
109pub trait LayeredAccess: Send + Sync + Debug + Unpin + 'static {
110    type Inner: Access;
111
112    type Reader: oio::Read;
113    type Writer: oio::Write;
114    type Lister: oio::List;
115    type Deleter: oio::Delete;
116
117    fn inner(&self) -> &Self::Inner;
118
119    fn info(&self) -> Arc<AccessorInfo> {
120        self.inner().info()
121    }
122
123    fn create_dir(
124        &self,
125        path: &str,
126        args: OpCreateDir,
127    ) -> impl Future<Output = Result<RpCreateDir>> + MaybeSend {
128        self.inner().create_dir(path, args)
129    }
130
131    fn read(
132        &self,
133        path: &str,
134        args: OpRead,
135    ) -> impl Future<Output = Result<(RpRead, Self::Reader)>> + MaybeSend;
136
137    fn write(
138        &self,
139        path: &str,
140        args: OpWrite,
141    ) -> impl Future<Output = Result<(RpWrite, Self::Writer)>> + MaybeSend;
142
143    fn copy(
144        &self,
145        from: &str,
146        to: &str,
147        args: OpCopy,
148    ) -> impl Future<Output = Result<RpCopy>> + MaybeSend {
149        self.inner().copy(from, to, args)
150    }
151
152    fn rename(
153        &self,
154        from: &str,
155        to: &str,
156        args: OpRename,
157    ) -> impl Future<Output = Result<RpRename>> + MaybeSend {
158        self.inner().rename(from, to, args)
159    }
160
161    fn stat(&self, path: &str, args: OpStat) -> impl Future<Output = Result<RpStat>> + MaybeSend {
162        self.inner().stat(path, args)
163    }
164
165    fn delete(&self) -> impl Future<Output = Result<(RpDelete, Self::Deleter)>> + MaybeSend;
166
167    fn list(
168        &self,
169        path: &str,
170        args: OpList,
171    ) -> impl Future<Output = Result<(RpList, Self::Lister)>> + MaybeSend;
172
173    fn presign(
174        &self,
175        path: &str,
176        args: OpPresign,
177    ) -> impl Future<Output = Result<RpPresign>> + MaybeSend {
178        self.inner().presign(path, args)
179    }
180}
181
182impl<L: LayeredAccess> Access for L {
183    type Reader = L::Reader;
184    type Writer = L::Writer;
185    type Lister = L::Lister;
186    type Deleter = L::Deleter;
187
188    fn info(&self) -> Arc<AccessorInfo> {
189        LayeredAccess::info(self)
190    }
191
192    async fn create_dir(&self, path: &str, args: OpCreateDir) -> Result<RpCreateDir> {
193        LayeredAccess::create_dir(self, path, args).await
194    }
195
196    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
197        LayeredAccess::read(self, path, args).await
198    }
199
200    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
201        LayeredAccess::write(self, path, args).await
202    }
203
204    async fn copy(&self, from: &str, to: &str, args: OpCopy) -> Result<RpCopy> {
205        LayeredAccess::copy(self, from, to, args).await
206    }
207
208    async fn rename(&self, from: &str, to: &str, args: OpRename) -> Result<RpRename> {
209        LayeredAccess::rename(self, from, to, args).await
210    }
211
212    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
213        LayeredAccess::stat(self, path, args).await
214    }
215
216    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
217        LayeredAccess::delete(self).await
218    }
219
220    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
221        LayeredAccess::list(self, path, args).await
222    }
223
224    async fn presign(&self, path: &str, args: OpPresign) -> Result<RpPresign> {
225        LayeredAccess::presign(self, path, args).await
226    }
227}
228
229#[cfg(test)]
230mod tests {
231    use std::sync::Arc;
232
233    use futures::lock::Mutex;
234
235    use super::*;
236    use crate::services::Memory;
237
238    #[derive(Debug)]
239    struct Test<A: Access> {
240        #[allow(dead_code)]
241        inner: Option<A>,
242        stated: Arc<Mutex<bool>>,
243    }
244
245    impl<A: Access> Layer<A> for &Test<A> {
246        type LayeredAccess = Test<A>;
247
248        fn layer(&self, inner: A) -> Self::LayeredAccess {
249            Test {
250                inner: Some(inner),
251                stated: self.stated.clone(),
252            }
253        }
254    }
255
256    impl<A: Access> Access for Test<A> {
257        type Reader = ();
258        type Writer = ();
259        type Lister = ();
260        type Deleter = ();
261
262        fn info(&self) -> Arc<AccessorInfo> {
263            let am = AccessorInfo::default();
264            am.set_scheme("test");
265            am.into()
266        }
267
268        async fn stat(&self, _: &str, _: OpStat) -> Result<RpStat> {
269            let mut x = self.stated.lock().await;
270            *x = true;
271
272            assert!(self.inner.is_some());
273
274            // We will not call anything here to test the layer.
275            Ok(RpStat::new(Metadata::new(EntryMode::DIR)))
276        }
277    }
278
279    #[tokio::test]
280    async fn test_layer() {
281        let test = Test {
282            inner: None,
283            stated: Arc::new(Mutex::new(false)),
284        };
285
286        let op = Operator::new(Memory::default())
287            .unwrap()
288            .layer(&test)
289            .finish();
290
291        op.stat("xxxxx").await.unwrap();
292
293        assert!(*test.stated.clone().lock().await);
294    }
295}
```
