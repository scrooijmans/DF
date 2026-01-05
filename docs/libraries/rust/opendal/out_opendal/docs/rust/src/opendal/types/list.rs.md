# 

opendal/types/

list.rs

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
18use std::pin::Pin;
19use std::task::Context;
20use std::task::Poll;
21use std::task::ready;
22
23use futures::Stream;
24
25use crate::raw::*;
26use crate::*;
27
28/// Lister is designed to list entries at given path in an asynchronous
29/// manner.
30///
31/// - Lister implements `Stream<Item = Result<Entry>>`.
32/// - Lister will return `None` if there is no more entries or error has been returned.
33pub struct Lister {
34    lister: Option<oio::Lister>,
35
36    fut: Option<BoxedStaticFuture<(oio::Lister, Result<Option<oio::Entry>>)>>,
37    errored: bool,
38}
39
40/// # Safety
41///
42/// Lister will only be accessed by `&mut Self`
43unsafe impl Sync for Lister {}
44
45impl Lister {
46    /// Create a new lister.
47    pub(crate) async fn create(acc: Accessor, path: &str, args: OpList) -> Result<Self> {
48        let (_, lister) = acc.list(path, args).await?;
49
50        Ok(Self {
51            lister: Some(lister),
52
53            fut: None,
54            errored: false,
55        })
56    }
57}
58
59impl Stream for Lister {
60    type Item = Result<Entry>;
61
62    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
63        // Returns `None` if we have errored.
64        if self.errored {
65            return Poll::Ready(None);
66        }
67
68        if let Some(mut lister) = self.lister.take() {
69            let fut = async move {
70                let res = lister.next_dyn().await;
71                (lister, res)
72            };
73            self.fut = Some(Box::pin(fut));
74        }
75
76        if let Some(fut) = self.fut.as_mut() {
77            let (lister, entry) = ready!(fut.as_mut().poll(cx));
78            self.lister = Some(lister);
79            self.fut = None;
80
81            return match entry {
82                Ok(Some(oe)) => Poll::Ready(Some(Ok(oe.into_entry()))),
83                Ok(None) => {
84                    self.lister = None;
85                    Poll::Ready(None)
86                }
87                Err(err) => {
88                    self.errored = true;
89                    Poll::Ready(Some(Err(err)))
90                }
91            };
92        }
93
94        Poll::Ready(None)
95    }
96}
97
98#[cfg(test)]
99#[cfg(feature = "services-azblob")]
100mod tests {
101    use futures::StreamExt;
102    use futures::future;
103
104    use super::*;
105    use crate::services::Azblob;
106
107    /// Inspired by <https://gist.github.com/kyle-mccarthy/1e6ae89cc34495d731b91ebf5eb5a3d9>
108    ///
109    /// Invalid lister should not panic nor endless loop.
110    #[tokio::test]
111    async fn test_invalid_lister() -> Result<()> {
112        let _ = tracing_subscriber::fmt().try_init();
113
114        let builder = Azblob::default()
115            .container("container")
116            .account_name("account_name")
117            .account_key("YWNjb3VudF9rZXk=") // Valid base64 encoding of "account_key"
118            .endpoint("https://account_name.blob.core.windows.net");
119
120        let operator = Operator::new(builder)?.finish();
121
122        let lister = operator.lister("/").await?;
123
124        lister
125            .filter_map(|entry| {
126                dbg!(&entry);
127                future::ready(entry.ok())
128            })
129            .for_each(|entry| {
130                println!("{entry:?}");
131                future::ready(())
132            })
133            .await;
134
135        Ok(())
136    }
137}
```
