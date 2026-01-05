# One post tagged with "internal"

[View All Tags](https://opendal.apache.org/blog/tags/)

## [Apache OpenDALâ„¢ Internal: Data Reading](https://opendal.apache.org/blog/how-opendal-read-data/)

August 15, 2023 Â· 9 min read

<a href="https://github.com/Xuanwo" rel="noopener noreferrer" target="_blank">Xuanwo</a>

As the Apache OpenDALâ„¢ community continues to grow, new abstractions are constantly being added, which has brought some burdens to new contributors participating in development. Many maintainers hope to have a deeper understanding of OpenDAL's internal implementation. At the same time, OpenDAL's core design has not changed significantly for a long time, making it possible to write a series on internal implementation. I believe now is the time to write a series of articles on OpenDAL's internal implementation, to explain from the maintainer's perspective how OpenDAL is designed, implemented, and how it can be expanded. With the impending release of OpenDAL v0.40, I hope this series of articles will better help the community understand the past, master the present, and shape the future.

The first article will discuss OpenDAL's most commonly used data reading function. I will start from the outermost interface and then gradually unfold according to the calling sequence of OpenDAL. Let's get started!

**Tags:**

- <a href="https://opendal.apache.org/blog/tags/internal/" class="tag_f4_J tagRegular_MJrJ" rel="tag">internal</a>

<a href="https://opendal.apache.org/blog/how-opendal-read-data/" aria-label="Read more about Apache OpenDALâ¢ Internal: Data Reading"><strong>Read more</strong></a>
