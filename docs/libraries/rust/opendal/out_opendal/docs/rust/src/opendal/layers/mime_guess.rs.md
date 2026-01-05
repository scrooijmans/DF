# 

opendal/layers/

mime_guess.rs

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
18use crate::Result;
19use crate::raw::*;
20
21/// A layer that can automatically set `Content-Type` based on the file extension in the path.
22///
23/// # MimeGuess
24///
25/// This layer uses [mime_guess](https://crates.io/crates/mime_guess) to automatically
26/// set `Content-Type` based on the file extension in the operation path.
27///
28/// However, please note that this layer will not overwrite the `content_type` you manually set,
29/// nor will it overwrite the `content_type` provided by backend services.
30///
31/// A simple example is that for object storage backends, when you call `stat`, the backend will
32/// provide `content_type` information, and `mime_guess` will not be called, but will use
33/// the `content_type` provided by the backend.
34///
35/// But if you use the [Fs](../services/struct.Fs.html) backend to call `stat`, the backend will
36/// not provide `content_type` information, and our `mime_guess` will be called to provide you with
37/// appropriate `content_type` information.
38///
39/// Another thing to note is that using this layer does not necessarily mean that the result will 100%
40/// contain `content_type` information. If the extension of your path is custom or an uncommon type,
41/// the returned result will still not contain `content_type` information (the specific condition here is
42/// when [mime_guess::from_path::first_raw](https://docs.rs/mime_guess/latest/mime_guess/struct.MimeGuess.html#method.first_raw)
43/// returns `None`).
44///
45/// # Examples
46///
47/// ```no_run
48/// # use opendal::layers::MimeGuessLayer;
49/// # use opendal::services;
50/// # use opendal::Operator;
51/// # use opendal::Result;
52/// # use opendal::Scheme;
53///
54/// # fn main() -> Result<()> {
55/// let _ = Operator::new(services::Memory::default())?
56///     .layer(MimeGuessLayer::default())
57///     .finish();
58/// Ok(())
59/// # }
60/// ```
61#[derive(Debug, Clone, Default)]
62#[non_exhaustive]
63pub struct MimeGuessLayer {}
64
65impl<A: Access> Layer<A> for MimeGuessLayer {
66    type LayeredAccess = MimeGuessAccessor<A>;
67
68    fn layer(&self, inner: A) -> Self::LayeredAccess {
69        MimeGuessAccessor(inner)
70    }
71}
72
73#[derive(Clone, Debug)]
74pub struct MimeGuessAccessor<A: Access>(A);
75
76fn mime_from_path(path: &str) -> Option<&str> {
77    mime_guess::from_path(path).first_raw()
78}
79
80fn opwrite_with_mime(path: &str, op: OpWrite) -> OpWrite {
81    if op.content_type().is_some() {
82        return op;
83    }
84
85    if let Some(mime) = mime_from_path(path) {
86        return op.with_content_type(mime);
87    }
88
89    op
90}
91
92fn rpstat_with_mime(path: &str, rp: RpStat) -> RpStat {
93    rp.map_metadata(|metadata| {
94        if metadata.content_type().is_some() {
95            return metadata;
96        }
97
98        if let Some(mime) = mime_from_path(path) {
99            return metadata.with_content_type(mime.into());
100        }
101
102        metadata
103    })
104}
105
106impl<A: Access> LayeredAccess for MimeGuessAccessor<A> {
107    type Inner = A;
108    type Reader = A::Reader;
109    type Writer = A::Writer;
110    type Lister = A::Lister;
111    type Deleter = A::Deleter;
112
113    fn inner(&self) -> &Self::Inner {
114        &self.0
115    }
116
117    async fn read(&self, path: &str, args: OpRead) -> Result<(RpRead, Self::Reader)> {
118        self.inner().read(path, args).await
119    }
120
121    async fn write(&self, path: &str, args: OpWrite) -> Result<(RpWrite, Self::Writer)> {
122        self.inner()
123            .write(path, opwrite_with_mime(path, args))
124            .await
125    }
126
127    async fn stat(&self, path: &str, args: OpStat) -> Result<RpStat> {
128        self.inner()
129            .stat(path, args)
130            .await
131            .map(|rp| rpstat_with_mime(path, rp))
132    }
133
134    async fn delete(&self) -> Result<(RpDelete, Self::Deleter)> {
135        self.inner().delete().await
136    }
137
138    async fn list(&self, path: &str, args: OpList) -> Result<(RpList, Self::Lister)> {
139        self.inner().list(path, args).await
140    }
141}
142
143#[cfg(test)]
144mod tests {
145    use futures::TryStreamExt;
146
147    use super::*;
148    use crate::Metadata;
149    use crate::Operator;
150    use crate::services::Memory;
151
152    const DATA: &str = "<html>test</html>";
153    const CUSTOM: &str = "text/custom";
154    const HTML: &str = "text/html";
155
156    #[tokio::test]
157    async fn test_async() {
158        let op = Operator::new(Memory::default())
159            .unwrap()
160            .layer(MimeGuessLayer::default())
161            .finish();
162
163        op.write("test0.html", DATA).await.unwrap();
164        assert_eq!(
165            op.stat("test0.html").await.unwrap().content_type(),
166            Some(HTML)
167        );
168
169        op.write("test1.asdfghjkl", DATA).await.unwrap();
170        assert_eq!(
171            op.stat("test1.asdfghjkl").await.unwrap().content_type(),
172            None
173        );
174
175        op.write_with("test2.html", DATA)
176            .content_type(CUSTOM)
177            .await
178            .unwrap();
179        assert_eq!(
180            op.stat("test2.html").await.unwrap().content_type(),
181            Some(CUSTOM)
182        );
183
184        let entries: Vec<Metadata> = op
185            .lister_with("")
186            .await
187            .unwrap()
188            .and_then(|entry| {
189                let op = op.clone();
190                async move { op.stat(entry.path()).await }
191            })
192            .try_collect()
193            .await
194            .unwrap();
195        assert_eq!(entries[0].content_type(), Some(HTML));
196        assert_eq!(entries[1].content_type(), None);
197        assert_eq!(entries[2].content_type(), Some(CUSTOM));
198    }
199}
```
